# RUN-16 — the group tier model: canonical write-up and spec incorporation

`Run summary, 2026-07-18. Branch claude/group-tier-model-run-16, from main at the RUN-15 merge
(8782654). A docs-and-registers run: no code, no money, no credentials. It lands the canonical
group tier model as alpha/experiments/appview-infra/GROUPS.md v2 (Section A placed largely
verbatim), stages its spec-facing implications, and updates the registers. Docs-TDD: the
corpus-tests claims-checks were extended RED first with the new model's invariants, then the
documents landed to turn them green. The reviewed spec (beta/drystone-spec/part-*, conventions)
is untouched; no Part 2 or social-mapping status tag moved.`

## What the model is (GROUPS.md v2)

The two-tier framing of the RUN-15 D11 brief (below/above `group_scale_boundary`) is superseded and
extended: **one lineage, one envelope, one delivery plane, one catalogue**, with the tier expressed as
**two independent policy axes** —

- **Membership policy**: `open` | `gated` | `sealed`
- **Write policy**: `open` | `members` | `named-set` | `single`

The familiar shapes (open forum, newsletter, announcement channel, working group, sealed circle) are
coordinates on those axes. What changes between a 2-person sealed circle and a 100k open forum is which
proofs the scope pays for and which transports its traffic rides — not a different primitive. The
RUN-15 Variant A/B write-path fork analysis is **preserved intact** as GROUPS.md Section B; the owner
decisions it asked are restated unchanged (plus one new) in A.10. This reverses no landed decision.

## Docs-TDD — the corpus-tests red → green table

`alpha/experiments/appview-infra/corpus-tests/groups_claims.bats` was extended RED first (commit
`a21596f`), asserting the v2 model's invariants against a GROUPS.md that did not yet carry them; the
documents then landed to turn each green. The seven D11 v1 checks are preserved and never went red.

| # | Corpus-test claim (v2 invariant) | RED (a21596f) | GREEN by |
|---|---|---|---|
| 8 | two policy axes, not one (membership + write) | ✗ | GROUPS.md v2 (`801ce68`) |
| 9 | three tiers — open/broadcast, gated/backplane, sealed | ✗ | GROUPS.md v2 |
| 10 | silence is not a verdict (decay is presentation) | ✗ | GROUPS.md v2 |
| 11 | a role's sequence numbers are delivery cursors, never order | ✗ | GROUPS.md v2 |
| 12 | key authority lives in the DID document, via PDS attestation | ✗ | GROUPS.md v2 |
| 13 | sealed-scope helper-index rows are observation-born | ✗ | GROUPS.md v2 |
| 14 | delivery roles are separate processes, not one primitive | ✗ | GROUPS.md v2 |
| 15 | history backfill is scoped by membership interval | ✗ | GROUPS.md v2 |
| 16 | the iroh overlay is loaded only by sealed scopes + governance | ✗ | GROUPS.md v2 |
| 17 | the open-topic survival rule is validate before relay | ✗ | GROUPS.md v2 |
| 18 | message identity is the hash of the canonical envelope | ✗ | GROUPS.md v2 |
| 19 | the RUN-16 model note is staged (proposed-changes + reviews-log) | ✗ | staged note (`7905e77`) |

Final state: **19/19 green** (7 preserved D11 checks + 12 new v2 checks); `red count: 0`. Bats 1.10.0.

## Files touched (one commit per step)

| Commit | Step | File(s) |
|---|---|---|
| `a21596f` | corpus-tests RED | `appview-infra/corpus-tests/groups_claims.bats` (+12 checks) |
| `801ce68` | GROUPS.md v2 | `appview-infra/GROUPS.md` (rewritten around Section A; Section B preserved) |
| `7905e77` | staged spec note + reviews-log | `beta/drystone-spec/proposed-changes-2026-07-experiment-reconciliation.md` (RUN-16 update appended to the RUN-15 section), `beta/impl/experiments/drystone-reviews-and-experiments-log.md` (RUN-16 row) |
| `92869b2` | registers | `alpha/experiments/EXPERIMENT-BACKLOG.md` (§6d), `alpha/experiments/MASTER-INDEX.md` (RUN-16 row) |
| `34912c1` | cross-reference disposition | `beta/impl/experiments/drystone-reviews-and-experiments-log.md` (records the pointer stays in the staged note) |
| (this) | summary | `alpha/experiments/RUN-16-SUMMARY.md` |

## Register rows (backlog §6d)

The model opens or records five items:

- **(a) Sealed-tier ceiling — churn-simulation experiment.** Runnable candidate: measure the sealed
  tier's practical ceiling (A.9 "low hundreds") as an evidence-graded curve over the croft-group
  loopback harness (no boxes) before ratifying a number — the A.10(2) reframe.
- **(b) The write-policy axis.** Landed design (A.2): the second, independent policy axis, enforced by
  the same gate at serve and relay time; enforcement reuses the backplane roster-lookup + R7 machinery.
- **(c) Sealed-scope helper-index rows are observation-born.** Taxonomy correction (A.7): forward
  secrecy ages the ciphertext out, so these rows are not re-derivable projections — class them
  canonical (a `state.db` sidecar) or knowingly accept them losable. Flagged for the kit's backup-audit
  invariant (RUN-15 D5).
- **(d) Self-host an iroh relay vs public relays (A.10 item 4).** Owner call, pointing at the hosting
  kit's service-manifest model (a relay is another always-on service the kit could provision).
- **(e) History-convergence role — membership-interval backfill.** Landed design (A.7); the
  interval-scoping-of-offering mechanism is a shaped RUN-17 proof over the loopback convergence harness
  (RBSR is `Modeled` at loopback from RUN-12 Part 3b).

## Spec-facing (staged, `needs-call` — nothing landed)

A **RUN-16 update** was appended to the RUN-15 section of
`proposed-changes-2026-07-experiment-reconciliation.md` (not a rewrite):

1. the two-tier framing is superseded by the three-tier / two-axis model;
2. the trusted-gatekeeper acceptance now attaches to the **backplane (gated/open)** tier, with
   membership as a **universally verifiable public fact** — the fused sealed-tier check (MLS
   key-schedule membership MAC) **splits** in the backplane into signature-vs-DID-document-key
   (authorship) plus roster-lookup-at-causal-position (membership);
3. the **delivery plane as authority-free roles realized as separate processes** (web-native DS, swarm
   peer, history-convergence node, helpers — sequence numbers are delivery cursors, never order), with
   the **transport split** (overlay only for sealed scopes; backplane on the plain web stack;
   validate-before-relay swarm gossip; envelope-hash dedup) and **membership-interval backfill**.

`needs-call` is preserved; the reviews-log carries the row in the same commit; **no status tag moved**.
Cross-reference disposition: the GROUPS.md v2 pointer is carried by the staged note (tier-cleanliness
forbids a clean beta doc from carrying an alpha-stage back-reference), not written into
`social-mapping.md`.

## What remains owner-open (A.10, unchanged by this run)

1. **Large-tier write path** — Variant A (repo-canonical ciphertext, server-held scope key; strong
   portability, minimal backup) vs Variant B (server-canonical content; simplest build, heaviest
   backup, weakest portability). GROUPS.md Section B scores both; D12 (RUN-15) built the fork-agnostic
   serving.
2. **The `group_scale_boundary` number** — reframed: the sealed-tier ceiling is measurable (backlog
   §6d(a) churn simulation) and should be measured before a number is ratified. Working number 5000.
3. **Launch order** — softened: the open tier is a zero-decision on-ramp; croft-groups before or with
   stellin-appview.
4. **New (from A.7):** whether to self-host an iroh relay (a hosting-kit service-manifest question) or
   use public relays initially.

## Gates (definition of green for this run)

- **corpus-tests:** `bats alpha/experiments/appview-infra/corpus-tests/groups_claims.bats` → 19/19 green.
- **site gate:** `python3 site/build.py --check` → `gate OK` (0 hard-gated; the same 7 companion
  allowlisted refs as baseline — the RUN-16 update to the published `proposed-changes` doc introduced
  no new unresolved §-reference). GROUPS.md and the registers are corpus (not in the published set); the
  proposed-changes and reviews-log docs are.

## Reproduce

```bash
cd /home/user/discovery
bats alpha/experiments/appview-infra/corpus-tests/groups_claims.bats     # 19/19
PUPPETEER_EXECUTABLE_PATH=/opt/pw-browsers/chromium-1194/chrome-linux/chrome \
  python3 site/build.py --check                                          # gate OK, 0 hard-gated
```
