# The sovereign PDS/AppView "club": what owning the read-layer unlocks

date: 2026-06-22

status: research deliverable (analytical lens). Source dialogue:
`../seeds/transcripts/raw/croft-atproto-sovereign-appview-open-social-dialogue-2026-06-22.md`; every
substantive claim web-verified in its `-FACTCHECK.md` companion (2026-06-22, five passes). Related
**projects/tools** are registered in `../ECOSYSTEM.md` §5e/§5f — this is the *analysis*. Pairs with
`atproto-private-data-architecture.md` (the private-data WG / host-trust line) and `public-social-protocols.md`.

purpose: the user flagged this body as *"esp good — bluesky private blocking with our own appview as a
feature."* It maps a buildable shape of Croft's own stance: own the **AppView** (the read/index layer)
and the **PDS** (the data/write layer) for a small group, and a long list of features the attention
economy withholds becomes available — without leaving the global network. Headline: **the PDS/AppView
separation is leverage, and almost every capability below is a direct expression of a Croft principle we
already hold.**

---

## 1. The core insight: experience-shaping, not content-shield

Bluesky's own words: *"Blocks on Bluesky are public… a thorn in our side… a tension with the structure
and values of the protocol, and might persist even with private data features."* The dialogue works out
why, and lands somewhere Croft already stands:

- **Blocking splits in two.** *Inbound* (blocking them from your sight) is **100% effective and private**
  if you own the AppView — you drop their posts/replies/likes/quotes before they reach your client, and
  the blocklist is a private DB row, **un-scrapable** (vs Bluesky's public `app.bsky.graph.block` ledger
  that third-party tools scrape). *Outbound* (stopping them seeing your public posts) is **structurally
  impossible** while federated — public data broadcasts to the firehose; incognito/sockpuppet/API defeat
  it. X conceded this: blocked users can now *view* but not *interact*.
- **So the power is experience-shaping.** A **local shadow ban / "black hole"**: the target's reply
  "succeeds" (mirage of engagement) while your AppView silently discards it — no notification, no thread
  branch, no quote-amplification. You strip their ability to *touch your corner of the network or get a
  reaction*, rather than chasing the impossible goal of blinding them.

**Croft tie-in.** This is the **social-layer** thesis restated from the read side: *openness caps
propagation; a large public group is a visibility sink, not a conduit; inward visibility and outward
propagation are independent parameters.* "Can't hide public data, but can refuse to amplify or react" is
the same shape as "structural, not runtime, enforcement" — you don't beg the sender's client to behave,
you control what your own infrastructure renders.

## 2. The feature list — and the principle each one expresses

| Capability (from owning PDS+AppView) | Croft principle it expresses |
|---|---|
| **Off-repo private data + internal-only feeds** (custom `com.x.*` lexicons that never broadcast to the Relay; "watercooler" feed) | Host-untrusted / **blind-broker**; "born-into-a-regime, can't silently change visibility" (COHESION §26; social-layer V3) |
| **Encrypted blob vault** (client-side AES-GCM → `uploadBlob` → Relays mirror ciphertext → outside `getBlob` = garbage; the public net as a free encrypted hard drive) | **content-blind mule**; the `encrypted-blob-share` spike |
| **Asymmetrical federation** ("gated castle" — ingest the whole firehose, broadcast nothing internal) | scoped visibility, quiet membership (S1/S3); "different, not weaker" |
| **Private cooperative Labelers** (pod-wide, crowdsourced moderation tags, hidden from the web) | the **geer** gating-peer (label-not-enforce separation); "never algorithmically adjudicate a social dispute" |
| **Collective-defense muting** ("blast radius" — auto-block the whole branch of a dogpile, silently) | the trap-door / re-formation backstop; freeze-by-default |
| **Multi-source AppView** (ATProto + ActivityPub + Nostr + RSS into one timeline; AppView is source-agnostic by CQRS) | **honest-seams ponds** made concrete at the index layer |
| **CAR/MST offline mesh** (signed records sync P2P over BLE/Wi-Fi-Direct; a local micro-AppView rebuilds the timeline; re-federate on reconnect) | the iroh/lineage CAR-sync + `ios-opportunistic-p2p` work |
| **Zero-knowledge discovery** (local AI ranks the firehose on-device; no behavioral tracking leaves your hardware) | **non-extraction**; user-need-first |
| **Dwell-time / passive "Nielsen" curation** (surface what was genuinely read, not what provoked a reply) | Tier-3 "shapeability + stability; constant UI change is quietly extractive" |

**Two honesty caveats** (per FACTCHECK): the **offline-mesh** "two devices auto-sync while locked"
inherits the *same aspirational caveat* the iroh FACTCHECK already flagged (OS kills background P2P;
CoreBluetooth doesn't relaunch on new-advertiser) — the CAR/MST *mechanism* is sound, the unattended-wake
is not a given. And the **dual-PDS** "one identity, two servers" is **not native** (a DID maps to one PDS
+ one repo signing key, one MST); you achieve it via DID-document **sidecar service endpoints** or
**off-repo storage**, not delegate keys.

## 3. The attention-economy frame (the user's "Nielsen vs Meta manipulation")

The user is explicit: *leery of attention traps and the race to the bottom for rage-bait; this must be
about community, not extraction.* Meta dynamically *alters reality* to exploit the nervous system (drops
infuriating content to force a reply → notification loop). A **Nielsen-style** model treats attention as
finite and precious — *passively sample* what the community values and bubble it up **without mutating
behavior**. Owning the AppView is what makes the Nielsen posture buildable: a "hype dial / toxicity
slider," thread-summaries, quiet collective bookmarks ("the collective brain"), symmetric incognito
presence. This is the product-layer expression of `principles.md` Tier-1 (**non-extraction is the
point**) and Tier-3 (**shapeability paired with stability**) — and it is the antidote to the Fediverse's
own usability flaws the dialogue catalogues (the Mastodon "island blind spot," the "dictator admin,"
push-fan-out cost): **global awareness with local sovereign filtering, and a credible exit so organizers
can't trap anyone.**

## 4. Twitter Circles — a cautionary proof, not trivia

Circles (Aug 2022 → Oct 31 2023) is independent corroboration of a Croft invariant. It tried to gate
*public-plane* posts with a visibility clause; when the ranking logic changed, private posts **leaked**
to strangers' "For You" feeds — and the feature died. Communities (subreddit-style; killed ~May 2026,
<0.4% users / ~80% of spam reports) failed too; **Group DMs/XChat won by being a hard binary — fully
public, or fully E2EE-isolated.** The lesson is Croft's **social-layer V3 / structural-not-runtime**
principle, proven by a billion-user failure: *don't build a semi-private overlay on a public broadcast
plane; make private data **structurally** private (off-repo / E2EE), or it leaks.* Cite this wherever
social-layer argues visibility regimes.

## 5. What we take / leave / watch

- **Take (build-on):** the **AppView-as-private-gatekeeper** pattern; off-repo private feeds + encrypted
  blobs; multi-source ingestion (the index-layer ponds); the ready-to-hack AppViews (**AppViewLite** lean
  C#, **Blacksky/rsky-wintermute** Rust + private-community scaffolding, **Zeppelin** full Docker stack);
  **Groundmist** as the closest living local-first-private relative; the **AP↔AT bridge** (Bridgy Fed /
  A New Social) + **Bounce** as a credible-exit instrument; client bases **Ouranos** (web), **Heron**
  (offline-first KMP), **atcute** (from-scratch TS). All registered in ECOSYSTEM §5e/§5f.
- **Leave / watch:** the **PDS hacks** that depend on the official SQLite schema (Lexicon-Firewall
  middleware, Ghost-Delete cache trick, SQL-injection blob registration) are **fragile** — prototype,
  don't enshrine (ties to the E23 file-proxy item). The offline-mesh unattended-wake caveat above.
- **Surface, don't resolve:** the "club as a service" / managed-PDS framing is real demand but
  for-profit-shaped; Croft's stance is cooperative — feeds the sustainability↔mechanism question
  (open-considerations §8; the §25/§26 PDS-economics items). Groundmist's "private by default" is
  currently *intent*, not security reality (its sync server ships with auth disabled).

## Design conclusions (pointers)

- The "experience-shaping / structural-not-runtime" corroboration belongs alongside `../thinking/social-layer.md`
  (V3 regimes) and `../crystallized/principles.md`; tracked in `../COHESION.md` §28.
- Related register: `../ECOSYSTEM.md` §5e (PDS implementations/hosts/blob backends) + §5f (bridge +
  AppView + client tooling). Companion analysis: `atproto-private-data-architecture.md`.
- Naming candidates from Part 1 (Till/Tillage + the open-ecosystem set) are a **reservoir** in
  `../NAMING.md`, not adopted.
- Term hygiene: A New Social is a "nonprofit" (not confirmed 501(c)(3)); it's **Free Our Feeds** (not "AT
  Community Fund"); Series B closed 2025 / disclosed 2026; don't pin the Aggregation Theory quote to a
  dated essay.
