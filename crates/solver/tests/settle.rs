//! The solver settles intents from the intent pool: Alice gives 4 apples for at least 4 bananas, Bob gives 7 bananas
//! for at least 3 apples, and Carol gives 5 cherries for at least 10 apples, which no one offers. Behind `slow-tests`:
//! it needs nargo, bb, forge and anvil, and the first run makes the Groth16 setup.

#![cfg(feature = "slow-tests")]

use std::path::PathBuf;

use anoma_arm::Fr;
use anoma_arm::belt::Belt;
use anoma_arm::digest::Action;
use anoma_arm::resource::{NullifierKey, RESOURCE_BYTES, Resource};
use anoma_discovery::ReceiverKeys;
use anoma_integration_test::chain::Chain;
use anoma_integration_test::fixtures::{self, intent_logic_ref, token, token_logic_ref};
use anoma_intent::Terms;
use anoma_intent_pool::IntentPool;
use anoma_prover::action::{self, Logic};
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
    let solver = Solver {
        keys: ReceiverKeys::random(),
        owner: sally,
        intent_logic_ref: intent_logic_ref()?,
        token_logic: Logic::trivial(),
        token_logic_ref: token_logic_ref()?,
    };

    let mints = [
        fixtures::mint(token(APPLE, 4, alice, 11)?, 21)?,
        fixtures::mint(token(BANANA, 7, bob, 12)?, 22)?,
        fixtures::mint(token(CHERRY, 5, carol, 13)?, 23)?,
    ];
    let mut minted = action::prove(toolchain, &mints[0])?;
    for mint in &mints[1..] {
        minted = minted.compose(action::prove(toolchain, mint)?);
    }
    let minted = transaction::prove(toolchain, &minted, &keys)?;
    chain.execute(&minted).await?;
    let mut belt = Belt::default();
    belt.append(
        &minted
            .actions
            .iter()
            .map(|action| action.commitment)
            .collect::<Vec<_>>(),
    );

    // Each party proves the action that gives up its minted token for an intent, and submits it to the pool.
    let mut pool = IntentPool::default();
    let mut propose = |index: usize, owner, want, min_quantity, receiver: &ReceiverKeys, seed| {
        let actions = fixtures::propose(
            mints[index].created,
            owner,
            belt.path(index)?,
            terms(want, min_quantity, owner)?,
            receiver.receiver(),
            &solver.keys.receiver(),
            seed,
        )?;
        anyhow::Ok(pool.submit(actions))
    };
    let alice_intent = propose(0, alice, BANANA, 4, &alice_keys, 101)?;
    let bob_intent = propose(1, bob, APPLE, 3, &bob_keys, 102)?;
    let carol_intent = propose(2, carol, APPLE, 10, &carol_keys, 103)?;

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

fn terms(want: u64, min_quantity: u128, receiver: NullifierKey) -> anyhow::Result<Terms> {
    Ok(Terms {
        want_logic_ref: token_logic_ref()?,
        want_label_ref: Fr::from(want),
        min_quantity,
        receiver_nullifier_key_commitment: receiver.commitment(),
    })
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
