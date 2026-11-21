//! An action as the contract takes it, and the root of its action tree.

use crate::Fr;
use crate::app_data::AppData;
use crate::digest::ActionInstance;
use crate::hash::{Domain, hash};

/// An action as the contract takes it: the tag, logic ref and app data of each resource, and the root of the
/// commitment tree that holds the consumed resource.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Action {
    pub nullifier: Fr,
    pub consumed_logic_ref: Fr,
    pub consumed_app_data: AppData,
    pub commitment: Fr,
    pub created_logic_ref: Fr,
    pub created_app_data: AppData,
    pub commitment_tree_root: Fr,
}

impl Action {
    pub fn instance(&self) -> ActionInstance {
        ActionInstance {
            nullifier: self.nullifier,
            consumed_logic_ref: self.consumed_logic_ref,
            consumed_app_data_hash: self.consumed_app_data.hash(),
            commitment: self.commitment,
            created_logic_ref: self.created_logic_ref,
            created_app_data_hash: self.created_app_data.hash(),
            commitment_tree_root: self.commitment_tree_root,
        }
    }
}

/// The root of the action tree of a compliance unit: the hash of its two tags.
pub fn action_tree_root(nullifier: Fr, commitment: Fr) -> Fr {
    hash(Domain::ActionTree, &[nullifier, commitment])
}
