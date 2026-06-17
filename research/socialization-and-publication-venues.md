# Where to Socialize and Publish the Croft Protocol

**Subject:** Venues for sharing the Croft protocol spec — community/practitioner spaces, standards bodies, academic peer review, and defensive-publication / prior-art channels.

**Purpose:** Decide where to put a complete Croft spec in front of the right audiences to (1) get it spread and used, and (2) establish durable, examiner-discoverable prior art so no one else can patent the design. These two goals are tracked together because the strongest prior-art vehicles (an IETF Internet-Draft, an arXiv preprint) double as credibility and reach.

**Date:** 2026-06-16

**Status:** Research deliverable. No application code. Current (2026) facts verified against primary sources where reachable; items not verifiable are marked `[UNVERIFIED]`. Sequencing assumes a "defensive publication" intent (spread + block-others-patenting), **not** retaining a patent option for ourselves — see the disclosure caveat in the closing section.

---

## Executive summary

Croft is a **layered** protocol — encrypted QUIC transport with relay placement (iroh), MLS group key agreement, Ed25519 device identities under a lineage/standing membership model, SHA-256 content addressing and gossip topics, and a public-social side keyed to atproto DIDs. No single venue covers all of it. The strategy is therefore **per-layer**: socialize each layer where its experts already are, and **sequence** so the foundations (transport, crypto, data model) are validated in high-signal communities *before* any broad public launch.

The central operational insight: an **individual IETF Internet-Draft** is the single highest-leverage first move. It is free, self-serve, requires no membership or working-group sponsorship, produces a third-party-timestamped, examiner-known, publicly archived dated disclosure (strong prior art), and simultaneously signals to a working group that there is a concrete proposal. It serves both goals at once.

The flagship public moment is **Local-First Conf 2026** (Berlin, Jul 12–14), whose stated theme explicitly names "peer-to-peer, self-sovereign identity, ATproto" — a near-exact description of Croft. Its CfP is open as of this writing.

The one-directional caveat, carried from the IP discussion: every venue here is a **public disclosure**. The first post or draft starts the US patent grace clock and forecloses most foreign patent rights. That is the intended outcome for a defensive posture, but it means the patent-it-ourselves door closes the moment we socialize.

---

## Two-track framing

```
  PRIOR-ART TRACK  (serves "block the patent")        SPREAD TRACK  (serves "be used")
  ┌──────────────────────────────────────┐           ┌──────────────────────────────────────┐
  │ individual IETF Internet-Draft         │           │ iroh / n0 community (home venue)       │
  │   free, self-serve, dated, examiner-   │           │ Local-First Conf 2026 (flagship)       │
  │   known archive — do FIRST             │           │ awesome-iroh PR, newsletters, Show HN  │
  │ arXiv preprint (cs.NI + cs.CR/cs.DC)    │           │ atproto dev community (public side)    │
  │ (optional) Technical Disclosure Commons │           │ Willow/Earthstar peers (data model)    │
  └──────────────────────────────────────┘           └──────────────────────────────────────┘
              the same publications double as credibility for the spread track
```

---

## Layer → venue map

| Croft layer | Primary venue | Fit / why |
|---|---|---|
| Transport (iroh QUIC, relay placement, NAT fallback) | **iroh / n0 community** — Discord, GitHub Discussions, `awesome-iroh` | Home community; n0 actively showcases exactly this. No clean *standards* home for iroh-style relay placement; MASQUE is adjacent prior art only. |
| Group crypto (MLS) | **OpenMLS maintainers + IETF MLS WG** | Correctness review of MLS usage before broadcasting. Lineage/standing maps to MLS custom-credential work the WG is actively doing. |
| Device identity / DIDs | **W3C Credentials CG** | Lowest-barrier expert review of the DID layer; home for DID-*method* discussion (the DID WG explicitly does not standardize methods). |
| Public-social side (atproto DIDs) | **IETF atproto WG + W3C Social Web WG + atproto dev community** | Public-identity layer is in scope for the atproto WG; encrypted core is explicitly out (that is MLS/MIMI). Brand-new WG = high leverage to engage early. |
| Data model / membership / sync | **Willow / Earthstar / p2panda peers + Malleable Systems forum** | Sharpest peer critique of content addressing, sync, and membership models — these communities work on exactly these problems. |
| Decentralization thesis | **IRTF DINRG + ANRW '26 workshop** | Research forum to socialize the architecture with decentralization researchers; produces a citable association. |
| Peer-reviewed validation | **PoPETs / NSDI / USENIX Security** | Rigorous review + strong citable prior art (see academic table). |
| Prior-art timestamp | **IETF I-D archive + arXiv** | Dated, examiner-discoverable, persistent (see prior-art table). |

---

## 1. Community / practitioner venues

### iroh / n0 — top-tier fit (home venue)
Croft is built directly on iroh; n0 cultivates an ecosystem and will amplify work built on their stack.
- **Channels:** Discord (positioned as the community hub), GitHub Discussions (`n0-computer/iroh`), Twitter `@iroh_n0`, Mastodon `@n0iroh`, a Matrix room, blog, YouTube.
- **Showcase:** `n0-computer/awesome-iroh` catalogs 40+ projects (live "Social Media" and "Collaboration & Productivity" categories — Croft fits). Add via PR.
- **Engage:** (1) GitHub Discussion on relay-placement + MLS-over-iroh asking for transport feedback; (2) PR to `awesome-iroh`; (3) introduce in Discord projects channel.
- **Effort:** Low–medium. **Audience:** iroh/Rust transport engineers. **Payoff:** deep transport-level feedback, n0 amplification, possible collaborators already building P2P social on the same stack.

### Local-First community — top-tier fit (flagship)
- **Local-First Conf 2026:** Jul 12–14, Festsaal Kreuzberg, Berlin. Single track, ~350 people. **Theme explicitly: "beyond CRDTs to explore malleable software, peer-to-peer, self-sovereign identity… ATproto."** CfP open; most talks community-sourced. Informal gathering Jul 11; Day 3 is an Ink & Switch–hosted Lab Day (unconference + demos).
- **localfirst.fm:** active podcast; running a 2026 conf special series. Guest/episode pitch target once Croft is demoable.
- **Local First Discord** (linked from conf site); **localfirstweb.dev** is the web-dev slice.
- **Ink & Switch orbit:** intellectual center (Automerge, Patchwork); engage via the conference, their dispatch, and the Discord.
- **Framing note:** Croft is P2P-messaging-first, not CRDT-first — lead with the local-first / self-sovereign-identity / P2P angle the theme invites; mention Automerge adjacency to connect with the CRDT core.
- **Effort:** CfP submission low; attending Berlin high. **Payoff:** highest concentration of the exact target audience plus the podcast for reach.

### Decentralized / P2P communities
- **Willow / Earthstar / p2panda orbit — high fit for protocol-design peer review.** Active in 2026 (Earthstar v11 / Willow-powered, in beta). These people reason about content addressing, sync, and capability/membership — the sharpest critique of Croft's data + membership model. Engage via GitHub and the Malleable Systems forum (hosts Willow threads). Effort low–medium.
- **DWeb Camp 2026 ("Root Systems") — high fit for mission/values audience.** Jul 8–12, Alte Hölle near Berlin (moved from California this year; immediately precedes Local-First Conf — one Berlin trip covers both). Internet Archive–founded; censorship resistance, identity for the stateless, data sovereignty. Community on Mastodon `@dweb@social.coop`. Effort high (in-person, applications). Payoff: values-aligned early adopters, visibility among DWeb funders. Medium for deep protocol critique.
- **Spritely Institute (Goblins / OCapN) — medium fit.** Active (Goblins v0.18.0, Apr 2026); forum `community.spritely.institute`, office hours. Relevant if Croft's lineage/standing membership takes a capability framing. Low effort.
- **Matrix — medium fit.** Active, large secure-messaging audience, decentralized-identity MSCs, Protocol Labs overlap. Different architecture (homeservers, not pure local-first P2P).
- **IPFS / libp2p (Protocol Labs) — lower fit.** Adjacent, not native (Croft is on iroh, not libp2p). Engage only for content-addressing cross-pollination.
- ⚠️ **Classic Secure Scuttlebutt — largely inactive for new-protocol socialization in 2026.** Energy migrated to Earthstar/Willow/p2panda; most active surviving impl is Āhau (Māori data sovereignty). Go to the successors.

### atproto / Bluesky developer community — strong fit (public-social side)
- **Discord:** "ATProto Touchers" at `discord.atprotocol.dev`. **Forum:** `discourse.atprotocol.community`. **Hub:** `links.atprotocol.dev` (channels, AT Community Fund, events).
- **ATmosphereConf 2026:** Vancouver, Mar 26–29 — **already passed** relative to 2026-06-16. Engage via Discord/forum; target the next edition. `[UNVERIFIED]` next-edition dates.
- Spring 2026 roadmap shows the Bluesky protocol team actively discussing architecture with the developer community — receptive moment.
- **Frame** Croft as an atproto-adjacent fabric (encrypted P2P groups complementing the public atproto graph). **Payoff:** Lexicon/DID feedback, a developer audience hungry for protocol-adjacent apps. Effort low (Discord/forum); medium (conf).

### Secure-messaging / cryptography practitioners
- **OpenMLS** (pure-Rust MLS lib; maintainers Kohbrok/Robert) — high fit for "am I using MLS correctly," especially given Croft is Rust/iroh. Engage via GitHub. Low–medium effort.
- **IETF MLS WG** implementer discussion (`mls@ietf.org`) — straddles standards/community; use for correctness review of the MLS usage, not for mindshare. (See standards section.)

### Aggregators / publishing for reach (after a runnable demo exists)
- **Hacker News (Show HN):** highest reach, spiky. Rule: "Show HN is for things people can try" — needs a working demo/repo. Title `Show HN: Croft – <one-line technical description>`; no hype words; post Tue–Thu ~9am–12pm ET; answer comments hard in the first 60 min.
- **lobste.rs:** smaller, higher-signal, more technical; better comments for a protocol. Tag `distributed`/`crypto`/`rust`.
- **localfirstnews.com:** weekly (Thursdays) local-first newsletter; well-targeted. Submit via their GitHub "Contribute" or `hello@localfirstnews.com`.
- **Bluesky / Mastodon tech circles:** natural given the atproto tie-in; low effort, compounds.
- **Reddit:** r/selfhosted, r/privacy, r/decentralization, r/rust. Secondary reach, not feedback. `[UNVERIFIED]` current per-subreddit activity.

---

## 2. Standards-body and technical-review venues

### IETF / IRTF

**MLS WG (Messaging Layer Security) — best crypto-layer fit.** Owns RFC 9420 (protocol) + RFC 9750 (architecture); active 2026 work includes `draft-ietf-mls-extensions` (rev 09, expires Sep 3 2026), `draft-ietf-mls-partial`, post-quantum `draft-ietf-mls-combiner` (milestone Dec 2026), additional credential types, MIMI support. The "new credential types" work is exactly where Croft's Ed25519-device-identity / lineage-standing model would be discussed (MLS credentials are pluggable). Engage: `mls@ietf.org`, GitHub `ietf-wg-mls`; meets 3×/year + interims; free to join the list. Highest-quality cryptographic review available; strong prior art.

**MIMI WG (More Instant Messaging Interoperability) — adjacent.** Cross-provider messaging interop over MLS (arch, room policy, content, protocol drafts active in 2026). Croft's rooms/membership/policy map onto MIMI's room-policy abstractions. Relevant if Croft wants broader interop; less central if it stays a closed fabric. Engage: `mimi@ietf.org`, GitHub `ietf-wg-mimi`. Useful for aligning vocabulary and positioning lineage/standing against MIMI's policy model.

**Authenticated Transfer WG (atp / atproto) — newly chartered, identity-layer relevance.** Formed Mar 19 2026 (Applications & Real-Time area). Standardizes the atproto sync layer: repo data structure, low-latency sync, `at://` URI scheme, account-identifier resolution. **DIDs are in scope** (`did:web` backwards-compat required; PLC + did:web resolution). **End-to-end-encrypted / non-public messaging is explicitly OUT of scope**, as are application schemas and social-app semantics. So Croft's **public, atproto-DID-keyed side is in scope** (validate identity/resolution conformance here); Croft's **encrypted core is out** (that is MLS/MIMI). Engage: `atp@ietf.org`, GitHub `ietf-wg-atp`. Early engagement in a brand-new, high-interest WG is high-leverage.

**DINRG (IRTF Decentralization of the Internet RG) — research-forum fit.** Open research forum on internet-centralization and decentralization mitigations; documents outcomes via papers/RFCs. Met at IETF 125 (Mar 16 2026). Ideal room to *present* Croft's decentralization architecture (local-first, decentralized identity, relay overlays as anti-consolidation) to decentralization researchers and gain a citable IRTF association. Will not standardize Croft. Low barrier (list post or presentation slot).

**MASQUE WG — narrow relay/QUIC-proxy relevance.** `draft-ietf-masque-quic-proxy-07` (WG Last Call, expires Sep 17 2026) adds QUIC-aware proxying through HTTP/3 proxies. Croft's iroh relay-fallback solves a related problem but is its own design, not MASQUE. Read for prior art / framing; not an obvious place to standardize Croft. NAT traversal for P2P-QUIC is fragmented across MASQUE and individual drafts — no single clean standards home for iroh-style relay placement. `[UNVERIFIED]` whether any 2026 WG charters P2P-QUIC NAT traversal specifically.

**GNAP — not applicable.** WG concluded (core shipped as RFC 9635, Oct 2024). Delegated-authorization protocol; not a fit for Croft's identity/key model.

**Internet-Draft as a prior-art vehicle — strong yes.** Anyone can submit an individual I-D (`draft-<name>-croft-...`) via the Datatracker submit tool — no WG sponsorship, membership, or fee; RFCXML preferred. Establishes a dated public disclosure in an examiner-known corpus and signals a concrete proposal to a WG. Caveat: I-Ds expire after 6 months and are non-citable *as standards*, but the **archived dated copy persists** and is discoverable — fine as prior art / timestamp after expiry.

### W3C

**Credentials Community Group (CCG) — identity-layer incubator, best low-barrier entry.** Active 2026 (~600 participants, weekly calls); incubator where DIDs and Verifiable Credentials originated. Strong fit for Croft's DID + Ed25519 device-identity layer; broadest prior-art surface into the DID/VC ecosystem. Because the DID WG won't standardize DID *methods*, method-level discussion (including atproto's `did:plc`/`did:web` usage) belongs here. Engage: sign the W3C Community CLA, join `public-credentials@w3.org`. No W3C membership needed. Low–moderate effort.

**Social Web WG — revived, direct fit for the public social layer.** New charter runs Jan 15 2026 – Jan 31 2028; maintains ActivityPub, Activity Streams, WebSub, Webmention; may adopt LOLA (account portability — relevant to the portable-identity story). Chair Darius Kazemi; backwards-compatible spec iteration targeted Q3 2026. Engage: `public-socialweb@w3.org` + GitHub (open to follow); apply as Invited Expert for a formal voice; W3C Patent Policy (royalty-free). High reach into the fediverse.

**DID WG — conformance target, higher barrier.** Active; DID v1.1 reached Candidate Recommendation Snapshot Mar 5 2026. Governs the DID core data model Croft must conform to but **does not standardize individual methods**. Treat as a conformance target to read/align with; route method-level work through the CCG. Requires W3C Membership or Invited Expert status. `[UNVERIFIED]` charter end-date (live page says Oct 28 2026; 2024 charter doc says Apr 28 2026 — likely an extension).

**Solid CG / Linked Web Storage WG — adjacent, weaker fit.** LWS WG (Sept 2024–Sept 2026) taking Solid to Recommendation; auth suites include `did:key`. Server-pod, HTTP-, RDF-centric — not CRDT/P2P-native. Useful for auth/data-store interop prior art only.

**No W3C home for local-first/CRDT or MLS.** The CRDT-for-RDF CG closed May 21 2026; no active W3C local-first/CRDT group exists. No W3C secure-messaging/MLS group — that is IETF (RFC 9420/9750). **Skip W3C FedID WG** (FedCM is browser-mediated centralized-IdP login — opposite of Croft's model).

---

## 3. Academic venues (peer review + citable prior art)

| Venue | Fit | 2026–27 timing (verified unless noted) | Bar / payoff |
|---|---|---|---|
| **USENIX NSDI '27** | Best overall — overlay/relay transport, NAT traversal, gossip, QUIC P2P | Fall abstracts ~Sep 10 2026; conf May 11–13 2027 Providence. Full-paper date `[UNVERIFIED]` | Very high (~15–20%); needs implementation + measurement. Top networking citation weight |
| **USENIX Security '27** | Strong — MLS/identity-lineage security architecture | Cycle 1 ~Aug 25 2026; Cycle 2 ~Jan 26 2027; conf Aug 2027 Denver | Very high (~15%); will scrutinize lineage/standing vs. standard MLS guarantees |
| **IEEE S&P 2027** | Moderate-strong — needs sharp security novelty | Cycle 1 paper ~Jun 11 2026; Cycle 2 ~Nov 17 2026; conf May 2027 Montreal | Highest bar (~12–15%); likely overreach for a first systems-design paper |
| **PoPETs / PETS 2027** | Most forgiving for an independent author — privacy/metadata of relay placement, gossip, atproto-DID layer | Rolling: May 31 / Aug 31 / Nov 30 2026 / Feb 28 2027 | Journal model with accept/revise/reject; structured revision path. Strong privacy citation |
| **ACM CoNEXT 2026** | Strong, friendlier to design + early eval than NSDI | Summer cycle ~May 29 2026 (likely passed); conf Dec 7–11 2026 Utrecht | High but below NSDI |
| **ACM IMC 2026** | Weak — only if a measurement study (relay/NAT success rates in the wild) | Cycle 2 ~Apr 29 2026 (passed); conf Oct 12–16 2026 Karlsruhe | Design itself out of scope |
| **HotNets 2026** (workshop) | Excellent for design-stage forward-looking paper | `[UNVERIFIED]` CFP not yet published; historically late-Jun/early-Jul deadline, Nov workshop | Idea-not-implementation bar; plant a flag early |
| **ANRW '26** (ACM/IRTF workshop) | Strong + strategic — IETF-adjacent audience | Co-located IETF 126 Vienna, Jul 20 2026; submit ~April `[UNVERIFIED]` | Lower selectivity; standards reach + real citation |
| **FOCI 2026** (workshop) | Good for censorship-resistance / open-comms angle | Submit ~Apr 20 2026 (passed); workshop Jul 20 2026 Calgary (w/ PETS) | Workshop bar; narrower reach |

**Timing reality (June 2026):** near-term realistic targets are PoPETs rolling (Aug 31 / Nov 30), USENIX Security (Aug 25 / Jan 26), and NSDI fall abstract (~Sep 10). PoPETs' rolling model is the most accessible for an independent author. Use HotNets/ANRW workshops first if the work is still design-stage.

---

## 4. Defensive-publication / citable-timestamp venues

| Venue | Prior-art strength | Effort | Notes |
|---|---|---|---|
| **arXiv** (cs.NI primary; cross-list cs.CR, cs.DC) | Strong academic precedence; generally accepted as published prior art (immutable third-party timestamp) | Low cost; **endorsement is the friction** for a non-academic first-timer | Best combination of prior-art + reach + credibility. See endorsement note |
| **IETF I-D archive** | Strong — examiner-known, public, dated, persists after expiry | Low (free, self-serve) | Best dual-purpose vehicle (also a standards move) |
| **IP.com (Prior Art Database)** | Strongest examiner-discoverability — aggregated into databases patent offices search worldwide | Paid; formatted disclosure | Purpose-built defensive publication |
| **Technical Disclosure Commons** | Good — free, examiner-accessible repository | Low (free) | Free alternative to IP.com |
| **Linux Defenders** | Good — publishes into examiner-searchable databases (routes to IP.com) | Low/free, FOSS-oriented | Fits an open-source protocol; pairs disclosure with FOSS posture |

**arXiv endorsement (critical for an independent author):** new submitters need a positive endorsement per category. Auto-endorsement is affiliation-biased; an independent author will likely need manual endorsement from an established cs.NI/cs.CR/cs.DC author. arXiv updated its endorsement policy Jan 2026 — read it first. **Highest-leverage move: an academic co-author** — solves endorsement and strengthens every reviewed venue. `[UNVERIFIED]` legal/patent weight of arXiv timestamps vs. jurisdiction — confirm with counsel if patent *defense* (not just disclosure) is a goal.

---

## Prioritized shortlist

**Go first — validate foundations privately, while pre-launch:**
1. **iroh Discord + GitHub Discussions** — transport feedback, home community, n0 amplification.
2. **OpenMLS / MLS-WG** — "is my crypto right" sanity check before broadcasting.
3. **Willow/Earthstar/p2panda + Malleable Systems forum** — sharpest peers on data model / membership / sync.
4. **Submit an individual IETF Internet-Draft** (`draft-<name>-croft-architecture`) — locks in the prior-art timestamp now; free, self-serve, examiner-known.

**Flagship public moment:**
5. **Local-First Conf 2026 CfP (open)** — Berlin, Jul 12–14; theme explicitly names P2P + self-sovereign identity + ATproto. Pairs with **DWeb Camp** (Berlin, Jul 8–12) in one trip.

**Then reach — after a runnable demo exists:**
6. **arXiv preprint** (cs.NI + cs.CR/cs.DC) — line up an academic co-author now (solves endorsement, unlocks the conferences).
7. **awesome-iroh PR + atproto Touchers Discord/forum.**
8. **localfirstnews.com feature + localfirst.fm podcast pitch.**
9. **Show HN (with live demo) + lobste.rs (higher signal).**
10. **Bluesky/Mastodon posts.**

**Engage on an ongoing basis (per layer):** W3C Credentials CG (DID layer), IETF atproto WG + W3C Social Web WG (public-social layer), IRTF DINRG / ANRW '26 (decentralization thesis).

**Academic full paper (citable prior art + scrutiny):** PoPETs 2027 (rolling, most accessible; privacy framing) or NSDI '27 (Sep 10 abstract) if deployment/measurement data exists; HotNets/ANRW workshops first if design-stage.

**Optional formal defensive publication:** Technical Disclosure Commons (free) or IP.com (paid, strongest examiner discoverability) if blocking competitor patents is an explicit goal beyond what the I-D + arXiv timestamps already accomplish.

---

## Disclosure caveat (one-directional)

Every venue above is a **public disclosure**. The first I-D submission or community post starts the US patent grace clock and forecloses most foreign patent rights immediately. For a defensive posture (spread + use + block-others-patenting) this is the intended outcome — but the sequencing is one-way: once Croft is socialized, the option to patent it ourselves is effectively gone. This research assumes that defensive intent. This is general IP practice, not legal advice; confirm with counsel if a patent option is to be preserved for any component.

---

## Flagged unverified items

NSDI '27 full-paper date and spring round; HotNets 2026 and ANRW '26 exact deadlines; CoNEXT/IMC/FOCI summer-cycle dates relative to today; DID WG charter end-date conflict (Oct vs. Apr 2026); whether any 2026 WG charters P2P-QUIC NAT traversal; legal vs. academic weight of arXiv/I-D timestamps as patent prior art; ATmosphereConf next-edition dates; current per-subreddit activity; localfirstnews submission queue specifics.

---

## Sources

Community / practitioner
- iroh / n0: `github.com/n0-computer/iroh/discussions`, `github.com/n0-computer/awesome-iroh`.
- Local-First Conf 2026: `localfirstconf.com`, `localfirstconf.com/cfp`, `cfp-2026.localfirstconf.com`; localfirst.fm `localfirst.fm` (+ S3 conf special); `localfirstweb.dev`; Ink & Switch Patchwork `inkandswitch.com/project/patchwork`.
- DWeb Camp 2026: `dwebcamp.org/berlin-2026`, Internet Archive blog (Apr 2 2026).
- Willow/Earthstar: `forum.malleable.systems/t/the-willow-protocol/161`, `github.com/earthstar-project`. Spritely: `spritely.institute`, `community.spritely.institute`. Matrix: `matrix.org/category/news`. Protocol Labs governance: `protocol.ai/blog`.
- atproto community: `discord.atprotocol.dev`, `discourse.atprotocol.community`, `links.atprotocol.dev`, `atproto.com/blog/2026-spring-roadmap`, ATmosphereConf `atmosphereconf.org`.
- Secure messaging: OpenMLS `openmls.tech`.
- Reach: HN norms `github.com/minimaxir/hacker-news-undocumented`, `lucasfcosta.com/blog/hn-launch`; `localfirstnews.com`; TechCrunch "Beyond Bluesky" (Jun 2025).

Standards
- IETF MLS: `datatracker.ietf.org/wg/mls/about`, `draft-ietf-mls-extensions-09`, `draft-ietf-mls-partial`. MIMI: `datatracker.ietf.org/group/mimi/about`, `draft-ietf-mimi-arch`. atproto WG: `datatracker.ietf.org/doc/charter-ietf-atp`, `atproto.com/blog/kicking-off-the-atp-working-group`. DINRG: `datatracker.ietf.org/rg/dinrg/about`, `irtf.org/dinrg.html`. MASQUE: `datatracker.ietf.org/doc/draft-ietf-masque-quic-proxy`. GNAP (concluded): `datatracker.ietf.org/wg/gnap/about`. I-D submission: `datatracker.ietf.org/submit`, `authors.ietf.org/submitting-your-internet-draft`.
- W3C: Credentials CG `w3.org/community/credentials`; Social Web WG charter `w3.org/2026/01/social-web-wg-charter.html`, `socialwebfoundation.org/2026/01/15`; DID WG `w3.org/groups/wg/did`, `w3.org/TR/did-1.1`; LWS `w3.org/2024/09/linked-web-storage-wg-charter.html`; CRDT4RDF CG (closed) `w3.org/community/crdt4rdf`. RFC 9420 / RFC 9750.

Academic
- `usenix.org/conference/nsdi27`, `usenix.org/conference/usenixsecurity27`, `sp2027.ieee-security.org/cfpapers.html`, `petsymposium.org/cfp27.php`, `conferences.sigcomm.org/co-next/2026`, `conferences.sigcomm.org/imc/2026`, `sigcomm.org/events/hotnets-workshop`, `irtf.org/anrw/2026/cfp.html`, `foci.community`.

Defensive publication
- `info.arxiv.org/help/submit`, `info.arxiv.org/help/endorsement.html`, `blog.arxiv.org/2026/01/21` (endorsement policy update); IP.com `kb.ip.com/pad`; `wiki.endsoftwarepatents.org/wiki/Defensive_publication_and_prior_art_databases`; Linux Defenders `lwn.net/Articles/505030`.
