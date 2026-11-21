# Anoma local

Anoma local proves resource machine transactions on the device of each user. Each party proves its own actions with Noir. A relayer, a solver or the party composes the actions into a balanced transaction and wraps its proof into one Groth16 proof. The immutable `AnomaVerifier` contract checks that proof, adds the commitments of the created resources to its commitment tree, records the nullifiers, and makes the forwarder calls that the resources carry.

The terms are in [CONTEXT.md](CONTEXT.md).

## Layout

| Path | Content |
| --- | --- |
| `contracts/` | The `AnomaVerifier` contract, its Poseidon2 commitment tree and the action digest |
| `circuits/` | The Noir circuits: `compliance`, the logics `trivial_logic` and `intent_logic`, `first_step`, `next_step` and `root` |
| `crates/arm` | The resource machine outside the circuits: resources, kinds, deltas, the commitment tree and the action digest |
| `crates/groth16-wrap` | The Groth16 wrap of the root proof, through a Go library on gnark |
| `crates/prover` | Proves actions and transactions with nargo, bb and the Groth16 wrap |
| `crates/discovery` | Resource discovery as in Anoma Galileo: ECIES on secp256k1 and trial decryption |
| `crates/intent` | The intent resource, the inputs of its logic, and the payload for the solver |
| `crates/intent-pool` | A mock intent pool, which solvers search by discovery |
| `crates/solver` | A solver that matches intents with the CP-SAT solver of OR-Tools |
| `crates/bindings` | The Rust bindings of the `AnomaVerifier` |
| `crates/integration-test` | The fixtures and the anvil chain of the end-to-end tests |

## How a transaction is proved

1. For each action, its party proves the compliance circuit and the logics of the consumed and the created resource. These leaf proofs are ZK UltraHonk proofs.
2. A partial transaction is a list of proved actions and the sum of their rcv values. Partial transactions compose.
3. `first_step` verifies the leaf proofs of the first action. Each `next_step` verifies the previous step proof and the leaf proofs of one more action. Each step adds the action to the action digest and its delta to the sum of the deltas.
4. `root` verifies the last step proof, checks that the transaction is balanced, and binds the action digest to the chain ID and the address of the AnomaVerifier.
5. The gnark circuit wraps the root proof into a Groth16 proof. The contract checks it against the bound action digest of the transaction.

## Requirements

- Rust (stable) and Go 1.26.8 or later.
- Foundry 1.9 or later, and Bun for solhint.
- nargo 1.0.0 and bb 6.0.0. Set `NARGO` and `BB` if they are not on the `PATH`.
- OR-Tools 9.15 for the solver. `just crates-tooling` installs it on arm64 macOS into `~/.ortools`. On other systems, install it and set `ORTOOLS_PREFIX`.

## Checks

`just all-check` runs the checks of CI.

`just crates-test-slow` also runs the tests that prove transactions and execute them on anvil. The first run makes the Groth16 setup of the root circuit, which takes minutes and about 8 GB of memory. Later runs read the keys from `target/tmp/wrap-keys`. This setup has one party, so that party can make false proofs; a deployment needs a setup with many parties.

## License

Apache-2.0, see [LICENSE](LICENSE). The Poseidon2 contract in `contracts/vendor/` comes from the Privacy Boost protocol, under its own Apache-2.0 license.
