//! The trait of a prover of the Noir circuits: it gives the verification key of a circuit, proves it, and verifies
//! proofs.

use anoma_arm::Fr;

/// The values of the inputs of a circuit by their Noir names, in the shape of a `Prover.toml`.
pub type CircuitInputs = toml::Table;

/// The proof system of a proof: a leaf proof hides its witness, a step or root proof does not.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Target {
    Leaf,
    Step,
}

/// The verification key of a circuit, as the fields that a recursive verifier takes, and its hash.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct VerificationKey {
    pub fields: Vec<Fr>,
    pub hash: Fr,
}

/// A proof, as the fields that a recursive verifier takes, and its public inputs.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct Proof {
    pub fields: Vec<Fr>,
    pub public_inputs: Vec<Fr>,
}

/// Why a circuit prover gives no proof, or rejects one.
#[derive(Debug, thiserror::Error)]
pub enum CircuitError {
    #[error("the inputs do not satisfy {circuit}: {reason}")]
    Unsatisfied { circuit: String, reason: String },
    #[error("the proof of {circuit} does not verify: {reason}")]
    Rejected { circuit: String, reason: String },
    #[error("the prover fails on {circuit}: {source}")]
    Failed {
        circuit: String,
        source: Box<dyn std::error::Error + Send + Sync>,
    },
}

/// Proves and verifies the circuits of the workspace. The verification key of a circuit is the same for both targets.
pub trait CircuitProver {
    /// The verification key of a circuit. The hash of the key of a resource logic is its logic ref.
    fn verification_key(&self, circuit: &str) -> Result<VerificationKey, CircuitError>;

    /// Solves the circuit for the inputs and proves it.
    fn prove(
        &self,
        circuit: &str,
        inputs: &CircuitInputs,
        target: Target,
    ) -> Result<Proof, CircuitError>;

    /// Verifies a proof with `key` against `public_inputs`; `circuit` names the circuit in errors.
    fn verify(
        &self,
        circuit: &str,
        key: &VerificationKey,
        proof: &[Fr],
        public_inputs: &[Fr],
        target: Target,
    ) -> Result<(), CircuitError>;
}
