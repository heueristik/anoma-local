# Test vectors

Each directory holds the files written by `bb write_vk` and `bb prove` with `-t noir-recursive-no-zk`
(`vk`, `vk_hash`, `proof`, `public_inputs`) for one Noir circuit in `noir/`. They were generated with
nargo 1.0.0-rc.6 and bb 6.0.0 by `scripts/gen-testdata.sh`; `SHA256SUMS` lists the file hashes.

| Vector | What it exercises |
| --- | --- |
| keccak_8 | 8 Keccak-f permutations: lookup tables, arithmetic, delta range |
| ecc_1 | Grumpkin scalar multiplication and addition: elliptic gates, memory |
| mixed | 24 Keccak-f permutations, Poseidon2, read-only and read-write arrays with witness indices, range checks and a Grumpkin scalar multiplication; two public inputs |
| recurse_1 | verifies the keccak_8 proof in Noir: non-native field gates, Poseidon2, non-default pairing points |
| pub_3 | three public inputs, so their order and the public input delta are exercised |

Together they give every one of the 31 subrelations a nonzero selector in at least one vector.
