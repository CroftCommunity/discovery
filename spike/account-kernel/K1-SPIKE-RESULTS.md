# K1 results — cross-subdomain unpartitioned storage

Hypothesis H1 (the account-kernel make-or-break): a `kernel.<site>` iframe embedded under two different
same-site subdomains (`app-a`, `app-b`) shares one unpartitioned OPFS + IndexedDB. Plan:
`../../alpha/plans/2026-07-22-account-kernel-spike.md`.

Verdict shape per run: **PASS** (app-b reads app-a's nonce) / **FAIL** (partitioned) / **BLOCKED** (could
not run). Record IndexedDB and OPFS separately (they can partition differently).

## Runs

| Date | Env | Browser | IndexedDB | OPFS | Verdict | Notes |
|------|-----|---------|-----------|------|---------|-------|
| 2026-07-22 | local `*.localhost` | **Chromium 140** (Playwright) | **shared ✓** | **shared ✓** | **H1 PASS** | Automated via `run-k1-playwright.cjs` (local Chromium, no login). app-a wrote a nonce through the `kernel.localhost` iframe; app-b (different subdomain, same browser context) read the same nonce back via its own `kernel.localhost` iframe, for BOTH IndexedDB and OPFS. Controls held: same-page round-trip works; a fresh context reads `null` (no false positive). Necessary-not-sufficient — one engine, `.localhost` not a real registrable site (see caveat). |
| 2026-07-22 | local `*.localhost` | **Safari** | **not shared (null)** | **not shared (null)** | **H1 FAIL** | app-a wrote a nonce (`idb/opfs: written`); app-b's `kernel.localhost` iframe read `{idb:null, opfs:null}` — did NOT see app-a's write. **The OPPOSITE of Chromium: engine split.** Interpretation **CONFIRMED**: app-a self-read returns its own nonce (write persisted), app-b reads null, so Safari gives each top-level subdomain its own partition of the embedded kernel iframe (not eviction). **Decisive open caveat:** this may be a `.localhost` artifact — Safari may treat each `*.localhost` host as its own site (localhost is not a normal registrable domain), whereas on real `*.croft.ing` the subdomains share the registrable domain `croft.ing` and could behave differently. Cannot tell from `.localhost` alone, so the real-`*.croft.ing` Safari run is the decisive test. **WebKit == every iOS browser, so a real-Safari FAIL == an iOS FAIL.** |
| 2026-07-22 | **real `*.croft.ing`** | **Chromium 140** (Playwright) | **shared ✓** | **shared ✓** | **H1 PASS** | Live sites `kernel-k1` / `k1-appa` / `k1-appb`.croft.ing (GitHub Pages, HTTPS). app-a wrote a nonce, app-b (different same-site subdomain) read it back; controls held. The real-domain positive control: Chromium shares on a true registrable site, matching local. |
| 2026-07-22 | **real `*.croft.ing`** | **WebKit 26.5** (Playwright, Safari engine) | **not shared (null)** | tooling-errored (inconclusive) | **H1 FAIL (IndexedDB)** | app-a reads its OWN IndexedDB write (round-trip ✓), app-b reads `null` → **not shared → partitioned**. So WebKit partitions same-site subdomain storage **on the real registrable domain too** — the `.localhost`-artifact hypothesis is REFUTED. OPFS threw a Playwright-WebKit `UnknownError` (a tooling limitation, not a Safari signal) so OPFS is inconclusive here, but IndexedDB partitioning alone breaks the shared-origin model. NB: Playwright-WebKit approximates but is not shipping Safari; a real-Safari run on these URLs would close it 100% (expected FAIL, consistent with the earlier local-Safari FAIL). |

## Conclusion (2026-07-22)

**H1 is engine-split, and the split holds on real domains: Chromium shares same-site subdomain storage;
WebKit/Safari partitions it.** Evidence: Chromium PASS on both `.localhost` and real `croft.ing`; Safari
FAIL on `.localhost` (shipping Safari, user-run) and WebKit 26.5 FAIL (IndexedDB) on real `croft.ing`
(Playwright). Because WebKit is every iOS browser, the **shared-origin account-kernel (one `kernel.croft.ing`
iframe seen identically across `*.croft.ing` apps) does not work on iOS**, so it cannot be the estate
substrate for a client that must run on iPhones.

**Implication (the design pivot, T55):** drop the shared-store kernel; move to the fallback the plan
anticipated — **per-app storage plus a postMessage sync-coordinator** (the kernel becomes a message broker
that relays record changes / brokers a single writer, not a shared blockstore). K2-K6 as written (which
assumed the shared store) are superseded by this pivot; re-scope them against the coordinator model.

**100% closer — CONFIRMED 2026-07-22:** shipping Safari on a Mac, on the live `https://k1-appa.croft.ing/`
(Write) → `https://k1-appb.croft.ing/` (Read), returned the same **H1 FAIL** ("this subdomain's kernel sees
EMPTY storage"). So the result is confirmed on the exact engine users run, on the real registrable domain.
No caveat remains: **the shared-origin kernel does not work on Safari/iOS.**

**Test beds retained:** `kernel-k1` / `k1-appa` / `k1-appb`.croft.ing (GitHub Pages) are kept live as
zero-cost test beds for the re-scoped coordinator-model experiments; delete when the work concludes.

## How the 2026-07-22 run was invoked (reproduce)

Playwright's bundled-browser version did not match an installed build, so the run used an installed
Chromium via `executablePath` (the version-build check is bypassed):

```
python3 serve.py &                       # static harness on 127.0.0.1:8080
NODE_PATH="$HOME/.npm/_npx/e41f203b7505f1fb/node_modules" \
PW_EXEC="$HOME/Library/Caches/ms-playwright/chromium-1193/chrome-mac/Chromium.app/Contents/MacOS/Chromium" \
  node run-k1-playwright.cjs
```

(Node Playwright 1.61.1 from the npx cache; Chromium 140 full build 1193, launched headless.) The `.cjs`
reads `PW_EXEC` for the executable and falls back to Playwright's own download if unset.

## Interpretation guide

- **PASS on real `*.croft.ing`** → the shared-origin kernel model holds; proceed to K2 (domain separation)
  and K3 (single-writer).
- **FAIL on real `*.croft.ing`** → same-site subdomain embeds are partitioned; the one-mirror/one-outbox
  model fails as drawn. Fall back to the G0 alternative (per-app storage + a postMessage sync coordinator),
  and record the finding in T55.
- **PASS local but the real run still pending** → do not fold; `.localhost` may be special-cased.
