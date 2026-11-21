//! The port to an intent pool, which holds intents until the chain executes a transaction that fills them, and a pool
//! in memory. An intent is the proved actions of an unbalanced partial transaction. A pool checks no proof and no
//! payload; a solver checks what it takes.

mod memory;

use std::fmt::Debug;
use std::future::Future;

use anoma_prover::action::ProvedAction;

pub use memory::{MemoryIntentId, MemoryIntentPool};

/// An intent in a pool: its id in the pool, and its proved actions.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct PooledIntent<Id> {
    pub id: Id,
    pub actions: Vec<ProvedAction>,
}

/// Holds the intents that parties submit, for the solvers.
pub trait IntentPool {
    type IntentId: Copy + Ord + Debug;
    type Error: std::error::Error + Send + Sync + 'static;

    fn submit(
        &mut self,
        actions: Vec<ProvedAction>,
    ) -> impl Future<Output = Result<Self::IntentId, Self::Error>> + Send;

    /// All intents of the pool, in the order of submission.
    fn intents(
        &self,
    ) -> impl Future<Output = Result<Vec<PooledIntent<Self::IntentId>>, Self::Error>> + Send;

    /// Removes an intent, once the chain executes a transaction that fills it. The removal of an unknown intent does
    /// nothing.
    fn remove(
        &mut self,
        id: Self::IntentId,
    ) -> impl Future<Output = Result<(), Self::Error>> + Send;
}
