# Proposed Part 1 / Part 2 changes — implementation true-up (2026-10, for review)

`Status: REVIEWED, not applied. The owner walked all 46 items on 2026-10-29 and 2026-11-06; the
decisions are in § Owner decisions below and supersede every per-item "Decision" line. Part 1 and
Part 2 are still untouched: the application pass that edits them, the companions and the owning
repos' TODOs follows this document and lands separately, each accepted item as a changelog entry
(house style: "§X, done. <what> (evidence)"). Two threads are held open for discussion before
their items are applied (retention, T7). Brief:
alpha/seeds/generated-prompts/drystone-implementation-trueup-prompt.md.`

`Source of every claim below: the code, tests, runbooks, session files and experiment results in
the CroftC workspace as of 2026-10-28, cited by path and line. Another document's claim about the
code is not evidence here; where a claim could not be resolved to code it says so (§7).`

## Plain-English summary

The spec was last edited on 2026-08-25. Since then the system it describes has been built and run:
the relay enforces admission in production, the backbone crossed its crypto majors, the Android
app and a macOS shell run calling and chat on one shared Rust core over our own relay, the
convergence harness ran the governance fold's properties, and the record types the clients
exchange were written down as lexicons. This document walks Part 1 and Part 2 against all of
that and proposes, item by item, what should change.

Three kinds of change appear, each labelled: a **refinement** (the spec said X; building taught us
X′), an **omission** (the build needed something the spec never names), and a **clarity** fix (the
spec is right but hard to read or easy to misread). Two more labels cover the mechanical cases: a
**status-move** (only the evidence tag changes) and a **requirement/realization split** (the text
states a library's behaviour as if it were the protocol; the proposal restates the requirement and
moves the library to §10 or the profile).

The heart stays. Part 1's principles are the fixed point of this pass: no item below weakens a
principle to fit an implementation shortcut. Where the build falls short of a MUST, that is filed
against the repo (§5), not against the spec.

**The count.** Forty-six proposals: by primary class, 17 clarity, 10 omission, 7 status-move,
6 refinement, 3 requirement/realization splits, 3 that are chiefly a filing against a repo;
29 are `ready` and 17 carry a `needs-call`. Separately, 17 implementation gaps are filed against
five repos (§5), 11 facts the spec has no home for are placed (§3), and 11 questions could not be
resolved from the tree (§8). None of the 46 weakens a Part 1 principle; two (T-4-4, T-7-5)
*widen* a MUST from an algorithm to the property it was protecting.

**The five that matter most, one line each.**

1. **A production governance fold exists and the evidence set says none does** (T-E-1, T-7-3).
   `croft/core/social-tree-core` has shipped the fold since 2026-08-23; the convergence results,
   the evidence map and Appendix B were written as if the only fold were the harness's reference
   one. The map has no row for any of ten test files; §7.3.1 earns `Modeled`, never `Verified`.
2. **"Admission" is two events** (T-6-2, T-10-1, T-8-1). The relay admits an endpoint on a
   wall-clock lease; a Group admits a lineage on chain standing. Until the spec names the first,
   it cannot say whether Croft's enforcing relay is conformant with T7 ("no wall-clock for any
   authority-bearing decision") — the needs-call is whether T7 means Group standing or every layer.
3. **The §4.6 MUST names a codec the production fold does not use** (T-4-4). Croft hashes a
   hand-rolled fixed layout; dag-cbor lives in the history-store spike and CISS. The proposal makes
   determinism the requirement and dag-cbor the reference; the owner picks which encoding the
   wire-freeze pins.
4. **§7.3.2 contradicts itself on mutual expulsion, and the build has chosen** (T-7-4). Line 1056
   says the tiebreak decides; line 1066, §7.6.1 and the fold all say `CONTESTED`. Appendix E
   follows the wrong one.
5. **The profile and four `Verified` tags overclaim** (T-C-1, T-6-8). Croft's column declares the
   outer seal ON and a meer deployed; neither exists outside a spike. Four §6 tags rest on
   mls-rs-era crates that are not in the tree.

**What was not done.** Part 1 and Part 2 were not edited. No repo's `TODO.md` was touched; the
gap pointers in §5 are written here for the owner to approve before they travel. The
primary-source `[confirm]` items (Lamport, CAP, Ostrom, Beer) were out of scope and are
unchanged.

## Owner decisions (walk of 2026-10-29 and 2026-11-06)

One line per item, in walk order. Where the owner's words carry design intent they are quoted
verbatim. These supersede the `Decision` field on each proposal below; the per-item text is kept
as the record of what was proposed and why.

**Outcome in one line:** 36 items accepted as proposed or with a named option; 8 reshaped by the
owner into something the proposal had not drafted (items 15, 21, 23, 29, 33, 36, 38, 41, 43);
2 held open for discussion (27 retention, 39 T7); 1 gap withdrawn (36) and 1 held (39). No Part 1
principle was weakened; three MUSTs widened from an algorithm or a codec to the property
(T-4-4, T-7-5, T-8-1); one MUST added (T-7-2, the first product-surface MUST in Part 2); one
new body-level concept named (the **published record**, T-9-1); two design threads opened outside
the spec (identity flows from the persona; the public regime past the confidentiality ceiling).

1. **T-P1-1** → (a) fix the Part 1 §0 map line to match §1.2's body.
2. **T-R-1** → (a) the README teaches the A.9 ladder; legacy words alpha-tier only.
3. **T-R-2** → (a) replace the README glossary rows with the A.1–A.8 one-liners plus the Appendix D pointer.
4. **T-R-3** → (a) README status restated as of 2026-08-25, "beta"; sequencing box fixed.
5. **T-R-4** → (a) four August changelog entries (`2c310c2`, `3a78a02`, `6d3b740`, `ebf4327`) plus one for this true-up.
6. **T-R-5** → (b) rule change only; **no version numbers in the README**; pins live on the profile sheet.
7. **T-3-1** → (a) tag the §3.2 read-helper bullet `Verified` (`helper_seam.rs`, RUN-14 EXP-C) and add the EM row.
8. **T-4-1** → (a) profile line 138 corrected (BLAKE3 in production, vectors not re-run); §4.1 clause added; croft gap filed (§9 fold vectors).
9. **T-4-2** → (a) Appendix B: brand tags still minted, registered in croft `WIRE-REGISTER.md`; register-on-mint rule (croft follow-on).
10. **T-4-3** → (b) spec stays three-role; profile row names the two fields croft carries; Appendix D note; **plus a croft TODO to rename `DeviceId` to the spec's "client"** across crates and the FFI.
11. **T-4-4** → (a) the §4.6 MUST becomes determinism; dag-cbor is the reference realization; **croft migrates to dag-cbor at the wire-freeze** (croft TODO). The fixed layout is a conforming realization until then.
12. **T-5-1** → (b) the spec says nothing about directory binding; croft is non-conformant on §4.5 until the lineage work lands. The croft filing carries the owner's framing verbatim: *"the directory binding to me seems like a consequence of the broader client to persona assertion, and is rooted in a persona have root keypair that signs the client and can be derived, in our design thinking in current croft I think we've under represented that and had keys in a few contexts but haven't put enough focus on establishing initial self signed identity, you assert you are you, then others make the utility decision about whether it is true, then clients can prove they are descended from that persona root key pair … we need to do more work here but I don't want to pollute the spec at this point."*
13. **T-5-2** → (c) §5.2 unchanged. Direction recorded outside the spec (item 42's plan): the persona root is a user-generated keypair and the one backup; **the root signs each client credential** (owner chose signing over derivation: *"signing is the better version, the point really is to have the one persona one vote be the backstop"*); the ATProto DID maps to the root through a did:webvh document; DID = locator, root = standing.
14. **T-5-3** → (a) §5.10 carries the RUN-11 evidence the EM already has.
15. **T-6-1** → reshaped: §6.9.2 lists **five** mechanisms, each marked realized / not realized by the reference (*"we have 5 options defined, 2 of which are realized in current code so let's represent that way"*). Realized: out-of-band rendezvous; the published record (item 38). Not realized: DNS/Pkarr, Pkarr-on-DHT, mDNS. Body wording ecosystem-free; the ATProto specifics, the reverse-binding MAY and the `N0DisableRelay` pkarr trap go to §10.3.
16. **T-6-2** → (b) Appendix D: fabric admission vs Group admission; §10.2.2 "Group admission —"; §6.5.2 gains the MAY sentence.
17. **T-6-3** → (a) §6.1.1: the handshake-proven key is the only safe binding point for a fabric credential.
18. **T-6-4** → (b) profile `[deployment]` row only ("one endpoint per entry, never the pair", ADR-0006); §6.4 body unchanged.
19. **T-6-5** → (b) §6.5.2 stays SHOULD; split into the two mechanisms with the "never sell reachability as standing" rationale; croft declares flat on the profile.
20. **T-6-6** → (a) §6.2.1 parenthetical on which TLS the relay terminates.
21. **T-6-7** → reshaped: a **three-state send seam** in §6.10.4 — (1) refused/failed; (2) accepted with ≥1 neighbour, still no delivery ack; (3) **swarm empty**: accepted, zero neighbours, nothing left the device, and the sender MUST treat it as not sent (surface it, or hand the bytes to another carriage per §6.6). Owner: *"swarm is one transport backplane … it seems like a third state: swarm empty."* Events pinned to iroh-gossip 0.101.0; view sizes and PRUNE/GRAFT stay `[confirm]`. Croft check: is state 3 surfaced to the user or only logged?
22. **T-6-8** → (a) **and** (b): retag now (§6.6.1, §6.6.3, §6.8.3 → `Measured` report-only; §6.7's push half split out as `Design`), and file the re-earn on the openmls stack as a harness-extension item.
23. **T-6-9** → split and retag per item 22, **and** §6.6.5 enumerates the device fan-out models as mutually inclusive, profile declares (*"enumerate delivery models here … poll from a device that comes up to do a range query update seems to need inclusion as these seem like mutually inclusive options and we should define them and then the implementation layer chooses"*): (1) race delivery; (2) deliver-once via device-Group; (3) pull-on-wake via the §6.8.1 range query against a sibling or the meer. S4/E92 becomes "which of these, declared".
24. **T-6-10** → (a) §6.8.4 fifth class **session control** (real-time, never durable, loss visible); the "one stream ≠ one lane" note.
25. **T-6-11** → (a) §6.2.2: the exporter is used for the outer-seal key and the per-epoch queue name.
26. **T-6-12** → (a) dedup on the outermost sealed bytes MUST precede MLS; croft gap filed.
27. **T-6-13** → (b) the retention sentence and the CISS gap; **held open for discussion** (*"yes I think this needs more discussion"*): must the meer be able to erase; who ends holding; what the user is told about sealed bytes still held where they cannot reach.
28. **T-6-14** → (a) with A.9 tags on every claim: the two-target shape `Measured` (spike, loopback); "meer = CISS custodian-queue mode" `Design`; profile Deployment row corrected.
29. **T-6-15** → reshaped into a requirement/realization split (*"the history range consolidation helper is more or less the same ends through different realization here so maybe both modes have their place"*): §6.8.5 states the requirement, then names the G-hist mirror group (`Design`/`Synthesis`), the PDS-repo store (`Modeled`) and the CISS consumer (`Design`) as realizations served through §6.8.1; the profile declares which.
30. **T-7-1** → (b) fold §7.6.11 into §7.6.2 with per-experiment grades and **delete §7.6.11**; fix the map and cross-references.
31. **T-7-2** → (b) **MUST**: a client MUST NOT present an identity, reachability or admission state it has not confirmed against a live signal. The first product-surface MUST in Part 2; say so in the changelog.
32. **T-7-3** → (a) §7.3.1 `Design` → `Modeled` with bounds; tiebreak `Measured` on the production fold.
33. **T-7-4** → reshaped: **fork, not verdict.** Mutual expulsion is the canonical contradiction; both project `CONTESTED`; two exits, neither a hash — a `Resolution` by the Group, or the fork, where the lineages diverge and each member follows either or both. The tiebreak never applies to a §7.6.1 shape. One added sentence: **which lineage a member follows after a fork is that member's local policy**, never a protocol decision (*"Alice is my wife, any split I go with her"*; *"drop any group left without my connections"*). Owner: *"we've talked about how conflict is adjudicated and the really clear answer is ..it's not, it's intended or directed by 'rules'."* The examples live in `alpha/thinking/the-shape-of-disagreement.md:118,145,175`, which §7.6.2 already cites. Appendix E E5/E6 walk the fork. Croft check: does croft project the fork or only hold `CONTESTED`?
34. **T-7-5** → (a) outcome-form MUST, **with both conforming realizations named as examples** (the harness's tiered pass; croft's replay plus projection plus hard-stop).
35. **T-7-6** → (a) croft TODO ("not found", not "absent"); harness backlog row.
36. **T-8-1** → reworked (*"can't prove a negative … the intent is no helper can see user content without explicit authorization and the assumed is a structural falsity so it's not a question of desire but ability"*): a Tier-0 helper is **blind by construction** — content is sealed to Group membership and the helper holds no leaf, so no key exists to hold; a helper MUST NOT be granted a leaf except by an explicit chain-visible Group act (§3.2's read helper is that case); members can check the absence against the tree; `payload_keys_held = 0` is hygiene, never the proof. **The croft-stack gap is withdrawn.**
37. **T-8-2** → (a) two refusal-verbosity rows on the profile.
38. **T-9-1** → reshaped: a **new body-level concept, the "published record"** (owner's noun; not "directory": *"it's more about ONE user than it is about FINDING users"*). New §6.9.3, ecosystem-free: a persona publishes a record about itself, under its own key, at a host it chooses, resolvable from a human name; integrity is the key; observation and withholding are the host's, a soft and replaceable center; the reader fails closed (malformed ≠ absent). Resolution (§6.9) is reading someone's published record. §10.3 names the realizations: the ATProto PDS as reference (contract v2, three `ing.croft.*` types, lexicons unpublished), a did:webvh document, a signed `.well-known` file. §4.6: published records are not Drystone facts. Appendix D term. Follow-ons adopt the noun.
39. **T-10-1** → **held open for discussion.** Owner: *"no wall clock for authority bearing decisions...between nodes in a distributed system … a device will have to make decisions based on its own local clock almost no matter what we do … these are not decisions about authority, they do effect 'voice' … not the authority of the voice or the validity, only the mechanics of the exchange."* Two rules T7 collapsed: between nodes, never (standing, ordering, validity); within a node, the local clock as a private input to mechanics only, few and declared, affecting voice never authority. §7.4 line 528 already says this for freshness. The camping pass is a two-clock comparison between helpers in one deployment, fabric plane, so rule one must be scoped to the Group plane. Draft T7 text is in the open-threads note below. The croft-stack gap is held until the wording settles.
40. **T-10-2** → (a) all realization notes, tags per item.
41. **T-A11** → split (*"isn't this two things, capabilities for a group principal and capabilities for a persona?"*). **A11a persona-issued** (one issuer, one key; revocation is the issuer deleting or rotating its own grant): Track A de facto; realizations are the call grant in the published record and the fabric camping pass; the lifetime bound waits on T7. **A11b Group-issued** (issuer is a quorum; revocation is a governed act that survives an authority change): open; the only evidence is the §5.10 seam; Appendix A's prerequisite stands.
42. **T-A12** → (a) **Option A ratified**, grounded in item 13 (the root is the backup). Floor set by the owner: *"at a bare minimum we need a way to export … a protected archive that can be stored away and reimported. That's the baseline for disaster recovery … if that doesn't exist it needs to go high up on the list."* Croft gap, **high priority**: the `KeyCustody` seam designed around root export and import; the archive carries the root and the signed identity mappings. **Plus a new design thread**, a discovery plan "identity flows from the persona": the persona is its own self-signed root, not the ATProto identity; external identities are mapped to it by signed records; "same persona across N platforms?" is answerable by following the mappings to one root; trust is attested by other personas at their comfort level; backup and restore carry root and mappings together. Owner: *"identity at the core flows from the persona and that identity is effectively self-signed and then attested at the comfort level of other personas … we sort of got away from that in the experimentation phases and now we really need to pull it back together."* Items 12 and 13 hang off this plan.
43. **T-11-1** → reshaped: keep §11.9.1 as the **public-regime variant, tagged `Synthesis`**, exploratory (*"I wouldn't consider it to be defunct … anticipatory thinking … exploratory for now"*), rewritten to the current thinking: two limits at scale (the TreeKEM ceiling; confidentiality stops being a real presumption at large audience) and two moves taken together — keep the MLS packaging and drop the encryption (the confidentiality regime flips to public), and move carriage to broadcast and rebroadcast, firehose-shaped, which may itself be permissioned (*"a million person company could federate a bluesky … or pay for someone to do it"*). Goal: seamless from two people to two million; the threshold is unknown. Cross-reference fixed to §6.2.2, which gains one sentence.
44. **T-B-1** → (a) **plus** the Appendix B sentence names the handling mechanisms the spec already holds (§7.3.5 ceiling; §7.3.8 finality gate, which binds only irreversible governance acts and never the content plane; §7.4 freshness; §7.4.3 stamp; the corroboration dials) and states that the open obligation is **Appendix B (2), the liveness-under-partition argument for the fail-closed stall**, not detection, which the harness reproduced. Owner: *"demonstrating it doesn't discharge it … governance degrades gracefully and the messaging side is eventually consistent is fine … do I have enough to make a safe decision."* Harness backlog: a stage for the stall's liveness under partition.
45. **T-E-1** → (a) RESULTS.md sentence; ten EM rows; row 46 repointed; A7 retired.
46. **T-F-1** → (a) Negentropy entry updated; openmls row added; group retitled.

**Roll-ups implied by the above:** Appendix D gains fabric/Group admission, session control, published record, the A11a/A11b issuer line, and the DeviceId-is-client note; Appendix E E5/E6 walk the fork and the departure-versus-ban shape; the profile table changes per §6.1 below with the new rows items 10, 18, 19, 37, 38 and 40 call for.

**Open threads, to take up before the application pass touches their items.**

- *Retention (item 27).* Must the meer be able to erase; who ends holding (sender, recipient, Group policy, operator); what the user is told about sealed bytes still held on a store they cannot reach.
- *T7 wall-clock (item 39).* Draft for discussion: "Not require a wall-clock shared or compared between nodes for any decision that bears on Group standing, ordering, or the validity of a fact (Part 1 §2.0.1). A node MAY use its own clock as a private input to the mechanics of exchange (leases, timeouts, freshness windows, retry); such uses MUST be few, declared on the profile, and able to affect only whether and when a node is heard, never the authority or validity of what it says. The fabric plane MAY compare clocks across helpers for reachability leases (§6.5.2), never for a Group act."

**New work the walk created, by home.** Discovery: the plan "identity flows from the persona" (item 42); the §6.9.3 published-record subsection and the §11.9.1 rewrite (application pass); harness backlog rows for the re-earn (22), the closure (35) and the stall's liveness (44). Croft: `DeviceId` rename (10); dag-cbor migration at the freeze (11); persona root and signed client credentials (12, 13); swarm-empty surfacing check (21); dedup before MLS (26); fork projection check (33); conflicted-subgraph closure (35); the protected archive, high priority (42); plus the §5 rows unchanged. Croft-stack: the §8 row withdrawn; the T7 row held. CISS: retention and erasure, pending item 27. Connect and CroftC: adopt "published record" (38).

## How to read this

Each proposal carries:

- **ID** — `T-<area>-<n>`. Areas: `P1` (Part 1), `R` (the spec's front door: README and the
  filing logs), `4`…`11` (the Part 2 section), `A`…`D` (appendices), `C` (companions: profile,
  evidence map, open threads).
- **Target** — the Part and § and the current text, quoted verbatim with its line number.
- **Class** — `refinement` · `omission` · `clarity` · `status-move` · `req/realization split`.
- **Diff** — `-` current → `+` proposed, verbatim.
- **Principle** — the Part 1 principle the change serves. A clarity item that serves none says
  "readability" and names the rule it restores (the map matches the body; one vocabulary of record).
- **Evidence** — path:line, test name, PR number, runbook § or RUN summary.
- **Grade** — the A.9 rung the evidence earns, with its bound stated (loopback / LAN / LTE /
  production; reference fold / production code).
- **Decision** — `ready` (mechanical, low-risk) or `needs-call` (owner judgment), with the call
  stated as options in plain English.

The proposals are independent; approve any subset.

---

## 0. The repo sweep: where the evidence came from

Every repo under `CroftC/` was read for this pass. One line each: what it contributes to the
protocol, or why it does not. Four repos implement or constrain it; seven are application layer
with a lesson the spec should learn from; the rest are not relevant.

| Repo | Disposition | What it contributes, or why not | Pointers |
|---|---|---|---|
| **discovery** | implements (the spec's home) | The spec, the evidence map, the convergence harness (a faithful *reference* fold of R1–R4 with permutation-invariance proptests), and the live experiments: mls-replant, replant-continuity, croft-group, meer-queue, local_storage_projection (the redb-adapter lineage). | `beta/drystone-spec/`; `alpha/experiments/drystone-convergence/RESULTS.md:1-30`; `alpha/experiments/meer-queue/` |
| **croft** | implements | **A production governance fold**: `core/social-tree-core` (the fold and its projections, the A-series admission cross-check, charter thresholds, `CONTESTED`, the `Resolution` fact, a versioned wire register) — extracted at E117 Phase 2 from the mutation-vetted `local_storage_projection` experiment, 64 tests, linked into the shipping app through `croft-ffi` together with `chat-core`, `store-redb`, `keylayer-openmls` (openmls 0.8.1) and `transport-iroh`. Calling (`call-core`, `call-transport-iroh`) over our own relay. | `core/social-tree-core/README.md:1-14`; `WIRE-REGISTER.md:1-20`; `src/admission.rs:108-135`; `src/charter.rs:28-40`; `ffi/Cargo.toml:30-52` |
| **croft-stack** | implements | **Fabric-plane admission**: the relay runs `admission = "enforce"`, a camping-pass JWT (EdDSA; `exp`/`iat` with 30 s leeway; issuer `admit.croft.ing`) bound to the attaching `EndpointId`; sponsorship budgets; a flat identity-blind QoS ceiling; refusals with words. The relay is an unmodified `iroh-relay` wrapped by a counting airlock. The enforcement-scenario matrix. | `relay/deploy/croft-relay.toml:14-26`; `relay/source/crates/croft-relay-admit/src/token.rs:186-201`; `docs/ENFORCEMENT-SCENARIOS.md:1-10`; `sessions/2026-08-30-enforce-flip.md` |
| **CISS** | implements (storage substrate) | A metered content-addressed store (address of record SHA-256; BLAKE3 as the iroh/Bao transport hash), owner-manifested chains, gated per-DID reads, a `Principal` auth seam, the serverless `ciss-sync` fold (per-device signed heads, no primary, mtimes never evidence). **Planned, not built:** the content-blind history-convergence consumer and the meer custodian-queue mode. | `README.md:10-13,191-192`; `docs/SYNC-MODEL.md:1-40`; `docs/spec/gated-reads.md`; `crates/ciss-iroh/src/lib.rs:1-9`; `docs/plans/2026-08-24-1-plan-meer-custodian-queue.md:3-6` |
| **connect** | implements (the contract and the identity plane) | Calling contract v2; per-device `ing.croft.iroh.endpoint` records (one rkey per device, `self` primary); `ing.croft.call.grant` / `.policy` capability records; handle → DID → PDS → endpoint resolution. Direct evidence for §6.1's two identity planes and §4.5's multi-client fold. Lexicons unpublished (E161). | `docs/contract.md:1-40`; `lexicons/ing.croft.iroh.endpoint.json`; `lexicons/ing.croft.call.{grant,policy}.json` |
| croft-pwa | application layer, with lessons | Sealed box (ephemeral P-256 ECDH → HKDF → AES-256-GCM, fresh ephemeral per message); vault (PBKDF2 → HKDF → AES-GCM key-at-rest wrap); OAuth + DPoP; the ring/PDS walker. Bears on §5.11 (read-scoped keys), §6.2, §6.9. | `docs/ATPROTO.md:53-68,112-134`; `src/crypto/{sealedbox,vault}.ts` |
| forage | application layer, with lessons | Actor-scoped event ids (ADR 0001); a device-local state register with an "account half" per key (the local-first ↔ PDS persistence discipline, §4.6/§5.1); validate-on-the-way-in lexicon practice. | `docs/adr/0001-event-id-scheme.md`; `docs/DEVICE-LOCAL.md:15-40` |
| bluebird | application layer, with lessons | Origin of the hand-rolled atproto OAuth client; the custody spectrum (sponsor-only → self-custody) with honest degrade on a dead refresh chain; a sealed-box archive whose key is WebAuthn-wrapped (§5.4 custody seam, §8). | `docs/custody.md:20-60,97-121` |
| greetings_site | application layer, with a lesson | Server-blind link-key records: AES-256-GCM with the key only in the URL fragment; the PDS holds `{iv, ct}` — a minimal content-blind store (§6.6). | `src/crypto.ts:1-25` |
| arecipe | application layer, with a lesson | Lexicon ownership and consumption registry (consume `exchange.recipe.*`, mint `app.arecipe.*`): namespace ownership plus validate-in. | `docs/LEXICONS.md:1-25` |
| pdsview | application layer, with a lesson | PDS reading invariants: DID-canonical links, bidirectional handle verification (`alsoKnownAs`), origin allowlist — reusable for §6.9 discovery and §5.2 resolution. | `src/identity/{verify,resolve,did-doc}.ts` |
| fun | application layer, minor lesson | `pond-outcome`: a verifiable outcome is a replay of `(seed, moves)` to a state hash, never a stored field; `pond-docformat`: versioned forward-tolerant envelopes (analogues of §4.3/§4.6). | `crates/pond-outcome/src/lib.rs:1-12` |
| regift, view | application layer, nothing for the spec | Media extraction via share sheet; a static portal. No identity, no sync, no records. | — |
| arecipe_treatise, crofting_site, homebrew-tap, stellin | not relevant | Static pages, one formula, a scaffold not yet started. | — |
| experiments | not relevant (frozen) | Read-only since 2026-07-13; live successors in `discovery/alpha/experiments/`. | `.claude/ARCHITECTURE.md:166-174` |

**Two things the sweep found that the rest of this document turns on.** First, the workspace
*has* a production governance fold (croft `social-tree-core`), so every "no production fold
exists" sentence in the evidence set is now true only of the repository it was written in
(T-E-1, §6.2). Second, the word **admission** names two different events in the workspace — the
relay admitting an endpoint to the fabric (croft-stack, a wall-clock lease) and a Group admitting
a lineage as a member (Part 2 §10.2.2, chain standing) — and the spec uses the bare word for the
second only (T-6-2).

---

## 1. The gap matrix

One row per implemented capability. Columns: the spec §, what the spec says, what is built (repo
and path), the grade the evidence earns, and whether that is agreement or a gap.

### 1.1 Transport, admission and the crypto foundation (§4.1, §5.5, §6.1–§6.5, §6.9–§6.10, §8, §10.2.2, §10.3)

**Pins, read from the lockfiles 2026-10-28.** croft `Cargo.lock`: iroh 1.1.0, iroh-relay 1.1.0,
iroh-gossip 0.101.0, rustls 0.23.45, ed25519-dalek 2.2.0 (+3.0.0 transitive), blake3 1.8.7,
openmls 0.8.1 (`ports/keylayer-openmls/Cargo.toml:15`, exact), redb 2.6.3. croft-stack
`relay/source/Cargo.lock`: iroh 1.0.3, iroh-relay 1.0.3 (exact, `croft-relay-bin/Cargo.toml:17`),
iroh-gossip 0.101.0, jsonwebtoken 11.1.0, k256/p256 0.14.0, rustls 0.23.45, redb 4.1.0. Running:
relay + admit 0.2.1 under `admission = "enforce"` on `relay.croft.ing:8443`
(`.claude/DEPLOYED.md:12-17`; `croft-stack/sessions/2026-09-24-crypto-majors-v0.2.1.md:70-75`).
The client (iroh 1.1.0) and the relay (iroh-relay 1.0.3) interoperate in production across the
minor.

| Spec § (line) | Spec says | Built | Grade | Class |
|---|---|---|---|---|
| §4.1 (137) | "The reference implementation uses **Ed25519** … and **SHA-256** … `Verified` against real Ed25519 over live iroh" | The Proofs corpus is SHA-256 (`discovery/alpha/Proofs/lineage-groups/Cargo.toml:49`). **croft's production fold hashes BLAKE3 everywhere** (`croft/core/social-tree-core/src/model.rs:365` `blake3::hash`); Ed25519 `verify_strict` at `src/ports/ed25519.rs:81-88`; croft consumes only the §9 *signing* vectors (`tests/conformance_signing.rs`), never the fold vectors. | Verified (Proofs, SHA-256, loopback); Measured (croft, BLAKE3, two phones on one LAN, S2 runbook) | status-move → T-4-1 |
| §1.3 (51), App B (2784) | the `croft-*` tags are "shown where they are what was proven"; the rename is "an Appendix B item" | New brand tags keep being minted: `croft/s2/gossip-topic/v1` (`transport-iroh/src/transport.rs:63`), `croft/s2/pairing-blob/v1` (`pairing.rs:262`), `croft/head-ack/v1` (`social-tree-core/src/project/head_ack.rs:26`), ALPN `croft-call/0` (`call-transport-iroh/src/wire.rs:15`) | Verified (loopback pin tests) | clarity → T-4-2 |
| §4.1 (137) | "**MUST NOT** be silently negotiated down" | Frame and blob versions refused by number (`frame.rs:30,114-120`; `pairing.rs:39,134-140`); openmls pinned exact; no negotiation path exists | Verified (loopback) | agreement |
| §6.1.1 (570) | `EndpointId` is the TLS identity, "so it cannot be impersonated" | The relay binds its fabric capability to exactly that handshake-proven key: gate 3 `sub == connecting` (`croft-relay-admit/src/token.rs:196-201`); pin `valid_token_mismatched_id_denies_replay` | Verified (loopback, real `iroh-relay` server) + production (enforce flip) | refinement → T-6-3 |
| §6.1.1 (582) | "**MUST NOT** treat 'I have a verified connection' as 'this peer may act here'" | The severance is a crate boundary, not a runtime posture: `transport-iroh/src/lib.rs:4-7` ("fabric admission … and group admission stay severed by something stronger than discipline"), `RelayMode::Disabled`, ADR-0004 | Verified (loopback) + Measured (two phones, LAN) | agreement (build stronger than spec) |
| §6.2.1 (598) | "An **iroh relay** does **not** terminate the peer-to-peer QUIC and TLS session" | True of the end-to-end session; the relay *does* terminate the relay-client HTTPS/TLS link at its own edge (`croft-relay-bin/src/main.rs:516-521`), an unmodified `iroh-relay` behind a byte-counting airlock (`main.rs:18-21`) | Verified (production) | clarity → T-6-6 |
| §6.4 (666) | the relay "sees the *pair and byte count*" | The relay deliberately **records one endpoint per line, never a pair** (`main.rs:18-21,270-273,435-436`; ADR-0006) | Verified (production `usage` lines) | refinement → T-6-4 |
| §6.4 (666), §6.5.1 (698) | "roughly nine times in ten" direct | Runbook §17 (`croft/ops/RUNBOOK-two-device-call-test.md:1044-1067`): both calls relayed first, direct within ~5 s, LTE↔Wi-Fi, both directions | Measured (n = 2 calls) | agreement; the figure stays iroh's |
| §6.5.2 (702) | "**SHOULD** meter and isolate per tenant … **MUST** degrade *visibly*" | Per-token byte budget with a WARN on exhaustion (`main.rs:437-450`), spent-token refusal (`main.rs:95-104`), a flat identity-blind QoS ceiling (`config.rs:100-110`, ADR-0007), refusal with words (`embed/src/lib.rs:67-72`); pins `budget_drop.rs`, `admission_open.rs` | Verified (loopback) + production | refinement → T-6-5 |
| §6.5.2 (704), §10.3 (1868) | the relay "is never an authority"; a mandatory permanent hop is a disqualifier | Calling camps on **one** gated relay (`call-transport-iroh/src/relay.rs:26,86-94`); direct dial works; the social port is relay-free by construction. Carriage is bypassable; the *gate* lives on the only relay | Verified (loopback) / production | clarity → T-6-2 |
| §6.10.4 (940), §6.10 (916) | the four events and the send-side `broadcast` semantics are `[confirm]` | Matched on iroh-gossip 0.101.0 (`transport.rs:232-271`); `Lagged` → WARN; `subscribe(TopicId, bootstrap)`; **a broadcast into an empty swarm is delivered to nobody and reports success** (`transport.rs:331-335`) | Verified (loopback, 6 tests) + Measured (two phones) | status-move → T-6-7 |
| §6.9.2 (904), §6.11.2 (960) | Mode 2 discovery is "mDNS … every member discoverable and no seed" | The social port configures **no discovery service**: `presets::Minimal` + `MemoryLookup` + a dial card carried by a person (`transport.rs:14-24,204-212`); QR pairing blob; no mDNS crate in croft. Trap recorded in-code: `N0DisableRelay` still publishes to n0 pkarr | Measured (two phones, one Wi-Fi) | refinement → T-6-1 |
| §6.9.2 (900) | three mechanisms: DNS/Pkarr, Pkarr-on-DHT, mDNS | Calling adds a fourth: handle → DID → PDS record `ing.croft.iroh.endpoint` → `EndpointId` (`croft-stack/relay/source/DESIGN.md:16-17`; `connect/docs/contract.md:30-36`); and the relay enforces the reverse binding — an identity may only camp on an endpoint it publishes (`call-core/src/camp.rs:268-282`, `endpoint_unbound`) | Verified (production calls) | omission → T-6-1 |
| §8 (1650) | a Tier-0 helper "**MUST** hold no payload key and **MUST be able to prove it** (assert-and-log `payload_keys_held = 0`)" | The relay holds no signing material (`config.rs:130-131`) and prints only a verification-key fingerprint at start (`main.rs:578-585`); **no `payload_keys_held` assertion anywhere** in `relay/source/crates` | — | implementation-gap or clarity → T-8-1 |
| §8 (1652), profile (76, 132) | the failed-op response "is a governance dial … `Design`"; Croft declares "loud *(provisional)*" | Fabric refusals are loud and fully enumerated (`ENFORCEMENT-SCENARIOS.md:5-8,111-116`; WARN per refusal, never the proof) | Verified (production) | clarity → T-8-2 |
| §10.2.2 (1796) | "**admission** — the decision of who becomes a member" | croft-stack's `admission = "enforce"` is **fabric** admission; croft names the two (`transport-iroh/src/lib.rs:6-7`) | — | clarity → T-6-2 |
| §10.3 T7 (1860) | "Not require a wall-clock for any authority-bearing decision" | The fabric pass carries wall-clock `exp`/`iat` with 30 s leeway (`token.rs:188-194`), a 43200 s camping TTL, RFC 3339 `expires` in grant policy (`caps.rs:339-345`) | Verified (loopback matrix) | implementation-gap or clarity → T-10-1 |
| §6.12 (974) | media "**MUST** use the datagram flow" | No media and no datagram use in croft; a call exchanges one hello frame (`call-transport-iroh/src/wire.rs:1-7`) | — | spec-ahead-of-build (fine) |
| §8.2 (1678) (a) | freshness "**not** yet proven over … real-NAT (X1)" | The LTE↔Wi-Fi run is a *call*, not Group history convergence | Measured (calling only) | agreement — X1 stays open |

### 1.2 Group keying, storage, durability and the history store (§4.6, §5.11, §6.2.2, §6.6, §6.8.5, §7.6.11, §10.2)

Two reference stacks exist in the corpus: the delivery-layer rounds 1–3 ran on **mls-rs 0.55.2**
(`beta/impl/delivery-layer/05|06|07-*.md`) and their `e0-probe…e5-integration` crates are **not on
disk anywhere in the workspace**; everything since (meer-queue, mls-replant, replant-continuity,
croft) runs on **openmls 0.8.1**. Where a Part 2 `Verified` tag's only evidence is a round-1–3
test, it is unreproducible from the tree.

| Spec § (line) | Spec says | Built | Grade | Class |
|---|---|---|---|---|
| §4.6 (202) | "A fact **MUST** be encoded in a canonical, deterministic binary form (dag-cbor)" | croft's fact envelope is a **hand-rolled fixed-layout big-endian encoding** (`social-tree-core/src/model.rs:323-357` `canonical_bytes`; `wire.rs:14-16`), stored canonical-with-sig in redb (`store-redb/src/tables.rs:93-98`); **no cbor crate in `croft/Cargo.lock`**. dag-cbor exists only in `hist-atproto-spike` (serde_ipld_dagcbor; HIST-LIVE E1 green on a real PDS) and the CISS fs-manifest (`CISS/docs/SYNC-MODEL.md:12-14`) | croft: device; dag-cbor: live PDS | req/realization split → T-4-4 |
| §4.6 (206) | a node "**MAY** prune raw facts behind a verified checkpoint … rebuildable caches" | Five table families (`auth_/idx_/state_/meta_/local_`, `tables.rs:93-215`), `rebuild` (`fold_derived.rs:983`, `prop_i3_rebuild`), `compact_content` (`governance.rs:320`); MLS provider state also in redb (`keylayer-openmls/src/store.rs:1-30`) | device | agreement (companion `redb-storage-contract.md:82-102` names two families; the build has five) |
| §6.1.2 (578) | each client "is its own MLS member, one leaf, one signature key, one credential" | `MLS_128_DHKEMX25519_AES128GCM_SHA256_Ed25519`, per-store signer (`identity.rs:13-14`); the credential's identity bytes are the 32-byte `PrincipalId`, not a device id (`identity.rs:3-6,59`) | loopback + device | agreement; the lineage point is T-5-1 |
| §6.2.2 (604, 610) | content rides "inside `application_data` as opaque bytes" | `seal` = `create_message`, `open` = `process_message` (`keylayer-openmls/src/lib.rs:379-420`); `Session::send_sealed` seals the whole signed assertion envelope (`ffi/src/session.rs:767-808`); `chat-core` is pure, no MLS dependency | device (S2, two phones) | agreement |
| §6.2.2 (610) | "MLS's exporter (§8.5) remains available … the current design does not use it" | §6.4 (672) keys the outer seal "by an exporter output"; §6.4 (682) names the queue as "an exporter output shared per epoch"; S9 `export_secret("croft/meer-queue/v1")` (`meer-queue/TEST-LOG.md:678`) | loopback | clarity → T-6-11 |
| §6.4 (674–676), profile (74, 130) | "**When sealing is in effect, the mechanism and two wrapping rules are MUST**"; Croft declares "Sealing **ON** … epoch rule + wrap-once in force" | The outer seal exists **only in the spike** (`meer-queue/src/outer_seal.rs`, `tests/s17_nested_sealing.rs`); **zero occurrences** of `outer_seal` / `export_secret` / `wrap_once` in croft's `core`, `ports`, `ffi` | loopback only | implementation-gap (croft) + profile overclaim → T-C-1 |
| §6.3 (634) | "**Membership, not position on the fabric, is what authorizes reading**" | `receive_sealed` opens via MLS before the fold (`session.rs:823-845`); `a_strangers_valid_commit_stages_but_never_seats` (`tests/loopback_e2e.rs`); A3 in the port trait (`ports/keylayer.rs`) | loopback + device | agreement |
| §6.6.1 (724) | "`Verified` (exactly-once delivery holds down to the D-self floor …)" | Evidence is E5.1 on mls-rs (`05-…Results.md`); crate absent. croft: gossip only, `Event::Lagged` = drop + WARN (`transport.rs:247-250`), no buffer or retry | mls-rs loopback (unreproducible) | status-move → T-6-8 |
| §6.6.2 (728), profile (77, 133) | "`Verified` (Tier-0 meer holds sealed bytes and serves them on dial-home; admission denies a non-listed node)"; Croft declares "Mode 1 (relay + meer; relay.croft.ing)" | M1 confirmed on openmls 0.8.1 with an in-process CISS 0.6.0 and a loopback relay (`meer-queue/TEST-LOG.md:30-63`). **No meer is deployed** (`.claude/DEPLOYED.md:11-17`; `grep meer croft-stack` = 0); CISS has **no custodian or queue code** (`CISS/docs/plans/2026-08-24-1-plan-meer-custodian-queue.md:3-7` "nothing below is implemented"). "admission denies a non-listed node" appears in no spike log | loopback | profile overclaim → T-C-1; the tag's second clause → T-6-8 |
| §6.6.2 (730) | retention "is **delivery-bounded** … only until they drain on dial-home" | S5: CISS has no object DELETE; a sweep ends *serving*, not *holding* (`TEST-LOG.md:478-558`); S15 `Meer::sweep_with_retention` uses a liveness window; the profile (85, 135) says "Retention ≥ the liveness window" | loopback | clarity → T-6-13 |
| §6.6.2 (736) | "MLS applies commits forward-only and idempotently: a duplicate commit … is dropped" (the dual-delivery argument) | **S3 falsifies it for application messages**: openmls 0.8.1 *refuses* a duplicate (`process_message … "The requested secret was deleted to preserve forward secrecy"`, `TEST-LOG.md:216-248`), so content-hash dedup must precede MLS processing. croft opens MLS first and dedups in the fold after (`session.rs:823-845`); no seen-set on the sealed bytes — masked today by iroh-gossip's own message-id dedup | loopback (real lib) | refinement → T-6-12; implementation-gap (croft) |
| §6.6.3 (742) | "`Verified` (a gap … is filled by reconciling with a member reached off the fabric …)" | E3.3/E3.4 on mls-rs; crate absent. croft: a whole-record inline courier at join only (`transport-iroh/src/record.rs:1-45`, 4096-byte gossip ceiling, "Elsewhere is decoded and refused"); no steady-state RBSR in production | mls-rs loopback | status-move → T-6-8; spec-ahead-of-build for RBSR |
| §6.6.4 (758) | "`Verified` (one seal relayed by two paths deduplicates to a single entry …)" | S3/S3b confirmed (openmls); R2.2 byte-identical across D-self/D-meer/D-swarm | loopback | agreement, conditional on T-6-12 |
| §6.6.5 (766–768) | "`Verified` (… rejected by the real credential-validation hook at commit-build time)" | The hook was mls-rs's; openmls has no such hook in use (BasicCredential); no device-Group anywhere; croft holds "(at most one) live group" (`keylayer-openmls/src/lib.rs:62`); S4: "with a device group: NOT TESTED" | mls-rs loopback | status-move + req/realization split → T-6-9 |
| §6.8.5 (856–878) | the history store is a G-hist mirror group, `Design`/`Synthesis` | No G-hist experiment. The HIST lane spiked a **PDS** backend for the §G envelope ("everything Modeled", live E1–E8 green on bsky.social, `spike/hist_live/HIST-LIVE-RESULTS.md:33-43`); CISS's content-blind history-convergence consumer is "not built yet" (`CISS/README.md:12-14`) | PDS seam live; store: none | omission → T-6-15 (the spec names neither candidate) |
| §5.11 (516–534) | read-scoped asset keys, `Design`/`Synthesis` | Nothing built (`group-principal-seam/src/lib.rs:8` "resolved elsewhere, not modeled here") | none | agreement (tags honest) |
| §10.2.2 (1831–1840) | A1–A8 "`Measured` (S12…S26, C2–C4)" | croft's keylayer is "adapted … from the meer-queue lineage (S16/S23–S26)" (`lib.rs:1-7`): `return_via_external_commit` (`lib.rs:313`), PSK id = token (`lib.rs:56-59`), `both_admission_paths_end_to_end_at_loopback` | loopback + device | agreement |
| §10.5 (1912), profile (94, 139), `08-experiment-methodology.md:71` | "**MLS** core" / "openmls 0.8.1 reference stack" / "Real `mls-rs`" | Production is openmls 0.8.1 (`croft/Cargo.lock`); §10 never pins a library; §6.6's evidence and §7.6.11 name a different one than the profile | — | clarity → T-10-2 |
| `redb-storage-contract.md:42-60,117-147` (companion) | "Blobs live in iroh-blobs, never in redb"; three `[confirm against redb docs]` | croft: redb 2.6.3, **no iroh-blobs dependency**, `state_blob_presence_v1` presence-only; CISS: redb 4.1.0 and iroh-blobs (`ciss-iroh/src/lib.rs:1-14`); crash-shaped test `a_write_survives_a_process_that_never_gets_to_clean_up` (`store-redb/tests/persistence_pins.rs`) | device | refinement (companion) → §6.3 below |

### 1.3 Identity, presence, the one stream, and the interop surface (§4.5, §5.2, §6.6.5, §6.7, §6.8, §6.12, §9)

| Spec § (line) | Spec says | Built | Grade | Class |
|---|---|---|---|---|
| §4.5 (186) | "**MUST** fold by following each client's lineage back to its root … `Verified`: one human's two devices fold to one persona" | Earned by the Proofs tree: `lineage-mls/src/lib.rs:278` `fold_by_lineage`; `tests/t1_lineage_credential.rs:9-12` (the root key signs a claim binding lineage to device); over live iroh in `mls-welcome-over-iroh` | Verified (Proofs, loopback) | agreement (earned by the Proofs tree, not by croft) |
| §4.5 (186) | the membership key "**descends from the persona's key pair by signed credential**" | croft's leaf credential **asserts** the principal id — "the leaf credential's identity bytes ARE the core's 32-byte `PrincipalId`" (`keylayer-openmls/src/identity.rs:3-6,59`); binding checked by an in-memory registry (`ports/ed25519.rs:116`); "one device, one persona, is an S0 simplification … S3's DID↔persona binding is what separates them" (`ffi/src/session.rs:123-125`) | Verified at loopback for what it does; the MUST unbuilt | implementation-gap (croft, S3) → T-5-1 |
| §4.5.1 (192) | the envelope carries `author_client`, `author_device`, `author_principal` | Envelope wire v2 = `author_device(32) + author_principal(32) … lamport(8)` (`social-tree-core/src/wire.rs:14-16`); no `author_client`; the build's `DeviceId` *is* the signing key, i.e. the spec's client | Verified (loopback) + device (S2 rung 6) | clarity → T-4-3 |
| §4.5.1 (196) | "the **logical clock is per client and strictly logical, never a wall-clock** … lamport then a stable cryptographic tiebreak" | `LamportSource` / `MonotonicLamport` (`ports/ed25519.rs:143-150`); envelope v1's signed wall-clock field dropped (`WIRE-REGISTER.md`, O9); comparator v2 = lamport → content address (v1's `author_device` key dropped as party-privileging, G1) | Verified (loopback) + device | agreement |
| §5.2 (260) | a client is "one MLS **leaf**, one **signature key**, one **credential**" | `identity.rs:16-32` `Persona` = provider + leaf signer + credential | Verified (loopback) | agreement (term drift: croft calls the MLS client a `Persona`) |
| §3.1 (83), §4.5 | one persona, two clients, one key lineage | Realized at the **directory** layer, not by a signature chain: one `ing.croft.iroh.endpoint` record per device, `self` primary (`connect/docs/contract.md:30-36`); runbook §17 laptop arc published `croft-arc-callee` beside the phone's `self`; the admit helper refuses `endpoint_unbound` (`call-core/src/camp.rs:80`) | Measured (one LAN run, 2026-09-21) | req/realization split → T-5-1 |
| §4.5.1 (192), §5.2 (248) | a persona is "rooted in a **cryptographic key pair**" | Two identity roots in one client: calling = an atproto DID proven by OAuth (`android/.../AuthManager.kt:44`); chat = the device ed25519 key as principal (`ffi/src/session.rs:125`). The spec never relates a persona's lineage to an external account identity | Measured (device runs §15–§17) | omission → T-5-2 |
| §7.4 (1178) | "Freshness: no false 'current'" (governance scope) | Android renders any stored DID as "Signed in" (`CallScreen.kt`, "signed in shows the proven DID"); a dead refresh is swallowed (`MainViewModel.kt:328`); sessions died at ~10/6/5 days idle; the honest rule already exists for the arc — "A dead session says so … never says 'signed in'" (`call-session/src/session.rs:68-72`); plan ACCEPTED 2026-10-26, unbuilt | Measured (§15.2, §16, §17) | refinement → T-7-2; implementation-gap (croft) |
| §6.7 (776, 794, 798) | P-none is "always available as the floor"; a foregrounded app "needs no push host"; mobile wake is "not a requirement" | Camp = a live admitted relay connection while foregrounded; no push, no foreground service ("A backgrounded phone is not callable", `croft/TODO.md`; `docs/PLATFORM-POSTURE.md:86-96`) | Measured (LAN both ways; LTE↔Wi-Fi both ways) | agreement; the rest spec-ahead-of-build |
| §6.7 (786), §6.8.3 (842) | "`Verified` (a backgrounded device presents identical, complete, ordered history on waking, whether woken by push or by foreground poll)"; "`Verified` (exactly-once delivery … down to the D-self floor)" | **No EVIDENCE-MAP row** for §6.7 or §6.8.3; no matching test under `Proofs/` or `experiments/`; the sentence is verbatim in the seed corpus (`p10-full-part2-mechanics.md:2068-2069`); no push path exists anywhere | unearned as tagged | status-move → T-6-8 |
| §6.6.5 (766, 768) | "`Verified` (two co-entitled devices each decrypt and converge …)"; "`Verified` (a non-lineage credential is rejected by the real credential-validation hook …)" | No EVIDENCE-MAP row; `meer-queue/tests/s4_multi_device.rs:17-18`: "**with a device group: NOT TESTED.** §6.6.5 fan-out is not built"; croft has no device-Group | unearned as tagged | status-move → T-6-9 |
| §6.8.4 (846–850) | "**The application classifies each payload deliberately; the selector never guesses**"; live-durable is "fast delivery of durable messages, not a separate ephemeral lane" | One payload class (`social-tree-core/src/model.rs:546` `MessagePayload`); no selector — carriage is fixed per pond (chat = gossip, relay disabled; calling = direct with admitted relay fallback) | Verified (loopback) / Measured (device) | spec-ahead-of-build (vacuously conformant) |
| §6.8.4, §6.12 (974) | four payload classes; media on the datagram flow | A call carries no media: "one connection, the hello each way, the ending" (`call-transport-iroh/src/lib.rs:39`); the E129 endings verbatim (runbook §16:808-812). Call *signaling* fits none of the four classes | Measured | omission → T-6-10 |
| §9 (1684, 1688) | compatibility = derivations, pre-image, fold, floor, governance order; the profile enumerates "every open dial" | The only cross-implementation surface in live use is the **directory contract**: v2, `ing.croft.iroh.endpoint` / `.call.grant` / `.call.policy`, consumed by croft Android, `call-session/src/atproto.rs`, connect web and the admit service; lexicon JSON landed unpublished (connect #10; no `_lexicon.croft.ing` TXT); the profile has no directory, record-type or presence row | Measured (resolved on every device run) | omission → T-9-1 |
| §4.6 (202) | facts are dag-cbor; "a JSON … rendering **MUST NOT** be the artifact that is hashed or signed" | Directory records are JSON atproto records with their own version clock (`contract.md:16`); E161 Q1 (`type` vs `$type` is "a wire change") | n/a | clarity → T-9-1 (say §4.6 does not reach them) |
| §4.4 (178) | "**integrity alone MUST NOT be treated as authorization**" | Directory analogue: "A record with no `endpointId` is malformed (not 'not listed')" (`contract.md:52`); readers validate every foreign record on the way in (`.claude/LEXICONS.md` §4; a real PDS accepted a record missing every required field) | Measured | agreement in spirit → fold into T-9-1 |

**Grade ceiling for everything device-side in this table:** two phones on one LAN, and one
LTE↔Wi-Fi pair that hole-punched within ~5 s. A call that stayed relayed was never observed; no
backgrounded-callee success; no second persona-with-two-phones run.

### 1.4 The governance fold and the experiment program (§7, §11.11, Appendix B, EVIDENCE-MAP)

**The production fold.** `croft/core/social-tree-core/src/update.rs:1-12`: "The governance
transition — §7.3's fold, pure … Part 2 §7.3.1's layered resolution is realized as sequential
replay in `merge_cmp` order (lamport → content address) plus projections and hard-stops — G1's
withdrawn-alarm analysis is the equivalence argument; the arrival-order pins are its standing
test." Its redb adapter is `croft/ports/store-redb/src/fold_derived.rs` (2,589 lines) and
`governance.rs`. It ships in the client. The harness's reference fold (`drystone-convergence/src/fold.rs`)
implements the tiers literally; the production fold does not — it reaches the same resolved
state by replay plus projection, and the equivalence lives in a test header, not in Part 2.

| Spec § (line) | Spec says | Evidence | Grade now → earned | Class |
|---|---|---|---|---|
| §7.3.1 (1036–1046) keys 1–3 | the three-key order; tagged `Design.` | `drystone-convergence/tests/convergence.rs`: property A (1,000 cases × 5 shuffles), B, C, D1–D7, E, F; 27 green; gated in `smoke.yml:45` since 2026-10-27 | `Design` → **`Modeled`** (a reference fold: no crypto, no transport, single facts not quorums; the generator makes cross-author facts always concurrent, so causal structure appears only in hand cases; Stage 3 adversarial unbuilt) | status-move → T-7-3 |
| §7.3.1 (1038) key 1, the tiers | "Resolve the highest tier first, each tier against the settled result of the tiers above it" | The reference fold implements tiers (`fold.rs:26-34,167-195`); **production does not** — replay + projections + hard-stops, equivalence argued in `update.rs:8-12` and `croft-chat/tests/fold_ordering_keys.rs:14-19` ("the alarm was withdrawn") | `Design` | spec-prescribes-an-algorithm → T-7-5 |
| §7.3.1 (1040) key 2, causal | "the causally-later fact supersedes … observed-frontier reference" | `merge_cmp` = lamport → hash (`model.rs:401-405`); lamport enforced per device (`update.rs:1423-1431`); an antecedent-completeness gate at ingest (`update.rs:1433-1443`). **Could not confirm** that a device's lamport is bumped past its antecedents' lamports; if not, cross-device replay order is not happens-before and key 2 rests on the ingest gate, not the comparator | `Design` | clarity → §8 unresolved 1 |
| §7.3.1 (1042) key 3, tiebreak | "The default key is the **content address**" | `fold_ordering_keys.rs:97` `the_concurrent_tiebreak_is_the_content_address_never_the_device`, `:622` `merge_cmp_agrees_with_lamport_then_hash_on_every_pair`; comparator v2 (`WIRE-REGISTER.md`) | `Design` → `Measured` (real fold, real store, loopback) | status-move → T-7-3 |
| §7.3.1 (1034) precondition | "fewer than k concordant facts leaves the slot unchanged everywhere, including for the signatories" | `drystone-convergence/tests/stage4.rs` G1–G5, H1/H2 (quorum model, no crypto); `social-tree-core/tests/threshold_decisions.rs:131` `a_contested_subjects_approval_does_not_count` | `Design` → `Modeled` / `Measured` (croft) | status-move → T-7-3 |
| §7.3.2 (1056) | mutual expulsion "resolves by the concurrent tiebreak … Exactly one survives, never both and never neither" | **The build never tiebreaks a mutual expulsion**: `update.rs:995-1015` `mutual_expulsion_entry` → `ContestedEntry` ("both removes withheld from the replay (no verdict)"); `fold_pins.rs:280` `pin1_mutual_expulsion_projects_contested_both_orders`; and Part 2's own line 1066 says the subject of an open contradiction projects `CONTESTED`; §7.6.1 (1291) lists mutual removal as a *contradiction* | — | **internal contradiction** → T-7-4 |
| §7.3.2 (1058, 1060) | cross-slot projections; absent target = no-op, absent predecessor = gap; `Modeled.` | `convergence.rs` D1–D5, E1–E3; `stage2.rs:33` `stage2_referenced_gap_detected` | `Modeled` | agreement (EM row 46 still points at reviews-log prose; the code is in-tree) |
| §7.3.2 (1068) Resolution | "`Measured` (E117 P1: five arrival-order pins …)" | `fold_pins.rs:280-416` pin1–pin5 (real ed25519, in-memory); `croft-chat/tests/contested_projection.rs:175-418` | `Measured` | agreement; **zero EM rows** (`grep E117 EVIDENCE-MAP.md` = 0) → T-E-1 |
| §7.4.1 (1202) | "`Measured` (E117 P4: convergence pinned in both arrival orders)" | `removal_kind.rs:178` `a_departure_racing_a_readd_reconciles_identically_in_both_orders`, `:243` `a_ban_racing_a_readd_still_contests` | `Measured` | agreement; no EM row → T-E-1; Appendix E E6 lacks the third shape → T-7-4 |
| §7.5.2 (1269–1271) | the input set "**MUST** be closed under two distinct relations" | The frontier half is realized at ingest (`update.rs:1433-1443`); **the conflicted-subgraph closure has no visible realization in croft** and no harness stage | `Design` / `Verified` (Matrix cite) | implementation-gap (partial) → T-7-6 |
| §7.6 (1279) | "`Verified` (contradiction hard-stop; identical reformed genesis …)" | EM row 50 (`contradiction.rs`, `mutual_expulsion.rs`, loopback) | `Verified` (loopback) | agreement |
| §7.6.2 (1301) | membership half `Verified` (imported: replant-continuity @ `d52ed6f`); message half `Modeled` | `replant-continuity/tests/e12_7_{1,2,3}_*.rs`, `e12_2_message_continuity.rs`; EM rows 53–54 | as tagged | agreement |
| §7.6.11 (1447–1451, 1495) | "needs verification … pending execution" against `mls-rs 0.55.2` | Ran **twice**: all seven CONFIRMED on mls-rs 0.55.2 (`12-replant-experiments.md:3-5` "EXECUTED"; `07-…R3.md:24-34`), then re-run in-tree on openmls 0.8.1 (`mls-replant/tests/e12_{1,3,4,5,6}_*.rs`; `replant-continuity` E12.2/E12.7); `EXPERIMENT-BACKLOG.md:61` "Rung-A MLS mechanics … done and green" | `Design`+`[confirm]` → E12.1/E12.4 `Measured`; E12.3/E12.5/E12.6 `Verified` (real lib, in-process); E12.2/E12.7 as §7.6.2 | status-move → T-7-1 |
| §5.10 (496) | "`Design` (the reframe and the decomposition; seam brief, RUN-10)" | EM rows 75–76: `group-principal-seam/tests/seam.rs` (RUN-11, `Design`), `subspace_fold_green_real.rs` (`Verified` loopback, real openmls leaves) | `Design` (bindings) / `Verified`-loopback (subspace fold) | refinement → T-5-3 (the body lacks the parenthetical the map carries) |
| §3.2 (121) | the read/search helper "holds clear text and can search … it still holds no standing" (untagged) | `helper-seam/tests/helper_seam.rs:36,67,101` (RUN-14 EXP-C, real openmls, 3 PASS); landed in the companion (`proposed-changes-2026-07…:484` H-B) but never surfaced in Part 2 or EM | untagged → `Verified` (loopback; no-authority shown structurally, as the absence of an API) | omission → T-3-1 |
| §11.11 #3 (2631), App B (2774) | gap-completeness "`Load-bearing, unearned`" | `stage2.rs:129` `stage2_unreferenced_tail_gap_not_detectable` ("Record it in RESULTS.md as the case requiring completeness-ahead corroboration"); `fold_ordering_keys.rs:346` `two_peers_folding_different_subsets_can_resolve_standing_differently` | **unchanged** — both *exhibit* the beam; neither discharges it | agreement; App B's text reads as if nothing ran → T-B-1 |
| App B (2782) | tiebreak defaults "unresolved. `[confirm]`" | `model.rs:394-405`: content address only; no join-order or weighting option exists | `[confirm]` | spec-ahead-of-build (the build ships the safe floor) |
| App B (2800) | capped root, "the priority open security item" | No run; harness Stage 7 unbuilt (`open-threads.md:255`) | open | agreement |
| EVIDENCE-MAP (whole) | "One row per Part 2 status-tagged claim at or above `Modeled`" | `grep -c` in EM: `fold_ordering_keys` 0, `threshold_decisions` 0, `social-tree-core` 0, `helper-seam` 0, `drystone-convergence` 0, `contested_projection` 0, `removal_kind` 0, `fold_pins` 0, `E117` 0, `G1` 0. `MASTER-INDEX.md:86` A7: "The spec's evidence map does not cite it yet" | — | omission → T-E-1 |
| `drystone-convergence/RESULTS.md:3-11` | "No production implementation of the Drystone governance fold exists in this repository (`croftc/upstream-repo`)" | True of its origin repo and of discovery; **false workspace-wide** since 2026-08-23 (`6888525 E117 P2: the first core lands — social-tree-core`) | — | clarity → T-E-1 |
| `fold-semantics.md:99, 113-115` (companion) | order-independence "now `Verified` … (Proofs Part A)"; open items: tiebreak key `[confirm]`, which classes escalate | Part 2 tags the order `Design`; the content address is chosen (`model.rs:394`); the escalating classes are built (`update.rs:788-990`: mutual expulsion, removed-then-included, role thrash, competing RuleChange) | companion ahead of / behind the spec | clarity → §6.3 below |

---

## 2. Proposals, by section

### 2.1 Part 1

#### T-P1-1 — the §0 Map describes a §1.2 the body does not contain

- **Target:** Part 1 §0 Map, line 1284: *"§1.2 where Drystone sits, the complement-to-ATProto
  positioning and the one move that is genuinely its own."* Part 1 §1.2 (lines 89–124) contains no
  mention of ATProto; its body is the two-camps positioning (data layer vs governance layer) and
  the vertical-synthesis claim.
- **Class:** clarity.
- **Diff:**
  - `-` "§1.2 where Drystone sits, the complement-to-ATProto positioning and the one move that is genuinely its own."
  - `+` "§1.2 where Drystone sits, between the data-layer camp and the governance-layer camp, and the one move that is genuinely its own."
- **Principle:** readability (the map must match the body). The ATProto relationship is real and
  built (CISS carries atproto identity; the history store's B1 record is `ing.croft.hist.entry`;
  §7.3.9 folds the `did:plc` rotation key into recovery), but it is a Part 2 §10 / Appendix C
  positioning, not a Part 1 principle. If the owner wants the positioning in Part 1, the honest
  home is one sentence in §1.2 naming atproto as the public, hosted layer an ecosystem may compose
  with, and Drystone as the layer that holds no centre; see T-P1-2.
- **Evidence:** the two lines quoted; `grep -ni atproto part-1-reasoning-underpinnings.md` returns
  only the map line.
- **Grade:** n/a (editorial).
- **Decision:** ready (the diff above). **needs-call** only if the owner prefers to add the
  positioning to the body instead of removing it from the map — then it is one sentence, and it
  must stay a positioning, never a dependency (Part 1 §1.1: "no named ecosystem … is part of the
  protocol's definition").

### 2.2 The spec's front door: README.md and the filing logs

These are not Part 1 or Part 2 text, but they are the first thing an implementer reads and they
currently describe a spec two revisions old.

#### T-R-1 — the README teaches the retired status ladder

- **Target:** README.md "How to read the normative text": *"**Proof / status flags** travel with
  each carried claim: `green-real` (demonstrated with real crypto/transport), `green-model` (proven
  in the reference model), `design` (specified, not yet proven), `ENABLING` (a byte-level encoding
  …). Where a claim rests on an external fact not yet independently confirmed, it carries
  **[confirm before publish]**."*
- **Class:** clarity.
- **Diff:** replace the bullet with the A.9 ladder by name — `Verified`, `Verified-RFC`,
  `Modeled`, `Measured`, `Established`, `Design`, `Synthesis`, `Load-bearing, unearned`,
  `[gates-release]`, `[confirm]` — and a pointer: "the single ladder both Parts defer to is
  `conventions-and-decisions.md` A.9; the legacy words `green-real`/`green-model`/`ENABLING` are
  alpha-tier only (B.6)."
- **Principle:** readability (one vocabulary of record). A.9 itself rules: "They never appear in a
  live Part 1 / Part 2 sentence."
- **Evidence:** `conventions-and-decisions.md` §A.9 (lines 125–157) and B.6; Part 2 §1.1 reproduces
  the canonical ladder.
- **Decision:** ready.

#### T-R-2 — the README terminology table contradicts the vocabulary of record

- **Target:** README.md "Terminology (defined once, used consistently)". Four rows disagree with
  `conventions-and-decisions.md` A.1–A.8 and Part 2 Appendix D, and two rows point at superseded
  files:
  - **scope**: README *"The unit of shared state a set of principals holds in common (the 'common
    pasture'); a group."* Part 2 §3 / §5.4 / A.3: scope is the **exposure envelope**, wider than
    the Group; a relay or meer sits in the scope but not in the Group.
  - **capability**: README *"A delegated, additive, revocable means to *do* something, sitting on
    top of the rights floor (Part 2 §5.4)."* Part 2 §5.5 / D.5: a capability is Meadowcap's
    data-access grant, kept verbatim; §5.4 is **resources**.
  - **role**: README *"The descriptive, free-composing set of capabilities a principal holds right
    now (Part 2 §5.5)."* A.2 / D.5: lowercase role is the genus; a **Group Role** is in-Group
    governance authority, granted by consent, revocable.
  - **persona**: README *"`persona-definition.md` is the vocabulary of record"* — that file is in
    `superseded/` and the README's own superseded-companions paragraph says so.
  - **peer**: README *"see `bounded-contexts-and-vocabulary.md`"* — superseded, folded into
    `conventions-and-decisions.md`.
- **Class:** clarity.
- **Diff:** replace the table with the A.1–A.8 one-liners (group/Group, role/Group Role, scope,
  principal, the two revocation planes, persona, coinages, peer) plus the Appendix D pointer, or
  delete the table and point at Appendix D outright. Recommended: the one-liners — the README is
  where a newcomer meets the nouns.
- **Principle:** readability (one vocabulary of record); P-Peer-Equality indirectly, because the
  README's "capability sitting on top of the rights floor" is exactly the right/role/capability
  collapse §5.0 was written to prevent.
- **Evidence:** README.md lines quoted; `conventions-and-decisions.md` A.1–A.8; Part 2 Appendix D
  (lines 2888–2971); `superseded/README.md`.
- **Decision:** ready.

#### T-R-3 — the README status block and sequencing note describe an older revision

- **Target:** README.md status block: *"Status: Draft, consolidated (Part 1 = the p10 principles
  set; Part 2 = the p11 rebuild; document-pass-8)"*, *"Part 1: `part-1-reasoning-underpinnings.md`
  (the consolidated 'p9' set; document-pass-6)"*, *"Pending design review: the remaining open work
  is the design decisions and `ENABLING` wire encodings tracked in Part 2, Appendix B"*; and the
  sequencing box: *"Part 2 §7.2 (message formats) is not yet field-by-field, and the `ENABLING`
  encodings in Part 2 Appendix B are open."*
- **Class:** clarity / status-move.
- **What changed since:** Part 2 was revised on 2026-08-19 (E110: the A-series admission interface
  at §10.2.2; E111: `implementation-profile.md`; the readmission arc graduated into canonical) and
  2026-08-25 (the E133/E136 amendment set: five owner decisions in spec voice — resolution
  threshold, token issue/revoke weights, exit voice, departure kind, R7 semantics). §7.2 is the
  grant-and-revocation interface, not "message formats"; the message pre-image is §4.3 and the
  governance-fact encoding is §7.3.1, both `[gates-release]`.
- **Diff:** restate the status as of the last landing: "Status: beta; Part 2 revised through
  2026-08-25 (E133/E136); the A-series admission interface and the implementation profile landed
  2026-08-19. Open where the spec says so: the completeness-ahead beam, the `[gates-release]`
  encodings, the BLAKE3 re-proof (Appendix B)." In the sequencing box replace "Part 2 §7.2 (message
  formats)" with "the §4.3 / §7.3.1 byte encodings" and `ENABLING` with `[gates-release]`.
- **Principle:** readability; P-Knowable-Truth at the document level (the front door must not
  present a stale state as current — the spec's own §7.4 rule applied to itself).
- **Evidence:** `git log --since=2026-08-01 -- part-2-certifiable-design.md`: `2c310c2`,
  `3a78a02`, `6d3b740`, `ebf4327`; Part 2 §7.2 title (line 988).
- **Decision:** ready.

#### T-R-4 — no changelog records the August landings

- **Target:** `CHANGELOG.md` (filing-side log, "document-pass-0..10, newest first") ends at
  document-pass-10, 2026-07-07. `part-2-changelog.md`'s last entries are RUN-12-era (July).
  `grep -n 'E133\|E136\|E110\|E111\|2026-08' CHANGELOG.md part-2-changelog.md part-1-changelog.md`
  returns nothing.
- **Class:** omission (in the filing log, not the spec).
- **Diff:** add four entries in house style for `2c310c2` (step-5 merge, the readmission arc),
  `3a78a02` (E110, §10.2.2), `6d3b740` (E111, the profile), `ebf4327` (E133/E136, the five
  decisions), each naming the sections touched and the evidence (S23–S26, C2–C4, E117 P1/P4), and
  a fifth for this true-up when it is applied.
- **Principle:** P-Knowable-Truth (the record is auditable; a revision with no log entry is a
  silent mutation of the document set).
- **Evidence:** the git log above; the commit bodies carry the section lists.
- **Decision:** ready.

#### T-R-5 — the README's external-facts rule predates the iroh 1.0 pass

- **Target:** README.md: *"For atproto / iroh / iOS facts, the source of truth is the FACTCHECK SoT
  (iroh `1.0.0`); cite it, do not re-verify."*
- **Class:** clarity.
- **Diff:** Part 2 §6 now verifies the transport-plane claims against the iroh 1.0 primaries
  directly and scopes the remaining `[confirm]` flags to the separately-versioned crates
  (`iroh-gossip`, the address-lookup crates). Replace the bullet with that rule, and name the pins
  the implementing repos actually carry (see T-6-x, pending the survey) so the README's "iroh
  1.0.0" does not read as the deployed version.
- **Principle:** readability; the requirement/realization discipline of §10.
- **Evidence:** Part 2 §6 verification legend (line 546); Appendix B "iroh substrate confirmation,
  post-1.0" (line 2790).
- **Decision:** ready.

### 2.3 Part 2 §3–§4 — overview and data model

#### T-3-1 — §3.2: the read/search helper's no-standing claim has evidence the spec does not cite

- **Target:** §3.2 line 121, the read/search helper bullet (untagged).
- **Class:** omission (of evidence).
- **Diff:** append "`Verified` (evidence: `helper-seam/tests/helper_seam.rs`, RUN-14 EXP-C,
  loopback, real openmls; no-authority shown structurally — the helper has no API through which
  to act)" or, if §3.2 is to stay a tag-free cast list, add only the EVIDENCE-MAP row (T-E-1).
- **Principle:** P-Knowable-Truth. **Evidence:** matrix 1.4. **Decision:** ready (owner picks
  the placement).

#### T-4-1 — §4.1 and the profile: say what the production fold hashes

- **Target:** §4.1 line 137: *"The reference implementation uses **Ed25519** … and **SHA-256**
  (32-byte digests). `Verified` against real Ed25519 over live iroh"*; profile §3 line 138:
  *"**BLAKE3 committed**; running on the SHA-256-proven §4 until the wire-freeze re-proof"*.
- **Class:** status-move (profile) + clarity (spec).
- **What is built:** the Proofs corpus that earned §4's `Verified` is SHA-256; croft's shipping
  fold is **BLAKE3 everywhere** (`social-tree-core/src/model.rs:365`) and has run only the §9
  *signing* vectors (`tests/conformance_signing.rs`), never the fold vectors. The BLAKE3 re-base
  Appendix B calls open is de facto underway in production code, unproven against the vectors.
- **Diff:** profile line 138 → "**BLAKE3 in production code** (`social-tree-core`); the §9 fold
  vectors have not been re-run against it — the §4 `Verified` still stands on the SHA-256 Proofs
  corpus; the signing slice is run (`conformance_signing.rs`)". §4.1: add one clause after
  "`Verified` …": "(the Proofs corpus; the Croft client already hashes BLAKE3 and carries its
  own conformance status on the profile sheet)".
- **Principle:** P-Knowable-Truth. **Decision:** ready (profile); the §4.1 sentence stays until
  the re-proof, with the clause added.

#### T-4-2 — §1.3 / Appendix B: the brand-tag debt is growing, not historical

- **Target:** §1.3 line 51: *"Those values are shown where they are what was proven, but they
  are the reference profile, not the protocol"*; Appendix B line 2784.
- **Class:** clarity.
- **What is built:** new signed or hashed tags keep being minted in the brand namespace —
  `croft/s2/gossip-topic/v1`, `croft/s2/pairing-blob/v1`, `croft/head-ack/v1`, ALPN
  `croft-call/0` (matrix 1.1).
- **Diff:** Appendix B: "The reference implementation continues to mint tags in the brand
  namespace until the wire-freeze; each is registered in croft's `WIRE-REGISTER.md` so the rename
  is one enumerated pass, not an archaeology." And a rule: every new tag is registered there on
  minting.
- **Principle:** P-Knowable-Truth (the record of what must change is kept, not rediscovered).
  **Decision:** ready.

#### T-4-3 — §4.5.1: name the envelope the reference actually carries

- **Target:** §4.5.1 line 192: *"the envelope carries `author_client` (the signing identity the
  signature verifies against), `author_device` (the device hosting it), and `author_principal`"*.
- **Class:** clarity + profile pin.
- **What is built:** envelope wire v2 = `author_device(32) + author_principal(32)` + lamport
  (`wire.rs:14-16`); no `author_client`; the build's `DeviceId` **is** the signing key, i.e. the
  spec's *client*, and two croft apps on one phone each hold their own key, so the build has two
  clients per device and no device identifier at all.
- **Diff:** keep the three-role model; add "The reference profile carries two of the three — the
  signing key (the client) and the principal — and derives nothing per device; which fields a
  profile carries is a `[wire]` pin on the sheet." Profile §2.4: a row "Envelope author fields:
  `(signer-key-as-client, principal)`; layout `social-tree-core/WIRE-REGISTER.md` v2".
- **Principle:** the razor (do not carry a field nobody emits). **Decision:** ready.

#### T-4-4 — §4.6: the MUST is determinism; dag-cbor is one realization

- **Target:** §4.6 line 202: *"A fact **MUST** be encoded in a canonical, deterministic binary
  form (dag-cbor) for hashing and signing"*.
- **Class:** requirement/realization split.
- **What is built:** three encodings coexist. croft's facts are a hand-rolled fixed-layout
  big-endian envelope (`model.rs:323-357`), no cbor crate in its lockfile; dag-cbor is used by the
  HIST spike (live-proven: a record's CID serves as the reconciliation head with zero re-hash,
  B16) and by CISS's manifests. The profile says "Wire pins: none pinned."
- **Diff:**
  - `-` "**MUST** be encoded in a canonical, deterministic binary form (dag-cbor)"
  - `+` "**MUST** be encoded in a canonical, deterministic binary form — one valid byte sequence per fact on every node — and that form is a `[wire]` pin (§10.4). The reference realization is dag-cbor, chosen because the atproto ecosystem the history store composes with already canonicalizes in it (§6.8.5, B16); the Croft client's fixed-layout envelope (`WIRE-REGISTER.md` v2) is a second conforming realization until the `[gates-release]` pin."
  - The paragraph's argument for dag-cbor (map-key order, number model) stays as the rationale
    for the reference.
- **Principle:** P-Knowable-Truth (one FactId per fact is the requirement) and the razor (mandate
  no more than agreement needs).
- **Evidence:** matrix 1.2 row 1; `spike/hist_live/HIST-LIVE-RESULTS.md` E1.
- **Decision:** **needs-call.** Options, in plain English: (a) the spec requires determinism and
  names dag-cbor as the reference; croft migrates to dag-cbor at the wire-freeze (one encoding
  for facts and for the PDS-backed history store); (b) the spec requires determinism and pins its
  *own* fixed layout at the freeze, with dag-cbor only the history-store lens. (a) keeps one
  canonical form across the fact and the record that stores it; (b) keeps the fold free of a
  CBOR dependency. Either way the MUST becomes the property, which is the change proposed here.

### 2.4 Part 2 §5 — identity, rights, roles, capabilities

#### T-5-1 — §4.5 / §5.2: the production client asserts its principal; it does not descend by signed credential

- **Target:** §4.5 line 184–186: *"each **client** … carries a membership key that **descends from
  the persona's key pair by signed credential** … Receivers **MUST** fold by following each
  client's lineage back to its root"*.
- **Class:** implementation-gap (croft) **and** a requirement/realization question.
- **What is built:** croft's leaf credential's identity bytes *are* the `PrincipalId`
  (`identity.rs:3-6,59`); the binding is an in-memory registry; "one device, one persona, is an
  S0 simplification … S3's DID↔persona binding is what separates them" (`ffi/src/session.rs:123-125`).
  Today's forgery surface is bounded only because the asserted bytes are the device's own key.
  A **second realization of "lineage" exists at the directory layer**: the persona's DID writes
  one `ing.croft.iroh.endpoint` record per device (`connect/docs/contract.md:30-36`) and the
  admit helper refuses an endpoint the account does not publish (`camp.rs:80`, `endpoint_unbound`).
  That is repo-write authority as the credential — no signature chain, no fold, trusts the PDS.
- **Diff (spec):** no change to the MUST. Add to §10.4 (or §4.5 as an attribute-conditioned
  note): "A profile MAY additionally bind clients to a persona through an external identity's
  record authority (the persona's DID publishes the client's endpoint); this is weaker than the
  signed-credential chain — it is not verifiable offline and trusts the record host — and does
  not satisfy §4.5 on its own." Register the Proofs corpus (`t1_lineage_credential.rs`,
  `fold_by_lineage`) as the realization that does satisfy it.
- **Diff (repo):** croft `TODO.md` gains a pointer: §4.5 signed client credential — S3, the E120
  binding fact (`plans/2026-08-25-1-plan-product-shell-adoption.md:804`).
- **Principle:** P-Peer-Equality (the count must be per persona and unforgeable by adding
  clients). **Decision:** the gap filing is **ready**; whether to admit the directory binding
  into §10 as a named weaker realization is **needs-call**: (a) admit it, with its disqualifiers;
  (b) keep it out and leave croft non-conformant on §4.5 until S3.

#### T-5-2 — §5.2: the open seam should name the external-identity binding

- **Target:** §5.2 line 266: *"**Open seam: the principal that anchors a multi-client lineage.**"*
- **Class:** omission.
- **What is built:** two identity roots in one client — calling = an atproto DID proven by OAuth;
  chat = the device key as principal — and the spec never says how a persona's key lineage
  relates to an external account identity, though §7.3.9 already folds the `did:plc` rotation
  key into recovery and §6.9.2 (T-6-1) now resolves a DID to an endpoint.
- **Diff:** append to the seam paragraph: "and the **binding of a lineage to an external
  identity** (a DID, an OAuth-proven account) is part of the same seam: the binding is a recorded
  human act (a fact on the chain, S3), never an inference from possession of a session; the
  external identity is a locator and a vouch, never the root of standing."
- **Principle:** the razor applied to identity (Part 1 §2.0: a human act binds trust to a key).
  **Decision:** ready.

#### T-5-3 — §5.10: carry the RUN-11 evidence parenthetical the map already has

- **Target:** §5.10 line 496: *"`Design` (the reframe and the decomposition; seam brief, RUN-10)"*.
- **Class:** status-move (partial).
- **Diff:** "`Design` (the reframe and the decomposition; seam brief, RUN-10; evidence:
  `group-principal-seam/tests/seam.rs`, RUN-11 — re-issue-not-revoke-in-place across a
  fold-driven authority change) and `Verified` for the client→subspace fold (evidence:
  `subspace_fold_green_real.rs`, RUN-11, loopback, real openmls leaves)".
- **Principle:** P-Knowable-Truth. **Evidence:** EM rows 75–76. **Decision:** ready.

### 2.5 Part 2 §6 — transport and delivery

The layer note first, because several items below depend on it. **"Admission" names two events
in the workspace.** The relay admits an *endpoint* to the *fabric* on a wall-clock lease bound to
its TLS key (croft-stack; `admission = "enforce"`); a Group admits a *lineage* as a *member* on
chain standing (§10.2.2, the A-series). They fail differently and recover differently: a refused
attach is retried on a fresh pass in seconds; a refused merge is a governance event. croft keeps
them severed by a crate boundary (`transport-iroh/src/lib.rs:4-7`). The spec uses the bare word
for the second only.

#### T-6-1 — §6.9.2: two discovery mechanisms the build uses that the spec does not list

- **Target:** §6.9.2 lines 898–904 list three mechanisms: DNS/Pkarr, Pkarr-on-mainline-DHT, mDNS.
- **Class:** omission.
- **What is built:** (i) **out-of-band rendezvous** — the social port configures no discovery
  service at all (`presets::Minimal` + `MemoryLookup`), peers are learned from a dial card a
  person carries across (the QR pairing blob); two phones on one Wi-Fi, S2 runbook rungs 1–6.
  (ii) **An identity-layer record** — handle → DID → PDS record `ing.croft.iroh.endpoint` →
  `EndpointId` (`croft-stack/relay/source/DESIGN.md:16-17`; `connect/docs/contract.md:30-36`), with
  the reverse binding enforced by the admit helper: an identity may only camp on an endpoint it
  publishes (`camp.rs:268-282`). Production calls resolve this way.
- **Diff:** two new bullets after mDNS, in the section's own frame (who can observe or withhold,
  never whether they can lie): "**Out-of-band rendezvous (center-free, bounded to a social
  tie).** A self-described dial card (key plus current direct addresses) handed over by a person;
  no resolver, no publication, discoverable only by whoever was handed the card. The strictest
  no-external-contact posture; stale the moment addresses change." and "**An identity-layer
  record (a soft center at the record host).** The persona's own identity record names its
  endpoints; integrity is the identity's key (a forged record fails the DID's signature),
  observation and withholding sit with the record host. A helper MAY enforce the reverse binding:
  an endpoint is reachable as a persona only if that persona's record names it." Plus one
  realization trap for §10.3: `N0DisableRelay` still publishes to n0 pkarr.
- **Principle:** P-Durable-Enablement and P-Local-Truth (zero external contact is a real floor);
  P-Knowable-Truth for the binding. **Decision:** ready.

#### T-6-2 — §6.5.2 and §10.2.2: name fabric admission, and say a gated relay is still a bypassable one

- **Target:** §10.2.2 line 1796: *"**admission** — the decision of who becomes a member"*; §6.5.2
  line 702 names no gate on the relay.
- **Class:** clarity (naming) + a possible normative sentence.
- **Diff:** Appendix D gains **fabric admission** vs **Group admission** (one line each, with the
  recovery difference above). §10.2.2's first sentence becomes "Group admission — …". §6.5.2
  gains: "A relay **MAY** gate attachment on a fabric capability bound to the attaching
  `EndpointId` (§6.1.1); a gated relay remains a bypassable helper (T3, T4) and grants nothing in
  any Group." 
- **Principle:** P-Peer-Equality (the gate is reachability economics, never standing); the
  layer-boundary rule.
- **Evidence:** matrix 1.1 rows §6.1.1, §6.5.2, §10.2.2; the enforce flip session.
- **Decision:** the naming is **ready**; the MAY sentence is **needs-call**: (a) say nothing
  normative and leave the gate to §10.3 and the profile; (b) the one MAY sentence above. (b) is
  recommended — without it a reader of §6.5.2 cannot tell whether Croft's relay is conformant.

#### T-6-3 — §6.1.1: the channel identity is the binding point for any per-peer fabric credential

- **Target:** §6.1.1 lines 570–574: the plane "establishes **provenance of the channel**" and
  nothing more.
- **Class:** refinement.
- **Diff:** add "It is also the only safe binding point for a per-peer fabric credential: a pass
  bound to the handshake-proven key cannot be replayed from another endpoint, so possession of the
  bytes never equals attachment (the fabric-plane twin of A1)."
- **Principle:** P-Knowable-Truth. **Evidence:** `token.rs:196-201` gate 3; pin
  `valid_token_mismatched_id_denies_replay`. **Decision:** ready.

#### T-6-4 — §6.4: a relay records endpoints, never pairs

- **Target:** §6.4 line 666 describes what a relay *can see* ("the *pair and byte count*").
- **Class:** refinement.
- **What is built:** the production relay deliberately **records one endpoint per line, never the
  `(src, dst)` pair** (`main.rs:18-21,270-273,435-436`; ADR-0006).
- **Diff:** in the §6.4 attribute-conditioned style: "A relay's retained record names one
  endpoint per entry and never the pair, unless the deployment declares `pair-visible` on its
  profile sheet."
- **Principle:** P-Knowable-Truth (the metadata-transparency guard). **Decision:** **needs-call**:
  (a) normative in §6.4; (b) a profile `[deployment]` row only, since the relay sits outside any
  Group's charter. (b) is the smaller claim and matches §6.4's "deployment can opt out" framing.

#### T-6-5 — §6.5.2: metering is two things, and one of them is not optional

- **Target:** §6.5.2 line 702: *"A relay process **SHOULD** meter and isolate per tenant"*.
- **Class:** refinement.
- **What is built:** a **flat, identity-blind QoS ceiling** (ADR-0007: "throughput is managed,
  never sold") and a **per-sponsorship byte budget** with a visible drop; both pinned.
- **Diff:** "A relay **MUST** cap per-client throughput with a flat, identity-blind ceiling
  (because a ceiling that varied by identity would sell reachability, a resource, as standing);
  it **MAY** additionally meter a per-sponsorship budget, and a budget exhausted **MUST** drop
  visibly (§6.5.2's own rule)."
- **Principle:** P-Peer-Equality. **Decision:** **needs-call**: is the flat ceiling a protocol
  property or an operator's? §6.4's own rule (line 684) prefers the attribute-conditioned MUST
  over SHOULD wherever a legitimate deviation exists; here no legitimate deviation has been
  named.

#### T-6-6 — §6.2.1: which TLS the relay terminates

- **Target:** §6.2.1 line 598: *"An **iroh relay** does **not** terminate the peer-to-peer QUIC
  and TLS session"*.
- **Class:** clarity.
- **Diff:** "… does **not** terminate the peer-to-peer QUIC and TLS session (it terminates only
  the relay-client link's own TLS at its edge, through which the end-to-end session passes
  opaque)". **Evidence:** `croft-relay-bin/src/main.rs:516-521`. **Decision:** ready.

#### T-6-7 — §6.10 / §6.10.4: discharge two `[confirm]`s against iroh-gossip 0.101.0

- **Target:** §6.10.4 line 940: the four events and *"the send-side `broadcast` return semantics"*
  are `[confirm]`; §6.10 line 916.
- **Class:** status-move.
- **Diff:** pin to 0.101.0: "the net-layer events are `NeighborUp`, `NeighborDown`, `Received`,
  `Lagged` (verified against 0.101.0, the first release whose manifest asks `iroh ^1`); **a
  broadcast into a swarm with no neighbours is delivered to nobody and reports success**, so a
  sender that needs to know anyone is there must wait for `NeighborUp` first." Keep the view-size
  constants and PRUNE/GRAFT `[confirm]`.
- **Principle:** P-Knowable-Truth (the no-ack property, now stated at its sharpest).
  **Evidence:** `transport.rs:232-271,331-335`; `transport-iroh/Cargo.toml:22-26`. **Decision:** ready.

#### T-6-8 — §6.6.1, §6.6.3, §6.7, §6.8.3: four `Verified` tags with no reproducible evidence

- **Target:** §6.6.1 line 724; §6.6.3 line 742; §6.7 line 786 (*"`Verified` (a backgrounded device
  presents identical, complete, ordered history on waking, whether woken by push or by foreground
  poll)"*); §6.8.3 line 842 (*"`Verified` (exactly-once delivery holds under every combination of
  surviving paths …)"*).
- **Class:** status-move (down).
- **What the evidence is:** the delivery-layer round-1–3 tests on mls-rs 0.55.2, whose crates are
  not in the workspace; no EVIDENCE-MAP row for §6.7 or §6.8.3; no push path exists anywhere, so
  "woken by push" cannot have been Verified. A.9 (FND-T1): an experiment-earned tag resolves to
  a named test plus its RUN.
- **Diff:** each becomes "`Measured` (report only: `05/06/07-…Results.md`, mls-rs 0.55.2, loopback;
  the crates are not retained)" — or `Design` where the report does not carry the clause (the
  push half). Split the §6.7 push half out as `Design` explicitly.
- **Principle:** P-Knowable-Truth (the ladder is the spec's own freshness rule applied to
  itself). **Decision:** **needs-call**: (a) accept report-only grades; (b) re-earn the four on
  the openmls reference stack (a bounded harness extension, open-threads §5). (a) is honest now;
  (b) is the work.

#### T-6-9 — §6.6.5: the device-Group tags and the library-specific hook

- **Target:** §6.6.5 lines 766–768, two `Verified` tags; the second cites *"the real
  credential-validation hook at commit-build time"*.
- **Class:** status-move + requirement/realization split.
- **What is built:** no device-Group anywhere; the hook was mls-rs's; openmls has no such hook in
  use; S4: "with a device group: NOT TESTED".
- **Diff:** state the requirement ("admission policy evaluated before a commit is built" — A3/A5
  at the device-Group) and cite two realizations: the mls-rs credential-validation hook, and
  openmls's `stage_commit` → `merge_admission` split (croft `ports/keylayer.rs`). Retag per T-6-8.
- **Principle:** P-Peer-Equality. **Decision:** the split is **ready**; the S4/E92 dial
  (race delivery across a persona's devices — cheap, reveals device count to the meer — vs
  deliver-once, which needs the device-Group) is **needs-call**.

#### T-6-10 — §6.8.4: call signaling is a payload class the list lacks

- **Target:** §6.8.4 lines 846–854: four classes (live-durable, intrinsic-ephemeral,
  chosen-ephemeral, self-destruct).
- **Class:** omission.
- **What is built:** a call is "one connection, the hello each way, the ending"
  (`call-transport-iroh/src/lib.rs:39`) — dial, hello, ending (E129, verbatim on both screens),
  the admission swap. It is live, non-durable, and its loss must be *visible* (a timeout with
  words), which is none of the four.
- **Diff:** a fifth bullet: "**Session control.** Signaling for a live session (a dial, its
  answer, its ending): live, never durable, and **loss is visible** — a control message that does
  not arrive is a timeout the user reads, never a silent non-event — which is what distinguishes
  it from intrinsic-ephemeral state." Cross-reference §6.12.
- **Principle:** P-Knowable-Truth (loss-visibility floor, §6.5.3). Also a clarity note: the
  parent plan's "one stream" means one *client* with per-pond cores, not §6.8.4's single durable
  lane; a reader will conflate them. **Decision:** ready (placement §6.8.4 vs §6.12 preamble is
  the owner's).

#### T-6-11 — §6.2.2: the exporter *is* used

- **Target:** §6.2.2 line 610: *"MLS's exporter (§8.5) remains available should a later
  construction need a value cryptographically bound to group state, but the current design does
  not use it."*
- **Class:** clarity (internal contradiction with §6.4 lines 672 and 682).
- **Diff:** "MLS's exporter (§8.5) is used for two values that must be bound to group state
  without being content: the outer-seal key (§6.4) and the per-epoch queue name (§6.6.2); it is
  not used to bind content into the transcript." **Evidence:** `meer-queue/src/outer_seal.rs`;
  S9. **Decision:** ready.

#### T-6-12 — §6.6.2: dedup before MLS, because a duplicate application message is refused, not dropped

- **Target:** §6.6.2 line 736: *"gossip deduplicates beneath MLS and MLS applies commits
  forward-only and idempotently: a duplicate commit no longer matches current state and is
  dropped"* — used to argue that dual delivery is benign.
- **Class:** refinement.
- **What was measured:** S3 (openmls 0.8.1, real lib): a duplicate **application message is
  refused** — "The requested secret was deleted to preserve forward secrecy" — an error
  indistinguishable at a glance from tampering. The commit half of the sentence is right; it does
  not carry to application messages.
- **Diff:** "… a duplicate *commit* no longer matches current state and is dropped; a duplicate
  *application message* is **refused** by the ratchet, because its per-message secret was deleted
  after first use (forward secrecy). Content-hash dedup on the outermost sealed bytes therefore
  **MUST** precede MLS processing, because a client that feeds both copies to the key layer turns
  a normal delivery race into an alarm that looks like an attack."
- **Principle:** P-Durable-Enablement (racing the sources stays free, §6.6.4) and
  P-Knowable-Truth. **Evidence:** `meer-queue/TEST-LOG.md:216-248`. **Decision:** ready; plus a
  croft gap (§5): `receive_sealed` opens MLS first and dedups in the fold after
  (`session.rs:823-845`), masked today by iroh-gossip's message-id dedup.

#### T-6-13 — §6.6.2: one retention story

- **Target:** §6.6.2 line 730: *"Retention here is **delivery-bounded**: the meer holds a
  recipient's undelivered bytes only until they drain on dial-home"*; profile line 85: "retention
  ≥ the liveness window".
- **Class:** clarity.
- **Diff:** "Retention is **≥ the liveness window** (§11.6) and delivery-acknowledged within it;
  a sweep ends *serving*, not necessarily *holding* — a store without erasure keeps the sealed
  bytes past the window, which is a forward-secrecy fact (§8.1), not a breach." **Evidence:** S5,
  S15; `meer-queue/TEST-LOG.md:478-558`. **Decision:** ready.

#### T-6-14 — §6.6.2 / §10: the meer the workspace decided on

- **Target:** §6.6.2 describes an addressed single store; §10 names no meer realization.
- **Class:** omission.
- **What was decided and built:** the meer is a **CISS custodian-queue mode** (owner decision
  2026-08-07, `.claude/DECISIONS.md:68`; plan DRAFT, not implemented); the spike proved a
  **two-target** shape — a group queue named by an exporter output plus a personal inbox
  (S9–S14). Nothing is deployed.
- **Diff:** §6.6.2 one sentence on the two-target shape (group queue + personal inbox, both
  blind); §10.3 realization note: "Croft realizes D-meer as a mode of its storage server (CISS),
  designed, not yet deployed"; profile row per T-C-1.
- **Principle:** P-Durable-Enablement. **Decision:** ready.

#### T-6-15 — §6.8.5: name the two history-store backends actually explored

- **Target:** §6.8.5, the mirror-group store, `Design`/`Synthesis`; no backend named.
- **Class:** omission.
- **What exists:** the HIST lane proved the §G envelope over an **atproto PDS** backend live on
  bsky.social (B1–B7 `Modeled`; E1 CID-as-reconciliation-head green, B16) and CISS is the named
  "second consumer" (not built). No G-hist experiment has run.
- **Diff:** §10.3 realization note and Appendix B: "Two durable-history backends are in play for
  the reference: the persona's own PDS repo (the HIST lane; CID identity live-proven) and the CISS
  history-convergence consumer (planned). The G-hist mirror group's resumption-PSK binding is
  unexamined by either." **Principle:** P-Durable-Enablement. **Decision:** ready.

### 2.6 Part 2 §7 — synchronization and governance

#### T-7-1 — §7.6.11 describes the E12 validation as pending, against a library the experiments did not use

- **Target:** Part 2 §7.6.11, lines 1447–1451 (status banner): *"`Design; needs verification. …
  the composed operation is not yet exercised end-to-end on a real stack; the validation path is
  the E12 experiment set (below), and the whole subsection carries [confirm] until those run.`"*
  and lines 1495–1505: *"**Validation path (the E12 set, pending execution).** The mechanism above
  is demonstrated by seven experiments against `mls-rs 0.55.2`: E12.1 … E12.7 … Two library
  questions surface during execution and are not yet settled: whether `mls-rs` exposes ReInit as a
  first-class op … **[confirm]**"*
- **Class:** status-move + clarity.
- **What actually happened:** the E12 set ran **twice**. First on mls-rs 0.55.2, as the
  subsection planned — `beta/impl/delivery-layer/12-replant-experiments.md:3-5` reads "Status:
  experiment plan, EXECUTED … all 7 experiments (E12.1–E12.7) CONFIRMED against mls-rs 0.55.2";
  the crates from that round are not retained. Then re-run in-tree on **openmls 0.8.1**: E12.1,
  E12.3, E12.4, E12.5, E12.6 in `alpha/experiments/mls-replant/tests/e12_{1,3,4,5,6}_*.rs`
  (`Cargo.toml:10` pins `openmls = "=0.8.1"`); E12.2 and E12.7 in
  `alpha/experiments/replant-continuity/tests/`, E12.7 driven through the real
  `DerivedFold::ingest`. Part 2 §7.6.2 already cites the openmls run (`Verified` for the
  membership half, imported: replant-continuity @ `d52ed6f`; `Modeled`/loopback for the message
  half) and EVIDENCE-MAP rows 7.6.2 carry it. §7.6.11 was never updated to match its own §7.6.2.
  croft itself does no re-plant yet (single-member `create_group` only, `keylayer-openmls/src/lib.rs:93-97`).
- **Diff:**
  - `-` the banner's "not yet exercised end-to-end on a real stack … carries [confirm] until those run"
  - `+` "`Design` for the composed operation; the E12 set has run on two stacks (mls-rs 0.55.2, round 3, reports only; openmls 0.8.1 in-tree, Rung A, the governance chain at Rung B): E12.1 cost `Measured`; E12.3 dedup-not-fork `Verified`; E12.4 drift reset `Measured` (byte-size proxy, register `e12.4-byteproxy`); E12.5 leaf rotation and E12.6 last-resort `Verified`; E12.2 message continuity `Modeled`/loopback; E12.7 governance continuity `Verified` (membership half) — all in-process, real openmls crypto. What remains `[confirm]`: the `epoch_authenticator` fold question (Appendix B) and the stale-`GroupInfo` composed guarantee."
  - `-` "demonstrated by seven experiments against `mls-rs 0.55.2`" / the two `mls-rs` library questions
  - `+` the per-experiment evidence parentheticals; retire the two mls-rs questions as moot on the reference stack (openmls; the fresh-create path is what ran), and name the library as realization, not protocol.
- **Principle:** P-Knowable-Truth (the spec's status of a claim must be what the evidence earns,
  in both directions; here the text under-reports).
- **Evidence:** the test files named; `mls-replant/README.md` lines 1–25 ("Serves: Part 2 §7.6.2
  … landed 2026-07-13"); Part 2 §7.6.2 line 1301; EVIDENCE-MAP rows 7.6.2.
- **Grade:** as stated per experiment; all loopback / in-process, real openmls crypto.
- **Decision:** ready. One sub-item is **needs-call**: whether §7.6.11 should survive as a
  subsection at all, or fold into §7.6.2 now that its content is the same mechanism with the same
  evidence. Options: (a) keep it as the mechanics appendix of §7.6.2, re-statused; (b) fold the
  five mechanic bullets into §7.6.2 and delete §7.6.11. (a) is the smaller edit.

#### T-7-2 — §7.4: "no false current" reaches a client's own identity and reachability claims

- **Target:** §7.4 line 1180, scoped to the governance view: *"**MUST NOT** display a view as
  'current' unless …"*.
- **Class:** refinement.
- **What was learned:** the Android app renders any stored DID as "Signed in"
  (`CallScreen.kt`); a dead OAuth refresh is swallowed (`MainViewModel.kt:328`); sessions died at
  ~10, 6 and 5 days idle; both phones showed "Signed in" over "NOT camped". The honest rule
  already exists in the arc (`call-session/src/session.rs:68-72`: "A dead session says so …
  never says 'signed in'. An outage is not an authorization answer") and the fix is an ACCEPTED
  plan (`croft/plans/2026-09-28-plan-session-state.md`, five states, unbuilt). The layer argument:
  the OAuth session is a helper-facing proof, not the persona key, so the five states are
  application layer; what is protocol-shaped is the rule the build derived — a claim must come
  from a live signal, never from stored state — which is §7.4 generalized from the governance view
  to a client's claims about itself.
- **Diff:** after the beacon rule: "The same rule binds a client's claims about **itself**: a
  client **SHOULD NOT** present an identity, reachability, or admission state it has not confirmed
  against a live signal within its own measured window; stored credentials are not a session, and
  a connection that was open is not one that is."
- **Principle:** P-Knowable-Truth; Part 1 §2.6 legibility. **Decision:** ready as a SHOULD;
  **needs-call** for MUST (the spec has so far kept presentation in the profile's rider §5; a MUST
  here would be the first product-surface clause in Part 2). Croft gap filed either way (§5).

#### T-7-3 — §7.3.1: the convergence run earns `Modeled`, stated with its bounds

- **Target:** §7.3.1 line 1046: *"`Design.`"* (closing the three-key order); line 1034
  (precondition) *"`Design.`"*.
- **Class:** status-move.
- **Diff:** "`Modeled` (evidence: `drystone-convergence/tests/convergence.rs`, properties A–F and
  D6/D7, 27 green, gated in `smoke.yml` since 2026-10-27, MASTER-INDEX A7; a *reference* fold —
  no crypto, no transport, single facts not quorums, sampled; Stage 3 adversarial unbuilt). The
  content-address tiebreak is additionally `Measured` on the production fold at loopback
  (`croft-chat/tests/fold_ordering_keys.rs:97,622`)." The precondition: "`Modeled`
  (`stage4.rs` G1–G5, H1/H2) / `Measured` (`social-tree-core/tests/threshold_decisions.rs:131`)".
- **Principle:** P-Knowable-Truth — and never `Verified`; A.9's `Modeled` ("a reference model
  exists and is reasoning-complete, but is not yet backed by real crypto") fits exactly.
  **Decision:** ready.

#### T-7-4 — §7.3.2: mutual expulsion is a contradiction in the build and in §7.6.1, a tiebreak in line 1056

- **Target:** §7.3.2 line 1056: *"The **mutual-expulsion** case (A expels B while B expels A, at
  equal standing) resolves by the concurrent tiebreak (§7.3.1 key 3) … Exactly one survives,
  never both and never neither"*; versus line 1066 (E108): *"the subject of an open contradiction
  projects as neither member nor not-member but `CONTESTED`"*; §7.6.1 line 1291 lists "two
  personae at equal standing, each committing the other's removal" as the first **contradiction**
  shape; Appendix E beats E5/E6 (lines 2984–2986) follow line 1056.
- **Class:** clarity — an internal contradiction, with the build on one side.
- **What is built:** `update.rs:995-1015` `mutual_expulsion_entry` — "the pair as data, both
  parties as contested subjects, both removes withheld from the replay (no verdict)";
  `fold_pins.rs:280` `pin1_mutual_expulsion_projects_contested_both_orders`. The build never
  tiebreaks a mutual expulsion; it also carries the third shape §7.4.1 added (a departure racing a
  re-add reconciles; a ban racing a re-add contests, `removal_kind.rs:178,243`).
- **Diff:** rewrite line 1056 so mutual expulsion is the canonical contradiction (hard-stop →
  `CONTESTED` → `Resolution`), with the content-address tiebreak reserved for same-type
  concurrents that are *not* a §7.6.1 shape (two concurrent grants of the same role, two
  concurrent removals of different subjects); fix E5/E6 to match and add the §7.4.1 benign shape
  to E6.
- **Principle:** Part 1 §2.5 (no manufactured verdict where utility is open).
- **Decision:** **needs-call**: (a) the build is canonical and the tiebreak text goes; (b) keep
  the tiebreak as the "per-Group tolerance" option line 1062 names — then the build lacks the
  dial, and that becomes a croft TODO. (a) matches §7.6.1 and E108 as already written.

#### T-7-5 — §7.3.1 key 1: specify the outcome, not the algorithm

- **Target:** §7.3.1 line 1038 prescribes a layered pass ("Resolve the highest tier first, each
  tier against the settled result of the tiers above it").
- **Class:** clarity / requirement-realization split.
- **What is built:** the reference fold implements the tiers literally; the production fold
  reaches the same resolved state by sequential replay in `merge_cmp` order plus projections and
  hard-stops, with the equivalence argued in a module header (`update.rs:8-12`) and a test header
  (`fold_ordering_keys.rs:14-19`, G1's withdrawn alarm) and pinned by the arrival-order tests.
- **Diff:** state the tiers as the **required resolved state** ("the resolved state MUST equal
  the result of resolving tiers in this order") and add "an implementation MAY realize this by
  replay plus projection and contradiction hard-stop, provided the resolved state is identical on
  every arrival order" — and move the max-over-contenders fork-label rule (today only at line
  1068) into §7.3.1 where the comparator lives.
- **Principle:** P-Local-Truth (two conforming folds must agree, not share code).
- **Decision:** **needs-call**: (a) outcome-form, G1 as evidence; (b) keep the algorithm normative
  and file a croft TODO to implement the tiers. (a) is recommended: §9 already defines
  compatibility by agreement on the fold's result.

#### T-7-6 — §7.5.2: the conflicted-subgraph closure has no visible realization

- **Target:** §7.5.2 lines 1269–1271: the input set "**MUST** be closed under two distinct
  relations"; line 1273 calls it "the single most likely place for two implementations to
  disagree".
- **Class:** implementation-gap (croft) — spec unchanged.
- **What is built:** the frontier half at ingest (`update.rs:1433-1443`, every declared
  antecedent must be held); no conflicted-subgraph closure found in croft under any name; the
  harness has no stage for it.
- **Diff (repo):** croft `TODO.md` pointer; harness backlog row (open-threads §5).
- **Principle:** P-Knowable-Truth (convergence). **Decision:** ready (filing).

### 2.7 Part 2 §8–§10 — security, conformance, substrate

#### T-8-1 — §8: does "Tier-0 helper" include the relay?

- **Target:** §8 line 1650: *"a Tier-0 helper **MUST** hold no payload key and **MUST be able to
  prove it** (assert-and-log `payload_keys_held = 0`)"*; Appendix D.8 lists the relay among the
  three resource-asymmetry roles.
- **Class:** clarity, or an implementation gap.
- **What is built:** the relay holds no signing material (`config.rs:130-131`) and prints only a
  verification-key fingerprint at start (`main.rs:578-585`); **no `payload_keys_held` assertion
  anywhere** in `croft-stack/relay/source/crates`. The meer-queue spike does assert it.
- **Decision:** **needs-call**: (a) the clause covers the meer only — say so ("a Tier-0 *store*");
  (b) it covers every blind helper — then croft-stack owes one start-line field beside
  `pubkey_fingerprint` (cheap). **Principle:** P-Knowable-Truth.

#### T-8-2 — §8 and the profile: two refusal dials share one word

- **Target:** §8 line 1652 (the Group's failed-op response, `Design`); profile line 76 and 132
  ("loud *(provisional)*").
- **Class:** clarity.
- **What is built:** fabric refusals are loud and fully enumerated (`ENFORCEMENT-SCENARIOS.md`;
  WARN per refusal, never the proof); the Group-level signed, corroborated rejection does not
  exist. **Diff:** the profile carries both rows — "Refusal verbosity (fabric): loud, live";
  "Refusal verbosity (Group): loud, `Design`, unbuilt". **Decision:** ready.

#### T-9-1 — §9 / §6.9: the directory is the live interop surface, and the spec cannot see it

- **Target:** §9 line 1684 (what two implementations must agree on); §6.9 line 884 ("the one
  part of §6 the specification treats more lightly than it will ultimately need"); the profile
  §2.4 pins (signature, hash, ciphersuite, tag namespace, byte register).
- **Class:** omission.
- **What is built:** the only cross-implementation surface in live use is the **directory
  contract**: v2, `ing.croft.iroh.endpoint` / `.call.grant` / `.call.policy`, consumed by croft
  Android, `call-session/src/atproto.rs`, connect web and the admit service; lexicon JSON landed
  unpublished (connect #10; E161); readers fail closed ("A record with no `endpointId` is
  malformed (not 'not listed')", `contract.md:52`; validate every foreign record on the way in).
  §4.6's dag-cbor MUST does not reach these JSON records, and should say so.
- **Diff:** (i) a profile `[wire]`/`[deployment]` row: "persona-to-endpoint directory: record
  types, their version clock, the reader's fail-closed rule — Croft declares contract v2, the
  three `ing.croft.*` types (unpublished), malformed ≠ not listed"; (ii) §4.6: "Directory records
  a persona publishes about its endpoints (§6.9.2) are not Drystone facts; they carry their own
  version clock and are never hashed into the fold"; (iii) optionally a §6.9.3 "The directory"
  subsection making §6.9's admission into text.
- **Principle:** P-Durable-Enablement (the dial a second implementation must match to call
  anyone); P-Knowable-Truth (fail closed on foreign records, the §4.4 separation at the
  directory). **Decision:** **needs-call**: (a) profile row + the §4.6 sentence only;
  (b) also §6.9.3. 

#### T-10-1 — §10.3 T7: scope "authority-bearing"

- **Target:** §10.3 line 1860, T7: *"Not require a wall-clock for any authority-bearing
  decision"*.
- **Class:** clarity, or an implementation gap.
- **What is built:** the fabric pass has wall-clock `exp`/`iat` with 30 s leeway
  (`token.rs:188-194`), a 43,200 s camping TTL, RFC 3339 `expires` in grant policy
  (`caps.rs:339-345`); the clock is injected, never read in the core (`token.rs:19-27`).
- **Diff:** "T7: Not require a wall-clock for any decision that bears on **Group standing,
  ordering, or an authority bound** (Part 1 §2.0.1). A fabric lease — a relay's attachment pass —
  is not standing and **MAY** expire by wall-clock, provided its expiry can refuse only
  reachability, never a Group act."
- **Principle:** the razor (a lease is not standing; do not technicalize it as one).
  **Decision:** **needs-call**: (a) scope T7 as above; (b) keep T7 broad and file a croft-stack
  gap (epochs-or-generations lease lifetime), which the fabric has no clock for. (a) is
  recommended — the fabric has no governance generation to count.

#### T-10-2 — §10.2 / §10.3: realization notes the build earned

- **Target:** §10.2 "Reference realization" (line 1747), §10.3 "Reference realization" (line
  1872), §10.5 ledger.
- **Class:** omission (realization detail with no home).
- **Diff — §10.2 (MLS):** name the library and its limits: openmls 0.8.1, bare-signature
  (`BasicCredential`), `MLS_128_DHKEMX25519_AES128GCM_SHA256_Ed25519`; the resumption PSK is
  unreachable through external commit on this stack (S16, already in §11.6);
  `extensions-draft-08` export-tree methods uncompilable (`keylayer-openmls/src/store.rs:40-55`);
  the key layer is a port that never answers "admit?" (ADR-0003; S16's failure class). Two
  reference stacks exist in the corpus (mls-rs 0.55.2 in rounds 1–3; openmls since); say so once.
- **Diff — §10.3 (iroh):** the client runs iroh 1.1.0 against an iroh-relay 1.0.3 in production;
  camp-at-attach ("the token is the cache": hours-scale TTL, re-presented across reconnects,
  re-minted on refusal, `camp.rs:21-26`); **R0 — a dial never lowers admission**
  (`call-core/src/dial.rs:146-168`, measured §15.3: a tokenless re-attach under enforce was
  refused ~20× and the phone unreachable for four minutes); the fabric capability's shape
  (JWT/EdDSA, three gates, revocation = expiry + grant deletion, ADR-0003; sponsorship
  `BudgetBytes(262144)` introduction); the gossip broadcast ceiling of 4096 bytes and the
  Inline/Elsewhere record form it forced (`record.rs:11-27`); the severance as a crate boundary
  (ADR-0004) as the recommended shape for §6.1.1's MUST NOT; the `N0DisableRelay` trap; the
  relay flap as a `Measured` residual (runbook §17: one phone re-makes its relay connection every
  1–8 min at rest, the other never; cause unnamed; E128 first).
- **Diff — §10.5 ledger:** a "Directory" row (T-9-1) and a "Local store" row (redb 2.6.3 in
  croft, 4.1.0 in CISS; §4.6 says storage is free, so this is profile-grade, listed for the
  split).
- **Principle:** the §10 discipline itself. **Decision:** ready (owner trims).

#### T-A11 — Appendix A, the capability-mechanism track: what the build resembles

- **Target:** Appendix A line 2752 (Track A vs Track B, "deferred"); ROADMAP A11.
- **What exists:** the relay's capability is a signed per-endpoint token with expiry and
  grant-deletion as revocation (ADR-0003; "minutes-scale"); in-Group, re-issue-not-revoke-in-place
  (`group-principal-seam/tests/seam.rs:92`, `Design`) over per-Group epoch keys. Nothing
  Keyhive-shaped exists. **Resemblance: Track A** — with one divergence: the relay's bound is
  wall-clock seconds where line 2752 requires "an epoch or generation bound, not a wall-clock
  interval" (T-10-1), and the relay token is a *fabric* capability, not §5.5's data-access
  capability.
- **Decision:** **needs-call**, framed as options: (a) ratify Track A de facto, declaring the
  wall-clock expiry a fabric-plane profile attribute; (b) keep A11 uncommitted and record the
  build as "Track-A-shaped where it exists"; (c) do Appendix A's own named next step first
  ("define Drystone's revocation needs concretely") — still undone. No silent fold either way.

#### T-A12 — key custody: what the build does

- **Target:** §7.3.9 ("the **meer is always blind** … never becomes the recovery role"); ROADMAP A12.
- **What exists:** MLS state lives entirely on the adapter side over on-device redb
  (`ports/keylayer.rs:19-23`: "no method here exposes key material: backup/export/recovery is the
  named-but-not-designed `KeyCustody` seam"); the admit service holds no group keys; the broker
  vault holds OAuth sessions AES-GCM at rest (identity plane); CISS is a blind byte store; the only
  recovery artifact is `bip39-recovery-roundtrip` (EM row 83), not wired in. **Resemblance:
  Option A by omission** — no readable delegate exists, and nothing yet serves even Option A's
  sealed key material.
- **Decision:** **needs-call**: (a) ratify Option A as the default and make "history can be lost"
  a stated product string (rides E116); (b) leave A12 open, since the build's state is absence,
  not choice. The ROADMAP's own question — what structurally resists Option B creeping in — is
  untouched either way.

### 2.8 Part 2 §11 — large-group scaling

§11 is spec-ahead-of-build by design: no hot/cold split, liveness window, PCS cadence or
re-entry door exists in production (profile check, §6.1). One item.

#### T-11-1 — §11.9.1: a message layer §5 does not describe

- **Target:** §11.9.1 line 2378: *"… seeded from the MLS exporter* rather than distributed
  pairwise (Part 2 §5)"* — a per-sender symmetric hash ratchet seeded from the exporter.
- **Class:** clarity (a wrong cross-reference and a mechanism §6.2.2 contradicts).
- **What the spec says elsewhere:** §5 says nothing of the kind; the exporter appears only at
  §6.2.2 (610), §6.4 (672, 682) and here; §6.2.2 and the build use `PrivateMessage` directly.
- **Diff:** either delete the clause (the message layer is `PrivateMessage`, §6.2.2) or mark it
  as the >7k public-regime variant with its own tag and a cross-reference to §6.2.2, not §5.
- **Decision:** **needs-call**, in plain English: does the large-group design intend a different
  message layer from the small-group one? If no, delete; if yes, say so where §6.2.2 can see it.

### 2.9 Appendices

#### T-B-1 — Appendix B, the beam: say what ran

- **Target:** Appendix B line 2774: *"Its discharge path is the convergence experiment
  (permutation-invariance of the fold and, the load-bearing half, gap-detection …). Until that
  passes, the claim is conditional and unearned"*; `open-threads.md:19` "build the harness".
- **Class:** refinement (status unchanged).
- **Diff:** one sentence: "The permutation half ran green on the reference fold (2026-10-27,
  A7); the gap half ran and *reproduced* the limit — a referenced gap is detected
  (`stage2.rs:33`), an unreferenced tail gap is not (`stage2.rs:129`, the documented case); the
  four obligations above and Stages 3, 5, 6, 7, 8 remain." `open-threads.md`: "build" → "extend".
  **Status stays `Load-bearing, unearned`.**
- **Principle:** P-Knowable-Truth (the record says what ran, in both directions). **Decision:** ready.

#### T-E-1 — the evidence set: a production fold exists, and ten test files have no map row

- **Target:** `drystone-convergence/RESULTS.md:3-11` ("No production implementation of the
  Drystone governance fold exists in this repository"); `EVIDENCE-MAP.md` (no row cites
  `social-tree-core`, `fold_pins`, `threshold_decisions`, `removal_kind`, `fold_ordering_keys`,
  `contested_projection`, `helper-seam`, `drystone-convergence`, E117 or G1); `MASTER-INDEX.md:86`
  A7 ("The spec's evidence map does not cite it yet").
- **Class:** omission + clarity.
- **Diff:** RESULTS.md keeps its sentence (true of its origin repo) and gains one: "A production
  fold exists in `croft/core/social-tree-core` (E117 P2, 2026-08-23); this harness tests the
  specification's reference fold, not that implementation." EVIDENCE-MAP gains rows for every
  file named above, with grades per matrix 1.4; row 46's "reviews-log" pointer becomes
  `drystone-convergence/tests/convergence.rs`; the A7 note then retires.
- **Principle:** P-Knowable-Truth. **Decision:** ready.

#### Appendix D and E

- **D:** add **fabric admission** / **Group admission** (T-6-2); **session control** as a payload
  class if T-6-10 lands; note that the reference's `DeviceId` *is* the client's signing key (T-4-3).
- **E:** beats E5/E6 to carry `CONTESTED` and the §7.4.1 benign shape (T-7-4).

#### T-F-1 — Appendix F lags the body on two reference rows

- **Target:** Part 2 Appendix F, line 3112: *"**Negentropy**. A range-based set-reconciliation
  protocol, one candidate construction for RBSR (§6.8.1), not yet chosen (Appendix B)."* and the
  "Reference implementations and crates (all n0-computer)" group, lines 3078–3086, which lists no
  MLS library.
- **Class:** clarity.
- **Diff:**
  - `-` "one candidate construction for RBSR (§6.8.1), not yet chosen (Appendix B)"
  - `+` "the recommended construction for the linear `(device, lamport)` key space (§6.8.1; `beta/impl/drystone-design/rbsr-construction.md`, Part 3a), landed at loopback grade (RUN-12 Part 3b); the wire fingerprint stays `[gates-release]`."
  - `+` a row for **openmls** (the MLS reference stack §10.2 and the profile name, version per
    the survey) under a group retitled "Reference implementations and crates" without the
    "(all n0-computer)" qualifier.
- **Principle:** readability (every inline citation resolves to one entry, and the entry agrees
  with the body — Appendix F's own stated rule, line 3016).
- **Evidence:** Part 2 §6.8.1 line 824; §10.2 line 1749 and the profile §2.4 ("openmls 0.8.1
  reference stack"); Appendix F line 3016.
- **Decision:** ready (both rows; the openmls pin is `=0.8.1`, `keylayer-openmls/Cargo.toml:15`).

---

## 3. Omissions: built or learned, and no home in the spec

For each: what it is, where the evidence lives, and where it belongs — Part 2 normative, Part 2
§10 realization, the implementation profile, Appendix B (open), or **out of scope** (application
layer; the spec stays lean, ROADMAP A19).

| What | Where it lives | Where it belongs |
|---|---|---|
| **Fabric admission** as a named event: camp-at-attach, `enforce`, the three-gate pass, "the token is the cache" | croft-stack `token.rs`, `camp.rs:21-26`; ENFORCEMENT-SCENARIOS; the enforce flip session | §6.5.2 one MAY sentence (T-6-2); §10.3 realization + profile `[deployment]` rows (fabric posture, camping TTL, re-mint rule). Not Part 2 core. |
| **R0 — a dial never lowers admission** | `call-core/src/dial.rs:146-168`; §15.3 measured; §16 device-verified | §10.3 realization note; profile rider. It is a property of an admitting relay's client, not of the protocol. |
| **E153 — "tokens never verified"**: an attributed log line proved *presentation*, not *verification*; production ran v0.1.1 for five days | `croft-stack/sessions/2026-08-28-e153-tokens-never-verified.md:21-55,101-130` | `conventions-and-decisions.md` A.9 evidence-linkage notes: an instrument is evidence only if you know what it cannot show — the fabric-plane twin of A3/S16. |
| **The enforce flip** (2026-08-30, live-verified; v0.2.1 since 2026-09-24) | sessions; `DEPLOYED.md` | profile §3 as a dated `[deployment]` fact. |
| **The relay flap** (one phone re-attaches every 1–8 min at rest; cause unnamed) | runbook §17 lines 1073–1213 | §10.3 realization residual, `Measured`; Appendix B only if camping stability becomes load-bearing for presence. |
| **The E129 ending vocabulary** ("you ended the call" / "call ended: closed by peer: hangup (code 0)"; both sides stay camped) | runbook §16:808-812 | profile §5 rider (presentation obligation 5); out of Part 2. |
| **Call signaling as a payload class** | `call-transport-iroh/src/lib.rs:39` | §6.8.4 (T-6-10). |
| **"One stream"** = one client, per-pond cores, calling attaching at the rendered-principal seam (R5–R8 unstarted) | ADR-0002; the parent plan | out of scope (product architecture); one clarity note in T-6-10. |
| **The backgrounded-callee limit** ("not callable, and the caller only learns it after 20 s") | `croft/TODO.md`; PLATFORM-POSTURE | §6.7.2 already says wake is an accommodation; add the floor's user-visible consequence as a profile rider line. |
| **Lexicon record types as the interop surface**; the unpublished-namespace stage | `connect/lexicons/`; `.claude/LEXICONS.md:210-247` | T-9-1. |
| **Two identity roots and the S3 binding fact** | `AuthManager.kt:44`; `session.rs:123-125` | §5.2 seam (T-5-2). |
| **Session state** (stored / proven / unconfirmed / dead) | the ACCEPTED plan | application layer; the rule it derived is T-7-2. |
| **Meer = CISS custodian-queue mode; the two-target meer** | DECISIONS.md:68; S9–S14 | T-6-14. |
| **History-store backends explored** (PDS repo live; CISS planned) | HIST lane; CISS README | T-6-15. |
| **A reinstall clears the endpoint identity** → a directory record that lies until `rkey=self` is republished | `.claude/TESTBED.md:44-52` | one §6.9.2 sentence: a key that does not survive the app is a record that lies; otherwise testbed. |
| **openmls 0.8.1 limits the build hit** (export-tree methods; PSK via external commit) | `keylayer-openmls/src/store.rs:40-55`; S16 | §10.2 realization (T-10-2). |
| **Five redb table families and MLS provider state in redb**; crash-shaped persistence pin | `tables.rs:93-215`; `persistence_pins.rs` | `redb-storage-contract.md` (§6.3). |
| **Mutation evidence**: `fold_derived.rs` 54/115 caught; `store-redb` never mutated since promotion | `MUTATION_TESTING.md:211` | out of spec; croft TODO. |
| **Non-`Debug` secret discipline on `Endpoint`** | `transport.rs:101-110` | profile hygiene row; out of Part 2. |

---

## 4. Requirement/realization splits

Where normative text states a library's behaviour, or one realization's shape, as the protocol.

| Spec § | Today | Proposed | Item |
|---|---|---|---|
| §4.6 | "**MUST** be … (dag-cbor)" | the MUST is determinism; dag-cbor the reference | T-4-4 |
| §4.5 | one lineage realization (signed credential chain) | name the directory-authority binding as a weaker, disqualified-alone realization — or keep it out | T-5-1 |
| §6.6.5 | "the real credential-validation hook at commit-build time" (mls-rs) | the requirement: policy before the commit is built; two realizations | T-6-9 |
| §6.9.2 | three resolvers | five, in the section's own frame (who observes / withholds) | T-6-1 |
| §7.3.1 key 1 | a layered algorithm | the required resolved state; replay + projection as a conforming realization | T-7-5 |
| §10.2 | "MLS" with no library named; the corpus names two | openmls 0.8.1 as the reference stack, with its limits; mls-rs rounds as history | T-10-2 |
| §10.3 T7 | "any authority-bearing decision" | Group standing, ordering, bounds; a fabric lease may be wall-clock | T-10-1 |
| profile §3 | "running on the SHA-256-proven §4" | BLAKE3 in production; vectors not re-run | T-4-1 |

---

## 5. Implementation gaps (filed against the owning repo, not the spec)

Build falls short of a spec MUST, or of a declaration the profile makes for it. Each row becomes a
pointer in that repo's `TODO.md` (own worktree, own PR), never a loosening of the MUST. Where the
clause's scope is itself a needs-call (T-8-1, T-10-1), the row waits on the call.

| Repo | Clause | What is missing | Pointer |
|---|---|---|---|
| croft | §4.5 / §4.5.1 MUST (signed client credential descending from the persona's key) | the leaf asserts the principal id; no root-signed credential; one-device-one-persona | S3, E120 (`plans/2026-08-25-1-plan-product-shell-adoption.md:804`) → T-5-1 |
| croft | §6.4 MUST (outer seal, epoch rule, wrap-once) **when sealing is ON — and the profile declares ON** | no outer seal in `core`/`ports`/`ffi`; exists only in the meer-queue spike | E96 unwired → T-C-1 |
| croft | §6.6.2 (as refined by T-6-12) — dedup before MLS | `receive_sealed` opens MLS first; no seen-set on sealed bytes; masked by gossip dedup | T-6-12 |
| croft | §7.5.2 MUST (conflicted-subgraph closure) | frontier half only (`update.rs:1433-1443`) | T-7-6 |
| croft | §9 "**MUST** pass the conformance vectors" | signing slice only; the fold is BLAKE3, the vectors SHA-256; the profile's "scoped to the §9 vectors" is not yet earned | T-4-1 |
| croft | §7.4 as generalized (T-7-2), and the repo's own rule ("a surface never claims a capability it has not confirmed") | one-bit session state rendered as "Signed in"; plan ACCEPTED, Phase 1 not started | the session-state plan |
| croft | §6.6.1 D-self floor | no buffer or retry; `Lagged` = drop + WARN | spec-ahead; file as a floor obligation |
| croft | §6.8.1 / §6.6.3 | no steady-state reconciliation beyond the join-time inline record (4096-byte ceiling) | spec-ahead; the RBSR construction exists at loopback in discovery |
| croft | E128 (the flap) | `tracing` events exist at the FFI boundary; no logcat subscriber installed | `croft/TODO.md` already carries it |
| croft | §7.6.2 re-plant | single-member `create_group` only; no re-plant, no device-Group | spec-ahead |
| croft-stack | §8 "assert-and-log `payload_keys_held = 0`" | no such assertion in `relay/source/crates` | **withdrawn** (owner decision 36: blindness is structural, the log line proves nothing) |
| croft-stack | §10.3 T7 | wall-clock `exp`/`iat`, camp TTL, RFC 3339 `expires` | **held** until T7's two-rule wording settles (owner decision 39) |
| croft | §4.5.1 naming | the build's `DeviceId` is the spec's *client* | rename across crates + FFI (owner decision 10) |
| croft | §4.6 as decided | fixed-layout envelope; dag-cbor is the reference | migrate to dag-cbor at the wire-freeze (owner decision 11) |
| croft | §6.10.4 three-state send | `transport.rs:331-335` notes the trap | check whether "swarm empty" is surfaced to the user or only logged (owner decision 21) |
| croft | §7.3.2 fork, not verdict | `CONTESTED` held; fork posture "product-defaulted, defaults unchecked" | check whether the fork is projected as two followable lineages, with local follow policy (owner decision 33) |
| croft | §7.3.9 / A12 **high priority** | `KeyCustody` seam "named but not designed"; BIP39 round-trip unwired | a protected archive: export, store away, reimport — root + signed identity mappings (owner decision 42) |
| croft-stack | profile "Mode 1 (relay + meer)" | no meer service in the converge | T-C-1 |
| CISS | §6.6.2 D-meer realization (as decided: the custodian-queue mode) | custodial write refused 403 (S12); no object DELETE or retention (S5, E95); no queue kind; no `GroupInfo` channel (E105); history-convergence consumer not started | the DRAFT plan (`docs/plans/2026-08-24-1-plan-meer-custodian-queue.md`) |
| connect | the interop surface (T-9-1) | lexicons unpublished: no `_lexicon.croft.ing` TXT, no `com.atproto.lexicon.schema` records | E161 |
| discovery | EVIDENCE-MAP rows; RESULTS.md sentence; the round-1–3 crates | ten files with no map row; "no production fold" true only of the origin repo; the mls-rs crates behind four `Verified` tags are not retained | T-E-1, T-6-8 |
| discovery | ROADMAP E113 status drift | `ROADMAP_TODO.md:244` says "RESOLVED 2026-08-20"; the ACCEPTED plan closes it differently (D3(c)); `croft/TODO.md` still lists a WorkManager design the plan rejects | a one-line reconciliation |

---

## 6. Companion updates (contingent on the Part 2 decisions above)

### 6.1 `implementation-profile.md` — the Croft column against what Croft runs (T-C-1)

Read dial by dial against DEPLOYED.md, the relay config and the lockfiles.

| Dial | Croft column says | What Croft runs | Verdict |
|---|---|---|---|
| Serving door | B — token | Group plane: the **merge-side** cross-check only (`admission.rs:125-133,173-212`); no serve-side door in core; the *transport* door is enforced at the relay | partial — merge gate real; serve door B not found |
| Issuance timing | at-join | `TokenIssuance` facts folded (`admission.rs:215-236`); the minting site not visible from the core | could not check |
| Token lifetime | non-expiring, standing-checked | no expiry field in the Group token path | match |
| Serve posture | liberal-serve / strict-merge | strict-merge: `StandingExcluded` / `StandingContested` / stall fail-closed; serve side absent | partial |
| Resolution threshold | 2 | `charter.rs:35` `resolution_threshold: 2` | match |
| Readmission threshold | (spec default 1; no Croft row) | `charter.rs:38` `readmission_threshold: 1`; wire v4 | match — add the row |
| Exit voice | leaver's choice | `RemovalKind::Departure` vs quorum ceiling (`update.rs:185-195`) | match |
| Confidentiality regime | confidential | MLS AEAD is the only content protection | match |
| **Sealing (outer seal)** | **ON, epoch rule + wrap-once in force** | **not in the client** (spike only) | **mismatch** → "declared ON, not yet wired (E96)" |
| `carrier-visible` | not declared | nothing found | match |
| Refusal verbosity | loud (provisional) | fabric: loud and enumerated; Group: unbuilt | split into two rows (T-8-2) |
| **Deployment mode** | **Mode 1 (relay + meer; relay.croft.ing)** | relay live; **no meer** deployed or implemented | **mismatch** → "Mode 1: relay live; meer designed, not deployed" |
| Liveness window / hot-N | dynamic, ~1500 | no liveness, hot-N or dormancy code | not implemented — say so |
| Retention | ≥ window | nothing in core; CISS compaction is the only lever | not implemented |
| PCS posture | strict <250 / 24 h | no PCS cadence code | not implemented |
| Signature | Ed25519 | ciphersuite `…_Ed25519`; admit key PKCS#8 ed25519; two `ed25519-dalek` majors co-resident in croft's lock | match (note the two majors) |
| **Hash** | **BLAKE3 committed; "running on the SHA-256-proven §4"** | **BLAKE3 throughout** `social-tree-core`; vectors not re-run; CISS's address of record is SHA-256 | wording false → T-4-1 |
| Ciphersuite / credential | openmls 0.8.1, bare-signature | `=0.8.1`; `BasicCredential` | match |
| Tag namespace | `croft` brand tags until re-proof | and still minting them | match; register rule (T-4-2) |
| Wire pins | none pinned | `WIRE-REGISTER.md` "experiment-grade, awaiting freeze" | match |
| Conflict response | hold-on-conflict | `CONTESTED` blocks admission and enactment | match |
| Fork posture / horizon N | product-defaulted | `project/horizon.rs` exists; defaults unchecked | could not check |
| *(no row)* | — | iroh 1.1.0 (croft) vs 1.0.3 (croft-stack, CISS); redb 2.6.3 vs 4.1.0; jsonwebtoken 11.1.0; k256/p256 0.14 (relay) vs 0.13 (croft); rustls 0.23.45; envelope author fields; directory contract v2; fabric posture `enforce`, camp TTL 43,200 s, mint TTL 900 s, QoS 1 MB/s / 4 MB burst | **add rows** (T-4-3, T-9-1, T-10-2) |

### 6.2 `EVIDENCE-MAP.md`

Add rows for: `drystone-convergence/tests/convergence.rs` (A–F, D6/D7) and `stage2.rs`, `stage4.rs`
(§7.3.1, §7.3.2, §11.11 #3 — `Modeled`); `social-tree-core/tests/fold_pins.rs`,
`threshold_decisions.rs`, `removal_kind.rs`, `admission_pins.rs`, `admission_facts.rs` (§7.3.2
Resolution, §7.4.1, §10.2.2 — `Measured`); `croft-chat/tests/fold_ordering_keys.rs`,
`contested_projection.rs` (§7.3.1 key 3, §7.3.2 `CONTESTED` — `Measured`, loopback);
`helper-seam/tests/helper_seam.rs` (§3.2 — `Verified`, loopback). Replace row 46's reviews-log
pointer. Retag the four §6.6/§6.7/§6.8.3 rows per T-6-8 with "report only; crates not retained".
Retire the MASTER-INDEX A7 note. (T-E-1.)

### 6.3 `open-threads.md`, the companions, and Appendix B

- `open-threads.md:19` "build the harness" → "extend the harness" (Stages 3, 5, 6, 7, 8 remain;
  §5 already says Stages 1, 2, 4 ran). Add: the §7.3.1 key-1 equivalence rests on a test header
  (T-7-5); the key-2 lamport question (§8 item 1); the §7.5.2 closure gap (T-7-6).
- `fold-semantics.md:99` says order-independence is "`Verified` (Proofs Part A)" where Part 2 tags
  it `Design` — align to T-7-3's `Modeled`; close its open items at lines 113–115 (the tiebreak
  key is the content address; the escalating classes are built).
- `redb-storage-contract.md`: five table families, MLS state in redb, no iroh-blobs in croft, the
  redb major split, the crash-shaped pin; resolve or re-date its three `[confirm against redb docs]`.
- Appendix B: T-B-1; retire the mls-rs ReInit question (T-7-1); add the two history-store
  backends (T-6-15).

### 6.4 `CHANGELOG.md`

See T-R-4.

---

## 7. Cross-repo follow-ons (named, not edited here)

| Repo | Doc | Why it changes |
|---|---|---|
| CroftC | `.claude/ARCHITECTURE.md` (the croft-stack and connect cards; the lexicon seam) | if T-6-2 names fabric vs Group admission, the cards should use the same two words; the connect card should name the directory as the interop surface (T-9-1) |
| CroftC | `.claude/LEXICONS.md` | T-9-1's "directory records are not Drystone facts" is the LEXICONS rule stated from the spec side; cross-link |
| croft-stack | `docs/ENFORCEMENT-SCENARIOS.md` | T-8-2 (fabric refusal dial); T-10-1 if the lease-lifetime scope lands |
| croft-stack | `relay/source/DESIGN.md` | T-6-1 cites it for the PDS-record resolver; if §6.9.2 gains the bullet, DESIGN.md should point back |
| croft | `core/social-tree-core/WIRE-REGISTER.md` | T-4-2's register rule; T-4-3's envelope pin |
| croft | `docs/adr/0003-keylayer-port.md`, `0004-call-transport-port.md` | T-10-2 cites them as the recommended realization shapes |
| croft | `CLAUDE.md` § calling / chat | T-6-10's "one stream ≠ one lane" note |
| connect | `docs/contract.md` | T-9-1: the contract becomes a declared profile row; say so at the top |
| CISS | `README.md`, the custodian-queue plan | T-6-14/T-6-15 name CISS as the meer and history-store realization; the README's "not built yet" stays until it is |
| discovery | `alpha/experiments/drystone-convergence/RESULTS.md`, `MASTER-INDEX.md` | T-E-1 |

---

## 8. Could not resolve

1. **Whether croft's lamport is a Lamport clock or a bare per-device counter** — only the
   per-device monotonicity check was found (`update.rs:1423-1431`); whether a device bumps past
   its antecedents' lamports decides whether `merge_cmp` replay honours happens-before across
   devices (§7.3.1 key 2) or whether key 2 rests on the ingest gate alone.
2. **The evidence behind §6.9.1's "Co-location … `Verified` (measured)"** — no EVIDENCE-MAP row;
   nothing in croft or croft-stack measures two-relay reachability (one relay plus a staging
   listener exist); likely in `relay-lab-runs`, not opened.
3. **Whether any test backs the §6.6.5 / §6.7 / §6.8.3 `Verified` tags** — searched `Proofs/`,
   `experiments/`, EVIDENCE-MAP, both changelogs; only the seed-corpus sentence was found.
4. **"admission denies a non-listed node" (§6.6.2)** — not in `meer-queue/TEST-LOG.md`, `06-R2`
   or `07-R3`; only `SPEC2-OVERLAY.md:42` repeats it.
5. **The round-1–3 crates** (`e0-probe … e5-integration`) — not found anywhere under `CroftC/`;
   the E3.3/E3.4/E5.1/E5.2 tests cannot be re-run.
6. **Whether croft's invite seam binds the key package's leaf signature key to its asserted
   principal bytes** — `identity.rs` and `lib.rs:125,452` read; the invite path not traced end to
   end. If it does not, T-5-1's gap is live on the hand-carried pairing path today.
7. **Whether the Group re-entry token is minted at the Add-commit** (the profile's "at-join") —
   the core only folds `TokenIssuance` facts; the minting site is in an adapter or shell.
8. **Whether §7.5.2's conflicted-subgraph closure exists in croft under another name** — only the
   frontier gate was found; no exhaustive search of `store-redb`.
9. **What `fold-semantics.md:99` "Proofs Part A" folded** (R1–R4 or the older reconcile).
10. **The HyParView view sizes and PlumTree PRUNE/GRAFT behaviour at iroh-gossip 0.101.0** — the
    build reads none of them; left `[confirm]`.
11. **`DEPLOYED.md` is dated 2026-09-24** and is the ansible declaration, not a probe of the box
    (VERSIONING.md:105); "what is running" here means "what is declared".
