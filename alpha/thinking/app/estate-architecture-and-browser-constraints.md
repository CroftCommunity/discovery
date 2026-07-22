# Croft estate architecture — origin topology, storage, and auth under browser reality

date: 2026-07-22 · status: **decided (topology + storage baseline); auth mechanism has deferrable open
tests.** Grounded in the account-kernel spike (K1 + KC0), not assumption. Provenance:
`../../spike/account-kernel/FINDINGS-AND-PIVOT.md`, `.../KC0-RESULTS.md`, `.../K1-SPIKE-RESULTS.md`; plan
`../../plans/2026-07-22-account-kernel-spike.md`; thread `../../../beta/OPEN-THREADS.md` T55. Extends the
client ADR (`client-architecture-adr.md`, the shared-core/per-shell axis) on a new axis: how the *estate*
of apps shares (or does not share) storage and session in the browser.

## The governing constraint (measured, not assumed)

Browser storage isolation differs materially per engine, and it decides the whole shape:

- **Same-origin storage is shared on every engine.** Two pages on the *same origin* always see the same
  IndexedDB / OPFS, and a same-origin SharedWorker coordinates them — confirmed on Chromium **and WebKit**
  (KC0).
- **Cross-subdomain shared storage FAILS on WebKit/iOS.** A `kernel.croft.ing` iframe embedded across
  `*.croft.ing` subdomains is shared on Chromium but **partitioned on WebKit/Safari** — each subdomain gets
  its own store (K1, confirmed in shipping Safari on real `croft.ing`, refuting the `.localhost`-artifact
  escape). Because every iOS browser is WebKit, this is an iOS constraint, not a quirk.

**Lesson (standing):** cross-origin / partitioning behavior is browser *policy* and must be tested on real
Safari before it is relied on; same-origin behavior is spec-guaranteed and portable. Chromium and WebKit
gave *opposite* answers to the same cross-subdomain question.

## Origin topology (decided)

The estate is not one thing. Apps fall into three buckets by how much they can be trusted to share a
security context, and the bucket sets the origin:

```
┌─ SHARED ORIGIN (path-based)  e.g. croft.ing/greet-lite, /games, /pdsview ─────────┐
│  First-party, LOW untrusted-input pads.                                            │
│  Get the shared kernel for free: same-origin IndexedDB store + SharedWorker        │
│  coordinator + ONE session. Works on every engine (KC0). SSO among them is free.   │
└────────────────────────────────────────────────────────────────────────────────────┘
┌─ ISOLATED SUBDOMAINS  greetings.croft.ing, skylite.croft.ing ─────────────────────┐
│  First-party CODE, but HIGH untrusted-DATA input (skylite renders others' posts;   │
│  greetings handles card content) OR their own security posture (skylite's crypto). │
│  Own origin = blast-radius containment: an XSS rendering bug can't reach another    │
│  app's session/storage. A defense-in-depth CHOICE, not forced.                     │
└────────────────────────────────────────────────────────────────────────────────────┘
┌─ SEPARATE REGISTRABLE DOMAIN  (untrusted-CODE sandbox) ───────────────────────────┐
│  Runs a STRANGER'S code: the "open any card" renderer (.xdc / arbitrary card       │
│  HTML+JS), wrapped/community pads. MUST be off-origin, opaque, strict CSP, no       │
│  network, postMessage-only. Never shares the estate session. (The domain-           │
│  separation decision, T55, now load-bearing.)                                       │
└────────────────────────────────────────────────────────────────────────────────────┘
        │
        ▼  data authority for all of them
   THE USER'S PDS  — cross-device / cross-app coherence is eventual via PDS sync
                     (realtime only where a pad needs it; latency = KC3, open).
```

### The distinction that assigns the bucket: untrusted DATA vs untrusted CODE
- **Untrusted DATA** (skylite showing a stranger's post; a card's text/image) — handled safely in
  first-party code with escaping + CSP. Does *not* force a separate origin; isolating anyway is a
  defense-in-depth choice.
- **Untrusted CODE** (a stranger's app bundle actually executing) — *must* be a separate registrable
  domain, sandboxed. This is the only case that forces off-origin.

## Storage backend (decided)

**IndexedDB is the cross-browser baseline** (shared same-origin on WebKit per KC0). **OPFS is a
Chromium-confirmed optimization**, used where available; the `repo-mirror` and outbox must not hard-depend
on it. (OPFS write errored under Playwright-WebKit on both single- and cross-origin — a tooling limitation,
not a Safari-OPFS verdict; shipping-Safari OPFS is an optional confirm, KC2, non-blocking.)

## Coordination (decided)

Within one origin: a **SharedWorker** owns the single-writer store + fans out changes (works on WebKit per
KC0); **Web-Locks + BroadcastChannel** are the same-origin fallback. No cross-subdomain channel is used —
there isn't a reliable one on WebKit, and the topology avoids needing one.

## Auth / SSO — the options and their browser support

The atproto session is **DPoP-bound** (every request proves possession of a per-client key). That, plus the
storage constraint, sets what is possible:

| Approach | How | Chrome | Safari/iOS | Server? |
|---|---|---|---|---|
| **Per-app login** (default) | each isolated app runs its own atproto OAuth | ✓ | ✓ | no |
| SSO among co-located pads | same shared origin = one session, free | ✓ | ✓ | no |
| **Storage "mock" SSO** | `account.croft.ing` iframe shares session via cross-subdomain storage | ✓ | **✗ (K1)** | no |
| **Cookie + BFF SSO** | small server holds session + DPoP key; same-site `Domain=croft.ing` cookie authenticates each app | ✓ | ✓ (untested, KC1) | **yes (small)** |

Key points:
- **The BFF is optional, never required.** Everything functions on **per-app login** (the serverless
  default — what the live pads already do). SSO is a UX/security add-on, and **starting per-app login
  forecloses nothing** — a BFF can be added later.
- **SSO is free among pads that share the single origin.** You only "pay" for SSO to the *deliberately
  isolated* subdomains (greetings, skylite).
- **A serverless / storage-based ("mock") cross-subdomain SSO is Chrome-only** — it rides the exact shared
  storage WebKit partitions (K1). Do not ship it as the estate SSO; it silently fails on iOS.
- **Cross-browser SSO needs a small server (BFF).** Cookies *do* cross subdomains on Safari (they are not
  storage-partitioned), but a cookie is a bearer token and cannot carry a DPoP-bound session by itself; the
  DPoP key must live somewhere non-partitioned, i.e. a server. So the BFF (an atproto OAuth *confidential
  client* holding the session + DPoP key, apps authenticating to it by cookie) is the mechanism that makes
  sign-in-once work on iOS. Public reads (`getRecord`) stay direct/serverless; only the authenticated path
  goes through it.

## What is decided vs still open

**Decided:** the three-bucket origin topology; single shared origin for co-located low-risk pads;
untrusted-CODE off-origin sandbox; IndexedDB storage baseline (OPFS as optimization); SharedWorker
coordination; per-app login as the serverless default with the BFF as an optional later SSO/key-custody
upgrade; the PDS as data authority.

**Open (deferrable, none blocks shipping per-app-login pads today):**
- **KC1** — does the same-site cookie → BFF handshake work cross-subdomain on shipping Safari? (gates the
  BFF SSO path)
- **KC2** — OPFS on shipping Safari (non-blocking; IndexedDB is the baseline)
- **KC3** — PDS-as-coordinator propagation latency: is eventual-via-PDS acceptable, or is a realtime relay
  needed for any pad?
- **atproto OAuth confidential-client / BFF token mechanics** — verify against the atproto OAuth spec
  before building the BFF (do not assume the DPoP/handoff details).

## Consequences for existing plans

- Supersedes the shared-subdomain-origin "account kernel" (T55): the shared kernel is real, but only
  **same-origin**, not cross-subdomain.
- The live pads (arecipe, skylite, greetings, pdsview) already fit: each is a self-contained per-app-login
  atproto client. Co-locating the low-risk ones onto a shared origin is an optional future consolidation,
  not a required migration.
- Roadmap (`../../../beta/croft/build-order-and-ponds-roadmap.md`) kernel entry reflects this pivot.
