//! A solver of intents. It finds the intents whose payloads are for its keys, checks their payloads and proofs, chooses
//! with CP-SAT the most intents that balance together, and proves the actions that fill them and that take the
//! surplus. The result is a balanced partial transaction, which the caller proves and sends.

pub mod matching;

use std::collections::BTreeSet;

use anoma_arm::resource::{NullifierKey, Resource};
use anoma_arm::{Fr, Scalar};
use anoma_discovery::{ReceiverKeys, discover};
use anoma_intent::Payload;
use anoma_intent_pool::PooledIntent;
use anoma_prover::PartialTransaction;
use anoma_prover::action::{self, ActionWitness, Logic};
use anoma_prover::circuit_prover::{CircuitError, CircuitProver};
use ark_ff::UniformRand;
use rand_core::OsRng;

use crate::matching::{KindRef, Order};

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error(transparent)]
    Matching(#[from] matching::Error),
    #[error(transparent)]
    Circuit(#[from] CircuitError),
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

/// A balanced partial transaction, and the ids of the intents that it fills, which the caller removes from the pool
/// once the chain executes the transaction.
pub struct Solution<Id> {
    pub partial: PartialTransaction,
    pub filled: Vec<Id>,
}

/// An intent that the solver can fill: its payload, and its actions as the payload checked them.
struct Candidate<Id> {
    id: Id,
    payload: Payload,
    partial: PartialTransaction,
}

impl Solver {
    /// Fills the most of the intents that balance together. Its actions of ephemeral resources show
    /// `commitment_tree_root`, a recent root of the chain. Returns `None` if it fills none.
    pub fn solve<Id: Copy>(
        &self,
        prover: &impl CircuitProver,
        intents: &[PooledIntent<Id>],
        commitment_tree_root: Fr,
    ) -> Result<Option<Solution<Id>>, Error> {
        let logic_keys = [
            prover.verification_key(&self.token_logic.circuit)?,
            prover.verification_key(anoma_intent::CIRCUIT)?,
        ];
        let (token_logic_ref, intent_logic_ref) = (logic_keys[0].hash, logic_keys[1].hash);
        let mut nullifiers = BTreeSet::new();
        let candidates: Vec<Candidate<Id>> = intents
            .iter()
            .filter_map(|intent| {
                intent
                    .actions
                    .iter()
                    .filter_map(|proved| discover(&proved.action.created_app_data, &self.keys))
                    .flatten()
                    .filter_map(|bytes| Payload::from_bytes(&bytes))
                    .filter(|payload| {
                        payload.offer.logic_ref == token_logic_ref
                            && payload.terms.want_logic_ref == token_logic_ref
                    })
                    .find_map(|payload| {
                        let partial = payload.check(&intent.actions, intent_logic_ref).ok()?;
                        Some(Candidate {
                            id: intent.id,
                            payload,
                            partial,
                        })
                    })
            })
            // An action whose proofs do not verify makes the proof of the whole settlement fail.
            .filter(|candidate| {
                candidate
                    .partial
                    .actions
                    .iter()
                    .all(|proved| proved.verify(prover, &logic_keys).is_ok())
            })
            .filter(|candidate| has_fresh_nullifiers(&candidate.partial, &mut nullifiers))
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
            witnesses.push(self.fill(
                &candidates[index].payload,
                intent_logic_ref,
                commitment_tree_root,
            )?);
        }
        for (&kind, &quantity) in &matching.surplus {
            witnesses.push(self.take_surplus(kind, quantity, commitment_tree_root)?);
        }
        let mut partial = PartialTransaction {
            actions: Vec::new(),
            rcv_total: Scalar::from(0u64),
        };
        for witness in &witnesses {
            partial = partial.compose(action::prove(prover, witness)?);
        }

        let mut filled = Vec::new();
        for (index, candidate) in candidates.into_iter().enumerate() {
            if matching.chosen.contains(&index) {
                partial = partial.compose(candidate.partial);
                filled.push(candidate.id);
            }
        }
        Ok(Some(Solution { partial, filled }))
    }

    /// The action that consumes the intent and creates the resource of its terms for its receiver.
    fn fill(
        &self,
        payload: &Payload,
        intent_logic_ref: Fr,
        commitment_tree_root: Fr,
    ) -> Result<ActionWitness, Error> {
        let mut witness = anoma_intent::fill(
            &payload.intent(intent_logic_ref),
            payload.nullifier_key,
            &payload.terms,
            self.token_logic.clone(),
            commitment_tree_root,
            Scalar::rand(&mut OsRng),
        )?;
        witness.created_app_data =
            anoma_discovery::app_data(&payload.receiver, &witness.created.to_bytes())?;
        Ok(witness)
    }

    /// The action that creates the surplus of a kind for the solver from an ephemeral resource of quantity 0.
    fn take_surplus(
        &self,
        kind: KindRef,
        quantity: u128,
        commitment_tree_root: Fr,
    ) -> Result<ActionWitness, Error> {
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
            commitment_tree_root,
            Scalar::rand(&mut OsRng),
        )?;
        witness.created_app_data =
            anoma_discovery::app_data(&self.keys.receiver(), &witness.created.to_bytes())?;
        Ok(witness)
    }
}

/// Whether no nullifier of the partial transaction repeats, in it or in `seen`; then it adds them to `seen`. A repeated
/// nullifier makes the contract revert the whole transaction.
fn has_fresh_nullifiers(partial: &PartialTransaction, seen: &mut BTreeSet<Fr>) -> bool {
    let mut own = BTreeSet::new();
    let fresh = partial.actions.iter().all(|proved| {
        !seen.contains(&proved.action.nullifier) && own.insert(proved.action.nullifier)
    });
    if fresh {
        seen.append(&mut own);
    }
    fresh
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
        want_quantity: payload.terms.want_quantity,
    }
}
