# 03 — The living ecosystem and how the options compare

date: 2026-06-24

status: synthesis (spine-complete).

verification: for atproto / iroh / iOS facts the source of truth is the FACTCHECK SoT — those facts are
cited, not re-verified. Figures for private companies (Discord, Bluesky valuations) are third-party
estimates, carried `[UNVERIFIED]`. Dialogue-sourced rows are flagged as such.

---

## Theme narrative (overview)

The present-day field is best read as a set of honest trades, not a ladder of better-to-worse. The central
finding holds across every system examined: **no deployed system delivers usability, decentralization, and
metadata protection simultaneously** — each buys one or two by spending the third. Signal buys world-class
UX with centralization and phone-rooted identity; SSB bought pure P2P and paid with multi-device hell,
recovery dead-ends, and unbounded logs; Briar buys the strongest threat model and refuses multi-device and
recovery outright; Delta Chat rides email for free reach and inherits email's metadata leak. The deeper
version is a *four-property impossibility*: group moderation + multi-device + PFS + offline-mesh cannot all
hold without an unequal, privileged peer. Croft does not dodge this — it makes the trade openly (the
meer/superpeer as an unequal-in-capability-but-not-in-rights sequencer; see `drystone-spec`, `04`, `06`).

On the public-social side the comparison narrows to one anchor reason: an atproto DID is a portable,
self-authenticating, user-held identifier that can serve as the *same* identity primitive on the public
side (atproto-native records) and the private side (the root to which MLS credentials and group membership
bind). ActivityPub's `@user@instance` identity is instance-bound, server-key-held, and migration-fragile,
so it could not serve that dual role. That dual-use identity is the load-bearing justification for the
atproto bet, and it survives scrutiny (its derivation is `05`).

The structural fact that *forces* Croft's architecture is atproto's public-by-default design: all repo data
is signed and broadcast; there are no ACLs in the core repo layer, and the spec warns against bolting
encryption onto a public append-broadcast tree. So the public/private split is necessary, not stylistic —
public records ride atproto, private messaging runs on the separate encrypted path (iroh + MLS +
Automerge), and the DID is the one element that legitimately spans both. Crucially, atproto's own private-
data work is converging on access-controlled, PDS-gated private data with the PDS as a *trusted agent*, and
treats true E2EE / zero-knowledge as the deferred, harder problem. Native-in-protocol E2EE still does not
exist; real atproto E2EE remains third-party (Germ's MLS, the XMTP bridge). Croft sits on the zero-knowledge
side of the exact line the atproto core team is reluctant to cross — so its host-untrusted MLS answer is
*more* differentiated now, not less.

Two reference points anchor the product framing. Discord sets the UX bar (it won on zero-friction joining)
and is structurally bad at durable, searchable, ownable knowledge while now under IPO-driven enshittification
pressure. Germ is the closest living cousin: an MLS E2EE messenger launched from a Bluesky profile, binding
identity via an "Anchor Key" in the public atproto profile — a shipped instance of "atproto for discovery,
E2EE off-repo." Finally, the sovereign PDS/AppView "club" reframes all of this from the read side: owning
the AppView and PDS for a small group unlocks a long list of capabilities the attention economy withholds —
without leaving the global network — and almost every one is a direct expression of a Croft principle.

## Charter — what this theme covers

**In scope.**

- The comparative field map across messaging and public-social, as a register of honest trades.
- "Different, not weaker": what it means concretely against each pole.
- The field's own *direction of travel* (atproto deferring E2EE; Discord's enshittification; the
  four-property impossibility as a universal toll) as the thing that sharpens Croft's position.
- Germ as the closest cousin; Discord as the UX bar; the three "own-your-data" poles (Solid / atproto / DSNP).
- The sovereign-AppView "club" as the read-side expression of the comparison.

**Out of scope (and where it lives).**

- The verification discipline / provenance-debt methodology itself → `drystone-spec`.
- The deep crypto-wars / Clearances / commons lineage → `02`.
- The Drystone wire spec, MLS lineage-groups proof internals, the meer mechanism → `04` (cited as
  *outcomes*, not mechanisms).
- did:plc resilience, did:webvh convergence, the recovery-anchor internals → `05` ("DID as dual-use anchor"
  is used here as a given).
- The cooperative/IP-stewardship mechanism, and Discord's captured-labor as a *cooperative* argument → `07`.

**Boundary calls.**

- The DID dual-use argument is *cited* here as the anchor reason for the public-social comparison; its
  identity-internals belong to `05`. The four-property impossibility and the meer-as-sequencer are *invoked*
  to explain why every system pays a toll somewhere; the meer's design lives in `04`/`06`. The Discord
  captured-moderator-labor framing is *noted* as the clean illustration of the extractive default, then
  handed to `07`.

## 1. The universal trade (the lens for the whole theme)

> "no system in the field delivers usability, decentralization, and metadata protection simultaneously"

*Verification:* **CONFIRMED** (corpus synthesis). The field's failures are real and instructive, not a foil
— "you must beat what is actually good." The deeper version, from the iOS work, is a **four-property
impossibility**: group moderation + multi-device + PFS + offline-mesh cannot coexist without an unequal,
privileged peer (PFS vs mesh-healing; moderation vs multi-device in a partition). MLS (RFC 9420) *assumes* a
Delivery Service for ordering; Croft's meer/superpeer is exactly that "unequal-in-capability, equal-in-
rights" sequencer.

## 2. The messaging field map

Signal (centralized, gold-standard UX, phone-rooted identity); SSB (the canonical cautionary tale —
single-device-tied keypair, no recovery, unbounded logs, weak forward secrecy); Matrix (production
decentralized-encrypted group chat, but Megolm metadata leak + federation-graph exposure + MLS-in-
distributed-topology unsolved); Briar (best metadata protection via Tor, *refuses* multi-device and recovery
by design); Delta Chat (the closest Rust+iroh cousin — validated the realtime-P2P / durable-store-and-forward
split, but transfer-then-diverge multi-device + email metadata leak); Session (no-phone keypair + mnemonic
recovery; dropped then restored PFS — the cautionary "don't trade away forward secrecy").

## 3. The public-social field map and the three "own-your-data" poles

X (proprietary, reach-only, no protocol); Bluesky/atproto (DID/PDS/relay/AppView/Lexicons split,
decentralized-by-design but centralized-in-practice via Bluesky defaults); Threads (centralized Meta product
with opt-in partial AP federation — the "walled garden bolts on open protocol" cautionary case);
Mastodon/ActivityPub (mature federation, app-type diversity via Pixelfed/PeerTube, but instance-bound
non-portable identity). The three "own your social data" poles, and Croft between them:

- **Solid** = private-by-default, *direct app↔Pod* access, RDF/WebID/Solid-OIDC.
- **atproto/PDS** = public-by-default, *indexed pipeline* (Relay→AppView), Lexicons.
- **DSNP** = a *blockchain consensus layer* (reference chain Frequency/Polkadot) holding the graph.

Croft is **none and borrows from each**: it rides atproto for public social, adds an E2EE private layer
(lineage-groups MLS) that neither Solid (app-mediated ACLs only) nor atproto (not native) provides, and
**rejects the chain** DSNP requires while sharing DSNP's unbundle-the-social-web + delegation-without-
surrendering-keys goals. Solid/WebID/Solid-OIDC/DPoP RFC 9449 and DSNP are CONFIRMED, verified-dialogue.

The test 03 applies to every pole here is the **"credibly decentralized but operationally centralized"
trap** — cryptographic portability that is technically real but economically meaningless because
aggregation re-centralizes, so the data is portable on paper while the network that gives it value is not.
(The principle itself is derived in `drystone-spec`; this theme only uses it as the yardstick.) Atproto is the clearest
case to hold to it: DIDs and self-authenticating repos make identity genuinely portable, yet if relays,
AppViews, and defaults concentrate, the portability buys little in practice. A pole passes only if it
**survives as small self-hosted nodes** — that is the line between credible decentralization and the trap.

## 4. The anchor reason — dual-use identity — and why the split is forced

The atproto DID is the *same* host-independent, user-held identity primitive on both the public and private
sides; ActivityPub's instance-bound, server-key-held identity could not serve that dual role without a
parallel system. This is the load-bearing justification for the atproto bet (its derivation is `05`). And
the architecture is *forced* by atproto's design:

> "All atproto repo data is public by design today." — and the spec warns against "bolting on" encryption to
> the public content-addressed tree.

*Verification:* **CONFIRMED** (cite FACTCHECK SoT — MST is atproto's structure). So public → atproto, private
→ encrypted P2P path, identity → the shared DID. Live evidence sharpens it: custom NSIDs
propagate on Jetstream with no pre-registration, and the full DID→signing-key→signed-commit→MST-root→CID→
bytes chain verifies with zero trust in the PDS/relay — **atproto gives cryptographic trust for free, zero
semantic trust** (own your schema, threading, moderation policy).

## 5. "Different, not weaker," concretely

Croft's claimable differentiators are where privacy and ease coincide: genuine forward secrecy by default
(X conceded X Chat has none — server-held Juicebox-recoverable keys); encrypted group metadata (X makes
group name/icon public); multiple unlinkable identities as a base feature; a deliberately-drawn public/
private boundary (not leaked-at, à la Twitter Circles); CRDT conflict-free concurrent group state (no field
competitor offers a shared concurrently-editable group document); true serverless when co-present without
SSB's UX cliff; and user-run, blind, self-hostable infrastructure. *(Harvests the germ-xchat inversion: the
privacy-preserving behaviour is the free one; the convenience behaviour is the effortful one.)*

"Different, not weaker" is a claim that can rot into an excuse, so it carries an obligation: it must be
*backed by a per-tier security properties matrix* — forward secrecy, post-compromise security, metadata
protection, offline capability, and central-compulsion-resistance, evaluated across the tiers (baseline /
standard / high). The matrix is the discipline, not a slogan: read down each tier and the trades are
explicit, so "different" can never quietly rationalize "weaker." The artifact itself need not be drawn here
to make the point; what matters is that the obligation is named — every tier owes an honest row, and a tier
that drops a property has to say which one and why.

Two of those properties are ones the centralized field structurally *cannot* match, and they are worth
claiming explicitly rather than burying in the matrix. The first is **transparent offline**: two phones can
sync with zero internet — co-present devices reconcile directly, no server in the loop — which a
phone-and-datacentre architecture cannot do by construction. The second is **no central operator to
compel**: a centralized messenger's strongest weakness is organizational, not cryptographic — one throat to
choke. Croft has no such throat. These are not "decentralized, therefore weaker" concessions; they are
positive properties the trusted-host designs decline to offer, and they are exactly where the matrix shows
Croft *ahead* rather than even.

## 6. The field's own direction sharpens the differentiation

> a real, community-led ATProto Private Data Working Group (Boris Mann; GitHub #3363 "Namespaces" →
> "buckets/realms", #121 "Encryption for private content"; Paul Frazee *informally*) is converging on
> access-controlled, PDS-gated private data (PDS as trusted agent) and **explicitly defers true E2EE /
> zero-knowledge**.

*Verification:* **CONFIRMED** (cite FACTCHECK SoT addendum). Croft already sits on the harder ZK side of that
exact line. The *fictional* "AT Messaging working group" stays REFUTED; this real
WG is a distinct, modest thing — and real atproto E2EE remains third-party. Germ is the closest living cousin:

> the first native-launched private messenger from a Bluesky profile (2026-02-18); cofounder/CTO Mark Xue
> (ex-Apple iMessage/FaceTime); MLS via the open AC Protocol (IETF `draft-xue-distributed-mls`); identity
> bound via an "Anchor Key" published in the atproto profile.

*Verification:* **CONFIRMED** (cite FACTCHECK SoT). A shipped "atproto for discovery, E2EE off-repo" idiom
that rhymes precisely with Croft's design.

## 7. Discord — the UX bar and the wedge

Discord won on zero-friction joining (the ten-second invite — no client / instance / server choice), a
beachhead feature, timing, and one cohesive brand; it is structurally bad at durable, searchable, ownable
knowledge and is under IPO-driven enshittification pressure (confidential S-1 filed January 6, 2026; ~$15B
target slipping; ads after nine years; an Oct 2025 breach exposing roughly 70,000 government-ID photos via a
third-party vendor). The founder anecdote that captures the UX gap:

> "over 95% of people stopped using the Matrix channels within a month" and moved to Discord.
> — the Discourse founder, on running both for over a year

*Verification:* **CONFIRMED** (founder's own account); the IPO/valuation figures are third-party
`[UNVERIFIED]` (Discord is private). The wedge: be the durable, searchable, ownable system of record + match
the live-presence core, and tell a longevity/ownership brand Discord can't. This grounds the tier-zero
deep-link resolver (E11) and the membership-vs-access split (developed in `06`/`08`).

## 8. The sovereign-AppView "club" — the read-side expression

Owning the AppView (read/index) + PDS (write) for a small group unlocks: private/un-scrapable *inbound*
blocking (outbound is structurally impossible while federated → experience-shaping "local shadow ban");
off-repo internal feeds; an encrypted-blob vault (the public net as a free encrypted hard drive); asymmetric
"gated-castle" federation; private cooperative labelers (= the *geer*, `06`); a multi-source AppView
(atproto + AP + Nostr + RSS = honest-seams ponds at the index layer, `08`); CAR/MST offline mesh. Each maps
to a held Croft principle. The cautionary proof that backs it:

> Twitter Circles (Aug 2022 → Oct 31 2023): private posts **leaked** to strangers' "For You" feeds when
> ranking logic changed, then died.

*Verification:* **dialogue-sourced, verified**. *Grounds:* backported as social-layer
invariant **S5** — private must be structural, not a runtime gate (`06`). *Honesty caveats:* offline-mesh
unattended-wake inherits the "OS kills background P2P" caveat (cite FACTCHECK SoT); dual-PDS "one identity,
two servers" is *not native* (sidecar service endpoints / off-repo, not delegate keys).

## 9. Substrate corroboration from the field

The iroh+Automerge+MLS stack is not a lone bet:

> Peat (Defense Unicorns) = Croft's exact substrate (Rust + iroh QUIC/BLE + Automerge CRDTs + MLS), proven
> in denied/degraded/contested tactical conditions.

*Verification:* **CONFIRMED** ("real despite smelling fabricated"). The strongest external validation that
the substrate bet is sound and survivable off-grid. Some corroborating rows are flagged dialogue-sourced;
others are dialogue-surfaced but web-verified in their FACTCHECK companions.

---

## What this theme establishes (and does not)

**Establishes:** Croft's position is "different, not weaker" against a field where every option pays a toll
somewhere; the public/private split is forced by atproto's public-by-default design; the field's own
direction (atproto deferring E2EE, Discord enshittifying) sharpens rather than erodes the differentiation;
Germ is the shipped proof-of-shape and Discord the UX bar to clear.

**Does not establish:** the private-company figures (Discord/Bluesky valuations, DAU) as anything but
third-party estimates; nor a settled outcome for atproto's Permissioned Data work — the single most
important external development to track, which could narrow or complement Croft's private path.
