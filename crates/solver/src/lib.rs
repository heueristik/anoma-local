//! A solver of intents. It finds the intents for its keys in an intent pool, checks their payloads, chooses with
//! CP-SAT the most intents that balance together, and proves the actions that fill them and that take the surplus.
//! The result is a balanced partial transaction, which the solver proves and sends.

pub mod matching;

use std::collections::BTreeMap;

use anoma_arm::digest::AppData;
use anoma_arm::resource::{NullifierKey, Resource, created_nonce};
use anoma_arm::{Fr, Scalar};
use anoma_discovery::ReceiverKeys;
use anoma_intent::Payload;
use anoma_intent_pool::{IntentId, IntentPool};
use anoma_prover::PartialTransaction;
use anoma_prover::action::{self, ActionWitness, Logic, ProvedAction};
use anoma_prover::toolchain::Toolchain;
use ark_ff::PrimeField;
use rand_core::{OsRng, RngCore};

use crate::matching::{KindRef, Order};

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error(transparent)]
    Matching(#[from] matching::Error),
    #[error(transparent)]
    Prove(#[from] anoma_prover::Error),
    #[error(transparent)]
    Arm(#[from] anoma_arm::Error),
    #[error(transparent)]
    Discovery(#[from] anoma_discovery::Error),
    #[error("cannot write the inputs of the intent logic: {0}")]
    Inputs(#[from] toml::ser::Error),
}

/// A solver: its keys, the owner of its surplus, and the logics that it proves.
pub struct Solver {
    pub keys: ReceiverKeys,
    pub owner: NullifierKey,
    pub intent_logic_ref: Fr,
    /// The logic of the tokens that the solver trades; it skips intents of other tokens.
    pub token_logic: Logic,
    pub token_logic_ref: Fr,
}

/// A balanced partial transaction, and the intents of the pool that it fills.
pub struct Solution {
    pub partial: PartialTransaction,
    pub filled: Vec<IntentId>,
}

/// An intent that the solver can fill: its actions and what its payload says about them.
struct Candidate {
    id: IntentId,
    actions: Vec<ProvedAction>,
    payload: Payload,
}

impl Solver {
    /// Fills the most intents of the pool that balance together, and takes them from the pool. Returns `None` if it
    /// fills none.
    pub fn solve(
        &self,
        toolchain: &Toolchain,
        pool: &mut IntentPool,
    ) -> Result<Option<Solution>, Error> {
        let candidates: Vec<Candidate> = pool
            .discover(&self.keys)
            .into_iter()
            .filter_map(|discovered| {
                let actions = pool.get(discovered.id)?;
                discovered
                    .payloads
                    .iter()
                    .filter_map(|bytes| Payload::from_bytes(bytes))
                    .find(|payload| {
                        self.can_fill(payload)
                            && payload.check(actions, self.intent_logic_ref).is_ok()
                    })
                    .map(|payload| Candidate {
                        id: discovered.id,
                        actions: actions.to_vec(),
                        payload,
                    })
            })
            .collect();
        let orders: Vec<Order> = candidates
            .iter()
            .map(|candidate| order(&candidate.payload))
            .collect();
        let chosen: Vec<&Candidate> = matching::solve(&orders)?
            .into_iter()
            .map(|index| &candidates[index])
            .collect();
        if chosen.is_empty() {
            return Ok(None);
        }

        // The given and wanted quantities of each kind; the matching makes the given ones at least the wanted ones.
        let mut totals: BTreeMap<KindRef, (u128, u128)> = BTreeMap::new();
        let mut witnesses = Vec::new();
        for candidate in &chosen {
            let order = order(&candidate.payload);
            totals.entry(order.give).or_default().0 += order.give_quantity;
            totals.entry(order.want).or_default().1 += order.min_quantity;
            witnesses.push(self.fill(&candidate.payload)?);
        }
        for (kind, (given, wanted)) in totals {
            if let Some(surplus) = given.checked_sub(wanted).filter(|&surplus| surplus > 0) {
                witnesses.push(self.take_surplus(kind, surplus)?);
            }
        }

        let mut partial = PartialTransaction {
            actions: Vec::new(),
            rcv_total: Scalar::from(0u64),
        };
        for witness in &witnesses {
            partial = partial.compose(action::prove(toolchain, witness)?);
        }
        let mut filled = Vec::new();
        for candidate in chosen {
            pool.take(candidate.id);
            partial = partial.compose(PartialTransaction {
                actions: candidate.actions.clone(),
                rcv_total: candidate.payload.rcv_total,
            });
            filled.push(candidate.id);
        }
        Ok(Some(Solution { partial, filled }))
    }

    fn can_fill(&self, payload: &Payload) -> bool {
        payload.offer.logic_ref == self.token_logic_ref
            && payload.terms.want_logic_ref == self.token_logic_ref
    }

    /// The action that consumes the intent and creates its minimum for its receiver.
    fn fill(&self, payload: &Payload) -> Result<ActionWitness, Error> {
        let terms = &payload.terms;
        let created = Resource {
            logic_ref: terms.want_logic_ref,
            label_ref: terms.want_label_ref,
            value_ref: Fr::from(0u64),
            quantity: terms.min_quantity,
            nonce: created_nonce(payload.intent.nullifier(payload.nullifier_key)?),
            nullifier_key_commitment: terms.receiver_nullifier_key_commitment,
            is_ephemeral: false,
            rand_seed: random(),
        };
        Ok(ActionWitness {
            consumed: payload.intent,
            consumed_logic: anoma_intent::consumed_logic(
                &payload.intent,
                payload.nullifier_key,
                terms,
                &created,
            )?,
            consumed_app_data: AppData::default(),
            nullifier_key: payload.nullifier_key,
            path: None,
            created,
            created_logic: self.token_logic.clone(),
            created_app_data: anoma_discovery::app_data(&payload.receiver, &created.to_bytes())?,
            rcv: random(),
        })
    }

    /// The action that creates the surplus of a kind for the solver from an ephemeral resource of quantity 0.
    fn take_surplus(&self, kind: KindRef, quantity: u128) -> Result<ActionWitness, Error> {
        let created = Resource {
            logic_ref: kind.logic_ref,
            label_ref: kind.label_ref,
            value_ref: Fr::from(0u64),
            quantity,
            nonce: Fr::from(0u64),
            nullifier_key_commitment: self.owner.commitment(),
            is_ephemeral: false,
            rand_seed: random(),
        };
        let mut witness = ActionWitness::from_ephemeral(
            created,
            0,
            NullifierKey(random()),
            self.token_logic.clone(),
            random(),
        )?;
        witness.created_app_data =
            anoma_discovery::app_data(&self.keys.receiver(), &witness.created.to_bytes())?;
        Ok(witness)
    }
}

fn order(payload: &Payload) -> Order {
    Order {
        give: KindRef {
            logic_ref: payload.offer.logic_ref,
            label_ref: payload.offer.label_ref,
        },
        give_quantity: payload.offer.quantity,
        want: KindRef {
            logic_ref: payload.terms.want_logic_ref,
            label_ref: payload.terms.want_label_ref,
        },
        min_quantity: payload.terms.min_quantity,
    }
}

/// A uniform field element: 512 random bits reduced modulo the field size.
fn random<F: PrimeField>() -> F {
    let mut bytes = [0; 64];
    OsRng.fill_bytes(&mut bytes);
    F::from_le_bytes_mod_order(&bytes)
}
