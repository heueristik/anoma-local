//! The trait of a resource logic: its circuit, and the private inputs that it takes for a resource of an action.

use std::fmt::Debug;

use anoma_arm::Fr;
use anoma_arm::resource::Resource;

use crate::Error;
use crate::action::ActionWitness;
use crate::circuit_prover::CircuitInputs;

/// The action of a resource whose logic a party proves, and the root of its action tree.
#[derive(Clone, Copy, Debug)]
pub struct LogicContext<'a> {
    pub action: &'a ActionWitness,
    pub is_consumed: bool,
    pub action_tree_root: Fr,
}

impl LogicContext<'_> {
    /// The resource whose logic the party proves.
    pub fn resource(&self) -> &Resource {
        if self.is_consumed {
            &self.action.consumed
        } else {
            &self.action.created
        }
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
