// oauth/jose.ts
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

// oauth/pkce.ts
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

// oauth/dpop.ts
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
async function createDpopProof(input) {
  const header = { typ: "dpop+jwt", alg: "ES256", jwk: input.key.publicJwk };
  const iat = input.iat ?? Math.floor(Date.now() / 1e3);
  const payload = { jti: randomB64url(16), htm: input.htm, htu: input.htu, iat };
  if (input.nonce) payload.nonce = input.nonce;
  if (input.accessToken) payload.ath = b64urlFromBytes(await sha256(input.accessToken));
  return signJwt(header, payload, input.key.privateKey);
}

// oauth/read.ts
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

// oauth/resolve.ts
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

// oauth/client.ts
function fetchOf3(cfg) {
  return cfg.fetchImpl ?? globalThis.fetch.bind(globalThis);
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
async function beginAuthorization(handleOrDid, cfg, deps = {}) {
  const fetchImpl = fetchOf3(cfg);
  const id = await resolveIdentity(handleOrDid, { ...deps, ...cfg.fetchImpl ? { fetchImpl } : {} });
  const pkce = await createPkce();
  const key = await generateDpopKey();
  const state = randomB64url(16);
  const { data, status } = await dpopForm(
    id.meta.pushed_authorization_request_endpoint,
    {
      client_id: cfg.clientId,
      response_type: "code",
      redirect_uri: cfg.redirectUri,
      scope: cfg.scope,
      state,
      code_challenge: pkce.challenge,
      code_challenge_method: "S256",
      login_hint: handleOrDid
    },
    key,
    fetchImpl
  );
  const requestUri = data.request_uri;
  if (typeof requestUri !== "string") {
    throw new Error(`PAR failed (${status})${data.error ? `: ${data.error}` : ""}`);
  }
  const authorizeUrl = new URL(id.meta.authorization_endpoint);
  authorizeUrl.searchParams.set("client_id", cfg.clientId);
  authorizeUrl.searchParams.set("request_uri", requestUri);
  return {
    authorizeUrl: authorizeUrl.toString(),
    pending: {
      state,
      verifier: pkce.verifier,
      dpopKey: await exportDpopKey(key),
      did: id.did,
      pds: id.pds,
      authServer: id.authServer,
      issuer: id.meta.issuer,
      authorizationEndpoint: id.meta.authorization_endpoint,
      tokenEndpoint: id.meta.token_endpoint,
      parEndpoint: id.meta.pushed_authorization_request_endpoint
    }
  };
}
async function completeAuthorization(pending, callback, cfg) {
  if (callback.state !== pending.state) throw new Error("OAuth state mismatch \u2014 refusing the callback");
  const fetchImpl = fetchOf3(cfg);
  const key = await importDpopKey(pending.dpopKey);
  const { data, nonce, status } = await dpopForm(
    pending.tokenEndpoint,
    {
      grant_type: "authorization_code",
      code: callback.code,
      redirect_uri: cfg.redirectUri,
      client_id: cfg.clientId,
      code_verifier: pending.verifier
    },
    key,
    fetchImpl
  );
  const accessToken = data.access_token;
  if (typeof accessToken !== "string") {
    throw new Error(`Token exchange failed (${status})${data.error ? `: ${data.error}` : ""}`);
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
    clientId: cfg.clientId,
    dpopKey: pending.dpopKey,
    ...nonce ? { dpopNonce: nonce } : {},
    ...typeof data.expires_in === "number" ? { expiresAt: Date.now() + data.expires_in * 1e3 } : {}
  };
}

// app.ts
var HELPER = "https://account.croft.ing";
var PAD_ORIGIN = location.origin;
var HANDLE = "ngvalidation2112.bsky.social";
var TICKET_KEY = "helper_ticket";
var PENDING_KEY = "pub_pending";
var pubCfg = {
  clientId: `${PAD_ORIGIN}/public-client-metadata.json`,
  redirectUri: `${PAD_ORIGIN}/`,
  scope: "atproto transition:generic"
};
var app = document.getElementById("app");
var logEl = document.createElement("pre");
logEl.id = "log";
function log(msg) {
  logEl.textContent += `${msg}
`;
}
function el(tag, text, onclick) {
  const e = document.createElement(tag);
  e.textContent = text;
  if (onclick) e.onclick = onclick;
  return e;
}
async function helperReachable() {
  try {
    const r = await fetch(`${HELPER}/healthz`, { signal: AbortSignal.timeout(4e3) });
    return r.ok;
  } catch {
    return false;
  }
}
async function whoamiBrokered() {
  const ticket = localStorage.getItem(TICKET_KEY);
  if (!ticket) return log("no helper ticket \u2014 sign in via the helper first");
  try {
    const r = await fetch(`${HELPER}/api/whoami`, { headers: { authorization: `Bearer ${ticket}` } });
    const data = await r.json();
    log(`brokered whoami [${r.status}]: ${JSON.stringify(data)}`);
  } catch (e) {
    log(`brokered whoami failed: ${e.message}`);
  }
}
async function browserOnlySignIn() {
  log("browser-only: beginning public-client OAuth (no helper)\u2026");
  const { authorizeUrl, pending } = await beginAuthorization(HANDLE, pubCfg);
  sessionStorage.setItem(PENDING_KEY, JSON.stringify(pending));
  location.assign(authorizeUrl);
}
async function handleCallback(params) {
  const ticket = params.get("ticket");
  const code = params.get("code");
  const state = params.get("state");
  const error = params.get("error");
  if (error) return log(`authorization error: ${error} ${params.get("error_description") ?? ""}`);
  if (ticket) {
    localStorage.setItem(TICKET_KEY, ticket);
    history.replaceState(null, "", PAD_ORIGIN + "/");
    log("signed in VIA HELPER (brokered). Ticket stored first-party; token stays on the helper.");
    return;
  }
  if (code && state) {
    const raw = sessionStorage.getItem(PENDING_KEY);
    if (!raw) return log("browser-only callback but no pending state");
    const pending = JSON.parse(raw);
    const session = await completeAuthorization(pending, { code, state }, pubCfg);
    sessionStorage.removeItem(PENDING_KEY);
    history.replaceState(null, "", PAD_ORIGIN + "/");
    log(`signed in BROWSER-ONLY (public client). DID: ${session.did} \xB7 access TTL ~${session.expiresAt ? Math.round((session.expiresAt - Date.now()) / 1e3) : "?"}s`);
    return;
  }
}
async function main() {
  const h1 = el("h1", "stellin.app \u2014 auth-helper integration demo");
  app.append(h1, logEl);
  const btnHelper = el("button", "1. Sign in via helper (brokered)", () => {
    location.assign(`${HELPER}/login?handle=${encodeURIComponent(HANDLE)}&return=${encodeURIComponent(PAD_ORIGIN + "/")}`);
  });
  const btnWho = el("button", "2. Who am I? (brokered call)", () => void whoamiBrokered());
  const btnFallback = el("button", "3. Sign in browser-only (fallback)", () => void browserOnlySignIn());
  app.append(btnHelper, document.createTextNode(" "), btnWho, document.createTextNode(" "), btnFallback);
  await handleCallback(new URLSearchParams(location.search));
  const up = await helperReachable();
  log(`helper /healthz reachable: ${up ? "YES \u2014 preferring the brokered helper session" : "NO \u2014 falling back to the browser-only public client"}`);
  if (!up) log("(the pad still works: use button 3 to sign in browser-only, exactly as it would with no helper at all.)");
}
void main();
