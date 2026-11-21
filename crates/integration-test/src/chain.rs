//! A local anvil chain with the AnomaVerifier contract and the Groth16 verifier of the wrap keys.

use std::path::Path;
use std::process::Command;

use alloy::network::TransactionBuilder;
use alloy::node_bindings::{Anvil, AnvilInstance};
use alloy::primitives::{Address, Bytes};
use alloy::providers::{DynProvider, Provider, ProviderBuilder};
use alloy::rpc::types::{TransactionReceipt, TransactionRequest};
use alloy::signers::local::PrivateKeySigner;
use anoma_bindings::conversion;
use anoma_bindings::generated::anoma_verifier::AnomaVerifier::{self, AnomaVerifierInstance};
use anoma_groth16_wrap::{Keys, solidity_verifier};
use anoma_prover::Transaction;
use anyhow::{Context, ensure};

/// The settings of the contracts in `contracts/foundry.toml` that change the bytecode.
const FOUNDRY_TOML: &str = r#"[profile.default]
solc = "0.8.38"
auto_detect_solc = false
evm_version = "osaka"
optimizer = true
optimizer_runs = 10_000
via_ir = true
bytecode_hash = "none"
"#;

/// An anvil chain, which stops when this drops, and the AnomaVerifier on it.
pub struct Chain {
    _anvil: AnvilInstance,
    pub verifier: AnomaVerifierInstance<DynProvider>,
}

impl Chain {
    /// Spawns anvil, compiles the Solidity verifier of `keys` with forge in `work`, and deploys it and the
    /// AnomaVerifier.
    pub async fn deploy(keys: &Keys, work: &Path) -> anyhow::Result<Self> {
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
        let verifier = AnomaVerifier::deploy(provider, groth16_verifier)
            .await
            .context("deploy the AnomaVerifier")?;
        Ok(Self {
            _anvil: anvil,
            verifier,
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
    std::fs::write(project.join("foundry.toml"), FOUNDRY_TOML)?;
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
