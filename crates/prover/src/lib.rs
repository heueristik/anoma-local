//! Proves the actions and transactions of the Anoma local circuits through two traits: a circuit prover and a wrapper
//! of the root proof.
//!
//! An action becomes a partial transaction of one action. Partial transactions compose, and a balanced one gets a
//! final proof that the contract checks.

pub mod action;
pub mod circuit_prover;
pub mod cli_prover;
mod error;
pub mod inputs;
pub mod resource_logic;
pub mod root_proof_wrapper;
pub mod transaction;

pub use error::Error;
pub use transaction::{PartialTransaction, Transaction};
