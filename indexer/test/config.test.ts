import { readFileSync } from "node:fs";

import { describe, expect, it } from "vitest";
import { parse } from "yaml";

interface Config {
  contracts: { name: string; events: { event: string }[] }[];
}

const config = parse(readFileSync(new URL("../config.yaml", import.meta.url), "utf8")) as Config;
const bindings = readFileSync(
  new URL("../../crates/bindings/src/generated/i_anoma_verifier.rs", import.meta.url),
  "utf8"
);
// The doc comment of the bindings starts with the Solidity source of the whole interface.
const interfaceSource = /^interface IAnomaVerifier \{$(.*?)^\}$/ms.exec(bindings)?.[1] ?? "";

describe("config.yaml", () => {
  it("indexes each event of IAnomaVerifier", () => {
    const interfaceEvents = [...interfaceSource.matchAll(/^\s*event (.+);$/gm)].map(
      (match) => match[1]
    );
    const verifier = config.contracts.find((contract) => contract.name === "AnomaVerifier");
    assert(interfaceEvents.length > 0, "the bindings declare the events of IAnomaVerifier");

    expect(verifier?.events.map(({ event }) => event).sort()).toEqual(interfaceEvents.sort());
  });
});

function assert(condition: boolean, premise: string): asserts condition {
  expect(condition, premise).toBe(true);
}
