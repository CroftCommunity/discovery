// src/server.ts
import { createServer } from "node:http";
import { writeFileSync as writeFileSync2, readFileSync as readFileSync2, existsSync as existsSync2, mkdirSync as mkdirSync2, appendFileSync, unlinkSync } from "node:fs";
import { join } from "node:path";

// src/oauth/jose.ts
function subtle() {
  const c = globalThis.crypto;
  if (!c?.subtle) throw new Error("WebCrypto is unavailable");
  return c.subtle;
}
function b64urlFromBytes(bytes) {
  let s = "";
  for (const b of bytes) s += String.fromCharCode(b);
  return btoa(s).replace(/\+/g, "-").replace(/\//g, "_").replace(/=+$/, "");
}
function b64urlFromString(str) {
  return b64urlFromBytes(new TextEncoder().encode(str));
}
function randomB64url(bytes = 32) {
  const buf = new Uint8Array(bytes);
  globalThis.crypto.getRandomValues(buf);
  return b64urlFromBytes(buf);
}
async function sha256(input) {
  const data = typeof input === "string" ? new TextEncoder().encode(input) : input;
  return new Uint8Array(await subtle().digest("SHA-256", data));
}
async function sha256B64url(input) {
  return b64urlFromBytes(await sha256(input));
}
async function signJwt(header, payload, key) {
  const signingInput = `${b64urlFromString(JSON.stringify(header))}.${b64urlFromString(JSON.stringify(payload))}`;
  const sig = await subtle().sign(
    { name: "ECDSA", hash: "SHA-256" },
    key,
    new TextEncoder().encode(signingInput)
  );
  return `${signingInput}.${b64urlFromBytes(new Uint8Array(sig))}`;
}

// src/oauth/pkce.ts
function generateVerifier() {
  return randomB64url(32);
}
async function challengeS256(verifier) {
  return sha256B64url(verifier);
}
async function createPkce() {
  const verifier = generateVerifier();
  return { verifier, challenge: await challengeS256(verifier), method: "S256" };
}

// src/oauth/dpop.ts
function subtle2() {
  const c = globalThis.crypto;
  if (!c?.subtle) throw new Error("WebCrypto is unavailable");
  return c.subtle;
}
function toPublicJwk(jwk) {
  if (jwk.kty !== "EC" || jwk.crv !== "P-256" || !jwk.x || !jwk.y) throw new Error("bad EC key");
  return { kty: "EC", crv: "P-256", x: jwk.x, y: jwk.y };
}
async function generateDpopKey() {
  const pair = await subtle2().generateKey({ name: "ECDSA", namedCurve: "P-256" }, true, ["sign"]);
  const pub = await subtle2().exportKey("jwk", pair.publicKey);
  return { privateKey: pair.privateKey, publicJwk: toPublicJwk(pub) };
}
async function exportDpopKey(key) {
  return { privateJwk: await subtle2().exportKey("jwk", key.privateKey), publicJwk: key.publicJwk };
}
async function importDpopKey(stored) {
  const privateKey = await subtle2().importKey(
    "jwk",
    stored.privateJwk,
    { name: "ECDSA", namedCurve: "P-256" },
    true,
    ["sign"]
  );
  return { privateKey, publicJwk: stored.publicJwk };
}
async function jwkThumbprint(jwk) {
  const canonical = `{"crv":"${jwk.crv}","kty":"${jwk.kty}","x":"${jwk.x}","y":"${jwk.y}"}`;
  return b64urlFromBytes(await sha256(canonical));
}
async function createDpopProof(input) {
  const header = { typ: "dpop+jwt", alg: "ES256", jwk: input.key.publicJwk };
  const iat = input.iat ?? Math.floor(Date.now() / 1e3);
  const payload = { jti: randomB64url(16), htm: input.htm, htu: input.htu, iat };
  if (input.nonce) payload.nonce = input.nonce;
  if (input.accessToken) payload.ath = b64urlFromBytes(await sha256(input.accessToken));
  return signJwt(header, payload, input.key.privateKey);
}

// src/oauth/read.ts
var PUBLIC_APPVIEW = "https://public.api.bsky.app";
var PLC_DIRECTORY = "https://plc.directory";
var AtprotoReadError = class extends Error {
  status;
  constructor(message, status) {
    super(message);
    this.name = "AtprotoReadError";
    this.status = status;
  }
};
var fetchOf = (deps) => deps.fetchImpl ?? globalThis.fetch.bind(globalThis);
async function getJson(res, what) {
  if (!res.ok) throw new AtprotoReadError(`${what} failed: ${res.status}`, res.status);
  return await res.json();
}
async function resolveHandle(handle, deps = {}) {
  const url = new URL("/xrpc/com.atproto.identity.resolveHandle", deps.appView ?? PUBLIC_APPVIEW);
  url.searchParams.set("handle", handle.replace(/^@/, "").trim());
  const data = await getJson(await fetchOf(deps)(url, { headers: { accept: "application/json" } }), "resolveHandle");
  if (typeof data.did !== "string") throw new AtprotoReadError("resolveHandle returned no DID");
  return data.did;
}
function pdsEndpointFromDoc(doc) {
  const svc = (doc.service ?? []).find(
    (s) => s.type === "AtprotoPersonalDataServer" || s.id === "#atproto_pds" || s.id.endsWith("#atproto_pds")
  );
  return svc?.serviceEndpoint ?? null;
}
async function resolvePds(did, deps = {}) {
  let docUrl;
  if (did.startsWith("did:plc:")) {
    docUrl = `${deps.plcDirectory ?? PLC_DIRECTORY}/${did}`;
  } else if (did.startsWith("did:web:")) {
    const rest = did.slice("did:web:".length);
    const parts = rest.split(":").map(decodeURIComponent);
    const host = parts[0];
    const path = parts.length > 1 ? parts.slice(1).join("/") + "/did.json" : ".well-known/did.json";
    docUrl = `https://${host}/${path}`;
  } else {
    throw new AtprotoReadError(`unsupported DID method: ${did}`);
  }
  const res = await fetchOf(deps)(docUrl, { headers: { accept: "application/json" } });
  if (!res.ok) throw new AtprotoReadError(`DID resolution failed: ${res.status}`, res.status);
  const doc = await res.json();
  const endpoint = pdsEndpointFromDoc(doc);
  if (!endpoint) throw new AtprotoReadError(`no PDS endpoint in DID document for ${did}`);
  return endpoint.replace(/\/+$/, "");
}

// src/oauth/resolve.ts
function fetchOf2(deps) {
  return deps.fetchImpl ?? globalThis.fetch.bind(globalThis);
}
async function json(res, what) {
  if (!res.ok) throw new Error(`${what} failed: ${res.status}`);
  return await res.json();
}
async function authServerFromPds(pds, fetchImpl) {
  const url = new URL("/.well-known/oauth-protected-resource", pds);
  const data = await json(await fetchImpl(url, { headers: { accept: "application/json" } }), "protected-resource");
  const servers = data.authorization_servers;
  const authServer = Array.isArray(servers) ? servers[0] : void 0;
  if (typeof authServer !== "string") throw new Error("no authorization server for PDS");
  return authServer.replace(/\/+$/, "");
}
async function fetchAuthServerMeta(authServer, fetchImpl) {
  const url = new URL("/.well-known/oauth-authorization-server", authServer);
  const m = await json(await fetchImpl(url, { headers: { accept: "application/json" } }), "authorization-server");
  const { issuer, authorization_endpoint, token_endpoint, pushed_authorization_request_endpoint } = m;
  if (typeof authorization_endpoint !== "string" || typeof token_endpoint !== "string" || typeof pushed_authorization_request_endpoint !== "string") {
    throw new Error("incomplete authorization-server metadata");
  }
  return {
    issuer: typeof issuer === "string" ? issuer : authServer,
    authorization_endpoint,
    token_endpoint,
    pushed_authorization_request_endpoint
  };
}
async function resolveIdentity(handleOrDid, deps = {}) {
  const fetchImpl = fetchOf2(deps);
  const did = handleOrDid.startsWith("did:") ? handleOrDid : await resolveHandle(handleOrDid, deps);
  const pds = await resolvePds(did, deps);
  const authServer = await authServerFromPds(pds, fetchImpl);
  const meta = await fetchAuthServerMeta(authServer, fetchImpl);
  return { did, pds, authServer, meta };
}

// src/assertion.ts
async function buildClientAssertion(input) {
  const iat = input.iat ?? Math.floor(Date.now() / 1e3);
  const exp = iat + (input.lifetimeSec ?? 60);
  const header = { typ: "JWT", alg: "ES256", kid: input.key.kid };
  const payload = {
    iss: input.clientId,
    sub: input.clientId,
    aud: input.issuer,
    jti: randomB64url(16),
    iat,
    exp
  };
  return signJwt(header, payload, input.key.privateKey);
}
function assertionSigner(clientId, key) {
  return (audience) => buildClientAssertion({ clientId, issuer: audience, key });
}

// src/confidential.ts
var CLIENT_ASSERTION_TYPE = "urn:ietf:params:oauth:client-assertion-type:jwt-bearer";
function fetchOf3(cfg2) {
  return cfg2.fetchImpl ?? globalThis.fetch.bind(globalThis);
}
async function dpopForm(endpoint, params, key, fetchImpl, opts = {}) {
  const attempt = async (nonce) => {
    const proof = await createDpopProof({
      key,
      htm: "POST",
      htu: endpoint,
      ...nonce ? { nonce } : {},
      ...opts.accessToken ? { accessToken: opts.accessToken } : {}
    });
    return fetchImpl(endpoint, {
      method: "POST",
      headers: { "content-type": "application/x-www-form-urlencoded", accept: "application/json", dpop: proof },
      body: new URLSearchParams(params).toString()
    });
  };
  let res = await attempt(opts.nonce);
  let serverNonce = res.headers.get("DPoP-Nonce") ?? void 0;
  let data = await res.json().catch(() => ({}));
  if (!res.ok && data.error === "use_dpop_nonce" && serverNonce) {
    res = await attempt(serverNonce);
    serverNonce = res.headers.get("DPoP-Nonce") ?? serverNonce;
    data = await res.json().catch(() => ({}));
  }
  return { data, nonce: serverNonce, status: res.status };
}
function assertionParams(assertion) {
  return { client_assertion_type: CLIENT_ASSERTION_TYPE, client_assertion: assertion };
}
async function confidentialBeginAuthorization(handleOrDid, cfg2, sign2, deps = {}) {
  const fetchImpl = fetchOf3(cfg2);
  const id = await resolveIdentity(handleOrDid, { ...deps, fetchImpl });
  const pkce = await createPkce();
  const key = await generateDpopKey();
  const state = randomB64url(16);
  const assertion = await sign2(id.meta.issuer);
  const { data, status } = await dpopForm(
    id.meta.pushed_authorization_request_endpoint,
    {
      client_id: cfg2.clientId,
      response_type: "code",
      redirect_uri: cfg2.redirectUri,
      scope: cfg2.scope,
      state,
      code_challenge: pkce.challenge,
      code_challenge_method: "S256",
      login_hint: handleOrDid,
      ...assertionParams(assertion)
    },
    key,
    fetchImpl
  );
  const requestUri = data.request_uri;
  if (typeof requestUri !== "string") {
    throw new Error(`PAR failed (${status})${data.error ? `: ${data.error}` : ""}${data.error_description ? ` \u2014 ${data.error_description}` : ""}`);
  }
  const authorizeUrl = new URL(id.meta.authorization_endpoint);
  authorizeUrl.searchParams.set("client_id", cfg2.clientId);
  authorizeUrl.searchParams.set("request_uri", requestUri);
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
      parEndpoint: id.meta.pushed_authorization_request_endpoint
    }
  };
}
async function confidentialCompleteAuthorization(pending, callback, cfg2, sign2) {
  if (callback.state !== pending.state) throw new Error("OAuth state mismatch \u2014 refusing the callback");
  const fetchImpl = fetchOf3(cfg2);
  const key = await importDpopKey(pending.dpopKey);
  const assertion = await sign2(pending.issuer);
  const { data, nonce, status } = await dpopForm(
    pending.tokenEndpoint,
    {
      grant_type: "authorization_code",
      code: callback.code,
      redirect_uri: cfg2.redirectUri,
      client_id: cfg2.clientId,
      code_verifier: pending.verifier,
      ...assertionParams(assertion)
    },
    key,
    fetchImpl
  );
  const accessToken = data.access_token;
  if (typeof accessToken !== "string") {
    throw new Error(`Token exchange failed (${status})${data.error ? `: ${data.error}` : ""}${data.error_description ? ` \u2014 ${data.error_description}` : ""}`);
  }
  if (typeof data.sub === "string" && data.sub !== pending.did) {
    throw new Error("Token subject does not match the resolved DID");
  }
  return {
    did: pending.did,
    pds: pending.pds,
    issuer: pending.issuer,
    accessToken,
    ...typeof data.refresh_token === "string" ? { refreshToken: data.refresh_token } : {},
    tokenEndpoint: pending.tokenEndpoint,
    clientId: cfg2.clientId,
    dpopKey: pending.dpopKey,
    ...nonce ? { dpopNonce: nonce } : {},
    ...typeof data.expires_in === "number" ? { expiresAt: Date.now() + data.expires_in * 1e3 } : {}
  };
}
async function pdsAuthedGet(session, path, fetchImpl = globalThis.fetch.bind(globalThis)) {
  const key = await importDpopKey(session.dpopKey);
  const url = new URL(path, session.pds).toString();
  const attempt = async (nonce) => {
    const proof = await createDpopProof({
      key,
      htm: "GET",
      htu: url,
      accessToken: session.accessToken,
      ...nonce ? { nonce } : {}
    });
    return fetchImpl(url, {
      method: "GET",
      headers: { accept: "application/json", authorization: `DPoP ${session.accessToken}`, dpop: proof }
    });
  };
  let res = await attempt(session.dpopNonce);
  let serverNonce = res.headers.get("DPoP-Nonce") ?? session.dpopNonce;
  let data = await res.json().catch(() => ({}));
  if (!res.ok && data.error === "use_dpop_nonce" && serverNonce) {
    res = await attempt(serverNonce);
    serverNonce = res.headers.get("DPoP-Nonce") ?? serverNonce;
    data = await res.json().catch(() => ({}));
  }
  return { data, nonce: serverNonce, status: res.status };
}
async function confidentialRefresh(session, sign2, fetchImpl = globalThis.fetch.bind(globalThis)) {
  if (!session.refreshToken) throw new Error("No refresh token \u2014 a new sign-in is needed.");
  const key = await importDpopKey(session.dpopKey);
  const assertion = await sign2(session.issuer);
  const { data, nonce, status } = await dpopForm(
    session.tokenEndpoint,
    {
      grant_type: "refresh_token",
      refresh_token: session.refreshToken,
      client_id: session.clientId,
      ...assertionParams(assertion)
    },
    key,
    fetchImpl,
    session.dpopNonce ? { nonce: session.dpopNonce } : {}
  );
  const accessToken = data.access_token;
  if (typeof accessToken !== "string") {
    throw new Error(`Refresh failed (${status})${data.error ? `: ${data.error}` : ""}${data.error_description ? ` \u2014 ${data.error_description}` : ""}`);
  }
  const nextNonce = nonce ?? session.dpopNonce;
  return {
    ...session,
    accessToken,
    refreshToken: typeof data.refresh_token === "string" ? data.refresh_token : session.refreshToken,
    ...nextNonce ? { dpopNonce: nextNonce } : {},
    ...typeof data.expires_in === "number" ? { expiresAt: Date.now() + data.expires_in * 1e3 } : {}
  };
}

// src/keystore.ts
import { readFileSync, writeFileSync, existsSync, mkdirSync } from "node:fs";
import { dirname } from "node:path";

// src/store.ts
var IV_BYTES = 12;
async function newStoreKey() {
  return crypto.subtle.generateKey({ name: "AES-GCM", length: 256 }, true, ["encrypt", "decrypt"]);
}
async function importStoreKey(raw) {
  return crypto.subtle.importKey("raw", raw, { name: "AES-GCM" }, false, ["encrypt", "decrypt"]);
}
async function exportStoreKey(key) {
  return new Uint8Array(await crypto.subtle.exportKey("raw", key));
}
async function encryptJson(key, obj) {
  const iv = crypto.getRandomValues(new Uint8Array(IV_BYTES));
  const plaintext = new TextEncoder().encode(JSON.stringify(obj));
  const ct = new Uint8Array(
    await crypto.subtle.encrypt({ name: "AES-GCM", iv }, key, plaintext)
  );
  const out = new Uint8Array(iv.length + ct.length);
  out.set(iv, 0);
  out.set(ct, iv.length);
  return out;
}
async function decryptJson(key, blob) {
  const iv = blob.subarray(0, IV_BYTES);
  const ct = blob.subarray(IV_BYTES);
  const plaintext = await crypto.subtle.decrypt({ name: "AES-GCM", iv }, key, ct);
  return JSON.parse(new TextDecoder().decode(plaintext));
}

// src/keystore.ts
function ensureDir(file) {
  mkdirSync(dirname(file), { recursive: true });
}
async function loadOrCreateAssertionKey(keyFile) {
  let privJwk;
  if (existsSync(keyFile)) {
    privJwk = JSON.parse(readFileSync(keyFile, "utf8"));
  } else {
    const pair = await crypto.subtle.generateKey({ name: "ECDSA", namedCurve: "P-256" }, true, ["sign", "verify"]);
    privJwk = await crypto.subtle.exportKey("jwk", pair.privateKey);
    ensureDir(keyFile);
    writeFileSync(keyFile, JSON.stringify(privJwk), { mode: 384 });
  }
  const privateKey = await crypto.subtle.importKey(
    "jwk",
    privJwk,
    { name: "ECDSA", namedCurve: "P-256" },
    false,
    ["sign"]
  );
  if (!privJwk.x || !privJwk.y) throw new Error("assertion key JWK missing EC coordinates");
  const publicJwk2 = { kty: "EC", crv: "P-256", x: privJwk.x, y: privJwk.y };
  const kid = await jwkThumbprint(publicJwk2);
  return { key: { privateKey, kid }, publicJwk: publicJwk2 };
}
async function loadOrCreateStoreKey(keyFile) {
  if (existsSync(keyFile)) {
    return importStoreKey(new Uint8Array(readFileSync(keyFile)));
  }
  const key = await newStoreKey();
  const raw = await exportStoreKey(key);
  ensureDir(keyFile);
  writeFileSync(keyFile, raw, { mode: 384 });
  return importStoreKey(raw);
}

// src/server.ts
var LISTEN = process.env.AUTH_HELPER_LISTEN ?? "127.0.0.1:8001";
var DATA_DIR = process.env.AUTH_HELPER_DATA_DIR ?? join(process.cwd(), "data");
var ORIGIN = process.env.AUTH_HELPER_ORIGIN ?? "https://account.croft.ing";
var SCOPE = process.env.AUTH_HELPER_SCOPE ?? "atproto transition:generic";
var CLIENT_ID = `${ORIGIN}/client-metadata.json`;
var REDIRECT_URI = `${ORIGIN}/callback`;
var ALLOWED_ORIGINS = (process.env.AUTH_HELPER_ALLOWED_ORIGINS ?? "https://stellin.app").split(",").map((s) => s.trim());
var ALLOWED_RETURNS = (process.env.AUTH_HELPER_ALLOWED_RETURNS ?? "https://stellin.app/").split(",").map((s) => s.trim());
var [HOST, PORT] = (() => {
  const i = LISTEN.lastIndexOf(":");
  return [LISTEN.slice(0, i), Number(LISTEN.slice(i + 1))];
})();
for (const sub of ["pending", "sessions", "tickets"]) mkdirSync2(join(DATA_DIR, sub), { recursive: true });
var { key: assertionKey, publicJwk } = await loadOrCreateAssertionKey(join(DATA_DIR, "assertion-key.jwk"));
var storeKey = await loadOrCreateStoreKey(join(DATA_DIR, "store-key.bin"));
var sign = assertionSigner(CLIENT_ID, assertionKey);
var cfg = { clientId: CLIENT_ID, redirectUri: REDIRECT_URI, scope: SCOPE };
var CLIENT_METADATA = {
  client_id: CLIENT_ID,
  client_name: "Croft Auth Helper (spike)",
  client_uri: `${ORIGIN}/`,
  redirect_uris: [REDIRECT_URI],
  scope: SCOPE,
  grant_types: ["authorization_code", "refresh_token"],
  response_types: ["code"],
  token_endpoint_auth_method: "private_key_jwt",
  token_endpoint_auth_signing_alg: "ES256",
  application_type: "web",
  dpop_bound_access_tokens: true,
  jwks_uri: `${ORIGIN}/jwks.json`
};
var JWKS = { keys: [{ ...publicJwk, use: "sig", alg: "ES256", kid: assertionKey.kid }] };
var safe = (s) => s.replace(/[^A-Za-z0-9._-]/g, "_");
var pendingPath = (state) => join(DATA_DIR, "pending", `${safe(state)}.enc`);
var sessionPath = (did) => join(DATA_DIR, "sessions", `${safe(did)}.enc`);
var ticketPath = (ticket) => join(DATA_DIR, "tickets", `${safe(ticket)}.enc`);
var measure = (line) => appendFileSync(join(DATA_DIR, "measurements.log"), `${(/* @__PURE__ */ new Date()).toISOString()} ${line}
`);
function corsHeaders(origin) {
  if (!origin || !ALLOWED_ORIGINS.includes(origin)) return {};
  return {
    "access-control-allow-origin": origin,
    "access-control-allow-headers": "authorization,content-type",
    "access-control-allow-methods": "GET,OPTIONS",
    vary: "Origin"
  };
}
function send(res, status, type, body, extra = {}) {
  res.writeHead(status, { "content-type": type, ...extra });
  res.end(body);
}
var server = createServer(async (req, res) => {
  try {
    const url = new URL(req.url ?? "/", ORIGIN);
    const origin = req.headers.origin;
    const cors = corsHeaders(origin);
    if (req.method === "OPTIONS") {
      res.writeHead(204, cors);
      return res.end();
    }
    if (url.pathname === "/healthz") return send(res, 200, "text/plain", "ok", cors);
    if (url.pathname === "/client-metadata.json") return send(res, 200, "application/json", JSON.stringify(CLIENT_METADATA));
    if (url.pathname === "/jwks.json") return send(res, 200, "application/json", JSON.stringify(JWKS));
    if (url.pathname === "/login") {
      const handle = url.searchParams.get("handle");
      if (!handle) return send(res, 400, "text/plain", "missing ?handle=");
      const ret = url.searchParams.get("return") ?? void 0;
      if (ret && !ALLOWED_RETURNS.some((p) => ret.startsWith(p))) return send(res, 400, "text/plain", "return URL not allowlisted");
      const { authorizeUrl, pending } = await confidentialBeginAuthorization(handle, cfg, sign);
      writeFileSync2(pendingPath(pending.state), await encryptJson(storeKey, { pending, returnUrl: ret }), { mode: 384 });
      console.log(`[auth-helper] authorize URL for ${handle} (return=${ret ?? "none"}):
${authorizeUrl}`);
      if (ret) {
        res.writeHead(302, { location: authorizeUrl });
        return res.end();
      }
      return send(res, 200, "text/plain", `Open this URL in a browser and authorize:

${authorizeUrl}
`);
    }
    if (url.pathname === "/callback") {
      const code = url.searchParams.get("code");
      const state = url.searchParams.get("state");
      const err = url.searchParams.get("error");
      if (err) return send(res, 400, "text/plain", `authorization error: ${err} ${url.searchParams.get("error_description") ?? ""}`);
      if (!code || !state) return send(res, 400, "text/plain", "missing code/state");
      const pfile = pendingPath(state);
      if (!existsSync2(pfile)) return send(res, 400, "text/plain", "unknown or expired state");
      const wrapper = await decryptJson(storeKey, new Uint8Array(readFileSync2(pfile)));
      const session = await confidentialCompleteAuthorization(wrapper.pending, { code, state }, cfg, sign);
      writeFileSync2(sessionPath(session.did), await encryptJson(storeKey, session), { mode: 384 });
      unlinkSync(pfile);
      const ttl = session.expiresAt ? Math.round((session.expiresAt - Date.now()) / 1e3) : void 0;
      measure(`login did=${session.did} access_expires_in=${ttl}s refresh=${session.refreshToken ? "yes" : "no"}`);
      console.log(`[auth-helper] session stored for ${session.did} (access TTL ~${ttl}s, refresh ${session.refreshToken ? "present" : "ABSENT"})`);
      if (wrapper.returnUrl) {
        const ticket = randomB64url(24);
        writeFileSync2(ticketPath(ticket), await encryptJson(storeKey, { did: session.did }), { mode: 384 });
        const back = new URL(wrapper.returnUrl);
        back.searchParams.set("ticket", ticket);
        res.writeHead(302, { location: back.toString() });
        return res.end();
      }
      return send(
        res,
        200,
        "text/plain",
        `Login complete.
  DID: ${session.did}
  access token TTL: ~${ttl}s
  refresh token: ${session.refreshToken ? "present" : "ABSENT"}

The helper now holds a DPoP-bound session and can refresh it server-side.
`
      );
    }
    if (url.pathname === "/api/whoami") {
      const authz = req.headers.authorization ?? "";
      const ticket = authz.startsWith("Bearer ") ? authz.slice(7) : void 0;
      if (!ticket) return send(res, 401, "application/json", JSON.stringify({ error: "missing bearer ticket" }), cors);
      const tfile = ticketPath(ticket);
      if (!existsSync2(tfile)) return send(res, 401, "application/json", JSON.stringify({ error: "unknown ticket" }), cors);
      const { did } = await decryptJson(storeKey, new Uint8Array(readFileSync2(tfile)));
      const sfile = sessionPath(did);
      if (!existsSync2(sfile)) return send(res, 404, "application/json", JSON.stringify({ error: "no session" }), cors);
      let session = await decryptJson(storeKey, new Uint8Array(readFileSync2(sfile)));
      let r = await pdsAuthedGet(session, "/xrpc/com.atproto.server.getSession");
      if (r.status === 401 && session.refreshToken) {
        session = await confidentialRefresh(session, sign);
        writeFileSync2(sfile, await encryptJson(storeKey, session), { mode: 384 });
        measure(`broker-refresh did=${did}`);
        r = await pdsAuthedGet(session, "/xrpc/com.atproto.server.getSession");
      }
      if (r.status < 200 || r.status >= 300) return send(res, 502, "application/json", JSON.stringify({ error: `PDS ${r.status}`, detail: r.data }), cors);
      return send(res, 200, "application/json", JSON.stringify({ did, handle: r.data.handle, via: "auth-helper (brokered, token server-side)" }), cors);
    }
    return send(res, 404, "text/plain", "not found", cors);
  } catch (e) {
    console.error("[auth-helper] error:", e.message);
    return send(res, 500, "text/plain", `error: ${e.message}`);
  }
});
server.listen(PORT, HOST, () => {
  console.log(`[auth-helper] listening on ${HOST}:${PORT}; client_id=${CLIENT_ID}; data=${DATA_DIR}`);
});
