//! The intent resource: an ephemeral resource whose label is the hash of its terms. Its creator gives up what the
//! action consumes; the action that consumes the intent must create a resource that fills the terms.

mod payload;

pub use payload::{Mismatch, Offer, Payload};

use anoma_arm::Fr;
use anoma_arm::hash::poseidon2_hash;
use anoma_arm::resource::{NullifierKey, Resource};
use anoma_prover::action::Logic;
use anoma_prover::inputs::{self, Field, Quantity};
use serde::Serialize;

/// The circuit of the intent logic.
pub const CIRCUIT: &str = "intent_logic";

/// What the creator of an intent wants: at least `min_quantity` of one kind, owned by one nullifier key.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Terms {
    pub want_logic_ref: Fr,
    pub want_label_ref: Fr,
    pub min_quantity: u128,
    pub receiver_nullifier_key_commitment: Fr,
}

impl Terms {
    /// The label of an intent with these terms, as the intent logic computes it.
    pub fn hash(&self) -> Fr {
        poseidon2_hash(&[
            self.want_logic_ref,
            self.want_label_ref,
            Fr::from(self.min_quantity),
            self.receiver_nullifier_key_commitment,
        ])
    }

    /// An intent with these terms: ephemeral, of quantity 1, with `logic_ref` as the key hash of the intent logic.
    pub fn intent(
        &self,
        logic_ref: Fr,
        nullifier_key: NullifierKey,
        nonce: Fr,
        rand_seed: Fr,
    ) -> Resource {
        Resource {
            logic_ref,
            label_ref: self.hash(),
            value_ref: Fr::from(0u64),
            quantity: 1,
            nonce,
            nullifier_key_commitment: nullifier_key.commitment(),
            is_ephemeral: true,
            rand_seed,
        }
    }
}

/// The logic of a created intent. It checks nothing, but nargo needs a value for each private input.
pub fn created_logic() -> Result<Logic, toml::ser::Error> {
    let zero = Resource {
        logic_ref: Fr::from(0u64),
        label_ref: Fr::from(0u64),
        value_ref: Fr::from(0u64),
        quantity: 0,
        nonce: Fr::from(0u64),
        nullifier_key_commitment: Fr::from(0u64),
        is_ephemeral: false,
        rand_seed: Fr::from(0u64),
    };
    let terms = Terms {
        want_logic_ref: Fr::from(0u64),
        want_label_ref: Fr::from(0u64),
        min_quantity: 0,
        receiver_nullifier_key_commitment: Fr::from(0u64),
    };
    consumed_logic(&zero, NullifierKey(Fr::from(0u64)), &terms, &zero)
}

/// The logic of a consumed intent: the intent opens its nullifier, and `created` is the created resource of the
/// action.
pub fn consumed_logic(
    intent: &Resource,
    nullifier_key: NullifierKey,
    terms: &Terms,
    created: &Resource,
) -> Result<Logic, toml::ser::Error> {
    let witness = Witness {
        intent: intent.into(),
        nullifier_key: Field(nullifier_key.0),
        terms: TermsInput {
            want_logic_ref: Field(terms.want_logic_ref),
            want_label_ref: Field(terms.want_label_ref),
            min_quantity: Quantity(terms.min_quantity),
            receiver_nullifier_key_commitment: Field(terms.receiver_nullifier_key_commitment),
        },
        created: created.into(),
    };
    Ok(Logic {
        circuit: CIRCUIT.to_string(),
        witness: toml::Table::try_from(witness)?,
    })
}

/// The private inputs of the intent logic.
#[derive(Serialize)]
struct Witness {
    intent: inputs::Resource,
    nullifier_key: Field,
    terms: TermsInput,
    created: inputs::Resource,
}

#[derive(Serialize)]
struct TermsInput {
    want_logic_ref: Field,
    want_label_ref: Field,
    min_quantity: Quantity,
    receiver_nullifier_key_commitment: Field,
}
