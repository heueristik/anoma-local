import { indexer } from "envio";

import { commitmentTreeId, eventId } from "./ids.js";
import { eventMetadata } from "./metadata.js";

// The constructor emits the root of the empty tree, so the tree of a verifier exists from its
// deployment block.
indexer.onEvent(
  { contract: "AnomaVerifier", event: "CommitmentTreeRootAdded" },
  async ({ event, context }) => {
    const treeId = commitmentTreeId(event);
    const tree = await context.CommitmentTree.get(treeId);

    if (context.isPreload) {
      return;
    }

    const leafCount = tree?.leafCount ?? 0n;
    context.CommitmentTree.set({
      id: treeId,
      chainId: event.chainId,
      verifier: event.srcAddress,
      leafCount,
      latestRoot: event.params.root,
    });
    context.CommitmentTreeRoot.set({
      id: eventId(event),
      ...eventMetadata(event),
      root: event.params.root,
      leafCount,
    });
  }
);
