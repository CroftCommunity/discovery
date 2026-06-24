# Roadmap (rough — organize-now, refine-later)

date: 2026-06-15

status: skeleton for refinement. This gathers milestones, available/wanted features, the
topic-vertical map, and a "next to do" list from across the corpus. It is intentionally
rough; we refine it as more fragments land (including a transcript that already carries
"next to do" thinking).

Naming/branding: **"Croft" is now the name center of gravity** (see NAMING.md). Org =
`CroftCommunity`. Sub-product names still open under it.

Recent additions folded in: the interaction-tiers model (interactive/quiet-large/broadcast,
type-at-creation; thinking/interaction-tiers.md), durable-product principles (settings-three-
audiences, shapeability-requires-stability/LTS-interfaces; crystallized/principles.md Tier 3),
and the encrypted-media path proven over real iroh-blobs (PR #5).

---

## Two tracks

The work runs on two interleaved tracks. The **validation track** de-risks the protocol
(proofs/experiments); the **product track** ships user-facing value. The product track's
messaging milestone is gated by the validation track.

```
VALIDATION TRACK (Proofs / experiments repos)
  Phase 0 scaffold ........................... GO
  Phase 1 crypto gate (openmls survivor re-key) GO   ← the make-or-break, passed
  Phase 2 data model + merge ................. GO
  Phase 2.5 / 2.6 adversarial ................ GO (2 gaps found + closed)
  Phase 3 real-iroh thin slice ............... GO (transport caveat)
  Social-layer V1–V9 ......................... GO (modeled)
  appview-validation (atproto reality) ....... done (spike)
  — still to validate: total-device-loss recovery; openmls leaf-credential (multi-device);
    real threshold-signed checkpoints; Achilles-heel adversarial research (unrun)

PRODUCT TRACK (the dossier's sequencing)
  M0  Vault / "Alt.Drive" — encrypted content-addressed substrate (macOS first).
        Single-user value on day one, before any network effects.
  M1  Secure group chat + always-on peer + PDS (~9–12mo, 2–3 eng in the dossier estimate).
        ← gated by the validation track above.
  M2  Social layer — the graph-you-hold, intimate groups, broadcast/civic regimes.
  M3  Economic layer — consumer-pull ad inversion, Lightning micropayments.
  M4  Cooperative governance — charter, dues, lifecycle.
  Full vision: ~3–5 year build; co-op incorporated early, "build in public."
```

## Available / wanted features (the build inventory)

Organized, not yet sequenced. "proven" = has a green proof; "designed" = specced;
"open" = unresolved.

| Feature | State | Notes |
|---|---|---|
| Lineage group chat (two-tree: governance + MLS) | proven (real openmls) | the core |
| Survivor-epoch reconnect / fork-as-feature | proven | E1.2/E2.x |
| Consensual backfill (history as navigable tree) | proven | E2.7 |
| Per-device keys under one DID lineage | designed + modeled | openmls leaf-credential check open |
| Device revocation as a governance op | designed + modeled | |
| Visibility regimes (intimate/public) + propagation geometry | proven (modeled) | V1–V9; V3 needs UX control |
| Broadcast / civic notice boards | designed (transcript) | social-layer §8–10 not yet written |
| Graph-you-hold social layer (label vs graph split) | designed | S3 quiet-membership unsolved |
| Blind superpeer / always-on peer (queue, snapshot, rendezvous) | proven blind (E3.4) | de-facto-mandatory for good UX |
| Encrypted content-addressed vault (iroh-blobs) | designed (dossier) | M0 wedge |
| Public content via atproto (DID, custom lexicons) | spike-validated | custom NSIDs propagate w/o registration |
| Multi-identity (no forced linkage) | designed | S4 |
| Offline transitive trust via Merkle proofs | designed (dossier) | hashing-tree proof incoming |
| Total-device-loss recovery | open | largest residual risk; no anchor chosen |
| Consumer-pull ad inversion + Lightning | designed (dossier) | proof-of-attention open |
| Cooperative (LCA + PBC, founder royalty, charter) | designed (dossier) | M4; own vertical |

## Topic verticals (standalone narratives + the overall one)

Several interrelated topics each have enough for a standalone narrative, plus the bigger
overall story. Planned set (see narrative/verticals/):

1. **The lineage-group protocol** — fork-as-feature, provenance as the dual primitive.

2. **Multi-device & identity** — keys ≠ identity; DID lineage; "all peers equal in rights,
   not capabilities"; recovery.

3. **The social graph you hold** — non-extraction as the point; label/graph split;
   freeze-by-default; quiet membership; visibility regimes; civic notice boards.

4. **The cooperative** — credit-union-not-a-club; charter, lifecycle, founder royalty,
   enshittification-proof clauses; the co-op IS the maintenance plan.

5. **The civic / philosophical why** — commons (Ostrom not Hardin); the recurring inversion;
   linear vs cyclical; renting-our-relationships-back.

6. **The substrate & economics** — vault, iroh stack, the realtime/durable split; consumer-pull
   ad inversion.

Overall narrative ties them: refusing extraction forces decentralization, which delivers
resilience and privacy — the ethical choice and the technical strength are the same choice.

## Next to do (rough — will merge with the transcript that carries this)

> See **`ROADMAP_TODO.md`** for the provenance-indexed backlog — every open item with a
> back-reference to its origin (`file:line`) and cross-links to open-edges / COHESION. The
> curated list below is the highest-leverage subset.

Highest-leverage first. To be reconciled with the "next to do" thinking in an incoming transcript.

1. **Decide the MPL-2.0 license gate** (lineage-groups PR #8 / Cycode) — `hpke-rs` is mandatory
   for RFC 9420 HPKE; no permissive substitute. A compliance call, not a code fix.

2. **Backport social-layer §8–10** into thinking/social-layer.md from the proven regime model
   (the doc's header promises sections that were never written; the proof already implemented
   them). Record the V3 "human-can-still-quote" limitation as a UX requirement.

3. **Choose a total-device-loss recovery anchor** — mnemonic seed vs social-recovery quorum vs
   broker-held encrypted backup. The field's hardest problem; we've chosen none.

4. **Verify the openmls leaf-credential dependency** (multi-device 8.1) so lineage-fold and
   lineage-counted thresholds rest on real library behavior, not the model's assumption.

5. **Run the Achilles-heel adversarial research** (seed prompt, unrun) — pressure-test the
   "is the superpeer secretly the ordering authority?" worry against what F-group asserts.

6. **Add the hashing-tree / Merkle trust-proof** to the Proofs repo (offline transitive trust),
   linking I3/I8 to the dossier's trust-graph design.

7. **Pin the name map** (dossier §1.1) before names leak into structure.

8. **Draft the cooperative charter specifics** (vertical 4) — governance lifecycle, founder
   royalty framing, enshittification-proof clauses.

9. **Run the deferred live-bsky validation** — `Proofs/encrypted-local-first-atproto/
   live-bsky-validate` is built and waiting on egress allowlist (bsky.social, plc.directory,
   the Jetstream host) + app-password env vars. Its RESUME.md is the self-contained handoff.
   Running it flips H2/H3/H5 from "validated locally" to "validated against the live network"
   and surfaces likely findings (custom-NSID acceptance, CID parity from DAG-CBOR
   canonicalization).

10. **Reconcile the public-path duplication** — PR #3's jetstream-appview / local-appview /
    end-to-end-slice overlap the standalone spikes (PR #4 public-roundtrip, PR #6
    appview-validation). Pick the canonical home, fold in the unique findings, stop
    maintaining three AppView slices (COHESION.md #5).

11. **Record the sequencer honestly** — the membership sequencer is load-bearing under
    concurrency (PR #3). Update the broker/superpeer framing: the claim is "minimal, blind,
    not a rights authority," not "no ordering authority." (COHESION.md #4.)

12. **The Croft app (client layer) — a new body of work, intake started 2026-06-22.** A
    composable "utility garden": one shell hosting **ponds** (Bluesky / Mastodon / Lemmy, kept
    native — honest seams) and **pads** (small self-contained apps), with the **Croft Group**
    pond (private chat + later P2P games) on iroh = the lineage-groups work surfaced. Design
    thinking landed at `thinking/app/` (philosophy, criteria, brand-draft, build-specs); the
    dialogue at `seeds/transcripts/raw/croft-app-design-dialogue-2026-06-20-to-22.md`. Stack
    decided: pure Rust functional core (`(state,intent)->(state,effects)`, WASM-clean) +
    per-shell imperative shells (CLI, web/Leptos, desktop/Tauri); web-first; the Crux *pattern*
    slim, not the framework. Phases: 0 (core+CLI+harness) **built**, 1 (Leptos web UI), 2
    (desktop wrap + composable shell + pinning). See `thinking/app/README.md`.

13. **Deferred: import the Croft-app Phase 0 code (CroftC PR #10) into `experiments/`** — and
    the IP/ownership decision it forces. Phase 0 (functional core + CLI, 20/20 tests green) was
    built in a **CroftC repo**; bringing it onto chasemp/CroftCommunity infra with a clean
    paper trail is the deliberate move, but it is the **user's call** (invention-assignment /
    conflict-of-interest with the Head-of-Product-Security role). Most time-sensitive. Not done
    this pass per the user ("untangle in this repo as a next step; don't sweat CroftC for now").

14. **Follow-on distillation from the app dialogue** — fold the industry research
    (iroh-in-browser maturity incl. relay-only browser peers; webxdc/Delta-Chat games + the
    WebRTC-transport-swap porting recipe; super-apps / W3C MiniApp; atproto appview routing via
    service-proxy/service-auth + the OpenMeet recipe; Rust client libs ATrium / Jacquard /
    megalodon-rs / lemmy-client-rs; Crux/FCIS) into `research/` and `ECOSYSTEM.md`, and reconcile
    the brand naming (Croft-the-product, Croft Group pond, pond/pad taxonomy, "Grow your own")
    into `NAMING.md` once `thinking/app/brand-and-voice-notes.md` settles.

15. **The sustainability ↔ cooperative *mechanism* question (existential).** The app dialogue
    surfaced the most important unthought thing: relays (browser peers are permanently relayed),
    the bridge node, the scoped appview, and push origination are ongoing metered cost. The
    cooperative is so far a *value*, not yet a *funding/governance mechanism*. Connects to item 8
    (cooperative charter) — the charter work must make "cooperative" do the sustainability work,
    not just the ethics. See `thinking/open-considerations.md`.
