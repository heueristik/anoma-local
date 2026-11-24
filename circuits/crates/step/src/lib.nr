use arm::{
    action::{action_tree_root, MAX_RESOURCES},
    digest::{extend, ResourceInstance},
    hash::{self, hash},
};
use bb_proof_verification::{
    UltraHonkProof, UltraHonkVerificationKey, UltraHonkZKProof, verify_honk_proof,
    verify_honk_proof_non_zk,
};
use std::embedded_curve_ops::EmbeddedCurvePoint;

/// The number of public inputs of a compliance unit for each resource: `is_consumed`, the tag, the logic ref and the
/// commitment tree root.
global COMPLIANCE_INPUTS_PER_RESOURCE: u32 = 4;

/// The key hashes of the compliance and step circuits; entry k - 1 is the circuit of an action of k resources.
pub struct CircuitKeys {
    pub compliance: [Field; MAX_RESOURCES],
    pub first_step: [Field; MAX_RESOURCES],
    pub next_step: [Field; MAX_RESOURCES],
}

impl CircuitKeys {
    /// The hash that a step state carries, so that each step and the root check the same keys.
    pub fn hash(self) -> Field {
        let mut keys = [0; 3 * MAX_RESOURCES];
        for i in 0..MAX_RESOURCES {
            keys[i] = self.compliance[i];
            keys[MAX_RESOURCES + i] = self.first_step[i];
            keys[2 * MAX_RESOURCES + i] = self.next_step[i];
        }
        hash(hash::CIRCUIT_KEYS, keys)
    }

    /// The key hash of the step circuit that made a step proof.
    pub fn step(self, circuit: StepCircuit) -> Field {
        if circuit.is_first_step {
            self.first_step[circuit.size - 1]
        } else {
            self.next_step[circuit.size - 1]
        }
    }
}

/// A step circuit: the first or a next step, for an action of `size` resources.
pub struct StepCircuit {
    pub is_first_step: bool,
    pub size: u32,
}

/// The public outputs of a step proof: the state of a transaction after some of its actions.
pub struct StepState {
    pub digest: Field,
    pub delta_x: Field,
    pub delta_y: Field,
    pub keys_hash: Field,
}

impl StepState {
    /// The public inputs of a step proof of this state, in the order of the fields.
    pub fn to_fields(self) -> [Field; 4] {
        [self.digest, self.delta_x, self.delta_y, self.keys_hash]
    }

    /// The sum of the unit deltas of the actions.
    pub fn delta(self) -> EmbeddedCurvePoint {
        EmbeddedCurvePoint::new(self.delta_x, self.delta_y)
    }
}

/// The public data of an action of `K` resources, its unit delta, and its leaf proofs: the compliance proof, and the
/// logic proof of each resource in the order of the list.
pub struct ActionProofs<let K: u32> {
    pub resources: [ResourceInstance; K],
    pub delta_x: Field,
    pub delta_y: Field,
    pub compliance_proof: UltraHonkZKProof,
    pub logic_vks: [UltraHonkVerificationKey; K],
    pub logic_proofs: [UltraHonkZKProof; K],
}

/// Starts a transaction with its first action. The keys become the ones that every later step and the root check.
pub fn first_step<let K: u32>(
    keys: CircuitKeys,
    compliance_vk: UltraHonkVerificationKey,
    action: ActionProofs<K>,
) -> StepState {
    let delta = verify_action(action, compliance_vk, keys.compliance[K - 1]);
    StepState {
        digest: extend(0, action.resources),
        delta_x: delta.x,
        delta_y: delta.y,
        keys_hash: keys.hash(),
    }
}

/// Adds the next action to the state of the previous step proof.
pub fn next_step<let K: u32>(
    previous_vk: UltraHonkVerificationKey,
    previous_proof: UltraHonkProof,
    previous: StepState,
    previous_circuit: StepCircuit,
    keys: CircuitKeys,
    compliance_vk: UltraHonkVerificationKey,
    action: ActionProofs<K>,
) -> StepState {
    assert(keys.hash() == previous.keys_hash, "the keys are not the keys of the previous step");
    verify_honk_proof_non_zk(
        previous_vk,
        previous_proof,
        previous.to_fields(),
        keys.step(previous_circuit),
    );
    let delta = previous.delta() + verify_action(action, compliance_vk, keys.compliance[K - 1]);
    StepState {
        digest: extend(previous.digest, action.resources),
        delta_x: delta.x,
        delta_y: delta.y,
        keys_hash: previous.keys_hash,
    }
}

/// Verifies the leaf proofs of an action and returns its unit delta. Logic proof i belongs to resource i, and a logic
/// ref is the key hash of its logic.
fn verify_action<let K: u32>(
    action: ActionProofs<K>,
    compliance_vk: UltraHonkVerificationKey,
    compliance_key_hash: Field,
) -> EmbeddedCurvePoint {
    let mut compliance_inputs = [0; COMPLIANCE_INPUTS_PER_RESOURCE * K + 2];
    let mut tags = [0; K];
    for i in 0..K {
        let resource = action.resources[i];
        let first = COMPLIANCE_INPUTS_PER_RESOURCE * i;
        compliance_inputs[first] = resource.is_consumed as Field;
        compliance_inputs[first + 1] = resource.tag;
        compliance_inputs[first + 2] = resource.logic_ref;
        compliance_inputs[first + 3] = resource.commitment_tree_root;
        tags[i] = resource.tag;
    }
    compliance_inputs[COMPLIANCE_INPUTS_PER_RESOURCE * K] = action.delta_x;
    compliance_inputs[COMPLIANCE_INPUTS_PER_RESOURCE * K + 1] = action.delta_y;
    verify_honk_proof(
        compliance_vk,
        action.compliance_proof,
        compliance_inputs,
        compliance_key_hash,
    );

    let root = action_tree_root(tags);
    for i in 0..K {
        let resource = action.resources[i];
        verify_honk_proof(
            action.logic_vks[i],
            action.logic_proofs[i],
            [resource.tag, resource.is_consumed as Field, root, resource.app_data_hash],
            resource.logic_ref,
        );
    }

    EmbeddedCurvePoint::new(action.delta_x, action.delta_y)
}

#[test]
fn step_picks_the_key_hash_of_the_step_circuit() {
    let keys = CircuitKeys {
        compliance: [0; MAX_RESOURCES],
        first_step: [1, 2, 3, 4, 5, 6, 7, 8],
        next_step: [11, 12, 13, 14, 15, 16, 17, 18],
    };
    assert_eq(keys.step(StepCircuit { is_first_step: true, size: 3 }), 3);
    assert_eq(keys.step(StepCircuit { is_first_step: false, size: 8 }), 18);
}

#[test]
fn hash_changes_with_each_key() {
    let keys = CircuitKeys {
        compliance: [1; MAX_RESOURCES],
        first_step: [2; MAX_RESOURCES],
        next_step: [3; MAX_RESOURCES],
    };
    let mut other = keys;
    other.next_step[MAX_RESOURCES - 1] = 4;
    assert(keys.hash() != other.hash());
}
