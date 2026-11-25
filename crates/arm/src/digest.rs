//! The action digest that binds the final proof to the actions of the transaction, as the contract and the step
//! circuits compute it.

use ark_ff::PrimeField;
use sha2::{Digest, Sha256};

use crate::Fr;
use crate::action::Action;
use crate::field::{low_253_bits, to_bytes};

/// The chain and the address of an AnomaVerifier, to which the proof of a transaction is bound.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Deployment {
    pub chain_id: u64,
    pub verifier: [u8; 20],
}

impl Deployment {
    /// The public input of the final proof: the low 253 bits of the SHA-256 hash of the chain ID, the address of the
    /// verifier and the action digest, each as 32 big-endian bytes.
    pub fn bind(&self, digest: Fr) -> Fr {
        let mut bytes = [0; 3 * 32];
        bytes[32 - 8..32].copy_from_slice(&self.chain_id.to_be_bytes());
        bytes[2 * 32 - 20..2 * 32].copy_from_slice(&self.verifier);
        bytes[2 * 32..].copy_from_slice(&to_bytes(digest));
        low_253_bits(Sha256::digest(bytes).into())
    }

    /// The address of the verifier as a field element, as the root circuit takes it.
    pub fn verifier_field(&self) -> Fr {
        Fr::from_be_bytes_mod_order(&self.verifier)
    }
}

/// Extends the digest of the actions before an action with the action; the digest of no actions is zero. Each
/// resource adds its `is_consumed` flag, tag, logic ref, commitment tree root and app data hash.
pub fn extend(digest: Fr, action: &Action) -> Fr {
    let mut bytes = Vec::with_capacity((1 + 5 * action.resources.len()) * 32);
    bytes.extend_from_slice(&to_bytes(digest));
    for resource in &action.resources {
        for field in [
            Fr::from(resource.is_consumed),
            resource.tag,
            resource.logic_ref,
            resource.commitment_tree_root,
            resource.app_data.hash(),
        ] {
            bytes.extend_from_slice(&to_bytes(field));
        }
    }
    low_253_bits(Sha256::digest(&bytes).into())
}

/// The digest of all actions of a transaction, which is the public input of its final proof.
pub fn action_digest(actions: &[Action]) -> Fr {
    actions.iter().fold(Fr::from(0u64), extend)
}
