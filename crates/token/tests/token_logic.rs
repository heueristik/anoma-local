//! Proves actions of token resources with the token logic. Behind `slow-tests`: it needs nargo, bb and the compiled
//! circuits.

#![cfg(feature = "slow-tests")]

use std::sync::Arc;

use anoma_arm::belt::Belt;
use anoma_arm::resource::{NullifierKey, Resource};
use anoma_arm::{Fr, Scalar};
use anoma_integration_test::fixtures::circuit_prover;
use anoma_prover::Error;
use anoma_prover::action::{self, ActionWitness, Membership, ResourceWitness};
use anoma_prover::circuit_prover::{CircuitError, CircuitProver};
use anoma_token::{CIRCUIT, TokenLogic, public_key_hash};
use p256::ecdsa::SigningKey;

#[test]
fn prove_proves_a_mint_that_the_issuer_signs() -> anyhow::Result<()> {
    let issuer = key(1);
    let witness = mint(token(&issuer, &key(2), 5)?, Some(issuer))?;

    let partial = action::prove(circuit_prover(), &witness)?;

    partial.actions[0].verify(
        circuit_prover(),
        &[circuit_prover().verification_key(CIRCUIT)?],
    )?;
    Ok(())
}

#[test]
fn prove_proves_a_transfer_that_the_owner_signs() -> anyhow::Result<()> {
    let (issuer, owner) = (key(1), key(2));
    let witness = transfer(token(&issuer, &owner, 5)?, owner)?;

    let partial = action::prove(circuit_prover(), &witness)?;

    partial.actions[0].verify(
        circuit_prover(),
        &[circuit_prover().verification_key(CIRCUIT)?],
    )?;
    Ok(())
}

#[test]
fn prove_proves_the_consumption_of_an_ephemeral_token_of_quantity_0_without_a_signature()
-> anyhow::Result<()> {
    let witness = ActionWitness::from_ephemeral(
        token(&key(1), &key(2), 5)?,
        0,
        NullifierKey(Fr::from(7u64)),
        Arc::new(TokenLogic::default()),
        Belt::default().root(),
        Scalar::from(8u64),
    )?;

    action::prove(circuit_prover(), &witness)?;
    Ok(())
}

#[test]
fn prove_errors_on_a_transfer_that_another_key_signs() -> anyhow::Result<()> {
    let (issuer, owner) = (key(1), key(2));

    let result = action::prove(
        circuit_prover(),
        &transfer(token(&issuer, &owner, 5)?, key(3))?,
    );

    expect_unsatisfied(result, "the signer is not the owner of the token")
}

#[test]
fn prove_errors_on_a_mint_without_a_signature() -> anyhow::Result<()> {
    let result = action::prove(circuit_prover(), &mint(token(&key(1), &key(2), 5)?, None)?);

    expect_unsatisfied(
        result,
        "the signer is not the issuer of the ephemeral token",
    )
}

/// The P-256 key of the secret scalar `seed`.
fn key(seed: u8) -> SigningKey {
    let mut secret = [0; 32];
    secret[31] = seed;
    SigningKey::from_slice(&secret).expect("a nonzero scalar below the group order is a secret key")
}

/// A persistent token of `issuer`, owned by `owner` with the nullifier key 42. The action that creates it sets its nonce.
fn token(issuer: &SigningKey, owner: &SigningKey, quantity: u128) -> anyhow::Result<Resource> {
    Ok(Resource {
        logic_ref: circuit_prover().verification_key(CIRCUIT)?.hash,
        label_ref: public_key_hash(issuer.verifying_key()),
        value_ref: public_key_hash(owner.verifying_key()),
        quantity,
        nonce: Fr::from(0u64),
        nullifier_key_commitment: NullifierKey(Fr::from(42u64)).commitment(),
        is_ephemeral: false,
        rand_seed: Fr::from(9u64),
    })
}

/// The action that creates `token` from an ephemeral token of its kind and quantity, which `signer` signs.
fn mint(token: Resource, signer: Option<SigningKey>) -> anyhow::Result<ActionWitness> {
    Ok(ActionWitness::from_ephemeral(
        token,
        token.quantity,
        NullifierKey(Fr::from(7u64)),
        Arc::new(TokenLogic {
            signer: signer.map(|key| Arc::new(key) as _),
        }),
        Belt::default().root(),
        Scalar::from(8u64),
    )?)
}

/// The action that consumes `token`, the only commitment in the tree, with a signature of `signer`, and creates a token
/// of the same kind and quantity.
fn transfer(token: Resource, signer: SigningKey) -> anyhow::Result<ActionWitness> {
    let nullifier_key = NullifierKey(Fr::from(42u64));
    let mut belt = Belt::default();
    belt.append(&[token.commitment()]);
    Ok(ActionWitness::new(
        vec![
            ResourceWitness::consumed(
                token,
                Arc::new(TokenLogic {
                    signer: Some(Arc::new(signer)),
                }),
                nullifier_key,
                Membership::Persistent(belt.path(0)?),
            ),
            ResourceWitness::created(
                Resource {
                    rand_seed: Fr::from(10u64),
                    ..token
                },
                Arc::new(TokenLogic::default()),
            ),
        ],
        Scalar::from(8u64),
    )?)
}

fn expect_unsatisfied(
    result: Result<anoma_prover::PartialTransaction, Error>,
    message: &str,
) -> anyhow::Result<()> {
    let Err(Error::Circuit(CircuitError::Unsatisfied { reason, .. })) = result else {
        anyhow::bail!("expected inputs that do not satisfy the token logic, got {result:?}");
    };
    assert!(reason.contains(message), "{reason}");
    Ok(())
}
