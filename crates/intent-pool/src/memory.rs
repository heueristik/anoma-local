//! An intent pool in memory, for tests and for a solver that gets its intents directly.

use std::collections::BTreeMap;
use std::convert::Infallible;

use anoma_prover::action::ProvedAction;

use crate::{IntentPool, PooledIntent};

/// The number of an intent in a memory pool, in the order of submission.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct MemoryIntentId(u64);

#[derive(Clone, Debug, Default)]
pub struct MemoryIntentPool {
    intents: BTreeMap<MemoryIntentId, Vec<ProvedAction>>,
    next_id: u64,
}

impl IntentPool for MemoryIntentPool {
    type IntentId = MemoryIntentId;
    type Error = Infallible;

    async fn submit(&mut self, actions: Vec<ProvedAction>) -> Result<MemoryIntentId, Infallible> {
        let id = MemoryIntentId(self.next_id);
        self.next_id += 1;
        self.intents.insert(id, actions);
        Ok(id)
    }

    async fn intents(&self) -> Result<Vec<PooledIntent<MemoryIntentId>>, Infallible> {
        Ok(self
            .intents
            .iter()
            .map(|(&id, actions)| PooledIntent {
                id,
                actions: actions.clone(),
            })
            .collect())
    }

    async fn remove(&mut self, id: MemoryIntentId) -> Result<(), Infallible> {
        self.intents.remove(&id);
        Ok(())
    }
}
