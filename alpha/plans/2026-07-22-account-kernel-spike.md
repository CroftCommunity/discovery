# The account.croft.ing kernel — experiment-series scope (does the shared-origin proposition suit the need?)

date: 2026-07-22 · status: **K1 done (shared-origin FAILS on WebKit/iOS) → re-scoped to decide the
architecture.** See `FINDINGS-AND-PIVOT.md`. The original K1-K6 (below, kept for provenance) assumed a
shared subdomain origin; the re-scoped series that actually drives the decision is at the top, next.

## Re-scoped series (post-K1): decide the architecture with best information

**The fork K1 forces.** Cross-*subdomain* (cross-origin, same-site) storage is partitioned on WebKit/iOS,
so a shared kernel origin embedded across `*.croft.ing` subdomains is out. Two viable architectures remain:

- **ARCH-1 Single origin (path-based):** first-party pads live on paths of ONE origin
  (`croft.ing/greetings`, `/games`, …). Same-origin storage/session/SharedWorker are shared on every
  browser (K1's failure was cross-subdomain; single origin removes it). SSO is free. Cost: no isolation
  between first-party pads (untrusted pads MUST go off-origin, sandboxed); one-origin deployment.
- **ARCH-2 Subdomains + BFF:** keep per-subdomain pads (isolated). SSO via a small BFF at
  `account.croft.ing` holding the durable session + a same-site `Domain=croft.ing` cookie issuing
  short-lived tokens. Per-pad storage; cross-pad coherence via the PDS. Cost: a small server + secret.

The experiments below produce the data to choose. Each: what it proves, which ARCH it informs, how it runs,
and its current runnability. Runner: `run-kc*-playwright.cjs` (Playwright: Chromium via `PW_EXEC`, WebKit
via the installed build; test beds `kernel-k1`/`k1-appa`/`k1-appb`.croft.ing kept live).

| # | Question it answers | Informs | Runs how | Runnable now? |
|---|---|---|---|---|
| **KC0** | On ONE origin, do two same-origin top-level pages share IndexedDB + OPFS, and does a same-origin **SharedWorker** coordinate them, on **WebKit**? (validates ARCH-1's premise + doubles as an OPFS-on-WebKit signal) | ARCH-1 | two same-origin pages, write in A / read in B; + SharedWorker set/get; Chromium + WebKit | **YES** (Playwright, local + test bed) |
| **KC2** | Does the browser **repo-mirror** work on WebKit — OPFS blockstore (the Playwright-WebKit `UnknownError` was inconclusive) with an **IndexedDB blockstore fallback**, plus atcute CAR-parse + in-browser CID/sig verify against a recorded `.car` fixture? | both | load a fixture CAR, verify + store, on Chromium + WebKit; shipping-Safari OPFS confirm | **PARTIAL** (needs a `.car` fixture + atcute bundle; OPFS-vs-IDB testable now) |
| **KC1** | Does a same-site `Domain=croft.ing` **cookie → BFF** SSO actually work cross-subdomain on WebKit (ITP)? | ARCH-2 | a tiny BFF sets a cookie; a `*.croft.ing` page fetches it with credentials; check token issued | **NO** (needs a server; GH Pages is static) — spec ready; run on a BFF deploy or shipping Safari |
| **KC3** | PDS-as-coordinator: latency/UX of app-a write → app-b sees it via PDS sync (`getRepo since=rev` / firehose). Is eventual-via-PDS acceptable, or is a realtime relay needed? | both | write a record on the test atproto account, measure propagation to a second client | **NO** (needs a live PDS + the test account) — spec ready |
| **KC4** | Single-writer WITHIN one origin (multi-tab, same origin) via SharedWorker/Web-Locks on WebKit | both (folds into KC0) | two same-origin tabs contend for one OPFS handle; leader election | **YES** (folds into KC0) |

Decision logic: **KC0 PASS → ARCH-1 is viable and is the recommendation** (kernel works everywhere, SSO
free), pending KC2 confirming the mirror's storage backend (OPFS or IDB fallback). **KC0 SharedWorker FAIL
on WebKit → the coordinator uses Web-Locks + BroadcastChannel instead (still same-origin).** ARCH-2 is the
fallback if single-origin deployment is unacceptable; then KC1 (cookie→BFF) becomes the gating auth test.
KC3 informs realtime-vs-eventual in either ARCH.

**KC0 RESULT (2026-07-22) — DONE, ARCH-1 validated.** `KC0-RESULTS.md`. On one origin, two same-origin
tabs share **IndexedDB** and a same-origin **SharedWorker coordinates** them on **WebKit 26.5** (and
Chromium, which also shares OPFS). OPFS write errored under Playwright-WebKit on both single- and
cross-origin → a tooling limitation, not a Safari signal; use **IndexedDB as the cross-browser blockstore
baseline**, OPFS as a Chromium optimization. So the shared kernel works on every engine **if the
first-party estate is one origin**. **Recommendation: ARCH-1** (single origin, path-based; SSO free; no BFF
for auth; untrusted pads off-origin sandboxed). KC1 (cookie->BFF) is moot under ARCH-1. Remaining: KC2 OPFS
on shipping Safari (non-blocking, IDB baseline), KC3 PDS-coordinator latency (needs the test account).

---

## Original series (K1-K6) — superseded by the re-scope above (kept for provenance)

date: 2026-07-22 (original). Expanded from a single spike into an ordered series after a corpus-wide
fit/impact pass. K1 was run and CONCLUDED (FAIL on WebKit/iOS); K2-K6 assumed the now-refuted shared store.

Spike home (when built): `discovery/spike/account-kernel/` (sibling to `spike/hist_live/`).
Thread: `../../beta/OPEN-THREADS.md` T55 · backlog: `../ROADMAP_TODO.md` E44 · raw:
`../seeds/transcripts/raw/croft-encrypted-prefs-repo-mirror-and-account-kernel-2026-07-22.md`.
Roadmap placement: `../../beta/croft/build-order-and-ponds-roadmap.md` (the A→B bridge).

## Problem Statement

The dialogue proposes a single shared client-side origin, `account.croft.ing`, reached by every
`*.croft.ing` app through an embedded iframe, holding the whole estate's client state: session broker,
encrypted-prefs cache, a CID-verified `repo-mirror` of the user's PDS, and a write outbox. If it holds,
every Croft app becomes a **stateless skin** over one signed-in, one-mirror, one-outbox estate. That is a
strong proposition, and it is the thinnest-covered / most-load-bearing net-new idea in the 2026-07-22
batch. But it rests on several **browser-platform assumptions that are not obviously true** (cross-subdomain
unpartitioned storage, single-writer OPFS across tabs, in-browser CID verification), and getting it wrong
is expensive: it becomes a migration once three apps depend on it. So we prove it by spike before writing
it up as settled beta.

The concrete near-term need it must suit: **unify the already-live Track A pads** (arecipe, skylite,
pdsview, greetings — all separate `*.croft.ing`/product origins today) into one signed-in, one-mirror
estate, without each pad running its own sync loop, quota, and idea of "current."

## Reasoning

- **Why a spike, not a design doc.** The value is real but conditional on browser behavior we cannot
  settle by reasoning: state partitioning is keyed on the registrable site and is actively tightening
  under tracker mitigation; SharedWorker availability and OPFS single-writer semantics differ across
  engines. A 200-line spike answers "does it work" faster and more honestly than more prose.
- **Why now, not after Track B.** The kernel's first payoff is Track A (live pads), which exists today, so
  it does not wait on the unbuilt resolver. Proving it now de-risks the largest new idea while it is cheap.
- **Sandbox constraint (carried).** Live atproto over the public network does not run in this environment
  (browser egress is blocked — see the [[sandbox-browser-egress-blocks-live-tests]] note). So the spike is
  designed to prove the load-bearing mechanics **hermetically** (local origins + recorded CAR fixtures),
  and to hand the one real-domain/real-PDS leg to a credentialed environment.

## Corpus-pass synthesis (2026-07-22)

A four-way read-only pass mapped how the kernel fits, its impact on the live pads, what is already proven,
and its security/protocol touch. The findings reshape the scope.

### Fit — a new axis, cleanest as an adapter behind the port

The kernel **extends** the corpus's "don't duplicate across surfaces" ethos to a dimension the
functional-core / per-platform-shell ADR never covered: client **state / session / sync** (the ADR removes
duplicated *logic*; the kernel removes duplicated *state*). The cleanest way it fits the settled
architecture is as a **storage/PDS adapter behind the existing I/O port** (`effects.rs`), so the pure core
never sees it. Its most novel security move (untrusted renderer on a separate registrable domain) is
**already settled design intent** — the honest-seams "the seam that is a UX boundary is also a security
boundary" rule in `../../beta/croft/product-the-garden-of-ponds.md`.

Two real tensions to respect, not resolve by fiat:
- **DECISION 1 (the pure core holds no storage/state).** The kernel is a stateful async actor; it is safe
  only if it lives strictly shell-side of the port. The erosion risk is skins treating the kernel as *the*
  model (reading/writing it directly) and bypassing core intents/view-models. The series must keep the
  kernel behind the port.
- **Honest-seams / per-pond native state (option C).** "One-mirror, one-outbox estate" is coherent for the
  **single-user atproto estate (Track A)** but conflicts with the no-fused-model thesis if generalized
  across ponds (fediverse, iroh groups). **Scope the kernel to Track A; do not generalize into a
  cross-pond fused store.**

**The kernel is web-only.** iframe / SharedWorker / OPFS / BroadcastChannel / registrable-site partitioning
have no native-shell analogue; the native/Tauri shells own platform-native stores (Keychain / Keystore /
Secret Service). The corpus implies this (Track A = hosted PWAs) but never states it, and it creates a
behavior-drift-vs-native gap against the ADR's "no behavior drift across platforms" promise. **Name it
explicitly: the kernel is a Track-A / PWA primitive; native shells keep their own stores.**

### Impact — the pads are siloed, the win is session unification, and arecipe is off-site

The four live pads are fully siloed (four origins, no shared state) — the redundancy the kernel removes is
real. But the pass corrected two premises:
- **The clearest concrete win is session-broker unification.** The OAuth/DPoP session pattern is
  implemented **three times**: arecipe and greetings both ship `@atproto/oauth-client-browser` (greetings'
  own comments say it copied arecipe); skylite hand-rolls a *third, independent* PKCE/PAR/DPoP stack. The
  repo-mirror is duplicated only once; a real write-outbox exists **nowhere** (all four write direct) — so
  the outbox is a net-new capability, not a consolidation.
- **arecipe is on `arecipe.app`, a different registrable domain** — so H1 shared storage **cannot** cover
  it; it is a postMessage-only (H6) consumer unless it moves to a `croft.ing` subdomain. The pad named as
  the kernel's first justification is the one structurally excluded from the shared-storage model. This is
  a **topology decision** the series must open with (see Experiment K0).
- **First adopters:** **pdsview** is the best first *mirror* home — but as a **blank slate** (zero-dep,
  zero-storage, same registrable site), not because it already mirrors (its "export .car" is a plain
  download link; it never parses/verifies a CAR). **arecipe's `recipes/cache.ts`** is the actual in-browser
  CID-verification *precedent* (recompute CID from record bytes via `@ipld/dag-cbor` + `multiformats`),
  though per-record not full-CAR, and off-site. **greetings** is the best first *write/session* adopter
  (same-site, minimal state, library session ready to hand off). **skylite is the hardest conflict** — its
  "on this device only" invariant, per-tab ephemeral sessions, dual sponsor/explorer identity, and bespoke
  non-`@atproto` OAuth make it philosophically and mechanically the worst early target.

### What is already PROVEN (do not re-prove)

The kernel's data algorithm is settled ground — proven in Rust against a live PDS (`spike/hist_live/`) and
in the `Proofs/encrypted-local-first-atproto/` slices. The series must **not** re-litigate:
1. CID = byte-head, zero re-hash (the `canonical.rs` two rules: map-key length-then-lex ordering;
   `$bytes`/`$link` handling). Live PDS, `hist_live` E1/OC-2.
2. CAR re-hydrates byte-for-byte; commit signature verifies against the DID doc; every block CID + MST
   reachability verifies. `hist_live` E3 / E-MST.
3. Incremental sync primitives: `?since=<rev>` delta CAR, `getBlocks` CID batch, no server rkey-range
   filter (enumerate client-side), snapshot cursor + firehose catch-up. `hist_live` E-Since/E-Range.
4. Delivery order ≠ fold order (`fold_by_antecedent_hashes` + `strict_fold`).
5. AEAD-encrypted content is a valid atproto lexicon record and publishes without reshaping, with provable
   non-leakage. Proofs H1/H2/H4.
6. Write mechanics: `applyWrites`, rkey-pinning, CID parity, idempotent create, live rate-limits + 1 MB cap.
   Proofs H5/H8, `hist_live` E8.
7. MLS-in-WASM feasibility + browser-storage-is-evictable + MLS-state-is-single-writer. Sealed-tier RUN-19.

**The genuinely open risk is entirely browser realization + shared-origin topology + concurrency + the
security boundary.** Everything the series spends budget on is one of those.

### Security / protocol placement

The kernel is a **utility-plane** construct (a rebuildable cache + an outbox) with a **thin authority
touch** (it holds the session/write-signing material and a prefs symmetric key). It does **not** originate
signed history (provenance stays in the PDS), and it does **not** hold MLS / sealed group state or touch
the social-graph substrate — those are Track B, deferred. Top risks the series must address: (1) the
domain-separation boundary must be *proven*, not assumed (H6, and its interaction with H1); (2) the kernel
is the estate's single highest-value compromise target, so XSS + supply-chain of the one bundle can
traverse everything (the "keep it tiny, dependency-light, most-reviewed" claim is a security control to
operationalize); (3) browser key custody has no recovery path built (I9 open), and if MLS/sealed state ever
enters the kernel the blast radius flips from "reset prefs" to unrecoverable E2EE corruption — so the
sealed-tier boundary is a decision gate before graduation.

## Approach

A minimal multi-origin browser spike. Two throwaway "skin" pages on different local subdomains, one kernel
iframe origin, exercising the real mechanics against recorded fixtures. Build only what tests a hypothesis;
delete the rest. The synthesis above turns this into an **ordered series**: two decision gates open it, then
experiments sequenced by risk and dependency, reusing the proven Rust algorithm rather than re-proving it.

### Decision gates (resolve before / early in the series — the user's calls)

- **G0 — Estate topology. RESOLVED 2026-07-22 (user):** the kernel is **`*.croft.ing` same-site only** —
  the pads it unifies live under `croft.ing` so H1 shared storage covers them. **Off-site pads (arecipe on
  `arecipe.app`) are "a whole other deal" and get separate handling** (postMessage service at most, likely
  excluded from the shared-storage estate for now). The on-site/off-site line is treated as a meaningful
  architectural boundary in its own right — part of the reasoning about what belongs in the estate (the
  "split good and bad"), to be thought through separately, not forced into the shared-storage model. So the
  series targets `*.croft.ing` subdomains (pdsview, skylite, greetings), not arecipe.app.
- **G1 — Sealed-tier boundary.** Will MLS / sealed group state ever live in the kernel, or is the kernel
  permanently public-tier (public records + prefs ciphertext) with sealed state kept elsewhere? The answer
  sets the blast radius and must be fixed before graduation, not after three pads depend on it.

### Experiment series (ordered by risk and dependency)

Each experiment states what it proves, what it reuses (so it does not re-prove settled ground), and its
env. IDs are the ROADMAP_TODO backlog handles once opened.

1. **K1 — Cross-subdomain unpartitioned storage (H1, make-or-break).** A `kernel.croft.test` iframe
   embedded in `app-a.croft.test` + `app-b.croft.test`: do they share one OPFS/IndexedDB under state
   partitioning? Hermetic first-pass (local TLS subdomains), real-domain confirmation on `*.croft.ing`
   (credentialed env). **Kill gate:** if partitioned, the shared-origin model fails as drawn (fall back to
   G0's postMessage-service topology). Test first; nothing else matters if this fails.
   **DONE — CONCLUDED 2026-07-22: FAIL on WebKit/iOS (engine split holds on real domains).** Chromium: H1
   PASS on both `.localhost` and real `croft.ing` (live Pages `kernel-k1`/`k1-appa`/`k1-appb`.croft.ing).
   WebKit/Safari: FAIL — shipping Safari on `.localhost` (app-a self-read OK, app-b null) and WebKit 26.5 on
   real `croft.ing` (IndexedDB: app-a self-read OK, app-b null; OPFS tooling-errored in Playwright-WebKit,
   inconclusive but IndexedDB partitioning alone suffices). The `.localhost`-artifact hypothesis is
   **refuted** — WebKit partitions on a real registrable site too. Since WebKit == every iOS browser, the
   shared-store kernel is out for iPhones. Full evidence: `spike/account-kernel/K1-SPIKE-RESULTS.md`.
   **PIVOT:** K2-K6 (which assumed the shared store) are superseded; re-scope against a per-app-storage +
   postMessage sync-coordinator model (kernel = broker, not shared blockstore). Optional 100% closer:
   shipping-Safari on the live URLs (expected FAIL).
2. **K2 — Domain-separation boundary (H6, pairs with K1).** An untrusted renderer on a *separate
   registrable domain* cannot read the kernel's storage/session; the only channel is a kernel-controlled
   postMessage. Same harness as K1. Security decision gate (T55 Gate 1).
3. **K3 — Single-writer across tabs (H2).** One origin-scoped SharedWorker (Web-Locks leader-election
   fallback) owns the OPFS blockstore + one sync loop, no access-handle contention. This is the exact
   RUN-19-named-but-unbuilt hazard; shared by the mirror *and* the outbox. Hermetic.
4. **K4 — Browser realization of the proven mirror (H5-mirror).** Port the `hist_live` algorithm to the
   browser via `atcute`: CAR-parse + in-browser CID + commit-signature verify + MST reachability + an OPFS
   CID-keyed blockstore, against a **recorded `.car` fixture**. **Reuse, do not re-prove:** the algorithm's
   correctness (proven in Rust) and arecipe's `recipes/cache.ts` per-record CID precedent. The one new
   sub-check: does `atcute`'s encoder reproduce `canonical.rs`'s two rules (map-key ordering; `$bytes`/
   `$link`)? If not, the browser mirror re-hashes. Hermetic (fixture); live getRepo leg → credentialed env.
5. **K5 — Session broker + prefs round-trip + fan-out (H3/H4/H5-prefs).** postMessage RPC for record reads
   + transferable-ArrayBuffer blobs (H3), BroadcastChannel change fan-out to two skins (H4), and the
   `ing.croft.account.prefs` ciphertext encrypt → write → read-back-decrypt round-trip (H5-prefs), with one
   shared session handed to both skins. This is the concrete "unify the 3x session pattern" win. Hermetic
   with a fixture session; the live OAuth/DPoP leg → credentialed env.
6. **K6 — Write outbox durability.** Queue in OPFS, single-writer submit (reuses K3's lock), replay on
   reconnect, respecting the proven live constraints (429 policy, 1 MB cap, conflict rejection from
   `hist_live` E8). Net-new (no outbox exists anywhere today). Live leg → credentialed env.

Sequencing: K1 (+K2, same harness) gate the whole thing; K3 unblocks both K4 and K6 (shared lock); K4/K5/K6
can then proceed in parallel. K1 and the live legs of K4/K5/K6 need the credentialed env; the rest is
hermetic here.

### Hypotheses (the load-bearing claims each experiment tests, each pass/fail)

- **H1 — Shared unpartitioned storage across subdomains (the make-or-break).** A `kernel.<site>` origin
  embedded as an iframe in two different `app-a.<site>` / `app-b.<site>` pages sees the *same* OPFS/IndexedDB
  under modern state partitioning, because they share a registrable site. If storage is partitioned
  per-embedding-context, the one-mirror model fails and the whole kernel needs rethinking. **Highest risk;
  test first.**
- **H2 — Single-writer sync loop across tabs.** One origin-scoped SharedWorker (with a Web Locks
  leader-election fallback where SharedWorker is absent, e.g. some mobile Safari) owns the OPFS blockstore
  and one sync loop no matter how many app tabs are open, with no OPFS access-handle contention.
- **H3 — RPC + blob transfer are fast enough.** postMessage RPC for record-sized reads is acceptably fast;
  blobs move as transferable ArrayBuffers (zero-copy). Measure a rough read latency and a blob handoff.
- **H4 — Change fan-out.** A record write in the kernel fans out over BroadcastChannel and both app skins
  re-render live.
- **H5 — The `repo-mirror` verifies and reads offline.** getRepo CAR → verify the commit signature against
  the DID document + every block CID against its hash → CID-keyed OPFS blockstore → read-your-own-data
  offline; an `ing.croft.account.prefs` ciphertext record round-trips. (atcute `@atcute/car`+`@atcute/cbor`
  are real per the 2026-07-22 fact-check, `research/atproto-clientside-search.md`.)
- **H6 — Domain-separation boundary holds (the tentative conclusion).** An untrusted renderer on a
  *separate registrable domain* cannot read the kernel origin's session/storage; the only channel is an
  explicit postMessage the kernel controls. Validates the T55 tentative conclusion.

### Hermetic vs. credentialed-env split

- **Hermetic here (local origins + fixtures):** H2, H3, H4, H5 (mirror mechanics against a **recorded
  `.car` fixture** — pdsview/arecipe already capture real getRepo responses; reuse one), H6. Needs a local
  multi-subdomain setup (a dev proxy or hosts entries mapping `*.croft.test` → localhost, TLS for the
  secure-context APIs) and a browser driving localhost.
- **Best in a real-domain / credentialed env:** **H1** — faithful state-partitioning behavior keys on a
  real registrable site (eTLD+1); `.test` may not exercise it identically, so H1 gets a hermetic first-pass
  and a confirmation on real `*.croft.ing` subdomains. The **live PDS getRepo leg** of H5 also hands to a
  credentialed env (recorded fixtures cover the mechanics here).

### Success / kill criteria

- **Suits the need (→ graduate):** H1 + H2 + H5 hold. The shared-origin kernel is viable; fold T55 into a
  `beta/impl/` kernel doc and plan the real build that unifies the live Track A pads (pdsview is the natural
  first `repo-mirror` home).
- **Needs rethink (→ do not fold):** H1 fails (storage partitions per-subdomain). The one-mirror model is
  not achievable as drawn; fall back options to evaluate — per-pad storage with a sync coordinator, or a
  different cross-origin sharing mechanism. Record the finding; the proposition is refuted for the
  as-drawn design, which is a valuable answer.
- Partial (H1/H2 hold, H5 has friction): viable with caveats; fold with the caveats named.

### What the spike does NOT build

No production kernel, no real auth/OAuth, no live network calls, no UI beyond two throwaway skins, no iroh,
nothing Track B. It is a mechanism probe, deleted after its result is recorded in a `SPIKE-RESULTS.md`
(shape: hypothesis → verdict → evidence, like `spike/hist_live/HIST-LIVE-RESULTS.md`).

## Graduation

On a pass, the outputs are: a `spike/account-kernel/SPIKE-RESULTS.md` (verdicts + evidence), a T55 status
move (`in-progress` → the mechanics validated), and a fold of the settled kernel design into a new
`beta/impl/` doc with the domain-separation decision (H6) recorded and the real build planned against the
live pads. On a kill, the outputs are the SPIKE-RESULTS finding and a T55 note that the as-drawn design is
refuted, with the fallback options to evaluate.

## Open decisions (surface, not resolve)

- **Local multi-subdomain harness choice** (dev proxy vs hosts + local CA) — an implementation choice; I
  will pick the lightest that gives real secure-context subdomains unless you have a preference.
- **Who runs H1 on real `*.croft.ing`** — the real-domain confirmation of the partitioning behavior needs a
  credentialed env; flag for hand-off.
- The domain-separation tentative conclusion (T55) is tested by H6, not assumed.
