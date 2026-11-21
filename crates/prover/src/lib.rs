//! Proves the actions and transactions of the Anoma local circuits with nargo, bb and the Groth16 wrap.
//!
//! An action becomes a partial transaction of one action. Partial transactions compose, and a balanced one gets a
//! final proof that the contract checks.

pub mod action;
pub mod circuit_prover;
mod error;
pub mod inputs;
pub mod toolchain;
pub mod transaction;

pub use error::Error;
pub use transaction::{PartialTransaction, Transaction};
