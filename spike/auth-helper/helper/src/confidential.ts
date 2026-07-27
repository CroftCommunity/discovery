import { createPkce } from './oauth/pkce.ts';
import { generateDpopKey, exportDpopKey, importDpopKey, createDpopProof, type DpopKey, type StoredDpopKey } from './oauth/dpop.ts';
import { resolveIdentity, type ResolveDeps } from './oauth/resolve.ts';
import { randomB64url } from './oauth/jose.ts';
import { type ClientAssertionSigner } from './assertion.ts';

/**
 * Confidential atproto OAuth client (FLOW-SPEC §5/§6). Reuses the live-verified
 * public-client primitives (PKCE, PAR, DPoP, resolve) verbatim; the only deltas
 * over the public flow are the two `client_assertion*` form fields added to every
 * authenticated POST (D4), and holding a server-side private key (D5). The DPoP
 * nonce-retry (dpopForm) is copied unchanged from croft-pwa's proven client.ts.
 */

export const CLIENT_ASSERTION_TYPE = 'urn:ietf:params:oauth:client-assertion-type:jwt-bearer';

export interface ConfidentialConfig {
  /** The hosted client-metadata.json URL — the OAuth client_id. */
  readonly clientId: string;
  readonly redirectUri: string;
  readonly scope: string;
  readonly fetchImpl?: typeof fetch;
}

export interface PendingAuth {
  readonly state: string;
  readonly verifier: string;
  readonly dpopKey: StoredDpopKey;
  readonly did: string;
  readonly pds: string;
  readonly issuer: string;
  readonly authorizationEndpoint: string;
  readonly tokenEndpoint: string;
  readonly parEndpoint: string;
}

export interface OAuthSession {
  readonly did: string;
  readonly pds: string;
  readonly issuer: string;
  readonly accessToken: string;
  readonly refreshToken?: string;
  readonly tokenEndpoint: string;
  readonly clientId: string;
  readonly dpopKey: StoredDpopKey;
  readonly dpopNonce?: string;
  readonly expiresAt?: number;
}

function fetchOf(cfg: ConfidentialConfig): typeof fetch {
  return cfg.fetchImpl ?? globalThis.fetch.bind(globalThis);
}

interface XrpcJson {
  error?: string;
  [k: string]: unknown;
}

/** Copied verbatim from croft-pwa client.ts: DPoP form POST with the single use_dpop_nonce retry. */
async function dpopForm(
  endpoint: string,
  params: Record<string, string>,
  key: DpopKey,
  fetchImpl: typeof fetch,
  opts: { nonce?: string; accessToken?: string } = {},
): Promise<{ data: XrpcJson; nonce: string | undefined; status: number }> {
  const attempt = async (nonce: string | undefined): Promise<Response> => {
    const proof = await createDpopProof({
      key,
      htm: 'POST',
      htu: endpoint,
      ...(nonce ? { nonce } : {}),
      ...(opts.accessToken ? { accessToken: opts.accessToken } : {}),
    });
    return fetchImpl(endpoint, {
      method: 'POST',
      headers: { 'content-type': 'application/x-www-form-urlencoded', accept: 'application/json', dpop: proof },
      body: new URLSearchParams(params).toString(),
    });
  };

  let res = await attempt(opts.nonce);
  let serverNonce = res.headers.get('DPoP-Nonce') ?? undefined;
  let data = (await res.json().catch(() => ({}))) as XrpcJson;

  if (!res.ok && data.error === 'use_dpop_nonce' && serverNonce) {
    res = await attempt(serverNonce);
    serverNonce = res.headers.get('DPoP-Nonce') ?? serverNonce;
    data = (await res.json().catch(() => ({}))) as XrpcJson;
  }
  return { data, nonce: serverNonce, status: res.status };
}

/** The two confidential-auth form fields (D4). */
function assertionParams(assertion: string): Record<string, string> {
  return { client_assertion_type: CLIENT_ASSERTION_TYPE, client_assertion: assertion };
}

/** Step 1: resolve, sign a client assertion, push the authorization request, return the URL to visit. */
export async function confidentialBeginAuthorization(
  handleOrDid: string,
  cfg: ConfidentialConfig,
  sign: ClientAssertionSigner,
  deps: ResolveDeps = {},
): Promise<{ authorizeUrl: string; pending: PendingAuth }> {
  const fetchImpl = fetchOf(cfg);
  const id = await resolveIdentity(handleOrDid, { ...deps, fetchImpl });
  const pkce = await createPkce();
  const key = await generateDpopKey();
  const state = randomB64url(16);
  const assertion = await sign(id.meta.issuer);

  const { data, status } = await dpopForm(
    id.meta.pushed_authorization_request_endpoint,
    {
      client_id: cfg.clientId,
      response_type: 'code',
      redirect_uri: cfg.redirectUri,
      scope: cfg.scope,
      state,
      code_challenge: pkce.challenge,
      code_challenge_method: 'S256',
      login_hint: handleOrDid,
      ...assertionParams(assertion),
    },
    key,
    fetchImpl,
  );
  const requestUri = data.request_uri;
  if (typeof requestUri !== 'string') {
    throw new Error(`PAR failed (${status})${data.error ? `: ${data.error}` : ''}${data.error_description ? ` — ${data.error_description}` : ''}`);
  }

  const authorizeUrl = new URL(id.meta.authorization_endpoint);
  authorizeUrl.searchParams.set('client_id', cfg.clientId);
  authorizeUrl.searchParams.set('request_uri', requestUri);

  return {
    authorizeUrl: authorizeUrl.toString(),
    pending: {
      state,
      verifier: pkce.verifier,
      dpopKey: await exportDpopKey(key),
      did: id.did,
      pds: id.pds,
      issuer: id.meta.issuer,
      authorizationEndpoint: id.meta.authorization_endpoint,
      tokenEndpoint: id.meta.token_endpoint,
      parEndpoint: id.meta.pushed_authorization_request_endpoint,
    },
  };
}

/** Step 2: exchange the callback code for DPoP-bound tokens, with the client assertion. */
export async function confidentialCompleteAuthorization(
  pending: PendingAuth,
  callback: { code: string; state: string },
  cfg: ConfidentialConfig,
  sign: ClientAssertionSigner,
): Promise<OAuthSession> {
  if (callback.state !== pending.state) throw new Error('OAuth state mismatch — refusing the callback');
  const fetchImpl = fetchOf(cfg);
  const key = await importDpopKey(pending.dpopKey);
  const assertion = await sign(pending.issuer);

  const { data, nonce, status } = await dpopForm(
    pending.tokenEndpoint,
    {
      grant_type: 'authorization_code',
      code: callback.code,
      redirect_uri: cfg.redirectUri,
      client_id: cfg.clientId,
      code_verifier: pending.verifier,
      ...assertionParams(assertion),
    },
    key,
    fetchImpl,
  );
  const accessToken = data.access_token;
  if (typeof accessToken !== 'string') {
    throw new Error(`Token exchange failed (${status})${data.error ? `: ${data.error}` : ''}${data.error_description ? ` — ${data.error_description}` : ''}`);
  }
  if (typeof data.sub === 'string' && data.sub !== pending.did) {
    throw new Error('Token subject does not match the resolved DID');
  }

  return {
    did: pending.did,
    pds: pending.pds,
    issuer: pending.issuer,
    accessToken,
    ...(typeof data.refresh_token === 'string' ? { refreshToken: data.refresh_token } : {}),
    tokenEndpoint: pending.tokenEndpoint,
    clientId: cfg.clientId,
    dpopKey: pending.dpopKey,
    ...(nonce ? { dpopNonce: nonce } : {}),
    ...(typeof data.expires_in === 'number' ? { expiresAt: Date.now() + data.expires_in * 1000 } : {}),
  };
}

/**
 * A DPoP-authed GET to the session's PDS on the pad's behalf — the broker leg.
 * Uses the server-held access token (never exposed to the pad). Mirrors
 * croft-pwa's pdsJson (POST) for the read side: Authorization: DPoP <token> plus
 * a DPoP proof carrying the access-token hash (ath), with the use_dpop_nonce retry.
 */
export async function pdsAuthedGet(
  session: OAuthSession,
  path: string,
  fetchImpl: typeof fetch = globalThis.fetch.bind(globalThis),
): Promise<{ data: XrpcJson; nonce: string | undefined; status: number }> {
  const key = await importDpopKey(session.dpopKey);
  const url = new URL(path, session.pds).toString();
  const attempt = async (nonce: string | undefined): Promise<Response> => {
    const proof = await createDpopProof({
      key,
      htm: 'GET',
      htu: url,
      accessToken: session.accessToken,
      ...(nonce ? { nonce } : {}),
    });
    return fetchImpl(url, {
      method: 'GET',
      headers: { accept: 'application/json', authorization: `DPoP ${session.accessToken}`, dpop: proof },
    });
  };
  let res = await attempt(session.dpopNonce);
  let serverNonce = res.headers.get('DPoP-Nonce') ?? session.dpopNonce;
  let data = (await res.json().catch(() => ({}))) as XrpcJson;
  if (!res.ok && data.error === 'use_dpop_nonce' && serverNonce) {
    res = await attempt(serverNonce);
    serverNonce = res.headers.get('DPoP-Nonce') ?? serverNonce;
    data = (await res.json().catch(() => ({}))) as XrpcJson;
  }
  return { data, nonce: serverNonce, status: res.status };
}

/**
 * SERVER-SIDE refresh (no browser) with the client assertion. atproto refresh
 * tokens are single-use and rotate, so the returned session replaces the old one.
 */
export async function confidentialRefresh(
  session: OAuthSession,
  sign: ClientAssertionSigner,
  fetchImpl: typeof fetch = globalThis.fetch.bind(globalThis),
): Promise<OAuthSession> {
  if (!session.refreshToken) throw new Error('No refresh token — a new sign-in is needed.');
  const key = await importDpopKey(session.dpopKey);
  const assertion = await sign(session.issuer);
  const { data, nonce, status } = await dpopForm(
    session.tokenEndpoint,
    {
      grant_type: 'refresh_token',
      refresh_token: session.refreshToken,
      client_id: session.clientId,
      ...assertionParams(assertion),
    },
    key,
    fetchImpl,
    session.dpopNonce ? { nonce: session.dpopNonce } : {},
  );
  const accessToken = data.access_token;
  if (typeof accessToken !== 'string') {
    throw new Error(`Refresh failed (${status})${data.error ? `: ${data.error}` : ''}${data.error_description ? ` — ${data.error_description}` : ''}`);
  }
  const nextNonce = nonce ?? session.dpopNonce;
  return {
    ...session,
    accessToken,
    refreshToken: typeof data.refresh_token === 'string' ? data.refresh_token : session.refreshToken,
    ...(nextNonce ? { dpopNonce: nextNonce } : {}),
    ...(typeof data.expires_in === 'number' ? { expiresAt: Date.now() + data.expires_in * 1000 } : {}),
  };
}
