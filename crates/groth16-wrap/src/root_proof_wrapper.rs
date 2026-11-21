//! The root proof wrapper of the Anoma prover: a Groth16 proof of the root proof, which the Solidity verifier of gnark
//! checks.

use std::path::Path;

use anoma_arm::field::{fields_to_bytes, to_bytes, to_hex};
use anoma_prover::circuit_prover::{CircuitProver, Proof};
use anoma_prover::root_proof_wrapper::{RootProofWrapper, TransactionProof};
use anoma_prover::transaction::ROOT;

use crate::{Error, FIELD_BYTES, Groth16Proof, WrapKeys, prove, single_party_setup, verify};

/// Wraps root proofs with the keys of the wrapper circuit for the verification key of the root circuit.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Groth16Wrapper {
    pub keys: WrapKeys,
}

impl RootProofWrapper for Groth16Wrapper {
    type Error = Error;

    fn wrap(&self, root_proof: &Proof) -> Result<TransactionProof, Error> {
        let public_inputs: Vec<[u8; FIELD_BYTES]> = root_proof
            .public_inputs
            .iter()
            .copied()
            .map(to_bytes)
            .collect();
        let proof = prove(
            &self.keys,
            &fields_to_bytes(&root_proof.fields),
            &public_inputs,
        )?;
        Ok(TransactionProof {
            bytes: proof.bytes,
            public_inputs: root_proof.public_inputs.clone(),
        })
    }
}

impl Groth16Wrapper {
    /// Sets up the keys for the root circuit of `prover` in `directory`, once for each hash of its verification key.
    ///
    /// One party makes the setup and knows its secret, so it can make false proofs: the keys are for tests only.
    pub fn single_party_setup(
        prover: &impl CircuitProver,
        directory: &Path,
    ) -> Result<Self, Error> {
        let root = prover.verification_key(ROOT)?;
        let keys = directory.join(to_hex(root.hash));
        if keys.exists() {
            return Ok(Self {
                keys: WrapKeys::new(keys),
            });
        }
        Ok(Self {
            keys: single_party_setup(&fields_to_bytes(&root.fields), &keys)?,
        })
    }

    /// Checks a transaction proof against its public inputs; [`Error::Rejected`] if the proof is invalid.
    pub fn verify(&self, proof: &TransactionProof) -> Result<(), Error> {
        verify(
            &self.keys,
            &Groth16Proof {
                bytes: proof.bytes.clone(),
                public_inputs: proof.public_inputs.iter().copied().map(to_bytes).collect(),
            },
        )
    }
}
