# Reference index: "the mechanics stand on named shoulders"

Every external source the Drystone spec relies on, grouped by kind, with the exact locator (RFC
number and the section *the spec relies on*, DOI, or canonical URL), the epistemic tag the spec
carries for it, a PRIMARY/SECONDARY marker, and the spec section or companion doc that leans on it.
This is the citation-dense layer: a wrong section number here is a real defect, so each locator is
the one the spec actually cites, and every correction the changelogs record and every claim still
carrying `[confirm]` is flagged in the closing note.

The sources of record live in Part 1's "References (Part 1)" and "Upstream reference links" sections,
Part 2 Appendix C (prior art), Appendix D.3 (MLS-carried terms), and Appendix F (references and
external primaries); the DAG-CBOR primer and the conventions companion carry the rest.

Marker key:

PRIMARY = the normative artifact itself (the published RFC, the W3C Recommendation, the protocol
spec, the peer-reviewed paper, the book, the MSC/CVE record).

SECONDARY = an Internet-Draft or working-group thread standing in for a not-yet-final normative
source, or a summary standing in for a primary not consulted directly.

Epistemic tag = the spec's own status-flag ladder (conventions companion §A.9), reproduced per
source: `Verified-RFC` (checked against a normative primary), `Verified` (demonstrated against real
crypto/transport), `Established` (an inherited primitive used as-is), `Design` (committed but
unproven), `Synthesis` (assembled across sources), `[confirm]` (rests on an external fact not yet
independently verified), `[gates-release]` (must be pinned before publication/interop).

Facts about **iroh, atproto, and iOS** are cited from the FACTCHECK source of truth and are not
re-verified here: iroh core is 1.0 (wire/API-stable), the atproto repository model is where the
DAG-CBOR content-addressing discipline is borrowed from, and the iOS delivery constraint is APNs.

---

## RFCs and IETF drafts

- **RFC 9420**, *The Messaging Layer Security (MLS) Protocol* (IETF, 2023). PRIMARY · `Verified-RFC`.
  The group/epoch/commit model, ratchet tree, `PublicMessage`/`PrivateMessage` framing and AAD,
  resumption PSKs, and credential validation as application policy. Relied on across Part 2 §§4, 5,
  6.2, 6.6, 6.8, and Appendix D.3. Specific locators the spec pins: §8.2 (transcript hash represents
  one linear sequence, §7.6.3), §8.7 (`epoch_authenticator` for whole-Group consistency, §7 / hard
  case 10), §16.4 and §16.4.3 (`SenderData`/metadata analysis, §6.4 — see corrections), §12.1.6,
  §11.2, §6.3.1/§6.3.2 (encryption), §5.3.3 (client uniqueness; multiple devices as distinct
  clients), §8.6, §10, §15.3, §16.6, §16.8.

- **RFC 9750**, *The Messaging Layer Security (MLS) Architecture* (IETF, 2024). PRIMARY ·
  `Verified-RFC`. The MLS threat model and the Delivery Service / Authentication Service roles.
  Two load-bearing locators: **§6.4** (MLS enforces no access control on group operations; the
  application owns its access-control policy — the slot Drystone's governance fold fills) and
  **§6.3** (no operation requires two clients online simultaneously). Also §8 (recommends a
  reliability-and-metadata-confidential transport such as TLS/QUIC, §6.2/§6.4), §8.2.4 (device-level
  metadata-correlation tradeoff, §10.2), §2.1 (client/member/group definitions), §6.1/§7 (ReInit
  non-atomicity, §7.6.3), §5, §6.2, §6.6. Relied on across Part 2 §§5, 6.2, 6.6, 6.8, Appendix D.3.

- **RFC 8446**, *TLS 1.3* (IETF, 2018). PRIMARY · `Verified`/`Established`. The transport-encryption
  basis of iroh's QUIC sessions; §5.4 cited. Relied on at Part 2 §6.2.1.

- **RFC 9000**, *QUIC: A UDP-Based Multiplexed and Secure Transport* (IETF, 2021). PRIMARY ·
  `Established`. The transport iroh runs over. Relied on at Part 2 §6.2.1, §6.5.

- **RFC 8032**, *Edwards-Curve Digital Signature Algorithm (EdDSA)* (IETF, 2017). PRIMARY ·
  `Established`. Ed25519, the scheme behind the `EndpointId`, Pkarr records, and Willow subspace
  keys. Relied on at Part 2 §4.3, §6.

- **BCP 14** = **RFC 2119** + **RFC 8174**. PRIMARY · `Established`. The interpretation of the
  normative keywords MUST / SHOULD / MAY. Cited at Part 2 §1.

- **`draft-kohbrok-mls-dmls`**, *Decentralized MLS* (Kohbrok; IETF Internet-Draft; earlier
  `draft-kohbrok-mls-decentralized-mls-00`). SECONDARY (Internet-Draft) · `[confirm]` on draft
  revision. A decentralized-operation MLS profile on the FREEK construction, recovering forward
  secrecy lost to center-free commit ordering at a storage cost. Relied on at Part 2 §8.1, §8.1.1,
  §10.2, Appendix A.1, Appendix C.3.

- **`draft-xue-distributed-mls`**, *Distributed MLS* (Xue et al.; IETF Internet-Draft). SECONDARY
  (Internet-Draft). A "Send Groups" profile for PCS/FS without global ordering consensus; declines to
  resolve commit collisions and hands them to "the members" — the gap Part 2 §7.6 fills. Relied on at
  Part 2 §10.2, Appendix A.1, Appendix C.3.

- **`draft-ietf-avtcore-rtp-over-quic`** (RoQ), -14 as of the last revision. SECONDARY
  (Internet-Draft, no RFC number). The RTP-over-QUIC carriage for real-time media. Relied on at Part
  2 §6.12. **Correction (see closing note): this was wrongly cited as RFC 9714; it is a draft, not a
  published RFC.**

- **MLS working-group federation discussion** (`mls-federation` issue tracker, issue 6). SECONDARY
  (WG thread). The record of why MLS's commit-ordering assumption is judged unusable in
  eventually-consistent settings. Relied on at Part 2 §7.9.2, §10.2.

---

## W3C, Matrix, and other protocol specifications

- **Willow Protocol** (willowprotocol.org). PRIMARY · `Established` (data model) / `Verified`
  verbatim (Meadowcap). The namespace/subspace/path data model, Willow Confidential Sync, Private
  Area and Private Interest Intersection, and the Willow'25 parameter set. Drystone is built
  Willow-*shaped*, not Willow-*dependent*. Relied on across Part 2 §§5.5, 5.10, 5.11, 6.8.5, §7.1,
  §10.4, Appendix C.1.

- **Meadowcap** (willowprotocol.org/specs/meadowcap). PRIMARY · `Verified` verbatim. The capability
  model (an unforgeable read/write data-access token) and the communal-vs-owned namespace
  distinction — the model for the group-principal. Relied on at Part 2 §5.0, §5.4, §5.5, §5.10.

- **W3C ActivityPub** Recommendation (2018), https://www.w3.org/TR/activitypub/. PRIMARY. The
  contextual-identity lineage (no global town square; contextual flows over context collapse).
  Grounds Part 1 §2.3 and Part 2 §5.6.

- **Matrix State Resolution** v2 (**MSC1442**) and v2.1 (**MSC4297**) (matrix.org). PRIMARY ·
  confirmed against Matrix primaries. The causal-and-auth-DAG resolution with Kahn's-algorithm
  topological sort; v2.1's empty-set start and conflicted-subgraph replay is the closure Drystone
  *adopts* at Part 2 §7.5.2. Relied on at Part 2 §7.5.2, Appendix A, Appendix C.2.

- **Matrix MSC4289** (Project Hydra) (matrix.org). PRIMARY · confirmed (was `[confirm]`; see note).
  Room access control in which the creator holds "infinitely high" power — the apex-model steelman
  against `P-Peer-Equality`. Relied on at Part 1 §2.3, Part 2 §5.7, §7.3, Appendix B, Appendix C.2.

- **Matrix MSC4291** (matrix.org). PRIMARY. Room version 12: the room ID becomes the hash of the
  creation event (cryptographic create-event uniqueness), the structural root-id closure Drystone
  reaches. Relied on at Part 2 §7.6.4, Appendix A/B, Appendix C.2.

- **Matrix MSC1501** (matrix.org). PRIMARY. Room-version upgrade via tombstone-and-predecessor, the
  room-recreation model Drystone's fork is contrasted against. Relied on at Part 2 §7.6.4.

- **Matrix Project Hydra disclosure** (August 2025), the **v1.16 release notes**, and the CVE
  records — **CVE-2025-54315** (rooms before v12 lack cryptographic create-event uniqueness; High;
  no known exploitation path) and **CVE-2025-49090** (the state-reset class v2.1 fixes). PRIMARY ·
  confirmed against Matrix primaries. Cited as cautionary evidence for the monotonic-fold choice.
  Relied on at Part 2 §7.3, §7.5.2, §7.6.4, Appendix A/B, Appendix C.2. (Part 1 §2.2 invokes
  CVE-2025-49090 and still carries an inline `[confirm]` — see note.)

- **DAG-CBOR / IPLD** (the IPLD family; CBOR Tag 42 for CID links; used by the atproto repository
  model — cite FACTCHECK). PRIMARY · `Established` primitive. The determinism-for-content-addressing
  discipline behind canonical-on-the-wire bytes. See the DAG-CBOR primer; grounds Part 2 §4.6, §7.7,
  §11.1. Drystone's *own* canonical byte layout is `[gates-release]` (Part 2 Appendix B), not frozen.

- **Pkarr**, Public-Key Addressable Resource Records (pkarr.org). PRIMARY · `Verified`. Ed25519-signed
  DNS records keyed by the public key, resolvable via DNS or the BitTorrent mainline DHT. Relied on
  at Part 2 §6.9.2 (republish interval carries `[confirm]`).

- **BLAKE3** (spec). PRIMARY · `Verified` (length-extension property) / `[gates-release]` (§4
  re-proof). The committed content-addressing hash for entry digests and chain links; the Willow
  Earthstar instantiation fixes the payload hash as BLAKE3-256. Relied on at Part 2 §4, §6.8.5, §7,
  §10.4, Appendix B. Willow flags it "to be replaced by WILLAM3," a moving target.

- **SHA-256**. PRIMARY · `Established`/legacy. The hash Part 2 §4 is currently proven on, retained
  until the §4 re-proof on BLAKE3 lands (Appendix B).

- **AEAD_CHACHA20_POLY1305**. PRIMARY · `Established`. The authenticated encryption in the Willow'25
  handshake parameters.

- **Apple Push Notification service (APNs)** (Apple Developer docs; an iOS fact — cite FACTCHECK).
  PRIMARY (vendor spec). 4 KB payload cap; throttled, best-effort background notifications. Relied on
  at Part 2 §6.7.2.

- **Firebase Cloud Messaging (FCM)** (Firebase docs). PRIMARY (vendor spec). 4096-byte payload; not
  end-to-end encrypted. Relied on at Part 2 §6.7.2.

---

## Academic papers (the formal spine and the mechanism lineage)

- **Lamport, L.** "Time, Clocks, and the Ordering of Events in a Distributed System." *CACM* 21(7),
  1978, pp. 558–565. DOI: 10.1145/359545.359563. PRIMARY · `[confirm, verbatim]`. No global clock;
  "happened-before" is a partial order. Grounds Part 1 §2.0.1 and Part 2 §7.3.1. *Seam:* supplies the
  structural fact, not the "wall-clocks are an attack surface" inference (that is Drystone's).

- **Gilbert, S. & Lynch, N.** "Brewer's Conjecture and the Feasibility of Consistent, Available,
  Partition-Tolerant Web Services." *ACM SIGACT News* 33(2), 2002, pp. 51–59. DOI:
  10.1145/564585.564601. With Brewer's PODC 2000 keynote and "CAP Twelve Years Later," *IEEE
  Computer* 45(2), 2012 (DOI: 10.1109/MC.2012.37). PRIMARY · `[confirm, statement]`. Grounds
  local-first (Part 1 §1, §2.1). *Seam:* linearizable shared storage under partition, not "no shared
  truth of any kind."

- **Hellerstein, J. M. & Alvaro, P.** "Keeping CALM: When Distributed Consistency Is Easy." *CACM*
  63(9), 2020, pp. 72–81. https://cacm.acm.org/research/keeping-calm/ (arXiv:1901.01930). Conjectured
  PODS 2010; proven by Ameloot, Neven & Van den Bussche, *J. ACM* 60(2), 2013. PRIMARY · **Verified
  this revision** (Theorem 1 verbatim). Coordination-free consistency iff monotonic — the formal
  boundary under the resolvable/residue split. Grounds Part 1 §2.2/§2.5, Part 2 §7.3, §7.9, Appendix
  B, Appendix C.1.

- **Shapiro, M., Preguiça, N., Baquero, C. & Zawirski, M.** "Conflict-Free Replicated Data Types."
  *SSS 2011* (LNCS 6976), pp. 386–400. DOI: 10.1007/978-3-642-24550-3_29. Companion INRIA RR-7506,
  https://hal.inria.fr/inria-00555588. PRIMARY · `[confirm, statement]`. Convergence without a
  coordinator for the monotonic class. Grounds Part 1 §2.2, Part 2 §7.7, §7.1, Appendix C.1.

- **Kleppmann, M., Wiggins, A., van Hardenberg, P. & McGranaghan, M.** "Local-First Software: You Own
  Your Data, in Spite of the Cloud." *Onward!* 2019 (Ink & Switch). PRIMARY · `[confirm]` attribution.
  The data-ownership property Drystone realizes. Relied on at Part 2 §10.3.1, Appendix C.1.

- **Alwen, J., Mularczyk, D. & Tselekounis, Y.** "Fork-Resilient Continuous Group Key Agreement"
  (FREEK). IACR ePrint 2023/394. PRIMARY. The puncturable-PRF construction recovering forward secrecy
  lost to out-of-order commit processing, at a storage cost. Relied on at Part 2 §8.1.1, Appendix A.1.

- **Cremers, C., Günsay, E., Wesselkamp, V. & Zhao, M.** "ETK: External-Operations TreeKEM and the
  Security of MLS in RFC 9420." IACR ePrint 2025/229, 2025; EUROCRYPT 2026. PRIMARY. First
  computational security analysis of RFC 9420 covering external operations. Relied on at Appendix A.1.

- **Leitão, J., Pereira, J. & Rodrigues, L.** "HyParView: A Membership Protocol for Reliable
  Gossip-Based Broadcast." *DSN 2007*, pp. 419–428. DOI: 10.1109/DSN.2007.56. PRIMARY. The membership
  layer of the gossip overlay. Relied on at Part 2 §6.10.1.

- **Leitão, J., Pereira, J. & Rodrigues, L.** "Epidemic Broadcast Trees" (PlumTree). *SRDS 2007*, pp.
  301–310. DOI: 10.1109/SRDS.2007.27. PRIMARY. The broadcast layer of the gossip overlay. Relied on at
  Part 2 §6.10.3.

- **Demers, A., et al.** "Epidemic Algorithms for Replicated Database Maintenance." *ACM PODC* 1987.
  PRIMARY. The epidemic-dissemination lineage the Delivery Fabric draws on. Relied on at Part 2 §6.3.

- **Meyer's range reconciliation** (range-based set reconciliation / RBSR). PRIMARY. Prior art for the
  RBSR sync Drystone uses. Relied on at Part 2 §6.8.1, §7.1, Appendix C.1. **Negentropy** is one
  candidate RBSR construction, not yet chosen (Part 2 §6.8.1, Appendix B); the RBSR arXiv identifier
  carries `[confirm]`.

- **Soler, et al.** A 2025 measurement of MLS Delivery-Service commit-application latency (a
  convergence window on the order of two seconds). PRIMARY · full title and venue **to be pinned
  before publication**. Relied on at Part 2 §7.9.2.

---

## Named prior-art (the cross-disciplinary and governance-frontier grounding, Part 1 §3)

These ground *values*, not mechanisms; the spec cites them as corroboration that the design is
discovered rather than invented, and marks the value-to-mechanism gap explicitly.

- **Mill, J. S.** *On Liberty*, 1859. https://www.gutenberg.org/ebooks/34901. PRIMARY · `Verified`.
  Silencing the dissenter is an unjustifiable cost — the fork as the dignified exit. Grounds Part 1
  §2.5, §3, Part 2 §7.6, §8.

- **Hayek, F. A.** "The Use of Knowledge in Society." *American Economic Review* 35(4), 1945, pp.
  519–530. https://www.jstor.org/stable/1809376. PRIMARY · `Verified verbatim`. Dispersed knowledge;
  utility cannot be centrally computed. Grounds Part 1 §2.0, §3. *Seam:* about the utility layer, not
  the provenance layer.

- **Ashby, W. R.** *An Introduction to Cybernetics*, 1956, p. 207 (the Law of Requisite Variety).
  http://pcp.vub.ac.be/books/IntroCyb.pdf. PRIMARY · `Verified`. Only variety absorbs variety.
  Grounds Part 1 §2.3, §3.

- **Beer, S.** *Brain of the Firm*, 1972 (2nd ed. Wiley, 1981). The algedonic channel and the
  "specify only somewhat" design discipline. PRIMARY · `[confirm]`. Grounds the escalate-the-hard-case
  posture (Part 1 §2.5, Part 2 §7.6). **The "specify only somewhat" wording is `[confirm]` against the
  primary edition; the Cybersyn/OGAS capacity figures (10–30% transport capacity; the OGAS history)
  are `[confirm]` against primary sources; the "aids to human viability, not excuses for automatic
  command" phrasing is a labeled synthesis gloss, not a verbatim Beer line.**

- **Popper, K.** *Conjectures and Refutations*, 1963 (Routledge), §XVII (p. 30). PRIMARY · `Verified`.
  Knowledge is finite, ignorance infinite; corroboration, never verification — the shape of provenance.
  Grounds Part 1 §2.0, §2.2, §3.

- **Ostrom, E.** *Governing the Commons: The Evolution of Institutions for Collective Action.*
  Cambridge University Press, 1990. DOI: 10.1017/CBO9780511807763. Design principles **6**
  (conflict-resolution mechanisms) and **7** (minimal recognition of the right to organize). PRIMARY ·
  `[confirm, verbatim wording of principles 6 and 7 against the 1990 primary]`. Grounds Part 1 §2.3,
  §2.4, §3, Part 2 §7.6, Appendix C.4/C.5. Cited in the lowercase-group (social) sense, with the
  capital-G Group named as the *realization*.

- **Wilson, D. S., Ostrom, E. & Cox, M. E.** "Generalizing the core design principles for the efficacy
  of groups." *Journal of Economic Behavior & Organization* 90S, 2013, pp. S21–S32. DOI:
  10.1016/j.jebo.2012.12.010. PRIMARY · `[confirm, verbatim; distinct from the 1990 book — do not
  conflate]`. The subsidiarity capstone (lowest jurisdiction unless ineffective). Grounds Part 1 §3.

- **Schneider, N., De Filippi, P., Frey, S., Tan, J. Z. & Zhang, A. X.** "Modular Politics: Toward a
  Governance Layer for Online Communities." *Proc. ACM Human-Computer Interaction* 5 (CSCW1), 2021,
  article 16. DOI: 10.1145/3449090. PRIMARY · `[confirm, quotations against the CSCW paper]`. The
  nearest neighbor in intent (governance as an open protocol standard), which roots authority in a
  platform operator and brackets the wire encodings Part 2 supplies. Grounds Part 1 §1.2, Part 2
  Appendix C.4.

- **Spritely Institute** (Lemmer-Webber, C.), "Technical Values and Design Goals"
  (spritely.institute/about). PRIMARY · `Verified verbatim` (the page). The contextual-identity posture
  (contextual flows over context collapse; trust as contextual/revocable). Grounds Part 1 §2.3, Part 2
  §5.6.

- **Stiegler, M.**, "An Introduction to Petname Systems" (the petname tradition; Spritely Brux).
  PRIMARY. The nearest prior art for human-meaningful naming over non-human-meaningful keys. Grounds
  Part 1 §2.3 (naming).

- **GnuPG / OpenPGP web of trust** (gnupg.org). PRIMARY (reference implementation/spec). The "casual"
  and "extensive" verification levels, and the failure mode (scalar, context-free, automatically
  transitive trust) whose lesson shapes the graded-vouch rule. Cited at Part 1 §2.0, Part 2 §5.6.

- **Blockchain / DLT governance and the DAO hard-fork.** PRIMARY (documented case). The
  global-consensus-on-a-canonical-chain model, forks as failures — the inverse of Drystone's fork as a
  first-class good; the DAO fork as the case where human forking is forced. Cited at Part 1 §1.2, Part
  2 Appendix C.2.

- **Hyperledger Fabric** and **Keyhive / p2panda** (related systems). PRIMARY. Fabric's
  membership-scoped channels over a shared gossip overlay mirror the Delivery-Fabric-versus-Group split
  (Part 2 §6.3); Keyhive/p2panda are the capability/local-first comparison at Part 2 §5.11, where the
  durability-not-forward-secrecy claim carries `[confirm]`.

---

## Citation-accuracy and open-`[confirm]` note

This is the layer where a wrong section number is a defect, so the corrections the changelogs record
are surfaced here, followed by the claims still not settled.

**Corrections already applied (do not reintroduce the old form):**

- **RFC 9750 §3.5 → §6.4.** The "MLS enforces no access control on group operations; the application
  owns the policy" citation was corrected from §3.5 to **§6.4** (changelog document-pass-2, fix D1).
  The correct locator §6.4 is used throughout.

- **RFC 9420 §16.4 generation-counter claim removed.** An earlier draft claimed `PrivateMessage`
  exposes a per-sender **generation counter** and that a gap reveals a missed message to an observer.
  Verified verbatim against §16.4: **that claim was wrong and was removed** — `generation` lives inside
  the AEAD-encrypted `SenderData`, so it is not visible in the framing to an observer (document-pass-3
  / document-pass-5). What remains verified: `group_id` and `epoch` are cleartext in the header and
  unprotected against the DS; group membership is inferable.

- **RoQ is an Internet-Draft, not RFC 9714.** The real-time-media carriage (Part 2 §6.12) was wrongly
  cited as "RFC 9714 (Standards Track)"; corrected to `draft-ietf-avtcore-rtp-over-quic` (-14), which
  has **no RFC number**. The media path rides a not-yet-final draft (document-pass-6). The media
  codec/RTP path is `Design`.

- **Sigstore mischaracterization corrected.** Sigstore was loosely called "countersigning"; corrected
  to name its actual primitive — **Rekor**, an append-only Merkle transparency log with signed
  tree-head checkpoints. Drystone draws only the checkpoint / Merkle-consistency **analogy** (Part 2
  §7.3.7 roll-ups), not a literal Sigstore feature; the Sigstore name no longer appears in the current
  Part 2 body.

- **Epoch-number metadata leak confirmed** verbatim against RFC 9750 (opaque `group_id` + numerical
  epoch = change count, correlatable by a network observer; mitigation is a metadata-confidential
  transport), per RFC 9750 §8.2.4 for the device-level correlation tradeoff.

**Load-bearing claims still carrying `[confirm]` (not settled until pulled from the primary):**

- The distributed-systems formal-spine verbatims: **Lamport** (verbatim), **Gilbert & Lynch / CAP**
  (statement), **CRDT** (statement) all remain `[confirm]` in Part 1. CALM is the exception — verified
  this revision.

- **Matrix MSC4289 steelman and the CVE-2025-49090 root-cause.** Part 2 Appendix C reports the Project
  Hydra facts as now confirmed against Matrix primaries, but **Part 1 still carries inline `[confirm]`
  flags** (§2.2: CVE-2025-49090 root-cause against the MSC4297 primary text; §2.3: the MSC4289
  uncapped-creator steelman). This is a residual Part 1/Part 2 status inconsistency worth reconciling.

- **Ostrom principles 6 and 7** (verbatim wording against the 1990 primary) and **Wilson-Ostrom-Cox
  2013** subsidiarity (verbatim, and distinct from the 1990 book).

- **Beer**: "specify only somewhat" verbatim; the Cybersyn/OGAS capacity figures and history; and the
  labeled "aids to human viability" synthesis gloss.

- **Modular Politics** quotations against the CSCW paper.

- **Soler et al. 2025** MLS-latency measurement — full title and venue to be pinned before publication.

- Draft- and crate-level: the **`draft-kohbrok-mls-dmls`** revision, the **RBSR arXiv id** and CRDT
  report identifiers, **iroh-gossip** internals against its pinned pre-1.0 version, the
  **Pkarr/address-lookup** republish-and-expiry intervals, and the **Keyhive/p2panda**
  durability-not-forward-secrecy claim.

- Byte-level `[gates-release]` items that gate interop (not `[confirm]` but release-blocking): the
  Drystone canonical wire encoding over DAG-CBOR-shaped bytes, and the BLAKE3-based §4 re-proof (§4 is
  `Verified` on SHA-256 today; BLAKE3 is the committed suite).
