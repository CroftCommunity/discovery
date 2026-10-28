# Handoff: Drystone Part 1 + Part 2 true-up pass (proposals, not edits)

You are picking up the Drystone protocol docs pass in the CroftC workspace
(`/Users/cpettet/git/chasemp/CroftC`). Read `.claude/CLAUDE.md` and `discovery/AGENTS.md` first;
the workspace rules (worktrees, claims, landing by PR, explicit staging) apply throughout.

## The goal, in the owner's words

> "do a pass on proposed changes and updates for drystone part 1 and 2 to true it up with the
> actual implementation work and progress thus far, some of it will be refinements of drystone,
> some of it may be things we didn't know to include, some of it is likely to be adding clarity
> but we want the purpose and heart to remain, we want to define drystone in requirement terms
> and the approaches that fulfill like MLS and Iroh, and we want drystone to be readable and
> understandable but still technical enough to define itself"

Read that as five commitments:

1. **True-up.** The spec was last edited 2026-08-25 (the E133/E136 amendment set). Two months
   of building have happened since. Every place the spec and the built reality disagree gets a
   proposal.
2. **Three kinds of change, each labelled.** *Refinement* (the spec said X, building taught us
   X'); *omission* (the build needed something the spec never names); *clarity* (the spec is
   right but hard to read or easy to misread). Plus *status-move* where only the evidence tag
   changes.
3. **The heart stays.** Part 1's principles (P-Local-Truth, P-Knowable-Truth, P-Peer-Equality,
   P-Durable-Enablement, the razor "compute provenance, never utility", fork-not-verdict) are the
   fixed point. A proposal may sharpen a principle's wording; it may never weaken one to fit an
   implementation shortcut. If the build departs from a principle, the proposal names the
   departure as an implementation gap, not a spec change.
4. **Requirements first, realizations second.** Drystone defines itself as requirements
   (MUST/SHOULD bars, preferring the attribute-conditioned MUST, Part 2 §6.4). MLS (openmls),
   iroh (QUIC, relay, gossip), redb, atproto/DAG-CBOR are *realizations that fulfil* those
   requirements, not the definition. Part 2 §10 ("Substrate Requirements and Reference
   Realizations") and `implementation-profile.md` already carry this split. Strengthen it and
   apply it consistently: wherever the text says "iroh does X" as if that were the protocol, the
   proposal restates it as "the protocol requires X; iroh fulfils it by Y", with the realization
   moved to §10, the profile or a clearly marked realization note.
5. **Readable and still self-defining.** A newcomer should be able to follow it, and two
   independent implementers should still interoperate from it. Lead each section with an actor
   narrative (the running-example cast), gloss every domain noun at first use, keep normative
   text precise. Clarity proposals must not trade precision for friendliness.

## The whole workspace is the evidence base, not discovery

The spec lives in discovery, but **discovery is not the only repo that refines the protocol.**
Most of what was built since August lives elsewhere. The owner stated this explicitly when the
brief was filed (2026-10-28). Treat every repo under `CroftC/` as a possible source of evidence:

- **Implementing repos, which Drystone must true up against:** `croft` (client, call-core,
  chat-core, `croft-ffi`, the openmls and iroh ports, redb), `croft-stack` (relay, admit,
  enforcement scenarios, sessions, runbooks), `CISS` (storage service, atproto identity, meter
  plane, `ciss-iroh`), `connect` (the contract, lexicons, directory).
- **Repos that may hold protocol lessons without implementing the core:** `forage`, `bluebird`,
  `croft-pwa`, `pdsview`, `view`, `regift`, and the others. Look for atproto identity and sign-in,
  record shapes, PDS walking, local-first storage and sharing. Most will turn out to be
  application layer. Record that as a disposition; do not assume it.
- **The workspace layer:** `.claude/ARCHITECTURE.md` (repo cards and seams),
  `.claude/DECISIONS.md`, `.claude/LEXICONS.md`, `.claude/DEPLOYED.md`, `.claude/TESTBED.md`.
- **The frozen `experiments/` repo** is read-only history. Its live successors are in
  `discovery/alpha/experiments/`.

Two consequences:

1. **Step 2 opens with a sweep.** Every repo under `CroftC/` gets one line in the proposals
   document: *relevant* (with what it contributes), *application layer* (why), or
   *not relevant* (why). No repo is skipped silently.
2. **The outputs reach past discovery.** Where the build falls short of the spec, the
   implementation gap is filed against the owning repo, as a pointer in that repo's `TODO.md`,
   in its own worktree and PR. Where a proposal would change another repo's docs (an
   ARCHITECTURE seam, a lexicon, the enforcement-scenario matrix, a repo CLAUDE.md that describes
   the protocol), list it under **Cross-repo follow-ons** with the owning repo named. Do not edit
   those docs in this pass; they follow the owner's decisions on the proposals. A19 still holds:
   evidence comes from the whole workspace, but the spec core stays lean.

## What this pass produces (and what it does not)

**Produces:** a reviewable proposals document, in the house format the July reconciliation used,
plus a gap matrix. **Does not:** edit `part-1-reasoning-underpinnings.md` or
`part-2-certifiable-design.md`. Part 1 and Part 2 are reviewed material. Nothing lands in them
until the owner approves proposals line by line; approved items land later as changelog entries.

Deliverables, all in a discovery worktree on one branch:

1. `discovery/beta/drystone-spec/proposed-changes-2026-10-implementation-trueup.md`. Model it on
   `proposed-changes-2026-07-experiment-reconciliation.md` (read its "How to read this"). Each
   item carries:
   - **ID** (`T-1`, `T-2`, …)
   - **Target**: Part and § and the current text, quoted verbatim.
   - **Class**: `refinement` | `omission` | `clarity` | `status-move` | `requirement/realization split`.
   - **Diff**: `-` current → `+` proposed, verbatim.
   - **Principle**: which Part 1 principle the change serves. A change that serves none is
     suspect; say so.
   - **Evidence**: file:line in the implementing repo, test name, PR number, runbook § or RUN
     summary. Code is the source of truth; another doc's claim about the code is not evidence.
   - **Evidence grade** on the A.9 ladder (`conventions-and-decisions.md` §A.9). Be explicit
     about loopback versus hardware versus production, and reference fold versus production
     code. `drystone-convergence/RESULTS.md` states it tests a *reference fold, not a
     production fold*; keep that distinction in every claim that cites it.
   - **Decision**: `ready` (mechanical) or `needs-call` (owner judgment), with the call stated as
     options in plain English.
   Group items by Part 2 section, with Part 1 items in their own group. Open the document with a
   one-screen plain-English summary: how many items of each class, and the five most consequential
   items with one line each.
2. **A gap matrix** (a section of the same document): one row per implemented capability, with
   columns for spec § · what the spec says · what is built (repo + path) · grade · gap or
   agreement. This is the backbone the proposals are drawn from. Build it first.
3. **An "omissions" section**: things built, decided or learned that the spec has no home for.
   For each, propose where it belongs: Part 2 normative, Part 2 §10 realization, the
   implementation profile, Appendix B (open), or *out of scope for Drystone* (application layer,
   which the spec deliberately excludes). Do not inflate the core; A19 in
   `discovery/alpha/ROADMAP_TODO.md` records why the core stays lean.
4. **Companion updates as proposals too**: `README.md` (status block), `EVIDENCE-MAP.md` (rows the
   new evidence earns), `implementation-profile.md` (the Croft column against what Croft actually
   runs), `open-threads.md` / Appendix B (threads the build closed or opened). These are lighter
   documents. Draft the edits in the same PR as separate commits, but mark them as contingent on
   the Part 2 decisions they follow from.

## Step 0: reconstruct and claim

- `git worktree list`, `git status --porcelain`, `ls .coordination/claims/` in CroftC and in
  discovery. Say what you found.
- Create `worktrees/drystone-trueup/discovery` on `claude/drystone-trueup`, with a `FEATURE.md`.
- If any other session holds a claim on `discovery/beta/drystone-spec`, stop and message it.

## Step 1: read the spec as it stands

Canonical, in order:

- `discovery/beta/drystone-spec/README.md`: the map and the status block.
- `part-1-reasoning-underpinnings.md` (≈1.3k lines) and `part-2-certifiable-design.md`
  (≈3.2k lines).
- `conventions-and-decisions.md`: the vocabulary discipline and the A.9/A.11 ladders.
- `implementation-profile.md`, `EVIDENCE-MAP.md`, `open-threads.md`, Part 2 Appendix B.
- `CHANGELOG.md` and `part-2-changelog.md` heads, to learn the house style for landed changes.

Not current; read only for provenance: `superseded/`,
`part-2-certifiable-design-WORKING-2026-08-16.md` (graduated into canonical 2026-08-19), the
dossier, the scenario walk, the review talkthrough. Design companions that elaborate Part 2:
`discovery/beta/impl/drystone-design/*.md`, `beta/impl/mls/`, `beta/impl/delivery-layer/`.

## Step 2: survey the built reality, across every repo

Start with the repo sweep above (`ls /Users/cpettet/git/chasemp/CroftC`, one disposition per
repo). Then survey from code and primary records, not from summaries. Starting map (verify each
row; it is a starting point, not a finding, and the sweep may add rows):

| Area | Where it lives | Spec it should true against |
|---|---|---|
| Transport: iroh QUIC, our own relay | `croft/ports/transport-iroh`, `croft/ports/call-transport-iroh`, `croft-stack/relay/source/` | Part 2 §6.1.1, §6.2.1, §6.5 (C-direct / C-relay), §6.9 |
| Admission: capability tokens, camp-at-attach, `enforce` live since 2026-08-30 | `croft-stack` (admit + relay), `croft-stack/docs/ENFORCEMENT-SCENARIOS.md`, `sessions/2026-08-30-enforce-flip.md`, `sessions/2026-08-28-e153-tokens-never-verified.md` | Part 2 §10.2.2 (the A-series admission interface, E110), §5.5 capabilities, §8 |
| Crypto majors: jsonwebtoken 11, k256/p256 0.14 (backbone v0.2.1) | `croft-stack/sessions/2026-09-24-crypto-majors-v0.2.1.md` | Part 2 §4.1, §10 realizations, profile wire pins |
| Group keying: MLS via openmls | `croft/ports/keylayer-openmls`; experiments `mls-replant`, `iroh/crates/mls-welcome-over-iroh` | Part 2 §6.1.2, §6.2.2, §5.11, §7.6.11 re-plant |
| Gossip / swarm | `iroh-gossip` in croft, croft-stack, CISS | Part 2 §6.5.3, §6.10 |
| Local storage: redb | `croft/ports/store-redb`, `beta/impl/drystone-design/redb-storage-contract.md` | Part 2 §4.6 (free in local storage), §10 |
| Calling and chat as one stream; call-core shared through `croft-ffi` to Android and macOS | `croft/core/call-core`, `croft/core/chat-core`, `croft/ffi`, croft `plans/2026-09-08-plan-one-stream-calling-and-chat.md`, runbook §15–§17 | Part 2 §6.8 payload classes, §6.6.5 device-Group, §4.5 multi-client |
| Session state: credentials vs live session, scheduled refresh (plan ACCEPTED, build not started) | croft `plans/2026-09-28-plan-session-state.md` | Part 2 §5.2 identity model, §6.7 presence; likely an omission or out-of-scope call |
| Relay flap: the Pixel reconnects at rest (reason not yet named) | croft runbook §17 "The flap, RUN" | Part 2 §6.9, §6.4 metadata floor; record as evidence of a gap, not a spec change |
| Storage service and identity: CISS (atproto identity v0.3.0, `did:` meter plane v0.10.0, `ciss-iroh`) | `CISS/` | Part 2 §6.6.2 D-meer, §6.8.5 history store, §10 |
| Meer as the CISS custodian queue | `discovery/alpha/experiments/meer-queue/` (STATE-AND-NEXT, S8-RESULTS), the memory note pointed at by `.claude/DECISIONS.md` | Part 2 §6.6.2 D-meer |
| Record types as lexicons (`ing.croft.*`, unpublished; E161 open questions) | `connect/lexicons/`, connect #10 | Part 2 §9 interop, `dag-cbor-and-content-addressing.md` |
| Governance fold: convergence harness, R1–R4 (+ A12) properties | `discovery/alpha/experiments/drystone-convergence/RESULTS.md` (discovery #67), MASTER-INDEX row A7 | Part 2 §7.3.1, Appendix B beam, `open-threads.md` priority shape |
| Delivery-layer experiments R1–R3 | `beta/impl/delivery-layer/05–07-*Results*.md` | Part 2 §6, §11.11 measurements |
| atproto history backend: CID as reconciliation head | `hist-atproto-spike`, `spike/hist_live/HIST-LIVE-RESULTS.md` (B16) | Part 2 §6.8.5, `dag-cbor-and-content-addressing.md` |
| Other seams | `discovery/alpha/experiments/{group-principal-seam,helper-seam,replant-continuity,tier-proof}` | Part 2 §5.10, §3.2 helper cast, §7.6.11 |

Also read `.claude/ARCHITECTURE.md` (repo cards and seams), `.claude/DEPLOYED.md` (what is
actually running) and `discovery/alpha/ROADMAP_TODO.md` rows A11, A12, A19, E30, E31, E110–E112,
E133, E136, E161, E162. A11 (capability mechanism Track A vs Track B) and A12 (key custody) are
exactly the "requirement vs realization" seams; the build may have decided one de facto. If so,
that is a `needs-call` proposal, never a silent fold.

Fan-out is allowed: the survey splits naturally by row group (transport + admission;
MLS + storage + CISS; calling + session; governance + experiments). Spawn read-only subagents
with `model: "opus"` set explicitly. Give each its rows, the spec sections to compare, and the
evidence rules above. Have each return gap-matrix rows with file:line evidence. Verify a sample
of each agent's rows yourself before using them.

## Step 3: write the proposals

Rules while drafting:

- **Requirement voice in Part 2 normative text.** "A node MUST …" / "where attribute X holds, a
  node MUST …". A named library appears in normative text only as "for example". Realization
  detail moves to §10, the profile, or a marked realization note.
- **Status honesty.** Never raise a tag past what the evidence earns. A loopback measurement is
  not a hardware one. The reference fold is not the production fold (none exists yet; say so if
  Part 2 implies otherwise). A device run on two phones is `Measured` at that scope, no wider.
  Where Part 2 is *ahead* of the build, that is fine. A spec may lead an implementation, so
  only flag it when the text claims the build exists.
- **Implementation gaps are not spec changes.** Where the build falls short of a MUST, list it in
  a separate "implementation gaps" section and point it at the owning repo's TODO. Do not
  propose loosening the MUST unless the build revealed the MUST was wrong. Then it is a
  `needs-call` refinement and says why.
- **Readability.** Clarity proposals lead with the actor narrative (the §3.1 running example and
  Appendix E's cast) and gloss every coined term once (Appendix D is the term lattice; a new term
  needs a D entry). Keep Part 1 the "why" and Part 2 the "what". A clarity edit must not move
  reasoning into Part 2 or mechanics into Part 1.
- **Name layer boundaries.** Transport plane vs group (MLS) plane vs governance fold vs
  application. "Bob is cut off" must say which layer. The relay refusing an attach (admission)
  and a Group revoking a member (governance) are different events with different recovery.
- **Quote verbatim.** Every `-` line is copied from the file, and every owner quote is verbatim.

## Step 4: land the proposals (not the spec)

- Commit in small, explicit-path commits (`docs(drystone): …`), then rebase onto origin/main.
- Open a PR on discovery titled `drystone: implementation true-up proposals (2026-10)`. The body
  holds the plain-English summary and the count by class. Push and open without asking.
- **Ask before merging.** Merging the proposals document does not apply them.
- Implementation-gap pointers in other repos' `TODO.md` each get their own worktree and PR in that
  repo, opened without asking, merged only on approval.
- Report back with: the PR link (and any cross-repo TODO PRs); the repo sweep's dispositions; the five `needs-call` items that matter most, each as a decision
  with options in plain English; the implementation gaps found, each pointed at its repo; and
  anything you could not resolve, with what blocked it.

## Out of scope

- Editing Part 1 or Part 2 text (later pass, after approval).
- Application-layer product decisions (Croft app UX, the session-state build, the v0.6.0 cut).
- The workspace-wide advisory triage (croft-stack `TODO.md` § Open). It is a separate session.
- Publication mechanics (Zenodo DOI, `[gates-release]` wire encodings), except to note where
  the build has pinned an encoding the spec still marks open.

## Done means

A reader who knows nothing about the last two months can open the proposals document and see,
section by section, where Drystone stands against what was built. Each change shows its class,
its evidence and the principle it serves. The owner can approve or reject each item on its own,
and approving them all leaves Drystone fully stated as requirements, with MLS and iroh in their
place as the realizations that fulfil it.
