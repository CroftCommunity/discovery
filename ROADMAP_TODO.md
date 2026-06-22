# Roadmap TODO — items up for consideration, with origin back-references

date: 2026-06-22

status: living index. This is a **provenance-indexed backlog**: one row per open item, each
pointing back to where it originated (file:line). It does **not** replace the homes that carry
the reasoning — `ROADMAP.md` (curated roadmap + "Next to do"), `thinking/open-edges.md`
(topic-organized review surface with `[doable]/[decision]/[resource]` tags),
`thinking/open-considerations.md` (unresolved strategic questions), and `COHESION.md` (seams).
It aggregates them into one actionable list so nothing hides in a single doc.

**How to read the Origin column:** `file:line` is a **point-in-time anchor** (as of 2026-06-22) —
files shift, so the *section header* named in the row is the durable reference; the line number
is a convenience. Type legend mirrors open-edges: **[decision]** (user/product gate — surface,
don't resolve), **[doable]** (no blocker), **[resource]** (blocked on hardware/account/network),
**[backport]** (corpus-coherence work), **[explore]** (adjacent/new thinking).

---

## A. Decisions — gates that are the user's to make (surface, don't resolve)

| # | Item | Type | Origin (file:line) | Also tracked |
|---|---|---|---|---|
| A1 | **MPL-2.0 license gate** — `hpke-rs` mandatory for RFC 9420 HPKE; no permissive substitute. Compliance call, not code. | [decision] | `ROADMAP.md:102` | `AGENTS.md:56` |
| A2 | **Total-device-loss recovery anchor** — mnemonic seed vs social-recovery quorum vs broker-held encrypted backup. Largest residual risk; none chosen. | [decision] | `ROADMAP.md:109` | `thinking/open-edges.md:83`, `thinking/open-considerations.md:28`, `COHESION.md:212`, `AGENTS.md:64` |
| A3 | **S3 — quiet membership** (be in a group without exposing other edges). | [decision] | `thinking/open-edges.md:89` | `ROADMAP.md:63` |
| A4 | **S4 — multi-identity, no forced linkage** (distinct lineages, no provable correlation). | [decision] | `thinking/open-edges.md:91` | `ROADMAP.md:67` |
| A5 | **T8 / V3 republish UX control** — structural V3 done; the human-layer "can still quote" control unspecified. | [decision] | `thinking/open-edges.md:93` | `ROADMAP.md:61`, `ROADMAP.md:107` |
| A6 | **Vote-accumulation under churn/partition** — vote expiry, retraction, stale-vote handling. | [decision] | `thinking/open-edges.md:70` | — |
| A7 | **Pin remaining sub-product name map** (top-level "Croft" pinned; sub-products open). | [decision] | `ROADMAP.md:121` | `NAMING.md`; app brand draft `thinking/app/brand-and-voice-notes.md`, `NAMING.md:108` |
| A8 | **Import Croft-app Phase 0 (CroftC PR #10) → `experiments/` + the IP/ownership decision** — ✅ **IMPORT DONE 2026-06-22** (`experiments/croft-app-phase0/`, byte-identical), decision **exercised** by the user directing the import; paper trail in `experiments/croft-app-phase0/PR-CONVERSATION.md`. **Residual (user's):** the CroftC-side PR #10 is still **OPEN** — decide whether to close/annotate it on the CroftC side to complete the clean paper trail. | [decision] | `ROADMAP.md:157` | `COHESION.md` §23, `experiments/croft-app-phase0/PR-CONVERSATION.md` |
| A9 | **Anchor-URI stability contract** — the cross-platform provenance anchor in `alsoKnownAs` must be a *stable logical URI* (domain/path committed to staying resolvable/redirecting) with *mutable content*; pointing at a content-addressed/immutable thing re-freezes what portability is meant to avoid. Deliberate choice, not default. | [decision] | `thinking/cross-platform-identity-provenance.md:213` | `COHESION.md` §21 |
| A10 | **PDS-held vs self-controlled did:plc rotation key** — changes who can issue future operations and whether the did:webvh root genuinely functions as a recovery anchor for the Bluesky spoke. Staging advice exists; the keep-vs-drop-PDS-key call is the user's. Relates A2. | [decision] | `thinking/cross-platform-identity-provenance.md:218` | A2, `COHESION.md` §21 |

## B. Validation / spikes

| # | Item | Type | Origin (file:line) | Also tracked |
|---|---|---|---|---|
| B1 | **Run deferred live-bsky validation** — built, waiting on egress allowlist + app-password env; flips H2/H3/H5 to live-network. | [resource] | `ROADMAP.md:126` | `thinking/open-edges.md:122` (T10) |
| B2 | **Verify openmls leaf-credential dependency** (multi-device 8.1) — so lineage-fold/thresholds rest on real library behavior. | [doable] | `ROADMAP.md:112` | `thinking/open-edges.md:137`, `COHESION.md:137` |
| B3 | **Run Achilles-heel adversarial research** (seed prompt, unrun) — "is the superpeer secretly the ordering authority?" | [doable] | `ROADMAP.md:115` | `thinking/open-considerations.md:50` |
| B4 | **Add hashing-tree / Merkle trust-proof** (offline transitive trust) to Proofs. | [doable] | `ROADMAP.md:118` | `ROADMAP.md:68` |
| B5 | **Relay capacity ceilings** — real absolute throughput (generators understated it). | [doable] | `thinking/open-edges.md:58` | `croft-relay-lab` memory |
| B6 | **E6 steady-state goodput under shaping + bandwidth cap.** | [doable] | `thinking/open-edges.md:61` | — |
| B7 | **E0-NAT hole-punch ingress** — public 3343/3478 currently closed. | [resource] | `thinking/open-edges.md:124` | — |
| B8 | **E4 — LVS frontend** (`ipvsadm`); E1 suggests one fat process suffices. | [resource] | `thinking/open-edges.md:125` | — |
| B9 | **E8 / E9 — `meer` binary** (superpeer bridge + confidentiality tiers); **T13 — iOS build host**. | [resource] | `thinking/open-edges.md:126` | `thinking/open-edges.md:123` |
| B10 | **Background BLE-meshing feasibility spike (NEW)** — CoreBluetooth doesn't relaunch on new-advertiser discovery; Berty says bg P2P dies in seconds. Prove before any design leans on it. | [explore] | `thinking/ios-opportunistic-p2p.md:82` | `COHESION.md:17`, `...-FACTCHECK.md` |

## C. Backports / reconciliations (corpus hygiene)

| # | Item | Type | Origin (file:line) | Also tracked |
|---|---|---|---|---|
| C1 | **Backport social-layer §8–10** into `thinking/social-layer.md` (proof implemented regimes the doc's header promises). | [backport] | `ROADMAP.md:105` | `COHESION.md:52` |
| C2 | **Reconcile public-path duplication** — PR #3 vs PR #4/#6 overlap; pick canonical AppView home. | [backport] | `ROADMAP.md:133` | `COHESION.md:105` |
| C3 | **Record the sequencer honestly** — "minimal, blind, not a rights authority," not "no ordering authority." | [backport] | `ROADMAP.md:138` | `COHESION.md` §4 |
| C4 | **Reconcile peer-equality wording** — "equal in rights, not capabilities" (current) vs dossier's "ability/capacity." | [backport] | `COHESION.md:256` | `crystallized/principles.md` |
| C5 | **Distill the app-dialogue research into `research/` + `ECOSYSTEM.md`** — iroh-in-browser (relay-only browser peers), webxdc/Delta-Chat games + the WebRTC-transport-swap recipe, super-apps/W3C-MiniApp, atproto appview routing (service-proxy/service-auth + OpenMeet recipe), Rust client libs (ATrium/Jacquard/megalodon/lemmy-client-rs), Crux/FCIS. The `ECOSYSTEM.md` §5c rows are staged **dialogue-sourced, pending independent verification**. | [backport] | `ROADMAP.md:164` | `ECOSYSTEM.md` §5c, `thinking/app/README.md` "Follow-ons" |
| C6 | **Reconcile app brand naming into `NAMING.md`** once `brand-and-voice-notes.md` settles (Croft product vs umbrella; pond/pad; "Grow your own"). The app-level answer to A7. | [backport] | `NAMING.md:108` | `thinking/app/brand-and-voice-notes.md`, `COHESION.md:322` (DRIFT), A7 |

## D. Strategic considerations (unresolved, surfaced not solved)

| # | Item | Type | Origin (file:line) | Also tracked |
|---|---|---|---|---|
| D1 | **What is the actual product?** (the M0 wedge framing). | [decision] | `thinking/open-considerations.md:19` | `ROADMAP.md` M0 |
| D2 | **Possible over-application of Automerge.** | [explore] | `thinking/open-considerations.md:40` | — |
| D3 | **Moderation & abuse under a blind broker** is unaddressed. | [decision] | `thinking/open-considerations.md:60` | `COHESION.md:157` |
| D4 | **Encrypt-then-content-address kills cross-user dedup** — a real tradeoff to record. | [explore] | `thinking/open-considerations.md:80` | `COHESION.md` §13 |
| D5 | **Infra-sustainability ↔ the cooperative as a *mechanism*** (existential) — relays (browser peers permanently relayed), bridge node, scoped appview, push origination = ongoing metered cost; "cooperative" is so far a value, not yet a funding/governance mechanism. The most important unthought thing. | [decision] | `thinking/open-considerations.md:89` | `ROADMAP.md:172` (§15) + §8 charter, `governance-and-survivability.md` |
| D6 | **Moderation/safety vs the kid-friendly goal** — E2EE-P2P-relay carries traffic the operator can't read, in tension with courting gen-alpha/kid use; the public bridge is an abuse vector. The app-layer sharpening of D3. | [decision] | `thinking/open-considerations.md:103` | D3, `thinking/geer-gating-peer.md` |
| D7 | **Cold-start for the owned pond + "composability: user-want or builder-elegance?"** — aggregator ponds inherit populations; Croft Group has the empty-room problem (the games hook is the only answer, and games is a *candidate*); the default ignore-everything experience must be compelling alone. Relates D1. | [explore] | `thinking/open-considerations.md:113` | D1 |
| D8 | **The centerless-meets-center frontier** — where a centerless federation touches politics/finance/technology (legal entity to sue/bill, name registrar, the relay that must scale, the money) without quietly growing a center at the seam. The largest-clothes version of the irreversible-singleton problem; arranged to be deferrable/excisable, but **deferred, not solved**. Wikipedia met it at the Foundation, AP at the instance, atproto at the relay. | [decision] | `thinking/local-first-as-design-imperative.md` (Open frontiers) | D5 (infra-sustainability ↔ cooperative), `COHESION.md` §22 |
| D9 | **Governance at scale** — representative quorum vs. the cheap-fork sybil defense getting expensive at 200k; likely direction **subsidiarity + liquid delegation** (instantly-revocable weight), with concentration the default failure (Pirate Party lesson) resisted by decay/caps/bounded-chains/expiry/visibility. Member ≠ governance-constituent must be modeled. Surfaced not solved. | [decision] | `thinking/local-first-as-design-imperative.md` (Open frontiers) | `COHESION.md` §22, A2 |
| D10 | **Forward-only revocation under irreversible commitments** — revoking consent can't rewind a spent check; decisions must be tagged reversible-or-committing at decision time, and the record must permanently/honestly attribute which consent supported which irreversible consequence. The governance-plane face of D4/A2. | [decision] | `thinking/local-first-as-design-imperative.md` (Open frontiers) | D8, `COHESION.md` §22 |

## E. Adjacent / new explorations (this batch, 2026-06-22)

| # | Item | Type | Origin (file:line) | Also tracked |
|---|---|---|---|---|
| E1 | **AT-Proto "atmospheric web" product surface** (Neo-GeoCities / open-LinkedIn / private AppView). Demand-side argument for the lineage-groups MLS crypto. | [explore] | `thinking/atproto-atmospheric-web.md:1` | `COHESION.md:17`, `ECOSYSTEM.md` §5b |
| E2 | **Private groups for any atmospheric-web product = Croft's lineage-groups MLS proof** — no native AT-Proto E2EE (REFUTED claim); Germ/XMTP are third-party. | [explore] | `thinking/atproto-atmospheric-web.md:47` | `ECOSYSTEM.md` §6 (Germ) |
| E3 | **Gateway-renders-HTML XSS surface** — declaration-not-execution design for any web bridge. | [explore] | `thinking/atproto-atmospheric-web.md:76` | — |
| E4 | **Music-over-Iroh streaming** (iroh-blobs + QUIC migration) — Aster is a real precedent. | [explore] | `thinking/ios-opportunistic-p2p.md:64` | `thinking/realtime-media-over-iroh.md` |
| E5 | **The Croft app / client layer (new body, 2026-06-22)** — composable ponds/pads garden riding the proven substrate; stack decided (Rust functional core + Tauri/Leptos, web-first, Crux *pattern* slim); Phase 0 built, Phases 1-2 spec'd. | [explore] | `thinking/app/README.md:1` | `ROADMAP.md:146` (§12), `COHESION.md:322` (§18) |
| E6 | **Games pond** — P2P games over iroh, outcomes as a custom lexicon (rendered-where-desired), Bluesky as the rendezvous channel; the hook that teaches composability; outcome-attestation is the real design work (set aside). **Research prompt now RUN → produced the `ponds/` catalog (E8).** | [explore] | `thinking/app/design-philosophy.md:634` | `thinking/app/ponds/`, `COHESION.md` §19 |
| E7 | **OpenMeet read-only-meets candidate pond** — thinnest useful slice (read-only your-own-meets); pond-worthiness is a product assessment, distinct from crediting its auth technique. | [explore] | `thinking/app/design-philosophy.md:641` | `ECOSYSTEM.md` §5c |
| E8 | **Ponds & pads catalog + build-order (2026-06-22)** — games/utilities/ritual catalog by inclusion pathway (build-fresh / wrap-via-webxdc-shim / port-via-WebRTC-swap), ranked by fun + by "have you seen this?" energy; six-phase build sequencing. | [explore] | `thinking/app/ponds/build-order.md:1` | `thinking/app/ponds/` (games/apps/launch-set lists), `COHESION.md` §19 |
| E9 | **Fair-reveal (commit-reveal) primitive** — one shared module giving fairness with no server: powers secret-ballot voting + dice + Two-Truths-and-a-Lie. Build in Phase 1 so later games/voting just "call it." Watch: nonce for low-entropy values, the commit-then-abort reveal deadline. | [doable] | `thinking/app/ponds/fair-reveal-primitive-spec.md:1` | `thinking/app/ponds/build-order.md` (Phase 1), local real-time voting |
| E10 | **On-device-LLM navigation assistant** — natural-language front-end to the deep-link resolver ("any travel games?" → catalog links). **Great-to-have, NEVER a requirement** (hardware coverage; first targets Android+macOS, iOS deferred); strictly optional, easily disabled, complete-navigation-without-it. Phase 5 (can slip indefinitely). | [explore] | `thinking/app/ponds/on-device-llm-feasibility.md:1` | `thinking/app/ponds/build-order.md` (Phase 5) |
| E11 | **Relay dependency / deferred-deep-link reality (the two "serverless"/"frictionless" asterisks)** — browser peers are permanently relayed (self-host relays if it grows, `[resource]`); seamless cold-install deep-linking is not privately achievable (Instant Apps + Firebase Dynamic Links dead; MMPs need fingerprinting) → claim-code one-more-tap, framed as a feature. | [decision] | `thinking/app/ponds/build-order.md` (tier-zero resolver) | `open-considerations.md` §8, `ECOSYSTEM.md` §1 (iroh relays) |
| E12 | **Cross-platform identity provenance (new body, 2026-06-20)** — closed-root-of-trust → out-of-band attestation is the only cross-platform linkage; hub-and-spoke (did:webvh root, did:plc/AP/Hive spokes); key lineage = attestation not derivation; `alsoKnownAs` equivalence ladder; did:webvh portability (`portable` genesis-only); did:plc↔did:webvh convergence as cheap hedge (keep the SCID as anchor). Extends `plc-identity-resilience.md` on a new axis. | [explore] | `thinking/cross-platform-identity-provenance.md:1` | `plc-identity-resilience.md`, `COHESION.md` §21, A9/A10 |
| E13 | **Per-platform trust-model doc (next artifact)** — the dialogue's repeatedly-offered, highest-leverage write-up: per network (Bluesky/AP/Mastodon/GoToSocial/Threads/Hive), the field used, what we claim / don't claim, the backlink mechanism, and exact verifier steps + pseudocode. Not yet written. | [doable] | `thinking/cross-platform-identity-provenance.md:222` | E12 |
| E14 | **Test `alsoKnownAs` extra-entry persistence** `[UNVERIFIED]` — confirm atproto PDS/PLC tooling preserves a non-`at://` provenance entry on write (and which AP servers tolerate a non-actor URI). Pure implementation check, not spec. | [explore] | `thinking/cross-platform-identity-provenance.md:159` | E12, B1 (live-bsky) |
| E15 | **Design-imperative / system-architecture body (new, 2026-06-20)** — the deep "why" (cross-field lineage) + the protocol-substrate architecture (delegates, planes, governance-as-substrate, federation, local-first-as-premise). Grounds the existing principles in a 2,400-year lineage; resolves the Kleppmann reconvergence tension (per-plane policy). | [explore] | `thinking/local-first-as-design-imperative.md:1` | `narrative/lineage-of-a-design-imperative.md`, `crystallized/principles.md`, `COHESION.md` §22 |
| E16 | **Federation / inter-collective peering PoC** — BGP-autonomy + postal-hierarchy + DID-signed routing; identity-vs-locator; recursive-worker resolution over atproto with DNS as a swappable resolver. Show bounded per-node state + log-depth resolution at small N (4 collectives) to demonstrate the scaling math. | [explore] | `thinking/local-first-as-design-imperative.md` (Federation) | D8, RINA/NDN/Yggdrasil prior art |
| E17 | **Three new threat-model seams (write each up before code)** — (1) MLS epoch keying × fork × offline pre-fork grant-holder; (2) the delegate courier-vs-agent unification (does C2/live-authority break the abstraction); (3) content-predicate search-coverage attestation (honest plaintext evaluation, not just structural coverage). | [doable] | `thinking/local-first-as-design-imperative.md` (What's new) | `COHESION.md` §22 |
| E18 | **Crypto-shred + moderation hold/release plane** — erasure = disassociate + purge group-held copies + honest-limit (can't reach independent copies); the chat plane needs crypto-shred from day one; holds = a `{pending,released,rejected}` predicate-gated moderation plane (the "two convergent groups, one a queue"). Willow-style prunable store for stable content. | [doable] | `thinking/local-first-as-design-imperative.md` (substrate / planes) | D6 (moderation), `app/ponds/` |
| B11 | **Finish Croft-app Phase 0 M6 — live Jacquard adapter** — wire the real atproto Rust client to live Bluesky behind the port (DECISION 1); M1–M5 done + real fixtures present, M6 deferred. Lights up the live happy path. | [doable] | `experiments/croft-app-phase0/PR-CONVERSATION.md` (Milestones) | `appview-validation` (live-bsky), B1 |
| C7 | **Reconcile as-built Phase-0 spec → thinking/app** — the PR's `BUILD-SPEC.md`/`design-philosophy.md` are what the code was written to; `thinking/app/` is more-developed (§3a proof, §1a, §4a). On graduation, reconcile, and address CodeRabbit doc nits ("written-down shortcut" undefined; DECISION-5 burden on the CLI fake). | [backport] | `COHESION.md` §23 | `experiments/croft-app-phase0/`, `thinking/app/` |
| E19 | **Adapt croft-chat-cli to the shared-core / per-platform-shell model** — ✅ **architecture DECIDED 2026-06-22** (`thinking/app/client-architecture-adr.md`): one shared functional core + thin per-platform shells, two callout axes (platform `effects.rs` + implementation adapters behind a port). `croft-chat-cli` already has the implementation seam (`Transport` port + in-proc fake) but **not** the core/shell — so adoption is greenfield growth, not a refactor. Gap analyzed; **plan not yet drafted** (user's next-step call). Decomposition **RESOLVED (option C)**: per-pond domain cores unified by the shared `shell` composition layer (group-core + Transport port symmetric to feed-core + Bluesky port); cross-pond awareness = read-only shell composition, interactivity = a deferred broker. Also close the `iroh/`-not-in-`experiments/README` gap. | [doable] | `thinking/app/client-architecture-adr.md` | `experiments/iroh/crates/croft-chat-cli` (+ memory), `COHESION.md` §23, `crystallized/principles.md` Tier 3 |

---

## Maintenance

When an item closes, mark it here and in its origin doc, and add the COHESION row if it closed a
seam. When a new open edge is surfaced (in a transcript, proof, or review), add a row with its
origin `file:line` so this stays the single place to scan "what's up for consideration." Re-anchor
line numbers opportunistically — section headers are the durable key.
