import { createServer } from 'node:http';
import { writeFileSync, readFileSync, existsSync, mkdirSync, appendFileSync, unlinkSync } from 'node:fs';
import { join } from 'node:path';
import { randomB64url } from './oauth/jose.ts';
import {
  confidentialBeginAuthorization,
  confidentialCompleteAuthorization,
  confidentialRefresh,
  pdsAuthedGet,
  type ConfidentialConfig,
  type PendingAuth,
  type OAuthSession,
} from './confidential.ts';
import { assertionSigner } from './assertion.ts';
import { loadOrCreateAssertionKey, loadOrCreateStoreKey } from './keystore.ts';
import { encryptJson, decryptJson } from './store.ts';

// The confidential auth-helper spike server (FLOW-SPEC §6). Serves the four
// contract endpoints behind Caddy TLS; /login begins the flow and PRINTS the
// authorize URL (human authorizes in a browser); /callback completes it and
// stores the DPoP-bound session encrypted at rest. Server-side refresh is the
// separate refresh-cli.

const LISTEN = process.env.AUTH_HELPER_LISTEN ?? '127.0.0.1:8001';
const DATA_DIR = process.env.AUTH_HELPER_DATA_DIR ?? join(process.cwd(), 'data');
const ORIGIN = process.env.AUTH_HELPER_ORIGIN ?? 'https://account.croft.ing';
const SCOPE = process.env.AUTH_HELPER_SCOPE ?? 'atproto transition:generic';
const CLIENT_ID = `${ORIGIN}/client-metadata.json`;
const REDIRECT_URI = `${ORIGIN}/callback`;
// Cross-origin pads (non-croft.ing) that may use the helper. Comma-separated origins.
const ALLOWED_ORIGINS = (process.env.AUTH_HELPER_ALLOWED_ORIGINS ?? 'https://stellin.app').split(',').map((s) => s.trim());
// Allowlisted return URLs the ticket handoff may redirect back to (prefix match).
const ALLOWED_RETURNS = (process.env.AUTH_HELPER_ALLOWED_RETURNS ?? 'https://stellin.app/').split(',').map((s) => s.trim());

const [HOST, PORT] = ((): [string, number] => {
  const i = LISTEN.lastIndexOf(':');
  return [LISTEN.slice(0, i), Number(LISTEN.slice(i + 1))];
})();

for (const sub of ['pending', 'sessions', 'tickets']) mkdirSync(join(DATA_DIR, sub), { recursive: true });

const { key: assertionKey, publicJwk } = await loadOrCreateAssertionKey(join(DATA_DIR, 'assertion-key.jwk'));
const storeKey = await loadOrCreateStoreKey(join(DATA_DIR, 'store-key.bin'));
const sign = assertionSigner(CLIENT_ID, assertionKey);
const cfg: ConfidentialConfig = { clientId: CLIENT_ID, redirectUri: REDIRECT_URI, scope: SCOPE };

const CLIENT_METADATA = {
  client_id: CLIENT_ID,
  client_name: 'Croft Auth Helper (spike)',
  client_uri: `${ORIGIN}/`,
  redirect_uris: [REDIRECT_URI],
  scope: SCOPE,
  grant_types: ['authorization_code', 'refresh_token'],
  response_types: ['code'],
  token_endpoint_auth_method: 'private_key_jwt',
  token_endpoint_auth_signing_alg: 'ES256',
  application_type: 'web',
  dpop_bound_access_tokens: true,
  jwks_uri: `${ORIGIN}/jwks.json`,
};

const JWKS = { keys: [{ ...publicJwk, use: 'sig', alg: 'ES256', kid: assertionKey.kid }] };

const safe = (s: string): string => s.replace(/[^A-Za-z0-9._-]/g, '_');
const pendingPath = (state: string): string => join(DATA_DIR, 'pending', `${safe(state)}.enc`);
const sessionPath = (did: string): string => join(DATA_DIR, 'sessions', `${safe(did)}.enc`);
const ticketPath = (ticket: string): string => join(DATA_DIR, 'tickets', `${safe(ticket)}.enc`);
const measure = (line: string): void => appendFileSync(join(DATA_DIR, 'measurements.log'), `${new Date().toISOString()} ${line}\n`);

function corsHeaders(origin: string | undefined): Record<string, string> {
  if (!origin || !ALLOWED_ORIGINS.includes(origin)) return {};
  return {
    'access-control-allow-origin': origin,
    'access-control-allow-headers': 'authorization,content-type',
    'access-control-allow-methods': 'GET,OPTIONS',
    vary: 'Origin',
  };
}

function send(
  res: import('node:http').ServerResponse,
  status: number,
  type: string,
  body: string,
  extra: Record<string, string> = {},
): void {
  res.writeHead(status, { 'content-type': type, ...extra });
  res.end(body);
}

const server = createServer(async (req, res) => {
  try {
    const url = new URL(req.url ?? '/', ORIGIN);
    const origin = req.headers.origin;
    const cors = corsHeaders(origin);

    // CORS preflight for the cross-origin pad API.
    if (req.method === 'OPTIONS') {
      res.writeHead(204, cors);
      return res.end();
    }

    if (url.pathname === '/healthz') return send(res, 200, 'text/plain', 'ok', cors);
    if (url.pathname === '/client-metadata.json') return send(res, 200, 'application/json', JSON.stringify(CLIENT_METADATA));
    if (url.pathname === '/jwks.json') return send(res, 200, 'application/json', JSON.stringify(JWKS));

    if (url.pathname === '/login') {
      const handle = url.searchParams.get('handle');
      if (!handle) return send(res, 400, 'text/plain', 'missing ?handle=');
      const ret = url.searchParams.get('return') ?? undefined;
      if (ret && !ALLOWED_RETURNS.some((p) => ret.startsWith(p))) return send(res, 400, 'text/plain', 'return URL not allowlisted');
      const { authorizeUrl, pending } = await confidentialBeginAuthorization(handle, cfg, sign);
      writeFileSync(pendingPath(pending.state), await encryptJson(storeKey, { pending, returnUrl: ret }), { mode: 0o600 });
      console.log(`[auth-helper] authorize URL for ${handle} (return=${ret ?? 'none'}):\n${authorizeUrl}`);
      // For a cross-origin pad, 302 straight to the PDS consent screen; for a bare curl, print it.
      if (ret) {
        res.writeHead(302, { location: authorizeUrl });
        return res.end();
      }
      return send(res, 200, 'text/plain', `Open this URL in a browser and authorize:\n\n${authorizeUrl}\n`);
    }

    if (url.pathname === '/callback') {
      const code = url.searchParams.get('code');
      const state = url.searchParams.get('state');
      const err = url.searchParams.get('error');
      if (err) return send(res, 400, 'text/plain', `authorization error: ${err} ${url.searchParams.get('error_description') ?? ''}`);
      if (!code || !state) return send(res, 400, 'text/plain', 'missing code/state');
      const pfile = pendingPath(state);
      if (!existsSync(pfile)) return send(res, 400, 'text/plain', 'unknown or expired state');
      const wrapper = (await decryptJson(storeKey, new Uint8Array(readFileSync(pfile)))) as { pending: PendingAuth; returnUrl?: string };
      const session = await confidentialCompleteAuthorization(wrapper.pending, { code, state }, cfg, sign);
      writeFileSync(sessionPath(session.did), await encryptJson(storeKey, session), { mode: 0o600 });
      unlinkSync(pfile);
      const ttl = session.expiresAt ? Math.round((session.expiresAt - Date.now()) / 1000) : undefined;
      measure(`login did=${session.did} access_expires_in=${ttl}s refresh=${session.refreshToken ? 'yes' : 'no'}`);
      console.log(`[auth-helper] session stored for ${session.did} (access TTL ~${ttl}s, refresh ${session.refreshToken ? 'present' : 'ABSENT'})`);
      // Cross-origin pad: mint an opaque ticket (handle to the brokered session) and redirect back
      // to the pad's own origin. The pad stores the ticket first-party (NOT a cross-site cookie).
      if (wrapper.returnUrl) {
        const ticket = randomB64url(24);
        writeFileSync(ticketPath(ticket), await encryptJson(storeKey, { did: session.did }), { mode: 0o600 });
        const back = new URL(wrapper.returnUrl);
        back.searchParams.set('ticket', ticket);
        res.writeHead(302, { location: back.toString() });
        return res.end();
      }
      return send(
        res,
        200,
        'text/plain',
        `Login complete.\n  DID: ${session.did}\n  access token TTL: ~${ttl}s\n  refresh token: ${session.refreshToken ? 'present' : 'ABSENT'}\n\nThe helper now holds a DPoP-bound session and can refresh it server-side.\n`,
      );
    }

    // Brokered API: the pad presents its ticket; the helper acts with the server-held session
    // and returns identity. The token NEVER crosses to the pad.
    if (url.pathname === '/api/whoami') {
      const authz = req.headers.authorization ?? '';
      const ticket = authz.startsWith('Bearer ') ? authz.slice(7) : undefined;
      if (!ticket) return send(res, 401, 'application/json', JSON.stringify({ error: 'missing bearer ticket' }), cors);
      const tfile = ticketPath(ticket);
      if (!existsSync(tfile)) return send(res, 401, 'application/json', JSON.stringify({ error: 'unknown ticket' }), cors);
      const { did } = (await decryptJson(storeKey, new Uint8Array(readFileSync(tfile)))) as { did: string };
      const sfile = sessionPath(did);
      if (!existsSync(sfile)) return send(res, 404, 'application/json', JSON.stringify({ error: 'no session' }), cors);
      let session = (await decryptJson(storeKey, new Uint8Array(readFileSync(sfile)))) as OAuthSession;
      let r = await pdsAuthedGet(session, '/xrpc/com.atproto.server.getSession');
      if (r.status === 401 && session.refreshToken) {
        // Access token likely expired — refresh server-side (no browser) and retry once.
        session = await confidentialRefresh(session, sign);
        writeFileSync(sfile, await encryptJson(storeKey, session), { mode: 0o600 });
        measure(`broker-refresh did=${did}`);
        r = await pdsAuthedGet(session, '/xrpc/com.atproto.server.getSession');
      }
      if (r.status < 200 || r.status >= 300) return send(res, 502, 'application/json', JSON.stringify({ error: `PDS ${r.status}`, detail: r.data }), cors);
      return send(res, 200, 'application/json', JSON.stringify({ did, handle: r.data.handle, via: 'auth-helper (brokered, token server-side)' }), cors);
    }

    return send(res, 404, 'text/plain', 'not found', cors);
  } catch (e) {
    console.error('[auth-helper] error:', (e as Error).message);
    return send(res, 500, 'text/plain', `error: ${(e as Error).message}`);
  }
});

server.listen(PORT, HOST, () => {
  console.log(`[auth-helper] listening on ${HOST}:${PORT}; client_id=${CLIENT_ID}; data=${DATA_DIR}`);
});
