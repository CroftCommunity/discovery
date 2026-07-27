# auth-helper spike — findings (plain English, for review)

date: 2026-07-24 · status: **concluded for the core claim; two live measurements still running.**
Companion docs: `FLOW-SPEC.md` (the grounded flow, every field cited), `BOX-CHANGELOG.md` (every change
made to the OVH box + how to undo it), `deploy/` (the exact unit + Caddy config as deployed).

## Why we did this

Every Croft pad is meant to work with **no server of ours**. A browser-only pad signs a user into their
atproto account directly, but because a browser app is a "public" OAuth client, the account session it
holds is **short-lived** — the spec caps it at about **two weeks**, after which the user has to sign in
again.

The idea on the table was an **optional** helper: a small backend that holds a private key and is
therefore a "confidential" OAuth client. A confidential client is allowed **much longer** sessions and
can **refresh them in the background with no browser open**. If a pad can lean on this helper when it's
running — and fall straight back to browser-only sign-in when it isn't — users get long-lived sessions
without the helper ever becoming a requirement.

Nobody had actually run this. An earlier proof (`authserve`) deliberately skipped the interactive OAuth
login, calling it out of reach in a headless environment. So this was the **least-proven piece** of the
whole accelerator plan. This spike ran exactly that missing leg, live, against a real account.

## What we built (both left running, at zero cost)

- **The helper** at `https://account.croft.ing` — a confidential OAuth client on a rented OVH server.
  It holds the private key (which never leaves the box), completes the real login, holds the session
  encrypted on disk, and refreshes it in the background.
- **A demo pad** at `https://stellin.app` — a deliberately *different* domain (not `croft.ing`), to
  prove the helper works for any pad, not just ones on its own domain. The pad can sign in two ways:
  through the helper, or — if the helper is down — directly as its own browser-only public client.

## What we tested and what happened

**1. Can a confidential client complete a real login?** Yes. We signed the real test account
(`ngvalidation2112.bsky.social`) in through the helper. The authorization server (bsky.social) accepted
the helper's private-key-based client authentication and issued a real, working session with a refresh
token. **Proven.**

**2. Can the helper refresh that session in the background, with no browser?** Yes. We ran the refresh
on the server, no browser involved. It got a fresh access token and, as the spec requires, the refresh
token **rotated** (each one is single-use). **Proven.**

**3. Can a pad on a completely different domain use the helper?** Yes, and this is the important one.
From `stellin.app`, the pad sent the user to the helper to sign in, got back an opaque **ticket** (not a
cookie — see the design note below), and then asked the helper "who am I?". The helper answered by
calling the user's account **on the pad's behalf**, using the session it holds. The pad got back the
correct identity (`ngvalidation2112.bsky.social`) and **never touched the token itself** — it stays on
the helper. Confirmed live in a real browser (screenshots on file). **Proven.**

**4. Does the pad still work when the helper is switched off?** Yes — proven end-to-end. We stopped the
helper and drove the pad in a real browser (Playwright): its "is the helper reachable?" check correctly
flipped to **no**, and clicking the fallback button ran the pad's **own** browser-only public-client
login against bsky.social — which completed and signed the user in as the **same account**
(`did:plc:xyfhcaweaeyew3zrgk6jaln7`), with the helper entirely down. Then we restarted the helper. So the
helper is a **true optional accelerator**: present, it lengthens sessions and brokers cross-origin calls;
absent, the pad degrades to exactly today's browser-only behaviour with nothing broken. **Proven.**
(Side note: with the helper down, its `/healthz` returns a plain 502 with no CORS header, so the browser
reports it as unreachable — the pad treats any failure as "helper down", which is the correct and robust
behaviour.)

## The session-lifetime picture (the headline number)

The original mental model was "browser-only ≈ 2 weeks, backend helper ≈ 3 months." That is essentially
right, and the helper is if anything **better** than "3 months":

| | Browser-only pad (public) | Helper (confidential) |
|---|---|---|
| Access token (short-lived working token) | ~1 hour | ~1 hour (measured: 3599s) |
| Refresh token | up to **2 weeks** | up to **180 days** (~6 months) |
| Overall session (kept alive by refreshing) | up to **2 weeks** | **may be unlimited** |

These numbers come from the atproto OAuth specification, which now gives us a **citable source** for the
"~2 week" figure that previously had none (this was tracked as Open decision 9). The access token being
short-lived is the same for both — the advantage is entirely in **how long you can keep the session
alive**, which is where the helper wins.

**What we have confirmed live** so far: the access token is ~1 hour, and background refresh genuinely
works and rotates. **What is still being measured**: the *actual* long-run session lifetime the live
server enforces — there is no field in the response that states it; it only reveals itself by refreshing
day after day until a public session dies (~2 weeks) while the confidential one keeps going. A daily
background refresh is now running on the box to capture this over the coming weeks.

## Two things worth carrying forward

**A spec gap we hit.** The written spec lists one metadata field (`token_endpoint_auth_signing_alg`) as
optional, but the live bsky.social server **rejected the confidential client without it**. Adding it
fixed the login. Recorded in the spec-divergence register so we don't trust the spec text here again.

**The cross-domain design choice (and a hard-won lesson applied).** For a pad on a different domain, the
tempting way to carry the session across is a cross-site cookie — but Safari/WebKit (every iOS browser)
actively purges those, which is exactly the class of failure the earlier account-kernel spike got burned
by. So instead the pad receives an **opaque ticket** it stores in its own first-party storage and sends
back to the helper as a bearer token. No cross-site cookie, so it is immune to that whole problem. This
is the robust pattern for the shared helper serving the whole estate.

## Still open (honest gaps)

- **Long-run session survival** — the daily refresh timer must run for >2 weeks to *observe* the
  confidential-vs-public lifetime gap the spec promises. Running now; not yet observed.
- **The public-vs-confidential delta, live** — to measure the gap directly we would also leave a
  browser-only session of the *same* account refreshing alongside the helper's, and watch which dies
  first. Not yet set up (the browser-only login was completed once, above, but not put on a refresh loop).
- This is a **throwaway spike**, not the product. The real broker will be rebuilt in Rust, hardened,
  and multi-account. Nothing here is production.

## Go / no-go on the confidential-client value claim

**GO.** The thing this spike existed to de-risk — that a confidential, backend-held OAuth client can log
a real account in and keep its session alive server-side with no browser, and that a pad on any domain
can prefer it when present and fall back cleanly when absent — is **proven end-to-end against the live
network**. The session-length advantage is real and now has a spec citation. The remaining items are
confirmations over calendar time, not open questions about whether the mechanism works.

## Reproduce (testbeds kept live)

- Helper: `https://account.croft.ing/{healthz,client-metadata.json,jwks.json,login,callback,api/whoami}`.
- Pad: `https://stellin.app/` (buttons 1–3), its public client at `/public-client-metadata.json`.
- Code + tests: `helper/` (run `npx vitest run` — 15 hermetic tests). Browser fallback client: `pad/`.
- Browser-only fallback proof: `pad/pw-fallback.mjs` (Playwright). Run with the helper stopped and
  `TEST_HANDLE` / `TEST_PASSWORD` in the env; it drives the pad through a real browser-only login and
  asserts the "signed in BROWSER-ONLY" result. (Chrome extension is disabled here; Playwright is used.)
- Every server change and its exact undo: `BOX-CHANGELOG.md`. Teardown script at the foot of that file
  returns the box to baseline.
- Background refresh log on the box: `/opt/auth-helper/data/measurements.log`.
