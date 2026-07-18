# RUN-18 — Reception completeness + the publications positioning (delta on RUN-16/17)

`Run summary, 2026-07-18. Branch claude/publications-run-18-eh07xx (the dispatch harness's
designated branch; the brief named claude/publications-run-18 — same run, suffixed name), from
origin/main at the RUN-17 merge (fff7acc), satisfying the sequencing gate (main carries merged
RUN-16 + RUN-17). A DELTA run per the brief: living corpus documents amended, the landed
tier-proof experiment extended; nothing re-run; the frozen RUN-16/17 summaries untouched.
TDD red-first throughout (docs-TDD for Part A, code-TDD for Part B). Rust 1.94.1, bats 1.13.0.
ATP_TEST_* credentials are unset, so B5's live deletion is BLOCKED and the harness delete event
stands in, tagged (guardrail 4: never silently downgraded). One provenance note, recorded not
glossed: the brief's PUBLICATIONS-DESIGN.md payload did not accompany the dispatched brief, so
PUBLICATIONS.md was authored directly from the brief's §0.2 specification (its content
requirements are the corpus-test assertions 26–33, all green).`

## Environment preflight

| Check | Result |
|---|---|
| Sequencing gate | main = `fff7acc` (RUN-17 merge, atop RUN-16 merge) — **satisfied** |
| `cargo` / `rustc` | 1.94.1 |
| `bats` | 1.13.0 (installed this run via npm; RUN-16 used 1.10.0) |
| `ATP_TEST_HANDLE` / `_PASSWORD` | **unset** → B5 live deletion BLOCKED (guardrail 4) |
| Site gate deps (`markdown`, mermaid-cli) | installed per `site/requirements.txt` + `npm ci --prefix site` |

## Part A — corpus (red → green)

| Step | What | Commit | State |
|---|---|---|---|
| A1 | corpus-tests +15 RUN-18 checks (reception ×5, cross-pointers, publications ×7, staged note, link+anchor audit) | `a82e225` | **RED** (15 fail, 19 landed stay green) |
| A2 | GROUPS.md: the canonical reception paragraph at A.2; A.8 silent-to-detected sentence; A.9 history-row pointer | `6d2cb76` | GREEN (checks 20–24) |
| A3 | `appview-infra/PUBLICATIONS.md` landed + reciprocal GROUPS.md intro pointer; joins the link + anchor audit | `265f875` | GREEN (checks 25–33) |
| A4 | RUN-18 addendum staged (proposed-changes RUN-16 section) + reviews-log row, same commit; backlog §6f rows | `179a79f` | GREEN (check 34) |

**corpus-tests before/after: 19 → 34 checks, 34/34 green** (`bats
appview-infra/corpus-tests/groups_claims.bats`). Site gate
(`python3 site/build.py --check`) **gate OK** at A3 and A4 (no new unresolved §-references;
GROUPS.md/PUBLICATIONS.md are corpus, outside the published set — their gate is the corpus-tests
link + anchor audit, which PUBLICATIONS.md joined at A3).

## Part B — experiment (red → green, per part)

| Part | What | RED | GREEN | Tests | Grade |
|---|---|---|---|---|---|
| B1 | chaining validation: fold genesis-anchor + chain heads; relay `Unchained`, running heads; open-write exempt | `7dc19ec` | `55a9cc3` | 5 | **component** |
| B2 | gap detection + repair: middle gap named from the chain alone (no oracle); interval backfill repairs; provably complete up to newest held | `7f08173` | `9348371` | 2 | **component** |
| B3 | the tail, honestly: claim = `complete as of <newest held>`, never currency; swarm-path arrival advances it (multimodal closure) | `508d730` | `7dc469d` | 2 | **component** |
| B4 | chaining × interval: repairs within [J, now) only; the crossing visible as structure; offering refuses pre-J | — (int.) | `58bd2c2` | 2 | **component** |
| B5 | retraction three-way: existence provable from the chain; never-existed / retracted / withheld-from-me classified distinctly; vanilla contrast asserted | `d90878c` | `ed04b8e` | 3 | **component** (live BLOCKED) |
| B6 | auditable reach: independent refold = served count; unsubscribe moves both identically; asserted-only count detected unsupported | `33d0aff` | `6c86230` | 3 | **component** (vs the landed harness) |

_B4 is integration over B1–B3 + the landed P8 offering rule and introduces no new machinery by
design, so no red state exists for it (the RUN-17 P6b precedent: its "red" would be the absence
of B1–B3, already covered). All other parts were committed failing first._

**Suite: 49 → 66 tests in `tier-proof/` (+3 unchanged in `steward-seal/` = 69 total), all
green; `cargo clippy --all-targets` clean.** No new processes were added (brief §3): B2/B3
reuse the landed P8 `EnvelopeStore`/`converge`/`offer_interval` machinery, B6 the landed P2
fold.

## Predicted-vs-actual (live legs, guardrail 4)

| # | Prediction | Verdict | Observed |
|---|---|---|---|
| B5-live | with `ATP_TEST_*`, deleting the middle issue's record on the real PDS yields an authenticated absence a reader can verify, upgrading B5 to live grade | **BLOCKED (live)** | credentials unset; the live deletion was not run (guardrail 4). The harness's authenticated delete event stands in — `SPEC-DELTA[run18-retraction-local \| stand-in]`; `source::live_retraction_leg` reports BLOCKED, never fabricates. |
| B6-reuse | P2's live records may be reused where still present | **n/a** | RUN-17's P2 live legs were BLOCKED and wrote no live records; there are none to reuse. B6 graded component against the landed harness, as the brief's fallback names. |

## Brief-label → landed-location mapping (guardrail 2)

| Brief label | Landed location |
|---|---|
| "the two-axes section" | GROUPS.md **A.2** (paragraph appended after the coordinates paragraph) |
| "the transports section's swarm paragraph" | GROUPS.md **A.8**, the swarm-delivery paragraph (one sentence appended) |
| "the tier table's history-access row" | GROUPS.md **A.9**, `History access` row (pointer added in the open + gated cells — warranted, both carry the guarantee) |
| "PUBLICATIONS-DESIGN.md → alpha/experiments/appview-infra/PUBLICATIONS.md" | landed there; **payload absent from the dispatch**, authored from brief §0.2 (see preamble; its §6 anchors verified by corpus check 33) |
| "the RUN-16 section of the proposed-changes doc" | `beta/drystone-spec/proposed-changes-2026-07-experiment-reconciliation.md`, **RUN-18 addendum** appended after the RUN-16 update (same section, house pattern of appended updates) |
| branch `claude/publications-run-18` | `claude/publications-run-18-eh07xx` (dispatch harness's designated name) |
| RelayReject for the chaining check | `RelayReject::Unchained`; the chain-link convention is FIRST antecedent = author chain link (documented in `src/chain.rs` and enforced in `src/relay.rs`) |
| landed-test amendment forced by B1 | `tests/p3_write_policy.rs::author_post_into_newsletter_is_served` now anchors its post to genesis (living code; the only landed test the new rule touched) |

## Evidence mapping (for `(evidence: …, RUN-18)` citations)

B1–B4 prove the GROUPS.md **A.2 reception paragraph** clause by clause: chaining REQUIRED and
enforced at the relay with the first envelope genesis-anchored (B1); any gap detected as a known
omission from public data alone and repaired via interval backfill from any role (B2); the
withheld-tail limit and its multimodal mitigation — the claim scoped to the newest held
envelope, advancing when the swarm path delivers (B3, which is also the A.8 silent-to-detected
sentence, executable); and open enrollment composing with the interval rule — detection may see
history's shape, offering respects the interval (B4, also A.7/A.9's interval-backfill cells).
B5 proves the PUBLICATIONS.md **§4 history row** (tamper-evident vs tamper-free, the three-way
distinction) and its §6 anchor; B6 proves the **§4 reach row** and the §5 auditable-reach and
structural-consent bullets (the unsubscribe is an authenticated deletion). The corpus-tests
(A1) hold the documents to saying exactly what B proves.

## Registers

- `EXPERIMENT-BACKLOG.md` §6f — reception completeness (proven by Part B), the publications doc
  landed, the auditable-count claim tied to B6, the B5 live leg BLOCKED row.
- `MASTER-INDEX.md` — RUN-18 appended to the `appview-infra/` and `tier-proof/` rows.
- `SPEC-DIVERGENCE-REGISTER.md` — `run18-retraction-local` row + summary-paragraph mention.
- `proposed-changes-…-reconciliation.md` — RUN-18 addendum (`caveat`/`needs-call`, staged);
  reviews-log row in the same commit (`179a79f`). The reviewed spec (`part-*`, conventions) is
  untouched.

## Gates (definition of green for this run)

- corpus-tests: **34/34** green.
- `cargo test` (tier-proof): **66/66** green; steward-seal untouched (3/3).
- `cargo clippy --all-targets`: clean.
- site gate: **gate OK** (0 hard-gated; baseline allowlist unchanged).

## Non-goals (one line each, brief §4)

- **No re-run of RUN-16/17** — this is a delta; the landed P-suite is extended, not re-driven.
- **No frozen-record edits** — RUN-16/17 summaries untouched; this run has its own summary.
- **No freshness-dial decision** — the solicitation posture stays a governed dial, owner's call.
- **No swarm transport upgrade** — the P8 stand-in suffices (`run17-swarm-local` unchanged).
- **No paid-tier build** — PUBLICATIONS.md names it as a policy value, nothing more.
- **No Variant A/B or boundary decisions** — unchanged from RUN-17 (A.10 stays owner-open).

## Reproduce

```bash
cd /home/user/discovery
bats alpha/experiments/appview-infra/corpus-tests/groups_claims.bats   # 34/34
cd alpha/experiments/tier-proof && cargo test && cargo clippy --all-targets  # 66 green, clean
cd ../../.. && python3 site/build.py --check                            # gate OK
# With credentials, B5's deletion runs live instead of BLOCKED:
ATP_TEST_HANDLE=… ATP_TEST_PASSWORD=… cargo test --test b5_retraction
```
