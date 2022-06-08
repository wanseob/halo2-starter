use halo2_proofs::plonk::{Advice, Column, Instance, Selector};

#[derive(Clone, Debug)]
pub struct FieldConfig {
    /// For this chip, we will use two advice columns to implement our instructions.
    /// These are also the columns through which we communicate with other parts of
    /// the circuit.
    pub advice: [Column<Advice>; 2],

    /// This is the public input (instance) column.
    pub instance: Column<Instance>,

    // We need a selector to enable the multiplication gate, so that we aren't placing
    // any constraints on cells where `NumericInstructions::mul` is not being used.
    // This is important when building larger circuits, where columns are used by
    // multiple sets of instructions.
    pub s_mul: Selector,
}