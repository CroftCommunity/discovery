# Raw transcript — MLS terminology + grounding, Blacksky, Modular Politics, MLS journey/ecosystem (2026-07-06)

**Preservation status: preserved-condensed (cleaned-paste, content-faithful — §4).** User-pasted tail of a
multi-thread claude.ai session (batch eight). UI render chrome stripped ("Searched the web", "Fetched:",
"Presented N files", "Document · MD", "Show more"); edit/search narration condensed to substance.
Content-faithful, not a byte-pristine export. Durable outputs filed verbatim: the MLS-substrate bundle
(`beta/impl/mls/`), the shared writing method (`beta/impl/doc-writing-method.md`), the Modular Politics
prior-art analysis (`beta/philosophy/prior-art/`), and the reusable research prompt
(`seeds/generated-prompts/grounded-research-and-explanation-prompt.md`). This file preserves the reasoning
and the substantial research (Blacksky, MLS ecosystem) that did **not** become standalone docs.

---

## Thread A — MLS terminology: leaf vs client vs device (the conflation, resolved against the RFC)

**User:** MLS nomenclature is loosely used around leaf/client/device. My read: a leaf is the leaf *key* a
member holds in a group to decrypt content, used by a client (software on a device); a user can have
multiple devices; a device can have multiple clients; each client likely has one leaf key. Pull the RFC and
give concrete definitions of group, membership, client, leaf, the DS node, and where "device" is used.

**Resolved against RFC 9420/9750 (grounded):** a **group** is a logical collection of clients sharing a
common secret at a given time, its state a linear sequence of epochs each depending on its predecessor. A
**client** is software acting as a group member: one MLS **leaf**, one signature key, one credential,
authenticated via the AS. A **leaf/LeafNode** (§7.2) is the member's node in the ratchet tree holding their
public key material; the leaf *key* is what a client uses. A **device** is hardware; a device may host more
than one client, and a user may have more than one device. The **Delivery Service** (RFC 9750 §5) is the
largely-untrusted node that orders/relays; it can censor and observe metadata but cannot forge or decrypt.
So the hosting chain is user → devices → clients (each a leaf), and the loose usage collapses distinct
layers that the RFC keeps separate. This produced `mls-overview-and-terms.md`.

## Thread B — consistency/clarity/correctness passes + folding question

The MLS docs (overview, hardcases-and-posture, threading, method, session-summary) went through two passes.
The folding question ("fold the hard-cases and design doc together?") resolved to: they are already one file
(`mls-hardcases-and-posture.md` is both design and posture); the threading note stays separate per the
method's layer-separation rule (threading is mostly dataplane, touches MLS only at tier 3, cross-referencing
hardcases §9). The session summary had drifted (predated the second half of the session) and was brought
current; two corrections: RFC 9750 is the **April 2025** revision (not 2023), and the `epoch_authenticator`
is flagged as an underused MLS construct Drystone may be reinventing (§10 alignment map).

## Thread C — grounding MLS references in RFC specifics (self-containment)

"Make sure our MLS references are always grounded in RFC specifics and self-contained." Two secondary-source
leans were eliminated by grounding directly in RFC text: the key-schedule commit-secret/entropy claim
(overview §3.2, now RFC 9420 §8/§12) and the transcript-hash mechanic (now §8.2 by name). Every term got a
section anchor (client/member/group/epoch §2, LeafNode §7.2, credential §5.3/§7.2, verbs §10/§12, tree §7,
key schedule §8, deletion schedule §9.2, ReInit/branching §11.2/§11.3, resumption PSK §8.6, epoch
authenticator §8.7, stale-member SHOULDs §16.6, AS/DS RFC 9750 §4/§5). The one honestly-narrowed open item:
the exact §8.2 confirmed/interim hashing formula was not read verbatim this session, so it stays `[confirm]`.
Honest boundary stated: section-accurate and safe to fold, but each load-bearing section should be read in
full before any claim becomes normative Drystone text.

## Thread D — Blacksky (research; not distilled into a doc, preserved here)

Rich grounded research on Blacksky (Rudy Fraser), an atproto community-infrastructure project. Key points, as
a candidate ECOSYSTEM entry and a governance/prior-art reference:

- **What it is:** began as a custom Bluesky feed centering Black voices; grew into full atproto
  infrastructure (own relay, PDS, AppView, moderation service, feeds, client). Framing: "infrastructure for
  interdependence," organized around "what's good for us" not "what's good for engagement."
- **Governance, three layers:** deliberation via a **People's Assembly** run on **Polis** (the vTaiwan
  consensus-mapping tool), self-hosted, recently hundreds of participants / tens of thousands of votes on
  e.g. community guidelines and an AI-coding policy; **moderation labor** by paid, community-recruited,
  publicly-credited moderators (the @blacksky.app mod service is default-on and non-disableable in their
  client, though at the protocol layer another client still chooses to subscribe to the labels);
  **funding** entirely subscription-backed via Open Collective, no ads, ~$6,500/month.
- **The gap worth watching for the co-op work:** Fraser is founder/CEO of Black Sky Algorithms — a
  **company** steering community-governed infrastructure, not a formal cooperative or nonprofit. That
  participatory-governance-vs-corporate-form gap is directly relevant to the governance-layer capital/legal
  questions.
- **AppView code:** a deliberately-thin fork of `bluesky-social/atproto` at `blacksky-algorithms/atproto`
  (not accepting contributions); changes confined to `packages/bsky`, `services/bsky`, one migration. Ripped
  out the TS indexer for a Rust one (`rsky-wintermute`) for throughput (~90 rec/s TS backfill = 6.5 years vs
  10,000+ rec/s target); self-hosting-at-scale fixes; and **Community Posts** (a `community.blacksky.feed.*`
  lexicon + `community_post` table for private, membership-gated posts that live on the AppView, not the
  PDS — inverting atproto's PDS-as-source-of-truth for that record type). Candid README documents real
  failure modes (a 1.3B-row / 663 GB notification table from a missing unique constraint; ~66k
  COPY-escaping-corrupted records; Redis disabled over a serialization bug).
- **Scale:** 0→2M users, $0 out of pocket, 100% organic; runs a **full-network** AppView (indexes the whole
  atproto network, 36M+ accounts, ~18.5B records; recommended 48+ cores / 256 GB RAM / 28+ TB NVMe; 2–4 week
  backfill). Rationale: strategic independence + ecosystem redundancy.
- **Transferable ideas for Drystone/atproto:** traffic-class **queue separation** (live vs backfill, with a
  replayable upstream cursor as the real durability guarantee, local queue disposable — Fjall embedded LSM
  store, ~72h relay retention); and the **honesty about inverting your own source-of-truth invariant**
  (Community Posts as an explicit, hard-isolated exception rather than pretending private data fits the repo
  model). Blacksky is named by Bluesky (with Northsky, Habitat) as a team implementing non-public-data
  extensions ahead of the official "Permissioned Data" work.

## Thread E — Modular Politics (produced the filed prior-art analysis)

Grounded against arXiv:2005.13701 v3 (Schneider, De Filippi, Frey, Tan, Zhang; CSCW 2021). A conceptual
paradigm for online governance from composable computational parts, rooted in Ostrom's IAD (the
"action situation"). Two claims verified against the source: (1) **all permissions derive from the platform
operator** at the Instance level (Platform → Instance → Org → Module, authority top-down; "Ultimately, all
permissions in Modular Politics derive from those that the platform administrators specify at the level of
the Instance"); (2) **security/crypto explicitly deferred** ("does not consider matters such as security and
database structures"), and wire encodings never reached ("we have not specified the particular form it
should take in software"). Worked examples are hypothetical; no prototype. The authors are candid about
built-in political bias and the "tyranny of structurelessness" defense; lineage traced to 1982 Nomic and
FidoNet VOTEMGR. **For Drystone:** asymmetric-by-construction (operator-rooted) and substrate-agnostic-by-
choice — the cryptographic-resolution and wire-encoding layers Drystone works in are exactly what it left as
future work. "It drew the map and left the territory." (Filed as `beta/philosophy/prior-art/`.)

## Thread F — the comprehensive research/explanation prompt

The user asked for a comprehensive prompt building on the search-first frame plus the quote discipline, the
plain-language-with-metaphors directive, and the no-orphaned-load-bearing-concepts rule (the "TreeKEM fix":
the failure was using a term the explanation rested on without the introduce-explain-connect trio). Delivered
and filed verbatim at `seeds/generated-prompts/grounded-research-and-explanation-prompt.md`.

## Thread G — MLS journey/ecosystem (research; preserved here, not a standalone doc)

Grounded research to take MLS seriously and defend the choice:

- **MLS is half a two-protocol story.** MLS (RFC 9420) standardizes the encryption; cross-vendor interop
  needs **MIMI** (More Instant Messaging Interoperability, IETF WG) for identity, transport, addressing, and
  the "introduction problem." MIMI is still Internet-Drafts (draft-ietf-mimi-protocol-05, Oct 2025), authored
  cross-vendor (Barnes/Cisco, Hodgson & Ralston/Matrix, Kohbrok & Robert/Phoenix R&D, Mahy); its identity
  layer contemplates X.509 and **W3C Verifiable Credentials**.
- **Unusually heavily proven, and the proofs found real limits.** Machine-checked symbolic proofs on
  bit-precise specs: TreeSync (USENIX '23, Distinguished Paper + Internet Defense Prize) and TreeKEM (IEEE
  S&P 2025), which compose. But the external-operations analysis (Cremers et al., Eurocrypt 2026) found the
  shipped standard is **weaker than earlier drafts were proven to be**: MLS needs an **SUF-CMA** signature
  (IETF Ed25519), **not** merely EUF-CMA (ECDSA), or the proof doesn't hold; and external operations lower
  PCS to **session-state** rather than full-state healing (an attacker with your long-term key can re-insert
  themselves via external resync — which is why RFC 9750's HSM/long-term-key protection is load-bearing).
- **The three-layer decomposition** (the clean way to explain MLS): **TreeSync** (authenticated group state,
  signatures + Merkle-style hashes; the layer that answers "the server can't forge who's in the group,"
  motivated by the "double join" attack) → **TreeKEM** (the group key agreement, log-rekey, FS/PCS) →
  **TreeDEM** (per-message encryption keys, fine-grained forward secrecy). Dependency is one-directional;
  proofs are symbolic (Dolev-Yao, primitives as black boxes), not computational.
- **Adoption:** GSMA RCS Universal Profile 3.0 / Apple + Google (2026); Discord "DAVE" (E2EE audio/video, a
  non-chat use of MLS as a group-key primitive; confirm via Discord's own blog); Wire; Cisco/RingCentral via
  OpenMLS. AWS `mls-rs` (Rust, RFC 9420 conformant, no full third-party audit noted).
- **The case to others:** the only group-E2EE protocol that is simultaneously an open IETF standard,
  formally verified at its core, deployed at consumer scale, and multi-vendor-backed. Honest caveats to
  raise first: MLS alone isn't interop (needs MIMI, still draft); PCS has provable limits and degrades to
  session-state with external ops; cipher-suite choice (Ed25519 > ECDSA) materially affects the proofs.

**Reference list assembled this session** (context in Thread G; full links preserved in the source chat):
RFC 9420, RFC 9750; TreeSync (USENIX '23, eprint 2022/1732); TreeKEM (IEEE S&P 2025, eprint 2025/410); ETK /
external-ops (Eurocrypt 2026, eprint 2025/229); Signal Private Group System (CCS 2020, eprint 2019/1416);
Rösler et al. "More is Less" (2018, eprint 2017/713); Cremers publications list; MIMI charter +
draft-ietf-mimi-protocol / -content / draft-mahy-mimi-identity; Matthew Green's 2018 blog posts;
awslabs/mls-rs; OpenMLS/Phoenix R&D; and secondary corroboration (Wikipedia MLS, Feisty Duck, 9to5Google,
Element/EFF on the DMA, Blessing/Anderson "One Protocol to Rule Them All?", Ember). Caveats flagged at
source: Discord DAVE from a reference list not Discord's blog; PQ-for-MLS status from SEO-grade pages,
unverified.

---

*End of preserved tail. The next session (Claude Code, filing) filed the MLS-substrate bundle to
`beta/impl/mls/`, promoted the shared writing method to `beta/impl/doc-writing-method.md` (single canonical,
removing the delivery-layer copy), filed the Modular Politics analysis to `beta/philosophy/prior-art/`, and
filed the reusable prompt to `seeds/generated-prompts/`. Blacksky and the MLS journey/ecosystem research were
preserved here (Threads D and G) as candidates for a future ECOSYSTEM.md update, not distilled into docs this
session.*
