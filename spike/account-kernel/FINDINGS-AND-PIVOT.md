# account-kernel K1 — findings and why we pivoted

date: 2026-07-22 · status: **concluded**. Companion data log: `K1-SPIKE-RESULTS.md`. Plan:
`../../alpha/plans/2026-07-22-account-kernel-spike.md`. Thread: `../../beta/OPEN-THREADS.md` T55.

This is the durable record of *why* the account-kernel design pivoted away from a shared browser origin.
Read it before touching the coordinator-model re-scope, so the reasoning is not re-litigated or forgotten.

## The proposition that was on the table

The account-kernel design (raw:
`../../alpha/seeds/transcripts/raw/croft-encrypted-prefs-repo-mirror-and-account-kernel-2026-07-22.md`)
was: put one shared origin, `kernel.croft.ing`, inside every `*.croft.ing` app as an embedded iframe, and
have it hold the whole estate's client state — session, encrypted-prefs cache, a CID-verified PDS
`repo-mirror`, and a write outbox. Every app becomes a **stateless skin** over one signed-in, one-mirror,
one-outbox estate.

The entire design rests on one assumption: **that the `kernel.croft.ing` iframe sees the same storage no
matter which `*.croft.ing` subdomain embeds it**, because they are the same registrable site (`croft.ing`).
That is hypothesis **H1**, and it was flagged from the start as the make-or-break.

## What we tested (and why it is trustworthy)

A throwaway harness (this directory): a kernel iframe that reads/writes both **IndexedDB** and **OPFS** over
a postMessage probe, embedded by two "skin" pages on two different subdomains. Skin A writes a unique
nonce through the kernel iframe; skin B (a different subdomain, same browser profile) reads through its own
kernel iframe. If B sees A's nonce, the storage is shared. Three controls guarded against false readings:
a same-page round-trip (proves the probe works), and a fresh-profile read (proves a "shared" reading is not
stale data). We ran it on **two substrates** (`*.localhost` and real `*.croft.ing` on GitHub Pages over
HTTPS) and **three engines** (Chromium via Playwright, WebKit via Playwright, and **shipping Safari on a
Mac**).

## Results

| Engine | `.localhost` | real `*.croft.ing` | Shares same-site subdomain storage? |
|---|---|---|---|
| Chromium 140 | PASS | **PASS** | **Yes** — B reads A's nonce (IndexedDB + OPFS) |
| WebKit 26.5 (Playwright) | — | **FAIL (IndexedDB)** | **No** — B reads null; A self-read OK (OPFS tooling-errored, inconclusive) |
| Shipping Safari (Mac) | FAIL | **FAIL** | **No** — B's kernel "sees EMPTY storage" |

Consistent across every WebKit-family data point, on the real registrable domain, on the exact engine users
run. Chromium shares; WebKit/Safari does not.

## The mechanism

Chromium treats a `kernel.croft.ing` iframe embedded under `app-a.croft.ing` as a **first-party, same-site**
frame (top-level site and frame site are both `croft.ing`), so its storage is unpartitioned and shared
across every `*.croft.ing` top-level. WebKit/Safari partitions storage **per top-level site more
aggressively**: the embedded kernel iframe under `app-a` gets a different storage partition than the same
iframe under `app-b`, so each subdomain's kernel has its own, isolated store. (The self-read succeeds, so
the write persisted; it simply is not visible from a different top-level subdomain.)

We deliberately checked the tempting escape hatch — that this was a `.localhost` artifact (loopback hosts
are not normal registrable domains). It is not: WebKit partitions the same way on real `croft.ing`
subdomains. Hypothesis refuted.

## Why this forces a pivot

**Every browser on iOS is WebKit** (Apple requires it; Chrome/Firefox on iOS are WebKit shells). So a
substrate that only shares storage on Chromium cannot be the estate's client substrate for a product that
must run on iPhones. The shared-origin kernel — one `kernel.croft.ing` iframe as the single store/session
/outbox for the estate — **does not work on iOS.** It is not a bug we can patch; it is the platform's
storage-partitioning policy.

This is exactly what K1 existed to find, and it found it for the price of a throwaway spike — before any
pad was built against the shared-origin assumption.

## The lesson (worth keeping)

Browser storage/isolation semantics **differ materially per engine** and cannot be reasoned to from specs
alone — Chromium and WebKit gave opposite answers to the same question. Any future design that leans on a
cross-origin/cross-subdomain browser behavior must be **tested on WebKit (real Safari, real domain) before
it is committed**, not assumed. Same-origin behavior is portable; cross-origin/partitioning behavior is not.

## The pivot

Drop the shared blockstore. The kernel becomes, at most, a **shared client library each pad embeds
(per-app storage instances)** plus a **coordinator that brokers cross-pad concerns through channels that
actually work under partitioning** — with the **PDS as the cross-pad sync authority** rather than a shared
browser origin.

- **Preserved:** the code-reuse win (one `repo-mirror` / outbox / session library, instantiated per pad);
  the PDS-as-sync-authority model (already how the live pads work); the proven data algorithm (`hist_live`
  + the encrypted-local-first Proofs).
- **Lost:** the single shared runtime store, the one-runtime-session across pads, and "stateless skins over
  one origin." Session unification degrades from "one shared session" to at best "one shared login library"
  (and possibly per-pad logins — untested on WebKit; see the re-scoped series).

What is now *possible*, *impossible*, and *untested* on this substrate — and how to do the possible things
safely and reliably — is worked out in the capability-boundary analysis and the re-scoped coordinator-model
experiment series (`../../alpha/plans/2026-07-22-account-kernel-spike.md`, re-scoped section). Several
"possible" claims are still **untested on WebKit** and must be run there before they are relied on — the
K1 lesson, applied.

## Reproduce

Test beds kept live (zero cost): `kernel-k1` / `k1-appa` / `k1-appb`.croft.ing (GitHub Pages). Runner:
`run-k1-playwright.cjs` (env `PW_BROWSER` chromium|webkit, `PW_EXEC` for a chromium build, `APPA_URL`/
`APPB_URL`). WebKit build: `node <playwright>/cli.js install webkit`. See `K1-SPIKE-RESULTS.md` for exact
invocations.
