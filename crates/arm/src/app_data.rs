//! The app data of a resource, as the contract takes it, and its hash, which the resource logic takes.

use sha2::{Digest, Sha256};

use crate::Fr;
use crate::field::low_253_bits;

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

/// A count or length as 4 big-endian bytes; no payload of a transaction reaches 2^32 bytes.
fn length(value: usize) -> [u8; 4] {
    u32::try_from(value)
        .expect("a payload has fewer than 2^32 blobs and bytes")
        .to_be_bytes()
}
