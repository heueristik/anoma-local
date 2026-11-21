//! A mock intent pool: an in-memory list of intents, each the proved actions of a partial transaction. A solver finds
//! the intents for it by trial decryption of the discovery payloads of their created resources, as the Galileo
//! indexer finds resources. The pool checks no proof and no payload; a solver checks what it takes.

use std::collections::BTreeMap;

use anoma_discovery::{ReceiverKeys, discover};
use anoma_prover::action::ProvedAction;

/// The number of an intent in the pool, in the order of submission.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct IntentId(u64);

/// An intent whose discovery payload is for the keys of a solver, and its resource payloads that the solver decrypts.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Discovered {
    pub id: IntentId,
    pub payloads: Vec<Vec<u8>>,
}

#[derive(Clone, Debug, Default)]
pub struct IntentPool {
    intents: BTreeMap<IntentId, Vec<ProvedAction>>,
    next_id: u64,
}

impl IntentPool {
    pub fn submit(&mut self, actions: Vec<ProvedAction>) -> IntentId {
        let id = IntentId(self.next_id);
        self.next_id += 1;
        self.intents.insert(id, actions);
        id
    }

    /// The intents with a created resource whose discovery payload is for the keys, in the order of submission.
    pub fn discover(&self, keys: &ReceiverKeys) -> Vec<Discovered> {
        self.intents
            .iter()
            .filter_map(|(&id, actions)| {
                let found: Vec<Vec<Vec<u8>>> = actions
                    .iter()
                    .filter_map(|proved| discover(&proved.action.created_app_data, keys))
                    .collect();
                (!found.is_empty()).then(|| Discovered {
                    id,
                    payloads: found.into_iter().flatten().collect(),
                })
            })
            .collect()
    }

    pub fn get(&self, id: IntentId) -> Option<&[ProvedAction]> {
        self.intents.get(&id).map(Vec::as_slice)
    }

    /// Removes an intent, once the chain executes a transaction that fills it.
    pub fn take(&mut self, id: IntentId) -> Option<Vec<ProvedAction>> {
        self.intents.remove(&id)
    }

    pub fn len(&self) -> usize {
        self.intents.len()
    }

    pub fn is_empty(&self) -> bool {
        self.intents.is_empty()
    }
}
