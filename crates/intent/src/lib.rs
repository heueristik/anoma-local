//! The intent resource: an ephemeral resource whose label is the hash of its terms. Its creator gives up what the
//! action consumes; the action that consumes the intent must create the resource of its terms.

mod payload;

pub use payload::{Mismatch, Offer, Payload};

use anoma_arm::digest::AppData;
use anoma_arm::hash::{Domain, hash};
use anoma_arm::resource::{NullifierKey, Resource, created_nonce};
use anoma_arm::{Fr, Scalar};
use anoma_prover::Error;
use anoma_prover::action::{ActionWitness, Logic, Membership};
use anoma_prover::inputs::{self, Field, Quantity};
use serde::Serialize;

/// The circuit of the intent logic.
pub const CIRCUIT: &str = "intent_logic";

/// What the creator of an intent wants: each field of the resource that fills it, except the nonce, which comes from
/// the nullifier of the intent.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Terms {
    pub want_logic_ref: Fr,
    pub want_label_ref: Fr,
    pub want_value_ref: Fr,
    pub want_quantity: u128,
    pub receiver_nullifier_key_commitment: Fr,
    pub want_rand_seed: Fr,
}

impl Terms {
    /// The label of an intent with these terms, as the intent logic computes it.
    pub fn hash(&self) -> Fr {
        hash(
            Domain::Terms,
            &[
                self.want_logic_ref,
                self.want_label_ref,
                self.want_value_ref,
                Fr::from(self.want_quantity),
                self.receiver_nullifier_key_commitment,
                self.want_rand_seed,
            ],
        )
    }

    /// The resource of these terms for the intent with the nullifier `intent_nullifier`: the resource that fills it.
    pub fn resource(&self, intent_nullifier: Fr) -> Resource {
        Resource {
            logic_ref: self.want_logic_ref,
            label_ref: self.want_label_ref,
            value_ref: self.want_value_ref,
            quantity: self.want_quantity,
            nonce: created_nonce(intent_nullifier),
            nullifier_key_commitment: self.receiver_nullifier_key_commitment,
            is_ephemeral: false,
            rand_seed: self.want_rand_seed,
        }
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

/// The action that consumes `intent` and creates the resource of its terms with `created_logic`. It shows the
/// historical root `commitment_tree_root`.
pub fn fill(
    intent: &Resource,
    nullifier_key: NullifierKey,
    terms: &Terms,
    created_logic: Logic,
    commitment_tree_root: Fr,
    rcv: Scalar,
) -> Result<ActionWitness, Error> {
    let created = terms.resource(intent.nullifier(nullifier_key)?);
    Ok(ActionWitness {
        consumed: *intent,
        consumed_logic: consumed_logic(intent, nullifier_key, terms, &created)?,
        consumed_app_data: AppData::default(),
        nullifier_key,
        membership: Membership::Ephemeral {
            root: commitment_tree_root,
        },
        created,
        created_logic,
        created_app_data: AppData::default(),
        rcv,
    })
}

/// The logic of a created intent. It checks nothing, but nargo needs a value for each private input.
pub fn created_logic() -> Result<Logic, Error> {
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
        want_value_ref: Fr::from(0u64),
        want_quantity: 0,
        receiver_nullifier_key_commitment: Fr::from(0u64),
        want_rand_seed: Fr::from(0u64),
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
) -> Result<Logic, Error> {
    let witness = Witness {
        intent: intent.into(),
        nullifier_key: Field(nullifier_key.0),
        terms: TermsInput {
            want_logic_ref: Field(terms.want_logic_ref),
            want_label_ref: Field(terms.want_label_ref),
            want_value_ref: Field(terms.want_value_ref),
            want_quantity: Quantity(terms.want_quantity),
            receiver_nullifier_key_commitment: Field(terms.receiver_nullifier_key_commitment),
            want_rand_seed: Field(terms.want_rand_seed),
        },
        created: created.into(),
    };
    Ok(Logic {
        circuit: CIRCUIT.to_string(),
        witness: toml::Table::try_from(witness).map_err(|source| Error::Inputs {
            circuit: CIRCUIT.to_string(),
            source,
        })?,
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
    want_value_ref: Field,
    want_quantity: Quantity,
    receiver_nullifier_key_commitment: Field,
    want_rand_seed: Field,
}
