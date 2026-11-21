mod keys;

use bb_proof_verification::{UltraHonkProof, UltraHonkVerificationKey, verify_honk_proof_non_zk};
use std::embedded_curve_ops::{EmbeddedCurveScalar, fixed_base_scalar_mul};
use step::StepState;

/// Ends a transaction: checks the circuits of the step chain and the balance, and outputs the action digest.
fn main(
    last_vk: UltraHonkVerificationKey,
    last_proof: UltraHonkProof,
    last: StepState,
    last_is_first_step: bool,
    rcv_total: EmbeddedCurveScalar,
) -> pub Field {
    assert(
        last.compliance_key_hash == keys::COMPLIANCE_KEY_HASH,
        "the steps use another compliance circuit",
    );
    assert(
        last.first_step_key_hash == keys::FIRST_STEP_KEY_HASH,
        "the steps use another first step circuit",
    );
    assert(
        last.next_step_key_hash == keys::NEXT_STEP_KEY_HASH,
        "the steps use another next step circuit",
    );
    verify_honk_proof_non_zk(
        last_vk,
        last_proof,
        last.to_fields(),
        last.key_hash(last_is_first_step),
    );

    // Only the sum of the rcv values opens a delta whose kind parts cancel.
    assert(last.delta() == fixed_base_scalar_mul(rcv_total), "the transaction is not balanced");
    last.digest
}
