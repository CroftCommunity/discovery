# Reference index: "the impl layer, and what it is grounded against"

Every external source cited across the impl layer, grouped by type, with a resolvable
locator, the epistemic-status tag, a primary/secondary marker, and which impl document
relies on it. This is the per-layer source-of-record; the delivery-layer keeps its own
working references appendix at `delivery-layer/02-references.md`, and this index
subsumes it and reaches the rest of the layer (the drystone-design set, the MLS set, the
experiments set, and the top-level notes).

Marker key:

PRIMARY = the artifact itself (the RFC, the protocol specification, the paper, the crate's
own documentation or source, the platform vendor's own documentation).

PRIMARY-AS-CITED = a primary named and leaned on through a secondary that reproduces it
(a foundational paper cited via a crate's docs, not re-pulled to the paper itself).

SECONDARY = reporting, lecture notes, blog, or summary standing in for an underlying
primary not retrieved, used for corroboration only.

Epistemic-status tags (aligned to the layer's own status ladder, condensed for a source list):

- `normative-standard` — a published IETF RFC, checked against primary text this cycle.
- `ietf-draft` — an unstable IETF draft or pre-print draft; a moving target, lower stability.
- `specification` — a non-RFC protocol specification (Willow family, ATProto, Negentropy).
- `peer-reviewed` — a published peer-reviewed paper or its venue proceedings.
- `preprint` — an arXiv or IACR-eprint paper, not (or not yet) relied on as peer-reviewed.
- `measured-study` — a single empirical measurement, an observation under conditions, not a constant.
- `project-doc` / `crate-doc` — a library's or project's own documentation or source repository.
- `platform-doc` — a platform vendor's own developer documentation.
- `prior-art` — a shipping system cited as comparative or corroborating evidence.
- `secondary` — reporting / lecture notes / blog; corroboration only, never load-bearing alone.
- `empirical-proof` — a prototype run on a real library in the sibling `Proofs/` repo, honest to its own stated scope boundaries; discharges a specific impl claim.

Version-fact discipline: iroh and iroh-family version facts cite the FACTCHECK source of
truth and are not re-verified here. Pinned crate versions that are volatile are flagged
`[volatile]`.

---

## RFCs and specifications

### MLS (the cryptographic spine)

- **RFC 9420, "The Messaging Layer Security (MLS) Protocol."** IETF, July 2023. Locators used
  across the layer: §2 (terminology: client, member, group, epoch, leaf), §5.3 / §5.3.1
  (credential validation at the AS), §6.3.1 (reuse_guard / nonce), §7 (ratchet tree, LeafNode),
  §8 (key schedule), §8.2 (transcript hash), §8.6 (resumption PSK), §9 / §9.2 (secret tree,
  deletion schedule), §10 / §16.8 (KeyPackages, last-resort), §11.2 / §11.3 (ReInit, branching),
  §12 (proposals and commits), §12.1.6 (GroupInfo / external commit), §16.6 (SHOULD remove
  non-updating members), §16.7 (reuse). `normative-standard`. PRIMARY. URLs:
  `https://datatracker.ietf.org/doc/rfc9420/`, `https://www.rfc-editor.org/rfc/rfc9420.pdf`.
  Relied on by: the whole `mls/` set (`mls-overview-and-terms.md`, `mls-hardcases-and-posture.md`,
  `mls-session-summary.md`), `drystone-design/history-durability.md`, `drystone-design/scaling-and-ordering.md`,
  `drystone-design/governance-finality.md`, `drystone-design/asset-keying.md`,
  `delivery-layer/02-references.md`, `delivery-layer/07-history-modes.md`, `the-four-property-tension.md`.

- **RFC 9750, "The Messaging Layer Security (MLS) Architecture."** IETF, April 2025. Locators used:
  §3.7 (the client, not the user, is the basic unit), §4 / §5 (trusted AS, untrusted DS split), §6.1 /
  §6.6 / §6.7 (ReInit non-atomicity, state-loss rejoin, several-clients), §6.3 / §6.4 (no application
  access control), §7 (group creation flow, KeyPackage-at-plant-time), §8 security catalogue: §8.1.2
  (epoch-number metadata leak), §8.1.4 (invalid-commit / stale-GroupInfo hazard), §8.3 (inactive
  members), §8.6 (insider replay). `normative-standard`. PRIMARY. URLs:
  `https://www.ietf.org/rfc/rfc9750.html`, `https://www.rfc-editor.org/rfc/rfc9750.pdf`.
  Relied on by: the `mls/` set, `drystone-design/history-durability.md`, `drystone-design/scaling-and-ordering.md`,
  `delivery-layer/02-references.md`, `the-four-property-tension.md`.

- **"The Messaging Layer Security (MLS) Extensions" (draft-ietf-mls-extensions).** IETF draft.
  Grounds AppAck (detect dropped application messages via generation-sequence gaps), a candidate
  primitive for the gap-visibility requirement. `ietf-draft`. PRIMARY. **[confirm: a draft, not core
  RFC 9420; lower stability, moving target.]** URL:
  `https://messaginglayersecurity.rocks/mls-extensions/draft-ietf-mls-extensions.html`.
  Relied on by: `delivery-layer/02-references.md`, `mls/mls-overview-and-terms.md`.

- **DMLS (draft-kohbrok-mls-decentralized-mls-00, March 2025).** Modifies the MLS key schedule
  (puncturable-PRF-derived init secrets, content-derived epoch identifiers) to reduce forward-secrecy
  loss under key retention. `ietf-draft`. PRIMARY (verified against the draft). Relied on by:
  `mls/mls-hardcases-and-posture.md` (§8), `mls/mls-session-summary.md`; also named in
  `the-four-property-tension.md`.

- **draft-xue-distributed-mls.** Serverless-ordered MLS proposal, named as an existence-of-drafts
  point (drafts / proof-of-concept only as of mid-2026). `ietf-draft`. PRIMARY-AS-CITED.
  **[confirm: deployment-status claim, check against the draft and the IETF MLS production-user list.]**
  Relied on by: `the-four-property-tension.md`.

### Transport (QUIC)

- **RFC 9000, "QUIC: A UDP-Based Multiplexed and Secure Transport."** Locators: §8.2 (PATH_CHALLENGE /
  PATH_RESPONSE path validation), §10.1 (idle timeout), §19.2 (PING frames). `normative-standard`.
  PRIMARY. Relied on by: `drystone-design/liveness-freshness.md`. (The transport-liveness properties
  in `transport-iroh-gossip-and-quic.md` rest on the same QUIC mechanics, cited there through iroh.)

- **RFC 9308, "Applicability of the QUIC Transport Protocol."** §3, on path-liveness signalling.
  `normative-standard`. PRIMARY. Relied on by: `drystone-design/liveness-freshness.md`.

### Document / RFC-format discipline

- **RFC 7990, "RFC Format Framework."** Grounds the archival rule that any RFC carrying an SVG diagram
  must also carry an ASCII-art or text equivalent (the diagram discipline). `normative-standard`. PRIMARY.
  Relied on by: `doc-writing-method.md`.

- **BCP 14 (RFC 2119 / RFC 8174), normative keywords MUST / SHOULD / MAY.** Used as the keyword
  convention across the normative design docs. `normative-standard`. PRIMARY. Relied on by:
  `drystone-design/history-durability.md`, `drystone-design/asset-keying.md`,
  `drystone-design/fold-semantics.md`, `drystone-design/governance-finality.md`,
  `drystone-design/social-mapping.md`.

### Data-layer and reconciliation specifications

- **Willow Protocol — "Data Model" specification.** Path-addressed entries; newer-overwrites-older;
  prefix pruning as the deletion mechanism; tombstone retention; Meadowcap-gated writes and deletes;
  the Entry-carries-path fact; state-based-CRDT-under-join semantics; the pluggable AuthorisationToken /
  is_authorised_write parameters (Meadowcap is the default, not a requirement); subspace-id hashing
  guidance. `specification`. PRIMARY. URL: `https://willowprotocol.org/specs/data-model/index.html`.
  Relied on by: `delivery-layer/07-history-modes.md`, `drystone-design/history-durability.md`,
  `drystone-design/asset-keying.md`, `drystone-design/social-mapping.md`, `delivery-layer/02-references.md`.

- **Willow Protocol — "3d Range-Based Set Reconciliation" (RBSR) specification.** The
  fingerprint-then-split-or-send mechanics; logarithmic-rounds / low-bandwidth payoff; a candidate
  construction for device sync. `specification`. PRIMARY. URL:
  `https://willowprotocol.org/specs/rbsr/index.html`. Relied on by: `delivery-layer/02-references.md`,
  `delivery-layer/07-history-modes.md`, `drystone-design/history-durability.md`.

- **Willow Protocol — "Meadowcap" capability specification.** The capability system gating reads and
  writes; the decisive open question is whether communal read-capability issuance composes beneath
  fold-gated asset-key wrapping without a second authority path. `specification`. PRIMARY.
  **[confirm against `willowprotocol.org/specs/meadowcap` — the composition check is unverified.]**
  Relied on by: `drystone-design/asset-keying.md` (§D, §F), `delivery-layer/07-history-modes.md`,
  `drystone-design/history-durability.md`.

- **Negentropy (hoytech) — protocol description and implementations.** Grounds that the RBSR ordering
  criterion may be any monotonic 64-bit value (making RBSR compatible with the timestamp-free spine),
  incremental-hash fingerprints, and deployed status (Nostr NIP-77, strfry). `specification` /
  `project-doc`. PRIMARY. URL: `https://github.com/hoytech/negentropy`. Relied on by:
  `delivery-layer/02-references.md`.

- **ATProto specifications (`atproto.com/specs`).** Grounds the repo equivalence: an ATProto repository
  is a single-author signed Merkle structure mapping collection/record-key paths to content hashes, and
  a relay firehose is a union of per-author repositories; the substrate is last-writer-wins with no
  enforced chain. `specification`. PRIMARY (verified in an external research pass, not re-fetched this
  cycle). Relied on by: `drystone-design/social-mapping.md`.

- **Nostr NIP-77 and strfry.** Named as deployed RBSR-via-Negentropy evidence. `specification` /
  `prior-art`. PRIMARY-AS-CITED (via arXiv:2603.19820). Relied on by: `delivery-layer/02-references.md`.

### Prior-art protocol references (specifications / project docs)

- **Hyperledger Fabric — gossip data-dissemination protocol (project docs).** Closest whole-system
  prior art for the routing-fabric-larger-than-entitlement-group decoupling: signed gossip with
  Byzantine exclusion, state reconciliation, channels as the entitlement boundary with anchor peers.
  `project-doc` / `prior-art`. PRIMARY. URL:
  `https://hyperledger-fabric.readthedocs.io/en/latest/gossip.html`. Relied on by:
  `delivery-layer/02-references.md`, `delivery-layer/09-provenance.md`, `delivery-layer/04-pitch-technical.md`.

### Security-advisory references

- **CVE-2023-49295 (QUIC PATH_CHALLENGE / PATH_RESPONSE memory exhaustion, CVSS 7.5).** Unbounded
  queuing exhausts memory; fixed by capping queued PATH_RESPONSE frames at 256 (~4 kB/connection).
  `normative-standard` (advisory). PRIMARY. Relied on by: `drystone-design/liveness-freshness.md`.

- **CVE-2025-49090 (Matrix) with MSC4289 / MSC4291 / MSC4297.** Cited as a verified-against comparative
  security reference for the reviews/experiments log. `normative-standard` (advisory + Matrix spec
  change proposals). PRIMARY. Relied on by: `experiments/drystone-reviews-and-experiments-log.md`.

---

## Rust crates and libraries

- **iroh (core).** Endpoint, Connection, Router, QUIC over TLS 1.3, relay, key-based (Ed25519)
  addressing; wire-and-API-stable at **1.0.0**, experiments built against **1.0.1**. `crate-doc`.
  PRIMARY. Version facts cite the FACTCHECK source of truth (do not re-verify). Relied on by:
  `transport-iroh-gossip-and-quic.md`, `ios-background-execution-and-the-ble-caution.md`,
  the `mls/` set, `delivery-layer/02-references.md`, `delivery-layer/04-pitch-technical.md`,
  `delivery-layer/00-session-summary.md`, `drystone-design/liveness-freshness.md`.

- **iroh-gossip.** HyParView (active view 5, passive view 30) plus PlumTree (eager/lazy sets); the
  IHave is internal only and not surfaced on the `api::Event` enum (`NeighborUp` / `NeighborDown` /
  `Received` / `Lagged`). Separately versioned and **pre-1.0**: crawled at 0.100.0 during design,
  confirmed by experiment at **0.101.0** against stable iroh 1.0.1. `crate-doc`. PRIMARY. **[volatile:
  pre-1.0, event-enum spelling not frozen; confirm against the pinned release. FACTCHECK source of
  truth for the version facts.]** URLs:
  `https://docs.rs/iroh-gossip/latest/iroh_gossip/proto/index.html`, `https://github.com/n0-computer/iroh-gossip`.
  Relied on by: `transport-iroh-gossip-and-quic.md`, `delivery-layer/02-references.md`,
  `drystone-design/liveness-freshness.md`.

- **iroh-blobs.** Content-addressed blob store; large payloads live here by hash, never in redb.
  `crate-doc`. PRIMARY. Relied on by: `drystone-design/redb-storage-contract.md`.

- **iroh-dns / Pkarr signed packets.** Discovery surface: a Pkarr signed packet carries the Node ID and
  home relay URL, published by default to an n0-hosted iroh-dns server (default TTL 7200 s). `crate-doc` /
  `project-doc`. PRIMARY. URL: `https://www.iroh.computer/blog/iroh-dns`. Relied on by:
  `delivery-layer/02-references.md`, `drystone-design/liveness-freshness.md`.

- **iroh-base 1.0.** Endpoint key curve. `crate-doc`. PRIMARY. **[confirm: not re-checked this round;
  version facts cite the FACTCHECK source of truth.]** Relied on by: `delivery-layer/02-references.md`.

- **iroh-ffi (first-party Swift bindings).** Shipped with iroh core 1.0 (mid-2026); the supported
  Rust-core / Swift-shell bridge. `crate-doc`. PRIMARY. Version facts cite the FACTCHECK source of
  truth. Relied on by: `ios-background-execution-and-the-ble-caution.md`.

- **iroh-ble-transport / `blew` (community BLE transport crate).** Not part of core iroh; its one public
  demonstration is unencrypted. Named to establish that the device-to-device BLE substrate is unofficial
  and immature. `crate-doc`. SECONDARY (community, immature). **[confirm: community crate, not core iroh;
  FACTCHECK source of truth.]** Relied on by: `ios-background-execution-and-the-ble-caution.md`.

- **mls-rs (awslabs).** An RFC 9420 implementation; experiments run against **0.55.2**. Grounds:
  propose-then-commit, offline Add via pre-computed key packages, configurable storage, subgroup
  branching, the credential-validation hook, and HPKE-bound Welcomes as the cryptographic entitlement
  boundary. `crate-doc` / `prior-art`. PRIMARY (existence proof, not a specification). **[volatile:
  0.55.2 pinned for the experiment rounds.]** URL: `https://github.com/awslabs/mls-rs`. Relied on by:
  `delivery-layer/02-references.md`, `delivery-layer/04-pitch-technical.md`,
  `delivery-layer/08-experiment-methodology.md`, `delivery-layer/10-experiments-round2.md`,
  `delivery-layer/00-session-summary.md`, `mls/mls-hardcases-and-posture.md`.

- **OpenMLS.** The RFC 9420 implementation under the Soler et al. 2025 measurement study, and the
  subject of an independent late-2025 spec-vs-production review. `crate-doc` / `prior-art`. PRIMARY.
  **[confirm: the "substantial spec-vs-production gap" review is a deployment-status claim.]** Relied
  on by: `drystone-design/scaling-and-ordering.md`, `the-four-property-tension.md`.

- **redb.** Author Christopher Berner. Embedded, transactional, ACID, pure-Rust key-value store
  (copy-on-write B-trees, LMDB-inspired); the derived-index engine. Load-bearing features: MVCC
  (single writer, many readers), constant-time savepoints (~64 kB per 1 GB), multimap tables with
  orderable values, stable file format with per-transaction durability tuning. `crate-doc`. PRIMARY.
  **[confirm against redb docs: every feature fact carries this flag; and the standing risk — redb is
  ACID-by-design and well-tested but has NO published Jepsen-grade linearizability / crash-safety
  evidence, an open risk on the authoritative-tier durability path.]** Relied on by:
  `drystone-design/redb-storage-contract.md`.

- **UniFFI (Mozilla).** Cross-language Rust binding generator; the type-safe surface for the
  Swift-shell / Rust-core bridge. `crate-doc`. PRIMARY. Version/identity fact cites the FACTCHECK
  source of truth. Relied on by: `ios-background-execution-and-the-ble-caution.md`.

- **HPKE (Hybrid Public Key Encryption, RFC 9180) — as used within MLS.** The mechanism binding a
  Welcome to a specific key package, shown in experiment to make the entitlement boundary cryptographic
  rather than a policy check. Referenced by mechanism name ("HPKE-bound Welcomes"), not as a pinned
  standalone crate. `normative-standard` (mechanism) / PRIMARY-AS-CITED (through mls-rs behavior).
  Relied on by: `delivery-layer/04-pitch-technical.md`, `delivery-layer/08-experiment-methodology.md`,
  `delivery-layer/00-session-summary.md`.

- **Hash primitives: SHA-256 and BLAKE3.** SHA-256 on the message layer (ordering tiebreak over real
  ciphertext, AuthorityState fingerprinting), BLAKE3 on the governance layer; the §4-vs-§7 split is
  the standing reconciliation. `normative-standard` / `Established`. PRIMARY. **[confirm: hash-function
  reconciliation across the asset-key layer.]** Relied on by: `drystone-design/asset-keying.md`,
  `experiments/drystone-experiments-consolidated.md`, `delivery-layer/10-experiments-round2.md`,
  `experiments/drystone-reviews-and-experiments-log.md`.

- **BeeKEM / Keyhive and Cryptree (Ink & Switch, and the Cryptree construction).** BeeKEM-shaped
  decentralized continuous group key agreement (concurrent re-key by tree-healing); Cryptree in-place
  decryption of held-but-locked ciphertext, judged redundant in Drystone's topology. `prior-art`.
  SECONDARY (design pointers, not adopted). **[confirm: BeeKEM concurrency model and its TreeKEM-needs-
  central-total-order rationale; Cryptree model.]** Relied on by: `drystone-design/asset-keying.md`.

- **p2panda access-control model.** Independent local-first work corroborating that causal-history
  access forgoes forward secrecy for durable read-scoped assets. `prior-art`. SECONDARY. **[confirm:
  p2panda access-control model.]** Relied on by: `drystone-design/asset-keying.md`.

- **Automerge / Blocklace / BFT-CRDT — NOT cited in the impl layer.** Noted for the inventory: the
  impl layer names generic CRDT / CvRDT strong-eventual-consistency properties but does not cite
  Automerge, the Blocklace, or BFT-CRDT by name (those references live in other layers). No impl entry.

---

## Papers

- **Demers et al., "Epidemic Algorithms for Replicated Database Maintenance."** Xerox PARC, ACM PODC
  1987. The root of the delivery layer's lineage: rumor mongering, anti-entropy, the two paired,
  epidemiology vocabulary, death certificates for deletion. `peer-reviewed`. PRIMARY (retrieved via
  lecture-notes / summaries this round). **[confirm: cite the canonical PODC 1987 paper directly, not
  lecture summaries, in any published version.]** Representative copy:
  `https://courses.grainger.illinois.edu/cs525/sp2016/EpidemicReplicated.pdf`. Relied on by:
  `delivery-layer/02-references.md`.

- **Leitao, Pereira, Rodrigues, "HyParView." DSN 2007.** The active/passive-view membership protocol
  iroh-gossip implements. `peer-reviewed`. PRIMARY-AS-CITED (via iroh-gossip crate docs; the paper not
  re-retrieved this round). Relied on by: `transport-iroh-gossip-and-quic.md`,
  `delivery-layer/02-references.md`, `drystone-design/liveness-freshness.md`.

- **Leitao et al., "Epidemic Broadcast Trees" (PlumTree). SRDS 2007.** The eager/lazy broadcast-tree
  refinement iroh-gossip implements. `peer-reviewed`. PRIMARY-AS-CITED (via iroh-gossip crate docs).
  Relied on by: `transport-iroh-gossip-and-quic.md`, `delivery-layer/02-references.md`,
  `drystone-design/liveness-freshness.md`.

- **Meyer, A., "Range-Based Set Reconciliation." arXiv:2212.13567 (2023); also SRDS 2023.** The RBSR
  primitive, its logarithmic-rounds complexity statement, termination and correctness by induction.
  `preprint` / `peer-reviewed`. PRIMARY. URLs: `https://arxiv.org/abs/2212.13567`,
  `https://arxiv.org/pdf/2212.13567`. Relied on by: `delivery-layer/02-references.md`,
  `delivery-layer/09-provenance.md`.

- **"Range-Based Set Reconciliation via Range-Summarizable Order-Statistics Stores." arXiv:2603.19820
  (2026).** The storage-backend obligation (summarize an arbitrary range, split by cardinality,
  enumerate residuals without rescanning) and confirmation of deployed RBSR (Nostr NIP-77, strfry).
  `preprint`. PRIMARY. URL: `https://arxiv.org/html/2603.19820`. Relied on by:
  `delivery-layer/02-references.md`.

- **Hayashibara, Defago, Yared, Katayama, "The φ Accrual Failure Detector." SRDS 2004.** DOI
  10.1109/RELDIS.2004.1353004. The accrual failure detector replacing a binary up/down verdict with a
  continuous suspicion estimate. `peer-reviewed`. PRIMARY. Relied on by:
  `drystone-design/liveness-freshness.md`.

- **CALM — Hellerstein and Alvaro, "Keeping CALM: When Distributed Consistency Is Easy." arXiv:1901.01930
  (CACM 2020); proven for relational transducers by Ameloot et al. (2013).** A program has a consistent,
  coordination-free distributed implementation iff it is monotonic; the load-bearing framing that
  freshness/completeness is non-monotonic and so has no coordination-free detector. `peer-reviewed`.
  PRIMARY. Relied on by: `drystone-design/liveness-freshness.md`, `mls/mls-hardcases-and-posture.md`,
  `mls/mls-session-summary.md`, `experiments/drystone-experiments-consolidated.md`,
  `experiments/drystone-reviews-and-experiments-log.md` (CALM Theorem 1).

- **Soler et al. (2025), MLS scaling measurement study (OpenMLS, groups up to ~5,000 members).** The
  single source for the ~2-second commit-serialization inconsistency window. `measured-study`. PRIMARY.
  **[confirm: one study under specific conditions, not a protocol constant; deployment figures may
  differ.]** Relied on by: `drystone-design/scaling-and-ordering.md`.

- **Alwen, Mularczyk, Tselekounis, "Fork-Resilient Continuous Group Key Agreement" (FREEK). IACR eprint
  2023/394.** The forward-secure-retained-init-secret construction DMLS builds on. `preprint`. PRIMARY
  (verified against the eprint). Relied on by: `mls/mls-hardcases-and-posture.md`,
  `mls/mls-session-summary.md`; the measured decentralized-MLS cost curve is credited in the cairn layer.

- **Kleppmann, "Designing Data-Intensive Applications."** The reference note's subject: the three design
  imperatives, the storage-engine (B-Tree vs LSM-Tree) and isolation lessons, the Twitter fan-out and
  doctors-on-call write-skew examples. `peer-reviewed` (textbook). PRIMARY (book and author attribution
  stand). **[confirm: the four surfaced quotes are marked [UNVERIFIED] and must be checked against the
  book before any publish.]** Relied on by: `references-designing-data-intensive-applications.md`.

---

## Tools, platform documentation, and comparative systems

### Platform (push / notification) documentation

- **Apple, "Creating the Remote Notification Payload."** 4 KB standard / 5 KB VoIP payload cap;
  background-update notifications are low priority and throttled (guideline: a few per hour, dynamic);
  never include sensitive data. `platform-doc`. PRIMARY. URL:
  `https://developer.apple.com/library/archive/documentation/NetworkingInternet/Conceptual/RemoteNotificationsPG/CreatingtheNotificationPayload.html`.
  Relied on by: `delivery-layer/02-references.md`, `ios-background-execution-and-the-ble-caution.md`.

- **Firebase Cloud Messaging, "Message types" (Google).** 4096-byte payload cap; the FCM connection is
  not end-to-end encrypted, applications must supply their own E2E; notification-vs-data background
  handling. `platform-doc`. PRIMARY (page dated 2026-06-22). URL:
  `https://firebase.google.com/docs/cloud-messaging/customize-messages/set-message-type`. Relied on by:
  `delivery-layer/02-references.md`, `ios-background-execution-and-the-ble-caution.md`.

- **APNs silent-push throttling (conflicting secondaries): Bugfender, TechConcepts, Pushy support, Newly.**
  Conflicting rate numbers (2–3/hour, ~5/day, aggressive rate-limiting, `content-available: 1` semantics).
  `secondary`. SECONDARY. **[confirm: numbers conflict; only Apple's "a few per hour, dynamic" is
  treated as pinnable.]** Relied on by: `delivery-layer/02-references.md`,
  `ios-background-execution-and-the-ble-caution.md`.

- **iOS background-execution platform facts (SLC, BGAppRefresh, BGProcessing, silent push + NSE,
  geofence; NSE 24 MB cap; `NSFileProtectionCompleteUntilFirstUserAuthentication`; CoreBluetooth State
  Restoration).** `platform-doc`. PRIMARY. These cite the FACTCHECK source of truth and are not
  re-verified; several carry FACTCHECK verdicts to preserve: **[confirm/FACTCHECK: silent-push vs NSE
  conflated in the source — both ~30 s, NSE cap is 24 MB not larger (PARTLY); geofence dwell understated
  (PARTLY); CoreBluetooth restoration covers established/pending connections only, NOT new-advertiser
  discovery (REFUTED as described).]** Relied on by: `ios-background-execution-and-the-ble-caution.md`.

### iroh project documentation (n0 primary)

- **"Iroh 1.0 — Dial Keys, not IPs" (n0 release post).** 1.0 as the first major release; wire-stability
  tied to major versions; the 0.35 public-relay window through 2026-12-31. `project-doc`. PRIMARY. URL:
  `https://www.iroh.computer/blog/v1`. Relied on by: `delivery-layer/02-references.md`.

- **iroh FAQ (n0).** The relay is "just another UDP socket"; sees EndpointId-pair and byte volume until
  hole-punch, then out of path; n0 states it does not record this. `project-doc`. PRIMARY. URL:
  `https://www.iroh.computer/docs/faq`. Relied on by: `delivery-layer/02-references.md`.

- **iroh release-candidate posts (1.0.0-rc.0, 1.0.0-rc.1).** Core API surface stabilized; address-lookup
  crates split out; relay AccessControl trait redesign; custom-transport traits behind an unstable
  feature, not covered by 1.0 guarantees. `project-doc`. PRIMARY. URLs:
  `https://www.iroh.computer/blog/iroh-1-0-0-rc-0`, `https://www.iroh.computer/blog/iroh-1-0-0-rc-1`.
  Relied on by: `delivery-layer/02-references.md`, `ios-background-execution-and-the-ble-caution.md`
  (the `unstable-custom-transports` path).

- **iroh "Gossip Broadcast" docs.** TopicId model; topic-as-swarm-and-broadcast-scope; bootstrap-peer
  join; one-topic-vs-scoped guidance. `project-doc`. PRIMARY. URL:
  `https://docs.iroh.computer/connecting/gossip`. Relied on by: `delivery-layer/02-references.md`.

- **iroh GitHub releases.** Relay breaking-changes context (AccessControl trait, ConnectionId).
  `project-doc`. PRIMARY. URL: `https://github.com/n0-computer/iroh/releases`. Relied on by:
  `delivery-layer/02-references.md`.

- **iroh 1.0 launch coverage (secondaries): TechTimes, Pinggy, AlternativeTo, Founderland, byteiota,
  StackRadar.** Corroborate the 2026-06-15 date, ~90% hole-punch, relay-sees-identifiers-not-content.
  `secondary`. SECONDARY. **[confirm: byteiota's 2026-09-30 relay-sunset conflicts with the n0 primary's
  2026-12-31; trust the n0 primary.]** Relied on by: `delivery-layer/02-references.md`.

### Comparative messaging systems (prior art, named)

- **Delta Chat (blog, Rust API docs, forum; `notifications.delta.chat` push relay).** The inverse stack
  (email/SMTP-native with iroh as a realtime add-on); corroborates content-hash dedup, application-level
  sequence numbers, the separate push-relay-host pattern, and ephemeral realtime identities without MLS.
  Also the Rust-core / Swift-shell / push-into-background reference. `prior-art`. PRIMARY (checked against
  Delta Chat primaries this round). **[confirm: the exact token/heartbeat privacy design of
  `notifications.delta.chat` was not pinned this round.]** Relied on by:
  `delivery-layer/06-deltachat-analysis.md`, `ios-background-execution-and-the-ble-caution.md`.

- **Berty (engineering blog).** A zero-server P2P messenger over BLE / Multipeer Connectivity; its own
  writing reports the OS tears a backgrounded P2P node down within seconds — the load-bearing negative
  result. `prior-art`. PRIMARY (Berty's own blog). Cites the FACTCHECK source of truth. Relied on by:
  `ios-background-execution-and-the-ble-caution.md`.

- **The four-property field: MLS, Briar, SSB (Secure Scuttlebutt), Keet (Holepunch), Matrix (Olm/Megolm),
  Cwtch.** Each named as evidence that a system reaching for all four properties either seats an unequal
  peer or drops a property. `prior-art`. PRIMARY (by system behavior). **[confirm: the Matrix and Cwtch
  entries carry inline `[confirm]` flags; deployment-status claims need a primary check before external
  use.]** Relied on by: `the-four-property-tension.md`.

### Document-method verification-pass primaries

- **GnuPG manual; Spritely values page.** Named as "applied instances" of a verify-when-reachable pass
  (alongside RFC 9750 §6.3/§6.4 and the Meadowcap spec) that cleared four reachability-only `[confirm]`
  flags. `platform-doc` / `project-doc`. PRIMARY. Relied on by: `doc-writing-method.md`. (Illustrative of
  the method, not load-bearing on a technical claim.)

---

## Empirical proofs (sibling Proofs/ repo)

Prototype results that discharge specific impl claims on real libraries. These live in the sibling
`Proofs/` repo and are cited here by name, not by a cross-repo path; they run a different stack
(openmls 0.8.1, automerge 0.7.4, a local atproto stack) than the delivery-layer experiments
(mls-rs 0.55.2, iroh 1.0.1), so each is scoped to exactly the assumption it exercises. `empirical-proof`
means a prototype run on a real library, honest to its own stated scope boundaries (transport stubbed,
single-process members, and the like). PRIMARY (the proof artifact itself).

- **Proof: openmls survivor external-commit re-key (Phase 1 GO).** `Proofs/alpha/lineage-groups/PHASE_1_FINDINGS.md`.
  On openmls 0.8.1 with openmls_rust_crypto 0.5.1: external-commit survivor re-key derives an identical
  group secret on both sides (E1.2), fresh-genesis mints a clean unrelated third epoch (E1.3), PCS holds
  across removal (E1.1), and a revocation produced while a peer is offline still re-keys on later apply
  (E1.4). Discharges the survivor/re-key feasibility beta had listed as a future integration experiment.
  `empirical-proof`. PRIMARY. Relied on by: `experiments/drystone-experiments-consolidated.md` (I2, Stage 9).

- **Proof: cross-machine deterministic reconcile (Part A).** `Proofs/alpha/lineage-groups/PART_A_RECONCILE_FINDINGS.md`.
  The reconcile computation (fork detection, deterministic survivor selection, contradiction hard-stop with
  both branches preserved, no auto-resolve) produced a byte-identical verdict on three genuinely separate
  machines with no superpeer and no orderer, invariant across every merge order tested. Discharges the
  order-independence half of the fold's permutation-invariance claim; does not discharge gap-completeness
  (partition modeled as op-log exchange). `empirical-proof`. PRIMARY. Relied on by:
  `drystone-design/fold-semantics.md`, `drystone-design/scaling-and-ordering.md`.

- **Proof: backfill admission requires standing plus contiguity (A2.3).** `Proofs/alpha/lineage-groups/PHASE_2_5_FINDINGS.md`.
  A signature-deep backfill check let two illegitimate branches through (a stranger's perfectly-signed
  history; a gapped/reordered but well-signed sequence); closed by additionally requiring contiguous
  sequence and author standing on the lineage. `empirical-proof`. PRIMARY. Relied on by:
  `drystone-design/history-durability.md`.

- **Proof: per-epoch admin authority (A2.4).** `Proofs/alpha/lineage-groups/PHASE_2_6_FINDINGS.md`.
  A naive check against the immutable genesis admin set let a removed or compromised-then-evicted admin
  keep full governance authority forever; closed by scoping authority to current-epoch standing so a
  departed admin's signatures stop counting toward thresholds. `empirical-proof`. PRIMARY. Relied on by:
  `drystone-design/governance-finality.md`.

- **Proof: source-agnostic AppView over local and Jetstream sources (H3).** `Proofs/alpha/encrypted-local-first-atproto/jetstream-appview/`.
  The identical ingest/index/serve code (asserted byte-identical at runtime) ran over a local encrypted
  stack and a real atproto Jetstream firehose (genuine CIDv1 dag-cbor, create/update/delete, cursor-resume),
  producing the same served views; the index is a disposable projection rebuilt from source.
  `empirical-proof`. PRIMARY. Relied on by: `drystone-design/social-mapping.md`.

- **Proof: public-private split, no-dangling-reference redaction.** `Proofs/alpha/encrypted-local-first-atproto/public-private-split/`.
  Default-deny visibility keyed by the record's private AT-URI (not a published field); a public reaction
  whose subject strong-reference pointed at a non-public record was redacted (dropped) before crossing the
  boundary, because publishing it would leak the private record's existence and AT-URI. `empirical-proof`.
  PRIMARY. Relied on by: `drystone-design/social-mapping.md`.

- **Proof: encrypt-then-content-address dedup loss.** `experiments/alpha/encrypted-blob-share/` (spike).
  A BLAKE3 content hash taken over ciphertext means two personas sealing the same plaintext under different
  keys or nonces produce different content hashes, so cross-user and cross-Group dedup is forfeited while
  within-Group (shared-key) dedup holds. `empirical-proof`. PRIMARY. Relied on by:
  `drystone-design/asset-keying.md`.

---

## Coverage and open [confirm]s

Coverage: this index reaches every impl document that carries an external citation — the delivery-layer
set (00–10, 12), the drystone-design set (asset-keying, fold-semantics, governance-finality,
history-durability, liveness-freshness, redb-storage-contract, scaling-and-ordering, social-mapping,
cast-beat-map), the MLS set (overview-and-terms, hardcases-and-posture, session-summary,
side-histories-and-threading), the experiments set, and the top-level notes (transport, DDIA, doc-writing
method, iOS/BLE, four-property tension). READMEs were skipped by instruction. `delivery-layer/02-references.md`
is the layer's own working appendix and is subsumed here rather than duplicated.

The most notable open items, by kind:

- **redb feature facts and crash-safety (the sharpest open [confirm]).** Every redb feature fact carries
  `[confirm against redb docs]`, and the standing risk is that redb has NO published Jepsen-grade
  linearizability or crash-safety evidence — an open risk on the authoritative-tier durability path that
  the derived tier's rebuildability only partially mitigates. This is the single most load-bearing
  unresolved external dependency in the layer.

- **iOS FACTCHECK verdicts to preserve, not soften.** CoreBluetooth restoration does NOT wake on a
  new advertiser (REFUTED as described); silent-push vs NSE were conflated and the NSE cap is 24 MB
  (PARTLY); geofence dwell was understated (PARTLY). These are the load-bearing corrections behind the
  BLE-scavenger negative result.

- **Deployment-status claims (verify against primary, do not state as settled).** "Every shipping MLS
  deployment is server-ordered" and "the serverless escapes (DMLS/FREEK, draft-xue-distributed-mls) are
  drafts / proof-of-concept as of mid-2026," plus the OpenMLS spec-vs-production gap; the Matrix and
  Cwtch four-property entries; and the Soler et al. 2025 window as one study, not a constant.

- **Volatile crate versions.** iroh-gossip is pre-1.0 (event-enum spelling unfrozen; pinned 0.100.0 →
  0.101.0); mls-rs pinned at 0.55.2; iroh-base 1.0 not re-checked. All iroh-family version facts defer to
  the FACTCHECK source of truth and are not re-verified here.

- **Willow / Meadowcap composition, hash-function reconciliation, and the local-first pointers.** The
  Meadowcap-beneath-key-wrapping composition check is unverified; the SHA-256-vs-BLAKE3 asset-layer split
  is open; and the BeeKEM/Keyhive, Cryptree, and p2panda pointers are design references carrying `[confirm]`,
  not adopted dependencies.
