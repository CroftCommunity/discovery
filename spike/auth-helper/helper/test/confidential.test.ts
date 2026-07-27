import { describe, it, expect } from 'vitest';
import {
  confidentialBeginAuthorization,
  confidentialCompleteAuthorization,
  confidentialRefresh,
  CLIENT_ASSERTION_TYPE,
  type ConfidentialConfig,
} from '../src/confidential.ts';
import { assertionSigner, type AssertionKey } from '../src/assertion.ts';
import { decodeJwt } from '../src/oauth/jose.ts';
import { jwkThumbprint, type PublicJwk } from '../src/oauth/dpop.ts';

// The confidential deltas over the proven public flow (FLOW-SPEC §5/§6, D4):
// every authenticated POST (PAR, token, refresh) carries client_assertion_type +
// client_assertion IN ADDITION TO the DPoP header. Nothing else in the request
// bodies changes. These tests pin that, hermetically, with a mocked PDS/AS.

const CLIENT_ID = 'https://account.croft.ing/client-metadata.json';
const REDIRECT = 'https://account.croft.ing/callback';
const HANDLE = 'test.example';
const DID = 'did:plc:test1234';
const PDS = 'https://pds.example';
const AUTH = 'https://auth.example';
const ISSUER = 'https://auth.example';

async function genSigner(): Promise<ReturnType<typeof assertionSigner>> {
  const pair = await crypto.subtle.generateKey({ name: 'ECDSA', namedCurve: 'P-256' }, true, ['sign']);
  const pub = (await crypto.subtle.exportKey('jwk', pair.publicKey)) as JsonWebKey;
  const publicJwk: PublicJwk = { kty: 'EC', crv: 'P-256', x: pub.x!, y: pub.y! };
  const key: AssertionKey = { privateKey: pair.privateKey, kid: await jwkThumbprint(publicJwk) };
  return assertionSigner(CLIENT_ID, key);
}

function json(body: unknown, init: { status?: number; nonce?: string } = {}): Response {
  const headers: Record<string, string> = { 'content-type': 'application/json' };
  if (init.nonce) headers['DPoP-Nonce'] = init.nonce;
  return new Response(JSON.stringify(body), { status: init.status ?? 200, headers });
}

/** A mock fetch driving the full resolve chain + PAR, recording request bodies. */
function resolveAndParFetch(record: { parBody?: URLSearchParams }): typeof fetch {
  return (async (input: RequestInfo | URL, init?: RequestInit): Promise<Response> => {
    const url = typeof input === 'string' ? input : input.toString();
    if (url.includes('resolveHandle')) return json({ did: DID });
    if (url.startsWith('https://plc.directory/'))
      return json({ id: DID, service: [{ id: '#atproto_pds', type: 'AtprotoPersonalDataServer', serviceEndpoint: PDS }] });
    if (url.includes('oauth-protected-resource')) return json({ authorization_servers: [AUTH] });
    if (url.includes('oauth-authorization-server'))
      return json({
        issuer: ISSUER,
        authorization_endpoint: `${AUTH}/authorize`,
        token_endpoint: `${AUTH}/token`,
        pushed_authorization_request_endpoint: `${AUTH}/par`,
      });
    if (url === `${AUTH}/par`) {
      record.parBody = new URLSearchParams(String(init?.body));
      return json({ request_uri: 'urn:ietf:params:oauth:request_uri:abc123' });
    }
    throw new Error(`unexpected fetch: ${url}`);
  }) as typeof fetch;
}

describe('confidentialBeginAuthorization', () => {
  it('adds the private_key_jwt client assertion to the PAR request, bound to the AS issuer', async () => {
    const rec: { parBody?: URLSearchParams } = {};
    const sign = await genSigner();
    const cfg: ConfidentialConfig = { clientId: CLIENT_ID, redirectUri: REDIRECT, scope: 'atproto', fetchImpl: resolveAndParFetch(rec) };

    const { authorizeUrl, pending } = await confidentialBeginAuthorization(HANDLE, cfg, sign);

    expect(rec.parBody?.get('client_assertion_type')).toBe(CLIENT_ASSERTION_TYPE);
    const assertion = rec.parBody?.get('client_assertion');
    expect(typeof assertion).toBe('string');
    const { payload } = decodeJwt(assertion!);
    expect(payload.iss).toBe(CLIENT_ID);
    expect(payload.aud).toBe(ISSUER);
    // standard PAR params still present
    expect(rec.parBody?.get('code_challenge_method')).toBe('S256');
    expect(rec.parBody?.get('redirect_uri')).toBe(REDIRECT);
    // authorize URL carries client_id + the PAR request_uri
    const u = new URL(authorizeUrl);
    expect(u.searchParams.get('client_id')).toBe(CLIENT_ID);
    expect(u.searchParams.get('request_uri')).toBe('urn:ietf:params:oauth:request_uri:abc123');
    expect(pending.issuer).toBe(ISSUER);
    expect(pending.did).toBe(DID);
  });
});

describe('confidentialCompleteAuthorization', () => {
  it('exchanges the code with the assertion + DPoP and verifies the token subject', async () => {
    const sign = await genSigner();
    let tokenBody: URLSearchParams | undefined;
    const fetchImpl = (async (input: RequestInfo | URL, init?: RequestInit): Promise<Response> => {
      tokenBody = new URLSearchParams(String(init?.body));
      return json({ access_token: 'at1', refresh_token: 'rt1', expires_in: 3600, sub: DID, token_type: 'DPoP' });
    }) as typeof fetch;

    // Build a pending via begin, then swap in the token fetch.
    const rec: { parBody?: URLSearchParams } = {};
    const beginCfg: ConfidentialConfig = { clientId: CLIENT_ID, redirectUri: REDIRECT, scope: 'atproto', fetchImpl: resolveAndParFetch(rec) };
    const { pending } = await confidentialBeginAuthorization(HANDLE, beginCfg, sign);

    const cfg: ConfidentialConfig = { clientId: CLIENT_ID, redirectUri: REDIRECT, scope: 'atproto', fetchImpl };
    const session = await confidentialCompleteAuthorization(pending, { code: 'authcode', state: pending.state }, cfg, sign);

    expect(tokenBody?.get('grant_type')).toBe('authorization_code');
    expect(tokenBody?.get('code')).toBe('authcode');
    expect(tokenBody?.get('client_assertion_type')).toBe(CLIENT_ASSERTION_TYPE);
    expect(decodeJwt(tokenBody!.get('client_assertion')!).payload.aud).toBe(ISSUER);
    expect(session.accessToken).toBe('at1');
    expect(session.refreshToken).toBe('rt1');
    expect(session.did).toBe(DID);
    expect(session.issuer).toBe(ISSUER);
  });

  it('rejects a state mismatch', async () => {
    const sign = await genSigner();
    const rec: { parBody?: URLSearchParams } = {};
    const beginCfg: ConfidentialConfig = { clientId: CLIENT_ID, redirectUri: REDIRECT, scope: 'atproto', fetchImpl: resolveAndParFetch(rec) };
    const { pending } = await confidentialBeginAuthorization(HANDLE, beginCfg, sign);
    const cfg: ConfidentialConfig = { clientId: CLIENT_ID, redirectUri: REDIRECT, scope: 'atproto', fetchImpl: resolveAndParFetch({}) };
    await expect(confidentialCompleteAuthorization(pending, { code: 'x', state: 'wrong' }, cfg, sign)).rejects.toThrow(/state/i);
  });
});

describe('confidentialRefresh', () => {
  const baseSession = {
    did: DID,
    pds: PDS,
    issuer: ISSUER,
    accessToken: 'old',
    refreshToken: 'rt1',
    tokenEndpoint: `${AUTH}/token`,
    clientId: CLIENT_ID,
  };

  it('refreshes server-side with the assertion and adopts the rotated refresh token', async () => {
    const sign = await genSigner();
    let body: URLSearchParams | undefined;
    // Provide a stored DPoP key by running begin first (it exports one into pending).
    const rec: { parBody?: URLSearchParams } = {};
    const beginCfg: ConfidentialConfig = { clientId: CLIENT_ID, redirectUri: REDIRECT, scope: 'atproto', fetchImpl: resolveAndParFetch(rec) };
    const { pending } = await confidentialBeginAuthorization(HANDLE, beginCfg, sign);

    const fetchImpl = (async (_i: RequestInfo | URL, init?: RequestInit): Promise<Response> => {
      body = new URLSearchParams(String(init?.body));
      return json({ access_token: 'at2', refresh_token: 'rt2', expires_in: 3600 });
    }) as typeof fetch;

    const session = { ...baseSession, dpopKey: pending.dpopKey };
    const next = await confidentialRefresh(session, sign, fetchImpl);

    expect(body?.get('grant_type')).toBe('refresh_token');
    expect(body?.get('refresh_token')).toBe('rt1');
    expect(body?.get('client_assertion_type')).toBe(CLIENT_ASSERTION_TYPE);
    expect(decodeJwt(body!.get('client_assertion')!).payload.aud).toBe(ISSUER);
    expect(next.accessToken).toBe('at2');
    expect(next.refreshToken).toBe('rt2'); // rotation
  });

  it('honours the use_dpop_nonce handshake with a single retry', async () => {
    const sign = await genSigner();
    const rec: { parBody?: URLSearchParams } = {};
    const beginCfg: ConfidentialConfig = { clientId: CLIENT_ID, redirectUri: REDIRECT, scope: 'atproto', fetchImpl: resolveAndParFetch(rec) };
    const { pending } = await confidentialBeginAuthorization(HANDLE, beginCfg, sign);

    let calls = 0;
    const nonces: (string | null)[] = [];
    const fetchImpl = (async (_i: RequestInfo | URL, init?: RequestInit): Promise<Response> => {
      calls += 1;
      const dpop = (init?.headers as Record<string, string>).dpop!;
      nonces.push((decodeJwt(dpop).payload.nonce as string | null) ?? null);
      if (calls === 1) return json({ error: 'use_dpop_nonce' }, { status: 400, nonce: 'server-nonce-1' });
      return json({ access_token: 'at3', refresh_token: 'rt3', expires_in: 3600 });
    }) as typeof fetch;

    const session = { ...baseSession, dpopKey: pending.dpopKey };
    const next = await confidentialRefresh(session, sign, fetchImpl);

    expect(calls).toBe(2);
    expect(nonces[0]).toBeNull();
    expect(nonces[1]).toBe('server-nonce-1'); // retry carried the server nonce in the DPoP proof
    expect(next.accessToken).toBe('at3');
  });
});
