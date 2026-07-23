# Raw: a privacy-preserving usage-measurement kit — edge counters over a declared graph (2026-07-23)

**Preservation status: preserved-condensed (cleaned-paste, content-faithful — NOT byte-pristine) for the
dialogue; the RUN-MEASURE-01 experiment file below is preserved VERBATIM (it was pasted in full) —
PLAYBOOK §4.** Source: a pasted Claude chat session, 2026-07-23. UI render chrome stripped (thinking-summary
headers, "Document · MD" markers). No secrets. Part of the 2026-07-23 batch (plan:
`alpha/plans/2026-07-23-transcript-intake-socialization-arecipe-measurement.md`).

**Status: a NEW helper concept** — a cross-property (not arecipe-specific) privacy-preserving usage-
measurement kit; arecipe is the recommended low-risk pilot. Lands as alpha thinking + ROADMAP_TODO, not a
beta doc.

**Flags — technical/quantitative claims `[UNVERIFIED]`** (Phase-2 fact-check): sendBeacon landing rates
(~95.8% on visibility/pagehide; ~82.9% across all four unload events); the 64 KiB sendBeacon cap; the CNIL
July 2025 self-assessment tool and its three-event ceiling; Poplar / Poplar1 / VDAF (prefix-counting private
heavy-hitters); the Litestream ~2.6M PUT/month figure from the prior run; R2's 1M Class-A free tier. Ties to
Skylite's sponsor-visible-vs-child-private stance and the existing Caddy/systemd/SQLite/Litestream/R2 kit
shape. Umami is named as the prior tool the user has used.

**Fact-checked 2026-07-23** → `alpha/research/2026-07-23-batch-factcheck.md`. **One REFUTED, one PARTLY,
rest CONFIRMED:** (REFUTED) the **sendBeacon rates are mis-sourced/inverted** — real is ~82–83% at
unload/pagehide/visibility and ~92–93% at page *load*; 95.8% appears in no source (E5 must be fixed).
(PARTLY) the **64 KiB cap is a Fetch-`keepalive`/browser behavior, not a Beacon-spec mandate** — Chrome
enforces it as a *shared* quota across in-flight beacons (E5's ceiling test must account for that). CONFIRMED:
bfcache (unload/beforeunload disqualify; pagehide/visibilitychange recommended); **CNIL July 4 2025** tool
(max 3 event types; nearest-ten rounding); **Poplar1/VDAF** (needs ≥1 honest server / two non-colluding
operators); **R2 free tier 1M Class-A/mo** + the ~2.6M-PUT/mo arithmetic (only if written every second).

**Decision (user, 2026-07-23):** default **local-history retention = bounded retention + a visible clear
control** (mirrors Skylite's sponsor-visible-vs-child-private stance). This resolves one of RUN-MEASURE-01
§11's owner open items; the panel shows what is kept locally vs what leaves. (Kit **name and home** remain
open; **rounding vs exact counts** stays gated on E8.)

---

## The dialogue

**User:** Ok so I have a set of mostly PWA/SPA sites and no interest in user data but a need to understand
actual usage — what works, what's never used, etc. I've used Umami for basic stats on some cases but it's
not a total solve and I want to explore friendly options in this space. I fear a new helper type may be
warranted and I want to get the edges right and always show users what's there. Is there a way to basically
record local path journeys in some cryptographic manner, or link clicks or something, as a counter to give
some idea of how things are found and used? A crypto path would be like a pre-staged from-here-to-here
ordered event that increments a counter locally, and that's part of shown and flushed. And we could use that
page to show queued-for-flush and stats-last-n or something — make it useful for them as well. And I like the
Mozilla thinking: we run far less or no feature-and-A/B tests really at the moment, so expires is good but not
the current story but still useful, and it could honor at source client-side too.

**Assistant:** What you described has a name: **an edge counter over a declared graph.** It needs no
cryptography, and cryptography would not add much.

**How it works.** A registry entry is a *transition*, not a page. `nav.recipes_index__to__recipe_detail` is
one counter. The journey information lives in the counter's name, which you wrote at build time, rather than
in a per-user sequence assembled at runtime. Increment locally, flush the bag of counts. Sum across everyone
and you have a **transition matrix**, which renders as a flow diagram: how people arrive at a thing and what
they do next.

**What it can and cannot tell you.** It is **first-order only.** A→B→C is indistinguishable from A→B
happening separately from X→B→C. That is the loss, and it is the same fact as the privacy property — you
cannot buy one back without selling the other. Two mitigations to build in from the start: an **`other`
bucket** for transitions you did not declare (so you learn unmodelled traffic exists even though you can't see
what it is); and **declared edge sets** rather than the full matrix (n pages implies n² edges, most of them
noise).

**The cryptographic version, and why not.** It exists and is well specified. The **Poplar** protocol solves
private heavy-hitters: each client holds a bit-string and the aggregation servers compute the set of strings
occurring at least T times, using a secret-shared point function that lets servers count how many client
strings begin with a given prefix. **Poplar1** in the VDAF draft implements that prefix counting — literally
your question, and it discovers popular paths you never declared. But the guarantee holds only as long as at
least one aggregation server executes honestly, which means **two operators who do not collude.** You have one
VPS. Skip it, and note in the docs that the declared-edge counter is the degenerate single-operator case of
the same idea.

**The panel doing double duty is the strongest part of this.** It gives a clean two-tier story that is true
rather than reassuring: **local view** — rich, ordered, your actual session, never transmitted, clearable;
**outbound view** — the exact counter names and counts queued for the next flush. "Everything about you stays
on this device; here is the number that leaves" is a claim you can *demonstrate* rather than assert. One seam
you already know how to think about: a rich local history is readable by whoever holds the device — your
Skylite work landed on sponsor-visible-versus-child-private as a deliberate stance, so that call has to be
made here too, and probably lands as bounded retention plus a visible clear control.

**On expires.** Honoring it client-side is right, for a PWA-specific reason: a service worker can keep serving
a cached bundle long after the registry moved on, so a build-time lint alone lets a stale client emit retired
metrics for months. A runtime date check in the generated code stops that at source. Given little test
coverage today, runtime honoring is not the belt-and-braces option — it is the *only* enforcement you have.

**User:** Ok put together a set of experiments to run in Claude Code to prove this out end to end.

**Assistant:** Nine experiments, E0 through E8, framed as a proof-out in a scratch repo rather than a build.
Two are load-bearing in a way the others are not. **E3** is the one that can kill the design — it treats the
receiver as an attacker (link flushes to devices, re-order them, single out an individual), with mitigations
tested one at a time and attack scores before/after; if re-linkage still succeeds at high precision once IP
is dropped and the schedule is jittered, the counters-not-logs premise does not hold. **E2** contains a
falsification test worth watching: construct two genuinely different path populations with identical
transition matrices; if that pair cannot be constructed, the matrix carries more individual information than
the design assumes — a finding, not a failed task. **E8** turns the rounding decision into a table rather than
an argument: at what traffic level does nearest-ten rounding stop destroying the dead-feature signal. Four
open items are left for the owner, including whether `edge` sits inside the three-event ceiling or reads as a
fourth type (the run records the argument both ways). [File below, verbatim.]

---

## RUN-MEASURE-01 (verbatim)

**RUN-MEASURE-01: proof-out experiments for the counter-based measurement kit**
Date: 2026-07-23. Type: experiment run, not a product build. Self-contained: yes.

### 0. What is being proved

A usage-measurement design for PWA/SPA properties with these properties:
1. Instrumentation is declared in a **registry** file. The registry generates the client calls, the
   user-facing disclosure panel, and the test fixtures. Drift between them is a failing test, not a
   documentation debt.
2. The client keeps **counters, not logs**. It increments named counters in local storage and flushes a bag
   of name-to-count pairs. No ordering, no session key, no sequence leaves the device.
3. Journey information is carried in **declared edge counters** (`nav.a__to__b`) so aggregate flow is
   recoverable while individual journeys are not held anywhere.
4. The disclosure panel does double duty: it shows the user their own rich local view (never transmitted)
   alongside exactly what is queued to leave.
5. Metric `expires` dates are honored **at runtime in the generated client**, not only at build time.

The run must produce honest findings, including negative ones. An experiment that shows the design leaks is a
successful experiment.

**Standing conventions (apply to every phase).** TDD, red first, always — every acceptance bullet becomes a
failing test before implementation; fixtures before features; the run summary shows red-to-green ordering
with evidence; where a phase is visual/exploratory and test-first is not honest, say so explicitly rather
than retrofitting a claim. Declared stand-ins — anything simulated (synthetic traffic, faked network,
mocked eviction) is registered in a stand-ins list with what a live version would change. Verify-in-run
ledger — items in §10 are empirical; probe them, do not assume; record what you observed, including "could
not determine". No unsourced claims in output docs — cite the spec if a doc asserts a spec behavior.

**Repo homing.** Run inside a scratch experiment repo (`measure-proof` or similar). Deliverables are
findings, goldens, and harnesses. Extraction into a real repo is a later, explicit step and is not part of
this run.

### 1. E0: harness and ground truth

Everything downstream depends on generating a known population of behavior and comparing what the system
recovers against what happened. **Build:** a synthetic traffic generator producing a corpus of ground-truth
sessions (ordered page sequences and feature touches; configurable population size, path-length
distribution, fixed seed); two profiles — `small` (~100–1000 sessions/month) and `medium` (~50k
sessions/month), parameterised (note real profiles are owner-supplied later). **Acceptance:** same seed →
byte-identical corpora; the corpus is the only source of truth used in E2, E3, E8 (no experiment generates
its own ad-hoc data).

### 2. E1: registry generates three artifacts, drift fails the build

**Registry format.** YAML, Glean-shaped. Each entry declares at minimum: `type`, `description`, `expires`,
and a `disclosure` string in plain language for the panel. Metric types permitted: `page` (presence on a
page + page info); `feature` (a functionality used — button, link — + associated info such as destination or
label); `timing` (load time, scroll, time spent); `edge` (a declared transition, `from`/`to`). The four types
are deliberate — the first three mirror the ceiling in CNIL's July 2025 self-assessment tool for
consent-exempt audience measurement; `edge` is our addition and its relationship to that ceiling is an open
question (§11), not a settled claim. **Build:** a generator emitting, from one registry file: typed client
instrumentation calls; a disclosure-panel data structure (name, plain-language description, expiry); test
fixtures. **Named invariant tests (must never be deleted):** `emit-without-registry-entry-fails`;
`registry-entry-without-disclosure-fails`; `panel-matches-registry` (panel contents derived from the
registry, asserted by golden file — hand-editing the panel breaks the test). **Acceptance:** all three red
first, then green, then a deliberate drift in each direction shown to fail.

### 3. E2: do edge counters actually recover flow

**Claim:** summing declared edge counters across a population yields a transition matrix from which useful
aggregate flow can be reconstructed. **Counter-claim (equally important):** a first-order transition matrix
cannot distinguish distinct path populations, and this ambiguity is the privacy property. **Build and
measure:** run the E0 corpus through the edge counters, aggregate, reconstruct a flow view; compare against
ground truth with a divergence measure reported separately for paths of length 2, 3, 4+ (quantify the
degradation). **The falsification test:** construct two genuinely different ground-truth populations that
produce identical transition matrices; assert equality in a test. If you cannot construct such a pair, that
is a significant finding reported prominently. Implement an `other` bucket for undeclared transitions; verify
traffic outside the declared edge set is counted as volume without being identified. **Acceptance:**
divergence numbers per path length; the identical-matrix pair exists and is asserted; the `other` bucket
counts undeclared traffic without naming it.

### 4. E3: adversarial re-linkage (the experiment most likely to fail)

Treat it as an attack, not a checkbox. **Setup:** a receiver storing flush payloads exactly as a naive
implementation would (receipt timestamp, source IP, TLS/connection metadata, request ordering). **Attack
tasks** (E0 ground truth as scoring oracle): **A1 Re-linkage** — group flushes belonging to the same device;
score precision/recall. **A2 Re-ordering** — given linked flushes, recover session order; score against
ground truth. **A3 Singling out** — isolate an individual using combinations (rare edge + rare timing +
coarse geo); report the smallest combination that isolates one contributor in `small`. **Then test
mitigations, one at a time, re-scoring each attack after each:** jittered vs fixed flush schedule; IP
discarded or truncated at the receiver before any write; no per-flush identifier of any kind; coarse time
bucketing of the receipt timestamp; minimum-count suppression at write time. **Acceptance:** a table of
attack scores before and after each mitigation; an honest statement of residual linkage that survives all
mitigations. If A1 succeeds at high precision even after mitigation, say so plainly and flag it as blocking —
the whole design rests on it failing.

### 5. E4: what crosses the boundary

**Claim:** the local view is rich, the outbound payload is counts, and the boundary is enforced by code
rather than convention. **Build:** a local store holding the rich view (ordered events, timestamps, the
user's own session); a flush serialiser producing the outbound payload. **Tests:** schema assertion on the
wire payload (unordered map of counter name → integer, plus a coarse period marker); property test over many
generated local states asserting the payload contains no ordering, no session identifier, no timestamp finer
than the declared bucket; `local-stays-local` named invariant (fails if any field present only in the local
store appears in a serialised payload); round-trip — reconstructing the local ordered view from the payload
is impossible (demonstrate, do not assert).

### 6. E5: PWA lifecycle reality

Use Playwright. Real browser, real service worker. **Measure, do not assume:** flush reliability on
`visibilitychange` → hidden and `pagehide`, across browsers in the harness (report observed landing rate;
published measurements put sendBeacon at ~95.8% during those events, all four unload events at ~82.9% —
expectations to check, not results to reuse). Confirm no `unload`/`beforeunload` listener exists anywhere in
the generated client (named invariant `no-unload-listeners`; these break bfcache). Offline: go offline,
generate activity, come back online, confirm counters survive and flush; then a cold start with the app
closed between. Payload size: sendBeacon caps queued data at 64 KiB — determine at what counter cardinality a
flush approaches the ceiling and what the client does when it would exceed it (chunking or dropping, chosen
and tested). Storage eviction: attempt to induce eviction of the local store and record the behavior (note
honestly if the harness cannot reproduce real iOS eviction — a likely stand-in).

### 7. E6: expires honored at source

**Claim:** a stale cached bundle stops emitting retired metrics without any server involvement. **Tests:**
build a client from a registry with a metric whose `expires` is yesterday; assert the generated client does
not increment it, with no network and no server contact; build-time lint — a registry with an expired metric
produces a warning, and a configurable mode that fails the build; simulate the stale-service-worker case —
serve an old bundle, advance the clock past an expiry, confirm emission stops. **Note for the summary:** with
limited test coverage in the wider codebase, runtime honoring is the only enforcement actually present in
production — say so explicitly rather than presenting build-time lint as a safety net.

### 8. E7: receiver and infra fit

Match the existing kit shape: Caddy vhost, systemd unit, canonical SQLite, Litestream to R2, per-service
manifest declaring canonical vs disposable paths. **Measure:** write frequency — compute PUTs/month for N
sites at candidate flush cadences (per-session, hourly, daily); compare against R2's 1M Class-A free tier
(the prior finding on this box: a 1s Litestream sync interval on a continuously-writing DB produced ~2.6M
PUT/month, over the free tier); establish which flush cadences keep the measurement service inside budget and
state the crossover point. Destroy-and-restore drill for the measurement service, consistent with the
existing fire-drill acceptance pattern.

### 9. E8: the rounding question, answered with a number

The open decision: round counts to the nearest ten (CNIL's recommended technical measure for anonymous
statistics) or keep exact counts backed by a written anonymity justification. **Do not argue this. Measure
it.** Take the E0 `small` corpus; compute, under nearest-ten rounding, how many genuinely-zero-use features
are indistinguishable from features used one to nine times; repeat for `medium`; report the traffic threshold
above which rounding stops destroying the dead-feature signal. **Acceptance:** a single table the owner can
read to make the call. No recommendation in the summary beyond the numbers.

### 10. Verify-in-run ledger

Empirical questions; record observed answers, including failures to determine. (1) Does
`visibilitychange` → hidden fire reliably when an installed PWA is backgrounded on iOS, as distinct from a
browser tab? (2) localStorage vs IndexedDB for the counter store — which survives better under eviction
pressure, and does the choice change the offline result in E5? (3) Actual sendBeacon behavior when the
payload exceeds 64 KiB — does it return false cleanly, and does the client detect it? (4) Does the Litestream
PUT math from the prior run hold for a low-write summary workload? (5) Whether any browser in the harness
coalesces or drops beacons under rapid visibility flapping.

### 11. Open items for the owner

These block or shape work and should not be resolved by the run. **Pilot site** — recommendation is arecipe
(real feature surface, a genuine "which views are dead" question, no child-safety scrutiny riding on a first
attempt; Skylite should be last). **Rounding vs exact counts** — gated on E8 output. **Local history
retention** — a rich local view is readable by whoever holds the device; Skylite already treats
sponsor-visible vs child-private as a deliberate stance; needs a bound, a clear control, and a decision on
default retention. **Whether `edge` sits inside the three-event ceiling** — an `edge` counter is arguably a
`feature` use with its destination recorded (within the ceiling as written); it may also read as a fourth
type; the run records the argument both ways. **Name for the kit** — placeholder throughout.

### 12. Run summary requirements

Red-to-green evidence per phase (with the honest carve-out where test-first was not applicable); the declared
stand-ins list; E2 divergence table and the identical-matrix pair; E3 attack-score table before/after each
mitigation plus the residual-linkage statement; E5 observed landing rates per browser; E7 PUT arithmetic and
the cadence crossover; E8 rounding table; the verify-in-run ledger with observed answers; a plain statement
of anything the run could not establish.
