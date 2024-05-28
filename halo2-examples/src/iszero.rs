use halo2_examples::is_zero_gadget::{IsZeroChip, IsZeroConfig};
use halo2_proofs::{
    arithmetic::FieldExt,
    circuit::{Layouter, SimpleFloorPlanner, Value},
    plonk::{Advice, Circuit, Column, ConstraintSystem, Error, Selector},
    poly::Rotation,
};

#[derive(Default)]
struct IsZeroCircuit<F: FieldExt> {
    value: Value<F>,
}

impl<F: FieldExt> Circuit<F> for IsZeroCircuit<F> {
    type Config = (IsZeroConfig<F>, Column<Advice>, Selector);
    type FloorPlanner = SimpleFloorPlanner;

    fn without_witnesses(&self) -> Self {
        Self::default()
    }

    fn configure(meta: &mut ConstraintSystem<F>) -> Self::Config {
        let value = meta.advice_column();
        let value_inv = meta.advice_column();
        let s_enable = meta.selector();

        let is_zero_config = IsZeroChip::configure(
            meta,
            |meta| meta.query_selector(s_enable),
            |meta| meta.query_advice(value, Rotation::cur()),
            value_inv,
        );

        (is_zero_config, value, s_enable)
    }

    fn synthesize(
        &self, 
        config: Self::Config, 
        mut layouter: impl Layouter<F>
    ) -> Result<(), Error> {
        let (is_zero_config, value, s_enable) = config;

        let is_zero_chip = IsZeroChip::construct(is_zero_config);

        let res = layouter.assign_region(
            || "is_zero region",
            |mut region| {
                s_enable.enable(&mut region, 0)?;
                region.assign_advice(|| "value", value, 0, || self.value)?;
                let res = is_zero_chip.assign(&mut region, 0, self.value)?;
                Ok(res)
            },
        )?;

        println!("Res = {:?}", res.value());

        Ok(())
    }
}

fn main() {
    use halo2_proofs::dev::MockProver;
    use halo2_proofs::halo2curves::bn256::Fr as Fp;

    let value = Fp::from(1);

    let circuit = IsZeroCircuit {
        value: Value::known(value),
    };

    let prover = MockProver::run(4, &circuit, vec![]).unwrap();
    prover.assert_satisfied();
}