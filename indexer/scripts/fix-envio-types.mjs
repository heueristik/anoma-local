// The envio 3.15.0 codegen declares a local `EvmBlock` that envio's own `EvmBlock` hides inside
// `declare module "envio"`, so `event.block` types as `any`. The rename keeps the local type.
import { readFileSync, writeFileSync } from "node:fs";

const file = new URL("../.envio/types.d.ts", import.meta.url);
writeFileSync(file, readFileSync(file, "utf8").replace(/\bEvmBlock\b/g, "ConfigEvmBlock"));
