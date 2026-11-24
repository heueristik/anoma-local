//! Proves transactions with the step, root and Groth16 circuits. Behind `slow-tests`: it needs nargo, bb and the
//! compiled circuits, and the first run makes the Groth16 setup, which takes minutes and about 8 GB of memory.

#![cfg(feature = "slow-tests")]

mod common;

use anoma_arm::digest::{Deployment, action_digest};
use anoma_groth16_wrap::{Groth16Wrapper, WrapKeys};
use anoma_prover::{Error, action, transaction};
use anyhow::Context;

/// An AnomaVerifier at the address 0x0101…01 on chain 1.
const DEPLOYMENT: Deployment = Deployment {
    chain_id: 1,
    verifier: [1; 20],
};

#[test]
fn prove_gives_a_groth16_proof_of_the_bound_action_digest_of_a_composed_transaction()
-> anyhow::Result<()> {
    let prover = common::circuit_prover();
    let wrapper = Groth16Wrapper::single_party_setup(prover, &common::wrap_keys_directory())
        .context("set up the Groth16 keys")?;
    // Each action alone is unbalanced; their imbalances cancel.
    let first = action::prove(prover, &common::ephemeral_action(1, 5, 3)?)?;
    let second = action::prove(prover, &common::ephemeral_action(3, 3, 5)?)?;
    assert!(
        !first.is_balanced() && !second.is_balanced(),
        "the premise is that each action is unbalanced"
    );
    let partial = first.compose(second);

    let transaction = transaction::prove(prover, &wrapper, &partial, &DEPLOYMENT)?;

    wrapper
        .verify(&transaction.proof)
        .context("verify the Groth16 proof")?;
    assert_eq!(
        transaction.proof.public_inputs,
        [DEPLOYMENT.bind(action_digest(&transaction.actions))]
    );
    Ok(())
}

#[test]
fn prove_errors_on_an_unbalanced_transaction() -> anyhow::Result<()> {
    let prover = common::circuit_prover();
    let partial = action::prove(prover, &common::ephemeral_action(1, 5, 3)?)?;
    let wrapper = Groth16Wrapper {
        keys: WrapKeys::new(common::wrap_keys_directory()),
    };

    let result = transaction::prove(prover, &wrapper, &partial, &DEPLOYMENT);

    assert!(matches!(result, Err(Error::Unbalanced)), "got {result:?}");
    Ok(())
}
