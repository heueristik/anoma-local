/** A 32-byte hex value whose last bytes hold `value`. */
export function bytes32(value: number): `0x${string}` {
  return `0x${value.toString(16).padStart(64, "0")}`;
}

/** The simulated events of one EVM transaction, in the block with the same number as `hash`. */
export function evmTransaction<const T extends object>(block: number, events: T[]) {
  return events.map((event) => ({
    ...event,
    block: { number: block, timestamp: block * 12 },
    transaction: { hash: bytes32(block) },
  }));
}

/** The events that the verifier constructor emits. */
export function deployment(block: number, emptyTreeRoot: `0x${string}`) {
  return evmTransaction(block, [
    {
      contract: "AnomaVerifier",
      event: "CommitmentTreeRootAdded",
      params: { root: emptyTreeRoot },
    },
  ]);
}

/** The `ActionExecuted` event of an action that consumes `nullifiers` and creates `commitments`. */
export function action(nullifiers: `0x${string}`[], commitments: `0x${string}`[]) {
  return {
    contract: "AnomaVerifier",
    event: "ActionExecuted",
    params: {
      nullifiers,
      consumedLogicRefs: nullifiers.map(() => bytes32(0x10)),
      commitments,
      createdLogicRefs: commitments.map(() => bytes32(0x11)),
    },
  } as const;
}

/** The events that end an `execute` call: the new root, then `TransactionExecuted`. */
export function executeEnd(root: `0x${string}`, actionDigest: `0x${string}`) {
  return [
    { contract: "AnomaVerifier", event: "CommitmentTreeRootAdded", params: { root } },
    { contract: "AnomaVerifier", event: "TransactionExecuted", params: { actionDigest } },
  ] as const;
}
