use arm::digest::extend;
use bb_proof_verification::{UltraHonkProof, UltraHonkVerificationKey, verify_honk_proof_non_zk};
use step::{ActionProofs, StepState, verify_action};

/// Adds the next action to the state of the previous step proof.
fn main(
    previous_vk: UltraHonkVerificationKey,
    previous_proof: UltraHonkProof,
    previous: StepState,
    previous_is_first_step: bool,
    compliance_vk: UltraHonkVerificationKey,
    action: ActionProofs,
) -> pub StepState {
    verify_honk_proof_non_zk(
        previous_vk,
        previous_proof,
        previous.to_fields(),
        previous.key_hash(previous_is_first_step),
    );
    let delta =
        previous.delta() + verify_action(action, compliance_vk, previous.compliance_key_hash);
    StepState {
        digest: extend(previous.digest, action.instance),
        delta_x: delta.x,
        delta_y: delta.y,
        compliance_key_hash: previous.compliance_key_hash,
        first_step_key_hash: previous.first_step_key_hash,
        next_step_key_hash: previous.next_step_key_hash,
    }
}
