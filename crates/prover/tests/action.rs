//! Proves actions with the leaf circuits. Behind `slow-tests`: it needs nargo, bb and the compiled circuits.

#![cfg(feature = "slow-tests")]

mod common;

use anoma_arm::Fr;
use anoma_arm::action::{MAX_RESOURCES, action_tree_root};
use anoma_arm::field::coordinates;
use anoma_prover::Error;
use anoma_prover::action::{ProvedAction, prove};
use anoma_prover::circuit_prover::{CircuitError, CircuitProver};
use anoma_prover::resource_logic::{ResourceLogic, TrivialLogic};

#[test]
fn prove_gives_the_public_inputs_that_the_step_circuits_check() -> anyhow::Result<()> {
    // An action of one consumed and two created resources uses a compliance circuit for more than a pair.
    let witness = common::split_action(1, 5, &[2, 3])?;

    let partial = prove(common::circuit_prover(), &witness)?;

    let [proved] = &partial.actions[..] else {
        anyhow::bail!("expected one action, got {}", partial.actions.len());
    };
    let resources = &proved.action.resources;
    let (delta_x, delta_y) = coordinates(proved.delta);
    let compliance_inputs: Vec<Fr> = resources
        .iter()
        .flat_map(|resource| {
            [
                Fr::from(resource.is_consumed),
                resource.tag,
                resource.logic_ref,
                resource.commitment_tree_root,
            ]
        })
        .chain([delta_x, delta_y])
        .collect();
    assert_eq!(proved.compliance_proof.public_inputs, compliance_inputs);
    let root = action_tree_root(&proved.action.tags())?;
    assert_eq!(proved.logic_proofs.len(), resources.len());
    for (resource, logic_proof) in resources.iter().zip(&proved.logic_proofs) {
        assert_eq!(
            logic_proof.proof.public_inputs,
            [
                resource.tag,
                Fr::from(resource.is_consumed),
                root,
                resource.app_data.hash()
            ]
        );
    }
    assert_eq!(partial.rcv_total, witness.rcv);
    assert!(partial.is_balanced());
    Ok(())
}

#[test]
fn prove_errors_on_a_logic_ref_that_is_not_the_key_hash_of_the_logic() -> anyhow::Result<()> {
    let mut witness = common::ephemeral_action(1, 5, 5)?;
    witness.resources[1].resource.logic_ref += Fr::from(1u64);

    let result = prove(common::circuit_prover(), &witness);

    assert!(
        matches!(result, Err(Error::LogicRef { .. })),
        "got {result:?}"
    );
    Ok(())
}

#[test]
fn prove_errors_on_an_action_without_a_consumed_resource() -> anyhow::Result<()> {
    let mut witness = common::ephemeral_action(1, 5, 5)?;
    witness.resources.remove(0);
    assert!(
        witness
            .resources
            .iter()
            .all(|resource| !resource.is_consumed()),
        "the premise is that the action consumes no resource"
    );

    let result = prove(common::circuit_prover(), &witness);

    let Err(Error::Circuit(CircuitError::Unsatisfied { reason, .. })) = result else {
        anyhow::bail!("expected inputs that do not satisfy the compliance unit, got {result:?}");
    };
    assert!(
        reason.contains("the compliance unit consumes no resource"),
        "{reason}"
    );
    Ok(())
}

#[test]
fn prove_errors_on_more_resources_than_the_circuits_take() -> anyhow::Result<()> {
    let created_quantities = vec![1; MAX_RESOURCES];
    let witness = common::split_action(1, MAX_RESOURCES as u128, &created_quantities)?;

    let result = prove(common::circuit_prover(), &witness);

    assert!(
        matches!(
            result,
            Err(Error::ResourceCount {
                resource_count,
                max: MAX_RESOURCES
            }) if resource_count == MAX_RESOURCES + 1
        ),
        "got {result:?}"
    );
    Ok(())
}

#[test]
fn verify_accepts_the_proofs_of_an_action() -> anyhow::Result<()> {
    let prover = common::circuit_prover();
    let proved = proved_action()?;

    proved.verify(prover, &[prover.verification_key(TrivialLogic.circuit())?])?;
    Ok(())
}

#[test]
fn verify_errors_on_a_changed_compliance_proof() -> anyhow::Result<()> {
    let prover = common::circuit_prover();
    let mut proved = proved_action()?;
    let middle = proved.compliance_proof.fields.len() / 2;
    proved.compliance_proof.fields[middle] += Fr::from(1u64);

    let result = proved.verify(prover, &[prover.verification_key(TrivialLogic.circuit())?]);

    assert!(
        matches!(result, Err(Error::Circuit(CircuitError::Rejected { .. }))),
        "got {result:?}"
    );
    Ok(())
}

#[test]
fn verify_errors_on_a_logic_whose_key_the_verifier_does_not_know() -> anyhow::Result<()> {
    let proved = proved_action()?;

    let result = proved.verify(common::circuit_prover(), &[]);

    assert!(
        matches!(result, Err(Error::UnknownLogic { .. })),
        "got {result:?}"
    );
    Ok(())
}

/// The proved action of an ephemeral resource, whose two resources have the trivial logic.
fn proved_action() -> anyhow::Result<ProvedAction> {
    let mut partial = prove(
        common::circuit_prover(),
        &common::ephemeral_action(1, 5, 5)?,
    )?;
    partial
        .actions
        .pop()
        .ok_or_else(|| anyhow::anyhow!("prove gives no action"))
}
