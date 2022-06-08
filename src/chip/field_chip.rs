use std::marker::PhantomData;

use halo2_proofs::{
    arithmetic::FieldExt,
    circuit::Chip,
};

use super::field_config::FieldConfig;

pub struct FieldChip<F: FieldExt> {
    pub config: FieldConfig,
    pub _marker: PhantomData<F>,
}

// ANCHOR: chip-impl
impl<F: FieldExt> Chip<F> for FieldChip<F> {
    type Config = FieldConfig;
    type Loaded = ();

    fn config(&self) -> &Self::Config {
        &self.config
    }

    fn loaded(&self) -> &Self::Loaded {
        &()
    }
}

