# Drystone implementation profile: the dial sheet, with Croft as the reference profile

`Status: template authored 2026-08-19 (E111). The door, issuance, and finality dials are settled by`
`the S23–S26 + C2–C4 gate (green 2026-08-17, graduated into canonical Part 2 on 2026-08-19); the`
`sizing dials are gated on the §11.11 measurements and carry provisional values marked as such; the`
`wire pins are gated on the [gates-release] register. The Croft column is the reference profile —`
`the worked example a new implementer copies.`

## 1. What this sheet is, and how it works

The destination this artifact serves, in the owner's words (recorded at
`alpha/ROADMAP_TODO.md`, row E111): *"take Drystone spec 1 and 2 plus a pre-populated set of
implementation choices and hand it to someone who doesn't know shit about any of this, and they
can build a compliant thing."*

The mechanism is a **conformance profile**: one artifact enumerating every choice the
specification deliberately leaves open, such that a filled-in sheet **is** an implementation's
compliance declaration. The spec stays choice-free at the requirement layer (Part 2 §9, §10 state
the bars); all choices live here. A candidate implementation copies this sheet, fills in its own
column, and the filled sheet plus the Part 2 §9 conformance vectors *is* its claim — "we know
where we stand."

**The engine is the attribute-conditioned MUST** (adopted as Drystone normative style with E96;
Part 2 §6.4: *"prefer the attribute-conditioned MUST over SHOULD wherever a legitimate deviation
exists."*). Wherever a legitimate deviation from a default exists, the deviation is a **declared
attribute** on this sheet (visible, consented, checkable), never a SHOULD-licensed silent
exception. Consequences, in both directions: downstream spec text states properties flatly
per-attribute instead of hedging, and a reader of a filled sheet can assume every property the
declared attributes imply. SHOULD remains only for engineering advice with no conformance
content.

**Three scopes of dial**, marked on every row, because they are declared by different parties:

- **[wire]** — implementation-wide pins two implementations must share to interoperate. Declared
  once per implementation; several are `[gates-release]` (unpinned until the wire-freeze).
- **[deployment]** — operator-level choices (infrastructure present, service posture). Declared
  per deployment.
- **[charter]** — per-group governed attributes, set at genesis or under the group's own rules
  (R7), visible in the charter. The sheet records the implementation's *defaults* for these; a
  group's charter can move any of them within the spec's stated range.

**What a dial is not.** Where Part 2 states a MUST unconditionally, it does not appear here as a
choice — the sheet lists it only where a reader might mistake it for one (marked **not a dial**).
The exemplar: the merge-side finality gate is mandatory; only the serve side is dialable
(Part 2 §11.7, "The gates, layered").

## 2. The dial sheet (the template)

Each row: the dial, its scope, the options with their spec home, the spec-stated default, and
status. An implementation's declaration column is filled per §3's worked example.

### 2.1 Admission and re-entry (settled by the green gate)

| Dial | Scope | Options (spec home) | Spec default | Status |
|---|---|---|---|---|
| Serving door | [charter] | **A** open (standing check alone, or fully open) · **B** token (tree only against token + standing at head, after serve-time challenge) · **C** invite (no serving; universal invite machinery) — Part 2 §11.7 | **B** (canonical; the default membered group) | doors `Measured` (S22, S24, S25); door A's standing-check serve end-to-end untested (E112) |
| Issuance timing | [charter] + [deployment] | **at-join** (token minted at the Add-commit, held by the member from day one; no infrastructure) · **at-need-with-deposit** (requires a meer + identity-keyed mailbox) — Part 2 §11.7 | **at-join** (canonical) | at-join `Measured` (S24 models it throughout); at-need `Design`, gated on the store's third-party-deposit capability |
| Token lifetime | [charter] | **non-expiring** (standing-checked at head on every redemption) · **N-generation lapse** (lapses after N governance generations of dormancy; re-vouch to renew; never wall-clock) — Part 2 §11.7 | **non-expiring** (canonical) | non-expiring `Measured` (S24); the lapse variant `Design`, untested (E112 residual) |
| Serve posture | [charter] | **liberal-serve** (serve at the door's own check; worst bad-serve outcome is roster disclosure + refused-commit load) · **strict-serve** (tree-serving under the §7.3.8 gate; closes the stale-serve hole at the cost of stalling good-standing returners at a stale peer) — Part 2 §11.7 | **liberal-serve / strict-merge** (the default posture) | both arms `Measured` (S25 arms 3–4) |
| Merge finality | — | **not a dial.** Merging a `NewMemberCommit` is irreversible and sits under the §7.3.8 finality gate: corroborate-fresh or stall, fail closed — Part 2 §11.7 | mandatory | `Measured` (S25 arm 2, with HeadAck as freshness source, `Modeled` C3) |
| Admission fact | — | **not a dial.** Every admission deposits its admission fact; a merge that would not emit it is refused — Part 2 §10.2.2 (A8), §11.7 | mandatory | `Measured` (S24, C4) |
| Invite-lapse window | [charter] | committed-but-unredeemed invites are never-active leaves, expired by the ordinary liveness machinery; a charter wanting faster lapse applies a **shorter window to never-active leaves** — Part 2 §11.7 | ordinary liveness window | `Design`, unification untested (E112 residual) |
| Enactment (who fires the enforcing commit) | [charter] | the decision/enactment split (§7.3.6): designated actor, or **self-service** (the returner fires its own external commit), with the idempotent fallback if the designated actor is absent — Part 2 §7.3.6, §11.7 | self-service for returns | `Design` (per Part 2 §7.3.6, §7.6.7); the self-service path `Measured` (S14, S22) |

### 2.2 Regime and sealing (attribute-conditioned declarations)

| Dial | Scope | Options (spec home) | Spec default | Status |
|---|---|---|---|---|
| Confidentiality regime | [charter], **genesis-immutable** | **confidential** (content sealed; the §11.10 tiers price it at size) · **public** (§11.9.3: MLS as attestation, content on the public surface — experimental, gated on §11.11 item 7) — Part 2 §8, §11.9.3 | confidential | regime-at-genesis `Design` (Part 2 §8); no silent crossing — a regime change is a governed **successor-Group re-plant** (E109 bridge, §11.9.3/§11.10) |
| Sealing (the outer seal) | [charter] attribute with spec-stated default | **ON** for any confidentiality-regime group whose scope includes non-member carriers; **OFF** where every carrier is a member (Mode 2) or for public-regime groups with explicit public governance — Part 2 §6.4 | ON (per the condition) | mechanism `Measured` (S7/S17); attribute form `Design`, owner-ratified 2026-08-17 (E96). When in effect, the epoch rule and wrap-once rule are MUST |
| `carrier-visible` | [charter] declared deviation | a confidential-on-fabric group wanting conversation-attributable envelopes charters itself **carrier-visible** — consented, enumerated, subject-readable — Part 2 §6.4 | not declared | `Design` (E96); the deviation pattern this sheet exists for |
| Refusal verbosity | [charter] / [deployment] | **loud** (signed, corroborated rejection into Group immune memory) · **silent** (reject, no signal) · **blackhole** (tarpit) — Part 2 §8; applies at the door-B serve refusal (§11.7) | none stated; serious auto-response SHOULD require k-observer corroboration | `Design` (Part 2 §8) |
| Deployment mode | [deployment] | **Mode 1** relay + meer (internet-scale) · **Mode 2** direct P2P on a local network — Part 2 §6.11 | Mode 1 for internet deployments | `Verified` at the transport layer (§6) |

### 2.3 Sizing (gated on the §11.11 measurements — declare provisionally, revise on M1/M2)

| Dial | Scope | Options (spec home) | Spec default | Status |
|---|---|---|---|---|
| Liveness window | [charter] | the §11.6 band schedule (modest/aggressive per size band), or the recommended **dynamic policy**: drive the window from live hot-N against a target ceiling — Part 2 §11.6 | dynamic | policy `Design` (owner-ratified state model); the correct *values* are gated on §11.11 M1/M2 |
| Hot-N comfort ceiling | [deployment] target feeding the dynamic window | provisional **~1500** (tolerable ~2500) — Part 2 §11.6 | ~1500 | `Load-bearing, unearned` — set by §11.10.1 Experiment A; do not treat as an SLA |
| Retention | [charter] governance value | **retention ≥ the liveness window** — a per-Group governance value, never a service constant — Part 2 §11.6 | ≥ window | the floor is the rule (`Design`); the right margin above the floor is gated on M2 (backfill vs gap) |
| PCS posture | [charter] policy | **strict** below hot-N 250 (forced periodic Update) · **opportunistic** at and above (heals on ban/organic) — Part 2 §11.5, §11.10 | 250 threshold; 24 h strict cadence (the §11.10.1 fixed policy) | threshold placement `Design`, grounded in the RFC complexity results; cost discontinuity to be made visible at the 250 sweep point (Experiment A/C) |

### 2.4 Wire pins ([wire] — the interoperability layer)

| Pin | Current reference | Status |
|---|---|---|
| Signature | Ed25519 (RFC 8032) — Part 2 §10.4 | pinned by the spec (bar: key-as-identity, no silent downgrade) |
| Hash | **BLAKE3** committed; §4's `Verified` status currently stands on SHA-256; the re-base + conformance re-run land together at the wire-freeze — Part 2 §10.4 | `[gates-release]` |
| MLS ciphersuite + credential type | reference stack: openmls 0.8.1, bare-signature credential (the §11.10.1 baseline); final pin at wire-freeze — Part 2 §10.2, §11.10.1 | to pin; record per run until then (sz/state figures depend on it) |
| Domain-separation tag namespace | vendor-neutral `drystone-*` replaces the historical brand; the rename is signed-over and re-opens the §4 proofs, so it rides the same wire-freeze — Part 2 §1.3 | `[gates-release]` (Appendix B) |
| The byte-encoding register | the consolidated `[gates-release]` list: canonical governance-fact encoding (§7.3.1), content-id pre-image (§4.2), frontier-commitment + acceptance-record (§7.5.1), closure rule (§7.5.2), checkpoint/now encodings (§7.3.3, §7.3.7), envelope layout (§6.8.5), B1 record, RBSR fingerprint (§6.8.1), and the §11.11 set: re-entry token (PSK id + issuance fact), admission fact, HeadAck, governance-position pointer, ban-ceiling event — Part 2 Appendix B, §11.11 | `[gates-release]`, all unpinned |

**The honest interop statement a filled sheet must carry:** until the `[gates-release]` register
is pinned, no implementation can claim wire interoperability — a profile declares *which pin-set
version it implements* once one exists, and until then a conformance claim is scoped to the
Part 2 §9 vectors (the §4/§5/§6 proven layer) plus this sheet's declared attributes.

### 2.5 Temperament and posture (product-defaulted; every setting safe)

Per Part 2 §7.6.9 the protocol provides a safe range and the **product** defaults these by group
archetype; they appear here so the enumeration is complete, not because an implementation must
fix them globally.

| Dial | Scope | Options (spec home) | Spec default |
|---|---|---|---|
| Conflict response | [charter] | **hold-on-conflict** (flag, suspend enactment, resolve) · **auto-fork** — Part 2 §7.6.9 | hold-on-conflict for high-cohesion archetypes |
| Fork posture | [charter] | **merge-as-routine** · **fork-as-durable** (heal capability always present) — Part 2 §7.6.9 | posture, not policy — product-defaulted |
| Horizon cadence N | [charter] | contradiction re-evaluation boundary: every epoch commit + every N facts — Part 2 §7.6.9 | temperament dial; safe at every setting |

## 3. The Croft reference profile (the worked example)

Croft's declarations, one per dial above. Where a value is provisional or awaiting a product
pass, the cell says so — a profile that hid that would be the silent exception this sheet
forbids.

| Dial | Croft declares | Basis |
|---|---|---|
| Serving door | **B — token** | the canonical default membered group (§11.7); gate-measured |
| Issuance timing | **at-join** | canonical; no meer dependency for return rights |
| Token lifetime | **non-expiring**, standing-checked at head | canonical; the lapse variant stays off until tested (E112) |
| Serve posture | **liberal-serve / strict-merge** | the default layered posture (§11.7); worst bad-serve outcome is roster disclosure, never admission |
| Invite-lapse window | **ordinary liveness window** (no faster lapse) | the unification default; revisit if invite abuse appears |
| Enactment for returns | **self-service** (returner fires its own external commit) | the cost-on-returner setting (§11.7); measured path |
| Confidentiality regime | **confidential** at genesis for all Croft groups | Croft is a private messenger; the public regime (§11.9.3) is experimental and not offered |
| Sealing | **ON** (Mode-1 deployment: non-member carriers exist) | the spec condition selects ON; epoch rule + wrap-once in force |
| `carrier-visible` | **not declared** | no Croft group meters its own fabric; the deviation is unneeded |
| Refusal verbosity | **loud** *(provisional — final by the Croft presentation pass, E116)* | §8's corroboration SHOULD honored; verbosity interacts with the presentation obligations |
| Deployment mode | **Mode 1** (relay + meer; relay.croft.ing) | internet-scale deployment |
| Liveness window | **dynamic policy**, hot-N target ceiling **~1500** *(provisional — `Load-bearing, unearned`)* | §11.6 recommended policy; ceiling set properly by Experiment A |
| Retention | **≥ the liveness window**; margin set on M2 | the §11.6 floor; the backfill measurement prices the margin |
| PCS posture | **strict < 250 (24 h cadence) / opportunistic ≥ 250** | the §11.10.1 fixed policy, unchanged |
| Signature | **Ed25519** | spec pin |
| Hash | **BLAKE3 committed**; running on the SHA-256-proven §4 until the wire-freeze re-proof | the honest split the spec itself states (§10.4) |
| Ciphersuite / credential | **openmls 0.8.1 reference stack, bare-signature credential**; recorded per run, pinned at wire-freeze | §11.10.1 baseline |
| Tag namespace | **`drystone-*` on rename**; historical brand tags stand in the proven wire constants until the re-proof | §1.3 |
| Wire pins | **none pinned** — no wire-interop claim; conformance claim scoped to the §9 vectors + this sheet | the honest interop statement (§2.4) |
| Conflict response | **hold-on-conflict** | Croft's archetypes are high-cohesion; product defaults per §7.6.9 |
| Fork posture | **merge-as-routine** presented; capability per spec | product default |
| Horizon cadence N | **product-defaulted per archetype**; not implementation-fixed | §7.6.9 temperament dial |

## 4. What this sheet does not settle (named, so the sheet cannot overclaim)

- **The sizing dials' final values** — gated on §11.11 M1 (per-commit and fan-out at hot-N
  500/1000/2000 on representative hardware) and M2 (backfill vs gap). The §11.10.1 matrix is the
  procedure; every provisional figure above is an envelope value, not an SLA
  (`[gates-release]` for any figure that becomes one).
- **The wire pins** — the `[gates-release]` register (§2.4). The sheet carries the list; pinning
  is the wire-freeze work.
- **Door A's standing-check serve, the token-lapse variant, the invite-lifecycle unification,
  post-ban ledger hygiene** — E112 residuals; the affected dials say so in their rows.
- **The public regime's dials** — §11.9.3 is experimental, gated on §11.11 item 7
  (non-member-verifiable attestation); a profile declaring the public regime is declaring an
  experiment.

## 5. Rider: the Croft presentation obligations (product layer, flagged E116)

Carried on this sheet per E112 so they are not lost; they are **product** work, not protocol
dials. The obligations, with their spec homes:

1. **The factual fork statement** — a fork announcement carries a factual, non-editorial
   statement of cause in the dataplane (Part 2 §7.6.6); Croft's UI must render it as such.
2. **The exposure disclosure** — the §11.8 stale-admission exposure window (everything sent
   between admission and re-exclusion) and the §7.6.12 two-phase revocation interval must be
   disclosed to the group where they occur, not smoothed over.
3. **The three response registers reachable** — mute, governance, fork (Part 2 §7.6.5) must all
   be reachable in the product, with the lightest register the path of least resistance; A7
   stands — no per-member prompt at the admission gate.
4. **Returner-side "admission voided" legibility** — when a stale admission's span is closed
   forward (§11.7 comparator placement, §11.8 re-fire), the returner sees a legible
   "admission voided" state per E108's `CONTESTED`-pattern rendering, not a silent failure.

These graduate to the Croft product backlog as row **E116** in `alpha/ROADMAP_TODO.md`.
