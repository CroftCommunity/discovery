# auth-helper — security analysis (POC posture + production recommendations)

date: 2026-07-24 · scope: the confidential OAuth helper spike and the design it stands in for. This
separates **what the throwaway POC does today** from **what the production broker must do**. Nothing
here is production; the POC's job was to prove the mechanism, not to be safe to run at scale.

> This file is the spike's **threat-model reasoning** and stays in the design corpus. The **decisions**
> it produced (the H1–H8 hardening, KMS choice, tenant isolation) are tracked alongside the component in
> the croft-stack repo: `croft-stack/docs/auth-helper.md`.

## The core tradeoff (read this first)

A browser-only pad (the serverless floor) holds **no server-side secret** and has **no central
honeypot**: each user's session lives only in their own browser, DPoP-bound to a key that pad. Compromise
is per-user and per-device.

The confidential helper **concentrates risk to buy session longevity and brokering**. One backend holds
(a) a single client private key that authenticates the client to the authorization server, and (b) live
sessions for many users. That is a high-value target: it is the difference between "attacker phishes one
user" and "attacker who owns the box can act as every user who opted into the helper."

**The mitigation is architectural, not incidental:** the helper is **optional**. Every pad works without
it, and a user or pad that does not trust it simply uses the browser-only path (proven live). So the
security story is not "make the honeypot unbreakable" — it is "keep the honeypot optional, minimal,
revocable, and observable, and make sure removing it degrades to a safe floor." Every recommendation
below serves that.

## Assets, ranked by what an attacker gains

| # | Asset | Where it lives (POC) | What theft grants |
|---|---|---|---|
| A1 | **Client private key** (ES256 assertion key) | plaintext JWK, `data/assertion-key.jwk`, mode 0600 | Impersonate the *client* to the AS — mint client assertions, complete/refresh flows as the helper. Estate-wide. |
| A2 | **User sessions** (access + refresh tokens + per-session DPoP key) | AES-GCM `.enc` files, `data/sessions/` | Act as those users against their PDS; refresh indefinitely (confidential = long/unbounded session). |
| A3 | **Store key** (AES-256) | plaintext, `data/store-key.bin`, mode 0600 | Decrypt A2 at rest. |
| A4 | **Tickets** (bearer handles) | AES-GCM `.enc`, `data/tickets/`; copy in the pad's first-party storage | Call the helper's broker API as that user until the ticket is revoked. |
| A5 | **The box** (SSH/root) | OVH VM | Everything above, plus persistence and code substitution. |

Note the collapse: **A5 ⇒ A1+A2+A3+A4.** Because the client key, the store key, and the ciphertext it
protects all sit on the same disk as mode-0600 files, root on the box reads all of them. The POC's
"encryption at rest" therefore protects against *narrow* leaks (a stray backup of `sessions/` without the
key, a mis-scoped file read) — **not** against box compromise. Be honest about that: at-rest encryption
with a co-located key is a backup/least-privilege control, not a defense against a rooted host.

## Case-by-case walkthrough

### Case B — the box is compromised (root or the service user)
- **Impact:** total. A1–A4 all readable; attacker can also swap the binary and harvest going forward.
- **POC posture:** SSH is **password auth with passwordless sudo**; there is **no host firewall**
  (nothing else listens, but no default-drop); the service runs non-root under systemd with
  `ProtectSystem=strict`, empty caps, `ReadWritePaths` scoped to the data dir. So the *service* is
  reasonably boxed, but *host access* is weak (password SSH is the POC's biggest single weakness).
- **Production:**
  - SSH keys only, no passwords; fail2ban; nftables **default-drop** (22/80/443 only) — the host-kit
    already generates this, the spike just didn't apply it.
  - Move A1 out of the process entirely: sign client assertions via a **KMS/HSM** (cloud KMS, TPM, or
    PKCS#11) so the private key never exists in the helper's memory in the clear. Root on the box can
    then *use* the signing oracle while it runs, but cannot *exfiltrate* the key — which bounds the
    damage to the compromise window and makes rotation meaningful.
  - Envelope-encrypt A2: a KMS master key wraps per-record (or per-user) data keys; the unwrapped key
    lives only transiently in memory. Rust production zeroizes it (`Zeroize`/`ZeroizeOnDrop`) — the plan
    already calls for Rust here.
  - Audit logging off-box (append-only, shipped) so a compromise is *detectable* and *scoped in time*.

### Case A1 — the client private key leaks (without full box compromise)
- **Impact:** attacker can authenticate as the client to any atproto AS and drive OAuth flows in the
  client's name (start logins, complete token exchanges, refresh). Estate-wide because the helper is a
  single shared client identity.
- **POC posture:** plaintext on disk; in-memory it is a **non-extractable** `CryptoKey` (generated
  extractable, written to the 0600 file, re-imported non-extractable) — so a *memory* read is harder
  than a *file* read, but the file is the soft spot.
- **Production:**
  - KMS/HSM as above (never on disk in the clear).
  - **Key rotation via JWKS:** publish multiple keys with distinct `kid`s; rotate on a schedule and on
    suspicion. A leaked key is revoked by removing its JWK and rolling to a new `kid`; assertions signed
    with the old key stop validating. (The metadata already advertises `jwks_uri`, so rotation is a
    document update, not a client-id change.)

### Case A2 — the session store leaks (tokens exfiltrated)
- **Impact:** attacker holds users' refresh tokens. Because atproto refresh tokens **rotate** (single-use),
  a race exists: whoever refreshes first wins and the other's token becomes invalid — so a theft is
  partially *self-detecting* (the legit helper's next refresh fails, signalling compromise). But a
  confidential session can be very long-lived, so a stolen, unused refresh token is a durable capability.
  Access tokens are **DPoP-bound to the per-session key** (also in the store), so the DPoP key must be
  stolen alongside — which it is, since it is in the same record.
- **POC posture:** AES-GCM at rest under A3 (co-located). DPoP binding is real but does not help against
  a store theft that takes the DPoP key too.
- **Production:**
  - Envelope encryption with an off-box master (Case B).
  - **Revocation + rotation on suspicion:** call the AS token-revocation endpoint and force re-login;
    the browser-only floor means users can always re-establish without the helper.
  - **Anomaly detection** on refresh (unexpected `use` rate, geo/asn shifts, rotation-race failures).
  - Consider **per-user data keys** so one leaked record does not imply all.

### Case A4 — a ticket is stolen (the cross-origin capability)
- **Impact:** the thief can call the helper's **broker API** as that user — but only the surface the
  helper exposes (currently a read `whoami`; in production, whatever the outbox/broker allows), and only
  until the ticket is revoked. Crucially the thief **never gets the atproto token** — that stays on the
  helper. So a ticket is a *scoped, revocable* capability, which is strictly better than handing out the
  raw session.
- **POC posture:** opaque 24-byte random, stored **first-party** in the pad (not a cross-site cookie —
  deliberately, so Safari/WebKit ITP cannot be the failure mode; this applies the account-kernel K1
  lesson). TLS-only. **But: no expiry, no rotation, no origin binding, no per-call audit** — a stolen
  ticket works indefinitely.
- **Production:**
  - **Short TTL + refresh** on tickets (treat like a session cookie): rotate frequently, expire quickly.
  - **Bind the ticket to a pad-held DPoP key** so a stolen ticket without the key is useless (raises the
    bar from bearer to proof-of-possession, cross-origin-safe).
  - **Bind to the pad origin** and re-check it server-side; keep the CORS allowlist explicit (it is).
  - **Scope tickets** to the minimum broker operations the pad needs; per-ticket revocation list; audit
    every brokered call.

### Case E — the pad is XSS'd (browser-side compromise)
- **Brokered path:** XSS steals the **ticket**, not the token. Damage is bounded to the broker API and is
  **revocable** server-side. This is a real security *gain* of brokering over browser-only.
- **Browser-only path:** XSS can drive the DPoP-bound session directly from the page (the key is ideally
  non-extractable WebCrypto, so it may not be *exfiltrated*, but it can be *used* in-page while the XSS
  runs). Damage is per-user, not revocable centrally beyond the AS session.
- **Production (both):** strong CSP + SRI on pad assets (croft-pwa already does this; the spike's demo pad
  does not — it is a bare demo), Trusted Types, no third-party script, subresource pinning. The broker
  path additionally wants per-ticket scoping so XSS cannot exceed the pad's legitimate capability.

### Case F — network, TLS, redirect, CSRF
- **TLS:** Caddy auto-HTTPS (Let's Encrypt), HSTS worth adding. Good.
- **PKCE + `state`:** PKCE S256 is used; `state` is generated and **checked** on callback (mismatch
  rejected). CSRF on the callback is covered by `state` + the server-side pending record.
- **Redirect_uri / open-redirect:** the ticket handoff redirects only to an **allowlisted** `return`
  prefix (checked server-side); `redirect_uri` is fixed in client metadata and must match. Keep the
  allowlist tight; treat any `return`/`redirect` param as attacker-controlled.
- **DPoP nonce:** the `use_dpop_nonce` handshake is honored (single retry), so the AS's replay protection
  is respected.
- **Production:** add rate limiting on `/login`, `/callback`, `/api/*`; add HSTS; consider mTLS or an
  allowlist between Caddy and any internal broker services.

### Case G — blast radius of the *shared* helper
- The plan chose **one shared confidential client** for the whole estate (minimal held state, one
  identity). Security-wise that maximizes blast radius: A1 compromise is estate-wide.
- **Recommendation to weigh past POC:** isolate high-value tenants. Options: per-product confidential
  clients (more key sprawl, smaller blast radius each), or one client identity but **per-tenant signing
  keys / per-tenant data-key domains** so a breach is containable. Document the chosen point on the
  sprawl-vs-blast-radius curve explicitly; do not let "shared" be an unexamined default.

### Case H — supply chain / build
- **POC:** Node + a handful of dev deps; the pad is an esbuild bundle; the helper bundle is esbuild too.
  No dependency pinning/audit gate, no SRI on the demo pad.
- **Production:** lockfile + `npm audit`/`cargo audit` gate, pinned toolchains, reproducible builds,
  SRI on all pad assets, review of the bundle, signed releases into the forced-command deploy channel
  (the host-kit's `deploy-receive.sh` already constrains deploys to rsync-into-incoming + activate).

### Case I — availability / DoS
- Losing the helper is **not** a security failure by design: pads fall back to browser-only (proven).
  So DoS on the helper degrades UX (shorter sessions), it does not break access. This is the optionality
  dividend. Still add rate limiting and per-process cgroup limits (the host-kit does the latter).

## What the POC deliberately does NOT do (so it is not mistaken for safe)

- Client key and store key are **plaintext on disk** (0600) — no KMS/HSM, no envelope encryption.
- Tickets have **no expiry, rotation, origin-binding, or per-call audit**.
- SSH is **password auth**; **no host firewall**; creds are **throwaway and shared in the session**
  (owner rotates after).
- No rate limiting, no audit log, no anomaly detection, no key rotation.
- Single shared client identity — **no tenant isolation**.
- The demo pad has **no CSP/SRI** (croft-pwa's real pads do).

## Recommendations past POC, prioritized

1. **Get the client key off disk and out of memory** — KMS/HSM signing oracle + JWKS key rotation. (A1, Case B)
2. **Harden the host** — SSH keys only, nftables default-drop, fail2ban, unattended-upgrades, off-box audit log. (Case B)
3. **Envelope-encrypt sessions** with an off-box master + in-memory zeroization (Rust). (A2)
4. **Make tickets real session credentials** — short TTL, rotation, DPoP-bound proof-of-possession, origin binding, per-ticket scope + revocation, audit every brokered call. (A4)
5. **Decide tenant isolation explicitly** — per-tenant keys/data-key domains vs one shared client; record the blast-radius choice. (G)
6. **Revocation + detection** — AS token revocation path, refresh-rotation-race monitoring, anomaly alerts; lean on the browser-only floor for recovery. (A2)
7. **Pad hardening + supply chain** — CSP/SRI/Trusted Types on pads, dependency-audit gate, signed deploys. (E, H)
8. **Keep the floor sacred** — never let any pad *require* the helper; the optionality is the security backstop for all of the above.

## Standing convention — zeroize secret material (adopted 2026-07-24)

**Zeroize secret material in memory wherever the language allows it** — private keys, session/refresh
tokens, derived keys, passwords — as standard Croft practice, not just in the auth helper.

- **Rust** (the production broker's language): newtype wrappers deriving `Zeroize` + `ZeroizeOnDrop`;
  never `Debug`/serialize a secret in the clear. (Already the global Rust rule; this restates it as
  cross-cutting.)
- **Go / C / C++**: explicit wipe after use; keep secrets out of GC'd/immutable buffers.
- **GC languages (JS/TS, Python)**: be honest — you **cannot** reliably zeroize (immutable strings, a
  copying GC). Best-effort only (hold secrets in a `Uint8Array` you can overwrite, never a `string`;
  minimise lifetime). This limitation is itself a reason secret-holding components are Rust, not the
  Node spike. The POC helper does **not** zeroize — a known POC gap folded into H3.
- Pairs with: never log/print secrets, load from mode-0600/KMS not source, hold non-extractable keys
  where the platform allows (`extractable:false`, KMS/HSM signing oracle).

The natural home for the cross-language version is the synced coding standards
(`coding-agents/CLAUDE.md`), which already carries the Rust rule; broadening it there keeps it
consistent across every repo and Claude environment.
