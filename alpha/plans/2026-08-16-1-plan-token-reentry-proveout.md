# Plan: prove out token re-entry end to end (S23–S26)

`Written 2026-08-16, out of the readmission decision talk-through (decisions 1–2 of the
five-decision conversation; see the WORKING spec copy's §11.7/§11.6 REV blocks and ROADMAP rows
E105/E106/E107).`
`Status: RUN GREEN (2026-08-17, as amended by 2026-08-17-1-plan-head-currency-and-admission-fact.md
— the gate for decision-2's graduation was S23–S26 + C2–C4, C5 informative). Decision 2 GRADUATED
and MERGED into canonical part-2 on 2026-08-19 (step 5). Kept as the experiment record.`

## Problem statement

E106 is ratified: the governance-issued **external PSK** is the re-entry credential for the
external-commit path, persona-bound by a three-way cross-check (chain issuance fact + leaf
credential resolving to the lineage + attached PSK, standing at head). Decision 2 agreed the
surrounding shape: **two issuance mechanisms** (at-join canonical; at-need-with-deposit where a
meer + CISS mailbox exist), **three serving doors** as a group charter attribute (open /
token / invite), bare `GroupInfo` served freely, the ratchet tree as the gated artifact, and a
**strict-merge finality floor** (§7.3.8 applied to merging a `NewMemberCommit`).

But the measured base (S16, S18–S20, S22) covers the *mechanisms*, not the *composition*:

- Position 2 has never been exercised as an end-to-end **admission decision** (issue → dormancy
  across epoch rolls → serve-check → redeem → the refusal arms). E107's open third.
- The walk surfaced an unstated MLS constraint: **every incumbent that may process the return
  commit must resolve the PSK** — so token material is group state (a "token ledger") that must
  reach members who join *after* the issuance. Never tested.
- The stale-peer behaviour is measured only for the serve step (S22); the **merge step under
  governance lag**, with and without the §7.3.8 finality gate, is untested — and it is where the
  invisible-fork risk (S18) would materialize.

Unverified compositions are indistinguishable from hallucinations; the spec merge is gated on
these runs.

## Approach

Four experiments in the existing `alpha/experiments/meer-queue` harness (Rung A, real
openmls 0.8.1, real CISS where storage is involved), continuing the s-series. Issuance is
modeled **at-join** throughout (the canonical mechanism); the at-need-with-deposit variant is
**deferred** — it is gated on the CISS third-party-deposit blocker (the standing E95/payment
question) and adds no new redemption mechanics, only delivery.

### S23 — the token ledger: PSK resolvability across membership change

The constraint to prove, both directions:

1. **Negative arm first (RED):** incumbent B holds group state but *not* the PSK bytes for
   returner R's token. R presents a valid external commit + PSK proposal. What exactly happens at
   B — a clean processing error, a silent drop, a staged-commit failure? Name the failure mode;
   it decides how loud a missing-ledger bug is in production.
2. **Ledger transfer:** member C joins *after* R's token was issued (and after R went dormant).
   The token ledger reaches C (modeled as sealed app-layer state synced in-band — not in
   `GroupContextExtensions`, which leaks into served `GroupInfo`). R returns; C processes and
   merges R's commit successfully.
3. **Revocation as chain fact:** the issuance fact is marked revoked; incumbents still *hold*
   the PSK bytes but the policy check refuses. Proves revocation needs no key-deletion race.

### S24 — position 2 as an end-to-end admission decision (E107's open third)

One scenario, run whole: N=5 group, tokens minted at join (chain issuance facts + ledger);
member R goes dormant; the group **rolls ≥2 epochs** (including R's batched eviction); R returns.

- **Graceful arm:** R presents to an arbitrary member's serve endpoint → bare `GroupInfo`
  served unconditionally; tree released only after the serve check (issuance fact, unrevoked,
  standing at head, challenge signature proving lineage-key control). The serve exchange is the
  challenge-response protocol (talk-through, 2026-08-16): P-generated single-use nonce; R signs
  `tag("serve-tree/v1") ‖ nonce ‖ group_id ‖ psk_id` with a key chaining to the lineage root;
  response `{GroupInfo, tree}` sealed to R's presented key (design note; sealing itself may stay
  untested at this rung). Serve-protocol arms: **(s-i)** a replayed challenge-response is
  rejected (nonce single-use); **(s-ii)** a valid `psk_id` presented by a requester who cannot
  sign for the issued-to lineage is refused *at serve* (previously only killed at merge).
  Rate-limiting keys on `EndpointId` only — never authorization (standing rule). Refusal
  verbosity is the §8 loud/silent/blackhole dial, not tested here. R builds the external
  commit (leaf credential chained to lineage root + PSK proposal + AAD attestation); every
  incumbent applies the merge rule (`NewMemberCommit` + known unrevoked PSK + credential
  resolves to the issued-to lineage + standing at head) → merged, current, sends and reads at
  the new epoch (the S18 grade: AEAD-level, same-epoch).
- **Refusal arms, each its own test:** (a) token but no lineage key (stolen-token bearer) —
  dies at the credential half of the merge check; (b) lineage key but no token — dies at serve
  (no tree) *and* at merge if handed a leaked tree; (c) valid token + lineage, standing revoked
  at head — dies at serve and at merge.
- **Perishability check:** a `GroupInfo`+tree served at epoch E is refused after the group rolls
  to E+1 (formalizes the S20 refusal as a property of the serving design: leaked artifacts decay
  per roll; the token is the only durable thing).
- **Artifact-isolation check (cheap, closes the quadrant):** the tree *without* a current
  `GroupInfo` constructs no join (structurally implied — no `external_pub`, no group context —
  but never measured; the mirror of S19's measured GroupInfo-without-tree refusal). Together the
  two arms pin the advertising rule: `GroupInfo` freely, tree never, combo only through the door.

### S25 — the stale-peer matrix, with and without the finality gate

The propagation-window question in test form. Peer P is **key-current but
governance-lagging** (has not folded a ban of lineage X; X holds a pre-ban token).

| arm | serve posture at P | merge posture at incumbents | expected |
|---|---|---|---|
| 1 | liberal (best-known) | best-known | P serves; lagging incumbents merge, synced refuse → **measure the divergence** (the S18 invisible fork made concrete) |
| 2 | liberal | **strict (§7.3.8: stall until standing corroborated fresh)** | P serves; every incumbent stalls or refuses; X never seated — the "strict-merge + liberal-serve" middle holds |
| 3 | strict | strict | P stalls the serve; also measure the cost: dormant-in-good-standing R asking *the same stale P* is stalled too (the liveness price of strict-serve) |

| 4 | **none — all serve checks skipped** (sloppy/compromised server) | strict | requester with no token, or wrong lineage, receives `GroupInfo`+tree; every incumbent refuses the commit → **assert the requester's net gain is roster knowledge only, never admission** |

Arm 2 vs arm 3 is the data the finality-posture dial needs. Arm 4 turns the layering claim —
**the serve check protects the roster; the merge check protects the membership** — into a
measured property: under a strict merge, the worst outcome of any bad serve (sloppy,
compromised, or governance-stale) is roster disclosure. Also record how long P's lag
persists under the harness's sync cadence — the first real number toward the propagation
window (E107's other third).

### S26 — catch-up replay determinism: admission is evaluated at-position, never at-head

Member M is offline when returner R's valid join commit lands and the group extends past it.
The rule under test (talk-through, 2026-08-16): **the merge check for a commit at causal
position X is evaluated against the governance fold up to X, not the evaluator's current
head** — §7.3.1's authorization-at-causal-position rule extended to admission; valid-at-its-
position stays valid, later governance acts forward. The commit's AAD attestation carries the
claimed chain position as a **locator, never an authorization input** (§7.4.3 discipline): M
verifies the claim against its own fold-to-that-position.

1. **Convergence arm:** R joins validly at X; R is banned again at Y > X; M replays the whole
   sequence from behind. M must land **byte-identical** with the live members (join applied at
   X, ban applied at Y). This is the arm a head-anchored evaluation cannot pass — M would
   refuse the historically-valid join and self-exile.
2. **Position-anchoring pinned:** a characterization test that fails if the check consults
   fold-at-head instead of fold-at-position (the mutation this test exists to kill).
3. **Stretch arm — the stale-majority case:** a mostly-lagging group merges an at-position-
   invalid join; M syncs later. Posture under test: **live-edge refusal decides which branch is
   extended; catch-up correction is governance-forward, never chain-refusal** — M processes the
   commit structurally (an epoch roll carries no inherent social meaning, §7.6.2), reads
   standing from the fold (the invalid member is experientially excluded, §7.6.12 phase 1), and
   converges with the group once the corrective removal enacts (§11.8 re-fire). Chain-refusal
   during catch-up is reserved for a deliberate fork.

## Reasoning

- **Why these three and not more:** they cover exactly what decision 2 asserts beyond the
  measured base — ledger (S23), composition (S24), staleness posture (S25). Issuance-at-need,
  UX, and Croft dial surfaces add nothing cryptographic and wait for the CISS deposit work.
- **Why RED-first on S23:** the ledger constraint was *derived* from RFC/S16 reading, not
  measured; the failing arm proves the constraint is real before we build bookkeeping to
  satisfy it. If the negative arm unexpectedly passes, decision 2's ledger obligation dissolves
  and the spec text gets simpler — a result worth having either way.
- **Why S25 measures rather than assumes the divergence:** this arc's corrections (S14, S22,
  G1) all came from refusing to trust the plausible answer; arm 1's fork is plausible, not
  measured, and it is the one that would hurt a real group.
- **Method guards** (from this workspace's standing findings): surface every ingest/processing
  result — no `let _ =` (G1's lesson); compare branches at equal epoch numbers for the
  AEAD-grade checks (S19's lesson); commit green state before any hand-mutation; scenario
  realism per the owner's S-series challenges (no socially-unreachable pairings).

## Exit criteria

All S23–S26 arms green (or failing arms with named, understood failure modes), TEST-LOG.md
rows written with fidelity rungs, STATE-AND-NEXT amended. Then — and only then — decision 2's
REV blocks graduate from PRELIMINARY and the step-5 merge of the readmission conversation can
include them. E106's already-ratified pieces do not wait on this plan.
