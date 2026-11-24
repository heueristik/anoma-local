import type { EventLocation } from "./ids.js";

/** The fields that each event entity stores about its event. */
export function eventMetadata(event: EventLocation & { block: { timestamp: number } }) {
  return {
    chainId: event.chainId,
    verifier: event.srcAddress,
    txHash: event.transaction.hash,
    blockNumber: BigInt(event.block.number),
    logIndex: event.logIndex,
    timestamp: BigInt(event.block.timestamp),
  };
}
