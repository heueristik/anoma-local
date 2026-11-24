//! Runs the `Belt` of `anoma-arm` for the differential contract tests: `anoma-belt-ffi <command> <input>`.
//!
//! The input and the output are ABI encodings in hexadecimal with a `0x` prefix, as `vm.toString` writes them and
//! `vm.ffi` reads them:
//! - `roots (bytes32[] leaves, uint256[] appendSizes)` gives `bytes32[]`, the root after each append.
//! - `path (bytes32[] leaves, uint256 index)` gives `(bytes32[] siblings, bool[] isLeft)`, from the leaf up.

use alloy_primitives::{B256, U256, hex};
use alloy_sol_types::SolValue;
use anoma_arm::Fr;
use anoma_arm::belt::Belt;
use anoma_arm::field::{from_bytes, to_bytes};
use anyhow::{Context, bail, ensure};

fn main() -> anyhow::Result<()> {
    let mut arguments = std::env::args().skip(1);
    let (Some(command), Some(input), None) = (arguments.next(), arguments.next(), arguments.next())
    else {
        bail!("usage: anoma-belt-ffi <roots|path> <hex input>");
    };
    let input = hex::decode(&input).context("the input is not hexadecimal")?;
    let output = match command.as_str() {
        "roots" => roots(&input)?,
        "path" => path(&input)?,
        _ => bail!("unknown command {command}"),
    };
    print!("{}", hex::encode_prefixed(output));
    Ok(())
}

fn roots(input: &[u8]) -> anyhow::Result<Vec<u8>> {
    let (leaves, append_sizes) = <(Vec<B256>, Vec<U256>)>::abi_decode_params(input)
        .context("the input is not (bytes32[], uint256[])")?;
    let leaves = to_fields(&leaves)?;
    let mut belt = Belt::default();
    let mut roots = Vec::with_capacity(append_sizes.len());
    let mut next = 0;
    for (append, size) in append_sizes.iter().enumerate() {
        let size = usize::try_from(*size)
            .with_context(|| format!("append {append} has the size {size}"))?;
        let batch = leaves.get(next..next + size).with_context(|| {
            format!(
                "append {append} of {size} leaves goes past the {} leaves",
                leaves.len()
            )
        })?;
        belt.append(batch);
        roots.push(B256::from(to_bytes(belt.root())));
        next += size;
    }
    ensure!(
        next == leaves.len(),
        "the appends take {next} of the {} leaves",
        leaves.len()
    );
    Ok(roots.abi_encode())
}

fn path(input: &[u8]) -> anyhow::Result<Vec<u8>> {
    let (leaves, index) = <(Vec<B256>, U256)>::abi_decode_params(input)
        .context("the input is not (bytes32[], uint256)")?;
    let index =
        usize::try_from(index).with_context(|| format!("the index {index} is too large"))?;
    let mut belt = Belt::default();
    belt.append(&to_fields(&leaves)?);
    let path = belt
        .path(index)
        .with_context(|| format!("no path for the leaf {index}"))?;
    let (siblings, is_left): (Vec<B256>, Vec<bool>) = path
        .siblings
        .iter()
        .map(|sibling| (B256::from(to_bytes(sibling.hash)), sibling.is_left))
        .unzip();
    Ok((siblings, is_left).abi_encode_params())
}

fn to_fields(leaves: &[B256]) -> anyhow::Result<Vec<Fr>> {
    leaves
        .iter()
        .enumerate()
        .map(|(index, leaf)| {
            from_bytes(leaf.0).with_context(|| format!("leaf {index} is not in the field"))
        })
        .collect()
}
