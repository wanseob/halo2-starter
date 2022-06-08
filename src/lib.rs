mod chip;
mod circuit;
pub use circuit::MyCircuit;

#[cfg(test)]
mod tests {
    use crate::MyCircuit;
    use halo2_proofs::{dev::MockProver, pasta::Fp};

    #[test]
    fn circuit_test() {
        let result = 2 + 2;
        assert_eq!(result, 4);

        // ANCHOR: test-circuit
        // The number of rows in our circuit cannot exceed 2^k. Since our example
        // circuit is very small, we can pick a very small value here.
        let k = 4;

        // Prepare the private and public inputs to the circuit!
        let constant = Fp::from(7);
        let a = Fp::from(2);
        let b = Fp::from(3);
        let c = constant * a.square() * b.square();

        // Instantiate the circuit with the private inputs.
        let circuit = MyCircuit {
            constant,
            a: Some(a),
            b: Some(b),
        };

        // Arrange the public input. We expose the multiplication result in row 0
        // of the instance column, so we position it there in our public inputs.
        let mut public_inputs = vec![c];

        // Given the correct public input, our circuit will verify.
        let prover = MockProver::run(k, &circuit, vec![public_inputs.clone()]).unwrap();
        assert_eq!(prover.verify(), Ok(()));

        // If we try some other public input, the proof will fail!
        public_inputs[0] += Fp::one();
        let prover = MockProver::run(k, &circuit, vec![public_inputs]).unwrap();
        assert!(prover.verify().is_err());
        // ANCHOR_END: test-circuit
    }
}
