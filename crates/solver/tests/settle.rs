//! The solver settles intents from an intent pool: Alice gives 4 apples for 4 bananas, Bob gives 7 bananas for 3
//! apples, and Carol gives 5 cherries for 10 apples, which no one offers. Behind `slow-tests`: it needs nargo, bb,
//! forge and anvil, and the first run makes the Groth16 setup.

#![cfg(feature = "slow-tests")]

use std::path::PathBuf;
use std::sync::Arc;

use anoma_arm::Fr;
use anoma_arm::action::Action;
use anoma_arm::belt::{Belt, MembershipPath};
use anoma_arm::resource::{NullifierKey, RESOURCE_BYTES, Resource};
use anoma_discovery::{Discovery, EciesDiscovery, EciesSecretKeys};
use anoma_integration_test::chain::Chain;
use anoma_integration_test::fixtures::{self, token};
use anoma_intent_pool::{IntentPool, MemoryIntentId, MemoryIntentPool, PooledIntent};
use anoma_prover::action::{self, ActionWitness, ProvedAction};
use anoma_prover::resource_logic::TrivialLogic;
use anoma_prover::transaction;
use anoma_solver::Solver;
use anoma_solver::cp_sat::CpSatMatcher;
use anyhow::Context;
use tempfile::TempDir;

const APPLE: u64 = 1;
const BANANA: u64 = 2;
const CHERRY: u64 = 3;

#[tokio::test(flavor = "multi_thread")]
async fn solve_settles_the_intents_that_balance_together_and_leaves_the_others_in_the_pool()
-> anyhow::Result<()> {
    let prover = fixtures::circuit_prover();
    let wrapper =
        fixtures::groth16_wrapper(&PathBuf::from(env!("CARGO_TARGET_TMPDIR")).join("wrap-keys"))?;
    let work = TempDir::new()?;
    let chain = Chain::deploy(&wrapper.keys, work.path()).await?;
    let [alice, bob, carol, sally] = [1u64, 2, 3, 4].map(|seed| NullifierKey(Fr::from(seed)));
    let [alice_keys, bob_keys, carol_keys] = std::array::from_fn(|_| EciesSecretKeys::random());
    let solver = solver(sally);

    let mints = [
        fixtures::mint(token(APPLE, 4, alice, 11)?, 21)?,
        fixtures::mint(token(BANANA, 7, bob, 12)?, 22)?,
        fixtures::mint(token(CHERRY, 5, carol, 13)?, 23)?,
    ];
    let mut minted = action::prove(prover, &mints[0])?;
    for mint in &mints[1..] {
        minted = minted.compose(action::prove(prover, mint)?);
    }
    chain
        .execute(&transaction::prove(
            prover,
            &wrapper,
            &minted,
            &chain.deployment,
        )?)
        .await?;
    let belt = belt(&mints);

    // Each party proves the action that gives up its minted token for an intent, and submits it to the pool.
    let mut pool = MemoryIntentPool::default();
    let alice_intent = pool
        .submit(propose(
            &mints[0],
            belt.path(0)?,
            alice,
            (BANANA, 4),
            &alice_keys,
            &solver,
            101,
        )?)
        .await?;
    let bob_intent = pool
        .submit(propose(
            &mints[1],
            belt.path(1)?,
            bob,
            (APPLE, 3),
            &bob_keys,
            &solver,
            102,
        )?)
        .await?;
    let carol_intent = pool
        .submit(propose(
            &mints[2],
            belt.path(2)?,
            carol,
            (APPLE, 10),
            &carol_keys,
            &solver,
            103,
        )?)
        .await?;

    let solution = solver
        .solve(prover, &pool.intents().await?, belt.root())?
        .context("the solver fills no intent")?;
    assert_eq!(solution.filled, [alice_intent, bob_intent]);
    assert_eq!(
        ids(&pool).await?,
        [alice_intent, bob_intent, carol_intent],
        "the filled intents stay in the pool until the chain executes the settlement"
    );

    assert!(
        solution
            .partial
            .actions
            .iter()
            .all(|proved| proved.action.commitment_tree_root == belt.root()),
        "no action shows whether its consumed resource is ephemeral"
    );

    let settlement = transaction::prove(prover, &wrapper, &solution.partial, &chain.deployment)?;
    chain.execute(&settlement).await?;
    for &id in &solution.filled {
        pool.remove(id).await?;
    }
    assert_eq!(ids(&pool).await?, [carol_intent]);
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
        received(&settlement.actions, &solver.secret_keys),
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
    let alice_actions = propose(
        &mints[0],
        belt.path(0)?,
        alice,
        (BANANA, wanted_bananas),
        &EciesSecretKeys::random(),
        &solver,
        101,
    )?;
    let bob_actions = propose(
        &mints[1],
        belt.path(1)?,
        bob,
        (APPLE, wanted_apples),
        &EciesSecretKeys::random(),
        &solver,
        102,
    )?;
    // Anyone can submit the actions of Alice again; a fill of both copies would consume her apples twice.
    let intents = [
        intent("alice", alice_actions.clone()),
        intent("bob", bob_actions),
        intent("copy of alice", alice_actions),
    ];
    assert!(
        bananas >= 2 * wanted_bananas && 2 * apples >= wanted_apples,
        "the premise is that both copies and the intent of Bob balance together"
    );

    let solution = solver
        .solve(fixtures::circuit_prover(), &intents, belt.root())?
        .context("the solver fills no intent")?;
    assert_eq!(solution.filled, ["alice", "bob"]);
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
    let alice_actions = propose(
        &mints[0],
        belt.path(0)?,
        alice,
        (BANANA, alice_wants),
        &EciesSecretKeys::random(),
        &solver,
        101,
    )?;
    let bob_actions = propose(
        &mints[1],
        belt.path(1)?,
        bob,
        (APPLE, bob_wants),
        &EciesSecretKeys::random(),
        &solver,
        102,
    )?;
    let mut dave_actions = propose(
        &mints[2],
        belt.path(2)?,
        dave,
        (BANANA, dave_wants),
        &EciesSecretKeys::random(),
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
    let intents = [
        intent("alice", alice_actions),
        intent("bob", bob_actions),
        intent("dave", dave_actions),
    ];

    let solution = solver
        .solve(fixtures::circuit_prover(), &intents, belt.root())?
        .context("the solver fills no intent")?;
    assert_eq!(solution.filled, ["alice", "bob"]);
    Ok(())
}

#[test]
fn solve_skips_an_intent_whose_payload_is_for_another_solver() -> anyhow::Result<()> {
    let [alice, bob, dave, sally, trent] =
        [1u64, 2, 5, 4, 6].map(|seed| NullifierKey(Fr::from(seed)));
    let other_solver = solver(trent);
    let solver = solver(sally);
    let (apples, bananas) = (4, 7);
    let (alice_wants, bob_wants, dave_wants) = (4, 3, 3);
    let mints = [
        fixtures::mint(token(APPLE, apples, alice, 11)?, 21)?,
        fixtures::mint(token(BANANA, bananas, bob, 12)?, 22)?,
        fixtures::mint(token(APPLE, apples, dave, 14)?, 24)?,
    ];
    let belt = belt(&mints);
    let intents = [
        intent(
            "alice",
            propose(
                &mints[0],
                belt.path(0)?,
                alice,
                (BANANA, alice_wants),
                &EciesSecretKeys::random(),
                &solver,
                101,
            )?,
        ),
        intent(
            "bob",
            propose(
                &mints[1],
                belt.path(1)?,
                bob,
                (APPLE, bob_wants),
                &EciesSecretKeys::random(),
                &solver,
                102,
            )?,
        ),
        intent(
            "dave",
            propose(
                &mints[2],
                belt.path(2)?,
                dave,
                (BANANA, dave_wants),
                &EciesSecretKeys::random(),
                &other_solver,
                105,
            )?,
        ),
    ];
    assert!(
        2 * apples >= bob_wants && bananas >= alice_wants + dave_wants,
        "the premise is that the intents of Alice, Bob and Dave balance together"
    );

    let solution = solver
        .solve(fixtures::circuit_prover(), &intents, belt.root())?
        .context("the solver fills no intent")?;
    assert_eq!(solution.filled, ["alice", "bob"]);
    Ok(())
}

/// An intent that a test names, as a pool gives it.
fn intent(name: &'static str, actions: Vec<ProvedAction>) -> PooledIntent<&'static str> {
    PooledIntent { id: name, actions }
}

/// The ids of the intents in the pool.
async fn ids(pool: &MemoryIntentPool) -> anyhow::Result<Vec<MemoryIntentId>> {
    Ok(pool
        .intents()
        .await?
        .iter()
        .map(|intent| intent.id)
        .collect())
}

/// A solver that trades tokens of the trivial logic and keeps its surplus for `owner`.
fn solver(owner: NullifierKey) -> Solver<EciesDiscovery, CpSatMatcher> {
    Solver {
        discovery: EciesDiscovery,
        secret_keys: EciesSecretKeys::random(),
        matcher: CpSatMatcher,
        owner,
        owner_value_ref: Fr::from(0u64),
        token_logic: Arc::new(TrivialLogic),
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
    receiver: &EciesSecretKeys,
    solver: &Solver<EciesDiscovery, CpSatMatcher>,
    seed: u64,
) -> anyhow::Result<Vec<ProvedAction>> {
    fixtures::propose(
        mint.created,
        owner,
        path,
        fixtures::terms(want_label, want_quantity, owner, seed)?,
        receiver.address(),
        &solver.secret_keys.address(),
        seed,
    )
}

/// The label, quantity and owner of each created resource whose plaintext the keys decrypt, sorted.
fn received(actions: &[Action], keys: &EciesSecretKeys) -> Vec<(Fr, u128, Fr)> {
    let mut received: Vec<(Fr, u128, Fr)> = actions
        .iter()
        .filter_map(|action| {
            EciesDiscovery
                .discover(&action.created_app_data, keys)?
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
