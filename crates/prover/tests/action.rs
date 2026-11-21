//! Proves actions with the leaf circuits. Behind `slow-tests`: it needs nargo, bb and the compiled circuits.

#![cfg(feature = "slow-tests")]

mod common;

use anoma_arm::Fr;
use anoma_arm::field::coordinates;
use anoma_arm::resource::action_tree_root;
use anoma_prover::Error;
use anoma_prover::action::{Logic, ProvedAction, prove};
use anoma_prover::circuit_prover::{CircuitError, CircuitProver};

#[test]
fn prove_gives_the_public_inputs_that_the_step_circuits_check() -> anyhow::Result<()> {
    let witness = common::ephemeral_action(1, 5, 5)?;

    let partial = prove(common::toolchain(), &witness)?;

    let [proved] = &partial.actions[..] else {
        anyhow::bail!("expected one action, got {}", partial.actions.len());
    };
    let action = &proved.action;
    let (delta_x, delta_y) = coordinates(proved.delta);
    assert_eq!(
        proved.compliance_proof.public_inputs,
        [
            action.nullifier,
            action.consumed_logic_ref,
            action.commitment_tree_root,
            action.commitment,
            action.created_logic_ref,
            delta_x,
            delta_y,
        ]
    );
    let root = action_tree_root(action.nullifier, action.commitment);
    assert_eq!(
        proved.consumed_logic_proof.proof.public_inputs,
        [
            action.nullifier,
            1u64.into(),
            root,
            action.consumed_app_data.hash()
        ]
    );
    assert_eq!(
        proved.created_logic_proof.proof.public_inputs,
        [
            action.commitment,
            0u64.into(),
            root,
            action.created_app_data.hash()
        ]
    );
    assert_eq!(partial.rcv_total, witness.rcv);
    assert!(partial.is_balanced());
    Ok(())
}

#[test]
fn prove_errors_on_a_logic_ref_that_is_not_the_key_hash_of_the_logic() -> anyhow::Result<()> {
    let mut witness = common::ephemeral_action(1, 5, 5)?;
    witness.created.logic_ref += anoma_arm::Fr::from(1u64);

    let result = prove(common::toolchain(), &witness);

    assert!(
        matches!(result, Err(Error::LogicRef { .. })),
        "got {result:?}"
    );
    Ok(())
}

#[test]
fn verify_accepts_the_proofs_of_an_action() -> anyhow::Result<()> {
    let toolchain = common::toolchain();
    let proved = proved_action()?;

    proved.verify(
        toolchain,
        &[toolchain.verification_key(&Logic::trivial().circuit)?],
    )?;
    Ok(())
}

#[test]
fn verify_errors_on_a_changed_compliance_proof() -> anyhow::Result<()> {
    let toolchain = common::toolchain();
    let mut proved = proved_action()?;
    let middle = proved.compliance_proof.fields.len() / 2;
    proved.compliance_proof.fields[middle] += Fr::from(1u64);

    let result = proved.verify(
        toolchain,
        &[toolchain.verification_key(&Logic::trivial().circuit)?],
    );

    assert!(
        matches!(result, Err(Error::Circuit(CircuitError::Rejected { .. }))),
        "got {result:?}"
    );
    Ok(())
}

#[test]
fn verify_errors_on_a_logic_whose_key_the_verifier_does_not_know() -> anyhow::Result<()> {
    let proved = proved_action()?;

    let result = proved.verify(common::toolchain(), &[]);

    assert!(
        matches!(result, Err(Error::UnknownLogic { .. })),
        "got {result:?}"
    );
    Ok(())
}

/// The proved action of an ephemeral token, whose two resources have the trivial logic.
fn proved_action() -> anyhow::Result<ProvedAction> {
    let mut partial = prove(common::toolchain(), &common::ephemeral_action(1, 5, 5)?)?;
    partial
        .actions
        .pop()
        .ok_or_else(|| anyhow::anyhow!("prove gives no action"))
}
