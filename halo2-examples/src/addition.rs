use std::marker::PhantomData;
use halo2_proofs::circuit::Value;
use halo2_proofs::{
    arithmetic::FieldExt,
    circuit::{Chip, Layouter, SimpleFloorPlanner},
    plonk::{Advice, Circuit, Column, ConstraintSystem, Error, Instance},
    poly::Rotation,
};
use halo2_proofs::plonk::Selector;

// specify necessary columns in the main table
#[derive(Clone, Debug)]
struct MyConfig {
    advice: [Column<Advice>; 3],
    instance: Column<Instance>,
    s_add: Selector,
}

struct FChip<Field: FieldExt> {
    config: MyConfig,
    _marker: PhantomData<Field>,
}

impl<Field: FieldExt> Chip<Field> for FChip<Field> {
    type Config = MyConfig;
    type Loaded = ();

    fn config(&self) -> &Self::Config {
        &self.config
    }

    fn loaded(&self) -> &Self::Loaded {
        &()
    }
}

impl<Field: FieldExt> FChip<Field> {
    fn configure(
        meta: &mut ConstraintSystem<Field>,
        advice: [Column<Advice>; 3],
        instance: Column<Instance>,
    ) -> <Self as Chip<Field>>::Config {
        // specify columns used for proving copy constraints
        meta.enable_equality(instance);
        for column in &advice {
            meta.enable_equality(*column);
        }
        let s_add = meta.selector();

        // define addition gate
        meta.create_gate("add", |meta| {
            let s_add = meta.query_selector(s_add);
            let lhs = meta.query_advice(advice[0], Rotation::cur());
            let rhs = meta.query_advice(advice[1], Rotation::cur());
            let out = meta.query_advice(advice[2], Rotation::cur());
            vec![s_add * (lhs + rhs - out)]
        });

        MyConfig {
            advice,
            instance,
            s_add,
        }
    }
}

#[derive(Default)]
struct MyCircuit<Field: FieldExt> {
    u: Value<Field>,
    v: Value<Field>,
}

impl<Field: FieldExt> Circuit<Field> for MyCircuit<Field> {
    type Config = MyConfig;
    type FloorPlanner = SimpleFloorPlanner;

    fn without_witnesses(&self) -> Self {
        Self::default()
    }

    fn configure(meta: &mut ConstraintSystem<Field>) -> Self::Config {
        let advice = [meta.advice_column(), meta.advice_column(), meta.advice_column()];
        let instance = meta.instance_column();
        FChip::configure(meta, advice, instance)
    }

    fn synthesize(
        &self, config: Self::Config, mut layouter: impl Layouter<Field>
    ) -> Result<(), Error> {
        let res = self.u + self.v;

        let (x_a1, x_b1, x_c1) = layouter.assign_region(
            || "addition region",
            |mut region| {
                config.s_add.enable(&mut region, 0)?;
                let x_a1 = region.assign_advice(|| "x_a1",
                    config.advice[0], 0, || self.u)?;
                let x_b1 = region.assign_advice(|| "x_b1",
                    config.advice[1], 0, || self.v)?;
                let x_c1 = region.assign_advice(|| "x_c1",
                    config.advice[2], 0, || res)?;
                
                Ok((x_a1, x_b1, x_c1))
            }
        )?;

        layouter.constrain_instance(x_c1.cell(), config.instance, 0)?;
        println!("{:?} + {:?} = {:?}", x_a1.value(), x_b1.value(), x_c1.value());
        
        Ok(())
    }
}

fn main() {
    use halo2_proofs::dev::MockProver;
    use halo2_proofs::halo2curves::bn256::Fr as Fp;

    let u = Fp::from(3);
    let v = Fp::from(2);
    let res = u + v;

    let circuit = MyCircuit {
        u: Value::known(u),
        v: Value::known(v),
    };

    // the number of rows cannot exceed 2^k
    let k = 4;
    let prover = MockProver::run(k, &circuit, vec![vec![res]]).unwrap();
    assert_eq!(prover.verify(), Ok(()));
}