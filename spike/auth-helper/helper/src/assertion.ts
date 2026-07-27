import { signJwt, randomB64url } from './oauth/jose.ts';

/**
 * The confidential client's `private_key_jwt` authentication (RFC 7523 §2.2/§3),
 * as required by the atproto OAuth spec for confidential clients (FLOW-SPEC §5).
 * The private key is held server-side and NEVER leaves the box; only the public
 * half is published at jwks.json. This module produces the signed assertion that
 * proves *which client* is calling; DPoP (a separate key) proves possession.
 */

/** The server-held ES256 signing key plus the `kid` that matches jwks.json. */
export interface AssertionKey {
  /** ES256 (P-256) private key. Secret material — never logged or serialized in the clear. */
  readonly privateKey: CryptoKey;
  /** Key id, published in jwks.json so the AS selects the right key. */
  readonly kid: string;
}

export interface AssertionInput {
  /** The client_id (hosted client-metadata URL). Becomes both `iss` and `sub`. */
  readonly clientId: string;
  /** The authorization server's `issuer` — the assertion `aud`. */
  readonly issuer: string;
  readonly key: AssertionKey;
  /** Issued-at, unix seconds; injectable for tests. */
  readonly iat?: number;
  /** Assertion lifetime in seconds (default 60). */
  readonly lifetimeSec?: number;
}

/** Build a signed client-assertion JWT for one authenticated request. */
export async function buildClientAssertion(input: AssertionInput): Promise<string> {
  const iat = input.iat ?? Math.floor(Date.now() / 1000);
  const exp = iat + (input.lifetimeSec ?? 60);
  const header = { typ: 'JWT', alg: 'ES256', kid: input.key.kid };
  const payload: Record<string, unknown> = {
    iss: input.clientId,
    sub: input.clientId,
    aud: input.issuer,
    jti: randomB64url(16),
    iat,
    exp,
  };
  return signJwt(header, payload, input.key.privateKey);
}

/** A signer bound to one client_id + key, parameterised by audience (the AS issuer). */
export type ClientAssertionSigner = (audience: string) => Promise<string>;

export function assertionSigner(clientId: string, key: AssertionKey): ClientAssertionSigner {
  return (audience: string) => buildClientAssertion({ clientId, issuer: audience, key });
}
