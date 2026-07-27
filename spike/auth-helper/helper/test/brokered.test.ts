import { describe, it, expect } from 'vitest';
import { pdsAuthedGet } from '../src/confidential.ts';
import { decodeJwt } from '../src/oauth/jose.ts';
import { generateDpopKey, exportDpopKey } from '../src/oauth/dpop.ts';

// The broker leg: the helper makes a DPoP-authed call to the user's PDS on the
// pad's behalf, using the SERVER-HELD session. The pad never sees the token
// (FLOW-SPEC §6). This pins the request shape: Authorization: DPoP <token>, a
// DPoP proof carrying the access-token hash (ath), and the use_dpop_nonce retry.

async function session() {
  const key = await generateDpopKey();
  return {
    did: 'did:plc:test',
    pds: 'https://pds.example',
    issuer: 'https://auth.example',
    accessToken: 'access-xyz',
    refreshToken: 'r',
    tokenEndpoint: 'https://auth.example/token',
    clientId: 'https://account.croft.ing/client-metadata.json',
    dpopKey: await exportDpopKey(key),
  };
}

function json(body: unknown, init: { status?: number; nonce?: string } = {}): Response {
  const headers: Record<string, string> = { 'content-type': 'application/json' };
  if (init.nonce) headers['DPoP-Nonce'] = init.nonce;
  return new Response(JSON.stringify(body), { status: init.status ?? 200, headers });
}

describe('pdsAuthedGet (broker leg)', () => {
  it('sends Authorization: DPoP + a proof bound to the access token, and returns the PDS JSON', async () => {
    const s = await session();
    let auth: string | undefined;
    let dpopProof: string | undefined;
    const fetchImpl = (async (input: RequestInfo | URL, init?: RequestInit): Promise<Response> => {
      const h = init?.headers as Record<string, string>;
      auth = h.authorization;
      dpopProof = h.dpop;
      return json({ did: s.did, handle: 'test.bsky.social' });
    }) as typeof fetch;

    const { data } = await pdsAuthedGet(s, '/xrpc/com.atproto.server.getSession', fetchImpl);

    expect(auth).toBe('DPoP access-xyz');
    expect(typeof dpopProof).toBe('string');
    const { payload, header } = decodeJwt(dpopProof!);
    expect(header.typ).toBe('dpop+jwt');
    expect(payload.htm).toBe('GET');
    expect(payload.htu).toBe('https://pds.example/xrpc/com.atproto.server.getSession');
    expect(typeof payload.ath).toBe('string'); // access-token hash present
    expect((data as { handle: string }).handle).toBe('test.bsky.social');
  });

  it('honours the use_dpop_nonce retry', async () => {
    const s = await session();
    let calls = 0;
    const fetchImpl = (async (): Promise<Response> => {
      calls += 1;
      if (calls === 1) return json({ error: 'use_dpop_nonce' }, { status: 401, nonce: 'n1' });
      return json({ did: s.did, handle: 'test.bsky.social' });
    }) as typeof fetch;

    const { data } = await pdsAuthedGet(s, '/xrpc/com.atproto.server.getSession', fetchImpl);
    expect(calls).toBe(2);
    expect((data as { did: string }).did).toBe(s.did);
  });
});
