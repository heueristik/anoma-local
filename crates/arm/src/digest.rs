//! The action digest that binds the final proof to the actions of the transaction, as the contract and the step
//! circuits compute it.

use ark_ff::PrimeField;
use sha2::{Digest, Sha256};

use crate::Fr;
use crate::field::to_bytes;

/// Whether the contract emits a blob in an event.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DeletionCriterion {
    Immediately = 0,
    Never = 1,
}

/// A blob and its deletion criterion.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Blob {
    pub deletion_criterion: DeletionCriterion,
    pub bytes: Vec<u8>,
}

/// The application data of a resource, as four lists of blobs, in the order of the contract's `AppData`.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct AppData {
    pub resource_payload: Vec<Blob>,
    pub discovery_payload: Vec<Blob>,
    pub external_payload: Vec<Blob>,
    pub application_payload: Vec<Blob>,
}

impl AppData {
    /// The low 253 bits of the SHA-256 hash of the four payloads; each is a 4-byte blob count, then for each blob its
    /// 1-byte deletion criterion, its 4-byte length and its bytes.
    pub fn hash(&self) -> Fr {
        let mut packed = Vec::new();
        for payload in [
            &self.resource_payload,
            &self.discovery_payload,
            &self.external_payload,
            &self.application_payload,
        ] {
            packed.extend_from_slice(&length(payload.len()));
            for blob in payload {
                packed.push(blob.deletion_criterion as u8);
                packed.extend_from_slice(&length(blob.bytes.len()));
                packed.extend_from_slice(&blob.bytes);
            }
        }
        low_253_bits(Sha256::digest(&packed).into())
    }
}

/// An action as the contract takes it: the tag, logic ref and app data of each resource, and the root of the
/// commitment tree that holds the consumed resource.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Action {
    pub nullifier: Fr,
    pub consumed_logic_ref: Fr,
    pub consumed_app_data: AppData,
    pub commitment: Fr,
    pub created_logic_ref: Fr,
    pub created_app_data: AppData,
    pub commitment_tree_root: Fr,
}

impl Action {
    pub fn instance(&self) -> ActionInstance {
        ActionInstance {
            nullifier: self.nullifier,
            consumed_logic_ref: self.consumed_logic_ref,
            consumed_app_data_hash: self.consumed_app_data.hash(),
            commitment: self.commitment,
            created_logic_ref: self.created_logic_ref,
            created_app_data_hash: self.created_app_data.hash(),
            commitment_tree_root: self.commitment_tree_root,
        }
    }
}

/// The public data of one action, in the order of the contract's `Action`, with each app data replaced by its hash.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ActionInstance {
    pub nullifier: Fr,
    pub consumed_logic_ref: Fr,
    pub consumed_app_data_hash: Fr,
    pub commitment: Fr,
    pub created_logic_ref: Fr,
    pub created_app_data_hash: Fr,
    pub commitment_tree_root: Fr,
}

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

/// Extends the digest of the actions before an action with the action; the digest of no actions is zero.
pub fn extend(digest: Fr, action: &ActionInstance) -> Fr {
    let mut bytes = Vec::with_capacity(8 * 32);
    for field in [
        digest,
        action.nullifier,
        action.consumed_logic_ref,
        action.consumed_app_data_hash,
        action.commitment,
        action.created_logic_ref,
        action.created_app_data_hash,
        action.commitment_tree_root,
    ] {
        bytes.extend_from_slice(&to_bytes(field));
    }
    low_253_bits(Sha256::digest(&bytes).into())
}

/// The digest of all actions of a transaction, which is the public input of its final proof.
pub fn action_digest(actions: &[ActionInstance]) -> Fr {
    actions.iter().fold(Fr::from(0u64), extend)
}

fn low_253_bits(mut hash: [u8; 32]) -> Fr {
    hash[0] &= 0x1f;
    Fr::from_be_bytes_mod_order(&hash)
}

/// A count or length as 4 big-endian bytes; no payload of a transaction reaches 2^32 bytes.
fn length(value: usize) -> [u8; 4] {
    u32::try_from(value)
        .expect("a payload has fewer than 2^32 blobs and bytes")
        .to_be_bytes()
}
