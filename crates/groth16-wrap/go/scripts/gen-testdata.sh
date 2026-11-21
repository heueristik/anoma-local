#!/usr/bin/env bash
# Regenerates testdata/<circuit>/{vk,vk_hash,proof,public_inputs} from the Noir sources in testdata/noir.
# Needs nargo 1.0.0-rc.6 and bb 6.0.0 on PATH and GNU coreutils (run it on Linux or in a Linux container).
set -euo pipefail

root=$(cd "$(dirname "$0")/.." && pwd)

target=noir-recursive-no-zk

command -v nargo >/dev/null && command -v bb >/dev/null || { echo "nargo and bb must be on PATH" >&2; exit 1; }

[ "$(bb --version)" = "6.0.0" ] || { echo "bb 6.0.0 required, found $(bb --version)" >&2; exit 1; }

nargo --version | grep -q 'nargo version = 1.0.0-rc.6' || { echo "nargo 1.0.0-rc.6 required" >&2; exit 1; }

prove() {
	local name=$1

	local src=$root/testdata/noir/$name

	local out=$root/testdata/$name

	(cd "$src" && nargo execute --silence-warnings >/dev/null)

	rm -rf "$out"

	mkdir -p "$out"

	bb write_vk -b "$src/target/$name.json" -o "$out" -t $target >/dev/null

	bb prove -b "$src/target/$name.json" -w "$src/target/$name.gz" -k "$out/vk" -o "$out" -t $target >/dev/null

	bb verify -p "$out/proof" -k "$out/vk" -i "$out/public_inputs" -t $target >/dev/null

	echo "$name: proof generated and verified by bb"
}

hexlist() {
	od -An -v -tx1 -w32 "$1" | tr -d ' ' | sed 's/^/"0x/; s/$/"/' | paste -sd, -
}

prove keccak_8

prove ecc_1

prove mixed

prove pub_3

# recurse_1 verifies the keccak_8 proof inside Noir, so its pairing points are non-default.
inner=$root/testdata/keccak_8

{
	echo "vk = [$(hexlist "$inner/vk")]"

	echo "proof = [$(hexlist "$inner/proof")]"

	echo "pi = [$(hexlist "$inner/public_inputs")]"

	echo "key_hash = $(hexlist "$inner/vk_hash")"
} > "$root/testdata/noir/recurse_1/Prover.toml"

prove recurse_1

(cd "$root/testdata" && sha256sum */vk */vk_hash */proof */public_inputs > SHA256SUMS)

echo "nargo $(nargo --version | head -1 | awk '{print $4}'), bb $(bb --version)" > "$root/testdata/TOOLCHAIN"
