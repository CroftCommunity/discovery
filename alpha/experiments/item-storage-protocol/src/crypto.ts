// Cryptographic primitives, all from Node's built-in `crypto` (zero dependencies).
//
//   - Fingerprints:  SHA-256, hex-encoded.
//   - Signatures:    Ed25519 (RFC 8032), which is deterministic — the same key
//                    over the same message always yields the same signature, so
//                    signatures are reproducible across runs.
//
// SEAM: production content addressing uses CIDv1 over DAG-CBOR (multihash +
// codec + version), not bare hex SHA-256. Identity-from-content and tamper
// evidence are preserved; only the encoding differs.

import {
  createHash,
  createPrivateKey,
  createPublicKey,
  sign as nodeSign,
  verify as nodeVerify,
  type KeyObject,
} from "node:crypto";
import { canonicalize, type Json } from "./canonical.ts";

/** SHA-256 fingerprint, hex-encoded. This is an item's name. */
export function sha256hex(data: Buffer | string): string {
  return createHash("sha256").update(data).digest("hex");
}

export interface Keypair {
  privateKey: KeyObject;
  publicKey: KeyObject;
  /** Raw 32-byte Ed25519 public key, hex. Stable identifier material. */
  publicRawHex: string;
}

const PKCS8_ED25519_PREFIX = "302e020100300506032b657004220420";

/**
 * Derive a keypair deterministically from a 32-byte seed. Ed25519 private keys
 * ARE 32-byte seeds; we wrap the seed in the fixed PKCS8 DER header so Node will
 * import it. Deterministic keys + deterministic signatures = a fully
 * reproducible run, keys and all.
 */
export function keypairFromSeedHex(seedHex: string): Keypair {
  if (seedHex.length !== 64) {
    throw new Error("ed25519 seed must be 32 bytes (64 hex chars)");
  }
  const der = Buffer.from(PKCS8_ED25519_PREFIX + seedHex, "hex");
  const privateKey = createPrivateKey({ key: der, format: "der", type: "pkcs8" });
  const publicKey = createPublicKey(privateKey);
  const jwk = publicKey.export({ format: "jwk" }) as { x: string };
  const publicRawHex = Buffer.from(jwk.x, "base64url").toString("hex");
  return { privateKey, publicKey, publicRawHex };
}

/**
 * Deterministic per-actor seed: SHA-256 over the run's master seed and a label.
 * Distinct labels (e.g. "customer", "provider", "coop") yield independent keys.
 */
export function deriveSeedHex(masterSeed: number, label: string): string {
  return sha256hex(`${masterSeed}|${label}`);
}

/** Sign the canonical bytes of a JSON value. Returns a hex signature. */
export function signJson(privateKey: KeyObject, value: Json): string {
  const msg = Buffer.from(canonicalize(value), "utf8");
  return nodeSign(null, msg, privateKey).toString("hex");
}

/** Verify a hex signature over a JSON value against a raw public key (hex). */
export function verifyJson(publicRawHex: string, value: Json, sigHex: string): boolean {
  const publicKey = publicKeyFromRawHex(publicRawHex);
  const msg = Buffer.from(canonicalize(value), "utf8");
  try {
    return nodeVerify(null, msg, publicKey, Buffer.from(sigHex, "hex"));
  } catch {
    return false;
  }
}

const SPKI_ED25519_PREFIX = "302a300506032b6570032100";

/** Reconstruct a public KeyObject from a raw 32-byte Ed25519 key (hex). */
export function publicKeyFromRawHex(publicRawHex: string): KeyObject {
  const der = Buffer.from(SPKI_ED25519_PREFIX + publicRawHex, "hex");
  return createPublicKey({ key: der, format: "der", type: "spki" });
}

/**
 * Stable identifier derived from a public key. "We recognize you the same way
 * we count you": identity is a function of the key, computed the same way every
 * time.
 *
 * SEAM: production uses did:plc / did:key. Here it is a truncated hash of the
 * raw public key with a mock method prefix.
 */
export function identifierFromRawHex(publicRawHex: string): string {
  return "did:croft-mock:" + sha256hex(Buffer.from(publicRawHex, "hex")).slice(0, 32);
}
