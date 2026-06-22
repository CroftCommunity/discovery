# Fact-check — AT Proto sovereign PDS/AppView + open-social naming/interop dialogue

date: 2026-06-22 · companion to `croft-atproto-sovereign-appview-open-social-dialogue-2026-06-22.md`

purpose: AI-generated (Gemini, flagged unreliable). Every substantive assertion web-verified 2026-06-22
via five parallel research passes (naming/etymology · Fediverse interop + bridge · sovereign PDS/AppView
projects · Bluesky VC + Aggregation Theory · fork-tail: clients + Twitter Circles). Verdicts:
**CONFIRMED** · **PARTLY** (real but mis-described) · **REFUTED** · **UNVERIFIABLE**. Source URLs in the
cluster tables (kept compact here; agents captured full URLs). atproto/iroh facts defer to the standing
source of truth `atproto-atmospheric-web-iroh-mobile-FACTCHECK.md`.

## Headline

**Unusually accurate for Gemini** — among the most grounded intakes in this corpus. Every named project
(Bridgy Fed/A New Social, Groundmist, AppViewLite, Blacksky/rsky-wintermute, Zeppelin AppView, Colibri,
Ouranos, Heron, atcute, PDS MOOver, goat, Jetstream) is **real and correctly attributed**; the funding
rounds, the Bridgy Fed cost/scale figures, the Aggregation Theory framing/quote, the Twitter Circles
trilogy, and Bluesky's own "blocks are public" statement all **check out**. The PDS/AppView
"sovereign-club" architecture (inbound-private/outbound-impossible blocking, off-repo private feeds,
encrypted blobs, CAR/MST offline mesh, multi-source AppView) is **architecturally sound** — the protocol
constraints it leans on are real. Residual drift is minor and noted below; none of it changes the design
payload.

## Errors / imprecisions not to carry forward

1. **"A New Social is a 501(c)(3)" — UNVERIFIED.** Sources say "nonprofit"; the specific tax designation
   isn't confirmed. Say "nonprofit," not "501(c)(3)."
2. **"AT Community Fund" — imprecise/unverified name.** #IndieSky, Eurosky (Modal Foundation), and
   **Free Our Feeds** are real; "AT Community Fund" appears to conflate Free Our Feeds grant activity.
3. **Rhizome = "root system" — PARTLY.** Botanically a horizontal **stem**, not a root system (the
   Deleuze & Guattari decentralization metaphor + ginger example are fine).
4. **Chinese rewilding connotation — PARTLY.** 野's pejorative load is "rude/uncivilized/barbaric," not
   specifically "wasteland/desolation/abandonment" (and 重新野化 is the more standard rendering than 再野化).
5. **Series B "2025" — PARTLY.** ~$100M / Bain Capital Crypto correct, but it **closed Apr 2025, was
   disclosed Mar 2026**; don't present as fresh 2025 news. (Series A ~$15M / Blockchain Capital / Oct 2024
   is exact.) **43M users** crossed in **early 2026**, not within 2025.
6. **X Communities shutdown "~May 30, 2026" — PARTLY.** Feature-end announced Apr 23, 2026; original date
   May 6, extended migration deadline May 30. Use "~May 2026."
7. **Heron "WriteQueue" — UNVERIFIED.** Heron's offline-first/Room architecture is confirmed; a component
   literally named "WriteQueue" wasn't independently confirmed (likely real, flag as unverified).
8. **Threads "you cannot have an account without Instagram" — PARTLY/outdated.** True at 2023 launch;
   standalone signup now exists in the EU/UK (DMA).
9. **Aggregation Theory quote — attribution correct, source-page caveat.** The "Zero distribution costs…"
   quote is genuinely Thompson's and tied to Aggregation Theory, but lives on Stratechery's canonical
   concept page, not verbatim in the dated 2015/2017 essays — don't pin it to a specific dated article.
10. **No-till "releases trapped carbon" — debated.** The framing is sound but the carbon-sequestration
    rationale for no-till is scientifically contested; treat as debated, not settled.

## Cluster 1 — Naming / etymology

CONFIRMED: Cathedral & Bazaar (Eric S. Raymond); Agora (Greek assembly/market); Rootstock; Radix (Latin
"root"); Taproot; *salvaje/sauvage* carry "savage" sense; rewilding ↔ fortress-conservation / green-
colonialism critique; rewilding politically polarizing in rural Europe (wolves/bears); Tillage =
soil-prep; *Labranza/Labourage* (honest labor); 耕作 (diligence); no-till championed in regenerative ag;
"till" = cash drawer (+ glacial sediment). **PARTLY:** Rhizome (stem not root — #3); Chinese rewilding
connotation (#4). All real; the naming candidates (Till/Tillage + open-ecosystem set) are filed as a
**reservoir** in `NAMING.md`, not adopted.

## Cluster 2 — Fediverse interop + AP↔AT bridge

CONFIRMED: Threads on standard W3C ActivityPub + ActivityStreams 2.0 (not a fork); proprietary closed
engine vs open Mastodon; Fediverse-sharing opt-in; 800+ servers defederated (Fedipact); FEPs for quote
posts etc. (Meta implements FEP-e232/misskey_quote); Bridgy Fed = main bi-directional bridge run by **A
New Social**; the `@bsky.brid.gy` / `@ap.brid.gy` routing; mission "people, not platforms"; Ryan Barrett
("snarfed") + ~2 + volunteers, grants+Patreon, no ads; **snarfed/bridgy-fed**, CC0, Python, Granary +
Arroba; **the cost/scale figures (2k→150k users; $0.15→$0.03, 5×; 2.3 TB / 700M rows; ¼-at-$1/mo)
match the "Bridging on a budget" post exactly**; Bounce (credible-exit migration); RSS Parrot + Pinhole.
**PARTLY:** "501(c)(3)" (#1); Threads-needs-Instagram (#8 — outdated); Fedisky ("ActivityPub extension
for Bluesky PDS," and "mostly superseded" is unsupported).

## Cluster 3 — Sovereign PDS/AppView architecture + projects (the design core)

CONFIRMED: **Blocks on Bluesky are public** and Bluesky publicly calls it an unsolved tension that may
persist even with private-data features (docs.bsky.app/blog/block-implementation); inbound-filter-private
/ outbound-unenforceable-while-federated; **X lets blocked users view but not interact** (Nov 2024); a
self-hosted AppView can privately drop a target's content (the "local shadow ban" — architecturally
sound; "shadow ban" is the dialogue's label); **Jetstream** (compressed filterable JSON firehose);
off-repo private data via the #3363 "namespaces" direction; DID-document service endpoints + custom-
lexicon sidecars; encrypted-blob vault (CID over ciphertext); CQRS framing; CAR/MST offline-mesh sync;
**AppViewLite** (alnkesq, C#, ~2.2 GB/day); **Blacksky AppView + rsky-wintermute** (Rust, ~10k rec/s,
private-community-posts scaffolding); **zeppelin-social/bluesky-appview** (Docker stack + palomar);
**Colibri** (Discord-like, HN debate, verdverm permissioned prototypes); **#IndieSky / Eurosky / Free
Our Feeds** real; migration back-to-bsky for returning users only (importRepo blocked for net-new
external identities) via **PDS MOOver** / **goat**; migration uses signed PLC ops + email + CAR.
**PARTLY:** "AT Community Fund" (#2); Groundmist "Publish button" detail (design intent, not quoted) +
AppViewLite ".NET 9" exact version unconfirmed.

**GROUNDMIST — REAL.** By **grjte** (grjte.sh): a private, local-first ATProto layer; **PSS** = "personal
data store analogous to your atproto PDS, but private"; repos `grjte/groundmist-sync` + `grjte/groundmist-
notebook` (+ Library); ATProto OAuth+DPoP, **Automerge** (`@automerge/automerge-repo`), WhiteWind-lexicon
mapping; cites Ink & Switch local-first. **Caveat:** the sync server is an explicitly-labeled prototype
with **incomplete/insecure auth** ("all requests approved without authentication") — "private by default"
is the *intent*, not yet the security reality. The closest living relative of the model in this dialogue.

## Cluster 4 — Bluesky VC + Aggregation Theory (strategic background)

CONFIRMED: Series A ~$15M / Blockchain Capital / Oct 2024; Series B ~$100M / Bain Capital Crypto (timing
#5); PBC; no-token/NFT promise; ~43M users / ~20B+ records (timing #5); enshittification = Doctorow,
switching-cost thesis; portable identity → low switching cost → structural constraint on enshittification;
Aggregation Theory = Ben Thompson / Stratechery 2015; demand-not-supply thesis + costs→0; "Defining
Aggregators" L1/L2/L3 (Netflix / Uber-Airbnb / Google-Meta); the quote (attribution correct, #9);
Red Hat analogy ($34B IBM acquisition). All structural claims check out.

## Cluster 5 — Fork-tail: open-source clients + Twitter Circles trilogy

CONFIRMED: **Ouranos** (`pdelfan/ouranos`, Next.js web client); **atcute** (`mary-ext/atcute`, TS ATProto
utilities); **social-app** (official, open source); Twitter **Circles** launched Aug 30 2022 (≤150, IG-
Close-Friends framing), Apr 2023 leak to "For You" (confirmed "security incident"), sunset Oct 31 2023;
**Communities** discontinued ~May 2026 (#6) — **the <0.4%-users / 80%-of-spam-reports stat is a real
direct quote** from X head of product Nikita Bier; **XChat/Group DMs** E2EE + disappearing + join links,
limits raised toward 500→1,000. **PARTLY:** **Heron** (`tunjid/heron`, KMP+Compose, offline-first/Room
confirmed; "WriteQueue" unverified — #7); Communities date (#6). **CLIENTS REAL?** Ouranos, Heron, atcute
all real and correctly attributed.

## Why this matters for Croft design

- **The sovereign-AppView "club" is a real, buildable shape of Croft's own stance — and the user flagged
  it as especially valuable.** The split that makes it work (PDS = data/write, AppView = index/read; the
  AppView as the private gatekeeper) is the same separation Croft relies on, and the dialogue's
  conclusions converge on Croft's principles: **experience-shaping over content-shield** (you can't hide
  public data, you strip the ability to *touch your corner* — matches social-layer "openness caps
  propagation / visibility sink"); **off-repo private data + encrypted blobs** (the host-untrusted /
  blind-broker line, COHESION §26); **multi-source AppView** (ATProto + ActivityPub + Nostr + RSS — the
  honest-seams **ponds** model made concrete at the index layer); **CAR/MST offline mesh** (rhymes with
  the iroh/lineage CAR-sync + `ios-opportunistic-p2p` work — though "two devices auto-sync" carries the
  same aspirational caveat the iroh FACTCHECK already flagged). **Private cooperative Labelers** map onto
  the **geer** gating-peer (label-not-enforce). This is strong "build-on / learn↔" material for the app
  layer — register the projects, prototype the proxy.
- **The attention-economy framing is Croft's, verbatim.** The user's "Nielsen rating vs Meta
  manipulation / community not extraction" maps directly to `principles.md` Tier-1 (**non-extraction is
  the point**, **user-need-first**) and Tier-3 (**shapeability + stability; constant UI change is quietly
  extractive**). Dwell-time/passive-curation "calm" feeds and the "credible exit" selling point are the
  product-layer expression of those principles.
- **Twitter Circles is a cautionary proof, not just trivia.** "Semi-private on a public broadcast network
  is structurally fragile" (Circles leaked when ranking logic changed; the binary public-or-encrypted
  won) is independent corroboration of Croft's **"content is born into a visibility regime and cannot
  silently change it"** (social-layer V3) and the lineage-groups **structural-not-runtime enforcement**
  principle: don't build a gating-clause over a public plane; make private data *structurally* private
  (off-repo / E2EE). Cite it where social-layer argues regimes.
- **Aggregation Theory names the macro-bet.** "Decentralized protocols dropping switching costs to zero
  so users move demand freely" is the strategic frame under Croft's credible-exit / no-data-hostage
  stance and the sustainability↔cooperative-mechanism question (feeds open-considerations §8 / the §25
  PDS-economics item). Strategic background, not a design change.

## Provenance

Web verification 2026-06-22, five parallel passes. Standing source of truth for atproto/iroh:
`atproto-atmospheric-web-iroh-mobile-FACTCHECK.md` (+ its dated addenda). This body's PDS/Relay/AppView
mechanics overlap the §27 architecture explainer and §26 Germ/private-data intakes — cite those, not
re-derived here.
