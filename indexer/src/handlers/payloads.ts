import { indexer } from "envio";
import type { Payload } from "envio";

import { eventId } from "./ids.js";
import { eventMetadata } from "./metadata.js";

const PAYLOAD_KINDS = {
  ResourcePayload: "Resource",
  DiscoveryPayload: "Discovery",
  ExternalPayload: "External",
  ApplicationPayload: "Application",
} as const satisfies Record<string, Payload["kind"]>;

for (const event of Object.keys(PAYLOAD_KINDS) as (keyof typeof PAYLOAD_KINDS)[]) {
  indexer.onEvent({ contract: "AnomaVerifier", event }, async ({ event, context }) => {
    if (context.isPreload) {
      return;
    }

    context.Payload.set({
      id: eventId(event),
      ...eventMetadata(event),
      kind: PAYLOAD_KINDS[event.eventName],
      tag: event.params.tag,
      index: event.params.index,
      blob: event.params.blob,
    });
  });
}
