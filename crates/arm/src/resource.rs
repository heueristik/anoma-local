//! Resources, their commitments and nullifiers, as the compliance circuit computes them.

use crate::field::to_u128;
use crate::hash::{Domain, hash};
use crate::{Error, Fr};

/// The length of the plaintext of a resource: one field element for each field of the resource.
pub const RESOURCE_FIELDS: usize = 8;

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

    /// The plaintext of the resource, as a resource payload carries it: its fields in the order of the struct.
    pub fn to_fields(&self) -> [Fr; RESOURCE_FIELDS] {
        [
            self.logic_ref,
            self.label_ref,
            self.value_ref,
            Fr::from(self.quantity),
            self.nonce,
            self.nullifier_key_commitment,
            Fr::from(self.is_ephemeral),
            self.rand_seed,
        ]
    }

    /// The resource of a plaintext that `to_fields` wrote.
    pub fn from_fields(fields: &[Fr; RESOURCE_FIELDS]) -> Result<Self, Error> {
        let [
            logic_ref,
            label_ref,
            value_ref,
            quantity,
            nonce,
            nullifier_key_commitment,
            is_ephemeral,
            rand_seed,
        ] = *fields;
        let is_ephemeral = match is_ephemeral {
            flag if flag == Fr::from(0u64) => false,
            flag if flag == Fr::from(1u64) => true,
            _ => return Err(Error::InvalidField("is_ephemeral")),
        };
        Ok(Self {
            logic_ref,
            label_ref,
            value_ref,
            quantity: to_u128(quantity).ok_or(Error::InvalidField("quantity"))?,
            nonce,
            nullifier_key_commitment,
            is_ephemeral,
            rand_seed,
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
