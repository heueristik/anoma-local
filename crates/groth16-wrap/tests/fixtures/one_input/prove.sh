#!/usr/bin/env bash
# Writes vk, proof and public_inputs of the one_input circuit next to this script with nargo 1.0.0 and bb 6.0.0.
set -euo pipefail

nargo=${NARGO:-nargo}

bb=${BB:-bb}

target=noir-recursive-no-zk

cd "$(dirname "$0")"

[ "$("$bb" --version)" = "6.0.0" ] || { echo "bb 6.0.0 required, found $("$bb" --version)" >&2; exit 1; }

"$nargo" --version | grep -qx 'nargo version = 1.0.0' || { echo "nargo 1.0.0 required" >&2; exit 1; }

"$nargo" compile

"$nargo" execute

"$bb" write_vk --scheme ultra_honk --verifier_target $target --bytecode_path target/one_input.json --output_path .

"$bb" prove --scheme ultra_honk --verifier_target $target --bytecode_path target/one_input.json --witness_path target/one_input.gz --vk_path vk --output_path .

"$bb" verify --scheme ultra_honk --verifier_target $target --proof_path proof --vk_path vk --public_inputs_path public_inputs
