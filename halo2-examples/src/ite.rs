use halo2_examples::is_zero_gadget::{IsZeroChip, IsZeroConfig};
use halo2_proofs::{
    arithmetic::FieldExt,
    circuit::{Layouter, SimpleFloorPlanner, Value},
    plonk::{Advice, Circuit, Column, ConstraintSystem, Error, Expression, Selector},
    poly::Rotation,
};

#[derive(Debug, Clone)]
struct FunctionConfig<F: FieldExt> {
    selector: Selector,
    a: Column<Advice>,
    b: Column<Advice>,
    c: Column<Advice>,
    a_equals_b: IsZeroConfig<F>,
    output: Column<Advice>,
}

#[derive(Debug, Clone)]
struct FunctionChip<F: FieldExt> {
    config: FunctionConfig<F>,
}

impl<F: FieldExt> FunctionChip<F> {
    pub fn construct(config: FunctionConfig<F>) -> Self {
        Self { config }
    }

    pub fn configure(meta: &mut ConstraintSystem<F>) -> FunctionConfig<F> {
        let selector = meta.selector();
        let a = meta.advice_column();
        let b = meta.advice_column();
        let c = meta.advice_column();
        let output = meta.advice_column();
        let is_zero_advice_column = meta.advice_column();

        let a_equals_b = IsZeroChip::configure(
            meta,
            |meta| meta.query_selector(selector), // this is the q_enable
            |meta| meta.query_advice(a, Rotation::cur()) - meta.query_advice(b, Rotation::cur()), // this is the value
            is_zero_advice_column, // this is the advice column that stores value_inv
        );

        meta.create_gate("f(a, b, c) = if a == b {c} else {a - b}", |meta| {
            let s = meta.query_selector(selector);
            let a = meta.query_advice(a, Rotation::cur());
            let b = meta.query_advice(b, Rotation::cur());
            let c = meta.query_advice(c, Rotation::cur());
            // a  |  b  | c  | s      |a == b | output  |  s * (a == b) * (output - c) | s * (1 - a == b) * (output - (a - b))
            // --------------------------------
            // 10 | 12  | 15 | 1      | 0     | 10 - 12 | 1 * 0 * -17                  | 1 * 1 * 0 = 0
            // 10 | 10  | 15 | 1      | 1     |  15     | 1 * 1 * 0 (output == c)      | 1 * 0 * 15 = 0
            let output = meta.query_advice(output, Rotation::cur());
            vec![
                s.clone() * (a_equals_b.expr() * (output.clone() - c)), // in this case output == c 
                s * (Expression::Constant(F::one()) - a_equals_b.expr()) * (output - (a - b)), // in this case output == a - b
            ]
        });

        FunctionConfig {
            selector,
            a,
            b,
            c,
            a_equals_b,
            output,
        }
    }
}

#[derive(Default)]
struct FunctionCircuit<F> {
    a: F,
    b: F,
    c: F,
}

impl<F: FieldExt> Circuit<F> for FunctionCircuit<F> {
    type Config = FunctionConfig<F>;
    type FloorPlanner = SimpleFloorPlanner;

    fn without_witnesses(&self) -> Self {
        Self::default()
    }

    fn configure(meta: &mut ConstraintSystem<F>) -> Self::Config {
        FunctionChip::configure(meta)
    }

    fn synthesize(&self, config: Self::Config, mut layouter: impl Layouter<F>) -> Result<(), Error> {
        let chip = FunctionChip::construct(config);
        let is_zero_chip = IsZeroChip::construct(chip.config.a_equals_b.clone());

        let out = layouter.assign_region(
            || "f(a, b, c) = if a == b {c} else {a - b}",
            |mut region| {
                chip.config.selector.enable(&mut region, 0)?;
                region.assign_advice(|| "a", chip.config.a, 0, || Value::known(self.a))?;
                region.assign_advice(|| "b", chip.config.b, 0, || Value::known(self.b))?;
                region.assign_advice(|| "c", chip.config.c, 0, || Value::known(self.c))?;
                is_zero_chip.assign(&mut region, 0, Value::known(self.a - self.b))?;
                let output = if self.a == self.b { self.c } else { self.a - self.b };
                let out = region.assign_advice(|| "output", chip.config.output, 0, || Value::known(output))?;
                Ok(out)
            },
        )?;

        println!("Res = {:?}", out.value());
        Ok(())
    }
}

fn main() {
    use halo2_proofs::dev::MockProver;
    use halo2_proofs::halo2curves::bn256::Fr as Fp;

    let a = Fp::from(15);
    let b = Fp::from(10);
    let c = Fp::from(12);

    let circuit = FunctionCircuit {
        a: a,
        b: b,
        c: c,
    };

    let prover = MockProver::run(4, &circuit, vec![]).unwrap();
    prover.assert_satisfied();
}