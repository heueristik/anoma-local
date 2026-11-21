//! The intent resource: an ephemeral resource whose label is the hash of its terms. Its creator gives up what the
//! action consumes; the action that consumes the intent must create the resource of its terms.

mod payload;

pub use payload::{Mismatch, Offer, Payload};

use std::sync::Arc;

use anoma_arm::app_data::AppData;
use anoma_arm::hash::{Domain, hash};
use anoma_arm::resource::{NullifierKey, Resource, created_nonce};
use anoma_arm::{Fr, Scalar};
use anoma_prover::Error;
use anoma_prover::action::{ActionWitness, Membership};
use anoma_prover::circuit_prover::CircuitInputs;
use anoma_prover::inputs::{self, Field, Quantity, circuit_inputs};
use anoma_prover::resource_logic::{LogicContext, ResourceLogic};
use serde::Serialize;

/// The circuit of the intent logic.
pub const CIRCUIT: &str = "intent_logic";

/// What the creator of an intent wants: each field of the resource that fills it, except the nonce, which comes from
/// the nullifier of the intent.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
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

    /// The resource that fills the intent of these terms whose nullifier is `intent_nullifier`.
    pub fn filling_resource(&self, intent_nullifier: Fr) -> Resource {
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

/// The logic of an intent with `terms`. A consumed intent opens its nullifier, and its action creates the resource of
/// its terms.
#[derive(Clone, Copy, Debug)]
pub struct IntentLogic {
    pub terms: Terms,
}

impl ResourceLogic for IntentLogic {
    fn circuit(&self) -> &str {
        CIRCUIT
    }

    fn private_inputs(&self, context: &LogicContext) -> Result<CircuitInputs, Error> {
        // The logic checks nothing for a created intent, but nargo needs a value for each private input.
        let witness = if context.is_consumed {
            Witness::new(
                &context.action.consumed,
                context.action.nullifier_key,
                &self.terms,
                &context.action.created,
            )
        } else {
            let zero = Resource::default();
            Witness::new(
                &zero,
                NullifierKey(Fr::from(0u64)),
                &Terms::default(),
                &zero,
            )
        };
        circuit_inputs(CIRCUIT, &witness)
    }
}

/// The action that consumes `intent` and creates the resource of its terms with `created_logic`. It shows the
/// historical root `commitment_tree_root`.
pub fn fill(
    intent: &Resource,
    nullifier_key: NullifierKey,
    terms: &Terms,
    created_logic: Arc<dyn ResourceLogic>,
    commitment_tree_root: Fr,
    rcv: Scalar,
) -> Result<ActionWitness, Error> {
    let created = terms.filling_resource(intent.nullifier(nullifier_key)?);
    Ok(ActionWitness {
        consumed: *intent,
        consumed_logic: Arc::new(IntentLogic { terms: *terms }),
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

/// The private inputs of the intent logic.
#[derive(Serialize)]
struct Witness {
    intent: inputs::Resource,
    nullifier_key: Field,
    terms: TermsInput,
    created: inputs::Resource,
}

impl Witness {
    fn new(
        intent: &Resource,
        nullifier_key: NullifierKey,
        terms: &Terms,
        created: &Resource,
    ) -> Self {
        Self {
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
        }
    }
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
