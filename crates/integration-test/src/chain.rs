//! A local anvil chain with the AnomaVerifier contract and the Groth16 verifier of the wrap keys.

use std::path::Path;
use std::process::Command;

use alloy::network::TransactionBuilder;
use alloy::node_bindings::{Anvil, AnvilInstance};
use alloy::primitives::{Address, Bytes};
use alloy::providers::{DynProvider, Provider, ProviderBuilder};
use alloy::rpc::types::{TransactionReceipt, TransactionRequest};
use alloy::signers::local::PrivateKeySigner;
use anoma_arm::digest::Deployment;
use anoma_bindings::conversion;
use anoma_bindings::generated::anoma_verifier::AnomaVerifier::{self, AnomaVerifierInstance};
use anoma_groth16_wrap::{WrapKeys, solidity_verifier};
use anoma_prover::Transaction;
use anyhow::{Context, ensure};

/// The settings of the default profile in `contracts/foundry.toml` that change the bytecode.
const BYTECODE_SETTINGS: [&str; 7] = [
    "solc",
    "auto_detect_solc",
    "evm_version",
    "optimizer",
    "optimizer_runs",
    "via_ir",
    "bytecode_hash",
];

/// An anvil chain, which stops when this drops, and the AnomaVerifier on it.
pub struct Chain {
    _anvil: AnvilInstance,
    pub verifier: AnomaVerifierInstance<DynProvider>,
    /// The chain ID and the address of the verifier, to which the proofs of its transactions are bound.
    pub deployment: Deployment,
}

impl Chain {
    /// Spawns anvil, compiles the Solidity verifier of `keys` with forge in `work`, and deploys it and the
    /// AnomaVerifier.
    pub async fn deploy(keys: &WrapKeys, work: &Path) -> anyhow::Result<Self> {
        let anvil = Anvil::new()
            .args(["--hardfork", "osaka"])
            .try_spawn()
            .context("spawn anvil")?;
        let signer: PrivateKeySigner = anvil.keys()[0].clone().into();
        let provider = ProviderBuilder::new()
            .wallet(signer)
            .connect_http(anvil.endpoint_url())
            .erased();

        let bytecode = compile_groth16_verifier(&solidity_verifier(keys)?, work)?;
        let receipt = provider
            .send_transaction(TransactionRequest::default().with_deploy_code(bytecode))
            .await?
            .get_receipt()
            .await?;
        let groth16_verifier: Address = receipt
            .contract_address
            .context("the deployment of the Groth16 verifier has no contract address")?;
        let chain_id = provider.get_chain_id().await?;
        let verifier = AnomaVerifier::deploy(provider, groth16_verifier)
            .await
            .context("deploy the AnomaVerifier")?;
        let deployment = Deployment {
            chain_id,
            verifier: verifier.address().into_array(),
        };
        Ok(Self {
            _anvil: anvil,
            verifier,
            deployment,
        })
    }

    /// Executes a transaction and returns its receipt; a reverted execution is an error.
    pub async fn execute(&self, transaction: &Transaction) -> anyhow::Result<TransactionReceipt> {
        let receipt = self
            .verifier
            .execute(conversion::transaction(
                &transaction.actions,
                &transaction.proof.bytes,
            ))
            .send()
            .await
            .context("send the transaction")?
            .get_receipt()
            .await?;
        ensure!(
            receipt.status(),
            "the execution of {} reverted",
            receipt.transaction_hash
        );
        Ok(receipt)
    }
}

/// Compiles the Groth16 verifier with the settings of the contracts and returns its creation code.
fn compile_groth16_verifier(source: &str, work: &Path) -> anyhow::Result<Bytes> {
    let project = work.join("groth16-verifier");
    std::fs::create_dir_all(project.join("src"))
        .with_context(|| format!("create {}", project.display()))?;
    std::fs::write(project.join("foundry.toml"), bytecode_settings()?)?;
    std::fs::write(project.join("src/Groth16Verifier.sol"), source)?;

    let forge = std::env::var_os("FORGE").unwrap_or_else(|| "forge".into());
    let output = Command::new(&forge)
        .args(["build", "--root"])
        .arg(&project)
        .output()
        .context("run forge build")?;
    ensure!(
        output.status.success(),
        "forge build of the Groth16 verifier failed: {}",
        String::from_utf8_lossy(&output.stderr)
    );

    let artifact = project.join("out/Groth16Verifier.sol/Verifier.json");
    let json: serde_json::Value = serde_json::from_slice(
        &std::fs::read(&artifact).with_context(|| format!("read {}", artifact.display()))?,
    )?;
    let object = json["bytecode"]["object"]
        .as_str()
        .with_context(|| format!("{} has no bytecode", artifact.display()))?;
    Ok(object.parse()?)
}

/// A `foundry.toml` with the bytecode settings of the contracts.
fn bytecode_settings() -> anyhow::Result<String> {
    let path = Path::new(env!("CARGO_MANIFEST_DIR")).join("../../contracts/foundry.toml");
    let config: toml::Table = std::fs::read_to_string(&path)
        .with_context(|| format!("read {}", path.display()))?
        .parse()?;
    let profile = config
        .get("profile")
        .and_then(|profiles| profiles.get("default"))
        .and_then(toml::Value::as_table)
        .with_context(|| format!("{} has no default profile", path.display()))?;
    let mut settings = toml::Table::new();
    for name in BYTECODE_SETTINGS {
        let value = profile
            .get(name)
            .with_context(|| format!("{} does not set {name}", path.display()))?;
        settings.insert(name.to_string(), value.clone());
    }
    let mut profiles = toml::Table::new();
    profiles.insert("default".to_string(), settings.into());
    let mut foundry = toml::Table::new();
    foundry.insert("profile".to_string(), profiles.into());
    Ok(toml::to_string(&foundry)?)
}
