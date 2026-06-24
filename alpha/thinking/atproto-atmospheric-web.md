# AT Proto "atmospheric web": web-of-docs / Neo-GeoCities / open-LinkedIn

date: 2026-06-22

status: exploratory thinking, distilled from a fact-checked Gemini dialogue
(`seeds/transcripts/raw/atproto-atmospheric-web-iroh-mobile-dialogue.md` +
`...-FACTCHECK.md`). Only CONFIRMED/PARTLY claims carried here, flagged inline. This is an
*adjacent product surface*, not a Croft commitment — see "Where this sits for Croft" below.

---

## The idea

Define a lightweight document **Lexicon** on AT Proto (a `net.retro.page` / `net.geocities.page`
shape: title, body as a safe Markdown/HTML subset, links to DIDs/records) and render it with a
purpose-built **App View + client**. Because records live in the user's PDS as content-addressed
data, the "pages" are portable and host-independent — the cozy late-90s indie web, on
self-sovereign rails, with **composable moderation** (Ozone labelers as opt-in "curated web
rings") and **feed generators** for discovery. A web gateway (e.g. `geocities.pub/<did>/<page>`)
can compile records into safe static HTML5/CSS for the normal web — declaration, not arbitrary
JS execution (XSS/durability hazard). State/identity replace cookies (DID-signed tokens; a
guestbook signing is just writing a record). Same pattern extends to a **GeoCities 2027**
(themed "neighborhoods" via a profile tag the indexer groups on) and an **open-LinkedIn**
(`net.work.profile/experience/endorsement`, portable connections as follow/link records).

## What's real (verified ground it rests on)

- **"Atmospheric web"** is a genuine AT-Proto community term [CONFIRMED]. Building blocks are
  real: **lexicons**, **Ozone** signed labelers (moderation decoupled from hosting),
  **feed generators** (firehose → URI skeletons), **did:plc/did:web** + domain handles,
  **XRPC**, and a **free WebSocket firehose** [all CONFIRMED — atproto.com].
- Live precedent apps exist (the "smashed-together homestead" already exists in pieces):
  **Tangled** (Git), **WhiteWind**/**Leaflet**/**Standard.site** (long-form), **Semble**
  (link/knowledge curation), **Smoke Signal** (events), **Streamplace** (livestream), **Flashes**
  (photos), **npmx**, Automattic's **ATmosphere** WordPress bridge, **Blacksky** (independent
  infra) [CONFIRMED]. See `ECOSYSTEM.md §5b`.
- **Private AppViews** are a real architecture (index the firehose for a custom lexicon; gate by
  token/ACL) and a plausible business surface (B2B-compliance, vertical marketplaces, paid
  curation, signal feeds). Cost split is real: a **custom micro-lexicon AppView** is cheap
  (small VPS) because you discard 99.9% of the firehose; a **full-network** mirror is heavy —
  **Zeppelin** documented ~16 TB / ~$200-mo before being **decommissioned** [CONFIRMED; the
  decommission is the cautionary footnote]. A **hybrid client** routing `app.bsky.*` to the
  public AppView and `net.*` to your own is sound (XRPC = plain HTTP) [CONFIRMED pattern].

## What was wrong or overstated (do not carry as fact)

- **No native AT-Proto E2EE / private groups.** The dialogue's "AT Messaging MLS working group"
  is **REFUTED** — private messaging/groups are **third-party today**: **Germ DM** (MLS) and the
  **XMTP↔Bluesky bridge**. For the open-LinkedIn vein, "recruiter DMs / private alumni groups"
  has no native protocol answer yet.
- **Semble is a research knowledge network, not a Linktree clone**; **Standard.site/Leaflet are
  not Markdown-primary** (block-based) [PARTLY].
- The official Bluesky client won't *render* custom lexicons, but "hardcoded to one AppView /
  ignores them" is too strong — unknown fields pass through; the "open union" shows an empty
  embed, with a generic fallback only *planned* [PARTLY].

## Where this sits for Croft

This is an **adjacent surface**, not the Croft messaging/lineage core. Its relevance:

1. **The private-groups gap is Croft's thesis.** Every serious version of this (open-LinkedIn,
   closed neighborhoods, B2B) hits the wall the dialogue hit: AT Proto is public-by-default.
   Croft's **lineage-groups Phase-1 MLS proof** (GO on openmls 0.8.1) is precisely the missing
   private-group primitive. An atmospheric-web product is a *demand-side argument* for the crypto
   Croft already built. (Germ is the closest atproto+MLS cousin — `ECOSYSTEM.md §6`.)
2. **The "spoonful of sugar" framing** (delightful client over rugged self-sovereign data) is the
   same adoption thesis as Croft's — see `research/p2p-founder-motivations-adoption.md`.
3. **Public path already proven here:** PR #4 (public-roundtrip) and PR #6 (appview-validation)
   are live AT-Proto experiments; the "web of docs" rides the same public lexicon/firehose path.

## Open edges

- Recovery/identity (did:plc centralization vector) is unsolved here too — the same open problem
  flagged across the corpus (`thinking/plc-identity-resilience.md`).
- Private neighborhoods/groups need the MLS layer; "token-gated App View" curation ≠ encryption.
- Gateway-renders-HTML safely is a real XSS surface to design (declaration-not-execution).
