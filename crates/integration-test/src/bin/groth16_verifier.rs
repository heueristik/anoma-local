//! Writes the Solidity verifier of the Groth16 wrap for the compiled root circuit: `groth16_verifier <keys directory>
//! <output file>`. If the keys directory has no keys for the root circuit, it makes the single-party setup, whose party
//! can make false proofs; so the verifier is for tests only. A comment below the license identifier names the key hash
//! of the root circuit, which `just groth16-verifier-check` compares with the compiled root circuit.

use std::path::PathBuf;

use anoma_arm::field::to_hex;
use anoma_groth16_wrap::solidity_verifier;
use anoma_integration_test::fixtures;
use anoma_prover::circuit_prover::CircuitProver;
use anoma_prover::transaction::ROOT;
use anyhow::Context;

fn main() -> anyhow::Result<()> {
    let [keys, output]: [PathBuf; 2] = std::env::args_os()
        .skip(1)
        .map(PathBuf::from)
        .collect::<Vec<_>>()
        .try_into()
        .map_err(|_| anyhow::anyhow!("usage: groth16_verifier <keys directory> <output file>"))?;
    let root = fixtures::circuit_prover()
        .verification_key(ROOT)
        .context("write the key of the root circuit")?;
    let wrapper = fixtures::groth16_wrapper(&keys)?;
    let source = solidity_verifier(&wrapper.keys).context("export the Solidity verifier")?;
    let header = format!(
        "//! Generated for the root circuit with the key hash {}.\n\
         //! The keys come from a single-party setup. Its party can make false proofs, so this verifier is for tests \
         only.\n",
        to_hex(root.hash)
    );
    let (license, rest) = source
        .trim_start()
        .split_once('\n')
        .filter(|(license, _)| license.starts_with("// SPDX-License-Identifier:"))
        .context("the exported verifier does not start with a license identifier")?;
    std::fs::write(&output, format!("{license}\n\n{header}{rest}"))
        .with_context(|| format!("write {}", output.display()))?;
    println!(
        "Wrote the verifier for the root circuit with the key hash {} to {}",
        to_hex(root.hash),
        output.display()
    );
    Ok(())
}
