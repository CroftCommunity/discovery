// Canonical serialization for signing.
//
// A signature must cover an unambiguous byte string, and both parties must
// derive that same byte string from the same logical object regardless of key
// insertion order. So every object that gets signed is first canonicalized:
// object keys sorted lexicographically, no insignificant whitespace, recursively.
//
// SEAM: production atproto signs canonical DAG-CBOR, not JSON. This is
// deterministic JSON with sorted keys — the one deliberate simplification called
// out in Part 2 (alongside hex hashes standing in for CIDv1). The property we
// need (a stable, reproducible byte string per logical value) holds either way.

export type Json =
  | null
  | boolean
  | number
  | string
  | Json[]
  | { [key: string]: Json };

export function canonicalize(value: Json): string {
  if (value === null) return "null";
  const t = typeof value;
  if (t === "number") {
    if (!Number.isFinite(value as number)) {
      throw new Error("cannot canonicalize non-finite number");
    }
    return JSON.stringify(value);
  }
  if (t === "boolean" || t === "string") return JSON.stringify(value);
  if (Array.isArray(value)) {
    return "[" + value.map(canonicalize).join(",") + "]";
  }
  if (t === "object") {
    const obj = value as { [key: string]: Json };
    const keys = Object.keys(obj).sort();
    const parts = keys.map(
      (k) => JSON.stringify(k) + ":" + canonicalize(obj[k]),
    );
    return "{" + parts.join(",") + "}";
  }
  throw new Error(`cannot canonicalize value of type ${t}`);
}
