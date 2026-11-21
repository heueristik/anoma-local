use arm::{action::action_tree_root, digest::ActionInstance};
use bb_proof_verification::{UltraHonkVerificationKey, UltraHonkZKProof, verify_honk_proof};
use std::embedded_curve_ops::EmbeddedCurvePoint;

/// The public outputs of a step proof: the state of a transaction after some of its actions.
pub struct StepState {
    pub digest: Field,
    pub delta_x: Field,
    pub delta_y: Field,
    pub compliance_key_hash: Field,
    pub first_step_key_hash: Field,
    pub next_step_key_hash: Field,
}

impl StepState {
    /// The public inputs of a step proof of this state, in the order of the fields.
    pub fn to_fields(self) -> [Field; 6] {
        [
            self.digest,
            self.delta_x,
            self.delta_y,
            self.compliance_key_hash,
            self.first_step_key_hash,
            self.next_step_key_hash,
        ]
    }

    /// The key hash of the first or the next step circuit; a proof of either circuit can precede a next step.
    pub fn key_hash(self, is_first_step: bool) -> Field {
        if is_first_step {
            self.first_step_key_hash
        } else {
            self.next_step_key_hash
        }
    }

    /// The sum of the unit deltas of the actions.
    pub fn delta(self) -> EmbeddedCurvePoint {
        EmbeddedCurvePoint::new(self.delta_x, self.delta_y)
    }
}

/// The public data of an action, its unit delta, and the leaf proofs of its compliance unit and its two resource logics.
pub struct ActionProofs {
    pub instance: ActionInstance,
    pub delta_x: Field,
    pub delta_y: Field,
    pub compliance_proof: UltraHonkZKProof,
    pub consumed_logic_vk: UltraHonkVerificationKey,
    pub consumed_logic_proof: UltraHonkZKProof,
    pub created_logic_vk: UltraHonkVerificationKey,
    pub created_logic_proof: UltraHonkZKProof,
}

/// Verifies the three leaf proofs of an action and returns its unit delta; a logic ref is the key hash of its logic.
pub fn verify_action(
    action: ActionProofs,
    compliance_vk: UltraHonkVerificationKey,
    compliance_key_hash: Field,
) -> EmbeddedCurvePoint {
    let instance = action.instance;
    verify_honk_proof(
        compliance_vk,
        action.compliance_proof,
        [
            instance.nullifier,
            instance.consumed_logic_ref,
            instance.commitment_tree_root,
            instance.commitment,
            instance.created_logic_ref,
            action.delta_x,
            action.delta_y,
        ],
        compliance_key_hash,
    );

    let root = action_tree_root(instance.nullifier, instance.commitment);
    verify_honk_proof(
        action.consumed_logic_vk,
        action.consumed_logic_proof,
        [instance.nullifier, 1, root, instance.consumed_app_data_hash],
        instance.consumed_logic_ref,
    );
    verify_honk_proof(
        action.created_logic_vk,
        action.created_logic_proof,
        [instance.commitment, 0, root, instance.created_app_data_hash],
        instance.created_logic_ref,
    );

    EmbeddedCurvePoint::new(action.delta_x, action.delta_y)
}

#[test]
fn key_hash_picks_the_key_hash_of_the_step_circuit() {
    let state = StepState {
        digest: 0,
        delta_x: 0,
        delta_y: 0,
        compliance_key_hash: 1,
        first_step_key_hash: 2,
        next_step_key_hash: 3,
    };
    assert_eq(state.key_hash(true), 2);
    assert_eq(state.key_hash(false), 3);
}
