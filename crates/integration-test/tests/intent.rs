//! The intent example: Alice gives 4 apples for 4 bananas, Bob gives 7 bananas for 3 apples, and Sally solves in
//! several composition steps and keeps the surplus. Behind `slow-tests`: it needs nargo, bb, forge and anvil, and the
//! first run makes the Groth16 setup, which takes minutes and about 8 GB of memory.

#![cfg(feature = "slow-tests")]

use std::path::PathBuf;
use std::sync::Arc;

use anoma_arm::Fr;
use anoma_arm::Scalar;
use anoma_arm::belt::Belt;
use anoma_arm::resource::{NullifierKey, Resource};
use anoma_bindings::conversion::word;
use anoma_integration_test::chain::Chain;
use anoma_integration_test::fixtures::{
    self, Party, action as build_action, intent_logic_ref, token,
};
use anoma_intent::{IntentLogic, Terms};
use anoma_prover::action::{ActionWitness, Membership};
use anoma_prover::circuit_prover::CircuitError;
use anoma_prover::{Error, PartialTransaction, Transaction, action, transaction};
use anoma_token::TokenLogic;
use tempfile::TempDir;

#[tokio::test(flavor = "multi_thread")]
async fn execute_settles_the_intent_example_after_several_composition_steps() -> anyhow::Result<()>
{
    let prover = fixtures::circuit_prover();
    let wrapper = fixtures::groth16_wrapper(&wrap_keys_directory())?;
    let work = TempDir::new()?;
    let chain = Chain::deploy(&wrapper.keys, work.path()).await?;
    let mut belt = Belt::default();
    let [alice, bob, sally] = [1, 2, 3].map(Party::new);
    let [apple, banana] = issuers();

    let alice_apples = fixtures::mint(&apple, token(&apple, 4, &alice, 11)?, 21)?;
    let bob_bananas = fixtures::mint(&banana, token(&banana, 7, &bob, 12)?, 22)?;
    let mint = action::prove(prover, &alice_apples)?.compose(action::prove(prover, &bob_bananas)?);
    execute(
        &chain,
        &mut belt,
        &transaction::prove(prover, &wrapper, &mint, &chain.deployment)?,
    )
    .await?;

    // Each party proves its own action, which gives up its tokens for an intent, and gives Sally the intent.
    let alice_intent = Intent::new(&banana, 4, &alice, 101);
    let alice_part = alice_intent.give_up(&belt, 0, alice_apples.created, &alice, 31)?;
    let bob_intent = Intent::new(&apple, 3, &bob, 102);
    let bob_part = bob_intent.give_up(&belt, 1, bob_bananas.created, &bob, 32)?;

    let composed = alice_part.partial.compose(bob_part.partial);
    assert!(!composed.is_balanced());
    let composed = composed.compose(action::prove(
        prover,
        &alice_intent.fill(&alice_part.resource, belt.root(), 41)?,
    )?);
    assert!(!composed.is_balanced());
    let composed = composed.compose(action::prove(
        prover,
        &bob_intent.fill(&bob_part.resource, belt.root(), 42)?,
    )?);
    assert!(
        !composed.is_balanced(),
        "the surplus of 1 apple and 3 bananas is still open"
    );
    let composed = composed
        .compose(action::prove(
            prover,
            &take_surplus(token(&apple, 1, &sally, 15)?, belt.root(), 51)?,
        )?)
        .compose(action::prove(
            prover,
            &take_surplus(token(&banana, 3, &sally, 16)?, belt.root(), 52)?,
        )?);
    assert!(composed.is_balanced());

    let settlement = transaction::prove(prover, &wrapper, &composed, &chain.deployment)?;
    execute(&chain, &mut belt, &settlement).await?;
    // Alice computes the resource that fills her intent from her terms, with no payload from Sally.
    let alice_fill = alice_intent
        .terms()?
        .filling_resource(alice_part.resource.nullifier(alice_intent.nullifier_key)?);
    assert!(
        settlement
            .actions
            .iter()
            .any(|action| action.commitment == alice_fill.commitment())
    );
    Ok(())
}

#[test]
fn prove_errors_on_a_fill_with_another_quantity_than_the_terms() -> anyhow::Result<()> {
    let [_, banana] = issuers();
    let intent = Intent::new(&banana, 4, &Party::new(1), 101);
    let terms = intent.terms()?;
    let resource = terms.intent(
        intent_logic_ref()?,
        intent.nullifier_key,
        Fr::from(7u64),
        Fr::from(8u64),
    );
    let mut witness = intent.fill(&resource, Belt::default().root(), 41)?;
    witness.created.quantity += 1;

    let result = action::prove(fixtures::circuit_prover(), &witness);

    let Err(Error::Circuit(CircuitError::Unsatisfied { reason, .. })) = result else {
        anyhow::bail!("expected inputs that do not satisfy the intent logic, got {result:?}");
    };
    assert!(
        reason.contains("the created quantity is not the quantity of the terms"),
        "{reason}"
    );
    Ok(())
}

/// An intent of a party: what it wants, and the nullifier key that it gives to the solver.
struct Intent {
    want_issuer: Party,
    want_quantity: u128,
    receiver: Party,
    /// Makes the nullifier key of the intent and the random seed of its fill.
    seed: u64,
    nullifier_key: NullifierKey,
}

/// The action of a party that gives up its tokens, and the intent that the action creates.
struct Part {
    partial: PartialTransaction,
    resource: Resource,
}

impl Intent {
    fn new(want_issuer: &Party, want_quantity: u128, receiver: &Party, seed: u64) -> Self {
        Self {
            want_issuer: want_issuer.clone(),
            want_quantity,
            receiver: receiver.clone(),
            seed,
            nullifier_key: NullifierKey(Fr::from(seed)),
        }
    }

    fn terms(&self) -> anyhow::Result<Terms> {
        fixtures::terms(
            &self.want_issuer,
            self.want_quantity,
            &self.receiver,
            self.seed,
        )
    }

    /// Proves the action that consumes the token at `index` of the belt and creates this intent.
    fn give_up(
        &self,
        belt: &Belt,
        index: usize,
        token: Resource,
        owner: &Party,
        rcv: u64,
    ) -> anyhow::Result<Part> {
        let intent = self.terms()?.intent(
            intent_logic_ref()?,
            self.nullifier_key,
            Fr::from(0u64),
            Fr::from(rcv),
        );
        let witness = build_action(
            token,
            owner.token_logic(),
            owner.nullifier_key,
            Membership::Persistent(belt.path(index)?),
            intent,
            Arc::new(IntentLogic {
                terms: self.terms()?,
            }),
            rcv,
        )?;
        let resource = witness.created;
        Ok(Part {
            partial: action::prove(fixtures::circuit_prover(), &witness)?,
            resource,
        })
    }

    /// The action of the solver that consumes this intent and creates the resource of its terms for the receiver. It
    /// shows the historical root `root`.
    fn fill(&self, intent: &Resource, root: Fr, rcv: u64) -> anyhow::Result<ActionWitness> {
        Ok(anoma_intent::fill(
            intent,
            self.nullifier_key,
            &self.terms()?,
            Arc::new(TokenLogic::default()),
            root,
            Scalar::from(rcv),
        )?)
    }
}

/// The action of the solver that creates `created` from an ephemeral resource of its kind with quantity 0. It shows
/// the historical root `root`.
fn take_surplus(created: Resource, root: Fr, rcv: u64) -> anyhow::Result<ActionWitness> {
    Ok(ActionWitness::from_ephemeral(
        created,
        0,
        NullifierKey(Fr::from(rcv)),
        Arc::new(TokenLogic::default()),
        root,
        Scalar::from(rcv),
    )?)
}

/// Executes a transaction, then checks its nullifiers and the root of the belt with its commitments.
async fn execute(chain: &Chain, belt: &mut Belt, transaction: &Transaction) -> anyhow::Result<()> {
    chain.execute(transaction).await?;
    let commitments: Vec<Fr> = transaction
        .actions
        .iter()
        .map(|action| action.commitment)
        .collect();
    belt.append(&commitments);
    assert_eq!(
        chain.verifier.latestCommitmentTreeRoot().call().await?,
        word(belt.root())
    );
    for action in &transaction.actions {
        let nullifier = word(action.nullifier);
        assert!(
            chain
                .verifier
                .isNullifierContained(nullifier)
                .call()
                .await?,
            "the verifier does not contain the nullifier {nullifier}"
        );
    }
    Ok(())
}

/// The issuers of apples and bananas.
fn issuers() -> [Party; 2] {
    [1001, 1002].map(Party::new)
}

fn wrap_keys_directory() -> PathBuf {
    PathBuf::from(env!("CARGO_TARGET_TMPDIR")).join("wrap-keys")
}
