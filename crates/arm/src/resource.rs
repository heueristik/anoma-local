//! Resources, their commitments and nullifiers, as the compliance circuit computes them.

use crate::field::{from_bytes, to_bytes};
use crate::hash::{Domain, hash};
use crate::{Error, Fr};

/// The length of the plaintext of a resource: its eight fields as 32-byte big-endian words.
pub const RESOURCE_BYTES: usize = 8 * 32;

/// A resource of the resource machine, with every field as a BN254 field element.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct Resource {
    pub logic_ref: Fr,
    pub label_ref: Fr,
    pub value_ref: Fr,
    pub quantity: u128,
    pub nonce: Fr,
    pub nullifier_key_commitment: Fr,
    pub is_ephemeral: bool,
    pub rand_seed: Fr,
}

impl Resource {
    pub fn commitment(&self) -> Fr {
        let rcm = hash(Domain::Rcm, &[self.rand_seed, self.nonce]);
        hash(
            Domain::Commitment,
            &[
                self.logic_ref,
                self.label_ref,
                self.value_ref,
                Fr::from(self.quantity),
                self.nonce,
                self.nullifier_key_commitment,
                Fr::from(self.is_ephemeral),
                rcm,
            ],
        )
    }

    /// The plaintext of the resource, as a resource payload carries it: each field as a 32-byte big-endian word, in
    /// the order of the struct.
    pub fn to_bytes(&self) -> [u8; RESOURCE_BYTES] {
        let words = [
            self.logic_ref,
            self.label_ref,
            self.value_ref,
            Fr::from(self.quantity),
            self.nonce,
            self.nullifier_key_commitment,
            Fr::from(self.is_ephemeral),
            self.rand_seed,
        ];
        let mut bytes = [0; RESOURCE_BYTES];
        for (chunk, word) in bytes.as_chunks_mut::<32>().0.iter_mut().zip(words) {
            *chunk = to_bytes(word);
        }
        bytes
    }

    /// The resource of a plaintext that `to_bytes` wrote.
    pub fn from_bytes(bytes: &[u8; RESOURCE_BYTES]) -> Result<Self, Error> {
        let (words, _) = bytes.as_chunks::<32>();
        let field = |index: usize| from_bytes(words[index]);
        let quantity = words[3];
        if quantity[..16] != [0; 16] {
            return Err(Error::InvalidWord("quantity"));
        }
        let is_ephemeral = match words[6] {
            word if word == [0; 32] => false,
            word if word == to_bytes(Fr::from(1u64)) => true,
            _ => return Err(Error::InvalidWord("is_ephemeral")),
        };
        Ok(Self {
            logic_ref: field(0)?,
            label_ref: field(1)?,
            value_ref: field(2)?,
            quantity: u128::from_be_bytes(quantity[16..].try_into().expect("16 bytes")),
            nonce: field(4)?,
            nullifier_key_commitment: field(5)?,
            is_ephemeral,
            rand_seed: field(7)?,
        })
    }

    /// The nullifier, which only the key that opens the nullifier key commitment computes.
    pub fn nullifier(&self, key: NullifierKey) -> Result<Fr, Error> {
        if key.commitment() != self.nullifier_key_commitment {
            return Err(Error::NullifierKeyMismatch);
        }
        let psi = hash(Domain::Psi, &[self.rand_seed, self.nonce]);
        Ok(hash(
            Domain::Nullifier,
            &[key.0, self.nonce, psi, self.commitment()],
        ))
    }
}

/// The secret that lets its holder compute the nullifiers of the resources that commit to it.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct NullifierKey(pub Fr);

impl NullifierKey {
    pub fn commitment(&self) -> Fr {
        hash(Domain::NullifierKey, &[self.0])
    }
}

/// The nonce of a created resource, which the nullifier of the consumed resource of its compliance unit makes unique.
pub fn created_nonce(nullifier: Fr) -> Fr {
    hash(Domain::Nonce, &[nullifier])
}
