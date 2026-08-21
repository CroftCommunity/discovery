# Companion review: where social-tree-core sits in the whole program

`Written 2026-08-20 by the planning session, at the owner's direction, as a deliberately`
`wider-angle companion to the independent vet (REVIEW-social-tree-core-plan-2026-08-20.md).`
`The independent review answered "is the plan sound and spec-faithful"; this pass answers`
`"how does it align with everything else being built" — the calling track, the relay, the`
`identity story, the estate, and the spec's own trajectory. It does not repeat the vet's`
`findings; it assumes them. Claims are tagged verified (checked against a file this session)`
`or judged (assessment). No BLOCKER-grade material here; this is orientation, sequencing,`
`and a handful of additional cheap-now moves.`

---

## 1. Plain-English summary

This plan is the moment the program's two spines finally bolt together. Since June, Croft has
run two tracks that never touch: the **product/calling track** (croft/android, the connect
contract, the relay — all shipped, all device-validated, all riding *centered* rails: relay,
PDS, OAuth) and the **protocol/Drystone track** (the spec, the fold, MLS, the experiments —
all center-free, all loopback/Modeled, none of it in a product repo). The croft-home decision
is not just a repo choice; it is the first time Track B's code lands inside Track A's repo.
That is why it feels bigger than a crate move — it is. The alignment holds: nothing in the
plan contradicts the calling track, the spec's trajectory, or the client ADR, and several
things line up better than the plan itself notices. The genuinely new items this pass adds
are: name the two-admissions distinction before it confuses someone (§4); name the DID ↔
persona-key binding as the seam where the two tracks' identity models will meet (§5); carry
the substrate's existing device/principal split and charter-as-data through the re-cut intact
(§6); and sequence Phase 2's croft landing around M4 rather than into it (§7).

## 2. The program-level fit: two tracks, one repo, and why that is right

The croft repo today contains a shipped Kotlin calling app whose every load-bearing
dependency is centered infrastructure: `relay.croft.ing`, an atproto PDS, OAuth, a
denylist-at-mint (D3). The incoming crate is the center-free substrate whose Part 1 posture
is that no such center may hold authority. These are not in tension — Part 1 §2.7 permits
capability freely where authority is revocable, and the helpers-not-authorities discipline is
exactly what licenses a relay — but after Phase 2 the repo will *contain both postures*, and
the difference should be legible in the repo's own architecture record rather than tribal
knowledge. The croft ADR the plan already commissions (Pass 3 Q1: foundation-vs-feature-core
layering) is the natural place; see §4 for the specific paragraph it should carry.

Worth saying plainly: the sequencing that produced this moment was right. The calling track
proved the product shell, the release machinery, the device rituals, and the deployment
identity on centered rails that could ship *now*; the protocol track matured the center-free
substrate to C-series/mutation-vetted grade without product pressure deforming it. Landing
the substrate into the product repo only after both are true is the opposite of the usual
failure (shipping the ideology before the mechanism, or the product before the principles).
**judged.**

## 3. Spec Part 1: the plan advances the razor, and one old misfit dies at the right moment

The fold's refusal to manufacture a verdict is the razor (compute provenance, never utility)
made mechanism, and `CONTESTED` (Phase 1) is that refusal made *visible to the member* — the
projection stops pretending it knows. Putting E108 first is therefore philosophically the
correct priority, not merely dependency order: the honest-disagreement machinery hardens
before any product surface renders it. **judged**, anchored on part-1 §2.0 (the razor,
"trustworthy disagreement") and §2.5 (fork, not verdict).

The signed wall-clock timestamp inside `AssertionEnvelope.canonical_bytes()` (the vet's
R2/O9) deserves one more sentence than the vet gave it, because Part 1 §2.0.1 draws exactly
the line it sits on: a timestamp is a licensed *data-plane convenience* ("tolerable when it
only sequences what is shown") and disqualified from anything decided. Signing it into the
canonical bytes of *governance* facts elevates an assertion into what a later reader will
mistake for provenance — precisely the laundering §2.0.1 warns about, even though today's
comparator never consults it. P1 is already bumping the envelope schema; this is the one
moment the fix is free. Endorse O9 at P1, not later. **verified** (types.rs:251–252, 298;
part-1:178–227).

## 4. The two admissions: name the distinction before it bites

The program now has two live things called "admission," in two different repos, at two
different layers:

- **Fabric admission** (croft-stack / the relay, D3, M4): who may use the relay's resources —
  a *deployment*-scoped capability check (service-auth mint, sponsorship + device-scope
  claims, denylist, QoS ceiling). AGENTS.md literally names croft-stack the
  "MEMBERSHIP / admission backbone."
- **Group admission** (Part 2 §10.2.2 A-series, this plan's Phase 4): who is a *member* of a
  group — a governance decision, rights-bearing, center-free, where A3 (validity MUST NOT
  imply admission) is the load-bearing invariant.

They share a word and nothing else, and the failure mode writes itself: someone wires the
relay's admission signal into a group-admission decision (or describes one in the other's
terms in a doc), and the center quietly acquires authority over membership — the exact
S16-class failure (validity treated as admission) one layer down, and the Princeps-Problem
shape COHESION §24 names. One paragraph in the croft layering ADR — "the relay admits
*traffic*, never *members*; no signal from fabric admission is an input to the A-series" —
costs nothing now and forecloses it. **judged**; the collision surface is **verified**
(AGENTS.md "Calling surfaces"; part-2 §10.2.2 via the vet's C3; the D3 decision record per
the workspace orientation).

## 5. Identity: the seam where the two tracks will actually meet

Track A's identity is an atproto DID proven by OAuth (M3's `provenDid`); Track B's identity
is persona keys — Ed25519 with the razor's own gloss that "is this key the person I mean" is
a human judgment the system records, never computes (part-1 §2.0). Phase 7 (product shells
adopting the core) is where these must compose: the calling capability attaches to the
*rendered principal* seam (croft/CLAUDE.md), and once the social tree renders principals, the
natural product win is calling grants derived from group membership and standing rather than
only from explicit atproto records. That requires a DID ↔ persona-key binding — which per
Part 1 is a *recorded human act* (a vouch/binding fact), not a lookup.

Nothing in this plan forecloses that, and nothing needs to move into it. But nobody has named
the seam, and it is the single most predictable Phase-7 design question. Two cheap moves now:
(a) a ROADMAP_TODO row so it is on the board before the successor plan is drafted; (b) in
Phase 2, keep the core's principal type opaque-but-attributable (it already is —
`PrincipalId` — keep it that way; no atproto types anywhere near the core). **judged**;
the seam's two ends **verified** (croft/CLAUDE.md "Calling is a capability"; traits.rs
`(DeviceId, PrincipalId)` credential pairs).

## 6. Two things the substrate already got right — protect them through the re-cut

Checked this session because they are the classic expensive retrofits, and both are already
present. The re-cut should treat them as load-bearing surface, not incidental shape:

- **Device ≠ principal, as revocable pairs.** `traits.rs` models `DeviceId` distinct from
  `PrincipalId`, with `CredentialResolver` validating and revoking the *pair*
  (traits.rs:18–103). That is spec §4.5's multi-client fold guarantee (client-count is not
  persona-count) structurally present before any product multi-device work exists — the
  thing that, had it launched single-device-shaped, would have been the migration everyone
  dreads (and it feeds A2, the program's named biggest open problem). Phase 2 should carry
  the pair-shaped credential boundary into the core verbatim. **verified.**
- **Charter as data, not constants.** `GroupRules` with per-key thresholds and `RuleChange`
  as a governed fact (types.rs:196, 399, 467; governance.rs:44–49) means the group's rules
  are already folded state. The implementation profile's [charter] dials (door, issuance,
  lifetime, serve posture, temperament) are the same species — per-group governed values.
  The vet's O6 (profile as typed config) should be strengthened into a Phase-2 acceptance
  criterion: **the core reads its posture from charter state in the fold, never from
  compile-time constants** — `GroupRules` is the existing socket the dial sheet plugs into.
  A core that baked door-B/at-join in as assumptions would make the E111 sheet a fiction.
  **verified** (the socket) / **judged** (the criterion).

## 7. Sequencing against the live tracks: interleave, don't collide

The vet flagged the M4/P2 coordination (R7); the practical schedule follows from what each
phase touches. Phase 1 (E108) is entirely discovery-side — zero collision with M4, can start
on the go-ahead. Phase 2 is the collision point: it lands the root Cargo workspace and the
first CI gate in the same repo where the M4 session is mid-milestone. The natural interleave:
**run P1 concurrently with M4 now; land P2 at a coordinated moment** — after M4's current
milestone closes, or with an explicit heads-up so the M4 session expects the new required
check rather than meeting it mid-push. The gate is Rust-scoped and M4 is Kotlin/relay-side,
so the risk is friction, not breakage — but a surprise required check on someone's
in-flight milestone is exactly the kind of friction a one-line coordination avoids.
**judged**, collision surface **verified** (croft git log per the vet's R7; P1 scope
plan:92–103).

One adjacent watch-item, not a task: croft-stack's journaled admission/usage machinery and
the spec's meer role (store-and-forward custodian; the at-need-with-deposit issuance dial
depends on a meer's third-party-deposit capability, implementation-profile §2.1) are two
growth paths toward "the helper that holds things for absent members." They should converge
into one design eventually, not drift into two implementations nobody decided to have. A
COHESION seam-line when relay Phase 7–8 work starts is enough. **judged.**

## 8. The license call, recorded

Owner decision this session: **AGPL-3.0 for everything Drystone/Croft** — the core crate
declares it, and croft-chat's `MIT OR Apache-2.0` was an accident to correct, not a position.
Note this is *consistent with the standing July decision*, not a new one: ROADMAP_TODO A14
records the reference-code license as superseded 2026-07-09 → AGPL-3.0-or-later + DCO
("the copyleft lock"). The vet's R3 therefore resolves to: P2 declares AGPL-3.0 on
`social-tree-core`, and the discovery workspaces' license fields are corrected to match at
the pin-bump. A1 (the MPL-2.0 `hpke-rs` dep) is untouched by this — MPL and AGPL coexist;
that gate stays A1's. **verified** (A14 row; croft/LICENSE per the vet's R3).

## 9. A payoff the plan earns but never brags about

Extracting the fold + projection as a pure crate is also, for free, the engine of the
§11.9.2 public-projection tier: an AppView-shaped cache is "the projection, run by a helper,
over deliberately-public facts" — and a projection that is a pure function of the log is
exactly what a helper can run server-side without becoming an authority (it can withhold,
never lie — content-addressed, governance-anchored). The same purity that buys the wasm
gate buys the future read-tier without a second codebase. No action now beyond what the
Store-port advice already says (append-batch + snapshot-load; keep the projection
shell-agnostic); this is a reason to hold the purity line when it gets annoying mid-P2.
**judged**, anchored on part-2:2376–2388.

## 10. Additional cheap-now moves (beyond the vet's O1–O10)

- **O11 — the two-admissions paragraph** in the croft layering ADR (§4). One paragraph;
  forecloses a whole failure class.
- **O12 — a ROADMAP_TODO row for the DID ↔ persona binding seam** (§5), so Phase 7's biggest
  design question is on the board before its plan is drafted.
- **O13 — charter-as-data as a P2 acceptance criterion** (§6): no [charter] dial value as a
  compile-time constant in the core; `GroupRules` is the socket.
- **O14 — the credential-pair boundary travels verbatim** (§6): the `(DeviceId, PrincipalId)`
  pair shape is a §4.5 guarantee, not an implementation detail; the P2 test migration should
  include whatever pins it.
- **O15 — a COHESION seam-line for croft-stack ↔ meer convergence** (§7), opened when relay
  Phases 7–8 begin.

## 11. What this pass did not do

Did not re-verify anything the vet verified (its file:line evidence is taken as standing);
did not re-run any suite; did not read Part 2 beyond §1.3, §2, §11.9, and the sections the
vet anchored; did not evaluate the relay/M4 design itself (a concurrent session owns it —
§4 and §7 only *reference* its decided shape); did not re-open any Pass-3 owner decision.
The platform-comparison ground the vet covered (L4) is deliberately not repeated here.
