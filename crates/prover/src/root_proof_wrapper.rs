//! The port to a wrapper of the root proof: it turns the root proof of a transaction into the proof that the
//! AnomaVerifier checks.

use anoma_arm::Fr;

use crate::circuit_prover::Proof;

/// The proof of a transaction as the AnomaVerifier takes it, and the public inputs of the root proof, which it keeps.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct TransactionProof {
    /// The `proof` argument of `execute`.
    pub bytes: Vec<u8>,
    pub public_inputs: Vec<Fr>,
}

/// Wraps the root proof of a transaction.
pub trait RootProofWrapper {
    type Error: std::error::Error + Send + Sync + 'static;

    /// The proof that the AnomaVerifier checks; an error if the root proof does not verify.
    fn wrap(&self, root_proof: &Proof) -> Result<TransactionProof, Self::Error>;
}
