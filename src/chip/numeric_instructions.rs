use halo2_proofs::{
    arithmetic::FieldExt,
    circuit::{AssignedCell, Chip, Layouter, Region},
    plonk::Error,
};

use super::field_chip::FieldChip;

pub trait NumericInstructions<F: FieldExt>: Chip<F> {
    /// Variable representing a number.
    type Num;

    /// Loads a number into the circuit as a private input.
    fn load_private(&self, layouter: impl Layouter<F>, a: Option<F>) -> Result<Self::Num, Error>;

    /// Loads a number into the circuit as a fixed constant.
    fn load_constant(&self, layouter: impl Layouter<F>, constant: F) -> Result<Self::Num, Error>;

    /// Returns `c = a * b`.
    fn mul(
        &self,
        layouter: impl Layouter<F>,
        a: Self::Num,
        b: Self::Num,
    ) -> Result<Self::Num, Error>;

    /// Exposes a number as a public input to the circuit.
    fn expose_public(
        &self,
        layouter: impl Layouter<F>,
        num: Self::Num,
        row: usize,
    ) -> Result<(), Error>;
}

// ANCHOR: instructions-impl
/// A variable representing a number.
#[derive(Clone)]
pub struct Number<F: FieldExt>(AssignedCell<F, F>);

impl<F: FieldExt> NumericInstructions<F> for FieldChip<F> {
    type Num = Number<F>;

    fn load_private(
        &self,
        mut layouter: impl Layouter<F>,
        value: Option<F>,
    ) -> Result<Self::Num, Error> {
        let config = self.config();

        layouter.assign_region(
            || "load private",
            |mut region| {
                region
                    .assign_advice(
                        || "private input",
                        config.advice[0],
                        0,
                        || value.ok_or(Error::Synthesis),
                    )
                    .map(Number)
            },
        )
    }

    fn load_constant(
        &self,
        mut layouter: impl Layouter<F>,
        constant: F,
    ) -> Result<Self::Num, Error> {
        let config = self.config();

        layouter.assign_region(
            || "load constant",
            |mut region| {
                region
                    .assign_advice_from_constant(|| "constant value", config.advice[0], 0, constant)
                    .map(Number)
            },
        )
    }

    fn mul(
        &self,
        mut layouter: impl Layouter<F>,
        a: Self::Num,
        b: Self::Num,
    ) -> Result<Self::Num, Error> {
        let config = self.config();

        layouter.assign_region(
            || "mul",
            |mut region: Region<'_, F>| {
                // We only want to use a single multiplication gate in this region,
                // so we enable it at region offset 0; this means it will constrain
                // cells at offsets 0 and 1.
                config.s_mul.enable(&mut region, 0)?;

                // The inputs we've been given could be located anywhere in the circuit,
                // but we can only rely on relative offsets inside this region. So we
                // assign new cells inside the region and constrain them to have the
                // same values as the inputs.
                a.0.copy_advice(|| "lhs", &mut region, config.advice[0], 0)?;
                b.0.copy_advice(|| "rhs", &mut region, config.advice[1], 0)?;

                // Now we can assign the multiplication result, which is to be assigned
                // into the output position.
                let value = a.0.value().and_then(|a| b.0.value().map(|b| *a * *b));

                // Finally, we do the assignment to the output, returning a
                // variable to be used in another part of the circuit.
                region
                    .assign_advice(
                        || "lhs * rhs",
                        config.advice[0],
                        1,
                        || value.ok_or(Error::Synthesis),
                    )
                    .map(Number)
            },
        )
    }

    fn expose_public(
        &self,
        mut layouter: impl Layouter<F>,
        num: Self::Num,
        row: usize,
    ) -> Result<(), Error> {
        let config = self.config();

        layouter.constrain_instance(num.0.cell(), config.instance, row)
    }
}
