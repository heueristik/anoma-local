import { createTestIndexer } from "envio";
import { describe, expect, it } from "vitest";

import { bytes32, evmTransaction } from "./simulate.js";

const CHAIN_ID = 31337;

describe("the payload events", () => {
  it("store the kind of each payload", async () => {
    const indexer = createTestIndexer();
    const params = (blob: `0x${string}`) => ({ tag: bytes32(0xc0), index: 0n, blob });

    await indexer.process({
      chains: {
        [CHAIN_ID]: {
          simulate: evmTransaction(2, [
            { contract: "AnomaVerifier", event: "ResourcePayload", params: params("0x01") },
            { contract: "AnomaVerifier", event: "DiscoveryPayload", params: params("0x02") },
            { contract: "AnomaVerifier", event: "ExternalPayload", params: params("0x03") },
            { contract: "AnomaVerifier", event: "ApplicationPayload", params: params("0x04") },
          ]),
        },
      },
    });

    const payloads = await indexer.Payload.getAll();
    const kindOf = Object.fromEntries(payloads.map((entity) => [entity.blob, entity.kind]));
    expect(kindOf).toEqual({
      "0x01": "Resource",
      "0x02": "Discovery",
      "0x03": "External",
      "0x04": "Application",
    });
  });
});
