# CISS build session — file-sync M4/M5 → the self-assertion dials → v0.7.0 (2026-08-07 → 2026-08-09)

**Raw:** `ciss-filesync-dials-build-session-2026-08-07.jsonl` (verbatim session
transcript, 11 MB — the Claude Code session log; every tool call, test run,
and dialogue turn). This file is the condensed work record.

**Repos touched:** CISS (all code; PRs #18–#34, releases v0.6.0 + v0.7.0),
homebrew-tap (formula PRs #9/#10), discovery (E89/E90 stamps, this filing).
All CISS work merged CI-green to `main`; every phase RED-first; mutation
audits on new logic per milestone.

---

## Arc 1 — the file-sync ladder finishes and ships (2026-08-07)

- **M4 (PR #18)** — iroh peer-fetch + serverless converge. New crate
  `ciss-iroh`: `IrohPeer` (blobs keyed sha-256, moved by blake3/Bao, sha-256
  re-verified on receipt), `PeerFirst` (peer-preferred reads; a restore's
  origin egress = exactly 1 GET), `MeshPeer` (gossip frontier,
  newest-signed-counter-wins) — `converge()` ran unchanged with **no server**.
  Live drill: two real CLI processes, identical trees, conflict preserved.
- **M5 (PR #19)** — the cost twin: `sync price` (the server's own linked
  tariff), `sync ceiling` (defer-whole, never partial, never bills), POSTURE
  **B6** (exit-exempt) born.
- **v0.6.0 released** (PR #20 + tag + tap #9) — changelog practice began.

## Arc 2 — the follow-on wave (user-directed, 2026-08-07/08)

- **PR #21** — monotonic-period spend ledger (user ruling: *never timestamps
  as authority* — period counters; reset preserves history), per-profile
  aggregate ceiling, the complete cost picture (ceiling caps TRANSFER only;
  at-rest always queryable), `docs/SYNC-MODEL.md` (the 3-way fold explained
  thoroughly — user-directed).
- **PR #22** — relay by default: probed the LIVE `relay.croft.ing:8443`
  (relay-only blob fetch proven), precedence `--no-relay` > `--relay` >
  profile file > default, unreachable-relay degradation pinned hermetically.
- **PR #23** — serverless persistence: fs-backed iroh store + durable alias
  index; the "Wednesday story" (multi-round p2p converge across full
  restarts) fixed, proven by test + kill-everything drill.
- **PR #24** — `BlobTransport::metered()` (free p2p never deferred/ledgered —
  a live-caught bug) + `sync ceiling --reconcile` against `GET /meter`
  (baseline-adopt; closes the multi-device spend blind spot).

## Arc 3 — the design conversation that became the dials

Review of "add a co-signed spending limit" (ADR 0004, PR #25) surfaced the
user's key observation: **the mechanism already existed three times** —
manifest (I5), policy record (Z6), DeviceHead — all one pattern:
*self-assertion* ("users assert their own requirements directly; no customer
service typing into a database"). Re-planned as the **self-assertion dials
ladder** (PR #26; Passes 1+2 + three review rounds, PRs #27/#28). User
rulings captured: provider caps supersede (refuse-at-set + `min()` always) ·
pre-1.0 crude purge, no migration machinery · **countersign from the first
assertion** ("otherwise you can't discern failure from success") · ack key =
first `verificationMethod` in did.json · exempt egress served AND billed,
made *legible* via the **drawdown dial** ("we can tell the difference") ·
drawdown **reversible by dial** (B now, C's monotonic period-gate in
reserve) · shrink-only keep-set in drawdown.

## Arc 4 — the dials build D1–D5 (2026-08-08/09, PRs #29–#33)

- **D1** — `src/assertion.rs`: one envelope for every customer-signed
  setting (Model A key-derives-DID / Model C provider-attested; acks on
  every write; domain-separated preimages; uniform typed 409). Policy
  re-homed as the `policy` kind on `PUT/GET /{did}/assertion/…` (old tables
  wiped); the manifest conforms; the M3 error-text-match deleted. Mutants:
  51 → 3 real gaps killed.
- **D2** — `dial.ceiling` at-rest half: refused-at-set above
  `min(store_ceiling, did_cap)` with the bound quoted; `min()` at the quota
  gate; acks verified offline against the published key.
- **D3** — spend half (402 refuse-with-quote; postage-per-period per the
  transfer-is-the-threshold ruling; rent awaits the statement-scheduler
  SEAM), `dial.period` (accept snapshots the meter baseline — monotonic),
  `dial.account-mode` (drawdown per rulings), **B6 enforced in code**.
- **D4** — `dial.receipt-mode` + bilateral receipts: the `501` seam
  unstubbed; `POST /{did}/receipt/{hash}/countersign`; completed receipts =
  doubly-signed facts verified offline under the two published keys.
- **D5** — POSTURE §15 (invariants **D1–D6**, incl. "no operator write path
  to any customer setting exists"); **ADR 0004 → Accepted-as-amended**; plan
  closed.

## Arc 5 — v0.7.0 (2026-08-09)

PR #34 + tag + GitHub release (asset sha256 `3ded8515…` verified) + tap
PR #10 + `brew upgrade` → `ciss-ctl 0.7.0` confirmed. Full changelogs for
both components (the release notes).

## Deferred, recorded in the plan close-outs

Model-C (did:) dial submission via CLI · manifest-on-substrate (unlocks
did:-plane file-sync) · rent inside the server ceiling (statement-scheduler
SEAM) · drawdown monotonic period-gate (in reserve) · auto-countersign
batching in the sync client · statements endpoint (supersedes meter-baseline
reconciliation).

## Where the durable records live

- CISS plans: `docs/plans/2026-08-07-*.md` (ladder, per-milestone close-outs,
  dials plan with full Review Log), `docs/adr/0004-*.md` (Accepted),
  `docs/SECURITY-POSTURE.md` (B6, D-series), `docs/SYNC-MODEL.md`,
  `CHANGELOG.md` + `crates/ciss-cli/CHANGELOG.md`.
- Discovery: E89 lanes (a) and (b) stamped DONE; E90 stamped ladder-complete.
- Session memory: `ciss-ctl-client-shipped.md`, `monotonic-not-timestamps.md`.
