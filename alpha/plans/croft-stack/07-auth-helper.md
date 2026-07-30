# Phase 7 — Auth helper (confidential-client spike → shared broker)

← [06-iroh-relay.md](06-iroh-relay.md) · [roadmap](README.md) · next → [08-cache-server.md](08-cache-server.md)

**Status:** **DONE + LIVE (2026-07-29).** The validation spike proved GO (2026-07-24); the **production
Rust broker** is now built (`croft-stack/broker/`, TDD 70/70, clippy/fmt clean) and **live at
`account.croft.ing`** (governed 192M/256, keys 0600, prod-LE TLS; `/healthz`, `/jwks.json`,
`/client-metadata.json` serving). Build plan + phases: [auth-broker-plan.md](auth-broker-plan.md). ·
**Gate-out (spike, met):** a pad holds a helper-brokered session and falls back cleanly when the helper
is stopped. **Gate-out (production, met):** the Rust broker honors the confidential-client contract +
secrets discipline (0600 keys), governed unit, converge idempotent. Optional next: a live round-trip
(one interactive authorize).

---

## Spike outcome (2026-07-24) — GO, proven live

Full record: `discovery/spike/auth-helper/` (`FINDINGS.md`, `FLOW-SPEC.md`, `BOX-CHANGELOG.md`,
`helper/` with 15 hermetic tests, `pad/`). Throwaway spike, both testbeds left running at zero cost.

- **Confidential login — proven.** Helper at `https://account.croft.ing` (private key on the box)
  signed the real test account `ngvalidation2112.bsky.social` in; bsky.social accepted private-key
  client auth and issued a session + refresh token.
- **Server-side background refresh — proven.** Refreshed with no browser; the refresh token rotates
  (single-use), per spec.
- **Cross-domain brokered pad — proven (the important one).** `https://stellin.app` (a deliberately
  *different* domain) signed in via the helper, received an **opaque ticket** (first-party storage,
  bearer token — **not a cross-site cookie**, which Safari/WebKit purge — the account-kernel K1 lesson
  applied), then asked the helper "who am I?"; the helper answered on the pad's behalf and the pad
  **never touched the token**. This is the robust pattern for the shared estate helper.
- **Clean fallback — proven** (bar one human click). Helper stopped ⇒ pad's reachability check flipped,
  pad kept serving, its independent browser-only public client stayed in place; restarted fine.
- **TTL — spec-cited (retires Open decision 9).** Access token ~1h (measured 3599s). Refresh: public
  up to ~2 weeks; confidential up to ~180 days, session possibly unlimited via refresh. Long-run
  survival is now being **measured over calendar time** (daily refresh on the box); the mechanism is
  proven, the multi-week number is an observation-in-progress, not an open question.
- **Spec divergence registered.** `token_endpoint_auth_signing_alg` is spec-optional but bsky.social
  **rejects the confidential client without it**; adding it fixed login. Do not trust the spec text here.

**Still open (calendar-time confirmations, not mechanism questions):** long-run session survival
(>2wk observation running); the live public-vs-confidential delta (needs one more public login of the
same account); the final fallback click.

---

## Problem

A browser-only PWA is a *public* OAuth client, so its DPoP-bound session is short-lived (the observed
~2-week TTL — Open decision 9, no FACTCHECK yet). A shared, server-side **confidential** client can
refresh server-side and broker a longer-lived session — an optional accelerator, preferred when
reachable, with a clean fallback. This is the **least-proven piece** (net-new invention): `authserve.rs`
proved the service-auth JWT verifier but explicitly named the interactive OAuth login leg a non-goal.

**Production build plan (Rust, phase-planned): [auth-broker-plan.md](auth-broker-plan.md)** — the
hardened rewrite, TDD across small phases (Phase 0 crate-pinning → crypto → OAuth → session → server →
deploy). Scope fact: the spike is ~1,340 LOC hand-rolled crypto (no atproto-OAuth lib), so the Rust
broker is a comparable security-sensitive build, not a one-shot.

## Approach

**Spike first, then build.** The throwaway spike (like account-kernel/K1) **has confirmed** the
confidential-client login + server-side refresh leg live (see outcome above). Remaining work: build the
**production broker** as a governed mini-stack at `account.croft.ing` — Rust, hardened, multi-account —
reusing the proven flow and the opaque-ticket cross-domain pattern.

## The validation spike (executed 2026-07-24 — the sub-plan it followed, now the record)

One thing to prove live: a confidential client (backend-held key, hosted metadata) can (1) complete an
authorization-code login for a real account and (2) refresh **server-side, no browser**, past the
browser-only TTL — and a pad prefers the helper when up, falls back when down.

- **Ground truth first (no guessing).** Confirmed deltas over the existing PUBLIC client
  (`croft-pwa/client-metadata.json`, `token_endpoint_auth_method: "none"`): confidential flips to
  `private_key_jwt`, adds a hosted `jwks`, signs a client assertion at the token endpoint, holds the
  private key server-side, redirect under the helper's control. Reuse the proven flow code in
  `croft-pwa/src/atproto/oauth/{client,dpop,pkce,resolve,jose}.ts` (PAR + PKCE + DPoP) — extend, don't
  reimplement.
- **Stages (each gated):** (A) ground the flow, cited; (B) stand up the confidential client on the box
  behind Caddy TLS at `account.croft.ing` — serve `client-metadata.json`/`jwks`/`/callback`/`/healthz`,
  private key server-held (`Zeroize`, never logged/committed); (C) **human-in-the-loop** login: build
  the authorize URL, owner authorizes interactively, exchange the code with DPoP + `private_key_jwt`;
  (D) **measure** server-side refresh and compare confidential vs public TTLs live (this retires Open
  decision 9 / feeds the FACTCHECK); (E) preferred-when-reachable + clean fallback with the helper down.
- **Assets:** the box; a real test account (owner-provided); the three staged GitHub Pages repos
  (`k1-appa`/`k1-appb`/`kernel-k1` at `*.croft.ing`) as pad skins.
- **Deliverable:** `discovery/spike/auth-helper/FINDINGS.md` (account-kernel shape): what is PROVEN, the
  measured TTL table, what is UNTESTED, go/no-go on the confidential-client value claim. Outcomes only.

## Then: the shared broker (build)

Honors `CONTRACT.md` + the **secrets addendum** (confidential client key = `Zeroize`, never logged/
serialized clear). Shared, `account.croft.ing`, canonical data profile (sessions/keys). Deploy via the
forced-command channel. Wire one pad to **prefer** it (fall back to browser-only on failure).

## TODO (decide on arrival)
- [ ] Confidential client identity: `client_id = https://account.croft.ing/client-metadata.json`; is a
      `did:web` service identity needed anywhere, or is the URL client_id sufficient? (Independent of
      the contested Stellin service DID — Open decision 6.)
- [ ] Language: spike may extend the TS OAuth code; the production broker's language (Rust per kit
      discipline vs keep TS) — decide before building the mini-stack.
- [ ] Which pad wires "prefer helper" first for the gate.

## Risks & cautions
- **Least-proven piece** — if the spike stalls, the pre-authorized pivot is cache-first (Open decision
  3): bring Phase 8 up first and let the auth helper follow.
- Interactive authorize needs the owner (a human + browser) — the server-side refresh is the unattended
  part; plan the pause point.
- Secrets discipline: private key + account creds via env / mode-0600, never committed/logged; TDD
  red-first for kept code.

## Validation
A pad session brokered by the helper survives past the measured browser-only TTL; stopping the helper
returns the pad to browser-only OAuth with no break.

## References
`RUN-14-SUMMARY.md` (EXP-A; OAuth-login non-goal), `authserve.rs`, `serviceauth.rs`;
`croft-pwa/src/atproto/oauth/`, `croft-pwa/client-metadata.json`;
`discovery/spike/account-kernel/FINDINGS-AND-PIVOT.md` (session-broker/outbox lineage), OPEN-THREADS T55.
