# anoma-groth16-wrap

This crate wraps an UltraHonk proof of Barretenberg (bb) 6.0.0 into a Groth16 proof over BN254. An EVM contract checks the Groth16 proof with the Solidity verifier that gnark exports. The inner proof must use the `noir-recursive-no-zk` target: the Poseidon2 transcript, without zero knowledge. The Go library in `go/` makes the proofs. It is a fork of [ultrahonk-gnark](https://github.com/wstran/ultrahonk-gnark) (see `go/UPSTREAM.md`). The crate builds it as a C archive and calls it through a C interface.

## Interface

```rust
pub fn single_party_setup(inner_verifying_key: &[u8], directory: &Path) -> Result<WrapKeys, Error>;
pub fn prove(keys: &WrapKeys, inner_proof: &[u8], public_inputs: &[[u8; 32]]) -> Result<Groth16Proof, Error>;
pub fn verify(keys: &WrapKeys, proof: &Groth16Proof) -> Result<(), Error>;
pub fn solidity_verifier(keys: &WrapKeys) -> Result<String, Error>;
```

The inputs are the files that bb writes: `vk`, `proof` and `public_inputs`. A field element is 32 bytes, big-endian. `prove` first checks the inner proof natively and returns `Error::Rejected` if it does not verify. `verify` returns `Error::Rejected` for an invalid proof or invalid public inputs.

`Groth16Wrapper` implements the `RootProofWrapper` trait of `anoma-prover` with these functions. `Groth16Wrapper::single_party_setup` sets up the keys for the verification key of the root circuit, and `wrap` turns a root proof into the proof that the AnomaVerifier checks.

The Groth16 proof has one public input for each user public input of the inner proof, in the same order. With one user public input, the contract calls `verifyProof(bytes proof, uint256[1] input)`, which reverts for an invalid proof. `Groth16Proof::bytes` is the `proof` argument: A, B and C in the EIP-197 layout (256 bytes), then one gnark commitment and its proof of knowledge (64 bytes each). The commitment is not a public input: the verifier calculates its public value from the commitment with keccak256. The 8 pairing point limbs of bb are not public inputs either: bb 6.0.0 writes them into the `proof` file, and the circuit checks them as private inputs in its final pairing check.

`WrapKeys` is a directory with these files:

| File | Contents |
| --- | --- |
| `inner_vk` | The inner verifying key. Setup writes it last. |
| `circuit.r1cs` | The wrapper circuit. |
| `pk.bin` | The Groth16 proving key. |
| `vk.bin` | The Groth16 verifying key. |

The keys stay on disk. Only the paths and the small inputs and outputs go through the C interface.

## Trust

- The single-party setup is for tests only. Whoever runs it knows the setup secret and can make false proofs. Production keys need a multi-party ceremony over `circuit.r1cs`, for example with gnark's `backend/groth16/bn254/mpcsetup`. Phase 1 needs powers of tau of size 2^22 or more. This crate has no function that reads keys from a ceremony yet.
- ultrahonk-gnark is experimental and has no audit. Its differential tests compare the circuit with its native verifier, and the native verifier with `bb verify`. They do not show that the constraints reject every false witness.
- The circuit contains the inner verifying key as constants. One keys directory serves one inner circuit. A change of the Noir circuit needs a new setup and a new Solidity verifier.
- The Go module pins gnark v0.16.4. Later gnark releases fix soundness bugs in gadgets that the circuit uses. A move to another gnark release needs a new review of the circuit.
- Anyone can change a valid Groth16 proof into another valid proof for the same public inputs. Do not use the proof bytes as an identifier or a nullifier.

## Build and test

The build needs Go 1.26.8 or later and a C compiler for cgo. `build.rs` runs `go build -buildmode=c-archive` in `go/`. The first build downloads the Go modules in `go/go.sum` into the Go module cache. Set `GO` to use another Go binary.

```sh
cargo test -p anoma-groth16-wrap                         # the fast tests
cargo test -p anoma-groth16-wrap --features slow-tests   # with the single-party setup
(cd crates/groth16-wrap/go && go vet ./... && go test -short ./...)
```

Without `-short`, `go test` also solves the circuit for each upstream test vector, which takes many minutes.

The slow tests write the keys to the cargo directory for test files (`target/tmp`). `tests/fixtures/one_input/prove.sh` writes the fixture files `vk`, `proof` and `public_inputs` again. It needs nargo 1.0.0 and bb 6.0.0; set `NARGO` and `BB` if they are not on the `PATH`.

## Measured

Apple M5 Pro with 18 cores and 64 GB, macOS on arm64, Go 1.27.4, gnark v0.16.4, the `one_input` fixture (one public input). Peak memory is the maximum resident set size from `/usr/bin/time -l`.

| Step | Result |
| --- | --- |
| Wrapper circuit | 3,440,408 R1CS constraints |
| Single-party setup, with compile and the write of the keys | 141 s, 7.9 GB peak |
| Keys | `pk.bin` 1,524,087,595 bytes, `circuit.r1cs` 250,814,390 bytes, `vk.bin` 528 bytes |
| Prove, with the read of the keys and the native checks | 5.5 s to 5.7 s in three runs, 5.0 GB peak |
| Native verify | 3 ms |
| Proof | 384 bytes; the `verifyProof` calldata has 484 bytes |
| Solidity verifier (solc 0.8.38, via IR, 10,000 optimizer runs, Osaka) | 6,227 bytes of runtime code |
| `verifyProof` | 309,089 gas for the call; 336,729 gas for the whole transaction on anvil (Osaka) |
| `cargo test --features slow-tests`, setup and prove in one process | 145 s and 153 s in two runs, 7.6 GB and 12.4 GB peak |

On chain, `verifyProof` reverts with `ProofInvalid()` for a wrong public input, and it reverts for each of the 384 proofs with one flipped byte.
