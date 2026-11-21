//! The solver settles intents from the intent pool: Alice gives 4 apples for 4 bananas, Bob gives 7 bananas for 3
//! apples, and Carol gives 5 cherries for 10 apples, which no one offers. Behind `slow-tests`: it needs nargo, bb,
//! forge and anvil, and the first run makes the Groth16 setup.

#![cfg(feature = "slow-tests")]

use std::path::PathBuf;

use anoma_arm::Fr;
use anoma_arm::belt::{Belt, MembershipPath};
use anoma_arm::digest::Action;
use anoma_arm::resource::{NullifierKey, RESOURCE_BYTES, Resource};
use anoma_discovery::ReceiverKeys;
use anoma_integration_test::chain::Chain;
use anoma_integration_test::fixtures::{self, token};
use anoma_intent_pool::IntentPool;
use anoma_prover::action::{self, ActionWitness, Logic, ProvedAction};
use anoma_prover::transaction;
use anoma_solver::Solver;
use anyhow::Context;
use tempfile::TempDir;

const APPLE: u64 = 1;
const BANANA: u64 = 2;
const CHERRY: u64 = 3;

#[tokio::test(flavor = "multi_thread")]
async fn solve_settles_the_intents_that_balance_together_and_leaves_the_others_in_the_pool()
-> anyhow::Result<()> {
    let toolchain = fixtures::toolchain();
    let keys = fixtures::wrap_keys(&PathBuf::from(env!("CARGO_TARGET_TMPDIR")).join("wrap-keys"))?;
    let work = TempDir::new()?;
    let chain = Chain::deploy(&keys, work.path()).await?;
    let [alice, bob, carol, sally] = [1u64, 2, 3, 4].map(|seed| NullifierKey(Fr::from(seed)));
    let [alice_keys, bob_keys, carol_keys] = std::array::from_fn(|_| ReceiverKeys::random());
    let solver = solver(sally);

    let mints = [
        fixtures::mint(token(APPLE, 4, alice, 11)?, 21)?,
        fixtures::mint(token(BANANA, 7, bob, 12)?, 22)?,
        fixtures::mint(token(CHERRY, 5, carol, 13)?, 23)?,
    ];
    let mut minted = action::prove(toolchain, &mints[0])?;
    for mint in &mints[1..] {
        minted = minted.compose(action::prove(toolchain, mint)?);
    }
    chain
        .execute(&transaction::prove(toolchain, &minted, &keys)?)
        .await?;
    let belt = belt(&mints);

    // Each party proves the action that gives up its minted token for an intent, and submits it to the pool.
    let mut pool = IntentPool::default();
    let alice_intent = pool.submit(propose(
        &mints[0],
        belt.path(0)?,
        alice,
        (BANANA, 4),
        &alice_keys,
        &solver,
        101,
    )?);
    let bob_intent = pool.submit(propose(
        &mints[1],
        belt.path(1)?,
        bob,
        (APPLE, 3),
        &bob_keys,
        &solver,
        102,
    )?);
    let carol_intent = pool.submit(propose(
        &mints[2],
        belt.path(2)?,
        carol,
        (APPLE, 10),
        &carol_keys,
        &solver,
        103,
    )?);

    let solution = solver
        .solve(toolchain, &mut pool)?
        .context("the solver fills no intent")?;
    assert_eq!(solution.filled, [alice_intent, bob_intent]);
    assert_eq!(pool.len(), 1);
    assert!(pool.get(carol_intent).is_some());

    let settlement = transaction::prove(toolchain, &solution.partial, &keys)?;
    chain.execute(&settlement).await?;
    let label = |label: u64| Fr::from(label);
    assert_eq!(
        received(&settlement.actions, &alice_keys),
        [(label(BANANA), 4, alice.commitment())]
    );
    assert_eq!(
        received(&settlement.actions, &bob_keys),
        [(label(APPLE), 3, bob.commitment())]
    );
    assert_eq!(
        received(&settlement.actions, &solver.keys),
        [
            (label(APPLE), 1, sally.commitment()),
            (label(BANANA), 3, sally.commitment())
        ],
        "the solver keeps the surplus"
    );
    assert_eq!(received(&settlement.actions, &carol_keys), []);
    Ok(())
}

#[test]
fn solve_fills_a_copied_intent_once() -> anyhow::Result<()> {
    let [alice, bob, sally] = [1u64, 2, 4].map(|seed| NullifierKey(Fr::from(seed)));
    let solver = solver(sally);
    let (apples, wanted_bananas, bananas, wanted_apples) = (4, 3, 7, 3);
    let mints = [
        fixtures::mint(token(APPLE, apples, alice, 11)?, 21)?,
        fixtures::mint(token(BANANA, bananas, bob, 12)?, 22)?,
    ];
    let belt = belt(&mints);
    let mut pool = IntentPool::default();
    let alice_actions = propose(
        &mints[0],
        belt.path(0)?,
        alice,
        (BANANA, wanted_bananas),
        &ReceiverKeys::random(),
        &solver,
        101,
    )?;
    let alice_intent = pool.submit(alice_actions.clone());
    let bob_intent = pool.submit(propose(
        &mints[1],
        belt.path(1)?,
        bob,
        (APPLE, wanted_apples),
        &ReceiverKeys::random(),
        &solver,
        102,
    )?);

    // Anyone can submit the actions of Alice again; a fill of both copies would consume her apples twice.
    pool.submit(alice_actions);
    assert!(
        bananas >= 2 * wanted_bananas && 2 * apples >= wanted_apples,
        "the premise is that both copies and the intent of Bob balance together"
    );

    let solution = solver
        .solve(fixtures::toolchain(), &mut pool)?
        .context("the solver fills no intent")?;
    assert_eq!(solution.filled, [alice_intent, bob_intent]);
    Ok(())
}

#[test]
fn solve_skips_an_intent_with_an_invalid_proof() -> anyhow::Result<()> {
    let [alice, bob, dave, sally] = [1u64, 2, 5, 4].map(|seed| NullifierKey(Fr::from(seed)));
    let solver = solver(sally);
    let (apples, bananas) = (4, 7);
    let (alice_wants, bob_wants, dave_wants) = (4, 3, 3);
    let mints = [
        fixtures::mint(token(APPLE, apples, alice, 11)?, 21)?,
        fixtures::mint(token(BANANA, bananas, bob, 12)?, 22)?,
        fixtures::mint(token(APPLE, apples, dave, 14)?, 24)?,
    ];
    let belt = belt(&mints);
    let mut pool = IntentPool::default();
    let alice_intent = pool.submit(propose(
        &mints[0],
        belt.path(0)?,
        alice,
        (BANANA, alice_wants),
        &ReceiverKeys::random(),
        &solver,
        101,
    )?);
    let bob_intent = pool.submit(propose(
        &mints[1],
        belt.path(1)?,
        bob,
        (APPLE, bob_wants),
        &ReceiverKeys::random(),
        &solver,
        102,
    )?);
    let mut dave_actions = propose(
        &mints[2],
        belt.path(2)?,
        dave,
        (BANANA, dave_wants),
        &ReceiverKeys::random(),
        &solver,
        105,
    )?;
    assert!(
        2 * apples >= bob_wants && bananas >= alice_wants + dave_wants,
        "the premise is that the intents of Alice, Bob and Dave balance together"
    );

    let proof = &mut dave_actions[0].compliance_proof.fields;
    let middle = proof.len() / 2;
    proof[middle] += Fr::from(1u64);
    pool.submit(dave_actions);

    let solution = solver
        .solve(fixtures::toolchain(), &mut pool)?
        .context("the solver fills no intent")?;
    assert_eq!(solution.filled, [alice_intent, bob_intent]);
    Ok(())
}

/// A solver that trades tokens of the trivial logic and keeps its surplus for `owner`.
fn solver(owner: NullifierKey) -> Solver {
    Solver {
        keys: ReceiverKeys::random(),
        owner,
        token_logic: Logic::trivial(),
    }
}

/// The belt of the tokens that the mints create, in their order.
fn belt(mints: &[ActionWitness]) -> Belt {
    let mut belt = Belt::default();
    belt.append(
        &mints
            .iter()
            .map(|mint| mint.created.commitment())
            .collect::<Vec<_>>(),
    );
    belt
}

/// The proved action of `owner` that gives up the token of `mint` at `path` for an intent that wants `want`, a label
/// and a quantity, for the keys of `receiver`. The payload is for the keys of the solver.
fn propose(
    mint: &ActionWitness,
    path: MembershipPath,
    owner: NullifierKey,
    (want_label, want_quantity): (u64, u128),
    receiver: &ReceiverKeys,
    solver: &Solver,
    seed: u64,
) -> anyhow::Result<Vec<ProvedAction>> {
    fixtures::propose(
        mint.created,
        owner,
        path,
        fixtures::terms(want_label, want_quantity, owner, seed)?,
        receiver.receiver(),
        &solver.keys.receiver(),
        seed,
    )
}

/// The label, quantity and owner of each created resource whose plaintext the keys decrypt, sorted.
fn received(actions: &[Action], keys: &ReceiverKeys) -> Vec<(Fr, u128, Fr)> {
    let mut received: Vec<(Fr, u128, Fr)> = actions
        .iter()
        .filter_map(|action| {
            anoma_discovery::discover(&action.created_app_data, keys)?
                .iter()
                .filter_map(|bytes| {
                    Resource::from_bytes(<&[u8; RESOURCE_BYTES]>::try_from(bytes.as_slice()).ok()?)
                        .ok()
                })
                .find(|resource| resource.commitment() == action.commitment)
        })
        .map(|resource| {
            (
                resource.label_ref,
                resource.quantity,
                resource.nullifier_key_commitment,
            )
        })
        .collect();
    received.sort();
    received
}
