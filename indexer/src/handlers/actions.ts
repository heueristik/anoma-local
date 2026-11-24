import { indexer } from "envio";

import { commitmentTreeId, eventId, executeCountId, resourceId, transactionId } from "./ids.js";
import { eventMetadata } from "./metadata.js";

// The verifier appends the commitments in the order of the events and of the commitments in each
// event, so the leaf index of a commitment is the leaf count of the tree before it.
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

    const { nullifiers, consumedLogicRefs, commitments, createdLogicRefs } = event.params;
    const metadata = eventMetadata(event);
    const actionId = eventId(event);
    context.CommitmentTree.set({ ...tree, leafCount: tree.leafCount + BigInt(commitments.length) });
    context.Action.set({
      id: actionId,
      ...metadata,
      transaction_id: transactionId(event, executeCount?.count ?? 0),
    });
    nullifiers.forEach((nullifier, index) => {
      context.ConsumedResource.set({
        id: resourceId(event, "consumed", index),
        ...metadata,
        action_id: actionId,
        nullifier,
        logicRef: logicRef(consumedLogicRefs, index),
        index,
      });
    });
    commitments.forEach((commitment, index) => {
      context.CreatedResource.set({
        id: resourceId(event, "created", index),
        ...metadata,
        action_id: actionId,
        commitment,
        commitmentLeafIndex: tree.leafCount + BigInt(index),
        logicRef: logicRef(createdLogicRefs, index),
        index,
      });
    });
  }
);

/** The logic ref at `index`; the verifier emits one logic ref for each tag. */
function logicRef(logicRefs: readonly string[], index: number): string {
  const ref = logicRefs[index];
  if (ref === undefined) {
    throw new Error(`The ActionExecuted event has no logic ref for tag ${index}`);
  }
  return ref;
}
