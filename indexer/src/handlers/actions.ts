import { indexer } from "envio";

import { commitmentTreeId, eventId, executeCountId, transactionId } from "./ids.js";
import { eventMetadata } from "./metadata.js";

// The verifier appends the commitments in the order of the events, so the leaf index of a
// commitment is the leaf count of the tree before it.
indexer.onEvent(
  { contract: "AnomaVerifier", event: "ActionExecuted" },
  async ({ event, context }) => {
    const treeId = commitmentTreeId(event);
    const [tree, executeCount] = await Promise.all([
      context.CommitmentTree.get(treeId),
      context.ExecuteCount.get(executeCountId(event)),
    ]);

    if (context.isPreload) {
      return;
    }
    if (!tree) {
      throw new Error(
        `Chain ${event.chainId} verifier ${event.srcAddress} has no commitment tree: ` +
          `the start block must be at or before the deployment block of the verifier`
      );
    }

    context.CommitmentTree.set({ ...tree, leafCount: tree.leafCount + 1n });
    context.Action.set({
      id: eventId(event),
      ...eventMetadata(event),
      transaction_id: transactionId(event, executeCount?.count ?? 0),
      nullifier: event.params.nullifier,
      commitment: event.params.commitment,
      commitmentLeafIndex: tree.leafCount,
      consumedLogicRef: event.params.consumedLogicRef,
      createdLogicRef: event.params.createdLogicRef,
    });
  }
);
