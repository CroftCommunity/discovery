# Drystone Part 2 §7, Large-Group Scaling, Dormancy, and Re-entry

`Resolution: library (full specification). See Rule 16; the coffee-shop and pitch resolutions derive downward from this.`

`Status: design section. Most clauses here are Design or Synthesis; the two measurements in §7.11 are unearned. Read the A.9 tags, not the confidence of the prose.`

`Realizes: Part 1 principles of local authority, non-domination, provenance-over-secrecy at scale, and helper-not-authority. Clause-level Realizes markers appear inline.`

`Companion to Part 2 §5 (MLS subordination and access control) and Part 2 §6 (transport and delivery). This section depends on both and does not restate them.`

---

## Legend

Status flags, normalized to the A.9 ladder (Rule 6):

- ***Verified-RFC***: verified against a normative primary (RFC 9420, RFC 9750, or an IETF draft at a pinned revision).

- ***Established***: an established result in the literature, or an inherited primitive used as-is.

- ***Design***: specified but unproven; a decision this section commits to.

- ***Synthesis***: a claim assembled across several sources or several primitives rather than resting on one citation.

- ***Load-bearing, unearned***: a property the design leans on that is not yet earned by proof or measurement.

- **[confirm]**: rests on an external fact reachable but not yet independently verified here; do not treat as settled.

- **[gates-release]**: a byte-level or wire-format choice that must be pinned before public release.

Linkage markers, not status: **Realizes: P-X** (a clause discharging a Part 1 principle); section cross-references written as `§7.N` within this section and `Part 2 §N` across sections.

---

## 0. Map

`## 0. Map` per Rule 15. One line per section: scope, dependencies, orthogonality.

- **§7.1 Terms**: the vocabulary this section coins and inherits. *depends on:* Part 1, Part 2 §5, Part 2 §6 conventions. *orthogonal to:* nothing; everything reads against it.

- **§7.2 The scaling claim, and what it is not**: the outcome stated for the reader who needs only it, tied to principles. *depends on:* §7.1. *orthogonal to:* the mechanics that follow.

- **§7.3 Cast**: the demonstrative personae and the infrastructure roles this section draws on. *depends on:* §7.1. *orthogonal to:* the normative content, which stands without it (Rule 4).

- **§7.4 Requirement: cost scales on the live set, not the roster**: the central requirement and its realization. *depends on:* §7.1, Part 2 §5. *orthogonal to:* §7.9 (delivery) at the mechanism level.

- **§7.5 Scaled post-compromise security**: the strict/opportunistic PCS gradient and its threshold. *depends on:* §7.4, Part 2 §5. *orthogonal to:* §7.7 (re-entry) except at the credential.

- **§7.6 Hot and cold groups; liveness-driven migration**: the two-group structure and the migration rule. *depends on:* §7.4. *orthogonal to:* §7.9.

- **§7.7 Re-entry: the two-part credential**: how a cold persona returns at its own cost. *depends on:* §7.6, §7.8, Part 2 §5, Part 2 §6. *orthogonal to:* §7.9.

- **§7.8 The governance chain, bans, and standing resolution**: the single ordered authority for standing, and the determinism it rests on. *depends on:* §7.1, Part 2 §5. *orthogonal to:* the key layer (that is the point).

- **§7.9 Delivery under scale: the race, and graceful degradation**: why fan-out is a curve, not a ceiling. **§7.9.1** states the Force-1/Force-2 encryption posture (why Drystone can be large and encrypted); **§7.9.2** specifies the optional AppView-shaped public-projection read cache; **§7.9.3** sketches the *experimental* public-by-default regime above ~7k (MLS retained for attestation, message confidentiality conceded), with **§7.9.3.1** the experimental MLS-aware relay bridge feeding a privately-hosted AppView, **§7.9.3.2** the visibility/authority decoupling (read wide, membership as tight as local authority desires), and **§7.9.3.3** the attesting-core ceiling and scaling it past one tree via RFC 9750 parallel groups and Group-as-principal federation. *depends on:* Part 2 §6, §7.4, §7.6, §7.13. *orthogonal to:* §7.5, §7.8 at the mechanism level. *status:* §7.9.3 and its subsections are Design-experimental, more speculative than the rest of §7.

- **§7.10 Tiered performance expectations**: 0–1k, 1–3k, 3–7k, 7–10k, against the fixed policy; **§7.10.1** turns the tiers into a buildable experiment matrix (variables, procedures, thresholds). *depends on:* §7.4, §7.5, §7.6, §7.9. *orthogonal to:* §7.8. *feeds:* the two measurements of §7.11.

- **§7.11 Open items and the two unearned measurements**: what is genuinely undecided. *depends on:* all above. *orthogonal to:* nothing; it is the residual register.

- **§7.12 Posture summary**: one row per MLS-and-substrate case, per Rule 10. *depends on:* all above. *orthogonal to:* nothing; it is the index.

- **§7.13 Empirical basis**: the group-size and community-moderation evidence the design's premises rest on, tagged for what is established shape vs inferred number. *depends on:* the platform sources in Part 2 §6 references. *orthogonal to:* the mechanics; it is the warrant beneath §7.2 and §7.4.

- **§7.14 Research prompt (obligation)**: the standing task to quantify the inferred per-group rates from ancillary evidence. *depends on:* §7.13. *orthogonal to:* the mechanics; the full prompt is a companion artifact.

---

## 7.1 Terms

Per Rule 2, every load-bearing term is defined here, including inherited ones, with the source of inheritance noted. Case and qualifier carry which plane a term stands on (Rule 12).

- **group** (lowercase, social plane), a body of people who coordinate, whether or not they run on a protocol.

- **Group** (capital-G, technical plane), Drystone's in-system principal holding continuously-rotated key material shared among its member clients. *Inherited from Part 1/Part 2 §5.* MLS realizes a Group via a **group**'s (MLS-lowercase, the RFC's own object) ratchet tree; the mapping is stated where used.

- **persona** (plural **personae**), the identity term the mechanics use: a human-layer principal with standing, distinct from personhood, whose binding to one person is left to group judgment. *Inherited from Part 1.*

- **node**: a device or helper; never the person. A persona *has* devices that are nodes.

- **author lineage** (or **lineage**), the durable cryptographic identity a persona presents across device changes and key rotations. In this section, **the principal that standing, weighting, and bans attach to.** *Working definition for this section; source is Part 1's lineage treatment.* There is no standing author-member list: lineage is a point-in-time projection computed from the current client leaves and the governance chain, not a maintained registry.

- **hot Group**: the Group whose ratchet tree holds currently-live member clients (defined below). The confidentiality-and-membership boundary for active communication.

- **cold Group**: a linked Group holding member clients migrated out for dormancy. Near-zero steady-state cost; carries dormant personae without inflating the hot tree.

- **live** (of a client), has processed a hot-Group epoch within the liveness window (§7.6). **Liveness is defined by processing epochs, not by authoring messages:** a silent reader who still syncs is live.

- **member** (regime-dependent, reconciled here): in the encrypted tiers (§7.4 through §7.9.2), a member is any client in the MLS tree, which includes silent readers, because reading requires the ability to decrypt and therefore requires a tree leaf. In the public-by-default regime (§7.9.3), reading no longer requires membership (content is public, served through the read view), so a member is narrowed to an **attesting-and-governing participant**: one who authors-as-a-member or acts in governance, holding a write-and-govern credential rather than a read admission. The two senses share the MLS-leaf mechanism but differ in *who needs to hold a leaf*, which is why the tree at 7k+ collapses to the attesting core (§7.9.3). Where this section says "member" without qualification, the encrypted-tier sense is meant; the public-regime narrowing is marked at its point of use.

- **liveness window**: the trailing period after which a client not observed processing epochs is migrated to cold (§7.6). The master scaling knob.

- **governance chain**: the single cryptographically-ordered, deterministically-locally-rebuilt log of typed governance events (bans, standing changes, promotions) spanning both hot and cold Groups. Separate from the epoch chains. *Working definition for this section; source is Part 1's governance treatment.*

- **standing**: a lineage's current governance status, resolved by folding its typed governance events to the current head (§7.8). Determines whether it is a recognized first-class participant.

- **ban ceiling**: a governance-chain event revoking a lineage's standing; a position past which the lineage is not recognized. Bans are lineage-scoped and cover all of a lineage's clients.

- **history DAG**: the content-addressed, hash-linked record of a Group's payloads, queryable by hash range, decoupled from key continuity (§7.7). A dataplane structure. In Part 2's terms this is **forward-only mode** (§7.7): an append-only, hash-linked causal fold, the dataplane application of the same §7.3.1 ordering spine, chosen at Group creation and mutually exclusive with Willow-mutable mode. Forward-only is the mode Part 2 designates for "large, broadcast-like, or loosely-coupled Groups," which is exactly the large-group and public-regime case this section treats, so throughout this section "history DAG" means forward-only mode. ***Verified*** (per Part 2 §7.7).

- **two-part re-entry credential**: the single bound artifact a returning persona presents: a governance attestation (standing half) plus a cryptographic ticket to the current epoch (key half), under one fresh signature (§7.7).

- **resumption secret / resumption PSK**: the MLS per-epoch value that proves prior membership when injected into a later epoch. *Inherited term; MLS-layer object.* ***Verified-RFC*** (RFC 9420, key schedule and PSK sections).

- **external commit**: the MLS operation by which a party outside a group's current epoch adds itself, using a fetched GroupInfo rather than being added by a member. *Inherited term; MLS-layer object.* ***Verified-RFC***.

- **helper**: an infrastructure node (relay, store-and-forward node, overlay node, history-convergence node) that carries, holds, or serves traffic without governing. *Inherited from Part 2 §6.* The invariant that groups them: each is a scope participant offering a capability or resource, holding no standing, and is revocable in use (§7.9).

Non-terms held apart deliberately: **membership** (an MLS-layer fact about which clients can decrypt now) is not **standing** (a governance-layer fact about which lineages are recognized). The whole section rests on keeping these two in separate layers.

---

## 7.2 The scaling claim, and what it is not

**Realizes: P (provenance-over-secrecy at scale), P (usability as a first-class constraint).**

The claim, stated as an outcome for the reader who needs only it (Rule 16, library statement of the pitch's content): **Drystone groups are feasible from a handful of members to roughly ten thousand, because the cost of the cryptographic layer scales on the count of *live* members, not the count on the roster, and because both post-compromise security and delivery latency are gradients the group tunes rather than thresholds it must meet.** A ten-thousand-member group is carried as a much smaller live Group plus a large, cheap dormant archive.

What the claim is **not** (Rule 9, residual owned at the point of the claim): it is not a claim of continuous post-compromise security at all sizes (§7.5 relaxes it above a threshold); not a claim of real-time delivery to all members at all sizes (§7.9 makes delivery a latency curve); and not a claim of confidentiality against an insider-assisted adversary at any size (§7.8 states plainly that insider information flow is outside the achievable set). The property held across all sizes is **validity and legitimacy**, a lineage is recognized as a first-class participant if and only if the governance chain says so, and a banned lineage gains nothing beyond what it already held, with confidentiality and integrity as the gradient *means* to that end, strict where cheap and opportunistic where not.

The single forcing reason, which the rest of the section discharges: at the sizes and usage the platform evidence shows (most members dormant, moderation-relevant member-removals rare, message volume the dominant traffic), the binding constraints are not the cryptographic primitives but *tree bloat from dormant members* and *fan-out to the live set*. Both are addressed by one structural move (the hot/cold split, §7.6) and one delivery discipline (the race, §7.9), and the security posture is scoped (§7.5, §7.8) to match what those sizes can afford without a felt experience cost.

***Synthesis***. The claim assembles the requirement of §7.4, the PCS gradient of §7.5, the structure of §7.6, and the delivery curve of §7.9; none of these is a single-citation fact, and the tier numbers in §7.10 are reasoned shapes, not measurements. The two measurements that would ground the numbers are named in §7.11 and tagged ***Load-bearing, unearned***.

---

## 7.3 Cast

Per Rule 4, a fixed closed cast, players and resources both. A beat **MUST NOT** introduce an actor or resource not named here; adding one is an amendment to this section.

**Personae (the narrative spine).**

- **Ada**: a persona in good standing, one author lineage, two devices (a phone and a laptop, each a node). The everyday active member.

- **Boreas**: a persona in good standing who goes dormant for a long period and later returns. The re-entry protagonist.

- **Cyrus**: a persona whose lineage is banned partway through the scenario. The adversarial-standing protagonist: not an outside attacker but a former member whose recognition is revoked.

**Resources the scenario turns on (closed list).**

- **the hot Group**: Ada's and Boreas's live membership boundary.

- **the cold Group**: where Boreas is carried while dormant, linked to the hot Group.

- **the governance chain**: the single ordered log carrying Cyrus's ban and all standing.

- **the history DAG**: the content-addressed record Boreas backfills from on return.

- **the liveness window**: the policy period after which a dormant client migrates to cold.

**Infrastructure roles (the second closed cast, per Rule 4's infrastructure-cast rule).** Each is a scope participant holding a capability or resource, no standing, revocable in use. Named once here; referenced by name thereafter.

- **relay**: carries sealed bytes at the overlay layer; sees transient endpoint pairs, never plaintext. *(Part 2 §6.)*

- **store-and-forward node**: buffers sealed bytes for a not-currently-reachable recipient; sees ciphertext and coarse delivery metadata, never plaintext or ordering authority.

- **overlay node**: forwards along the epidemic broadcast tree; holds no global membership.

- **history-convergence node**: serves history-DAG ranges by hash on request; serves self-verifying content, holds no authority over it.

The chained journey through this cast is carried in Appendix (§7 appendix, external to this section file) per Rule 4; in-body beats below point to it.

---

## 7.4 Requirement: cost scales on the live set, not the roster

**Realizes: P (usability as a first-class constraint).**

### Requirement

The design requires that the recurring cost a member's device pays, per-message work, per-membership-change work, and resident state, **scale on the count of currently-live members, not on the total roster.** A group may carry a large number of members of whom a small fraction are live at any time; the live fraction is the population whose device costs the architecture must keep bounded, and the dormant remainder must be carriable at near-zero recurring cost.

Non-requirements, where a naive design would over-build (Rule 3 form): the design does **not** require that every member hold or process every other member's state continuously; does **not** require a single Group to hold the entire roster; and does **not** require synchronous global agreement on membership (§7.8 establishes that eventual convergence suffices).

### Current conforming implementation

MLS (RFC 9420) as the group key-agreement substrate, subordinate to Drystone per Part 2 §5. The relevant verified specifics:

- Per-message send and receive cost is independent of group size. ***Verified-RFC*** (RFC 9420; the sender-ratchet application-message construction). Drystone's message layer rides this; the hybrid keying that seeds it is specified in Part 2 §5 and is orthogonal here.

- A key-refreshing commit touches the log of the group size in cryptographic operations along the committer's direct path. ***Verified-RFC*** (RFC 9420, ratchet-tree and UpdatePath sections).

- Resident ratchet-tree state, a Welcome on join, and a full UpdatePath on a healing commit are each linear in the group's member count. ***Verified-RFC*** (RFC 9420; the ratchet tree holds a leaf per member, and the UpdatePath encrypts path secrets to resolutions summing to the member count in the worst case).

The mapping from requirement to realization: the requirement is stated over Drystone's **live member** concept; MLS's linear costs are over the **group**'s leaf count. The architecture makes these coincide by ensuring the hot Group's leaf count *is* the live count (§7.6), so MLS's linear terms are paid on the live set by construction. This is the load-bearing composition of the section: **MLS costs what MLS costs; Drystone controls what population MLS is asked to hold.**

***Synthesis***. That the linear MLS terms, paid on a live-only tree, yield a device cost dominated by message traffic at realistic usage is reasoned from the platform evidence (§7.10) and the per-tier analysis, not measured. The measurement that would ground it is §7.11's first item.

---

## 7.5 Scaled post-compromise security

**Realizes: P (usability as a first-class constraint), P (confidentiality as gradient means, not end).**

### The gradient

Post-compromise security, the property that the group re-keys to fresh entropy an attacker who compromised a member cannot follow, so that after a heal the attacker loses forward access, is achieved in MLS only by commits that inject fresh path entropy (an UpdatePath commit, or a removal). ***Verified-RFC*** (RFC 9420; PCS follows from members refreshing keys, and the RFC recommends periodic Updates on the order of hours to days for exactly this). Forcing that healing on a schedule across a large tree is a continuous cost linear in the tree, paid whether or not any compromise occurred.

The design therefore scales PCS by group size, on a fixed threshold:

- **Below 250 members: strict.** Periodic key-refreshing Updates are enforced, yielding continuous PCS. This is the no-compromise band: a group under 250 runs full forward *and* post-compromise security, and the recurring heal cost is small enough to be unfelt.

- **At and above 250 members: opportunistic.** Scheduled healing is not forced. PCS still occurs, every ban is a healing removal, and organic voluntary Updates heal, but the compromise window lengthens from the strict "hours" to "until the next natural epoch event." Across a few member-removals per day plus organic updates, natural healing remains frequent; what is given up is the *guarantee* of a short bound, not healing itself.

The threshold of 250 is chosen to coincide with the substrate's own large-group inflection: MLS delivery libraries treat a group above roughly 250 as "large" and change member-handling behavior there. ***Verified-RFC*** / **[confirm]** (the "large" threshold appears in MLS library conventions and the disnake/discord API documentation cited in Part 2 §6's references; the specific value is a library convention, not an RFC MUST, and is flagged for confirmation against the current library revision).

### Why the relaxation is correct rather than lax

**Realizes: P (usability as a first-class constraint).** A product that is more secure but has a bad experience is of no use. Above 250, forced periodic healing is a continuous, felt cost (battery and bandwidth, most acutely on mobile) securing a property the user cannot see. Opportunistic PCS trades that invisible property for responsiveness the user does feel. The security actually retained at these sizes, strong forward secrecy on live traffic, group validity via MLS membership, opportunistic post-compromise healing, matches the threat model the design commits to at scale (§7.2, §7.8): validity and legitimacy over adversarial secrecy.

***Design***. The threshold value, the strict/opportunistic split, and the claim that opportunistic PCS is the correct trade above 250 are decisions this section commits to, grounded in the usability principle and the substrate inflection, not in measurement of the felt cost. The measurement that would ground the felt-cost claim is §7.11's first item (per-commit cost at live-N sets how heavy forced healing would be).

### Normative clauses

A hot Group with fewer than 250 live members **MUST** enforce periodic key-refreshing Updates, because below this size continuous PCS is affordable and its absence would forgo post-compromise healing the design can cheaply provide (Realizes: P). ***Design***.

A hot Group at or above 250 live members **MUST NOT** be blocked from operation by absence of a scheduled heal, because forcing scheduled O(live-N) heals at this size imposes a continuous felt cost securing an unobservable property, which is the "secure but unusable" failure the usability principle forbids (Realizes: P). ***Design***.

A removal commit enacting a ban **MUST** carry a full UpdatePath, because a removal that does not rotate path entropy fails to exclude the removed lineage from future group secrets, which is the concrete failure a ban must prevent (Realizes: P, non-domination via enforceable exclusion). ***Verified-RFC*** (removal-with-UpdatePath is the RFC's exclusion mechanism).

---

## 7.6 Hot and cold Groups; liveness-driven migration

**Realizes: P (usability as a first-class constraint), P (helper-not-authority via portable structure).**

### The structure

A large community is carried as two linked Groups: a **hot Group** whose ratchet tree holds only live member clients, and a **cold Group** holding clients migrated out for dormancy. The hot tree is thereby kept to the live set, so §7.4's linear MLS terms are paid on that set. The cold Group, whose members neither send nor process epochs in steady state, costs the community near nothing: no fan-out, no commit participation, no hot-tree churn.

The two Groups are linked by the MLS resumption mechanism (§7.7, §7.8): a client's prior membership in one is provable when re-entering the other. ***Verified-RFC*** (RFC 9420; resumption PSK across linked groups, and its guarantee that entrants agree on a key iff they were members at the referenced epoch).

### Migration rule: liveness, not activity

The migration trigger is **liveness**, whether a client is still processing hot-Group epochs, **not activity**, whether it is authoring messages. A silent reader who still syncs is live and stays hot; only a client not observed processing epochs within the liveness window migrates to cold. This distinction is load-bearing for usability: it means aggressive migration windows catch only the genuinely absent, never the quiet reader, so the hot tree can be kept small without punishing lurkers.

Migration to cold is a removal from the hot Group and is **batched** (Rule 7's shared mechanism, gap-aware convergence, is orthogonal; the batching here is the MLS one): many dormant clients are removed in one commit rather than one commit each. ***Verified-RFC*** (RFC 9420 / OpenMLS; a single commit covers many removal proposals). The community pays a per-batch cost on a cadence, not a per-member cost.

### The liveness window as the master knob

The liveness window is the primary scaling control. Tightening it shrinks the hot tree (cheaper commits, joins, and fan-out) at the cost of sending more personae to cold more often (more returns, more backfill events). Loosening it improves the return experience at the cost of a larger hot tree. The window therefore trades **live-experience against return-experience**, and the correct setting is a function of group size and the actual live fraction, not a global constant.

The recommended schedule, tightening with size (Rule 5, the tradeoff stated):

| Band (total members) | Modest window | Aggressive window |
|---|---|---|
| 250–1k | 90 days | 45 days |
| 1–3k | 60 days | 30 days |
| 3–7k | 45 days | 21 days |
| 7–10k | 30 days | 14 days |

The recommended operating policy is **dynamic**: rather than a static per-band pick, drive the window from the live hot-N against a target ceiling. Set a hot-N comfort ceiling (provisionally ~1500, tolerable ~2500, ***Load-bearing, unearned*** pending §7.11's first measurement); tighten the window when hot-N approaches the ceiling and relax it when there is headroom. This makes the community self-regulate its hot-tree size, aggressive when crowded, modest when not, and adapts to actual liveness rather than nominal size, so an unusually active 10k community and a sleepy one are handled by the same rule at different set-points.

### Non-domination via portability

**Realizes: P (helper-not-authority, non-domination at the infrastructure layer).** Because membership authority lives in the members' own MLS state, standing in the governance chain (§7.8), and history integrity in the content-addressing (§7.7), a community can transition its delivery infrastructure (its store-and-forward nodes, its history-convergence nodes) without losing its membership, standing, or history. The structure holds nothing in the infrastructure that another helper cannot re-serve from data the members collectively hold and can self-verify. This is the operational form of non-domination: exit from any helper is available and costless to the community's existence.

### Normative clauses

Migration to cold **MUST** be triggered by liveness (epoch-processing) and **MUST NOT** be triggered by activity (message-authoring) alone, because migrating a silent-but-syncing reader would evict a fully-functioning member and degrade the reader's experience for no cost saving the live-set requirement needs (Realizes: P). ***Design***.

A helper serving the hot or cold Group **MUST NOT** be granted authority over membership, standing, history integrity, or ordering, because any such grant makes the community non-portable across helpers and reintroduces the domination that the helper-not-authority principle forbids; a helper accelerates, it never adjudicates (Realizes: P). ***Design***.

---

## 7.7 Re-entry: the two-part credential

**Realizes: P (usability as a first-class constraint), P (local authority, the returner bears its own cost).**

`Now suppose Boreas, dormant past the liveness window and migrated to cold, returns to the hot Group. (Beat: see §7 appendix, re-entry arc.)`

### The problem, and why a stored key package does not solve it

A returning persona must re-establish two distinct things, and neither artifact that seems obvious solves both:

- A **pre-published KeyPackage does not enable self-service return.** A KeyPackage is consumed by an *existing member* to add someone via a Welcome, and the Welcome is encrypted under the current epoch's secrets; the returner cannot produce their own Welcome. A stored KeyPackage therefore holds no place across epoch churn and grants the returner no ability to act. ***Verified-RFC*** (RFC 9420; a member uses a new member's KeyPackage to add them and constructs the Welcome).

- The **resumption secret survives epoch rolls and is the durable continuity token.** A resumption PSK from the epoch a member was last live in proves prior membership when injected into a later epoch, irrespective of all intervening key changes, and is a stored value, not an FS-deleted key. ***Verified-RFC*** (RFC 9420; resumption PSK from epoch *n* injected at *n+k* demonstrates membership at *n* regardless of intervening Updates). This lets the cold Group roll epochs freely while a member is gone; no frozen-epoch cold Group is required.

The self-service return path is therefore the **external commit**, not the KeyPackage: the returner fetches a current GroupInfo and constructs its own commit, so the cost falls on the returner rather than on an active member. ***Verified-RFC*** (RFC 9420; external commit lets a party add itself, and its resync form re-establishes a member's own representation).

**The GroupInfo the returner fetches is not trusted as authority; it is a claim corroborated against the governance chain** (Part 2 §7.4.2). MLS's own recovery hazard is that an external joiner cannot fully validate a GroupInfo's freshness, so a stale-but-validly-signed GroupInfo could defeat post-compromise security. Part 2 dissolves this by making the governance chain, not the GroupInfo, authoritative: a rejoining node treats the GroupInfo as a claim and corroborates it against the chain it already holds, with the monotonic fold and out-of-band history convergence closing the stale-GroupInfo and insider-replay hazards. The residual Part 2 carries is narrow and named: a node far enough behind that its own view has not passed the stale GroupInfo's epoch. So the earlier center-free "who vouches for the GroupInfo" worry has a specified answer, the governance chain does, and it is not an open question at this tier. ***Verified-RFC*** (the MLS hazards, per RFC 9750) / ***Design*** (the corroboration resolution, per Part 2 §7.4.2, carrying its own `[confirm]`).

### The credential

Re-entry presents a **single bound two-part credential** in which each half covers the other's blind spot:

- **The governance attestation (standing half).** A fresh signature by the returning lineage over a witnessed governance-chain position, proving lineage continuity and feeding the standing resolution (§7.8). It proves *who* and *whether recognized*; it does not admit the returner to the key layer, because governance does not gate keys.

- **The cryptographic ticket (key half).** The resumption PSK enabling a valid external commit that re-establishes current keys. It proves the returner *can* cryptographically rejoin; it is silent about current standing, because a resumption PSK is a fact about a past epoch and cannot reflect a later ban.

Neither half suffices alone: the ticket admits to keys without proving standing; the attestation proves standing without admitting to keys. The admission gate requires both.

**The two halves MUST be bound under one fresh signature**, not merely co-presented, because independently-presented halves permit pairing a stolen attestation with a different ticket; the fresh signature (over both halves plus a current challenge) proves present control of the lineage and welds the two into one non-separable proof. ***Design***.

**Re-admission is a decision and an enactment, and the "who pays" question is Part 2's enactment dial** (§7.3.6, §7.6.7). Part 2 separates the governance *decision* (which folds cheaply and is concurrency-safe) from the enforcing *commit* that enacts it (which serializes on the epoch chain and splits the audience). Re-admission maps onto this split exactly: validating the two-part credential and resolving standing is the *decision* (governance-layer, cheap, off the epoch chain), and the epoch roll that re-keys the returner into the current epoch is the *enactment* (the serialized, O(N) part). So the "returner does their own work" claim is precisely Part 2's **enactment dial**: who fires the enforcing commit is configurable, and the self-service external-commit path is the setting where the returner fires it themselves rather than an active member paying, with Part 2's idempotent fallback if the designated actor is absent. This grounds the local doc's re-entry cost story in specified machinery rather than a parallel construction: the two-phase decide-then-enact interval (§7.3.6) is also what lets a hold suspend enactment before the audience splits, so a contested re-admission need not roll an epoch while it is still resolving. ***Design*** (per Part 2 §7.3.6, §7.6.7).

### What the returner recovers, and the accepted residual

On admission the returner re-keys to the **current** hot epoch and participates immediately. Message history from the dormancy window is recovered separately, from the **history DAG** (§7.1) by hash-range query against a history-convergence node, decoupled from key continuity: the DAG serves self-verifying content, and the gap is re-keyed to the returner on promotion so archived content is readable. ***Synthesis*** (the decoupling of history-access from key-continuity is Drystone's construction over the content-addressed DAG and the gap re-key; it is not an MLS mechanism).

The accepted residual, owned here (Rule 9): retaining a re-grantable archival key so a returner can read the dormancy gap is a **bounded relaxation of forward secrecy for archived content.** Live message keys remain forward-secure; archived history is retained under a group-archival key returning members can obtain. This is a deliberate trade for return-completeness, correct under the scale threat model (§7.2), and it **MUST** be stated as such wherever the design's forward-secrecy posture is described, because leaving it implicit would let a reader over-trust FS on history the design does not provide (Realizes: honest-residual). ***Design***.

### Progressive return as a usability requirement

**Realizes: P (usability as a first-class constraint).** A returning member **MUST** be able to participate at the current epoch before history backfill completes, with backfill streaming in the background, because a return that blocks on full backfill turns every return into a stall and makes even a modest liveness window feel broken; conversely, seamless progressive return is what makes an aggressive window tolerable. The aggressiveness a community can adopt in §7.6 is bounded not by the protocol but by the quality of this return experience. ***Design***.

---

## 7.8 The governance chain, bans, and standing resolution

**Realizes: P (local authority first), P (provenance and transparency), P (non-domination).**

### One chain, spanning both Groups

Bans and standing are recorded on a **single governance chain** shared by the hot and cold Groups, separate from the epoch chains. This is the layer boundary drawn correctly (Rule 8, thing-vs-layer): the epoch split (§7.6) is a *key-layer* performance partition; standing is a fact about *personae*, who span both Groups, so its natural scope is the whole community and there is exactly one of it.

The alternative, a governance chain per Group, with bans committed to both, is rejected because it re-creates a cross-chain reconciliation problem one layer up: two ban positions that must be kept consistent is the very seam the single chain eliminates. ***Design***. (Stated as the current conclusion per Rule 1; no prior-draft contrast.)

### A ban is a fork with a quorum-stamped ceiling; standing resolves at head

A ban and a member leaving of its own accord are, in Part 2 §7.6.4, **the same underlying primitive seen from two directions**: lineage divergence at a head, two lineages sharing history up to a point and diverging after it, neither owed reconciliation by the other. A ban is the group ceasing to corroborate a member (the quorum stops vouching going forward); a voluntary fork is a member ceasing to require the group's corroboration. So a ban is **not a deletion** and Part 2 forbids implementing it as one: the group's authority reaches only to withdrawing its own corroboration going forward, never into what the member already holds. A banned lineage is **forked off whole**, keeping everything up to the ceiling, losing only forward corroboration. ***Design*** (per Part 2 §7.6.4).

The ceiling itself is the ban's artifact: a **quorum-stamped fact** carrying k-of-n group authority that says "the group, by its rules, stopped corroborating this member as of head H" (Part 2 §7.3.5). This artifact is load-bearing for provenance, it is the only thing that lets a later third party read that *the group* decided a departure rather than *one party* deciding it alone (a voluntary fork deposits no group-side artifact), so it **MUST** be preserved as distinct from a voluntary departure marker. Bans are **lineage-scoped and cover all of a lineage's clients**, so ban enforcement cannot be done by MLS's leaf-level membership logic, MLS sees clients, not lineages, and a banned lineage returning on a fresh client is a leaf MLS has no reason to reject. Ban enforcement therefore lives in an **application-layer admission gate** on external-commit acceptance, resolving the returner's claimed lineage against the ceiling. ***Synthesis*** (MLS sees clients: ***Verified-RFC***; the quorum-stamped ceiling and fork primitive: per Part 2 §7.3.5, §7.6.4; the admission gate is Drystone's construction).

**Standing is resolved over the full chain to head, never over the returner-asserted range.** A returner controls which governance position they attest to, so scoping the ban check to the asserted range would let a lineage banned at a later position re-enter by attesting to an earlier one, the ban lies outside the window the returner chose. The asserted position authorizes *continuity and backfill scope* only; the *ban determination* resolves current standing at head, which the returner cannot choose. ***Design***. (This closes the far-back re-entry hole; stated as the current rule per Rule 1.)

### Insider information flow is outside the achievable set

**Realizes: honest-residual (Rule 9), provenance-over-secrecy.** The ban check is an **integrity-and-standing mechanism, not a confidentiality mechanism against an insider-assisted adversary.** A member in good standing can disclose group contents to anyone, personae own their own history and share it as they choose, so no technical gate prevents a banned person from *learning* contents if a member feeds them. What the ban enforces is that a banned lineage is not a *recognized first-class participant*: it cannot author messages the group attributes to a standing member, cannot count in governance weight, and cannot hold current keys in its own right. This scoping is what makes the check correct rather than impossible: a ban is a statement about present recognition, resolved at head, not about what window of history a returner can prove they witnessed.

### The invariant that makes eventual consistency safe

`Now suppose Cyrus's lineage is banned at a governance position, but a node has not yet synced that far and admits Cyrus's return. (Beat: see §7 appendix, ban arc.)`

Ban propagation is **eventually consistent**, and this is safe rather than a liability, because the invariant is enforced by the key layer independently of governance-propagation timing: **a lineage sees only what it was entitled to during epochs where it held keys, and re-exclusion is eventually guaranteed by post-compromise re-keying.** A node that admits a banned-but-not-yet-known lineage will, on syncing the ban, issue the removal commit; the removal re-keys and the banned lineage is cut from new entropy, having gained only prior entitlement (zero marginal exposure, since a returned lineage briefly re-holds only keys it already had). ***Synthesis***. Consequently the design **does not require consensus on membership**, only eventual convergence, backstopped by re-keying, and dormancy cannot be used to evade a ban, because return re-subjects the lineage to the current epoch's membership and the ban re-fires on next sync.

The residual floor this leaves, owned here (Rule 9): re-exclusion is enacted by an active member committing a removal, so the *latency* of exclusion has a floor set by hot-Group commit liveness. Liberal admission is safe only while enough live members are present to enact removals promptly. The thing to monitor at scale is therefore **commit liveness, not membership consensus.** ***Design***.

The bound on this invariant, stated so it is not overclaimed (Rule 9): the self-healing covers a node that is merely *behind* (it admits a not-yet-known ban, later syncs, re-excludes). It does **not** cover a *genuine membership contradiction*, which Part 2 §7.6 routes to the fork and human escalation rather than to a fold that self-heals. So "eventual consistency is safe, re-exclusion heals it" is precise for the determinate case (a ban propagating late, a concurrent mutual-expulsion resolved by tiebreak) and is **not** a claim that every standing disagreement folds: the ones the fold cannot determinately resolve escalate, per the keystone above. ***Design*** (per Part 2 §7.6).

### The keystone: order-independent standing resolution, constructive not assumed

The safety of eventual consistency and of the head-resolved ban check both rest on one property: **two nodes that have received the same set of governance events resolve the same standing, regardless of the order the events arrived.** Part 2 §7.3.1 makes this order-independence *constructive* rather than an article of faith, by specifying the exact resolution the fold applies, so the local doc adopts that mechanism rather than positing a generic CRDT merge:

- **Authorization is a gate, not a ranking** (Part 2 §7.3.1). A governance change is an assembled k-of-n quorum of concordant facts, each counted only if its author held standing *at that fact's causal position* (the forward pass, Part 2 §7.5.2, which settles authority without consulting the contested outcome). Standing decides *who may act*; it never decides *whose otherwise-valid decision wins*, so no conflict is tipped by an author's authority. ***Design*** (per Part 2).

- **A layered operation-type precedence resolves conflicting kinds** (Part 2 §7.3.1 key 1): threshold changes, then membership removals, then role and capability removals, then grants, then membership additions, each tier resolved against the settled result of the tiers above. Subtractions before additions, biasing every intermediate state toward the restrictive reading, and membership brackets roles so a grant always projects onto already-settled membership. This is the cross-type ordering: it orders *kinds* of operation, privileging no party, which is why it is a safe default. ***Design*** (per Part 2).

- **Causal precedence within a tier, then a content-address tiebreak among genuine concurrents** (Part 2 §7.3.1 keys 2 and 3): the causally-later same-type fact supersedes the earlier for a slot; where two are mutually concurrent, the digest of the canonical fact encoding decides, party-neutral and identical everywhere. Never a wall-clock, which is uncorroborable and a social-engineering vector even with membership fully gated (Part 2 §7.3.1). ***Design*** (per Part 2).

- **Cross-slot effects are projections on the final resolved slots, never mid-fold mutations** (Part 2 §7.3.2): a removal revokes roles at its own causal position and the effective-roles projection makes a removed member hold no effective role, both computed as pure functions of the resolved sets, because a fold that mutated the state it is still reading would become order-dependent. ***Modeled*** (per Part 2).

Together these make the fold order-independent by construction: same event set in, same standing out, at every node. This is what makes the admission gate's head-resolution consistent across nodes rather than fork-inducing.

**What remains unearned is narrower than the whole fold: it is gap-completeness alone.** With the Part 2 §7.3.1 and §7.3.2 resolutions above, the fold's order-independence is constructive rather than asserted, so the single remaining unproven property is **gap-completeness**: whether a node can reliably tell it holds the complete causal set of facts up to a position, since the resolution is only correct over a complete set (an absent predecessor is a detected gap, Part 2 §7.3.2, not a silent miss). If gap-completeness holds, standing resolution is consistent across converged nodes by construction; if it fails, a node folds an incomplete set and can diverge. This is the Appendix B beam (Part 2 §7.9.2), and it is the correctly-located residual, narrower than the "cross-type commutativity" the earlier framing carried, because the cross-type resolution is now specified and constructive, not open. ***Load-bearing, unearned*** (gap-completeness, per Part 2 Appendix B); the resolution order it operates over is ***Design*** and constructive.

**Genuine contradictions escalate rather than folding** (Part 2 §7.3.2 boundary, §7.6). The resolution above handles conflicts where the ordering key yields a determinate, non-arbitrary winner (the concurrent mutual-expulsion case resolves *only because* a content-address tiebreak is non-arbitrary by construction). Where a contradiction is a genuine membership contradiction the fold cannot resolve without manufacturing a utility verdict, the protocol **hard-stops and escalates** to the §7.6 fork rather than folding. So it is not the case that all standing conflicts fold: determinate concurrents fold by tiebreak, genuine contradictions route to human escalation and the fork. ***Design*** (per Part 2).

### Positioning artifacts against the single chain

For the ceiling comparison to be a clean single-chain check, every artifact that a standing decision consults must be **positionable against the governance chain.** Each dataplane payload carries a unique governance-chain pointer (minted-at position), and the re-entry credential's attestation carries the governance position it was minted at, so "is this below the ban ceiling" is a comparison within one totally-ordered structure, needing no cross-layer epoch-to-position mapping. The credential's cryptographic ticket references an MLS epoch; the gate **MUST** confirm the ticket's epoch and the attestation's position resolve to a consistent era of the lineage's participation, because a recent ticket paired with an ancient attestation (or the reverse) would otherwise game the ceiling check. ***Design***.

### Cross-chain ordering: eliminated, not bridged

Within a single Group's epoch chain, ordering is total and cryptographic: every commit binds to its predecessor via the confirmed transcript, so no two commits can be misordered and the first-to-reach-wins delivery discipline (§7.9, Part 2 §6) is fork-proof intra-chain. ***Verified-RFC***.

Across the hot/cold boundary, ordering is **not** cryptographically total, a resumption PSK proves membership-at-an-epoch but does not order cold-chain events against hot-chain events. The design does not bridge this seam cryptographically; it **eliminates** it by placing all cross-chain ordering facts (a ban preceding a promotion) on the single governance chain, whose determinism (above) supplies the ordering the cryptography structurally cannot. The residual is that a promotion commit's *admission* is a governance-gated, eventually-consistent decision rather than a cryptographically-total one, covered by the re-exclusion invariant above. ***Synthesis***.

---

## 7.9 Delivery under scale: the race, and graceful degradation

**Realizes: P (usability as a first-class constraint), P (non-domination via graceful degradation).**

### Fan-out is a curve, not a ceiling

Delivering a message to the live set is the dominant recurring cost at scale (bans and adds are a trickle at the fixed policy; message traffic dominates, §7.10). The design does **not** treat fan-out as a hard ceiling. Delivery runs the **store-and-forward node and the overlay (swarm) as a race, first-to-reach wins**, giving delivery latency as the minimum of two mechanisms with anti-correlated failure modes: swarm is fast when peers are reachable and degrades when the live set is sparse or partitioned; store-and-forward is slower but robust to exactly those conditions. ***Design*** (the race is Drystone's delivery discipline; its components are Part 2 §6).

Consequently a large group traverses a **latency-versus-liveness curve at its own discretion**: hitting the fan-out limit means delivery leans harder on store-and-forward and latency rises gracefully, not that the group breaks. Immediacy costs liveness; a group that accepts higher latency simply operates further toward the store-and-forward end. This is the graceful-degradation form of non-domination: no single delivery mechanism is a point of failure or control.

### The race is safe for control-plane traffic because ordering is cryptographic

The race delivers messages in any order and with duplication; safety follows from ordering being carried **in the messages cryptographically**, not inferred from arrival. The delivery layer is therefore **pure transport**: it has one job, move bytes, and zero responsibility for sequencing. A recipient reconstructs the true sequence from the cryptographic linkage and dedupes the slower copy. For commits, which are order-required, it naturally holds an unapplicable later commit until its predecessor arrives, which is ordinary chain-following, not special race machinery. ***Verified-RFC*** (intra-chain commit ordering is total via the confirmed transcript) / ***Design*** (the race and first-to-reach-wins are Drystone's).

The distinction between planes is a property of *how the recipient consumes*, not of *how the delivery layer handles*: application and history payloads are order-tolerant (the history DAG reconciles by hash), commits are order-required, but in both cases the order is intrinsic to the data, so the delivery layer treats them identically. The DS role reduces to buffering; it is **not** an ordering authority. A malicious or buggy buffer can delay, duplicate, or reorder, and the worst it achieves is latency, because inducing a fork would require a recipient to accept a commit against the wrong prior state, which the linkage refuses. ***Design***.

### Owned residual: the store-and-forward node is a metadata observation point

**Realizes: honest-residual (Rule 9).** The store-and-forward end of the race parks sealed bytes for offline-but-live recipients, so it is a Carriage-plane entity that observes ciphertext plus coarse metadata (who has mail waiting, roughly how much, from whom if unpadded). At the scale threat model (validity and PFS over adversarial metadata privacy, §7.2) this is an accepted, deliberate Carriage-plane choice, not a defect. It **MUST** be stated as such wherever delivery robustness is described, because the race provides delivery robustness, not delivery privacy, and a reader must not mistake one for the other. Stronger metadata privacy at large size is a non-goal of this section, addressed if at all by lower-layer countermeasures Part 2 §6 notes rather than provides. ***Design***.

### Normative clauses

The delivery layer **MUST NOT** be relied upon to preserve message order, because doing so would make the store-and-forward/swarm race unsafe; order is carried cryptographically in the messages and reconstructed by the recipient, so a delivery layer that claimed ordering authority would be both unnecessary and a fork risk if buggy (Realizes: P, non-domination via authority-free infrastructure). ***Design***.

A commit received out of sequence **MUST** be held unapplied until its cryptographic predecessor is present, because applying a commit against the wrong prior state forks the group; the confirmed-transcript linkage is what makes the out-of-sequence condition detectable rather than silently misapplied (Realizes: group validity). ***Verified-RFC***.

### 7.9.1 Encryption posture: the two forces, and why Drystone can be large and encrypted

**Realizes: P (confidentiality as gradient means), P (non-domination), P (local authority first).**

`Purpose: state, as the design's explicit encryption posture, why a large end-to-end-encrypted group is achievable here when consumer platforms give it up at roughly 1,000 members. The empirical landscape is stated in §7.13; this subsection states what Drystone concludes from it.`

Across consumer platforms, the ability to end-to-end encrypt a group's text and the ability to make that group large trade off against each other, and every platform resolves the tension by surrendering one. The two E2EE-group-text platforms cap rosters near 1,000; the large-or-uncapped platforms do not E2EE group text at all. ***Established*** / **[confirm]** (the cross-platform pattern and its per-platform detail are in the §7.13 evidence and its references; specific caps are flagged for re-verification). The tension is driven by two distinct forces, and a design that wants both large and encrypted must defeat both.

**Force 1: the key-agreement cost curve.** Traditional group E2EE with per-member setup cost that grows with membership (pairwise channels, or sender-key *distribution* over pairwise channels) caps out where that cost becomes prohibitive, which on the E2EE-first platforms is near 1,000. ***Established*** (the pairwise/sender-key cost growth is the stated reason those platforms cap where they do; §7.13 references). Drystone defeats Force 1 with the substrate it already uses: MLS provides tree-based group key agreement at logarithmic rather than linear cost (§7.4), and the message layer is a per-sender symmetric hash ratchet *seeded from the MLS exporter* rather than distributed pairwise (Part 2 §5). This is the load-bearing point: **sender keys are the cheapest viable group message encryption (O(1) per message, no per-recipient work), and their one fatal weakness at scale, O(N-squared) pairwise key distribution, is exactly what seeding them from MLS removes.** There is no cheaper cryptographic message form to reach for; the design is already at the floor, and "viable above 1,500" is a question of minimizing how often the expensive MLS operations are paid (the liveness-window and opportunistic-PCS tuning of §7.5 and §7.6), not of finding a cheaper cipher.

**Force 2: server-mediated core function.** Independent of per-member cost, the more a platform's core function requires the server to read plaintext (ranking, full-text search, discovery, sitewide moderation, ad targeting, one-to-many broadcast), the less E2EE is even possible, because E2EE denies the server the plaintext those functions consume. ***Established*** (§7.13; this is why a public ranked/searchable forum is non-E2EE by construction). Drystone's ethos already answers Force 2 by construction: it runs no server that ranks, searches, or adjudicates content, so there is no server-readable-plaintext feature forcing the surrender. **This is the trade that buys large-and-encrypted, and it is owned as a deliberate price, not a gap:** giving up server-side ranking, search, and discovery is the cost of E2EE at scale, and it is a product decision, not a cryptographic one. Moderation specifically moves off content-inspection (which would require server-readable plaintext, the reason the large platforms keep text readable) and onto the governance-and-lineage layer (§7.8): moderation-by-standing rather than moderation-by-reading. That relocation is what lets the content stay encrypted while the community stays governable.

**The second warrant for the hot/cold split.** The design built the hot/cold split (§7.6) on *cost* grounds (the tree pays per member, activity concentrates in a small core). The encryption landscape supplies an independent *cryptographic* warrant for the same structure: large-group E2EE works best when a minority actively transmits and the majority passively receives, because sender-oriented schemes make a pure receiver nearly free while a sender pays the setup. ***Established*** (§7.13; the survey observation that sender-oriented E2EE favors a minority-transmit majority-receive shape). So the activity skew that motivated hot/cold on cost grounds *also* makes the sender-key message layer tractable: few active senders, many passive receivers, is the ideal shape for this encryption, not a compromise. Two independent reasons, tree cost and sender-key economics, point at the same structure, which is a strengthening of the design rather than a coincidence.

**The residual this posture inherits (Rule 9).** The property the incumbents' pre-MLS designs were found to lack, payload encryption without cryptographically-authenticated membership (a server able to inject a member), is the exact property MLS's authenticated membership is meant to close, and it is what Drystone's membership authority (§7.4, Part 2 §5) rests on. ***Established*** for the finding (the WhatsApp sender-key analyses in §7.13 references); ***Load-bearing, unearned*** for Drystone's closure of it, since the two-part credential and lineage-ban interlock (§7.7, §7.8) are the sound-but-unvetted composition of §7.11, not a proven one. The encryption posture validates the *direction*; it does not discharge the open items. And "cheapest viable form" is cheapest *at the scoped security level*: sender keys give forward secrecy cheaply and post-compromise security only on rekey, which is precisely why the strict-below-250 / opportunistic-above gradient (§7.5) exists. The cheapness and the threat-model scope are the same decision.

### 7.9.2 The public-projection tier: an optional AppView-shaped read cache

**Realizes: P (helper-not-authority), P (non-domination), P (confidentiality as gradient means).**

`Purpose: specify an optional, cheap, read-only tier for a read-mostly public audience, built on the public-substrate (AppView) pattern, and state the single property that keeps it inside the helper-not-authority ethos rather than reintroducing a trusted tier.`

The three-way population split the encryption posture implies (active members in the hot Group, dormant members in the cold Group, and a read-mostly audience that need not be in the encrypted group at all) admits a third, cheaper tier for the third population: an **optional public-projection cache**, indexed and served in the shape of an AT-Protocol-style AppView, giving read-mostly consumers fast indexable reads without paying MLS membership cost.

**The single property that makes this coherent.** The cache serves **a projection the group deliberately publishes as non-confidential, never a decryption of the encrypted interior.** This is the fork on which the whole tier turns: an AppView that served *decrypted* group content would hold the plaintext, could read everything, and would make the group's confidentiality depend on the cache being honest and available, which is a trusted tier and the precise centralization the ethos refuses (it is the public-substrate cautionary case, §7.13's Force-2 endpoint, reintroduced as a dependency). An AppView cannot provide cheap reads of *E2EE* content, because that is a contradiction: the cache would need the plaintext, defeating the encryption. What the design runs instead is **two confidentiality domains**: an E2EE MLS interior for members, and a deliberately-public exterior projection for read-mostly consumers, with the group authoring into whichever surface matches the intended audience. The boundary between them is a governance decision about *what to publish*, not a key the cache holds. The public projection never claimed confidentiality, so serving it in the clear surrenders nothing that was private.

**Why it stays a helper, not an authority.** The tier is **helper-optional**: the authoritative group functions fully without it, and it serves only a derived public projection, so if no one runs the cache the projection is simply not served and the encrypted group is unaffected. That satisfies the §7.6 helper invariant, a pure capability, revocable and swappable, holding no authority. The AppView pattern is borrowed for its *cheapness* (a derived index absorbing read fan-out) without its *authority* (the public-substrate systems make the AppView load-bearing for the whole system; here it is optional and scoped to a public projection). This is the exact move the helper-not-authority principle exists to permit: accelerate reads for the cheapest-to-serve population, adjudicate nothing.

**The residuals this tier introduces (Rule 9), both owned.** First, the cache is a **content-and-metadata observation point for everything published to it**, by construction: the public projection is readable by the cache operator and anyone they serve. This is acceptable if and only if the group correctly understands that publishing to the projection surface is publishing in the clear. The failure mode is a member believing the projection is as private as the MLS interior, a UX-and-governance problem rather than a cryptographic one, but a real one: the two domains **MUST** be visibly distinct to users, or content is published to the wrong surface, because a member who mistakes the public surface for the private one leaks what they meant to keep in the interior (Realizes: honest-residual). ***Design***. Second, the projection is a *derived* view, raising a freshness/authenticity question, but an *integrity* one rather than a confidentiality one (the content is non-confidential). It is answered as the history DAG answers its integrity (§7.7): published items are content-addressed and anchored to a governance-chain position (§7.8), so the cache **cannot forge or alter the projection, only withhold or delay it**, which is the acceptable helper failure mode; a cache serving free-form re-servable text rather than content-addressed governance-positioned items would fall outside the "helper cannot lie, only stall" box and **MUST NOT** be built that way (Realizes: P, non-domination). ***Design***.

### 7.9.3 The public-by-default regime above ~7k: MLS as attestation, not confidentiality

**Realizes: P (transparent tradeoffs and honest handling), P (local authority first), P (non-domination), P (confidentiality as gradient means).**

`Status: Design, experimental. This regime is a candidate direction the design finds promising, not a mechanism it commits to. It is more speculative than the tiers below it: those rest on confirmed MLS primitives and are specified-but-unproven, whereas this regime rests on a chain of plausible-but-unvalidated propositions (that a group accepts public-by-default; that a read-population-outside-the-tree model composes cleanly with attestation; and, least proven of all, that MLS content can flow to an AppView through a bridge that holds no authority, §7.9.3.1). Treat it as an exploratory regime to prototype and stress, not a settled part of the specification.`

`Purpose: sketch the fourth regime, above roughly 7,000 members, where the design deliberately inverts the confidentiality model rather than tuning it. This is a regime change made openly, not a degradation, and it carries a heavier honest-residual burden than the tiers below it because the confidentiality loss becomes the default rather than an opt-in surface.`

Below roughly 7k, the model is confidentiality-by-default: MLS encrypts the interior, and the public-projection cache (§7.9.2) is an *additional* surface a group opts into publishing. Above roughly 7k, the design permits inverting this: **the group's messages are public by default (clear text, served through the AppView-shaped read view as the primary surface), and MLS is retained not to encrypt messages but for group attestation and member management.** The read-only nonmember view stops being an optional cache of a published subset and becomes the primary content surface, because the primary surface is itself public. This is a deliberate regime change at scale, permitted because Drystone refuses nothing on confidentiality grounds but requires the tradeoff to be transparent and honestly handled (Realizes: P).

**What is conceded, what is retained, in the Force framing (§7.9.1).** The regime concedes Force 2 on purpose: the messages are public, so any server or AppView can read them, and that public-readability is exactly what makes the nonmember view cheap and universal. It continues to defeat Force 1 for the part that still pays: MLS's log-cost tree remains the authoritative answer to who is a legitimate member and what their lineage standing is (§7.4, §7.8). So the regime is **concede content confidentiality, retain cryptographic membership**, which is the honest inversion of the incumbent large-platform posture. The empirical landscape (§7.13) establishes that every platform which went large gave up group-text E2EE; this regime does openly, with attestation intact, what those platforms do silently, and keeps the property they discard entirely.

**Why this is not the public-substrate trap.** The §7.9.2 danger was an AppView serving *decrypted* content, holding the plaintext of something meant to be private. Here there is nothing encrypted to decrypt: the messages were authored public, so the AppView breaches no confidentiality boundary because at this tier there is, by explicit group choice, no confidentiality boundary on message content. The helper-not-authority invariant (§7.6) holds trivially: the read view serves public content, and MLS, held by the members, remains the sole authority on membership. The regime is clean for the same reason §7.9.2 is, and more so, because the content is not even a projection of a private interior but the natural public form of content that was never private.

**Identity preservation is the load-bearing retained property.** Giving up message confidentiality is what would otherwise make a large group indistinguishable from a plain public forum. What keeps it a Drystone group is that membership and standing remain cryptographically real: a nonmember reading the public view sees everything but **cannot author as a member, cannot be counted in governance, and cannot forge standing**, because MLS attestation and the governance/lineage chain still gate first-class participation (§7.7, §7.8). The regime is therefore **public content, cryptographic membership**, the inverse of the incumbent **server-readable-ish content, server-trusted membership** (the §7.13 WhatsApp finding: payloads encrypted, yet the server can inject a member). This regime keeps the exact property the incumbents lose and concedes the exact property they only appear to keep at scale.

**Where the security properties relocate (Rule 9).** Forward secrecy and post-compromise security on the message layer become close to moot at this tier, because there is no message secret to protect, and this **MUST** be stated rather than left implicit, since a reader carrying the strict/opportunistic PCS gradient (§7.5) down from the lower tiers would otherwise assume a content protection that no longer applies. The properties do not vanish; they relocate entirely to the attestation layer: FS and PCS now guard *who can attest as a member*, not *who can read messages*, because a compromised member key forging membership is still a live threat even when the content is public. The honest statement of security in this regime is "the MLS layer protects membership and standing; message content is public by design."

**The heavier consent-and-clarity burden (Rule 9).** Because the regime flips the default, the confidentiality loss is the resting state, not an edge case, so the honest-handling requirement is stronger than §7.9.2's. A member **MUST** be able to tell, unmistakably, that this is a public space, the way a user understands a public forum is public, because the failure mode (believing one is private while public) is now the default condition rather than a mistake at a boundary. The design owes the user that clarity as a hard requirement, because the entire regime rests on informed publication: everything a member writes is public unless the design provides an explicit, visibly-distinct private sub-surface (Realizes: honest-residual). ***Design***.

**The governance-visibility consequence, stated not backed into (Rule 9).** The identity/governance layer is more exposed at this tier, because it is the only protected thing and it operates against a fully public content backdrop. A public-by-default 7k+ group most plausibly has public governance as well: who is a member, who was banned, and what standing changed are visible, which is consistent with the transparency-and-provenance ethos but is a consequence to state, not a default to assume. Whether governance events remain in the MLS-attested layer or are themselves public is a design decision at this tier, and it interacts with the metadata-privacy non-goal (§7.9.2, §7.9.1 residual): a fully public group probably accepts public governance, and that acceptance **MUST** be explicit. ***Design***.

**Performance: where the scaling win is real, and where it is not (Rule 5).** The regime scales better, but the win must be carved precisely, because it is large on the axis that was the actual ceiling and absent on the axis a reader might assume it helps. MLS in the encrypted tiers does two jobs: agree the group key that seeds message encryption, and maintain authenticated membership. This regime drops the first and keeps the second, so the honest question is which of MLS's costs were attached to which job.

- *Per-message crypto goes to zero, but that was never the scaling problem.* Public messages carry no sender-key derivation, no exporter, no AEAD. But the per-message crypto was already O(1) in group size (the sender-key hybrid, §7.9.1), so dropping it saves a constant per message, not a scaling term. Minor.

- *The expensive MLS operations are unchanged per operation, because they are the membership job that is kept.* What made MLS costly at scale was never message encryption; it was the O(N) Welcome on join, the O(N) UpdatePath on a healing commit, and the O(N) resident tree state (§7.4). Every one of those is membership, which this regime retains. So MLS's *per-operation* cost does not scale better here. A spec sentence claiming "MLS scales better in the public regime" would be wrong; the correct statement is that the design pays MLS's costs far less often and offloads the fan-out MLS's delivery never solved.

- *The rate of expensive operations drops substantially, because PCS churn disappears.* In the encrypted tiers the O(N) UpdatePath commits happen for membership changes *and* for post-compromise healing, the latter paid on a schedule even when nothing happens, to keep message keys fresh (§7.5, §7.8). With no message secret to protect, the entire PCS-driven rekey cadence is set to zero: an O(N) operation is paid only on a genuine membership event, not on a healing timer. At the fixed policy (a few bans a day, hourly adds) that is a trickle. The win is on rate, not on per-operation cost. Moderate-to-large.

- *Fan-out, the actual ceiling, largely dissolves for the read population.* The binding constraint at scale was message fan-out to the live set (§7.9, §7.10), because only members could decrypt, so every message went to every live member. Here the messages are public and served through the AppView-shaped read view, so the read population is served by a derived index absorbing the fan-out rather than by peer delivery to each member, exactly the indexed-read economics that let public platforms serve millions. This converts an O(live-N) peer-fan-out problem into an indexed-read problem. This is where the vast win is real. Large, and it is the one that was actually the ceiling.

**The tree shrinks to the attesting core, because reading no longer requires membership.** The deepest effect: in the encrypted tiers a lurker still had to be an MLS leaf to decrypt, which is why hot/cold (§7.6) existed to move dormant lurkers out of the tree, yet the hot set still had to include silent-but-live *readers*. In this regime a pure reader is served by the read view without being in the MLS tree at all, because there is nothing to decrypt. So the tree collapses to *only the attesting members*, those who author-as-members or act in governance, which is smaller than the live-reader set of any encrypted tier. The O(N) operations that remain are O(N) over a smaller N, not because MLS got cheaper but because far fewer people need to be in the tree. This narrows what "member" means at this tier, reconciled in §7.1: a member here is an attesting-and-governing participant, not everyone with read access.

**Aggressive pruning becomes the default, because pruning no longer costs the pruned member read access.** This closes the pruning tension of §7.6 and §7.7 at its root. The aggressiveness of the liveness window was bounded by return-experience: pruning hurt because a pruned member lost read access until re-entry, so progressive return and gap backfill (§7.7) existed to make that return cheap. In the public regime, being pruned from the tree costs the pruned member *nothing on the read path*, because read runs through the public view, membership or not. Pruning removes only an attestation leaf a passive reader was not using. So the tree can be driven down to the currently-attesting core aggressively, because everyone pruned remains a first-class *reader*; re-admission happens only when someone wants to author-as-member or govern, which is rare and self-initiated (the resumption-PSK external-commit path of §7.7), and it carries no reading gap to backfill because the content was never gated from them. Membership at this tier is a **write-and-govern credential, not a read admission**, so a lapsed membership is like a lapsed posting permission, not removal from the room, which is why aggressive pruning is not merely tolerable but the obvious default. ***Design***. The magnitude of every win above depends on the same unmeasured constants as §7.11 (per-commit cost, fan-out cost at the attesting-N), so "vastly better" is a well-founded expectation of the design's shape, not a measured factor.

**Relationship to the tiers.** This regime is why the 7–10k tier (§7.10) is the one where usability is an engineering achievement rather than a default: at that scale a community may choose either to stay encrypted (paying the hot/cold and delivery costs of §7.6 and §7.9 on a large live set) or to enter this public-by-default regime (conceding content confidentiality to shed those costs while retaining attestation). The choice is the community's, made transparently, and neither is refused. The dynamic knob of §7.6 (tighten the liveness window, or shard) and this regime change are the two responses available when a hot tree grows past the comfortable band; this one trades confidentiality for scale, where the knob trades return-experience for a smaller tree.

#### 7.9.3.1 The bridge: an MLS-aware relay helper feeding a privately-hosted AppView

`Status: Design, experimental, and the least-proven piece of an already-experimental regime. The bridge's mechanical core (making an MLS membership attestation verifiable by a non-member from a forwarded artifact) is a real cryptographic design problem, sketched here, not solved. Prototype before relying.`

The flow from MLS group to read view runs through a **local MLS-aware relay helper** (the bridge) that forwards the group's public message stream into a **privately-hosted AppView engine** (the read-serving index). The bridge is only permissible in this regime, and the reason is exact: **the content is public by authorship, so the bridge breaches no confidentiality boundary.** A bridge in an encrypted tier would be the public-substrate trap (§7.9.2), holding decryption authority over private content; here there is no secret to decrypt, which is the precondition that makes any bridge safe. State that precondition wherever the bridge is described, because it is the whole difference between this and the trap.

**What the bridge is, and the qualifier that carries the weight.** The bridge sees the group's public message flow and forwards it into the AppView's ingestion. "MLS-aware" is load-bearing: it is not a dumb pipe, because it must understand MLS framing enough to extract the authored content *together with its attestation*, who authored this, under what membership, at what governance position (§7.8). That attestation is what makes the AppView view meaningfully more than a plain public feed: it carries "provably authored by an attested member at this standing," preserving the identity property (§7.9.3) that distinguishes a Drystone public group from a bare forum.

**The single property that keeps it a helper, not an authority.** The bridge **MUST** forward content-addressed, attestation-carrying, governance-positioned items (the §7.9.2 discipline), so that neither the bridge nor the AppView can forge or alter what the group authored, only fail to forward it. A bridge that transcribed free-form text would be an authority: it could lie about what was said or who said it. A bridge that forwards the *signed, content-addressed* items with their MLS attestation intact can only withhold or delay, the acceptable helper failure mode. The bridge is therefore a **transport of self-verifying attested items, not a transcriber of content** (Realizes: P, non-domination; helper accelerates, never adjudicates). ***Design, experimental***.

**Local and multiple, not a global service.** The bridge **MUST** be runnable locally to the group (by a member or the community's own infrastructure) and **MUST NOT** be architecturally required to be a single global service, because a single bridge everyone depends on is a chokepoint and a de facto authority even holding no keys. Local and multiple keeps it a helper in the strong sense: no single bridge is load-bearing, multiple bridges can run in parallel with the same first-to-reach-wins robustness as the delivery race (§7.9), and the community can always run its own (Realizes: P, non-domination). ***Design, experimental***.

**End-to-end, the read path stays in the "helper cannot lie, only stall" box.** Because the items are content-addressed and attestation-carrying, a reader consuming the AppView can verify the attestation itself: the AppView is not asking to be trusted about what the group said, it serves verifiable items, so even a compromised or dishonest AppView can only withhold, not fabricate. Bridge cannot fabricate (forwards signed items), AppView cannot fabricate (serves signed items), reader verifies. The privately-hosted AppView is the community's index, rehostable and replaceable, not a mandatory public utility.

**The residuals (Rule 9), all owned, one of them load-bearing-unearned.**

- *The bridge is a concentrated metadata observation point.* It sees the whole public stream and the attestation structure in one place, so it observes who authors what, when, at what standing, across the whole group, more than any individual reader sees. In this regime content and likely governance are already public (§7.9.3), but the *concentration* is new. The bridge, like the store-and-forward node (§7.9), is a deliberate metadata-visible point, and running it locally and multiply limits how much any one operator concentrates. ***Design***.

- *The attestation-extraction is the unproven mechanical core.* "The bridge extracts the attestation and forwards it verifiably" is easy to state and is **not** established as clean. The precise open problem: what does the bridge extract from MLS framing that lets a downstream *non-member* reader verify "attested member at standing X authored this," without the bridge being trusted and without the reader needing to be an MLS member (which would defeat the read-outside-the-tree model)? This is essentially *making an MLS membership attestation verifiable by a non-member from a forwarded artifact*. It may reduce to signing authored items with a credential chain a non-member can verify against the group's published membership and governance state, but that is a design sketch, not a proven mechanism, and it composes with the lineage-attestation questions already open in §7.11. ***Load-bearing, unearned***.

- *Freshness and selective omission return at the bridge.* The AppView serves a derived view and the bridge is where derivation happens, so whether the view is a faithful, current reflection of the group is determined by the bridge's behavior. Content-addressing means it cannot falsify, but it can lag or selectively omit, and a reader cannot tell from a single item whether they are seeing everything. This is the §7.9.2 integrity-not-confidentiality residual, now located at the bridge, and bounded the same way: governance-positioned items let a reader detect gaps in the position sequence, so omission is detectable even though latency is not. ***Design***.

#### 7.9.3.2 Visibility and authority as orthogonal dials

`Status: Design, experimental, an extension of the §7.9.3 regime. This is the local-authority principle at the public tier: because confidentiality is off the table, tightness stops being about who can see and becomes purely about who can act.`

In the encrypted tiers, visibility and authority are coupled: being a member means being able to read, so tightening membership tightens readership, and a community cannot be open to watchers while selective about participants. The public regime **decouples them**, and the decoupling is the deepest expression of local authority at this tier. Confidentiality is conceded (§7.9.3), so the question "who may see this" is answered *everyone* by construction, which frees "who may act as a member" to be set entirely independently. The community holds two separate dials:

- *The read dial* can be opened as wide as inclusion desires: content is public, served through the AppView, so the whole world may consume it with no relationship to membership at all.

- *The member dial* can be set as tight as the local authority desires: who may author-as-a-member, act in governance, and hold a return entitlement is whatever the community decides, and it can be strict even while reads are wide open.

A community can therefore be radically open in who it lets watch and radically selective in who it lets speak-as-a-member, and these are orthogonal settings rather than one coupled control. This is "as tight as the local authority desires" (§7.9.3) made precise: at this tier tightness is about authority, not visibility (Realizes: P, local authority first).

**Two reader populations, indistinguishable on the read path, different on the membership path.** The decoupling surfaces two kinds of reader the design can treat differently:

- *The public-relay-oriented reader*, an external account (an atproto or Bluesky-style follower) consuming the public surface the way anyone reads any public feed, with no relationship to the group's membership. They are the public; content reaches them because it is public.

- *The cold-but-entitled former member*, who was in the tree, holds a return credential, reads through the AppView like everyone else, and has a latent right to re-attest and do member operations.

From the read path the two are indistinguishable (both read public content). From the membership path they are entirely different (one holds an entitlement, the other does not). Which population a given reader falls into is itself the membership filter, set by the local authority: a community leaning far out for inclusion serves a large public-relay readership while keeping the entitled-member set small.

**Cold is an entitlement the member holds, not a population the group maintains.** This is why aggressive pruning (§7.9.3 performance) is nearly unconstrained here. A pruned-to-cold member costs the group nothing ongoing: not a leaf in the hot tree, not in any fan-out or commit, and their read experience is preserved through the AppView so they do not notice the demotion. The only thing carried is what *they* hold locally, their own return assertion and re-entry key, redeemed only if they choose a member operation. So the group does not store cold members; **the former members store their own right to return**, and the group carries a liberal *rule* ("once a member, entitled to return under these terms") rather than a maintained roster. Liberal return and aggressive pruning are the same move from two sides: liberal, always-available, loses-nothing-in-the-meantime return is exactly what makes aggressive pruning costless rather than hostile, so demotion to cold becomes a hair-trigger default and the hot tree collapses to only those mid-operation or recently active in governance. ***Design, experimental***.

**The one caveat the liberality must keep (Rule 9).** "Once a member, entitled to return" rests on the former member's kept credential, which is the offline-material-compromise risk of §7.11: a compromised return credential lets an attacker re-attest as that lineage, and liberal rules widen the window in which a stale credential is honored. The bound is the standing invariant (§7.8): re-attestation **MUST** be checked against the ban ceiling at return time, so "entitled to return" is always resolved against "not since revoked." Liberal on entitlement, strict on the revocation check at return, which keeps the liberality from becoming a laundering path for a banned lineage, the one way it could go wrong (Realizes: honest-residual). ***Design***.

#### 7.9.3.3 The attesting-core ceiling, and scaling it past one tree

`Status: Design, experimental, an extension of the §7.9.3 regime. The single-tree size envelope here is grounded in RFC 9750 and one benchmark study; the multi-tree scaling is grounded in RFC 9750's Parallel Groups clause but its win is conditional, matching the uploaded spec's §7.9.3 topology posture. Prototype before relying.`

The performance win of §7.9.3 relocates the ceiling from the *community* to the *attesting core*: since readers are off-tree, the MLS tree holds only personae who need key-gated membership state, and the question becomes how large that acting-and-governing set can grow. This subsection states the single-tree envelope, then the standards-grounded way to scale past it.

**Requirement.** The design requires that the attesting core be carriable within one MLS group up to the size the substrate supports without strain, and that beyond that size the core be splittable across multiple groups without loss of membership authority or standing, at a cost that is named rather than assumed away.

**Current conforming realization, the single-tree envelope.** MLS's own architecture states the size range a single group targets. RFC 9750 §2.1 says a group "may be as small as two clients (e.g., for simple person-to-person messaging) or as large as hundreds of thousands," and states the working target as systems that "aim to scale to groups with tens of thousands of members, typically including many users using multiple devices." ***Verified-RFC***. Against that design intent sits one measured data point: the inconsistency-window study ran OpenMLS on groups up to about 5,000 members (§7.9, Soler et al. 2025), the largest size for which this section has a measurement rather than a design aim. So the honest envelope has two markers to keep distinct: **tens of thousands is the substrate's stated design target for one tree** (***Verified-RFC***), and **about 5,000 is the largest size at which this section has an actual benchmark** (***Measured***, one study, not a protocol constant). The attesting-core ceiling for a single tree sits somewhere in that band, closer to the measured end until a benchmark at the design-target size exists, which is the §7.11 measurement gap applied here.

**What the tree holds is the minimum possible population, which is what makes the ceiling favorable.** The uploaded spec establishes that content is off the epoch chain and authority-only governance is off it, so only genuine key changes touch MLS. In the public regime a reader needs no leaf. So the tree holds only the acting-and-governing core, and the one linear-in-membership cost the spec refuses to claim away, the ratchet tree carried in Welcome and GroupInfo, is paid over the smallest population the design can arrange. The ceiling is set purely by irreducible MLS tree-linearity over that core, not by community size (Realizes: P, usability as a first-class constraint). ***Synthesis*** (the reduction is the uploaded spec's §7.9; the public-regime off-tree reader is §7.9.3 here).

**Scaling past one tree: parallel groups, standards-grounded and cost-named.** When the attesting core exceeds the single-tree envelope, the split is into multiple MLS groups, and MLS's architecture addresses this case directly rather than leaving it to invention. RFC 9750 §6.2 (Parallel Groups) states that "any user or client may have membership in several groups simultaneously" and that "MLS guarantees that the FS and PCS goals within a given group are maintained and not weakened by user membership in multiple groups." ***Verified-RFC***. So splitting the core across trees is a supported configuration that does not weaken per-group security. The cost is equally explicit and is the one the uploaded spec named: the RFC continues that "actions in other groups likewise do not strengthen the FS and PCS guarantees within a given group, e.g., key updates within a given group following a device compromise do not provide PCS healing in other groups; each group must be updated separately to achieve these security objectives." ***Verified-RFC***. A persona in many trees is many independent cryptographic states: a device compromise heals only in the tree that re-keys, and fan-out and catch-up for that persona are per-tree, matching the uploaded spec's honest cross-group cost (a persona in fifty groups is in fifty trees).

**A mitigation the uploaded spec's topology section did not cite.** RFC 9750 §6.2 also supplies a mechanism for the per-tree-healing cost: applications "can strengthen connectivity among parallel groups by requiring periodic key updates from a user across all groups in which they have membership," and "MLS provides a pre-shared key (PSK) mechanism that can be used to link healing properties among parallel groups." ***Verified-RFC***. So the "each tree heals separately" cost is not unmitigated: a PSK linking the parallel groups, plus a policy of cross-group key updates, lets a compromise recovery in one tree propagate healing to the others rather than requiring wholly independent recovery in each. This is a standards-sanctioned tool for the exact cost the sharding split incurs, and it is the natural companion to the resumption-PSK re-entry already used in §7.7. The residual is that this is a *mechanism*, not a free result: cross-group key updates are themselves O(N)-per-tree operations, so linked healing trades recovery-completeness against added commit load, a dial rather than a solution. ***Design*** for the Drystone use; ***Verified-RFC*** that the PSK-linking mechanism exists.

**The expensive shard is forced by entitlement divergence, not by size alone** (Part 2 §7.8). Before concluding that a large attesting core must split into multiple full MLS groups, apply Part 2's side-history cost ladder, because it draws the line precisely. A structural split that keeps the *same* entitlement (the same attesting set, merely organized into more than one history) is Part 2's **Tier 2**: a second dataplane hash tree sealed under the same Group's keys, entitlement inherited from the parent, costing "another hash tree to converge and nothing more, no O(N) instantiation, no ratchet tree, no Welcomes." Only when entitlement must *diverge* (a subset who may act or read and others who may not, cryptographically enforced) is Part 2's **Tier 3** forced: a real MLS branch over the subset with its own key layer, carrying the full O(N) instantiation and the freeze-then-strand hazard. Part 2's line is sharp: "entitlement inheritance is what makes tier 2 cheap, and also what disqualifies it the moment access must narrow." So the O(N)-per-subgroup cost of sharding the attesting core is incurred only where the shards need *different* membership or entitlement; a core that is merely large but uniformly entitled can be organized with Tier-2 structure at no O(N) instantiation cost. This refines the ceiling analysis: size alone does not force the expensive shard, entitlement divergence does. ***Verified*** (the Tier-2/Tier-3 cost distinction, per Part 2 §7.8).

**Federation of the core, and where the standards floor ends.** For an attesting core large enough to need many trees as one community, the composition is the Group-as-principal nesting (uploaded spec §5.10: a Group is a principal, communities nest, a federation is a grouping of communities), so a sharded attesting core is federated rather than run as one tree. The honest boundary: MLS federation is a separate and not-yet-complete standards effort, not part of the RFC-guaranteed core. RFC 9750 states that "while this document does not specify all necessary mechanisms required for federation, multiple MLS implementations can interoperate to form federated systems if they use compatible authentication mechanisms, cipher suites, application content, and infrastructure functionalities," and points to a separate FEDERATION document. ***Verified-RFC***. The IETF MIMI working group is the active effort on cross-provider interoperability. So federating a large attesting core is architecturally the right shape but rests on standards work still in progress, which places it firmly in the experimental register: the per-tree mechanics are RFC-grounded, but composing many trees into one federated community with consistent membership authority across them is beyond what the current RFCs guarantee, and is a Drystone construction question layered on an evolving standard.

**The residuals (Rule 9).** First, the single-tree ceiling's exact value is unmeasured at the substrate's design-target size: the band between the measured ~5,000 and the RFC's tens-of-thousands aim is where the real ceiling sits, and closing it is the §7.11 first measurement applied to the attesting core specifically (per-commit and Welcome cost at attesting-N of 5,000, 10,000, 20,000). Second, the parallel-groups win inherits the uploaded spec's conditionality: whether N smaller trees beat one large tree depends on the attesting core's actual distribution across groups, which is not established, matching the uploaded spec's `[confirm]` on the topology win pending a realistic membership distribution. Third, PSK-linked cross-group healing is a mechanism whose cost (added per-tree key updates) has not been modeled against the healing-completeness it buys, so "linked healing mitigates the sharding cost" is a direction, not a sized result. ***Design, experimental*** for the composition; the cited RFC mechanics are ***Verified-RFC***.

---

## 7.10 Tiered performance expectations

**Realizes: P (usability as a first-class constraint).**

Stated against the fixed operating policy: 90-day-class cold transfer (per the §7.6 schedule), a few member-removals per day at most (each an O(live-N) healing commit), synchronous key agreement, and scaled PCS (strict below 250, opportunistic above). Every tier gives both the total roster and the expected live hot-N, because all costs scale on the latter.

***Synthesis*** across all four tiers: the *regime transitions* (where a constraint becomes load-bearing) are the reliable content, grounded in the RFC complexity results and the platform evidence. The *absolute* figures are reasoned to the right order of magnitude and are **not** measured; the two measurements in §7.11 place real numbers on them. Read the tiers as an envelope shape, not as SLAs.

### 0–1k (hot-N: up to ~1000)

Everything is comfortable; strict PCS throughout is affordable; no cold split needed. Per-message O(1), commit O(log N), state a few MB, none strains a device. Below 250 is the strict-PCS gold band, indistinguishable from a small secure group. From 250–1k, strict-or-near-strict PCS is still cheap enough to keep by default; the 250 threshold marks where strictness becomes mandatory-by-policy, not where it becomes unaffordable. Delivery is swarm-dominant; store-and-forward barely invoked. **No usability tax.**

### 1–3k (hot-N: ~200–600)

The cold split begins to earn its keep; PCS goes opportunistic above 250; experience remains strong. Total-N and hot-N diverge meaningfully; all crypto costs are paid on the few-hundred live core. Opportunistic PCS avoids a felt background heal cost. The one new cost is a *returning* member's backfill (a "syncing" moment, borne by the returner, invisible to others), which progressive return (§7.7) keeps non-blocking. **First deliberate, invisible relaxation (strict → opportunistic PCS).**

### 3–7k (hot-N: ~400–1400)

The cold split is load-bearing; opportunistic PCS is essential; delivery becomes a tuned curve; experience is good with managed backfill. A 7k group with a ~1200 live tree behaves cryptographically like the 1k tier; the ~5800 cold members cost near nothing. Forced strict healing across a 1000+ tree would be the "secure but unusable" failure, so opportunistic PCS is required. Delivery leans on the store-and-forward/swarm race and operates on the latency curve rather than the free corner. **First tier where good experience requires active design** (non-blocking backfill, graceful delivery degradation), not just favorable protocol properties.

### 7–10k (hot-N: ~700–2000)

The hot tree itself approaches the sizes where MLS's own linear costs are felt; cold split is mandatory; delivery is firmly on the tradeoff curve; usability is an engineering achievement, not a default. A 10k roster with a 90-day window may carry a 1500–2000 live tree, paying 1k-tier-and-up costs on the hot set, with a large (8000+) cold archive. The critical question is whether liveness keeps hot-N in the comfortable band: if engagement is low, a 10k group behaves like the manageable end of 3–7k; if unusually high, hot-N may push past ~2000 and demand one of three responses, a tighter window (the dynamic knob, §7.6), hot-Group sharding, or entering the public-by-default regime (§7.9.3) that concedes content confidentiality to shed the encrypted-large-tree cost entirely. PCS is opportunistic and the "validity over secrecy" posture is fully assumed. Delivery lives in the store-and-forward-assisted region much of the time, fine for asynchronous, announcement-like, forum-like communities (what most groups this size are), marginal for tight real-time sync at this scale. **The protocol carries 10k; whether it is a good product at 10k depends on progressive-return UX, delivery-degradation UX, and honest liveness tuning.**

### Cross-tier through-line

- Crypto cost scales on hot-N, not total-N; the cold split is what makes 10k tractable.

- Sub-250 strict PCS is the no-compromise gold tier; opportunistic PCS above 250 preserves experience by trading an invisible property for felt responsiveness, increasingly essential going up.

- Bans are never a cost driver at the fixed policy; message fan-out is the real cost, and it is a graceful-degradation curve, not a wall.

- Experience is free at 0–1k, needs design at 3–7k, and is an engineering achievement at 7–10k.

- The liveness window is the master scaling knob: tighten to shrink hot-N and preserve performance at the cost of more returns; loosen for better return experience at a larger hot tree.

### 7.10.1 Experiment specification

`Purpose: turn the narrative tiers above into a buildable experiment matrix. The prose tiers state the expected regime and its reasoning (Rule 5); this subsection states the variables, procedures, and pass/fail thresholds an implementation harness measures against. Every quantity here is a hypothesis to test, not a verified figure: the expected values are the envelope shape from the prose, and a measured result that contradicts one is information, not a harness bug. All items feed the two ***Load-bearing, unearned*** measurements of §7.11.`

**Cost model and symbols.** The experiments measure these quantities; the symbols are used consistently across every experiment below.

- `hot_N`: number of live member clients in the hot Group (the ratchet-tree leaf count). The primary independent variable.

- `total_N`: total roster (hot plus cold). Independent; varied to confirm cold members are near-free.

- `t_commit(hot_N)`: wall-clock time for one member to *process* an incoming commit at a given hot_N. Measured output.

- `sz_commit(hot_N, path)`: serialized byte size of a commit, with `path` a boolean for whether it carries a full UpdatePath (healing) or not (bare add/remove). Measured output.

- `t_join(hot_N)`: wall-clock time for a new client to process a Welcome and initialize state at a given hot_N. Measured output.

- `sz_welcome(hot_N)`: serialized byte size of the Welcome (carries the ratchet tree). Measured output.

- `state_resident(hot_N)`: resident memory of the ratchet-tree and key-schedule state a member holds. Measured output.

- `t_msg`: wall-clock time to encrypt or decrypt one application message (hypothesis: independent of hot_N). Measured output.

- `fanout_total(live_online, degree)`: total network messages to deliver one payload to the online-live set over the overlay, as a function of the online-live count and the gossip fan-out degree. Measured output.

- `fanout_latency(live_online, degree, reach)`: time for a payload to reach a target fraction (`reach`, e.g. p50, p95, p99) of the online-live set. Measured output.

- `t_backfill(gap)`: wall-clock time for a returning client to reconcile a dormancy gap of `gap` events/epochs via history-DAG hash-range query. Measured output.

**Fixed operating policy for all experiments** (the §7.10 preamble, made concrete):

- Ban rate: 5 healing commits/day (upper bound of "a few a day"), each with a full UpdatePath. This is a *load* parameter, not a variable, except in Experiment E where it is swept.

- Add rate: 1/hour into the hot Group, bare (no UpdatePath), batched where concurrent.

- PCS: strict (forced periodic Update, cadence 24h) below hot_N = 250; opportunistic (no forced Update; heals only on ban/organic) at and above 250.

- Ciphersuite and credential type: pin one MLS ciphersuite and one credential type per run, and record them, because `sz_commit`, `sz_welcome`, and `state_resident` all depend on them (§7.4 note). Run the sweep at minimum with a bare-signature credential; optionally repeat with an X.509 credential to bound the heavier case.

**hot_N sweep points** (shared across Experiments A–C): `{50, 100, 250, 500, 750, 1000, 1500, 2000, 2500}`. The 250 point is included twice in analysis intent: as the top of strict and the bottom of opportunistic, so the PCS-cadence cost discontinuity is visible.

**total_N points** (Experiment D): `{1000, 3000, 7000, 10000}` each run at a fixed hot_N to isolate cold-carriage cost.

The experiments, each stated as independent variable, procedure, measured output, expected shape, and pass/fail threshold:

**Experiment A: commit processing and size vs hot_N.** Independent: `hot_N` across the sweep. Procedure: build a hot Group at each sweep point; issue one healing commit (full UpdatePath) and one bare add-commit; measure `t_commit` and `sz_commit(path=true)` and `sz_commit(path=false)` at each. Expected shape: `t_commit` grows with log(hot_N) for the compute path; `sz_commit(path=true)` grows roughly linearly in hot_N (UpdatePath resolutions sum toward hot_N); `sz_commit(path=false)` roughly flat. Pass/fail: `t_commit` at hot_N=2000 stays within an interactive budget (target: under 50 ms on the reference aarch64 device, ***Load-bearing, unearned*** threshold to confirm or revise); if `t_commit` grows faster than log (i.e. superlinear), flag as a tree-hygiene or implementation problem, not an expected result. This experiment sets **the hot-N comfort ceiling** (provisionally ~1500 in §7.6): the ceiling is the hot_N beyond which `sz_commit(path=true)` or `t_commit` exceeds the interactive budget.

**Experiment B: join and Welcome size vs hot_N.** Independent: `hot_N`. Procedure: at each sweep point, add one fresh client and measure `t_join` and `sz_welcome`. Expected shape: both linear in hot_N (Welcome carries the whole tree, §7.4). Pass/fail: `sz_welcome` at hot_N=2000 stays within a one-shot transfer budget acceptable over a mobile link (target: under a few MB, confirm); `t_join` stays within a tolerable one-time pause (target: under ~2 s, confirm). This is the cost a *fresh* join pays; note it is distinct from a *cold-return* (Experiment F), which pays external-commit plus backfill instead.

**Experiment C: message and state cost vs hot_N.** Independent: `hot_N`. Procedure: at each sweep point, measure `t_msg` (encrypt and decrypt one application message) and `state_resident`. Expected shape: `t_msg` flat (independent of hot_N, the O(1) claim of §7.4); `state_resident` linear in hot_N. Pass/fail: `t_msg` shows no systematic growth with hot_N (if it does, the O(1) message claim is violated and §7.4 must be revised); `state_resident` at hot_N=2000 fits comfortably in a mobile process (target: tens of MB, confirm). This experiment directly tests the load-bearing §7.4 claim that per-message cost is size-independent.

**Experiment D: cold-carriage cost (total_N with fixed hot_N).** Independent: `total_N` across its points, hot_N held fixed (e.g. at 1000). Procedure: build a community with the given total_N split into a hot Group of 1000 and a cold Group of the remainder; run the fixed message/ban/add load *in the hot Group only*; measure `t_commit`, `t_msg`, `state_resident`, and `fanout_total` for a hot-Group member. Expected shape: **all outputs flat across total_N**, because cold members do not participate in hot-Group operations (§7.6). Pass/fail: if any hot-Group cost grows with total_N, the cold split is leaking cost and the "cold members are near-free" claim (§7.6) is violated; this is the experiment that confirms or breaks the central structural move.

**Experiment E: ban-rate stress and batching.** Independent: ban rate, swept `{5/day, 50/day, 500-in-a-burst}` at a fixed hot_N (e.g. 1000). Procedure: for the steady rates, apply removals as individual healing commits and measure aggregate `t_commit` load and `fanout_total` per member per day; for the 500-burst, run it twice, once as 500 individual commits and once as a single batched multi-remove commit, and compare total fan-out bytes and total processing. Expected shape: steady rates negligible against message traffic (§7.10 "bans are never a cost driver"); the burst as individual commits produces roughly 500x the fan-out of the batched form. Pass/fail: batched-burst total fan-out is within a small multiple of a single healing commit's fan-out (confirming batching collapses the raid case, §7.6); if individual-commit burst does *not* dominate, re-examine the fan-out model. This experiment confirms that the fixed-policy ban rate is safe and that raids are survivable only via batching.

**Experiment F: cold-return cost and progressive-return latency.** Independent: dormancy `gap` (events/epochs missed), swept to correspond to `{1 day, 30 days, 90 days, 365 days}` of the fixed load at a given hot_N. Procedure: take a client dormant for each gap; measure (i) the time to re-establish current keys via external commit carrying a resumption PSK (should be independent of gap, since it re-keys to current epoch), and (ii) `t_backfill(gap)` for history-DAG reconciliation of the gap. Expected shape: re-key time flat across gap; `t_backfill` grows with gap. Pass/fail: the returner can *participate at the current epoch* (re-key complete) within an interactive budget regardless of gap (confirming progressive return, §7.7 normative clause), and `t_backfill` at the 90-day and 365-day gaps stays within a tolerable background-sync duration (target to confirm). This is §7.11's **second measurement** (return-backfill time vs gap) and it directly sets whether the §7.6 liveness-window schedule is right: if 90-day backfill is intolerable even in the background, the windows must shorten.

**Experiment G: fan-out curve and graceful degradation.** Independent: online-live count `live_online` `{100, 500, 1000, 2000}` and overlay fan-out `degree`; and a partition/sparsity parameter simulating fraction of the live set momentarily unreachable. Procedure: on a gossip testbed with the store-and-forward/swarm race enabled, deliver a payload and measure `fanout_total` and `fanout_latency` at p50/p95/p99 across `live_online`, then repeat with increasing unreachable fraction to observe the race falling through to store-and-forward. Expected shape: `fanout_latency` rises gracefully with `live_online` and with unreachable fraction, never cliff-edges to failure (§7.9 "curve, not a ceiling"); store-and-forward carries the tail when swarm cannot. Pass/fail: no delivery *failure* at any tested sparsity (delivery degrades to slower, not dropped); p95 latency at `live_online=2000` characterizes the upper-tier delivery experience and feeds the §7.10 3–7k and 7–10k "delivery on the curve" statements. This experiment, with A, is §7.11's **first measurement** (per-commit and fan-out cost at hot-N).

**Mapping experiments to the tiers.** Each narrative tier's load-bearing claim is tested by specific experiments, so a tier's "expected regime" becomes checkable:

| Tier | Load-bearing claim | Tested by | The number the experiment produces |
|---|---|---|---|
| 0–1k | Strict PCS affordable; nothing strains a device | A, B, C at hot_N ≤ 1000; the 24h-strict cadence as load | `t_commit`, `t_join`, `t_msg`, `state_resident` all well within budget through hot_N=1000 |
| 1–3k | Cold split earns its keep; opportunistic PCS avoids felt heal cost | D (cold-carriage flat); A with and without forced 24h Update to quantify the strict-vs-opportunistic delta | The per-day heal cost that opportunistic PCS avoids above 250 |
| 3–7k | Hot tree behaves like 1k tier; delivery on the curve | A, C at hot_N ≈ 1200; G at `live_online` ≈ 1200 | Confirmation that hot_N-sized cost, not total_N-sized, is paid; p95 delivery latency at ~1200 |
| 7–10k | MLS's own linear costs felt on the hot tree; sharding-or-tighten decision | A, B, C at hot_N ∈ {1500, 2000, 2500}; the comfort-ceiling determination | The hot_N at which `sz_commit`/`t_commit`/`sz_welcome` exceed budget = the real ceiling and the sharding trigger |

**Reference harness (current conforming implementation, Rule 3).** The requirement is an MLS implementation instrumentable for per-operation time and serialized size at controlled `hot_N`, plus a gossip-overlay testbed with the store-and-forward/swarm race. Current realization: OpenMLS built for aarch64, driving its group operations at the sweep points and recording `t_commit`, `sz_commit`, `t_join`, `sz_welcome`, `t_msg`, and `state_resident`; and a gossip testbed (the §7.9 realization, Part 2 §6) instrumented for `fanout_total` and `fanout_latency`. Pin and record the OpenMLS version, the ciphersuite, the credential type, and the device SoC with every result set, because every measured figure is contingent on them and a result without them is not comparable across runs. **[gates-release]** for any figure that becomes a stated SLA.

---

## 7.11 Open items and the two unearned measurements

Genuinely undecided or unearned, kept distinct from the decided-and-bounded postures of §7.12 (Rule 9).

**The two measurements that turn the envelope into a sized envelope (both ***Load-bearing, unearned***):**

1. **Per-commit and fan-out cost at hot-N = 500 / 1000 / 2000, on representative hardware.** Sets how tight the liveness window can be pushed (how small the hot tree gets), whether the 3–7k and 7–10k hot trees stay comfortable or need sharding, the real hot-N comfort ceiling (provisionally ~1500), and how heavy forced strict healing would actually be above 250. Gettable from an OpenMLS-on-aarch64 build running the group benchmark at these sizes plus a gossip testbed measuring fan-out latency and total message count versus live-N. *Extension for the experimental public regime (§7.9.3.3):* run the same commit and Welcome/GroupInfo benchmark at attesting-N of 5,000, 10,000, and 20,000, to place the single-tree attesting-core ceiling in the band between the measured ~5,000 (Soler 2025) and RFC 9750's tens-of-thousands design target; and, for the sharded case, measure the added per-tree cost of PSK-linked cross-group key updates against the healing-completeness they buy.

2. **Return-backfill time as a function of dormancy-gap size.** Sets whether a 90-day return feels like a quick sync or a long wait, and therefore whether the §7.6 windows are right or want shortening. Gettable from the history-DAG hash-range backfill against a history-convergence node over representative gap sizes.

**The sound-but-unvetted compositions (each ***Design*** or ***Synthesis***, none carrying an external proof):**

3. **Gap-completeness, the narrowed keystone (§7.8).** ***Load-bearing, unearned.*** The earlier framing of this item as "prove cross-type standing-merge commutativity from scratch" is superseded: Part 2 §7.3.1 and §7.3.2 *specify* the resolution (a layered operation-type precedence, then causal precedence, then a content-address tiebreak, with cross-slot effects as projections on resolved slots), which makes the fold's order-independence **constructive rather than assumed**. So the cross-type resolution is no longer the open problem, it is a specified ***Design*** mechanism adopted from Part 2 (§7.8 keystone). What remains unearned is the single narrower property Part 2 itself flags as the residual beam: **gap-completeness**, whether a node can reliably tell it holds the complete causal set of facts up to a position. The resolution is order-independent only over a *complete* set (an absent predecessor is a detected gap, Part 2 §7.3.2, and the resolution does not fold onward as if complete); if gap-completeness holds, standing resolution is consistent across converged nodes by construction, and if it fails, a node folds an incomplete set and can diverge. This is the Appendix B beam (Part 2 §7.9.2).

    **What the local doc must still do, given Part 2's resolution is adopted.** Two smaller obligations remain, both narrower than the old "prove commutativity from scratch":

    - *Confirm the local doc's standing event-types map onto Part 2's tiers.* The local doc uses `ban`, `unban`, `suspend`, `reinstate`; Part 2's layered order is stated over threshold changes, membership removals, role/capability removals, grants, and additions. Each local standing type must be placed in that tier structure (a ban is a membership removal carrying a ceiling stamp; a reinstate is a membership addition; suspend/unban must be assigned tiers), so that the local vocabulary inherits Part 2's constructive order rather than defining a parallel one. This is a mapping check, not a proof, and it is the standing obligation on any future standing type: place it in Part 2's tier order before admitting it. ***Design***.

    - *The gap-completeness beam is the actual proof target*, and it is Part 2's, not a local-doc-specific one. It gates the safety of eventual-consistency ban propagation and the head-resolved ban check (both assume converged nodes agree, which holds if and only if they folded complete sets). Both are sound *given* gap-completeness and unproven *without* it.

    **The concrete failure if gap-completeness does not hold.** A node that has folded an *incomplete* set can resolve standing differently from a node with the complete set, so one admits a returning lineage's external commit and the other rejects it, a **group fork** on membership that does not self-heal because neither node is wrong over the set it holds. This is distinct from the determinate-late-ban case (§7.8), which does self-heal: the fork here comes from *incompleteness*, not from ordering, which is why gap-completeness is the property that must be earned. It is silent until a specific gap pattern occurs in the field, so it must be confirmed by the Appendix B gap-detection work rather than left to surface in production.

    **Discharge path.** Part 2's Appendix B carries the gap-completeness beam as its named open problem; the local doc inherits it rather than restating a separate proof. The mapping check above (local standing types onto Part 2 tiers) is small and does not need a running system. On the beam's discharge, the eventual-consistency safety and head-resolved ban check move from ***Load-bearing, unearned*** to ***Design***; the tier-mapping obligation on future standing types remains permanently.

4. **The two-part re-entry credential (§7.7) and its ban-lineage interlock (§7.8).** Sound composition of confirmed primitives (resumption PSK, external commit, governance attestation), now grounded in Part 2's decision-vs-enactment split (§7.3.6, §7.6.7) and GroupInfo-as-claim corroboration (§7.4.2), but the binding that makes the ban-lineage check unforgeably upstream of the resume-vs-fresh decision, so a banned lineage cannot present a valid-looking credential while a legitimate new-device return can, is Drystone's construction and wants adversarial analysis.

5. **The resume-vs-fresh identity fork.** The design commits that a return within the window with valid continuity resumes the same author lineage, and outside the window or without valid continuity re-enters as a fresh principal, with the ban-lineage check upstream of both. The exact continuity-certificate contents and the window-as-fork-selector are ***Design***; the fork's correctness under the author-lineage (not per-client) identity model is unproven.

6. **Single-member time-delayed resumption-PSK presentation.** ***Verified-RFC*** that the resumption-PSK primitive is single-injection and time-arbitrary (not restricted to bulk branch/reinit). That individual re-entry into a long-lived Group is a *sanctioned pattern* rather than a novel composition of a blessed primitive is **not** established; and the RFC leaves ratchet-tree conveyance to the application, which the history-convergence node supplies, a Drystone construction, ***Design***.

7. **Non-member-verifiable membership attestation, for the experimental public regime (§7.9.3.1).** ***Load-bearing, unearned.*** The bridge feeding the AppView must let a *non-member* reader verify "attested member at standing X authored this item" from a forwarded artifact, without trusting the bridge and without the reader being an MLS member (which would defeat the read-outside-the-tree model). This is the mechanical core of the whole public-regime bridge and it is a real cryptographic design problem, not a solved one: it may reduce to signing authored items with a credential chain verifiable against the group's published membership and governance state, but that is a sketch, and it composes with the lineage-attestation questions in items 3 through 6. Until solved, the entire §7.9.3 regime is experimental, gated on this. The whole of §7.9.3 and §7.9.3.1 additionally carries a status weaker than the rest of §7: it is a candidate direction to prototype, not a specified commitment.

**[gates-release]:** the wire encodings of the two-part credential, the governance-position pointer carried on each payload, and the ban-ceiling event type must be pinned before public release.

---

## 7.12 Posture summary

Per Rule 10: one row per MLS-or-substrate case this section resolves, in body order. Decided-and-bounded postures only; genuinely-undecided threads are in §7.11, not here.

| Case (§) | What the substrate assumes / requires | Drystone posture (outcome) | Forcing reason |
|---|---|---|---|
| Linear tree/join/UpdatePath cost (§7.4) | MLS state, Welcome, and healing commits are linear in the group's member count | MLS holds only the live set; linear terms are paid on live-N, not the roster | Usability: recurring device cost must scale on live members |
| PCS via periodic Updates (§7.5) | RFC recommends scheduled key-refreshing Updates (hours–days) for continuous PCS | Strict below 250; opportunistic (natural heals only) at and above 250 | Usability: forced O(live-N) heals above 250 are a felt cost for an unobservable property |
| Removal enacts exclusion (§7.5, §7.8) | A removal with UpdatePath re-keys and excludes the removed leaf | Bans are enacted as removal-with-UpdatePath; exclusion is the mechanism | Non-domination: a ban must be enforceable, not merely recorded |
| KeyPackage adds a member (§7.7) | A KeyPackage is consumed by an existing member to build a Welcome | Not used for return; return is self-service external commit at the returner's cost | Usability + local authority: the returner bears its own re-entry cost |
| Resumption PSK proves prior membership (§7.7, §7.8) | A resumption PSK from epoch *n* proves membership at *n* when injected later, across key changes | The durable continuity token linking cold to hot; cold Group rolls freely | Usability: long dormancy without frozen cold state |
| GroupInfo authoritative to a rejoiner (§7.7, §7.8) | An external joiner cannot fully validate GroupInfo freshness (MLS recovery hazard) | GroupInfo is a claim corroborated against the governance chain, not authority (Part 2 §7.4.2); standing resolved to head; narrow residual (a node behind the stale epoch) | Provenance + validity: recognition is a governance fact, not a substrate claim |
| Ban semantics (§7.8) | MLS removal cuts a leaf; no notion of a lineage-scoped, provenance-bearing departure | A ban is a fork: the removed lineage is forked off whole keeping prior state, losing only forward corroboration, leaving a quorum-stamped ceiling (Part 2 §7.3.5, §7.6.4); genuine contradictions escalate, not fold | Authority reaches corroboration, never what a peer holds; furnish provenance, render no verdict |
| MLS sees clients, not lineages (§7.8) | MLS membership logic operates at the leaf/client level | Ban enforcement is an application gate resolving lineage against the ceiling; bans are lineage-scoped | Non-domination: a banned lineage must not re-enter on a fresh client |
| DS orders commits (§7.9) | MLS permits a DS to order/deliver; libraries often lean on it | DS reduces to a buffer; ordering is cryptographic and recipient-reconstructed; store-and-forward/swarm race, first-to-reach wins | Non-domination: infrastructure carries, never adjudicates |
| Intra- vs cross-chain ordering (§7.8, §7.9) | Intra-chain commit order is total via the confirmed transcript; cross-group order is not | Intra-chain race is fork-proof; cross-chain ordering eliminated by placing it on the single governance chain, backstopped by re-keying re-exclusion | Group validity under eventual consistency without membership consensus |
| Force 1, key-agreement cost (§7.9.1, §7.13) | Pairwise/sender-key E2EE setup cost grows with membership, capping E2EE groups near 1,000 | Defeated: MLS tree agreement (log cost) seeds a per-sender symmetric ratchet; sender keys at the floor, no cheaper form to reach for | Confidentiality as gradient means; large-and-encrypted requires log-cost keying |
| Force 2, server-mediated function (§7.9.1, §7.13) | Ranking/search/discovery/moderation need server-readable plaintext, which E2EE denies | Defeated by construction: no server that ranks/searches/adjudicates; moderation moves to governance-and-lineage, not content-reading | Local authority first; the deliberate price of E2EE at scale |
| Read-mostly public audience (§7.9.2) | Cheap indexable reads (AppView pattern) require the server to read content | Optional public-projection cache serves only a deliberately-published non-confidential projection, content-addressed and governance-positioned; never decrypts the interior | Helper-not-authority; two confidentiality domains, boundary is a publish decision not a held key |
| Above ~7k: confidentiality vs scale (§7.9.3) | Every large platform concedes group-text E2EE; staying encrypted costs hot/cold and delivery overhead on a large live set | Optional public-by-default regime: messages public (AppView is primary surface), MLS retained for attestation and membership only; content confidentiality conceded transparently, cryptographic membership kept | Transparent tradeoffs and honest handling; concede Force 2 on purpose, retain Force 1 where it pays |
| Attesting-core single-tree ceiling (§7.9.3.3) | One MLS group targets tens of thousands (RFC 9750 §2.1), benchmarked to ~5,000 (Soler 2025) | Tree holds only the acting-and-governing core (readers off-tree), so the ceiling is over the minimum possible population; real value unmeasured in the 5k-to-tens-of-thousands band | Usability; the linear ratchet-tree cost is paid over the smallest arrangeable set |
| Attesting-core beyond one tree (§7.9.3.3) | Parallel groups supported, security not weakened, but not strengthened across groups; each heals separately (RFC 9750 §6.2) | Shard the core across trees; per-tree healing mitigated by PSK-linked cross-group updates (RFC 9750 §6.2); federate via Group-as-principal, on evolving federation standards | Non-domination; standards-grounded per-tree, federation experimental pending MIMI/FEDERATION |

---

`End of §7. History of how this section's decisions were reached lives in the session-summary companion, not here (Rule 1). Vocabulary and supersession live in the conventions reference (Rule 13). The chained cast journey lives in the §7 appendix (Rule 4).`

---

## 7.13 Empirical basis: group size and community-moderation shape

`Purpose: state, as its own grounded content, the empirical picture the rest of §7 rests on when it says "most members dormant," "bans are rare," and "cost scales on the live set." Elsewhere this evidence appears only as justifying clauses; a premise the design leans on is stated here and tagged, per Rule 6, so it can be checked rather than assumed. The honest status of this whole section is that the distribution *shape* is well-established across independent platforms, while the *per-group operational rates* the design most wants are largely unpublished and are inferred; §7.14 is the research prompt to quantify them as far as ancillary evidence allows.`

**Realizes: the forcing reasons of §7.2 and §7.4 (cost scales on live-N because the live fraction is empirically small).**

### The distribution shape is heavy-tailed and consistent across platforms

Community size follows a heavy-tailed (power-law / log-normal) distribution on every open platform studied, which is why the *mean* group size is not a useful design figure and the *live fraction*, not the roster, is what the architecture must budget for. ***Established*** (the shape is a robust cross-platform finding in the online-community literature; Reddit and location-network datasets both exhibit power-law/log-normal community-size tails). Consequences the design uses:

- Most communities are small; a few are enormous; the largest are effectively unbounded relative to the median. So a design must *tolerate* a very large group without breaking while *optimizing* for the common case of tens to low hundreds of active participants.

- Membership counts massively overstate the active population. Platforms report that a large fraction of members are dormant, to the point that at least one platform is deprecating raw member count in favor of active-visitor metrics as the more honest measure of a community's size. **[confirm]** (this is the Reddit "weekly visitors replacing member count" change; cited in Part 2 §6 references, flagged for confirmation of current status). This is the direct empirical warrant for the hot/cold split: the roster is mostly ghosts, so paying cryptographic cost per roster-member is paying for the dead weight.

### Activity concentrates harder than membership, and concentration intensifies with size

Within a community, participation is far more skewed than membership: a small active core produces the large majority of contributions, and this concentration *increases* as a community grows. ***Established*** (the participation-inequality finding is standard; Reddit-scale studies report roughly an order-of-magnitude core producing the bulk of comments, with activity scaling super-linearly against member count). Two design consequences:

- A large group is not a scaled-up small group; it is a different regime in which a tiny core carries almost everything. This is why "live" is defined by epoch-processing, not authoring (§7.6): the reader-but-not-poster is real and common, and the truly-costly-to-carry population is the fully-dormant remainder, not the silent readers.

- The activity/membership divergence is exactly what the hot/cold split monetizes: the hot Group tracks the participating-or-present core, the cold Group holds the dormant tail, and MLS's per-member costs are paid on the former.

### Moderation is frequent on content, rare on members, and the two must not be conflated

The distinction is load-bearing for the design's ban-rate assumption (§7.5, §7.10), because in MLS a *member removal* rolls an epoch (a cost event) while a *content removal* does not touch the key layer at all:

- **Content removal (posts/comments) runs at a few percent of message volume.** ***Established*** / ***Synthesis*** (multiple independent peer-reviewed measurements on open Reddit data converge on a low-single-digit to high-single-digit percent content-removal rate, with event-driven spikes higher). These are *not* epoch-rolling events for Drystone; they are application-layer content actions.

- **Member bans are a small minority of moderation actions, and thus rare per group per day.** ***Synthesis*** / **[confirm]** (mod-action-log studies establish that user-banning is one of many action types and a small fraction of the total, which is dominated by content removals and approvals; a clean per-group-per-day ban rate is *not* surfaced as a headline figure in the sources found, and is one of the two numbers §7.14 targets). This is the empirical basis for the fixed-policy "a few bans a day at most" of §7.10, and it is the *weakest-sourced* of the design's premises, held as inference rather than measurement.

- **Raids/brigades produce bursts,** which is why batching (§7.6, Experiment E) is required: steady-state ban rate is a non-issue, clustered mass-removal is the real stress and is survivable only as a single batched commit.

### Governance shape: two real-world models bracket the design space

The two most-documented consumer platforms bracket the administrative structure Drystone's governance layer (§7.8) must generalize, and the comparison is stated here because the design's A-CGKA-style "small admin set over a large member set" choice is a synthesis of what these two shapes teach:

- **Flat and symmetric (Facebook groups):** a small number of mutually-equal admins, any of whom can remove any other, no ordering, no hard cap; ban capability shared with a moderator sub-role. ***Established*** / **[confirm]** (Facebook's own help documentation plus consistent practitioner sources; the two-role model may have drifted and is flagged). Failure mode: zero structural protection between admins (a compromised admin can depose the rest); mitigated socially, not cryptographically.

- **Ordered and rank-gated (subreddits):** a totally-ordered mod list where authority flows downward, granular per-mod permissions, and a "listed but zero-permission" state decoupling membership from capability. ***Established*** (Reddit's own moderator documentation). Failure mode contained by the ordering: a low-ranked compromise cannot depose the top; the catastrophic case is top-mod compromise.

The design implication stated in §7.8: rank is a *role* inequality (permitted under the peer model) not a *rights* inequality, so the subreddit-style ordered-authority shape is compatible with peer symmetry, and the flat-symmetric shape is the trivial special case. Both are policies the governance chain can express; neither is imposed.

### Concrete engineering ceilings from a purpose-built platform

Where consumer platforms publish almost nothing per-group, one purpose-built system publishes hard limits that serve as an external reference for "where a system that has solved group messaging at scale draws its lines":

- A default ceiling of **250,000 members** per server before manual intervention, and a **250-role** cap. ***Established*** / **[confirm]** (Discord API documentation, cited in Part 2 §6 references; pinned-revision confirmation flagged). The 250k figure is the outer wall a scaled system tolerates; the 250-role cap is an external reference for governance-structure size.

- A **250-member "large" threshold** above which the platform stops eagerly syncing the full member list and switches to lazy loading. ***Established*** / **[confirm]** (same source). This is the external corroboration for the §7.5 strict/opportunistic PCS threshold at 250: an independent scaled system also treats ~250 as the point where "everyone holds everyone" stops being free.

### The honest status of this evidence

The distribution *shape* (heavy tail, activity concentration intensifying with size, content-moderation-frequent / member-ban-rare) is ***Established*** across independent platforms and is safe to build premises on. The *specific per-group operational rates* the design would most like, member-ban rate per group per day, member join/add rate per group per day, and the live fraction as a function of total size, are **not published at per-group granularity on any consumer platform**, worsened by the post-2018 Facebook API lockdown and by Discord and Reddit reporting only platform-aggregate moderation figures. These are held as ***Synthesis*** / **[confirm]** inference, and quantifying them as far as ancillary evidence permits is the subject of §7.14. Every specific number that enters a stated SLA is **[gates-release]** and must trace to either a primary source or a §7.14 inference with its confidence stated.

## 7.14 Research prompt: quantifying the operational rates from ancillary evidence

`Purpose: the two premises §7.13 holds only as inference, member-ban rate and join/add rate per group per day, plus the live-fraction curve, have no official per-group source. This is the standing research task to quantify them as far as community-moderation signal allows, treating blogs, practitioner writing, and mod-tooling data as admissible corroboration explicitly marked as such. It is carried in the doc so the inference is improvable rather than frozen. The prompt itself, and its epistemic guardrails, are stated in Appendix G (external to this section file) and summarized here as an open research obligation, not a settled result.`

The obligation, stated so it is checkable: produce, for each of {member-ban rate per group per day, member add/join rate per group per day, live-fraction as a function of total roster}, a best-estimate range with an explicit confidence and an explicit source-quality tag, derived by triangulating (i) any primary platform disclosure, (ii) peer-reviewed community-moderation datasets, and (iii) clearly-labeled ancillary sources (practitioner blogs, mod-tooling vendor data, community self-reports), with (iii) used only as corroboration of a range that (i) or (ii) also supports, never as a sole basis. Any figure resting only on (iii) is reported as *indicative, ancillary-only* and does not enter a stated SLA. The full prompt, with its inference rules and anti-credulity guardrails, is the companion research-prompt artifact.

---

`Section §7 complete through §7.14. §7.13 states the empirical basis the design rests on; §7.14 records the standing obligation to improve the two inferred rates. Both are tagged so a reader separates the established shape from the inferred numbers.`
