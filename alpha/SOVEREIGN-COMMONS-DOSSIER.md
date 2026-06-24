---
title: Sovereign Commons / Alt.Drive — Thinking-Journey Dossier
status: working-dossier (consolidation for migration to a dedicated repo set)
compiled: 2026-06-15
spans: transcripts 003, 020–022, 033, 069–085, 125–126, 254–271 (+ topic syntheses)
purpose: >
  Consolidate the entire current thinking journey on building an open,
  sovereign, peer-to-peer social/data platform run as a cooperative, so it
  can be lifted out of the Vivian/Mycelium library and refined in its own home.
---

# Sovereign Commons / Alt.Drive — Thinking-Journey Dossier

> **What this is.** A single consolidation of a multi-month thinking arc that lives scattered across ~25 transcripts and topic syntheses in this library. It pulls together the *why* (philosophy, civics, commons theory), the *what* (goals and aspirations), and the *how* (concrete architecture, economics, governance) so the whole line of thought can move to a dedicated repo set and be refined there.
>
> **How to read it.** Sections 1–3 are the foundation (why this exists, what's broken, what it's for). Sections 4–8 are the design (architecture, identity, economics, cooperative). Sections 9–12 are the working material (naming, open questions, a source map, and a quote/phrase library). Section 13 is a suggested starting agenda for the new repos.
>
> **Reliability caveat — read before quoting anything externally.** Much of the early material (the Gemini-era transcripts, especially 254–262) carries explicit Verification Notes flagging unverified dates, statistics, and likely-fabricated citations (a "Composite Authority Pattern" / CAP risk). The *philosophical and architectural framings are sound*; specific numbers, dates, and verbatim third-party quotes must be re-verified before any publication-grade use. The single most repeated correction: **iroh is at `1.0.0-rc.0`, not "v0.97 / pre-1.0 / a 60-version gap."** Do not propagate the stale version framing.

---

## 0. Start here — two flags

Before refining anything else, two things stand out as the highest-leverage first moves:

1. **Lock the name map first (§1.1).** Altis / Alt.Drive / Vault / Sovereign Commons / Loci are used loosely and interchangeably across the source material. Everything downstream — repo structure, the public product name, the charter, the marketing — references these names, so resolving what each one *is* (and which survive) is the cheapest disambiguation available and should happen before deeper design work.

2. **Treat the provenance debt as a gate (§10.17, and the reliability caveat above).** The Gemini-era transcripts — especially **262** and **254–257** — carry **HIGH provenance risk**. The *framings are sound and worth keeping*; but specific **stats, dates, and third-party quotes need primary-source verification before anything goes public**. The single recurring stale fact to kill on sight: **iroh is `1.0.0-rc.0`, not "pre-1.0 / v0.97 / a 60-version gap."**

---

## 1. The arc at a glance

### 1.1 One project, many names

The thinking is one vision with several layered names that accreted over time. Mapping them is the first thing the new repo should pin down, because the names are used loosely across transcripts.

| Name | Refers to | Era / status |
|---|---|---|
| **Project Sovereign Commons** | The umbrella vision: encrypted vault substrate + federation + cooperative + anchor-peer network. | Current umbrella (262 →) |
| **Alt.Drive** (alt-drive) | The **Vault layer** alone — the encrypted local-folder substrate, stripped of federation/coop/anchor material so it can **ship first, independently**. | Current shipping wedge (271) |
| **Vault** | The user's canonical, content-addressed, encrypted data container. One per user; hostable anywhere. The substrate everything rides on. | Current core primitive |
| **Altis** | Earlier name for the decentralized-social + decentralized-advertising thesis. The consumer-driven ad inversion is its direct descendant. | Superseded-but-living (020, 262) |
| **Alt.** family | Naming convention for "frozen-glory" federated products: **Alt.Chirp, Alt.Vid, Alt.Yearbook**. Brainstorm lineage: SlowTech → Plain → Proper → Solid → **Alt.** | Naming idea |
| **Loci** (also Lesche, Kiva, Serai…) | Candidate name for the *third-place* social product itself (see §9). | Open naming |
| **Vivian / Mycelium** | The meta-project this whole library feeds. Sovereign Commons is a spoke of it. | Parent context |

Layered structure:

```
  ┌─────────────────────────────────────────────────────────┐
  │  Project Sovereign Commons (full vision)                 │
  │                                                          │
  │   ┌──────────────────────────────────────────────┐      │
  │   │  Upper layers: chat · social · ads · co-op     │  ← 269/270/271
  │   │            rides on                            │      │
  │   │   ┌────────────────────────────────────┐       │      │
  │   │   │  Alt.Drive = Vault layer            │  ← ships first
  │   │   │  (encrypted content-addressed       │       │      │
  │   │   │   substrate over iroh)              │       │      │
  │   │   └────────────────────────────────────┘       │      │
  │   └──────────────────────────────────────────────┘      │
  └─────────────────────────────────────────────────────────┘
```

### 1.2 Chronology of the recent arc (the live thinking)

The numbers are time-ordered. The most load-bearing recent transcripts:

- **020 (Altis era).** "A Blueprint for a Post-Platform Internet." Holochain-based agent-centric social platform; the Triple-Win advertising model; the "Free Tier" mandate; CSR (Cryptographic Stateless Rendezvous) as a patentable signaling innovation.
- **254 → 257 (the platform-vs-product spine).** Google+ failure (Yegge platform rant) → Spritely/OCap/petnames → ATProto critique. Establishes *what to avoid* and *what protocol philosophy to adopt*.
- **262 (2026-05-27, Gemini vetting + Claude).** The deepest blueprint. Fact-checked and corrected an over-claimed "Project Sovereign Commons / Altis" pitch. **Dropped Holochain**, chose the iroh stack, established the credit-union analogy, "equal in ability, not capacity," the Lantern visual identity, the three-category privacy model, and the federation strategy. Marks the shift from Gemini to Claude as primary thinking partner.
- **269 (2026-06-02, Claude).** Re-derived the design **chat-first** ("3-way P2P chat, data-sync not central API"), laddering up to the full cooperative + founder-royalty + governance-lifecycle architecture. Produced "the recurring inversion," the HA-peer's many faces, the three mule tiers.
- **270 (2026-06-02, Claude).** Tested Chase's **three-condition chasm hypothesis** for P2P adoption across ~16 projects. Found the **fourth bridge: institutional mandate** (Matrix → 25+ governments). Corrected founder-burnout memory (André Staltz / Manyverse, not Earthstar; Earthstar's Cinnamon passed away 2022).
- **271 (2026-06-03, Claude).** The reconciliation session. Established that 269 and Alt.Drive are the same project from opposite ends (independent convergence = validation). Logged the iroh 1.0-rc correction. **Chase's unifying move:** "messaging = vault artifacts, Automerge when interactive" — one substrate, one transport, one sync engine. **iOS feasibility verdict** (FileProvider + always-on-peer + APNs-wake only).

**Trajectory:** vault-first grounded blueprint (262) → chat-first re-derivation up to the full institutional design (269) → empirical adoption validation + the maintenance-phase thesis (270) → reconciliation onto one substrate + a concrete iOS/transport decision discipline (271).

---

## 2. The problem statement — what's broken

The motivating diagnosis, assembled across the commons/civic transcripts (254, 257, 259, 265, 268).

- **The diagnosed enemy is centralization *capture*, not centralization itself.** "It's not central resources that are an issue, it's centralization *capture* and thus control."
- **The newest enclosure is connection itself.** Having enclosed the physical commons (Enclosure Acts, the Bloody Code), the growth machine moved inward to enclose human relationship — the death of the third place, manufactured loneliness, and corporate platforms that **rent our relationships back to us** (pay-to-boost visibility; feeds tuned for outrage/comparison/anxiety). "A new play is to make even communication and connection a product that is sold to you at a premium. This is one of the saddest narratives in modern life."
- **"Credibly decentralized but operationally centralized."** The generalizable failure pattern (named off the ATProto critique): cryptographic portability that is technically real but economically meaningless because aggregation re-centralizes. The platform must not fall into this trap.
- **Platform-as-afterthought / user-need-last.** The Google+ lesson (§7 of the philosophy extraction): Google asked "how do we get Facebook's social data?" not "what do users need?" — company-centric, with mandatory cognitive friction (Circles), no real platform/API (Yegge: "you can't just bolt it on later").
- **Launch-and-land economics kill the niche communities** that actually care (photographers, Linux folks, writers, RPG groups — the "dive bar" users). The "Maintenance Tax" + promotion-for-launching culture means the things worth keeping get killed.
- **The "strongman game."** Power maintained by force and rationalized as inevitability. The hopeful corollary (flagged "super strong"): *if a system needs that much sustained force to keep people compliant, it is unnatural — and if it was built by force, it can be dismantled by choice.*

---

## 3. The WHY — philosophical & civic foundation

This is the value bedrock the platform is meant to embody. It is the part most worth preserving verbatim, because it is where the originality is.

### 3.1 Two operating systems: linear/extractive vs cyclical/relational (268)

| | Linear / extractive | Cyclical / relational |
|---|---|---|
| Time | line → history → apocalypse | circle: continuous renewal |
| Nature | machine/resource; "subdue the earth" | sacred; kinship over dominion |
| Waste | extract → product → discard | "No such thing as waste" |
| Value of a person | tethered to labor / net worth / class; expendable | inherent; flat hierarchy |
| Cognition | left-brain model over lived experience | present, relational, lived |

Chase's images for the linear OS: a "social virus of fixation on infinite growth," a "death march towards the impossible," a "carrot on a string over rocky cliffs."

### 3.2 The commons is real; the Tragedy is propaganda

- **Hardin's "Tragedy of the Commons" (1968)** was a hypothetical by a eugenicist, not field research; it conflated a *true commons* (organized, bounded, norm-enforcing) with an *open-access vacuum*. Hardin later conceded it should have been "the Tragedy of the *Unregulated* Commons."
- **Elinor Ostrom** (Nobel 2009) proved communities sustain shared resources for centuries — **clear boundaries, local rules, collective choice.** Long-loop proofs: Törbel alpine pasture (since ~1483), Valencia's Tribunal de las Aguas (1,000-yr water court), Bali Subak, Maine lobster "harbor gangs."
- These three Ostrom principles are the proposed **governance DNA of the platform and the co-op.**

### 3.3 The cooperative tradition (MN/WI civic culture)

- **Trust compounds like interest;** the cost of cooperation is low where the historical ROI has been high. Chase's load-bearing framing: *"this is kind of the crux of why trust is also valuable in an enterprise, it's compounding to a certain extent."* → the compounding/decompounding asymmetry: **high-trust systems get cheaper to run, low-trust ones more expensive.** Formalized elsewhere as **trust as the efficiency coefficient, O = P × η** (089).
- **Civic ROI vs Financial ROI:** maximize the "common wealth," generational time horizon, taxes-as-subscription, metric = social trust + human development.
- **Rochdale Principles** (1844): especially **"Education of the Members"** — a co-op can't survive with ignorant members.
- **Living proof:** the **Green Bay Packers** (only community-owned major US team, nonprofit, 500k+ shareholders, NFL banned the model after — "a living museum of a different way of thinking"). **Mondragon** (worker-owned federation). Empirical sustained-yield proof: the **Menominee Forest** (more standing timber today than its 1854 baseline). **Wāpahkoh / Waupaca = "Place of Tomorrow Seen Clearly."**
- **Credit-union lineage** as the institutional model (see §8): Schulze-Delitzsch → Raiffeisen → Desjardins → Filene. Motto: *"Not for profit, not for charity, but for service."*

### 3.4 Protocol philosophy: "Protocols, not platforms" + OCap

- **Masnick, "Protocols, Not Platforms" (2019):** open protocol layer where multiple entities compete; identity and data exist independently of any app; credible exit guaranteed.
- **Object-Capability (OCap) paradigm — "Designation is Authorization."** Holding a reference *is* permission. Built on **POLA** (Principle of Least Authority). Contrast with **ACLs**, which rely on insecure **ambient authority**. Lineage: Mark S. Miller / E-language / KeyKOS / EROS → **Spritely** (Christine Lemmer-Webber): Goblins → OCapN/CapTP → Guile/Hoot. Spritely is a 501(c)(3), **no VC, no tokens**, NLnet/NGI-funded — itself a model of the funding/governance Chase wants.
- **The petname system:** crypto keys mapped to **local, human-readable names chosen by the user**, not a global registry. Identity as context-specific personas (**Unums**), not one global moniker.
- **The structural inversion** (Chase: "this mirrors a lot of my thinking"): move from Account-centric + ACL-defended + algorithmically-discovered (AP/ATProto) → Persona-centric + capability-referenced + introduction-path-discovered (Spritely/OCap). **Introduction paths:** digital stamps, friend-of-a-friend handshakes, contextual slicing.

---

## 4. The architecture journey

### 4.1 Holochain era (020, 033, 069–085, and topic: altis-holochain-architecture)

The original substrate was **Holochain** (agent-centric, no global consensus). The design was fully fleshed out:

- **Two-layer data model:** each agent has one append-only signed **Source Chain** (a "personal mini-blockchain"); a shared **rrDHT** holds validated public state, sharded by **Responsibility Arcs**.
- **DNA = "Per-Network-Mutual-Smart-Contract":** Integrity Zome (immutable validation rules, "the Constitution") + Coordinator Zome (business logic). Validation by random peer-validators; **Membrane Proofs** as admission tickets; **Warrants** as cryptographic proof-of-violation.
- **Networking (Kitsune):** QUIC/WebRTC, ICE/STUN/TURN NAT traversal, bootstrap server for Agent-ID→IP mapping, mDNS on LAN. **Zero-arc nodes** for phones (run rules, keep own chain, store nothing for others — but *cannot choose their validators*, by design, as anti-collusion).
- **Identity:** Ed25519 keypair = permanent global ID; **DeepKey** HD key hierarchy; **Shamir's Secret Sharing** for recovery.
- **CSR (Cryptographic Stateless Rendezvous):** the patentable innovation — eliminate WebRTC's stateful signaling server via TOTP-style predictive rendezvous hashing (`Hash(shared_secret | time_tick)`), analogized to **SYN cookies** and **Ethernet CSMA/CD**. Defensive-patent strategy drafted.

### 4.2 Why Holochain was dropped (262)

- The "stack Holochain on iroh" idea was **illusory** — Holochain 0.6.1 already uses iroh as its default transport, so stacking added nothing net.
- **Holochain on mobile remains weak as of 2026** — a real production constraint.
- **What survived (borrow patterns, not the runtime):** the agent-centric ontology (your source-of-truth is your own signed chain, not a global ledger); source chains → Willow subspaces; countersigning; validation rules; mutual credit (value created by transaction, not minted); fork detection. The OCap paradigm is described as "the deeper version of what Holochain was pointing at."

### 4.3 The current substrate: iroh (262, 269, 271)

**CHOSEN: iroh** (n0, Rust, QUIC-first). Decisive reason: it ships **iroh-blobs**, the content-addressed (BLAKE3) blob primitive "nothing else matches." Stack map:

```
  iroh            QUIC connections, Ed25519 NodeId
   └─ iroh-gossip HyParView membership + PlumTree dissemination
   └─ iroh-docs   eventually-consistent KV (LWW by timestamp+author)
   └─ iroh-blobs  BLAKE3 content-addressed streaming
```

- **iroh-willow** (Willow protocol: 3D data model, Meadowcap capabilities, prefix-pruning = *true* deletion) is **not shippable in 2026.** Plan: build on iroh-docs now, design "Willow-shaped," plan a non-trivial migration. Can use **willow-rs** (Earthstar) for the data model + Meadowcap directly.

**Rejected / demoted:**

- **Veilid** — *demoted to a transport-port candidate*, not a winner. Its crypto set (Ed25519/x25519/XChaCha20-Poly1305/BLAKE3/Argon2) exactly matches Alt.Drive's, and its source-address-free private routing is the **best fit for a future metadata-resistant messaging layer** — but it has **no content-addressed large-blob transfer** (DHT = small mutable records). "A layer question, not a winner question."
- **libp2p** — rejected (mobile-weak; iroh is ~20 lines vs 200+).
- **ActivityPub federation in v1** — dropped (server-to-server, conflicts with local custody behind NAT). Federation priority for *later*: (1) ActivityPub, (2) AT Protocol, (3) Nostr, (4) web bridges. Skip Farcaster/Lens (blockchain deps conflict with values). **Bridgy Fed** (Ryan Barrett) = the AP↔ATProto bridge.
- **"Zero-knowledge matching" via hashed tokens** — rejected as a category error (hashed attributes are NOT ZK; brute-forceable). Use PSI later if needed.

### 4.4 The unifying move (271) — "messaging = vault artifacts, Automerge when interactive"

Chase's key collapse of complexity: one substrate, one transport, one sync engine.

- static/append artifacts → content-addressed blob + manifest entry (iroh-docs LWW)
- interactive artifacts → **Automerge** doc (live-merge over iroh, persisted as a vault blob) — **Automerge 3.0** (~10×, up to 100× memory reduction over v2)
- a group conversation → a **shared vault** (collectionKey wrapped to members)
- a content-blind HA peer → mules the shared vault's blobs without the key

Two sharp edges flagged: (1) segment the conversation log (don't make it "one message") to avoid blowing the ~10K manifest cap; (2) **three consistency models now coexist** (immutable blobs / LWW pointers / Automerge CRDT) — declare the model **per artifact type** in schema; Automerge ops must be **E2E-encrypted above transport** (the content-blind mule terminates iroh's TLS).

### 4.5 The HA / "anchor" peer — one box, many faces (269)

An always-on node (Pi, $5 VPS, NAS) that simultaneously is: superpeer/mailbox + content server/CDN edge (friends pull blobs by BLAKE3 hash) + Bluesky PDS + prekey server (Signal-style) + web bridge (WebSocket/WebTransport, since browsers can't be QUIC peers) + push relay (APNs/FCM) + **"taint" mode** (peers only with your own device keys = private encrypted backup). Reframed in 271 as **"triply load-bearing"** — connectivity + durability + iOS feasibility all depend on it. The push-relay role reintroduces an Apple/Google dependency, which is itself a co-op argument (one Apple Developer account, not per-member).

**Three storage/sync trust tiers ("multi mule modes"):** (1) encrypted-at-rest on your own node; (2) **encrypted-Willow "smart mule"** (HA node backs up a namespace it can't read — *but timestamps must stay plaintext = a real metadata leak; no reference implementation exists, original crypto engineering*); (3) **pure mule mode** (opaque content-addressed blocks, dumb block store).

### 4.6 iOS / mobile feasibility verdict (271)

- **Feasible only in one shape:** **FileProvider framework + always-on-peer + contentless APNs-wake push.** NOT as a full always-on peer (that's how Signal/WhatsApp/Matrix already work).
- Build (Rust+iroh → aarch64-apple-ios via **UniFFI**, the RustDesk/Mullvad/LibXMTP pattern): **high confidence.** Foreground blob sync: **high.**
- **The iOS risk is a *runtime* risk, not an *FFI* risk.** Open question: does the iroh *crate* behave on-device (battery / cellular-NAT / background)? **Unknown — must spike. No documented iroh-on-iOS-in-production reference exists.** Vault-iOS is softer than chat-iOS (files don't need sub-second delivery). Spike deferred until hardware is available.

---

## 5. Identity, trust & reputation layer

- **Favored model:** SSI on **W3C DIDs + Verifiable Credentials, deliberately blockchain-free**, with the **Spritely petname/OCap paradigm** as the deeper direction and **AT Protocol's DID design as the concrete production reference.**
- **Anchoring without central authority:** self-certifying DIDs (identifier = hash of initial DID doc / derived from public key) and `did:web`. ATProto's **`did:plc` + `did:web`** split with **handle/DID separation** ("the SSI promise operationalized at scale") — though `did:plc`'s central directory is flagged as a **centralization vector** and key recovery at consumer scale is "unpolished" (Kleppmann et al., CoNEXT 2024).
- **Core requirement (Chase):** *"I have to be able to create a signing pair and say 'this is me' and build up the social trust graph over time. That ability should coexist with additional appeal-to-authority VC aspects tied back to my own, self-controlled, self-sovereign identity."* → two trust types on one DID: **authoritative** (gov/university VCs) + **grassroots/social** (self-issued + peer VCs).
- **Trust & Sybil resistance:** web-of-trust + **invite-only cryptographic vouching** ("vouched for by two existing members") over economic staking. Surveyed: BrightID (proof-of-personhood — *not* Holochain-based, a recurring correction), Lens, Farcaster. Three Spritely **introduction paths** (stamps, FoaF handshakes, contextual slicing) reframe anti-Sybil as one path among several.
- **Offline transitive trust via Merkle proofs (Chase's proposed design):** each user keeps a Local Trust List → Local Merkle Root; on meeting, exchange roots and request a Merkle Proof that C is in B's graph *without B revealing the full list*. Aligns with the **ToIP Decentralized Trust Graph** WG (no defacto standard yet).
- **Composable moderation:** ATProto **labelers** (subscribe to independent label services) as the model — moderation client-side, not top-down.

---

## 6. Economic model

### 6.1 The consumer-driven ad inversion (the signature mechanism)

The throughline from Altis's **"Triple-Win Model"** (020) to the 269 ad inversion: invert surveillance advertising **from targeting to subscribing.**

| | Web 2.0 | This model |
|---|---|---|
| Data collection | secret behavioral tracking | user **explicitly publishes** interest tags / declares allowed ad types |
| Delivery | pushed via secret algorithm | user's device **pulls** matching offers; matching is **client-side** |
| Payment | platform charges advertiser, user gets nothing | micropayment to user (or a user-chosen non-profit) |
| Privacy | profile from surveillance | agent ID + public tags only; no personal data |

- **Triple-Win (Altis framing):** advertiser gets a verified, pre-qualified impression; user gets zero-cost privacy-preserving relevance; a **user-chosen non-profit** gets a passive auditable revenue stream. Proof-of-View = cryptographically signed attestation that a human device displayed the ad; fraud handled by a reputation service.
- **269 evolution:** advertisers publish offers to a namespace; **the user declares allowed ad types**; payment advertiser→user via **Lightning** (Bitcoin — an existing respected currency, *no new token*), no skim. Many brokers, consumer-side choice — "unpopular ad-gating providers get ignored." **Family-delegated gating** via Meadowcap: "Grandma can use a set list from her adult kids. Even none."
- **Contrast with Brave/BAT:** Brave still centralizes matching and mints a token; this keeps matching client-side and uses an existing currency.
- **Honest open problem:** proof-of-attention / attention fraud is unsolved without a controlled runtime (Brave controls the browser; you don't). Pushed to year 2–3; reputation-gated attestations are the defense.

### 6.2 Adtech as the adversarial counter-case (003 / adtech-attack-surface)

The surveillance-funded RTB/bidstream model is documented as *both* a privacy harm and a security attack surface (malvertising, zero-click render exploits, bidstream harvesting as mass passive surveillance, the "Loser's Data Loophole" where even losing bidders receive precise location/device data — e.g., the Mobilewalla/CBP and Gravy Analytics cases). This is the problem statement the ad inversion answers. (Some CVE/campaign names in that file need independent verification.)

### 6.3 Where blockchain fits (and doesn't)

- **Rejected:** public L1 blockchains for identity (cost/latency/environmental), custom app-chains (full-node requirement defeats device-level decentralization), and trustless maximalism ("protocols that try to do everything without trust hit economic limits that protocols-with-trust don't").
- **Retained as pattern source:** L2 offload/batch ideas map onto local-chain + shared-DHT; DAO governance patterns inform community governance.
- **The economic primitive actually chosen is institutional, not tokenized** — capped/royalty founder returns + cooperative governance (see §8).

---

## 7. Goals & aspirations

Separated deliberately from concrete decisions, because the new repo should keep the line clear.

**Aspirations / values (the "north star"):**

- A **cyclical/relational** operating system: inherent human worth, "no such thing as waste," refuse the engagement feed ("intellectual wandering vs feed-scrolling").
- A **re-claimed commons**, governed by Ostrom principles, not enclosed and rented back.
- **Member-owned cooperative** economics: Civic ROI over Financial ROI, generational horizon, "Education of the Members," trust-compounds.
- **Protocols, not platforms;** genuine decentralization that survives as small self-hosted nodes (escape "credibly decentralized but operationally centralized").
- **Capability/scope-based social fabric:** petnames, context personas, explicit introduction paths; *structural* abuse mitigation (revoke a facet, not the identity).
- **User-need-first**, never data-extraction-first.
- A **digital third place:** equalizing, lingering, between locked-corporate (Slack) and Wild-West-algorithmic (Twitter) — "the digital porch."
- A **"Free Tier" mandate:** "you can be part of this digital conversation if all you have is a phone with CPU, memory, storage, and an internet connection."
- **Security posture: "different, not weaker."** Tiered (baseline/standard/high). Claims properties Signal can't: **transparent offline** (two phones sync over Bluetooth, zero internet) and **no central operator to compel** ("Signal's weakness is organizational — one throat to choke; yours doesn't"). Guardrail: write a per-tier properties matrix so "different" never becomes "rationalized weaker."

**Concrete near/long-term goals:**

- **Near-term:** ship **Alt.Drive** (the macOS encrypted vault) first — single-user value on day one, before any network effects (local-first, Kleppmann et al. 2019). Secure chat + HA + PDS ≈ **9–12 months** (2–3 engineers).
- **Long-term:** full vision ≈ **3–5 year build.** Co-op runs in parallel — "incorporate early, charge dues, build in public."

**Adoption thesis (270):** the **three-condition chasm hypothesis** — a P2P tool reaches mainstream only with (1) product parity, (2) a non-extractive org model that sustains development, (3) an inciting event/cultural shift. Only **Signal** has clearly crossed. **Fourth bridge: institutional mandate** (Matrix → 25+ governments) — maps directly onto "credit union, not a club," and is worth designing for explicitly.

---

## 8. The cooperative / governance model

**Thesis:** "A social utility, structured not to extract but to reinforce. Like a **credit union, not a club.**" Why a co-op: nonprofit = grant-dependent; startup = investor-extraction; **co-op = member-owned, dues-funded, surplus reinvested, indefinite self-sustenance.** "Every revolution has a maintenance phase" → **the co-op IS the maintenance plan** (answers "who's still here in year seven, and why?").

- **Legal structure:** dual-entity **LCA (Limited Cooperative Association) + wholly-owned PBC** (model: Subvert), or a simpler consumer co-op. Statutes named: **Colorado LCA statute**; Wyoming LLC-as-cooperative. (Corrected: the universal "$50 filing / $0 franchise tax" claim is false — CA $800/yr, DE $300/yr; MO genuinely low.)
- **What the co-op operates:** managed HA-PDS nodes (you own keys/data; co-op operates hardware — "like a credit union holding deposits"); OSS software (moat = operational reliability + member trust, not code secrecy); optional ad-broker node; onboarding that makes the architecture "invisible to grandma."
- **Funding:** dues-funded (illustrative $2–5/month; 25K–50K members for sustainability). **Ad revenue flows to members, not the co-op** (co-op takes a small transparent fee).
- **Governance lifecycle (charter from day one — no bait-and-switch):** Stewardship (founders hold board authority) → transition trigger (earlier of ~8 years OR 500 hosted members + 12 months self-sufficiency; ~80% member break-glass override) → professionalized democracy.
- **Enshittification-proof clauses (binding charter):** 6-month deprecation + 18-month LTS; data portability; no dark patterns; **endowment cap** (surplus past N-months reserve → reduced dues / mission spend / donation); ad-revenue transparency.
- **Founder economics — the royalty resolution:** the initial "100× capped return" was rejected as incompatible with cooperative limited-return-on-capital law. Resolution = a **percentage-of-gross royalty** (e.g., **3% of gross**), starting only after 12-month self-sufficiency, framed legally as a **founder's royalty / technology licensing fee** (operating expense, not capital distribution) — "muted on impact," survives founder death, never threatens co-op finances.
- **"Equal in ability, not capacity":** the protocol treats all nodes equally; capacity differs by hardware/operator. Phones *can* be full nodes when users choose; anchor peers are well-resourced participants, not privileged authorities.
- **Lineage:** credit-union founders (Schulze-Delitzsch, Raiffeisen, Desjardins, Filene) and tech-worker-coop references (Informal Systems / Ethan Buchman, Palante, Cooperatus, Patio.coop, USFWC).

---

## 9. Naming & branding

### 9.1 The product name (the third-place social app)

Concept anchor: **Ray Oldenburg's "third place"** (*The Great Good Place*, 1989) — the equalizer; lingering over scrolling; the answer to the death-of-the-third-place enclosure.

**Chase's selection criteria (more durable than any single name):** level/equalizing · lingering over scrolling · "between-ness" · un-loaded · clean digital footprint.

- **Front-runner:** **Loci** (a network of interconnected spaces, not a monolith). Rejected **Stoa** as "in this vein but not that loaded" (brand collision + Stoicism baggage).
- **Live alternatives:** **Lesche** (literally the ancient word for a third place, near-zero footprint), **Kiva** (Puebloan chamber, "no head of the table"), **Liman / Atoll** (the recurring "safe harbor from the algorithmic open sea" image), **Exedra** (semicircular bench — "looking at one another, not at a linear feed"), **Serai/Caravanserai** (Silk Road inns), **Conviva** ("to live together"), **Roji** (tea-garden onboarding path).

### 9.2 The umbrella / substrate names

Project Sovereign Commons (umbrella) · Alt.Drive / Vault (substrate) · Altis (ads/social heritage) · the **Alt.** family (Alt.Chirp, Alt.Vid, Alt.Yearbook).

### 9.3 Visual identity (262)

**The Lantern** — a personal hand lantern (individual sovereignty) + streetlamp (shared infrastructure); warm amber, 19th-century engraving aesthetic; **three flames sharing one chimney** as a subtle community signal. Register adjacent to "Mundus sine caesaribus" ("a world without Caesars" — Jay Graber's SXSW 2025 shirt).

### 9.4 Altis classical naming system (020)

Altis (sacred grove at Olympia — "the trusted gathering place") · Altus (Latin: high/deep/noble) · AltID (identity / "unique torch") · Olympus (aspiration).

---

## 10. Open questions & unresolved tensions

The refinement backlog. Roughly ordered by how load-bearing.

1. **iroh on-device iOS runtime is UNPROVEN** (battery / cellular-NAT / background). No production reference. Must spike when hardware is available.
2. **Web/browser QUIC gap (#1 risk)** — browsers can't be peers; mitigated by the co-op bridge, but "no place where a web browser is a full peer."
3. **Mobile background execution reintroduces Apple/Google** (APNs/FCM mandatory) — a values compromise and a centralization seam.
4. **Encrypted "smart mule" Willow mode is research** — no reference implementation; timestamps must stay plaintext (metadata leak); original crypto engineering.
5. **Willow not shippable** — non-trivial migration from iroh-docs.
6. **Three coexisting consistency models** (immutable / LWW / CRDT) — must be declared per artifact type; Automerge ops need E2E encryption above transport.
7. **Double Ratchet × out-of-order/duplicate gossip** — skipped-key window risk (Double Ratchet assumes ordered delivery).
8. **Ad proof-of-attention / fraud unsolved** without a controlled runtime — deferred to year 2–3.
9. **Reputation/credits system is the hard one** — locally computed WoT, subjective, no global ledger.
10. **`did:plc` centralization vector** + consumer-scale key recovery is unpolished.
11. **Co-locating prekeys + ciphertext** raises blast radius (tiered mitigation: pay for a separate prekey node).
12. **PDS + AI agent on the same box** — a compromised agent's blast radius includes your public social identity (key-isolation matters).
13. **"Referenceable but not public"** is a genuine new capability category (content addressing + authorization-gated fetching) that AP/ATProto handle poorly — design open. (Privacy model: public / secret / referenceable-but-not-public.)
14. **Veilid evaluation half-confirmed** — ahead on privacy/crypto-agility, behind on blob primitives; DHT blob-throughput limits unconfirmed (~80% confident it can't do the large-blob job).
15. **Strategic positioning toward Bluesky / the AT Protocol team** — both technically (the federation bridge) and relationally — is undecided.
16. **Scope-based vs capability-everywhere** — chose scope-based for user-facing simplicity; an explicit simplicity-vs-power tradeoff to revisit.
17. **Provenance debt:** the Gemini-era citations (esp. 262 and 254–257) carry HIGH CAP risk; primary-source-verify before any external use. The quote "every revolution has a maintenance phase" is unattributable folk wisdom.

---

## 11. Source map

Where the thinking lives, so the new repo can cite back.

**Current / live (read in full for this dossier):**
- `transcripts/topics/sovereign-p2p-social-coop-stack.md` — the spine synthesis
- `transcripts/raw/262-project-sovereign-commons-cooperative-altis-vault-protocol-bluesky-vetting-gemini.md` — the deepest blueprint + fact-check
- `transcripts/raw/269-p2p-chat-architecture-iroh-willow-coop-claude.md` — chat-first re-derivation, the recurring inversion
- `transcripts/raw/270-p2p-founder-motivations-adoption-maintenance-quote-claude.md` — adoption thesis, fourth bridge
- `transcripts/raw/271-altdrive-vs-269-veilid-unified-substrate-ios-feasibility-claude.md` — reconciliation, unifying move, iOS verdict

**Vision / economics (Altis heritage):**
- `transcripts/topics/altis-ecosystem-decentralized-social.md`, `altis-decentralized-advertising.md`, `altis-holochain-architecture.md`, `altis-csr-innovation.md`
- `transcripts/topics/adtech-attack-surface.md` — the adversarial counter-case
- `transcripts/raw/020-altis-decentralized-social-platform-gemini.md` (origin)

**Technical building blocks:**
- `transcripts/topics/holochain-networking-and-validation.md`, `ssi-dids-verifiable-credentials.md`, `reputation-networks-trust-graphs.md`, `wot-family-social-app-design.md`, `crypto-blockchain-ecosystem.md`
- raw: 022, 033, 069–085, 125–126 (Holochain/SSI/WoT/P2P-protocol arc)

**The WHY (philosophy / civics / protocol):**
- `transcripts/topics/linear-vs-cyclical-commons-strongman-game.md`, `minnesota-wisconsin-civic-culture.md`
- raw: 254 (Google+/Yegge), 255 (Spritely/OCap/petnames), 257 (ATProto critique), 265 (third-place naming), 268 (commons/Ostrom/strongman)

**Cross-cutting synthesis (consolidated arguments not read here, pull next):**
- `transcripts/topics/cross-cutting-themes-254-263.md` (§I: the platform-vs-product five-transcript spine, 254→262)

---

## 12. Coined-phrase & quote library

The handles worth carrying into the new repo. (Verify third-party attributions before external use.)

**Chase-original framings:**
- **"Equal in ability, not capacity"** — peer-symmetric architecture (flagged as publishable).
- **"The recurring inversion"** — take an extractive stateful intermediary → reduce to stateless/content-blind/optional → wrap in an institution that reinforces rather than extracts (applied at five scales: relay → stateless rendezvous; relay → superpeer "lovely to have, not a have to"; routing server → smart mule; ad platform → consumer-side broker; compellable operator → cooperative).
- **"Credit union, not a club"** / "a social utility, structured not to extract but to reinforce."
- **"The co-op is the maintenance plan"** (paired with "every revolution has a maintenance phase").
- **"Referenceable but not public"** — the novel privacy category.
- **"Renting our relationships back to us"** — the newest enclosure.
- **"The strongman game"**; **"a death march towards the impossible"**; **"carrot on a string over rocky cliffs."**
- **"Trust is the efficiency coefficient — O = P × η"**; the compounding/decompounding asymmetry.
- **"Smart mule" / "pure mule" / "multi mule modes"**; **"taint" mode** (device-exclusive backup peer).
- **"Stateless rendezvous"** (SYN-cookie analogy); **"triply load-bearing"** (the always-on peer).
- **"Different, not weaker"** (security posture); **"transparent offline."**
- "Grandma can use a set list from her adult kids. Even none." (family-delegated ad gating)
- "It's not central resources that are an issue, it's centralization *capture* and thus control."
- "Lens polishing and aperture widening seem like the primary augmentation paradigms." (AI; publishable)

**Borrowed handles worth keeping:**
- **"Credibly decentralized but operationally centralized"** (the ATProto failure pattern).
- **"Message-passing vs shared-heap"** (Lemmer-Webber's architectural fork; shared-heap structurally requires corporate aggregators).
- **"Designation is Authorization" / POLA / ambient authority** (OCap vocabulary).
- **"Civic ROI vs Financial ROI"**; **"No drones in the hive"**; **"Place of Tomorrow Seen Clearly"** (Wāpahkoh).
- **"Launch-and-Land promotion culture" / "The Maintenance Tax" / "Boardroom Delusion"** (platform-failure handles).
- **"Protocols, not platforms"** (Masnick); **"Mundus sine caesaribus"** (Graber).

**Blog candidates surfaced along the way:** "Strategy failure dressed as product failure" (Google+); "The institutional pattern of killing niche successful products"; "Equal in ability, not capacity."

---

## 13. Suggested starting agenda for the new repo set

Not a plan — a refinement on-ramp, derived from the open questions.

1. **Pin the name map** (§1.1) and decide the public product name (§9.1) — everything downstream references it.
2. **Write the per-tier security properties matrix** (forward secrecy / post-compromise / metadata / offline / central-compulsion-resistance) so "different, not weaker" stays honest.
3. **Specify Alt.Drive (the vault) as the shippable wedge** — schema, the three consistency models per artifact type, the manifest cap, the encryption-above-transport requirement.
4. **Define the transport port** so iroh is swappable and Veilid remains a future messaging-layer candidate; mark the iOS-iroh-runtime spike as the gating unknown for any mobile claim.
5. **Draft the cooperative charter** with the governance lifecycle, enshittification-proof clauses, and the founder-royalty framing — early, in public.
6. **Verify the provenance debt** (§10.17) before anything is published.

> This dossier is a snapshot for migration. Once it lands in its own repo, treat §10 as the live backlog and §13 as the first sprint's shape.
