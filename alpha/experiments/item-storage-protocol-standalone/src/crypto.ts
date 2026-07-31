// The cryptographic floor: fingerprints (SHA-256) and signatures (Ed25519),
// both from Node's built-in crypto so the whole suite has zero dependencies.
//
// Keys are derived deterministically from a master seed + a label, so every run
// produces the same keypairs, the same signatures (Ed25519 is deterministic per
// RFC 8032), and therefore byte-identical ledgers. That reproducibility is what
// lets every assertion be exact.
//
// SEAM: hex-encoded SHA-256 stands in for a CIDv1 over DAG-CBOR. Real atproto
// content addresses are multihash-tagged CIDs; here a bare hex digest is the
// "fingerprint". The tamper-evidence property is identical; only the encoding
// differs.

import {
  createHash,
  createPrivateKey,
  createPublicKey,
  sign as nodeSign,
  verify as nodeVerify,
  type KeyObject,
} from "node:crypto";
import { canonicalize } from "./canonical.ts";

// PKCS8 DER prefix for an Ed25519 private key, followed by the 32-byte seed.
// This is a fixed ASN.1 structure (SEQUENCE / version / Ed25519 OID / OCTET
// STRING), so prefix ++ seed is a valid PKCS8 key we can import deterministically.
const PKCS8_ED25519_PREFIX = Buffer.from(
  "302e020100300506032b657004220420",
  "hex",
);

/** SHA-256 of a UTF-8 string or buffer, hex-encoded. This is our "fingerprint". */
export function sha256hex(data: string | Buffer): string {
  return createHash("sha256").update(data).digest("hex");
}

/** Fingerprint of raw item bytes. Named to match the Part 1 vocabulary. */
export function fingerprint(bytes: Buffer): string {
  return sha256hex(bytes);
}

/** Hash of a value's canonical form — the thing signatures are actually taken over. */
export function hashCanonical(value: unknown): string {
  return sha256hex(canonicalize(value));
}

export type Keypair = {
  label: string;
  privateKey: KeyObject;
  publicKey: KeyObject;
  /** Raw 32-byte Ed25519 public key, hex-encoded. The pinnable identity. */
  publicKeyHex: string;
};

/** Derive a deterministic Ed25519 keypair from a master seed and a role label. */
export function deriveKeypair(masterSeed: string, label: string): Keypair {
  const seed = createHash("sha256")
    .update(`${masterSeed}::keyseed::${label}`)
    .digest(); // 32 bytes, the Ed25519 secret scalar seed
  const der = Buffer.concat([PKCS8_ED25519_PREFIX, seed]);
  const privateKey = createPrivateKey({ key: der, format: "der", type: "pkcs8" });
  const publicKey = createPublicKey(privateKey);
  const jwk = publicKey.export({ format: "jwk" }) as { x: string };
  const publicKeyHex = Buffer.from(jwk.x, "base64url").toString("hex");
  return { label, privateKey, publicKey, publicKeyHex };
}

/** Reconstruct a verify-only KeyObject from a pinned raw public key hex. */
export function publicKeyFromHex(publicKeyHex: string): KeyObject {
  const jwk = {
    kty: "OKP",
    crv: "Ed25519",
    x: Buffer.from(publicKeyHex, "hex").toString("base64url"),
  };
  return createPublicKey({ key: jwk, format: "jwk" });
}

/** Sign a UTF-8 message, returning a hex signature. */
export function signMessage(kp: Keypair, message: string): string {
  return nodeSign(null, Buffer.from(message, "utf8"), kp.privateKey).toString("hex");
}

/** Verify a hex signature against a pinned public key (hex or KeyObject). */
export function verifyMessage(
  publicKey: string | KeyObject,
  message: string,
  signatureHex: string,
): boolean {
  const key = typeof publicKey === "string" ? publicKeyFromHex(publicKey) : publicKey;
  try {
    return nodeVerify(
      null,
      Buffer.from(message, "utf8"),
      key,
      Buffer.from(signatureHex, "hex"),
    );
  } catch {
    return false;
  }
}
