//! Measures the gas that a wrap, a transfer and an unwrap without fees, a transfer with change, and a split of one token
//! into three use on anvil with the Osaka hardfork, and prints it as a Markdown table. The created persistent tokens carry discovery app data for
//! their owners. It needs the compiled circuits, nargo, bb, forge and anvil, as the slow tests do.

use std::path::PathBuf;

use alloy::consensus::Transaction as _;
use alloy::primitives::{Address, TxHash};
use alloy::providers::Provider;
use anoma_arm::belt::Belt;
use anoma_arm::resource::{NullifierKey, Resource};
use anoma_arm::{Fr, Scalar};
use anoma_discovery::{Discovery, GrumpkinDiscovery, GrumpkinSecretKeys};
use anoma_groth16_wrap::Groth16Wrapper;
use anoma_integration_test::chain::Chain;
use anoma_integration_test::fixtures::{self, Party, token};
use anoma_prover::action::{self, ActionWitness, Membership};
use anoma_prover::transaction;
use anyhow::Context;
use tempfile::TempDir;

/// The base cost of a transaction.
const BASE_GAS: u64 = 21_000;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let wrapper =
        fixtures::groth16_wrapper(&PathBuf::from(env!("CARGO_TARGET_TMPDIR")).join("wrap-keys"))?;
    let work = TempDir::new()?;
    let chain = Chain::deploy(&wrapper.keys, work.path()).await?;
    let groth16_verifier = chain.verifier.GROTH16_VERIFIER().call().await?;
    let [alice, bob, issuer] = [1u64, 2, 1001].map(Party::new);
    let [alice_keys, bob_keys] = [(); 2].map(|()| GrumpkinSecretKeys::random());
    let mut belt = Belt::default();

    // Four tokens of Alice, which the transfer, the unwrap, the transfer with change and the split consume.
    let held: Vec<ActionWitness> = [11, 12, 13, 14]
        .into_iter()
        .map(|seed| fixtures::mint(&issuer, token(&issuer, 5, &alice, seed)?, seed + 10))
        .collect::<anyhow::Result<_>>()?;
    execute(&chain, &wrapper, &mut belt, &held).await?;
    let held_tokens = held
        .iter()
        .map(fixtures::first_created)
        .collect::<anyhow::Result<Vec<_>>>()?;
    let spend = |index: usize, created: Resource, rcv: u64, belt: &Belt| {
        fixtures::action(
            held_tokens[index],
            alice.token_logic(),
            alice.nullifier_key,
            Membership::Persistent(belt.path(index)?),
            created,
            bob.token_logic(),
            rcv,
        )
    };

    let mut executed = Vec::new();
    let wrap = [discoverable(
        fixtures::mint(&issuer, token(&issuer, 5, &alice, 15)?, 25)?,
        &alice_keys,
    )?];
    executed.push(("Wrap", execute(&chain, &wrapper, &mut belt, &wrap).await?));
    let transfer = [discoverable(
        spend(0, token(&issuer, 5, &bob, 16)?, 26, &belt)?,
        &bob_keys,
    )?];
    executed.push((
        "Transfer",
        execute(&chain, &wrapper, &mut belt, &transfer).await?,
    ));
    let ephemeral = Resource {
        is_ephemeral: true,
        ..token(&issuer, 5, &alice, 17)?
    };
    let unwrap = [spend(1, ephemeral, 27, &belt)?];
    executed.push((
        "Unwrap",
        execute(&chain, &wrapper, &mut belt, &unwrap).await?,
    ));
    // The change comes from an ephemeral token of quantity 0, which needs no signature.
    let change = ActionWitness::from_ephemeral(
        token(&issuer, 2, &alice, 18)?,
        0,
        NullifierKey(Fr::from(28u64)),
        alice.token_logic(),
        belt.root(),
        Scalar::from(28u64),
    )?;
    let transfer_with_change = [
        discoverable(
            spend(2, token(&issuer, 3, &bob, 19)?, 29, &belt)?,
            &bob_keys,
        )?,
        discoverable(change, &alice_keys)?,
    ];
    executed.push((
        "Transfer with change",
        execute(&chain, &wrapper, &mut belt, &transfer_with_change).await?,
    ));
    let split = [discoverable(
        fixtures::split(
            held_tokens[3],
            alice.token_logic(),
            alice.nullifier_key,
            Membership::Persistent(belt.path(3)?),
            &[
                token(&issuer, 1, &bob, 20)?,
                token(&issuer, 2, &bob, 21)?,
                token(&issuer, 2, &bob, 22)?,
            ],
            bob.token_logic(),
            30,
        )?,
        &bob_keys,
    )?];
    executed.push((
        "Split into three",
        execute(&chain, &wrapper, &mut belt, &split).await?,
    ));

    let mut gas = Vec::new();
    for (name, hash) in &executed {
        gas.push((*name, Gas::of(&chain, *hash, groth16_verifier).await?));
    }
    println!(
        "| Gas | {} |",
        gas.iter()
            .map(|(name, _)| *name)
            .collect::<Vec<_>>()
            .join(" | ")
    );
    println!("| --- |{}", " --- |".repeat(gas.len()));
    let columns: Vec<_> = gas.iter().map(|(_, gas)| gas.rows()).collect();
    for row in 0..columns[0].len() {
        let values: Vec<String> = columns
            .iter()
            .map(|column| column[row].1.to_string())
            .collect();
        println!("| {} | {} |", columns[0][row].0, values.join(" | "));
    }
    Ok(())
}

/// The gas of an executed transaction.
struct Gas {
    total: u64,
    calldata_bytes: u64,
    /// The calldata cost of EIP-2028; EIP-7623 makes the transaction pay more only if its execution uses little gas.
    calldata: u64,
    groth16_verifier: u64,
}

impl Gas {
    /// The rows of the table: their names and values.
    fn rows(&self) -> [(&'static str, u64); 6] {
        [
            ("Calldata bytes", self.calldata_bytes),
            ("Base cost", BASE_GAS),
            ("Calldata", self.calldata),
            ("Groth16 verifier call", self.groth16_verifier),
            (
                "Rest of the AnomaVerifier",
                self.total - BASE_GAS - self.calldata - self.groth16_verifier,
            ),
            ("**Total**", self.total),
        ]
    }

    async fn of(chain: &Chain, hash: TxHash, groth16_verifier: Address) -> anyhow::Result<Self> {
        let provider = chain.verifier.provider();
        let receipt = provider
            .get_transaction_receipt(hash)
            .await?
            .with_context(|| format!("no receipt for {hash}"))?;
        let transaction = provider
            .get_transaction_by_hash(hash)
            .await?
            .with_context(|| format!("no transaction {hash}"))?;
        let input = transaction.input();
        let zero_bytes = input.iter().filter(|&&byte| byte == 0).count() as u64;
        let trace: serde_json::Value = provider
            .raw_request(
                "debug_traceTransaction".into(),
                (hash, serde_json::json!({ "tracer": "callTracer" })),
            )
            .await?;
        let groth16_verifier = trace["calls"]
            .as_array()
            .into_iter()
            .flatten()
            .filter(|call| {
                call["to"]
                    .as_str()
                    .is_some_and(|to| to.eq_ignore_ascii_case(&groth16_verifier.to_string()))
            })
            .map(gas_used)
            .sum::<anyhow::Result<u64>>()?;
        Ok(Self {
            total: receipt.gas_used,
            calldata_bytes: input.len() as u64,
            calldata: 4 * zero_bytes + 16 * (input.len() as u64 - zero_bytes),
            groth16_verifier,
        })
    }
}

/// The action with app data that makes its created resource discoverable for the receiver of `keys`.
fn discoverable(
    mut witness: ActionWitness,
    keys: &GrumpkinSecretKeys,
) -> anyhow::Result<ActionWitness> {
    for created in witness.created_mut() {
        created.app_data =
            GrumpkinDiscovery.app_data(&keys.address(), &created.resource.to_fields())?;
    }
    Ok(witness)
}

/// Proves and executes the transaction of the actions, adds their commitments to `belt`, and returns its hash.
async fn execute(
    chain: &Chain,
    wrapper: &Groth16Wrapper,
    belt: &mut Belt,
    witnesses: &[ActionWitness],
) -> anyhow::Result<TxHash> {
    let prover = fixtures::circuit_prover();
    let partial = witnesses
        .iter()
        .map(|witness| action::prove(prover, witness))
        .reduce(|composed, proved| Ok(composed?.compose(proved?)))
        .context("the transaction has no action")??;
    let transaction = transaction::prove(prover, wrapper, &partial, &chain.deployment)?;
    let receipt = chain.execute(&transaction).await?;
    let commitments: Vec<Fr> = partial
        .actions
        .iter()
        .flat_map(|proved| proved.action.commitments())
        .collect();
    belt.append(&commitments);
    Ok(receipt.transaction_hash)
}

/// The gas that a call of a `callTracer` trace uses, which the trace gives as a hexadecimal string.
fn gas_used(call: &serde_json::Value) -> anyhow::Result<u64> {
    let hex = call["gasUsed"]
        .as_str()
        .with_context(|| format!("the call {call} has no gas"))?;
    Ok(u64::from_str_radix(hex.trim_start_matches("0x"), 16)?)
}
