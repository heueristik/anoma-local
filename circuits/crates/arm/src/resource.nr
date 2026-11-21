use crate::hash::{self, hash};

/// A resource of the resource machine, with every field as a BN254 field element.
pub struct Resource {
    pub logic_ref: Field,
    pub label_ref: Field,
    pub value_ref: Field,
    pub quantity: u128,
    pub nonce: Field,
    pub nullifier_key_commitment: Field,
    pub is_ephemeral: bool,
    pub rand_seed: Field,
}

impl Resource {
    pub fn commitment(self) -> Field {
        let rcm = hash(hash::RCM, [self.rand_seed, self.nonce]);
        hash(
            hash::COMMITMENT,
            [
                self.logic_ref,
                self.label_ref,
                self.value_ref,
                self.quantity as Field,
                self.nonce,
                self.nullifier_key_commitment,
                self.is_ephemeral as Field,
                rcm,
            ],
        )
    }

    pub fn nullifier(self, nullifier_key: Field) -> Field {
        assert(
            self.nullifier_key_commitment == nullifier_key_commitment(nullifier_key),
            "the nullifier key does not open the nullifier key commitment of the resource",
        );
        let psi = hash(hash::PSI, [self.rand_seed, self.nonce]);
        hash(
            hash::NULLIFIER,
            [nullifier_key, self.nonce, psi, self.commitment()],
        )
    }
}

pub fn nullifier_key_commitment(nullifier_key: Field) -> Field {
    hash(hash::NULLIFIER_KEY, [nullifier_key])
}

/// The nonce of a created resource, which the nullifier of the consumed resource of its compliance unit makes unique.
pub fn created_nonce(nullifier: Field) -> Field {
    hash(hash::NONCE, [nullifier])
}

#[test]
fn commitment_changes_with_the_quantity() {
    let resource = test_resource();
    let mut other = resource;
    other.quantity += 1;
    assert(resource.commitment() != other.commitment());
}

#[test]
fn nullifier_changes_with_the_nonce() {
    let resource = test_resource();
    let mut other = resource;
    other.nonce += 1;
    assert(resource.nullifier(TEST_NULLIFIER_KEY) != other.nullifier(TEST_NULLIFIER_KEY));
}

#[test(should_fail_with = "the nullifier key does not open the nullifier key commitment of the resource")]
fn nullifier_fails_with_another_nullifier_key() {
    let _ = test_resource().nullifier(TEST_NULLIFIER_KEY + 1);
}

global TEST_NULLIFIER_KEY: Field = 42;

fn test_resource() -> Resource {
    Resource {
        logic_ref: 1,
        label_ref: 2,
        value_ref: 3,
        quantity: 4,
        nonce: 5,
        nullifier_key_commitment: nullifier_key_commitment(TEST_NULLIFIER_KEY),
        is_ephemeral: false,
        rand_seed: 6,
    }
}
