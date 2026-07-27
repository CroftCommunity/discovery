import { describe, it, expect } from 'vitest';
import { buildClientAssertion, assertionSigner, type AssertionKey } from '../src/assertion.ts';
import { decodeJwt } from '../src/oauth/jose.ts';
import { jwkThumbprint, type PublicJwk } from '../src/oauth/dpop.ts';

// A confidential client authenticates with a signed JWT (RFC 7523 private_key_jwt).
// These are the invariants the atproto OAuth spec + RFC 7523 pin (FLOW-SPEC §5):
//   header: typ=JWT, alg=ES256, kid matches the published jwks key
//   claims: iss = sub = client_id ; aud = authorization server's issuer ;
//           jti unique per assertion ; iat present ; exp short.
// The signature must verify against the PUBLIC half we would publish at jwks.json.

const CLIENT_ID = 'https://account.croft.ing/client-metadata.json';
const ISSUER = 'https://bsky.social';

async function genKey(): Promise<{ key: AssertionKey; publicJwk: PublicJwk }> {
  const pair = await crypto.subtle.generateKey({ name: 'ECDSA', namedCurve: 'P-256' }, true, ['sign', 'verify']);
  const pub = (await crypto.subtle.exportKey('jwk', pair.publicKey)) as JsonWebKey;
  const publicJwk: PublicJwk = { kty: 'EC', crv: 'P-256', x: pub.x!, y: pub.y! };
  const kid = await jwkThumbprint(publicJwk);
  return { key: { privateKey: pair.privateKey, kid }, publicJwk };
}

function b64urlToBytes(s: string): Uint8Array {
  const b64 = s.replace(/-/g, '+').replace(/_/g, '/').padEnd(s.length + ((4 - (s.length % 4)) % 4), '=');
  const bin = atob(b64);
  return Uint8Array.from(bin, (c) => c.charCodeAt(0));
}

async function verifyEs256(jwt: string, publicJwk: PublicJwk): Promise<boolean> {
  const [h, p, sig] = jwt.split('.');
  const pub = await crypto.subtle.importKey('jwk', publicJwk as JsonWebKey, { name: 'ECDSA', namedCurve: 'P-256' }, false, ['verify']);
  return crypto.subtle.verify(
    { name: 'ECDSA', hash: 'SHA-256' },
    pub,
    b64urlToBytes(sig!) as BufferSource,
    new TextEncoder().encode(`${h}.${p}`) as BufferSource,
  );
}

describe('buildClientAssertion', () => {
  it('signs a JWT whose header names ES256 and the jwks kid', async () => {
    const { key } = await genKey();
    const jwt = await buildClientAssertion({ clientId: CLIENT_ID, issuer: ISSUER, key, iat: 1_700_000_000 });
    const { header } = decodeJwt(jwt);
    expect(header.typ).toBe('JWT');
    expect(header.alg).toBe('ES256');
    expect(header.kid).toBe(key.kid);
  });

  it('sets iss=sub=client_id, aud=issuer, iat, and a short exp', async () => {
    const { key } = await genKey();
    const jwt = await buildClientAssertion({ clientId: CLIENT_ID, issuer: ISSUER, key, iat: 1_700_000_000 });
    const { payload } = decodeJwt(jwt);
    expect(payload.iss).toBe(CLIENT_ID);
    expect(payload.sub).toBe(CLIENT_ID);
    expect(payload.aud).toBe(ISSUER);
    expect(payload.iat).toBe(1_700_000_000);
    expect(payload.exp).toBe(1_700_000_060); // default 60s lifetime
    expect(typeof payload.jti).toBe('string');
    expect((payload.jti as string).length).toBeGreaterThan(0);
  });

  it('mints a distinct jti per assertion', async () => {
    const { key } = await genKey();
    const a = decodeJwt(await buildClientAssertion({ clientId: CLIENT_ID, issuer: ISSUER, key }));
    const b = decodeJwt(await buildClientAssertion({ clientId: CLIENT_ID, issuer: ISSUER, key }));
    expect(a.payload.jti).not.toBe(b.payload.jti);
  });

  it('produces a signature that verifies against the published public key', async () => {
    const { key, publicJwk } = await genKey();
    const jwt = await buildClientAssertion({ clientId: CLIENT_ID, issuer: ISSUER, key });
    expect(await verifyEs256(jwt, publicJwk)).toBe(true);
  });

  it('assertionSigner yields a signer bound to the client_id, audience-parameterised', async () => {
    const { key, publicJwk } = await genKey();
    const sign = assertionSigner(CLIENT_ID, key);
    const jwt = await sign(ISSUER);
    const { payload } = decodeJwt(jwt);
    expect(payload.iss).toBe(CLIENT_ID);
    expect(payload.aud).toBe(ISSUER);
    expect(await verifyEs256(jwt, publicJwk)).toBe(true);
  });
});
