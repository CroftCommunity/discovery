# Corpus Analysis: GroupDynamics Seeds

date: 2026-06-15

author: analysis pass over the GroupDynamics.zip artifacts + the design dialogue transcript

purpose: map what the seeds contain, extract the through-lines, and flag the gaps before we build the separated document sets (research / thinking / crystallized / roadmap)

---

## 1. What is in the corpus

Five source artifacts, two kinds:

```
RESEARCH (the field, "theirs")
  └─ messaging-solutions-landscape.md   ~6,900 words, 4 comparison tables
        Signal · Delta Chat · SSB · Matrix · Briar · Session · WhatsApp/TG
        verified-current (June 2026), protocol-vs-product separated

THINKING (our design, "ours")
  ├─ thesis-lineage-groups.md           the core protocol thesis + 10 invariants
  │                                     + 3-phase experiment plan (E1.* / E2.* / E3.*)
  ├─ multi-device.md                    per-device-key model + Phase 2.5 (E2.9–E2.16)
  └─ social-layer.md                    the graph-you-hold layer (S1–S4 invariants)

SEED (the raw source the above were distilled from)
  └─ the design dialogue transcript (2026-06-13 to 06-14)
        + 2 prompts it generated but never turned into docs:
          - achilles-heel-research-prompt.md   (failure-mode research, unrun)
          - structural-tests-visibility-prompt.md (V1–V9 tests, unrun)
```

The four documents are already well-formed. The research doc is honest and current. The
thinking docs are coherent. This is not a pile of notes — it is a near-complete first
generation of separated outputs. The job now is less "create order from chaos" and more
"distill the recurring spine, close the gaps, and sequence toward build."

## 2. The spine: one thesis, restated three times

The whole corpus converges on a single claim, expressed at three altitudes. This is the
through-line worth promoting to a principle, because the dialogue arrives at it
independently more than once:

> Knowing reliably where something came from (provenance) is simultaneously the security
> invariant and the social-legibility invariant. They are the same fact with two payoffs.

- At the **protocol** altitude (thesis): model a group as a navigable lineage of
  conversations, not a flat eternal room. Cryptographic provenance is what makes both the
  security guarantees and the fluid social model possible.

- At the **multi-device** altitude: a person is a DID lineage; each device is a distinct
  key/member; "same person" is recovered one layer up (presentation fold + lineage-counted
  thresholds), not forced into the key layer. Keys are not identity.

- At the **social** altitude: you hold the graph of your life rather than having it held
  about you. The absence of an extraction model is not a missing feature — it is the point,
  and it is what lets the graph be honest.

A recurring meta-observation in the dialogue, stated explicitly three times: **the ethical
choice and the technical strength keep turning out to be the same choice.** Refusing
extraction forces decentralization; decentralization is what delivers resilience and
privacy. Worth stating as a first-class principle rather than a happy accident.

## 3. The architecture, in one frame

```
                 ┌─────────────────────────────────────────────┐
   SOCIAL LAYER  │ graph you hold · label layer (opt-in) ·       │  separate threat model
   (rides on top)│ visibility regimes · propagation geometry     │  (inside adversary,
                 └─────────────────────────────────────────────┘   topology deanonymization)
                                    ▲
   MULTI-DEVICE   per-device keys under one DID lineage; fold + lineage-counted thresholds
                                    ▲
   PROTOCOL       two trees bound together:
                    governance tree (signed op log, forks & heals)  ─┐
                    MLS epoch chain (single, linear, cannot merge)  ─┘ bound by:
                    "a governance op is enacted only once it is an MLS commit"
                                    ▲
   SUBSTRATE      iroh P2P · optional blind superpeer broker · Automerge CRDT · iroh-blobs
```

The entire design lives in **the binding between the two trees** and in **what happens at
reconnect** (survivor-epoch selection, conflict hard-stop, fork-as-feature). That is
correctly identified in the thesis as the riskiest seam and the thing Phase 1 must de-risk
first.

## 4. The proofs (what is meant to be validated, and how)

The thinking docs already carry a falsifiable backbone. Consolidated count:

- **10 protocol invariants** I1–I10 (genesis immutability, threshold soundness, provenance,
  forward-key linearity, deterministic survivor, no silent contradiction, history never
  corrupts, backfill verifiability, fold lossless, convergence).

- **Phase 1 experiments** E1.1–E1.4 — the crypto-feasibility gate. The single most
  important result: can openmls express "pick a survivor epoch, re-key the other side, or
  mint a third" with PCS intact, via external commits + reinit.

- **Phase 2 experiments** E2.1–E2.8 — the data-model/merge core (fork/heal, conflict
  hard-stop, backfill provenance, the explicit anti-regression test E2.8 against timestamp
  interleaving).

- **Phase 2.5 experiments** E2.9–E2.16 — multi-device (lineage fold, lineage-counted
  thresholds, device revocation, self-sync-as-backfill, leave-one-vs-leave-all). Gated on
  E2.10 (no quorum manufacture from own devices) and E2.12 (serverless self-sync).

- **Phase 3 experiments** E3.1–E3.4 — thin slice over real iroh, including the
  device-loss/recovery probe E3.3 and the blind-broker assertion E3.4.

- **Social-layer tests** V1–V9 — visibility regimes and propagation geometry. **Drafted as
  a prompt, not yet folded into any spec.** V5 and V8 (deanonymization defenses under a
  hostile sender) are the gate.

## 5. The principles (design + otherwise) the dialogue settled

These recur and read as decided, not tentative:

- **Provenance is the dual-purpose primitive** (section 2 above).

- **Forward-key convergence ≠ history reconciliation.** Never conflate them. The key is
  single and linear; history is data and never merges into one transcript. No timestamp
  interleaving ("six tapes in a room").

- **Forks are a feature.** Under partition, contradictory-but-valid commits are inevitable;
  the right resolution is a clean, attributable, non-insulting fork. Heal silently on no
  conflict; hard-stop and escalate to a human on real conflict. Never algorithmically
  adjudicate a social dispute.

- **Immutable genesis grounds the recursion.** The group's "constitution" (per-operation
  thresholds) is fixed at birth and is not itself renegotiable by the normal machinery.
  This is what stops "who decides who decides" turtles.

- **Per-operation thresholds tracking blast radius.** leave-self=1, add=low, boot=higher,
  dissolve=highest. Lenient no-surprise defaults; strictness behind an advanced menu or a
  single "how strict?" question.

- **Keys are not identity; thresholds count lineages, not leaves.** The defense against
  manufacturing a quorum from your own devices.

- **Fail early, fail clearly.** Stale must be visible; unavailable and murky are not
  allowed. Degradation (no-broker tier) must be surfaced, never silent.

- **Structural, not runtime, enforcement for safety limits.** A violating share should be
  unrepresentable / rejected by every verifier — never merely warned against on the
  sender's client, because the safety case is exactly the hostile sender.

- **Layer separation makes the social graph safe.** Structural graph vs opt-in label layer;
  most platforms fuse them and that fusion is the invasiveness.

- **Freeze by default; quiet membership; multi-identity as a fact of life** (S1–S4).

- **Content is born into a visibility regime and cannot silently change it.** Crossing
  intimate→public is a deliberate new authored republish, never a forward.

- **Openness caps propagation depth.** A large public group is a visibility sink, not a
  conduit; inward visibility and outward propagation are independent parameters.

## 6. The gaps (highest-value, address before building further)

1. **social-layer.md is incomplete relative to the dialogue.** Its header says it "covers
   the scale axis of broadcast/topic/civic groups (sections 8–10)" — but the body stops at
   section 7. The two-regime model, the born-into-a-regime rule, propagation geometry
   (outward depth vs inward visibility), and the civic-notice use case were all developed
   in the transcript and then the update was interrupted. **Sections 8–10 exist only in the
   transcript and in the V1–V9 test prompt.** This is the clearest unfinished business.

2. **Two generated prompts were never run or filed.** The Achilles-heel research prompt and
   the V1–V9 structural-test prompt are now preserved in `seeds/generated-prompts/`. Neither
   has been executed. The Achilles-heel research is explicitly designed to attack our model;
   running it is the honest next adversarial step.

3. **Two open questions are flagged as unsolved in the docs themselves and are load-bearing:**
   - **Total-device-loss recovery.** No anchor chosen (mnemonic seed vs social-recovery
     quorum vs broker-held encrypted backup). Named the single largest residual risk.
   - **The ordering/consensus "dirty secret."** Is the superpeer secretly the MLS ordering
     service, making the "decentralized" claim weaker than stated? The dialogue raises it;
     no doc resolves it. The Achilles-heel prompt targets it directly.

4. **One unverified protocol dependency the whole design leans on:** openmls must let a
   lineage-proving credential ride on the MLS leaf (for the fold and lineage-counted
   thresholds), and must support external-commit + reinit for survivor re-key. External
   commit is confirmed present; the graceful re-key flow and the leaf credential are
   "verify against the real library first" items.

5. **No single roadmap exists.** The phase plan lives inside thesis.md; multi-device adds
   Phase 2.5; the social layer adds an unsequenced V-series; recovery and the adversarial
   research are unscheduled. There is no one document sequencing the whole logical flow.

## 7. Proposed document sets (the separation you asked for)

```
discovery/
├── seeds/                         raw, immutable source
│   ├── groupdynamics-unpacked/    the 4 produced docs as delivered
│   └── generated-prompts/         the 2 unrun prompts (preserved)
├── research/                      the field (theirs)
│   └── messaging-solutions-landscape.md
├── thinking/                      our design (ours), evolving
│   ├── thesis-lineage-groups.md
│   ├── multi-device.md
│   └── social-layer.md            ← needs sections 8–10 added
├── crystallized/                  the distilled spine (to build)
│   ├── principles.md              design + otherwise (section 5 above, expanded)
│   ├── proofs.md                  the I/E/V ledger as one table with status
│   └── conclusions.md             settled vs open, with the honest open-risk list
└── ROADMAP.md                     one sequenced logical flow toward build
```

Raw seeds stay frozen; `thinking/` evolves; `crystallized/` is the new distilled value.

## 8. Recommended next steps, in order

1. **Close the social-layer gap** — write sections 8–10 (visibility regimes, propagation
   geometry, civic/broadcast scale axis) into `thinking/social-layer.md` from the transcript,
   so the doc matches its own header and the V1–V9 tests have a spec to reference.

2. **Build the crystallized set** — `principles.md`, `proofs.md` (the unified I/E/V ledger),
   `conclusions.md` (settled vs the two big open risks: recovery, ordering-authority).

3. **Write ROADMAP.md** — one sequenced flow: Phase 0→1 (crypto gate) → 2 → 2.5 → 3, with
   the adversarial research and recovery-anchor decision slotted as explicit gates.

4. **Decide the two open questions** (recovery anchor; whether the broker is the de-facto
   ordering authority) — these are design decisions the experiments cannot make for us, and
   the Achilles-heel research is the input for them.
