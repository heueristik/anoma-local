import { indexer } from "envio";

import { eventId, executeCountId, transactionId } from "./ids.js";
import { eventMetadata } from "./metadata.js";

// `execute` is `nonReentrant` and emits `TransactionExecuted` last, so the events of a verifier
// between two `TransactionExecuted` events in one EVM transaction belong to one `execute` call.
indexer.onEvent(
  { contract: "AnomaVerifier", event: "TransactionExecuted" },
  async ({ event, context }) => {
    const id = executeCountId(event);
    const executeCount = await context.ExecuteCount.get(id);

    if (context.isPreload) {
      return;
    }

    const executeIndex = executeCount?.count ?? 0;
    context.ExecuteCount.set({ id, count: executeIndex + 1 });
    context.Transaction.set({
      id: transactionId(event, executeIndex),
      ...eventMetadata(event),
      actionDigest: event.params.actionDigest,
    });
  }
);

indexer.onEvent(
  { contract: "AnomaVerifier", event: "ForwarderCallExecuted" },
  async ({ event, context }) => {
    const executeCount = await context.ExecuteCount.get(executeCountId(event));

    if (context.isPreload) {
      return;
    }

    context.ForwarderCall.set({
      id: eventId(event),
      ...eventMetadata(event),
      transaction_id: transactionId(event, executeCount?.count ?? 0),
      forwarder: event.params.untrustedForwarder,
      input: event.params.input,
      output: event.params.output,
    });
  }
);
