// src/refresh-cli.ts
import { readFileSync as readFileSync2, writeFileSync as writeFileSync2, existsSync as existsSync2, appendFileSync } from "node:fs";
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
async function signJwt(header, payload, key) {
  const signingInput = `${b64urlFromString(JSON.stringify(header))}.${b64urlFromString(JSON.stringify(payload))}`;
  const sig = await subtle().sign(
    { name: "ECDSA", hash: "SHA-256" },
    key,
    new TextEncoder().encode(signingInput)
  );
  return `${signingInput}.${b64urlFromBytes(new Uint8Array(sig))}`;
}

// src/oauth/dpop.ts
function subtle2() {
  const c = globalThis.crypto;
  if (!c?.subtle) throw new Error("WebCrypto is unavailable");
  return c.subtle;
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
function ensureDir(file2) {
  mkdirSync(dirname(file2), { recursive: true });
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
  const publicJwk = { kty: "EC", crv: "P-256", x: privJwk.x, y: privJwk.y };
  const kid = await jwkThumbprint(publicJwk);
  return { key: { privateKey, kid }, publicJwk };
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

// src/refresh-cli.ts
var DATA_DIR = process.env.AUTH_HELPER_DATA_DIR ?? join(process.cwd(), "data");
var ORIGIN = process.env.AUTH_HELPER_ORIGIN ?? "https://account.croft.ing";
var CLIENT_ID = `${ORIGIN}/client-metadata.json`;
var did = process.argv[2];
if (!did) {
  console.error("usage: refresh-cli <did>");
  process.exit(2);
}
var safe = (s) => s.replace(/[^A-Za-z0-9._-]/g, "_");
var file = join(DATA_DIR, "sessions", `${safe(did)}.enc`);
if (!existsSync2(file)) {
  console.error(`no stored session for ${did} at ${file}`);
  process.exit(1);
}
var { key: assertionKey } = await loadOrCreateAssertionKey(join(DATA_DIR, "assertion-key.jwk"));
var storeKey = await loadOrCreateStoreKey(join(DATA_DIR, "store-key.bin"));
var sign = assertionSigner(CLIENT_ID, assertionKey);
var before = await decryptJson(storeKey, new Uint8Array(readFileSync2(file)));
var oldRefresh = before.refreshToken;
var after = await confidentialRefresh(before, sign);
writeFileSync2(file, await encryptJson(storeKey, after), { mode: 384 });
var ttl = after.expiresAt ? Math.round((after.expiresAt - Date.now()) / 1e3) : void 0;
var rotated = after.refreshToken !== oldRefresh;
var line = `refresh did=${did} access_expires_in=${ttl}s refresh_rotated=${rotated}`;
appendFileSync(join(DATA_DIR, "measurements.log"), `${(/* @__PURE__ */ new Date()).toISOString()} ${line}
`);
console.log(`[refresh] ${line}`);
console.log(`[refresh] new access token acquired server-side, no browser. refresh token ${rotated ? "ROTATED (single-use, as spec)" : "unchanged"}.`);
