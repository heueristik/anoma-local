//! The trait of a resource logic: its circuit, and the private inputs that it takes for a resource of an action.

use std::fmt::Debug;

use anoma_arm::Fr;
use anoma_arm::resource::Resource;

use crate::Error;
use crate::action::{ActionWitness, ResourceWitness};
use crate::circuit_prover::CircuitInputs;

/// The action of a resource whose logic a party proves, the position of the resource in it, and the root of its action
/// tree.
#[derive(Clone, Copy, Debug)]
pub struct LogicContext<'a> {
    action: &'a ActionWitness,
    index: usize,
    action_tree_root: Fr,
}

impl<'a> LogicContext<'a> {
    /// The context of resource `index` of `action`, which must be a position in the action.
    pub(crate) fn new(action: &'a ActionWitness, index: usize, action_tree_root: Fr) -> Self {
        Self {
            action,
            index,
            action_tree_root,
        }
    }

    pub fn action(&self) -> &'a ActionWitness {
        self.action
    }

    /// The position of the resource in the action.
    pub fn index(&self) -> usize {
        self.index
    }

    pub fn action_tree_root(&self) -> Fr {
        self.action_tree_root
    }

    /// The private data of the resource whose logic the party proves.
    pub fn witness(&self) -> &'a ResourceWitness {
        &self.action.resources[self.index]
    }

    pub fn resource(&self) -> &'a Resource {
        &self.witness().resource
    }

    pub fn is_consumed(&self) -> bool {
        self.witness().is_consumed()
    }
}

/// A resource logic. The key hash of its circuit is the logic ref of its resources.
pub trait ResourceLogic: Debug + Send + Sync {
    fn circuit(&self) -> &str;

    /// The inputs of the circuit besides `tag`, `is_consumed`, `action_tree_root` and `app_data_hash`.
    fn private_inputs(&self, context: &LogicContext) -> Result<CircuitInputs, Error>;
}

/// The logic that accepts every resource, for test resources whose only owner check is the nullifier key.
#[derive(Clone, Copy, Debug, Default)]
pub struct TrivialLogic;

impl ResourceLogic for TrivialLogic {
    fn circuit(&self) -> &str {
        "trivial_logic"
    }

    fn private_inputs(&self, _context: &LogicContext) -> Result<CircuitInputs, Error> {
        Ok(CircuitInputs::new())
    }
}
