// Canonical serialization: the one byte-string every signature and hash is taken
// over. Determinism is the whole game here, so we sort object keys recursively
// and emit no incidental whitespace. Identical logical values always produce
// identical bytes, which is what makes every ledger run reproducible to the byte.
//
// SEAM: production atproto signs/hashes over DAG-CBOR (RFC 8949 deterministic
// encoding), not sorted-key JSON. Canonical JSON is the one deliberate wire
// simplification; the *property* we rely on (a single canonical byte-string per
// value) is the same one DAG-CBOR provides.

export type Json =
  | null
  | boolean
  | number
  | string
  | Json[]
  | { [key: string]: Json };

/** Deterministic stringify: keys sorted, no whitespace, integers only for money. */
export function canonicalize(value: unknown): string {
  return encode(value);
}

function encode(value: unknown): string {
  if (value === null || value === undefined) return "null";
  if (typeof value === "number") {
    if (!Number.isFinite(value)) {
      throw new Error(`non-finite number cannot be canonicalized: ${value}`);
    }
    // Integers serialize exactly; we keep money in integer cents precisely so we
    // never depend on float formatting for anything that must balance to the cent.
    return JSON.stringify(value);
  }
  if (typeof value === "boolean" || typeof value === "string") {
    return JSON.stringify(value);
  }
  if (Array.isArray(value)) {
    return "[" + value.map(encode).join(",") + "]";
  }
  if (typeof value === "object") {
    const obj = value as Record<string, unknown>;
    const keys = Object.keys(obj).sort();
    const parts = keys.map((k) => JSON.stringify(k) + ":" + encode(obj[k]));
    return "{" + parts.join(",") + "}";
  }
  throw new Error(`cannot canonicalize value of type ${typeof value}`);
}
