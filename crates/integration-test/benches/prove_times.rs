//! Measures the time of each proof of a wrap, a transfer and an unwrap without fees, of a transfer with change, and of a
//! split of one token into three, and prints the medians as a Markdown table. It needs the compiled circuits, nargo and
//! bb, as the slow tests do.

use std::cell::{Cell, RefCell};
use std::collections::BTreeSet;
use std::fmt;
use std::path::PathBuf;
use std::time::{Duration, Instant};

use anoma_arm::belt::Belt;
use anoma_arm::digest::Deployment;
use anoma_arm::resource::{NullifierKey, Resource};
use anoma_arm::{Fr, Scalar};
use anoma_groth16_wrap::Groth16Wrapper;
use anoma_integration_test::fixtures::{self, Party, token};
use anoma_prover::action::{self, ActionWitness, Membership};
use anoma_prover::circuit_prover::{
    CircuitError, CircuitInputs, CircuitProver, Proof, Target, VerificationKey,
};
use anoma_prover::root_proof_wrapper::{RootProofWrapper, TransactionProof};
use anoma_prover::transaction;
use anyhow::Context;

/// The number of times that each transaction is proved. The first proof of a process also reads the keys.
const ROUNDS: usize = 3;

/// A row of the table. The order of the variants and their fields is the order of the rows.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
enum Row {
    Action { action: usize, proof: ActionProof },
    Root,
    Wrap,
    Total,
}

/// A proof of one action: its leaf proofs in the order that `action::prove` makes them, then its step proof.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
enum ActionProof {
    Compliance,
    Logic { resource: usize },
    Step,
}

impl fmt::Display for Row {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Action { action, proof } => match proof {
                ActionProof::Compliance => write!(formatter, "Action {action} compliance"),
                ActionProof::Logic { resource } => {
                    write!(formatter, "Action {action} logic of resource {resource}")
                }
                ActionProof::Step => write!(formatter, "Action {action} step"),
            },
            Self::Root => write!(formatter, "Root"),
            Self::Wrap => write!(formatter, "Groth16 wrap"),
            Self::Total => write!(formatter, "**Total**"),
        }
    }
}

fn main() -> anyhow::Result<()> {
    let wrapper =
        fixtures::groth16_wrapper(&PathBuf::from(env!("CARGO_TARGET_TMPDIR")).join("wrap-keys"))?;
    let transactions = transactions()?;
    let mut rounds: Vec<Vec<Vec<(Row, Duration)>>> = vec![Vec::new(); transactions.len()];
    for round in 0..ROUNDS {
        for ((name, witnesses), times) in transactions.iter().zip(&mut rounds) {
            times.push(
                prove_times(&wrapper, witnesses)
                    .with_context(|| format!("prove the {name} in round {round}"))?,
            );
        }
    }

    println!(
        "| Proof | {} |",
        transactions.each_ref().map(|(name, _)| *name).join(" | ")
    );
    println!("| --- |{}", " --- |".repeat(transactions.len()));
    let rows: BTreeSet<Row> = rounds
        .iter()
        .flatten()
        .flatten()
        .map(|&(row, _)| row)
        .collect();
    for row in rows {
        let medians = rounds.iter().map(|times| {
            let mut values: Vec<Duration> = times
                .iter()
                .filter_map(|round| round.iter().find(|&&(name, _)| name == row))
                .map(|&(_, time)| time)
                .collect();
            values.sort();
            values
                .get(values.len() / 2)
                .map_or("–".to_string(), |median| {
                    format!("{:.2}", median.as_secs_f64())
                })
        });
        println!("| {row} | {} |", medians.collect::<Vec<_>>().join(" | "));
    }
    Ok(())
}

/// The transactions: their names, and the witnesses of their actions.
fn transactions() -> anyhow::Result<[(&'static str, Vec<ActionWitness>); 5]> {
    let [alice, bob, issuer] = [1u64, 2, 1001].map(Party::new);
    let wrap = fixtures::mint(&issuer, token(&issuer, 5, &alice, 11)?, 21)?;
    let held = fixtures::mint(&issuer, token(&issuer, 5, &alice, 12)?, 22)?;
    let mut belt = Belt::default();
    let held_token = fixtures::first_created(&held)?;
    belt.append(&[
        fixtures::first_created(&wrap)?.commitment(),
        held_token.commitment(),
    ]);
    let spend_held = |created: Resource, rcv: u64| {
        fixtures::action(
            held_token,
            alice.token_logic(),
            alice.nullifier_key,
            Membership::Persistent(belt.path(1)?),
            created,
            bob.token_logic(),
            rcv,
        )
    };
    let transfer = spend_held(token(&issuer, 5, &bob, 13)?, 23)?;
    let unwrap = spend_held(
        Resource {
            is_ephemeral: true,
            ..token(&issuer, 5, &alice, 14)?
        },
        24,
    )?;
    let partial_transfer = spend_held(token(&issuer, 3, &bob, 15)?, 25)?;
    // The change comes from an ephemeral token of quantity 0, which needs no signature.
    let change = ActionWitness::from_ephemeral(
        token(&issuer, 2, &alice, 16)?,
        0,
        NullifierKey(Fr::from(26u64)),
        alice.token_logic(),
        belt.root(),
        Scalar::from(26u64),
    )?;
    let split = fixtures::split(
        held_token,
        alice.token_logic(),
        alice.nullifier_key,
        Membership::Persistent(belt.path(1)?),
        &[
            token(&issuer, 1, &bob, 17)?,
            token(&issuer, 2, &bob, 18)?,
            token(&issuer, 2, &bob, 19)?,
        ],
        bob.token_logic(),
        27,
    )?;
    Ok([
        ("Wrap", vec![wrap]),
        ("Transfer", vec![transfer]),
        ("Unwrap", vec![unwrap]),
        ("Transfer with change", vec![partial_transfer, change]),
        ("Split into three", vec![split]),
    ])
}

/// Proves the transaction of the actions, and gives the time of each proof and the total time, in order.
fn prove_times(
    wrapper: &Groth16Wrapper,
    witnesses: &[ActionWitness],
) -> anyhow::Result<Vec<(Row, Duration)>> {
    let prover = TimedProver {
        inner: fixtures::circuit_prover(),
        times: RefCell::default(),
    };
    let timed_wrapper = TimedWrapper {
        inner: wrapper,
        time: Cell::default(),
    };
    let deployment = Deployment {
        chain_id: 31337,
        verifier: [0; 20],
    };
    let start = Instant::now();
    let partial = witnesses
        .iter()
        .map(|witness| action::prove(&prover, witness))
        .reduce(|composed, proved| Ok(composed?.compose(proved?)))
        .context("the transaction has no action")??;
    transaction::prove(&prover, &timed_wrapper, &partial, &deployment)?;
    let total = start.elapsed();

    // The prover makes the leaf proofs of each action, then one step proof for each action, then the root proof.
    let leaf_rows = witnesses.iter().enumerate().flat_map(|(action, witness)| {
        std::iter::once(ActionProof::Compliance)
            .chain((0..witness.resources.len()).map(|resource| ActionProof::Logic { resource }))
            .map(move |proof| Row::Action { action, proof })
    });
    let step_rows = (0..witnesses.len()).map(|action| Row::Action {
        action,
        proof: ActionProof::Step,
    });
    let rows: Vec<Row> = leaf_rows.chain(step_rows).chain([Row::Root]).collect();
    let proof_times = prover.times.into_inner();
    anyhow::ensure!(
        proof_times.len() == rows.len(),
        "the prover made {} proofs, expected {}",
        proof_times.len(),
        rows.len()
    );
    let mut times: Vec<(Row, Duration)> = rows.into_iter().zip(proof_times).collect();
    times.push((Row::Wrap, timed_wrapper.time.get()));
    times.push((Row::Total, total));
    Ok(times)
}

/// A circuit prover that records the time of each proof of the circuit prover that it holds.
struct TimedProver<'a> {
    inner: &'a dyn CircuitProver,
    times: RefCell<Vec<Duration>>,
}

impl CircuitProver for TimedProver<'_> {
    fn verification_key(&self, circuit: &str) -> Result<VerificationKey, CircuitError> {
        self.inner.verification_key(circuit)
    }

    fn prove(
        &self,
        circuit: &str,
        inputs: &CircuitInputs,
        target: Target,
    ) -> Result<Proof, CircuitError> {
        let start = Instant::now();
        let proof = self.inner.prove(circuit, inputs, target)?;
        self.times.borrow_mut().push(start.elapsed());
        Ok(proof)
    }

    fn verify(
        &self,
        circuit: &str,
        key: &VerificationKey,
        proof: &[Fr],
        public_inputs: &[Fr],
        target: Target,
    ) -> Result<(), CircuitError> {
        self.inner
            .verify(circuit, key, proof, public_inputs, target)
    }
}

/// A root proof wrapper that records the time of the wrap of the Groth16 wrapper that it holds.
struct TimedWrapper<'a> {
    inner: &'a Groth16Wrapper,
    time: Cell<Duration>,
}

impl RootProofWrapper for TimedWrapper<'_> {
    type Error = <Groth16Wrapper as RootProofWrapper>::Error;

    fn wrap(&self, root_proof: &Proof) -> Result<TransactionProof, Self::Error> {
        let start = Instant::now();
        let proof = self.inner.wrap(root_proof)?;
        self.time.set(start.elapsed());
        Ok(proof)
    }
}
