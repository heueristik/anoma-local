# Anoma local

The goal of Anoma local is to make general, privacy-preserving, intent-centric applications possible.

Anoma local proves resource machine transactions on the device of each user. Each party proves its own actions with Noir. A relayer, a solver or the party composes the actions into a balanced transaction and wraps its proof into one Groth16 proof. The immutable `AnomaVerifier` contract checks that proof, adds the commitments of the created resources to its commitment tree, records the nullifiers, and makes the forwarder calls that the resources carry.

The terms are in [CONTEXT.md](CONTEXT.md).

## Layout

| Path | Content |
| --- | --- |
| `contracts/` | The `AnomaVerifier` contract, its Poseidon2 commitment tree and the action digest, and the Groth16 verifier of the test keys |
| `circuits/` | The Noir circuits: the libraries `arm`, `compliance` and `step`, the logics `trivial_logic`, `intent_logic` and `token_logic`, `root`, and in `variants/` the compliance and step circuits for each size of action |
| `crates/arm` | The resource machine outside the circuits: resources, kinds, deltas, the commitment tree and the action digest |
| `crates/groth16-wrap` | The Groth16 wrap of a bb proof through a Go library on gnark, and the wrapper of the root proof |
| `crates/prover` | Proves actions and transactions through the `CircuitProver`, `RootProofWrapper` and `ResourceLogic` traits, and the circuit prover on nargo and bb |
| `crates/discovery` | The `Discovery` trait, and its scheme with ECIES on Grumpkin and Poseidon2 and trial decryption |
| `crates/intent` | The intent resource, its logic, and the payload for the solver |
| `crates/token` | The token resource and its logic, whose owner or issuer signs with a P-256 key |
| `crates/intent-pool` | The `IntentPool` trait, and a pool in memory |
| `crates/solver` | A solver of intents, the `Matcher` trait, and a matcher with the CP-SAT solver of OR-Tools |
| `crates/bindings` | The Rust bindings of the `AnomaVerifier` |
| `crates/belt-ffi` | The commitment tree of `crates/arm` as a command, which the differential contract tests run |
| `crates/integration-test` | The fixtures and the anvil chain of the end-to-end tests |
| `indexer/` | An Envio indexer of the events of the `AnomaVerifier`, which Anoma or a user can run |

## Components

The resource machine in `crates/arm`, the provers of actions and transactions, the intent and the solver use the components below only through their traits. An implementation can change without a change to the code that uses it.

| Trait | Module | Implementation | Job |
| --- | --- | --- | --- |
| `CircuitProver` | `anoma_prover::circuit_prover` | `CliProver`, which runs nargo and bb | Gives the verification key of a circuit, proves it, and verifies proofs |
| `RootProofWrapper` | `anoma_prover::root_proof_wrapper` | `Groth16Wrapper` in `crates/groth16-wrap` | Turns the root proof into the proof that the AnomaVerifier checks |
| `ResourceLogic` | `anoma_prover::resource_logic` | `TrivialLogic`, `IntentLogic` in `crates/intent`, `TokenLogic` in `crates/token` | Names the circuit of a logic, and makes its private inputs from the action of its resource |
| `Signer` | `anoma_token` | Each P-256 key that signs messages with ECDSA over SHA-256, such as `p256::ecdsa::SigningKey` | Signs the action tree root for the owner or the issuer of a token |
| `Discovery` | `anoma_discovery` | `GrumpkinDiscovery` | Makes a created resource discoverable for its receiver, and finds the resources of a receiver |
| `IntentPool` | `anoma_intent_pool` | `MemoryIntentPool` | Holds the intents for the solvers until the chain executes their fills |
| `Matcher` | `anoma_solver::matching` | `CpSatMatcher` | Chooses the intents that balance together |

Each implementation must keep these constraints:

- A circuit prover gives bb 6.0.0 UltraHonk proofs and keys: the step circuits verify the leaf proofs, and the hash of the key of a logic is its logic ref.
- A root proof wrapper needs its own verifier contract, which the AnomaVerifier gets at its deployment.
- A resource logic is a Noir circuit whose public inputs are the tag, `is_consumed`, the action tree root and the app data hash. A new logic is a new circuit with its own logic ref.
- A signer signs with ECDSA over SHA-256, as keriox and the secure element of a device do. The token logic accepts only the low s, so `TokenLogic` normalizes the signature.
- A discovery scheme encrypts BN254 field elements, as the plaintext of a resource and the intent payload are field elements. Its address is in the intent payload, so a new scheme also changes the payloads.
- The methods of an intent pool are async, so that a pool on a network can wait for its answers. The pool checks no proof; the solver does.

The chain is the ABI of the AnomaVerifier. `crates/bindings` and the `Chain` of the end-to-end tests are its client.

## How a transaction is proved

1. For each action, its party proves the compliance circuit for the size of the action and the logic of each of its resources. These leaf proofs are ZK UltraHonk proofs.
2. A partial transaction is a list of proved actions and the sum of their rcv values. Partial transactions compose.
3. `first_step_k` verifies the leaf proofs of the first action, of k resources. Each `next_step_k` verifies the previous step proof and the leaf proofs of one more action of k resources. Each step adds the action to the action digest and its delta to the sum of the deltas.
4. `root` verifies the last step proof, checks that the transaction is balanced, and binds the action digest to the chain ID and the address of the AnomaVerifier.
5. The gnark circuit wraps the root proof into a Groth16 proof. The contract checks it against the bound action digest of the transaction.

## Requirements

- Rust (stable) and Go 1.26.8 or later.
- Foundry 1.9 or later, and Bun for solhint.
- nargo 1.0.0 and bb 6.0.0. Set `NARGO` and `BB` if they are not on the `PATH`.
- Node.js 22.15 or later and pnpm 9.15.9 for the indexer. See [indexer/README.md](indexer/README.md).
- OR-Tools 9.15 for the solver. `just crates-tooling` installs it on arm64 macOS into `~/.ortools`. On other systems, install it and set `ORTOOLS_PREFIX`.

## Checks

`just all-check` runs the checks of CI.

`just crates-test-slow` also runs the tests that prove transactions and execute them on anvil. The first run makes the Groth16 setup of the root circuit, which takes minutes and about 8 GB of memory. Later runs read the keys from `target/tmp/wrap-keys`. This setup has one party, so that party can make false proofs; a deployment needs a setup with many parties.

`just crates-prove-times` prints the time of each proof of a wrap, a transfer, an unwrap, a transfer with change and a split into three as a Markdown table. `just crates-execution-gas` executes them on anvil with the Osaka hardfork and prints their gas. Both use the same keys.

`just crates-groth16-verifier` writes the Solidity verifier of these keys to `contracts/src/Groth16Verifier.sol`. Pass another path as an argument to write it elsewhere. The verifier contains the Groth16 verifying key of the root circuit, so each change of the root circuit or of the keys needs a new verifier. A comment below its license identifier names the key hash of the root circuit, and `just groth16-verifier-check` checks that hash against the compiled root circuit. CI runs this check only for `main`, so the root circuit can change on `next` without a new setup. The committed verifier comes from a single-party setup on one machine: only that machine has the proving key, and it can make false proofs. Use it for tests only.

## License

Apache-2.0, see [LICENSE](LICENSE). The Poseidon2 contract in `contracts/vendor/` comes from the Privacy Boost protocol, under its own Apache-2.0 license.
