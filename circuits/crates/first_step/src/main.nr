use arm::digest::extend;
use bb_proof_verification::UltraHonkVerificationKey;
use step::{ActionProofs, StepState, verify_action};

/// Starts a transaction with its first action; the key hashes become the ones that every later step and the root check.
fn main(
    compliance_vk: UltraHonkVerificationKey,
    compliance_key_hash: Field,
    first_step_key_hash: Field,
    next_step_key_hash: Field,
    action: ActionProofs,
) -> pub StepState {
    let delta = verify_action(action, compliance_vk, compliance_key_hash);
    StepState {
        digest: extend(0, action.instance),
        delta_x: delta.x,
        delta_y: delta.y,
        compliance_key_hash,
        first_step_key_hash,
        next_step_key_hash,
    }
}
