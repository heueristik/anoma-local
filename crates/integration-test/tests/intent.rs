//! The intent example: Alice gives 4 apples for 4 bananas, Bob gives 7 bananas for 3 apples, and Sally solves in
//! several composition steps and keeps the surplus. Behind `slow-tests`: it needs nargo, bb, forge and anvil, and the
//! first run makes the Groth16 setup, which takes minutes and about 8 GB of memory.

#![cfg(feature = "slow-tests")]

use std::path::PathBuf;

use anoma_arm::Fr;
use anoma_arm::Scalar;
use anoma_arm::belt::Belt;
use anoma_arm::resource::{NullifierKey, Resource};
use anoma_bindings::conversion::word;
use anoma_integration_test::chain::Chain;
use anoma_integration_test::fixtures::{self, action as build_action, intent_logic_ref, token};
use anoma_intent::Terms;
use anoma_prover::action::{ActionWitness, Logic};
use anoma_prover::{Error, PartialTransaction, Transaction, action, transaction};
use tempfile::TempDir;

const APPLE: u64 = 1;
const BANANA: u64 = 2;

#[tokio::test(flavor = "multi_thread")]
async fn execute_settles_the_intent_example_after_several_composition_steps() -> anyhow::Result<()>
{
    let toolchain = fixtures::toolchain();
    let keys = fixtures::wrap_keys(&wrap_keys_directory())?;
    let work = TempDir::new()?;
    let chain = Chain::deploy(&keys, work.path()).await?;
    let mut belt = Belt::default();
    let [alice, bob, sally] = [1u64, 2, 3].map(|seed| NullifierKey(Fr::from(seed)));

    let alice_apples = fixtures::mint(token(APPLE, 4, alice, 11)?, 21)?;
    let bob_bananas = fixtures::mint(token(BANANA, 7, bob, 12)?, 22)?;
    let mint =
        action::prove(toolchain, &alice_apples)?.compose(action::prove(toolchain, &bob_bananas)?);
    execute(
        &chain,
        &mut belt,
        &transaction::prove(toolchain, &mint, &keys)?,
    )
    .await?;

    // Each party proves its own action, which gives up its tokens for an intent, and gives Sally the intent.
    let alice_intent = Intent::new(BANANA, 4, alice, 101);
    let alice_part = alice_intent.give_up(&belt, 0, alice_apples.created, alice, 31)?;
    let bob_intent = Intent::new(APPLE, 3, bob, 102);
    let bob_part = bob_intent.give_up(&belt, 1, bob_bananas.created, bob, 32)?;

    let composed = alice_part.partial.compose(bob_part.partial);
    assert!(!composed.is_balanced());
    let composed = composed.compose(action::prove(
        toolchain,
        &alice_intent.fill(&alice_part.resource, 41)?,
    )?);
    assert!(!composed.is_balanced());
    let composed = composed.compose(action::prove(
        toolchain,
        &bob_intent.fill(&bob_part.resource, 42)?,
    )?);
    assert!(
        !composed.is_balanced(),
        "the surplus of 1 apple and 3 bananas is still open"
    );
    let composed = composed
        .compose(action::prove(
            toolchain,
            &take_surplus(token(APPLE, 1, sally, 15)?, 51)?,
        )?)
        .compose(action::prove(
            toolchain,
            &take_surplus(token(BANANA, 3, sally, 16)?, 52)?,
        )?);
    assert!(composed.is_balanced());

    let settlement = transaction::prove(toolchain, &composed, &keys)?;
    execute(&chain, &mut belt, &settlement).await?;
    // Alice computes the resource that fills her intent from her terms, with no payload from Sally.
    let alice_fill = alice_intent
        .terms()?
        .resource(alice_part.resource.nullifier(alice_intent.nullifier_key)?);
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
    let alice = NullifierKey(Fr::from(1u64));
    let intent = Intent::new(BANANA, 4, alice, 101);
    let terms = intent.terms()?;
    let resource = terms.intent(
        intent_logic_ref()?,
        intent.nullifier_key,
        Fr::from(7u64),
        Fr::from(8u64),
    );
    let mut witness = intent.fill(&resource, 41)?;
    witness.created.quantity += 1;
    witness.consumed_logic =
        anoma_intent::consumed_logic(&resource, intent.nullifier_key, &terms, &witness.created)?;

    let result = action::prove(fixtures::toolchain(), &witness);

    let Err(Error::Tool { output, .. }) = result else {
        anyhow::bail!("expected a failure of nargo, got {result:?}");
    };
    assert!(
        output.contains("the created quantity is not the quantity of the terms"),
        "{output}"
    );
    Ok(())
}

/// An intent of a party: what it wants, and the nullifier key that it gives to the solver.
struct Intent {
    want_label: u64,
    min_quantity: u128,
    receiver: NullifierKey,
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
    fn new(want_label: u64, min_quantity: u128, receiver: NullifierKey, seed: u64) -> Self {
        Self {
            want_label,
            min_quantity,
            receiver,
            seed,
            nullifier_key: NullifierKey(Fr::from(seed)),
        }
    }

    fn terms(&self) -> anyhow::Result<Terms> {
        fixtures::terms(self.want_label, self.min_quantity, self.receiver, self.seed)
    }

    /// Proves the action that consumes the token at `index` of the belt and creates this intent.
    fn give_up(
        &self,
        belt: &Belt,
        index: usize,
        token: Resource,
        owner: NullifierKey,
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
            Logic::trivial(),
            owner,
            Some(belt.path(index)?),
            intent,
            anoma_intent::created_logic()?,
            rcv,
        )?;
        let resource = witness.created;
        Ok(Part {
            partial: action::prove(fixtures::toolchain(), &witness)?,
            resource,
        })
    }

    /// The action of the solver that consumes this intent and creates the resource of its terms for the receiver.
    fn fill(&self, intent: &Resource, rcv: u64) -> anyhow::Result<ActionWitness> {
        Ok(anoma_intent::fill(
            intent,
            self.nullifier_key,
            &self.terms()?,
            Logic::trivial(),
            Scalar::from(rcv),
        )?)
    }
}

/// The action of the solver that creates `created` from an ephemeral resource of its kind with quantity 0.
fn take_surplus(created: Resource, rcv: u64) -> anyhow::Result<ActionWitness> {
    Ok(ActionWitness::from_ephemeral(
        created,
        0,
        NullifierKey(Fr::from(rcv)),
        Logic::trivial(),
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

fn wrap_keys_directory() -> PathBuf {
    PathBuf::from(env!("CARGO_TARGET_TMPDIR")).join("wrap-keys")
}
