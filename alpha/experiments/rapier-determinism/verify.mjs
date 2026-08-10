// The cross-check: instantiate the wasm32-unknown-unknown build and compare its
// digest against the golden vector recorded from the native run.
//
// Deliberately uses the bare WebAssembly JS API rather than wasm-bindgen. The
// export is a single `() -> i64`, so a binding generator would add a toolchain
// dependency and, worse, a layer of generated glue between the number the
// physics produced and the number we compare.
import { readFile } from "node:fs/promises";

// Must match `GOLDEN` in src/lib.rs.
const GOLDEN = 2135883295078246327n;

const wasmPath = new URL(
  "./target/wasm32-unknown-unknown/release/rapier_determinism.wasm",
  import.meta.url,
);

const bytes = await readFile(wasmPath);
const { instance } = await WebAssembly.instantiate(bytes, {});

const { spike_hash } = instance.exports;
if (typeof spike_hash !== "function") {
  console.error("FAIL: the wasm module exports no spike_hash");
  process.exit(2);
}

// i64 crosses the boundary as BigInt. Signed on the wasm side, so reinterpret
// to unsigned for comparison with the u64 the Rust test asserts.
const raw = BigInt(spike_hash());
const wasmHash = BigInt.asUintN(64, raw);

console.log(`native (golden): ${GOLDEN}`);
console.log(`wasm           : ${wasmHash}`);

if (wasmHash === GOLDEN) {
  console.log("\nMATCH — rapier enhanced-determinism agrees across aarch64 and wasm32.");
  process.exit(0);
}

// Report the magnitude of the disagreement, not just the fact of it: a digest
// mismatch says nothing about whether the sim diverged in the last bit or flew
// apart, and the next question is always "by how much".
console.log("\nMISMATCH — the two targets do not agree.");
console.log("The digest is order-sensitive and avalanche-y, so this says the runs");
console.log("differ, not by how much. Re-run with per-body output to size it.");
process.exit(1);
