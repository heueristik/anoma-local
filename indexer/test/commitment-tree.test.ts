import { createTestIndexer } from "envio";
import { describe, expect, it } from "vitest";

import { action, bytes32, deployment, evmTransaction, executeEnd } from "./simulate.js";

const CHAIN_ID = 31337;
// The indexer stores the roots that the events carry, so the roots are only labels here.
const EMPTY_TREE_ROOT = bytes32(0xe0);
const ROOT_AFTER_TWO = bytes32(0xe2);
const ROOT_AFTER_THREE = bytes32(0xe3);

describe("ActionExecuted", () => {
  it("gives each commitment the leaf count of the tree before it", async () => {
    const indexer = createTestIndexer();

    await indexer.process({
      chains: {
        [CHAIN_ID]: {
          simulate: [
            ...deployment(1, EMPTY_TREE_ROOT),
            ...evmTransaction(2, [
              action(bytes32(0xa0), bytes32(0xc0)),
              action(bytes32(0xa1), bytes32(0xc1)),
              ...executeEnd(ROOT_AFTER_TWO, bytes32(0xd0)),
            ]),
            ...evmTransaction(3, [
              action(bytes32(0xa2), bytes32(0xc2)),
              ...executeEnd(ROOT_AFTER_THREE, bytes32(0xd1)),
            ]),
          ],
        },
      },
    });

    const actions = await indexer.Action.getAll();
    const leafIndices = Object.fromEntries(
      actions.map((entity) => [entity.commitment, entity.commitmentLeafIndex])
    );
    expect(leafIndices).toEqual({
      [bytes32(0xc0)]: 0n,
      [bytes32(0xc1)]: 1n,
      [bytes32(0xc2)]: 2n,
    });

    const roots = await indexer.CommitmentTreeRoot.getAll();
    const leafCounts = Object.fromEntries(roots.map((entity) => [entity.root, entity.leafCount]));
    expect(leafCounts).toEqual({
      [EMPTY_TREE_ROOT]: 0n,
      [ROOT_AFTER_TWO]: 2n,
      [ROOT_AFTER_THREE]: 3n,
    });

    const [tree] = await indexer.CommitmentTree.getAll();
    expect(tree).toMatchObject({ leafCount: 3n, latestRoot: ROOT_AFTER_THREE });
  });

  it("errors on a verifier without the root of its deployment", async () => {
    const indexer = createTestIndexer();

    const processing = indexer.process({
      chains: {
        [CHAIN_ID]: {
          simulate: evmTransaction(2, [
            action(bytes32(0xa0), bytes32(0xc0)),
            ...executeEnd(ROOT_AFTER_TWO, bytes32(0xd0)),
          ]),
        },
      },
    });

    await expect(processing).rejects.toThrow(/has no commitment tree/);
  });
});
