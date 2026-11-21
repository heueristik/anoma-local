//! A solver of intents. It finds the intents for its keys in an intent pool, checks their payloads, chooses with
//! CP-SAT the most intents that balance together, and proves the actions that fill them and that take the surplus.
//! The result is a balanced partial transaction, which the solver proves and sends.

pub mod matching;

use anoma_arm::resource::{NullifierKey, Resource};
use anoma_arm::{Fr, Scalar};
use anoma_discovery::ReceiverKeys;
use anoma_intent::Payload;
use anoma_intent_pool::{IntentId, IntentPool};
use anoma_prover::PartialTransaction;
use anoma_prover::action::{self, ActionWitness, Logic};
use anoma_prover::toolchain::Toolchain;
use ark_ff::UniformRand;
use rand_core::OsRng;

use crate::matching::{KindRef, Order};

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error(transparent)]
    Matching(#[from] matching::Error),
    #[error(transparent)]
    Prove(#[from] anoma_prover::Error),
    #[error(transparent)]
    Discovery(#[from] anoma_discovery::Error),
}

/// A solver: its keys, the owner of its surplus, and the logic of the tokens that it trades.
pub struct Solver {
    pub keys: ReceiverKeys,
    pub owner: NullifierKey,
    /// The solver skips intents of tokens with other logics.
    pub token_logic: Logic,
}

/// A balanced partial transaction, and the intents of the pool that it fills.
pub struct Solution {
    pub partial: PartialTransaction,
    pub filled: Vec<IntentId>,
}

/// An intent that the solver can fill: its payload, and its actions as the payload checked them.
struct Candidate {
    id: IntentId,
    payload: Payload,
    partial: PartialTransaction,
}

impl Solver {
    /// Fills the most intents of the pool that balance together, and takes them from the pool. Returns `None` if it
    /// fills none.
    pub fn solve(
        &self,
        toolchain: &Toolchain,
        pool: &mut IntentPool,
    ) -> Result<Option<Solution>, Error> {
        let token_logic_ref = toolchain.key_hash(&self.token_logic.circuit)?;
        let intent_logic_ref = toolchain.key_hash(anoma_intent::CIRCUIT)?;
        let candidates: Vec<Candidate> = pool
            .discover(&self.keys)
            .into_iter()
            .filter_map(|discovered| {
                let actions = pool.get(discovered.id)?;
                discovered
                    .payloads
                    .iter()
                    .filter_map(|bytes| Payload::from_bytes(bytes))
                    .filter(|payload| {
                        payload.offer.logic_ref == token_logic_ref
                            && payload.terms.want_logic_ref == token_logic_ref
                    })
                    .find_map(|payload| {
                        let partial = payload.check(actions, intent_logic_ref).ok()?;
                        Some(Candidate {
                            id: discovered.id,
                            payload,
                            partial,
                        })
                    })
            })
            .collect();
        let orders: Vec<Order> = candidates
            .iter()
            .map(|candidate| order(&candidate.payload))
            .collect();
        let matching = matching::solve(&orders)?;
        if matching.chosen.is_empty() {
            return Ok(None);
        }

        let mut witnesses = Vec::new();
        for &index in &matching.chosen {
            witnesses.push(self.fill(&candidates[index].payload, intent_logic_ref)?);
        }
        for (&kind, &quantity) in &matching.surplus {
            witnesses.push(self.take_surplus(kind, quantity)?);
        }
        let mut partial = PartialTransaction {
            actions: Vec::new(),
            rcv_total: Scalar::from(0u64),
        };
        for witness in &witnesses {
            partial = partial.compose(action::prove(toolchain, witness)?);
        }

        let mut filled = Vec::new();
        for (index, candidate) in candidates.into_iter().enumerate() {
            if matching.chosen.contains(&index) {
                pool.take(candidate.id);
                partial = partial.compose(candidate.partial);
                filled.push(candidate.id);
            }
        }
        Ok(Some(Solution { partial, filled }))
    }

    /// The action that consumes the intent and creates the resource of its terms for its receiver.
    fn fill(&self, payload: &Payload, intent_logic_ref: Fr) -> Result<ActionWitness, Error> {
        let mut witness = anoma_intent::fill(
            &payload.intent(intent_logic_ref),
            payload.nullifier_key,
            &payload.terms,
            self.token_logic.clone(),
            Scalar::rand(&mut OsRng),
        )?;
        witness.created_app_data =
            anoma_discovery::app_data(&payload.receiver, &witness.created.to_bytes())?;
        Ok(witness)
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
            rand_seed: Fr::rand(&mut OsRng),
        };
        let mut witness = ActionWitness::from_ephemeral(
            created,
            0,
            NullifierKey(Fr::rand(&mut OsRng)),
            self.token_logic.clone(),
            Scalar::rand(&mut OsRng),
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
