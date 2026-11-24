# Indexer

An [Envio HyperIndex](https://docs.envio.dev/docs/HyperIndex/overview) indexer of the events of the `AnomaVerifier`. It writes the transactions, actions, payloads, forwarder calls and commitment tree roots of a verifier into Postgres and serves them on GraphQL. Anoma or a user can run it.

## What it stores

| Entity | Content |
| --- | --- |
| `Transaction` | One `execute` call and its action digest. One EVM transaction can hold several calls. |
| `Action` | The nullifier, the commitment and the two logic refs of one action, and the position of the commitment in the commitment tree (`commitmentLeafIndex`). |
| `Payload` | One blob of a resource whose deletion criterion is `Never`, with its kind: `Resource`, `Discovery`, `External` or `Application`. |
| `ForwarderCall` | The forwarder, the input and the output of one forwarder call. |
| `CommitmentTreeRoot` | A root of the commitment tree and the number of commitments under it. |
| `CommitmentTree` | The number of commitments and the latest root of each verifier. |

A wallet finds its resources with the `Discovery` payloads, and proves a consumed resource with the commitments in the order of `commitmentLeafIndex` and a root from `CommitmentTreeRoot`.

## Requirements

- Node.js 22.15 or later and pnpm 9.15.9, the version of `packageManager` in `package.json`. Without pnpm on the `PATH`, set `PNPM="npx --yes pnpm@9.15.9"` for the just recipes.
- Docker for `just indexer-dev`, which starts Postgres and Hasura.

## Configuration

`config.yaml` indexes one verifier on one chain. Two environment variables change it:

| Variable | Default |
| --- | --- |
| `ENVIO_RPC_URL` | `http://127.0.0.1:8545`, the RPC of anvil |
| `ENVIO_ANOMA_VERIFIER_ADDRESS` | The address of the verifier that the end-to-end tests deploy on a new anvil chain |

For another chain, change the `id` and the `start_block` of the chain in `config.yaml`. The start block must be at or before the deployment block of the verifier: the indexer counts the commitments from the deployment, and it stops with an error on a verifier without its first root.

## Run

From the repository root:

```bash
just indexer-deps
```

```bash
just indexer-dev
```

`just indexer-dev` indexes the chain and serves GraphQL at `http://localhost:8080`. To use your own Postgres, run `pnpm start` in this folder with the `ENVIO_PG_*` variables of the Envio documentation.

## Privacy

The queries to an indexer show which tags and payloads a wallet reads. A wallet that queries an indexer of another party should read whole tables, for example all `Discovery` payloads after a block, and filter them on the device. Your own indexer has no such leak.

## Checks

`just indexer-check` runs the format check, the type check, the lint and the tests. The tests run the handlers on simulated events, without a chain. `test/config.test.ts` checks that `config.yaml` indexes the same events as `IAnomaVerifier` in `crates/bindings`.
