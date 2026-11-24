/** The fields of an event that locate it on its chain. */
export interface EventLocation {
  chainId: number;
  srcAddress: string;
  logIndex: number;
  block: { number: number };
  transaction: { hash: string };
}

/** The id of the entity that one event creates. */
export function eventId(event: EventLocation): string {
  return `${event.chainId}_${event.block.number}_${event.logIndex}`;
}

/** The id of the commitment tree of a verifier. */
export function commitmentTreeId(event: EventLocation): string {
  return `${event.chainId}_${event.srcAddress}`;
}

/** The id of the `execute` count of a verifier in the EVM transaction of an event. */
export function executeCountId(event: EventLocation): string {
  return `${event.chainId}_${event.srcAddress}_${event.transaction.hash}`;
}

/** The id of the `execute` call at a position in its EVM transaction. */
export function transactionId(event: EventLocation, executeIndex: number): string {
  return `${executeCountId(event)}_${executeIndex}`;
}
