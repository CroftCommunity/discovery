# Drystone delivery layer: references

`Status: reference set for the delivery-layer design round`

`Companion to: 00-session-summary.md, 01-delivery-architecture.md`

---

## How to read this list

Each entry gives the source, what it grounds in the design, and a verification status.

- *Verified*: retrieved and checked against this source this round.

- **[confirm]**: load-bearing and to be re-confirmed against the primary before normative text, for the reason noted.

Source priority, per project discipline: standards bodies and specifications first, then peer-reviewed or arXiv papers, then official project docs and source repositories. Blogs and secondary commentary are corroboration only and are marked as such. Where the design leans on a secondary source, that is stated.

---

## Standards and specifications (the spine)

### MLS

- **RFC 9420, "The Messaging Layer Security (MLS) Protocol."** IETF, July 2023. Grounds: PrivateMessage end-to-end protection to the group epoch with forward secrecy and post-compromise security, independent of transport; the propose-then-commit model; credential validation at the Authentication Service, member-side, at Add/Update/external-join time (§5.3.1); LeafNode and ratchet-tree validation. *Verified (retrieved datatracker and rfc-editor copies this round).* Primary URLs: `https://datatracker.ietf.org/doc/rfc9420/`, `https://www.rfc-editor.org/rfc/rfc9420.pdf`.

- **RFC 9750, "The Messaging Layer Security (MLS) Architecture."** IETF. Grounds: the trusted-AS / untrusted-DS split (a DS may drop, delay, observe, but never forge or decrypt); the DS store-and-forward role Drystone keeps while refusing the ordering role; the basic unit of operation is the "client," not the user; and, load-bearing for the device pool, the explicit naming of application policy for "when a member is allowed to add or remove other members" and "when two credentials represent the same client ... when there are multiple devices for a given user." *Verified (retrieved ietf.org and rfc-editor copies this round).* Primary URLs: `https://www.ietf.org/rfc/rfc9750.html`, `https://www.rfc-editor.org/rfc/rfc9750.pdf`.

- **"The Messaging Layer Security (MLS) Extensions" (draft-ietf-mls-extensions).** IETF draft. Grounds: AppAck as a mechanism for clients to detect dropped application messages via generation-sequence gaps, a candidate primitive for the gap-visibility requirement. **[confirm: this is a draft, not core RFC 9420; lower stability, moving target.]** URL: `https://messaginglayersecurity.rocks/mls-extensions/draft-ietf-mls-extensions.html`.

### Foundational prior art (epidemic dissemination and anti-entropy)

- **Demers et al., "Epidemic Algorithms for Replicated Database Maintenance." Xerox PARC, ACM PODC 1987.** The root of the delivery layer's lineage. Grounds: rumor mongering (probabilistic spread, fast, incomplete: ancestor of gossip/C-swarm), anti-entropy (random-peer content exchange with checksum-until-agreement: ancestor of RBSR/gap-aware convergence), the documented pairing of the two (cheap incomplete gossip plus reliable anti-entropy, re-injecting found discrepancies as hot rumors), the epidemiology vocabulary (infective/susceptible/removed) the carrier metaphor lives in, and death certificates for propagating deletion (ancestor of Willow tombstones). *Verified (retrieved lecture-notes and summaries of the PODC '87 paper this round; the paper itself is the primary and should be cited directly in publication).* Representative copies: `https://courses.grainger.illinois.edu/cs525/sp2016/EpidemicReplicated.pdf`. **[confirm: cite the canonical PODC 1987 paper directly, not lecture summaries, in any published version.]**

- **HyParView (Leitao, Pereira, Rodrigues, DSN 2007) and PlumTree ("Epidemic Broadcast Trees," Leitao et al., SRDS 2007).** Grounds: the membership (active/passive view) and eager/lazy broadcast-tree refinement of rumor mongering that iroh-gossip implements. Cited via the iroh-gossip crate docs this round; the papers themselves are carried from prior rounds and should be cited directly in publication.

- **Hyperledger Fabric, gossip data dissemination protocol (project docs).** Closest whole-system prior art for the routing-fabric-larger-than-entitlement-group decoupling. Grounds: signed gossip dissemination with Byzantine exclusion; state reconciliation (pull missing blocks from peers that hold them); channels as the entitlement boundary with anchor peers spanning a larger cross-org overlay. Drystone differs in removing the central ordering service and using cryptographic (MLS) rather than policy entitlement, and CA-free cooperative identity. *Verified (retrieved the Fabric gossip docs this round).* URL: `https://hyperledger-fabric.readthedocs.io/en/latest/gossip.html`.

### Set reconciliation

- **Meyer, A. "Range-Based Set Reconciliation." arXiv:2212.13567 (2023); also SRDS 2023.** Grounds: the RBSR primitive (recursive partitioning, fingerprint comparison, descend on mismatch); the complexity statement used in the design (logarithmic rounds, communication within a logarithmic factor of optimal); termination and correctness by induction. *Verified (retrieved abstract and PDF this round).* URLs: `https://arxiv.org/abs/2212.13567`, `https://arxiv.org/pdf/2212.13567`.

- **Willow Protocol, "3d Range-Based Set Reconciliation" (RBSR spec).** Grounds: the worked mechanics of fingerprint-then-split-or-send, the logarithmic-rounds-low-bandwidth payoff, and a candidate construction for Drystone's device sync (Willow is already in the Part 1 data-layer lineage). *Verified (retrieved this round).* URL: `https://willowprotocol.org/specs/rbsr/index.html`.

- **Willow Protocol, "Data Model" spec.** Grounds (for the history-modes doc): path-addressed entries; newer-entry-overwrites-older semantics; **prefix pruning** as the deletion mechanism (writing at a prefix deletes prefixed entries, like overwriting a directory); deletes remove payload and metadata except a retained tombstone; writes and deletes gated by Meadowcap capabilities. *Verified (retrieved this round).* URL: `https://willowprotocol.org/specs/data-model/index.html`. Corroborating (secondary): the Willow project overview pages and the earthstar-project/willow-rs and willow-js implementations, which confirm destructive edits and tombstones are shipping concerns, not theoretical. *Secondary, corroboration only.*

- **Negentropy (hoytech), protocol description and implementations.** Grounds: that the RBSR ordering criterion may be any monotonic value fitting a 64-bit integer (timestamps are merely the obvious example), which is what makes RBSR compatible with the timestamp-free spine; incremental-hash fingerprints; deployed status. *Verified (retrieved GitHub this round).* URL: `https://github.com/hoytech/negentropy`. Corroborating implementation notes (secondary): Log Periodic, "Range-Based Set Reconciliation," `https://logperiodic.com/rbsr.html` (tree backend, interoperability across branching factors). *Secondary, corroboration only.*

- **"Range-Based Set Reconciliation via Range-Summarizable Order-Statistics Stores." arXiv:2603.19820 (2026).** Grounds: the storage-backend obligation (efficiency depends on a backend that can summarize an arbitrary range, split by cardinality, and enumerate residuals without rescanning); confirmation that practical RBSR via Negentropy is in use (Nostr NIP-77, strfry). *Verified (retrieved HTML and PDF this round).* URL: `https://arxiv.org/html/2603.19820`.

---

## Official project documentation and source (iroh)

- **iroh, "Iroh 1.0 - Dial Keys, not IPs" (n0 release post).** Grounds: 1.0 as the first major release; wire-stability tied to major versions; the 0.35 public-relay support window through 2026-12-31. *Verified (n0 primary, retrieved this round).* URL: `https://www.iroh.computer/blog/v1`.

- **iroh FAQ (n0 primary).** Grounds the relay's metadata exposure precisely: the relay is "just another UDP socket" for encrypted packets; it can know that one EndpointId talks to another and the byte volume, but only until the endpoints hole-punch a direct connection, after which it is out of the path; n0 states it does not record this. Used in §1.1 to separate the relay (iroh-layer, EndpointId-pair, transient) from a gateway (IP-layer, ephemeral-IP, identity-blind). *Verified (retrieved this round).* URL: `https://www.iroh.computer/docs/faq`.

- **iroh, "Dial by NodeID" / iroh-dns (n0 primary).** Grounds the discovery surface: a Pkarr signed packet carries the Node ID and home relay URL, resolvable by anyone who already knows the Node ID, published by default to an n0-hosted iroh-dns server. Used in §1.1 to name discovery as a distinct, opt-out-able metadata surface. *Verified (retrieved this round).* URL: `https://www.iroh.computer/blog/iroh-dns`.

- **iroh, release-candidate posts (1.0.0-rc.0, 1.0.0-rc.1).** Grounds: the core API surface being stabilized (Endpoint, Connection, Router, ALPN, QUIC/noq); the splitting-out of address-lookup crates into separate repos/versioning; relay AccessControl trait redesign; that custom-transport traits are gated behind an unstable feature and not covered by 1.0 guarantees. *Verified (retrieved this round).* URLs: `https://www.iroh.computer/blog/iroh-1-0-0-rc-0`, `https://www.iroh.computer/blog/iroh-1-0-0-rc-1`.

- **iroh, "Gossip Broadcast" docs.** Grounds: TopicId model; topic-as-swarm-and-broadcast-scope; bootstrap-peer join; one-topic-for-all vs scoped-topics guidance. *Verified (retrieved this round).* URL: `https://docs.iroh.computer/connecting/gossip`.

- **iroh-gossip, crate docs (docs.rs) and proto module.** Grounds: HyParView (active view 5, passive view 30) plus PlumTree (eager/lazy sets); the internal IHave mechanism (lazy push of message hash). *Verified (retrieved docs.rs proto module).* Version note: crawled at 0.100.0 (pinning `iroh =1.0.0-rc.1`) during design, then **confirmed by experiment E0.1 at 0.101.0 building against stable iroh 1.0.1**. Critical correction from experiment E1.1: the IHave is **internal only** and is **not** surfaced on the application `api::Event` enum (`NeighborUp` / `NeighborDown` / `Received` / `Lagged`), so the earlier "a subscribed node observes presence via Ihave" reading was wrong; see design doc §2.2, §4. URLs: `https://docs.rs/iroh-gossip/latest/iroh_gossip/proto/index.html`, `https://github.com/n0-computer/iroh-gossip`.

- **iroh, GitHub releases.** Grounds: relay breaking-changes context (AccessControl trait, ConnectionId), corroborating the rc-1 post. *Verified (retrieved this round).* URL: `https://github.com/n0-computer/iroh/releases`.

---

## Official platform documentation (push)

- **Apple, "Creating the Remote Notification Payload" (Local and Remote Notification Programming Guide).** Grounds: 4 KB payload cap for standard remote notifications (5 KB VoIP); that background update notifications are low priority and APNs may throttle them altogether, with a guideline of no more than a few per hour, dynamic; the warning never to include sensitive data in a payload. *Verified (Apple primary, retrieved this round).* URL: `https://developer.apple.com/library/archive/documentation/NetworkingInternet/Conceptual/RemoteNotificationsPG/CreatingtheNotificationPayload.html`.

- **Firebase Cloud Messaging, "Message types" (Google).** Grounds: 4096-byte payload cap for both notification and data messages; the explicit statement that the FCM connection is not end-to-end encrypted and applications must supply their own E2E for sensitive data; notification-vs-data handling in background vs foreground. *Verified (Firebase primary, retrieved this round; page dated 2026-06-22).* URL: `https://firebase.google.com/docs/cloud-messaging/customize-messages/set-message-type`.

---

## Secondary and corroborating sources (not load-bearing)

These corroborate or illustrate; none is relied on alone, and any conflict with a primary is resolved in the primary's favor.

- **APNs silent-push throttling (secondary, conflicting numbers).** Bugfender, "Advanced iOS push notifications" (`https://bugfender.com/blog/advanced-ios-push-notifications/`): iOS dynamically throttles and may skip background execution even after APNs accepts. TechConcepts, "iOS Push Notifications with APNs" (`https://techconcepts.org/blog/ios-push-notifications`): "2 to 3 deliveries per hour per device." Pushy support (`https://support.pushy.me/...`): "about 5 per day." Newly, "iOS Push Notifications ... (2026)" (`https://newly.app/guides/ios-push-notifications`): `content-available: 1` semantics, aggressive rate-limiting. **[confirm: numbers conflict; only Apple's "a few per hour, dynamic" is treated as pinnable.]**

- **iroh 1.0 launch coverage (secondary).** TechTimes, Pinggy, AlternativeTo, Founderland, byteiota, StackRadar. Corroborate the 2026-06-15 date, ~90% hole-punch, relay-sees-identifiers-not-content, and the gossip-is-separate framing. byteiota notes a relay-sunset date for pre-1.0 of 2026-09-30, which conflicts with the n0 primary's 2026-12-31. **[confirm: trust the n0 primary on the sunset date; secondaries disagree.]**

- **MLS implementation (corroboration).** awslabs/mls-rs (`https://github.com/awslabs/mls-rs`): an RFC 9420 implementation; corroborates propose-then-commit, offline Add via pre-computed key packages, configurable storage, subgroup branching. Useful as an existence proof that the policies the design relies on are implementable; not a specification.

---

## Sources carried from prior rounds (not re-retrieved this session)

These ground claims in the design that originate in Part 1 or earlier substrate work and were not re-pulled this round. Listed for completeness; their own verification status lives in Part 1's reference legend.

- iroh-base 1.0 (endpoint key curve): **[confirm]** not re-checked this round.

- The HyParView paper (Leitao et al., DSN 2007) and the PlumTree paper (Leitao et al., "Epidemic Broadcast Trees," SRDS 2007): cited via the iroh-gossip crate docs, which name both; the primaries themselves were not re-retrieved this round.

- Drystone Part 1 (`drystone-part1.md`): the source of P-Local-Truth, P-Knowable-Truth, P-Peer-Equality, P-Durable-Enablement, the §2.3 resource/role/right/weight model, the §2.0.1 time-is-not-a-fact corollary, the §2.5 fork-not-verdict terminus, and the §2.6 field-integrity dependency. Read this round; its own `[confirm]` items are tracked in its legend, not duplicated here.
