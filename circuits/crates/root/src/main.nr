mod keys;

use arm::digest::bind;
use bb_proof_verification::{UltraHonkProof, UltraHonkVerificationKey, verify_honk_proof_non_zk};
use std::embedded_curve_ops::{EmbeddedCurveScalar, fixed_base_scalar_mul};
use step::{StepCircuit, StepState};

/// Ends a transaction: checks the circuits of the step chain and the balance, and outputs the action digest bound to
/// the chain ID and the address of the AnomaVerifier.
fn main(
    last_vk: UltraHonkVerificationKey,
    last_proof: UltraHonkProof,
    last: StepState,
    last_circuit: StepCircuit,
    rcv_total: EmbeddedCurveScalar,
    chain_id: Field,
    verifier: Field,
) -> pub Field {
    assert(last.keys_hash == keys::CIRCUIT_KEYS.hash(), "the steps use other circuits");
    verify_honk_proof_non_zk(
        last_vk,
        last_proof,
        last.to_fields(),
        keys::CIRCUIT_KEYS.step(last_circuit),
    );

    // Only the sum of the rcv values opens a delta whose kind parts cancel.
    assert(last.delta() == fixed_base_scalar_mul(rcv_total), "the transaction is not balanced");
    // The contract binds the digest to its own chain and address, so that a proof verifies on one AnomaVerifier only.
    bind(chain_id, verifier, last.digest)
}
