# Croft.ing: the website, the plot, and the serverless homestead

date: 2026-07-10

status: product-surface synthesis. The Plot·Shed·Gate model, the single-renderer resolution pattern, and
the serverless social mapping are settled design intent; the plot-tender widget set is a curated catalog of
design intent; the open product calls (abuse model, HTML/CSS sandbox safety, the group-of-widgets scope) are
surfaced below, not resolved.

verification: the atproto mechanics this surface rides on — `did:plc` (immutable), the public
unauthenticated AppView (`public.api.bsky.app`), `com.atproto.identity.resolveHandle`, PDS blob storage +
CDN, custom feed generators, the firehose/relay — are settled protocol facts; cite the source-of-truth
FACTCHECK (`../../alpha/seeds/transcripts/raw/atproto-atmospheric-web-iroh-mobile-FACTCHECK.md`), do not
re-verify. Genuinely new claims carried here are flagged `[confirm before publish]` inline: the platform
rug-pull figures, the exact XRPC method names, and the third-party project mechanics named as prior art.

---

## Overview

This doc holds the shape of **Croft.ing**: the public web surface of the Croft product and the personal
"plot" a crofter tends. Where the sibling `product-the-garden-of-ponds.md` covers the garden of ponds and
pads on the shared client core, this doc covers the *homestead surface* — the website a newcomer meets, the
personal plot each crofter grows, and the deliberately serverless architecture that lets both stand for
decades at near-zero cost.

The through-line is a reframe carried into the visual and technical design: a domain name is a **beautified
pointer, not a container**. A crofter's plot is a safety-deposit box; whether it is opened through the
master-key URL (`plot.croft.ing?user=…`) or through a personal gate (`name.croft.ing`), it is the same box
with the same contents and the same capabilities — one has a cleaner address, that is the only difference.
Identity, data, and the social gravity around a plot live on the open protocol, not in a database this
product owns. That is what makes the homestead un-rug-pullable, and it is why the whole surface can be a
static single-page/progressive-web app with no backend of its own.

## 1. The spatial model: Plot · Shed · Gate

The product's geography mirrors the natural rhythm of a person with their own place: a home, a workshop,
and the road out.

```
[ THE SHED ]   private workspace   — offline writing, local backups, drafts that compost
      |
[ THE PLOT ]   public sanctuary    — a hand-drawn sketch that blooms into watercolor as it is tended
      |
[ THE GATE ]   the shared commons  — the road to the valley (atproto/Bluesky): lanterns, notes, crops
```

- **The Shed** is the private, local-first workspace. It is quiet and distraction-free; it respects focus,
  battery, and the possibility of being offline. Writing happens here in private, and only sturdy work is
  carried out into the light of the Plot. Vocabulary follows the metaphor: not "Publish" but **Plant** /
  **Tend**; not "Drafts" but **In the Shed** / **Composting**.
- **The Plot** is the public home page, rendered as a cozy hand-drawn *sketch* on first arrival — not
  "broken" or "empty" but a quiet pencil draft waiting for ink. As the crofter tends it (a first post, a
  lantern status, links along the wall, chosen pigments), elements transition from graphite line-drawing to
  saturated watercolour. The empty state is **Fallow Soil** ("this soil is resting"), reframing an
  uncustomised plot from a failure of content into a healthy phase of preparation.
- **The Gate** is the threshold where the private plot meets the public road. Sign-in is framed as walking
  through the gate ("Approach the Gate" / "Enter"), and the road itself is the atproto network (surfaced to
  newcomers as Bluesky — the "mailbox at the edge of the property").

The spatial model is the product's organising frame, not decoration: it dictates where each capability
lives (introspection in the Shed, expression on the Plot, connection at the Gate) and keeps the surface
legible without an algorithmic feed.

## 2. The single-renderer "safety deposit box"

There is exactly one rendering path, and its only job on load is to determine the **target DID**, then fetch
that repository and paint the canvas. The domain is an alias resolved to the same DID; there is no separate
"preview" state and no duplicate code.

Resolution accepts a handle *or* a DID, from a query param, a path, or the host:

```js
// One resolver, four cases. If it starts with 'did:', use it; else resolve the handle.
//   plot.croft.ing?user=alice.bsky.social   (friendly)
//   plot.croft.ing?did=did:plc:1234...      (durable — survives handle changes)
//   plot.croft.ing/did/did:plc:1234...
//   name.croft.ing                          (custom alias → resolve host to DID)
const res = await fetch(
  `https://public.api.bsky.app/xrpc/com.atproto.identity.resolveHandle?handle=${encodeURIComponent(clean)}`
);
const { did } = await res.json();
```

This buys three product properties that matter:

1. **No onboarding limbo.** A crofter is authenticated (has a DID) long before their `name.croft.ing`
   subdomain exists in DNS. The `plot.croft.ing?user=…` path gives them a live, fully interactive plot the
   instant they sign up — no waiting on DNS propagation, no sterile "Server Not Found."
2. **Shareability before the address exists.** They can share `plot.croft.ing?user=…` immediately;
   when the custom domain later propagates, the *same records* load at `name.croft.ing` with no migration
   and no broken links.
3. **Graceful transport fallback.** `plot.croft.ing` is always reachable under the wildcard `*.croft.ing`
   certificate even when a custom domain has a bad DNS or SSL day.

The `did:`-prefixed form is the durable link: because the DID is immutable, an old `?did=` URL never breaks
when a crofter changes their handle. [DID resolution and the public AppView are settled; cite FACTCHECK SoT.
The `resolveHandle` XRPC name is `[confirm before publish]`.]

## 3. The serverless social engine

The homestead has no database and no indexer of its own. Every social capability maps onto a **standard**
atproto record against an **Anchor Post** (one Bluesky post per Croft page, whose AT-URI is stored in the
page's frontmatter), so the already-scaled public Bluesky AppView does all the aggregation work for free.
This is the deliberate rejection of a custom AppView: running a bespoke indexer is the exact
high-bandwidth, high-cost failure mode that sank centralised predecessors (see §6), and it would betray the
"keep it simple, un-rug-pullable" stance.

| Croft feature | Physical metaphor | Standard atproto mapping |
|---|---|---|
| Footprints | a visitor count / hit counter | a like (`app.bsky.feed.like`) on the Anchor Post; read via `getLikes` |
| The Guestbook / Gate Ledger | an inn register | replies (`app.bsky.feed.post`, parent/root = anchor); read via `getPostThread` |
| Media (the Image Shed) | a photo drawer | uploaded as **blobs to the crofter's own PDS**, served via the public CDN |
| Themes (Seeds) | seed packets you pass on | the plot's CSS/palette/layout serialised as a lightweight JSON record |
| Profile card / link-in-bio | a signpost | `getProfile` on the resolved DID (avatar, banner, bio) — public read, no OAuth |

Two consequences fall out of this mapping. First, **spam is expensive**: leaving a footprint or a note
requires an authenticated PDS write, so bots cannot cheaply inflate a counter. Second, the surface is
**dual-platform for free**: a note left on the static site *is* a Bluesky reply and appears in the timeline;
a like there *is* a footprint here. The plot and the social network feed each other with no extra plumbing.
[XRPC method names `getLikes` / `getPostThread` / `getProfile` are `[confirm before publish]`; blob + CDN
and firehose/relay mechanics: cite FACTCHECK SoT.]

**Open product call (surfaced, not resolved):** routing footprints/guestbook through *public* XRPC writes
means the abuse surface is whatever the underlying network's moderation gives, plus whatever the plot
chooses to filter client-side (e.g. valid-DID checks, allow/deny by Stone Circle). The abuse/moderation
model for public-write widgets is an open design decision.

## 4. The plot-tender delights (catalog of design intent)

The point of the homestead is not to invent the next platform mechanic but to **persist the prior great
things** of the personal web, made durable and sovereign by living on the PDS. The catalog below is design
intent, curated from the resurrected Web-1.0/1.5 patterns; it is not a committed build list, and the scope
(which ship in the default template vs. as optional widgets) is itself an open call.

- **The Lantern Vigil** *(the anchor delight)* — working in the Shed lights a flickering CSS lantern on the
  public Plot, with a quiet self-set status ("Mending a fence"). It replaces the anxiety of the "green
  online dot" (which demands an instant reply) with ambient, zero-pressure co-presence. It extends to
  **Pulling Up a Stool** (a friend joins a lit Shed over a serverless WebRTC data channel — a shared quiet
  workspace, no video/audio) and to a **valley-at-night Commons map** where lit lanterns glow and a click
  sends a silent drifting "moth."
- **The Stile** — the classic webring reborn: a footer banner over a `stile` list of neighbour plots, a
  literal walkable path through the valley, rather than an algorithmic feed.
- **The Stone Circle** — the MySpace "Top 8" as a cluster of interlocking stones; hover reveals a
  neighbour's lantern/Sundial status. Cooperative valley, not popularity contest.
- **The Specimen Book** — instead of a bare comment, a visitor "presses" a signed local plant or stone
  (native to their timezone/season) into the sidebar: "a sprig of heather, pressed by a neighbour on a rainy
  morning."
- **The Greenhouse / Passing Seeds** — CSS custom properties as literal cuttings: "take a cutting" grafts a
  neighbour's palette or border style (stored as JSON on their PDS) onto your own; web design as
  cooperative propagation.
- **The interactive homestead map** — SVG-overlay navigation (click the wall → the Stile, the gate → the
  Ledger, the chimney → the Hearth ambient audio, the garden patch → the posts), responsive via CSS grid so
  it scales to mobile — navigation as exploration of a space.
- **Dynamic environments** — the plot's palette shifts by the *creator's* local time and season (day /
  dusk / midnight; seasonal overlays), so a visitor glimpses a slice of the crofter's real climate.
- **Colored Ink** — a watercolour pigment palette (Soot, Ochre, Madder Root, Woad Blue, Lichen Yellow) with
  an SVG filter that warps crisp borders into organic ink-bled lines, over the tectonic granite/moss/ruddy
  base (the visual system lives in `../socialization/`).

These are cozy-by-design and privacy-respecting (e.g. the anti-analytics "Soil Quality" meter audits the
*code's* health, not the visitor). What makes them better than their 1990s ancestors is that the drafts,
grafted CSS, pressed specimens, and guestbook carvings are all signed records on the crofter's PDS — they
survive the death of any single viewer.

## 5. Integration into Bluesky and the main app

Because the plot is native to atproto, it is not an island; it threads back into daily Bluesky use.

- **The Gate Feed** — a tiny, free, standard feed generator filtering the firehose for `#crofting`,
  `croft.ing` links, or `.croft.ing` authors, pinnable in Bluesky as "The Croft Valley": a calm, slow-life
  timeline. Posts in it can carry Croft metadata ("a lantern is lit — step through the gate").
- **The Subdomain Signpost** — claiming `yourname.croft.ing` and setting it as the Bluesky handle makes the
  profile itself a verified signpost: `@yourname.croft.ing` is a cryptographic badge that this person has a
  plot in the valley. [Domain handles: cite FACTCHECK SoT.]
- **The Croft Starter Pack** — a Bluesky starter pack embedded in the PWA lobby so a newcomer follows the
  valley's people and pins the Gate Feed in one click, never landing on a blank lonely screen.
- **The Firehose Windmill** — an essay's Anchor Post replies rendered on the static page as leaves falling
  onto a garden path; the static site animated by the wind of the public network.

The net is a "serverless symphony": subdomains for identity, custom feeds for quiet curation, anchor posts
for conversation — a surface that feels deeply woven into a large social network at zero infrastructure cost.

## 6. Why serverless-and-sovereign is the whole point

The design is a direct response to the graveyard of personal-web platforms. The pattern repeats: a platform
invites people to build homes, then the landlord bulldozes them when the model shifts. GeoCities was bought
by Yahoo (~$3.7B, 1999), neglected, and deleted in 2009; MySpace's 2019 server migration lost roughly
twelve years of music/media (~50M tracks by ~14M artists); Cohost — modern, ad-free, cooperative, beloved
for its "CSS crimes" — ran a high-custom centralised network with no ads or VC, burned out, and shut down
on 2025-01-12. [All figures `[confirm before publish]`.]

The drystone answer is an "immune system": decouple identity and data from the hosting app. A crofter's
plot does not depend on this product surviving — identity is on the open protocol, data is on their own PDS
(and, via the Oak Desk, mirrored as human-readable Markdown on their local disk). If the `croft.ing` viewer
disappears, the crofter repoints their domain at another viewer and the same HTML/CSS renders unchanged.
That is the concrete meaning of "grow your own": the homestead is built to stand like a drystone wall —
simple, sovereign, and outliving any single host — not to scale to a billion users for an investor.

The prior-art landscape shows this is a live movement, not a solo bet: `spores.garden` (Hypha Co-op)
derives a user's palette, isoline background, and flower identicon from their immutable DID and lets people
"plant" flowers in each other's gardens; `Standard.site` (the `site.standard.document` lexicon) is a
PDS-native long-form publishing set Croft can adopt for blogging; `Linkat` and `Bio.blue` show the
link-in-bio card built purely on public reads; `kibun.social` shows a status lexicon for a live `/now`
board. [Third-party mechanics `[confirm before publish]`; Standard.site is tracked as adoptable prior art.]

## What this layer establishes (and does not)

Establishes the Croft.ing web surface and the personal-plot product: the Plot·Shed·Gate spatial model, the
single-renderer "safety deposit box" resolution pattern, the serverless social engine (footprints/guestbook/
media/themes mapped onto standard atproto records against an Anchor Post), the curated plot-tender delight
catalog, and the Bluesky integration surface. It establishes *why* the surface is serverless-and-sovereign
(the rug-pull graveyard and the drystone immune-system answer).

Does **not** define the neutral protocol or reference core (those are `../drystone-spec/` and `../impl/`);
does **not** voice or pitch the product for adoption, nor own the visual identity, motto, or logo (those are
`../socialization/`, including the tectonic palette, the "grow your own ___" motto, the elevator-pitch /
one-pager / library depth model, and the CROFT.ing wall-to-gate mark); and does **not** resolve the open
product calls it surfaces (the public-write abuse/moderation model, the user-HTML/CSS sandbox-safety
posture given the "no JavaScript" stance, and the ship-scope of the delight catalog).
