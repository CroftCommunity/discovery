# Field-trades / four-property impossibility / DMLS-FREEK + redb dialogue (claude.ai, 2026-06-26)

> **Fidelity caveat (§4).** Cleaned-paste — content-faithful, **not a byte-pristine export**. Search-action
> lines and citation widgets stripped; web sources condensed into per-claim notes. Two parts: a **redb**
> deep-dive (confirms facts used in the redb build prompt) and an **adversarial investigation** of the
> field-comparison framing + the four-property "impossibility" that beta `03` asserts.
>
> **Verification disposition:** all facts here are **web-verified in this dialogue, not yet in the FACTCHECK
> SoT → [confirm before publish]**. Two are **corrections to claims currently asserted in `beta/03`**
> (Signal phone-rooted; Delta Chat metadata) and one is a **softening of an overstated claim** (the
> four-property impossibility). DMLS/FREEK + draft-xue-distributed-mls are surfaced as decentralized-MLS
> prior art directly relevant to the Drystone spec's serverless ordering.

---

## Part A — redb (confirms the build-prompt facts)

- **What it is:** a simple, portable, high-performance, **ACID, embedded key-value store in pure Rust**,
  loosely inspired by lmdb; data in a collection of **copy-on-write B-trees** (lmdb's design). Author
  **Christopher Berner** (`cberner`). Interface is **BTreeMap-like with persistence + thread-safety, fully
  type-safe** (table definitions typed at compile time).
- **History:** long beta with a blunt "may eat your data / no file-format stability" warning; **1.0 landed
  June 2023** (comparable perf to rocksdb/lmdb, memory-safe, zero-copy reads, MVCC multiple concurrent
  readers). Emerged alongside Berner's `fleetfs` distributed FS.
- **Stability:** now **mature — "stable and maintained," stable file format** with a reasonable upgrade-path
  commitment. #2 Rust DB impl, ~721k downloads/mo, 557 crates, MIT/Apache. **Honest caveat:** ACID by
  design + well-tested, but **no Jepsen-grade / published linearizability tests** (CoW + checksum design,
  no formal crash-safety evidence).
- **Notable features (the differentiators the build leans on):** **per-transaction durability tuning**
  (non-durable commits keep ACI, drop D, faster); **savepoints** (capture + roll back state at any future
  point — sub-transactions / complex rollback / distributed commit protocols); **MVCC non-blocking readers
  alongside a single writer**; **single-writer, single-process by design.**
- **Fit:** great for a single-file embedded store, no C/C++ dep, no server, type-safe BTreeMap ergonomics,
  lmdb-alternative-where-memory-safety-matters. **Poor fit** for multiple writer processes, a query language
  (SQL/Cypher), or cross-process shared concurrency. **Aligned with Croft's local-first, single-node,
  pure-Rust profile** (single-writer + stable file format are features here, not constraints).

*Bearing on the corpus:* this **confirms** the redb facts used in `seeds/generated-prompts/redb-social-graph-
layer-build-prompt.md` and `thinking/social-graph-as-substrate.md` §5 (1.0/savepoints/MVCC/per-txn
durability/stable-file-format). The redb **3.x-specific API** details remain web-sourced.

---

## Part B — investigation of the field framing + the four-property "impossibility"

The statement investigated (the `03` framing): *"no deployed system delivers usability, decentralization,
and metadata protection simultaneously — each buys one or two by spending the third. Signal buys UX with
centralization and phone-rooted identity; SSB bought pure P2P and paid with multi-device hell / recovery
dead-ends / unbounded logs; Briar buys the strongest threat model and refuses multi-device + recovery;
Delta Chat rides email for free reach and inherits email's metadata leak. The deeper version is a
four-property impossibility: group moderation + multi-device + PFS + offline-mesh cannot all hold without an
unequal, privileged peer."*

### What survives scrutiny (confirmed)

- **SSB — confirmed on every point.** Signed append-only single-writer-per-feed; the feed is owned by one
  public key, so if that node is offline the message can't be written and no other node can take over
  writing without sharing keys — exactly what breaks multi-device. Unbounded-log + replication costs real.
- **Briar — confirmed, and by design.** No account recovery (forget password / lose phone → identity +
  messages gone, new identity + re-add). Strong threat model as described (Tor hides who-talks-to-whom;
  per-contact encrypted on-device; Bluetooth/Wi-Fi blackout fallback).
- **Signal — mostly right, ONE qualifier.** Centralization + world-class UX uncontroversial. But
  **"phone-rooted identity" needs softening:** a phone number is still required *at registration*, but since
  **usernames (2024)** it is **no longer the contact-graph identifier** (you can hide your number even in
  groups; others can't find you by number). So phone-rooted at the **registration/Sybil layer**, not the
  **contact-graph layer**.

### What is now false / outdated (corrections to `03`)

- **Delta Chat "inherits email's metadata leak" — now substantially FALSE as written.** True historically
  (classic Delta over normal email: To/CC reveal who-talks-to-whom). But **as of 2.48+ (March 2026)** a chat
  message reveals **close to zero metadata to servers**: `Auto-Submitted`, `References`, `In-Reply-To`,
  `Subject`, `To`, and group-membership headers are all moved into the **encrypted part** — full **Header
  Protection per RFC 9788** — leaving only a minimal outer envelope; the outer `Date` is **randomized on a
  5-day window** to defeat timestamp correlation. With a **chatmail relay**, apps store **no metadata about
  contacts/groups** on servers, not even encrypted. **Two caveats keep it from a total reversal:** (1) this
  is the **chatmail** config, not classic email (the transport dependency `03` already noted); (2) **no
  Sealed Sender yet** — the relay can still observe which addresses talk to which. Honest restatement:
  **Delta retains a relational-metadata residue at the relay, not the full header leak.**

### The four-property "impossibility" is OVERSTATED (the load-bearing fix)

The four-property version (group moderation + multi-device + PFS + offline-mesh ⇒ a privileged peer) is
**not a proven impossibility** — it is a **real, well-documented engineering tension with a specific
mechanism that people are actively dissolving.**

- **The mechanism is exactly the one the framework intuits.** MLS requires a **Delivery Service** to agree
  on Commit *ordering*; in decentralized settings without an authoritative orderer, conflicts (forks) are
  inevitable and must be resolved later, and members must **retain key material to process commits
  out-of-order, which reduces forward secrecy.** So the ordering authority and PFS *do* trade against each
  other when you remove the server.
- **But the trade is quantitative, not binary.** **DMLS** (Phoenix R&D) extends MLS so key material can be
  retained to process Commits out-of-order **with reduced FS impact**, building on **FREEK** (Fork-Resilient
  Continuous Group Key Agreement, **Alwen, Mularczyk, Tselekounis**). The architecture already contemplates
  a **decentralized Delivery Service** (clients instantiate it by transmitting MLS messages over a P2P
  network). And **plain MLS already handles forks without a trusted referee:** if a Commit for a past epoch
  arrives, clients use a **deterministic tie-breaking policy** to keep or revert. So the ordering function
  can be a **deterministic protocol role, not an unequal privileged peer** — "privileged, unequal peer"
  smuggles in a centralization conclusion the cryptography doesn't require.

**How FREEK/DMLS actually recovers FS (cost-shifting, not magic):**
- Root problem: Commits aren't commutative → members must agree on order; MLS offloads this to a DS.
  Remove the server → can't force one order → concurrent same-epoch commits **fork**, turning the epoch
  *chain* into a **DAG**; members **lose FS** because they can't immediately delete secret key material, and
  a compromise can propagate down the epoch DAG.
- **FREEK insight — delete *selectively*, not retain wholesale.** Clients **puncture** the retained shared
  key material when processing a commit; puncturing stops an adversary who later obtains the material from
  re-computing that epoch's keys, yet is **specific to that one commit** (others can still be processed and
  punctured). Mechanically a **puncturable PRF (PPRF)**: a binary tree (depth 256 for 256 output bits);
  computing an output walks root→leaf, **deleting direct-path secrets** (makes the same output underivable
  = the FS property) while **keeping co-path secrets** (allows other outputs).
- **It is cost-shifting, and the cost is storage:** one PPRF eval needs ≥256×32 B ≈ **8 kB**, the next 255,
  etc.; total storage scales with retention period, group size, ciphersuite key size, and **fork
  frequency**. The authors frame it modestly — a **building block that meaningfully improves** FS where
  forks are inevitable, **not** a full restoration of server-ordered MLS's deletion-schedule FS.

**So the precise impact:** the four properties **don't collapse to a hard binary impossibility**; the
escape is **partial and paid for** (degraded PFS bought back incrementally with storage + complexity). The
**honest-trades framing survives better than the strict-impossibility framing** — that is the irony.

### Deployment status (the "no deployed system" airtightness check)

- **No shipping product deploys DMLS/FREEK fork-resilience.** It is a **proof-of-concept** (Phoenix R&D IETF
  internet-draft + a PoC OpenMLS fork). An independent **Nov 2025** OpenMLS review: a **substantial gap
  between spec and production**; server-side building blocks **"not really ready for use."** A **second,
  independent** decentralized-MLS draft exists — **`draft-xue-distributed-mls`** (per-member "Send Groups"
  for PCS+FS **without global ordering consensus**, for P2P/partitioned topologies) — i.e. a **live research
  frontier**, not settled shipping tech. (A Matrix-side DMLS OpenMLS fork by Hubert Chathi/`uhoreg` also
  exists — broader than just Phoenix.)
- **Every shipping MLS deployment is centralized/server-ordered:** IETF lists production users **Webex,
  Wire, Discord** (+ drones); **Wire** fully moved Proteus→MLS (group text/audio/video, thousands);
  **Google Messages + Apple Messages** began **MLS-E2EE-over-RCS in May 2026**, carrier/provider-mediated.
  The MLS architecture RFC names the **centralized DS as the norm**, the **decentralized DS** (P2P) as the
  alternative.
- **Clean, airtight phrasing:** *every shipping MLS system relies on a central delivery service for
  ordering, exactly as the framework predicts; the serverless variants that break the privileged-peer
  dependency (DMLS/FREEK, distributed-MLS) exist only as drafts + PoC code as of mid-2026.* The privileged
  ordering peer is **empirically universal in deployment, theoretically escapable, and nobody has shipped
  the escape.** (Caveat: "no deployed system" is a hard strict-negative across niche tools — use **"no major
  deployed system / no production deployment we're aware of."**)

### Net (the three fixes the framework needs)

1. **Signal:** phone-rooted **at registration**, not in the contact graph (usernames 2024).
2. **Delta Chat:** no longer the full email metadata leak under chatmail — down to a **relational-metadata
   residue** at the relay (RFC 9788 Header Protection, 2.48+).
3. **Four-property "impossibility":** an **engineering tension with a quantified FS cost and an active
   counterexample (DMLS/FREEK)**, not an impossibility; the ordering role can be **deterministic/distributed**;
   exclude in-progress protocols explicitly and say **"no production deployment"** rather than
   "impossible." The **honest-trades** spirit holds — better, in fact, than the impossibility framing.

*Relevance to Drystone:* the spec's §7 governance (serverless, **timestamp-free deterministic order**,
**fork-by-construction**, **deterministic tie-break**) **is** a decentralized-DS / decentralized-MLS
approach — DMLS/FREEK and draft-xue are **sibling prior art**, and FREEK's **out-of-order-commit FS cost**
(retain-and-puncture) is the exact cost the spec's fork/reconcile model must reckon with (couples T29 and
the survivor/re-key path T22).
