import { createTestIndexer } from "envio";
import { describe, expect, it } from "vitest";

import { action, bytes32, deployment, evmTransaction, executeEnd } from "./simulate.js";

const CHAIN_ID = 31337;
const FIRST_DIGEST = bytes32(0xd0);
const SECOND_DIGEST = bytes32(0xd1);

describe("TransactionExecuted", () => {
  it("links the actions and forwarder calls of each execute call in one EVM transaction", async () => {
    const indexer = createTestIndexer();

    await indexer.process({
      chains: {
        [CHAIN_ID]: {
          simulate: [
            ...deployment(1, bytes32(0xe0)),
            ...evmTransaction(2, [
              action([bytes32(0xa0)], [bytes32(0xc0)]),
              {
                contract: "AnomaVerifier",
                event: "ForwarderCallExecuted",
                params: {
                  untrustedForwarder: "0x000000000000000000000000000000000000f0f0",
                  input: "0x01",
                  output: "0x02",
                },
              },
              ...executeEnd(bytes32(0xe1), FIRST_DIGEST),
              action([bytes32(0xa1)], [bytes32(0xc1)]),
              ...executeEnd(bytes32(0xe2), SECOND_DIGEST),
            ]),
          ],
        },
      },
    });

    const transactions = await indexer.Transaction.getAll();
    const digestOf = Object.fromEntries(
      transactions.map((entity) => [entity.id, entity.actionDigest])
    );
    expect(Object.values(digestOf).sort()).toEqual([FIRST_DIGEST, SECOND_DIGEST]);

    const actions = await indexer.Action.getAll();
    const transactionOfAction = Object.fromEntries(
      actions.map((entity) => [entity.id, entity.transaction_id])
    );
    const created = await indexer.CreatedResource.getAll();
    const digestOfCommitment = Object.fromEntries(
      created.map((entity) => [entity.commitment, digestOf[transactionOfAction[entity.action_id]]])
    );
    expect(digestOfCommitment).toEqual({
      [bytes32(0xc0)]: FIRST_DIGEST,
      [bytes32(0xc1)]: SECOND_DIGEST,
    });

    const [forwarderCall] = await indexer.ForwarderCall.getAll();
    expect(digestOf[forwarderCall.transaction_id]).toBe(FIRST_DIGEST);
  });
});
