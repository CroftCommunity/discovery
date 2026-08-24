# Forage — naming the social forum site (Claude Code session, 2026-08-21)

**What this is.** The naming session for the atproto social-forum project: from "graze is
taken and amble too" through five register pivots (water/journey → small-town gossip third
space → .exchange compounds → creative TLDs → nature-behavior names → corvids → domain
hacks) to the decision: **Forage, at forage.fyi (purchased in-session), `/f/` path
convention**. Includes the availability sweeps (DNS NS-delegation checks per candidate), the
atproto/bluesky collision searches, the USPTO knockout search, and the corvid brand-world
material (rookery / ravens / "Nevermore" as charter) left available but not load-bearing.

**Fidelity.** Live Claude Code session, filed same-day by the same session. USER turns
preserved **VERBATIM** (typos kept as-is). ASSISTANT turns **content-faithful, NOT
byte-pristine** — candidate lists, check results, and reasoning preserved; connective prose
tightened. Tool invocations summarized as bracketed notes. One user turn was a screenshot
(Porkbun cart) — described in a bracketed note. No credentials appeared; nothing redacted.

**Ground truth.** Domain availability = the in-session `dig NS` / RDAP checks (a snapshot —
statuses drift; "no NS delegation" was used as the likely-available heuristic, RDAP 404 as
the definitive check for forage.fyi). Trademark findings are a knockout search, not legal
clearance. Distillation: `research/forage-name-clearance-2026-08.md`, `NAMING.md`
(forum-layer: Amble → Forage), COHESION §75, ROADMAP_TODO E122.

---

## Round 1 — the brief: water/journey/information flowing, croft-universe

**USER (verbatim):**
> I was sure I had a pick for my social forum site in graze but come to find out it's taken
> for sure in atproto space then I really liked amble but it's taken as well, and I'm not as
> love with it.
>
> I want to find a name that has to do with a journey or water flowing or information
> flowing but natural, in the same universe as croft and drystone, ideas?

**ASSISTANT (condensed).** Read the brief as: stay in the agrarian hill-country lexicon
croft/drystone come from, pick its water and path words. Candidates offered — water: beck,
burn, leat (hand-dug mill channel — "a built channel for natural flow"), ghyll, rill, linn,
spate, eddy, riffle, firth/strath; journey: wend (noting the `.ing` domain trick — the
project already lives at croft.ing, and wend.ing spells "wending"), ford, holloway, stile,
drove, cairn, meander.

*[Tool note: whois to whois.nic.google failed (sandbox DNS); fell back to `dig +short NS`
per domain — a registered domain almost always has NS delegation. Checked 13 `.ing`
candidates.]* Results: wend/ford/beck/rill/meander/linn REGISTERED; leat, eddy, riffle,
spate, ghyll, burn (premium-suspect), stile, drove undelegated. *[Web searches for
leat/eddy/riffle/spate/ghyll in atproto/bluesky context: nothing found.]* *[Checked
`.bsky.social` handles: all taken — noted as irrelevant since the project would use its own
domain as handle.]* Recommended **leat** (meaning bullseye), runner-up eddy, mentions for
riffle and ghyll.

## Round 2 — Latin eddy; meander not sold

**USER (verbatim):**
> is there a latin word for eddy?
>
> and I like meander but I'm just not sold on it yet

**ASSISTANT (condensed).** Latin: **vortex/vertex** (the standard word), gurges, vorago,
turbo; Greek dinē. Cautioned that Latin breaks the Anglo-Saxon/Norse register of
croft/drystone. On meander: it is itself the river Maeander turned common noun; offered the
vernacular neighbors **purl** (the purling brook — water + quiet talk) and **wimple**
(Scots, "the wimplin' burn" — meander in croft-dialect). *[dig checks:]* purl.ing
REGISTERED; wimple.ing, vortex.ing (premium-suspect) undelegated. Floated the tie-breaker
heuristic: say "I saw it on ___" aloud.

## Round 3 — the sound-alike filter arrives

**USER (verbatim):**
> wimple is too close to wimp, eddy too close to ed the name, and leat too close to "leet"

**ASSISTANT (condensed).** Adopted the user's sound-alike test as a standing filter. New
candidates screened against it: runnel (run + tunnel, harmless), wynd (Scots winding lane;
homophone "wind/wined" flagged), esk (Brittonic "water"; neighbor "ask"), trod, swale,
freshet, drift (premium-suspect), stravaig, shiel (shiel.ing spells "shieling").
*[dig: all 12 undelegated.]* *[Web search runnel/wynd/esk in atproto: nothing.]* Ranked:
runnel, esk, wynd.

## Round 4 — pivot to the small-town gossip third space

**USER (verbatim):**
> ok what if we pivot and make it about where you get the news, or the gossip in a small
> town or community, the adjacent third space

**ASSISTANT (condensed).** The gossip institutions of small-town Britain: **blether**
(blether.ing spells the Scots gerund), smiddy (the blacksmith's, proverbial news hub),
steamie ("talk of the steamie"), howff, snug, ceilidh, natter, claver; proclamation places:
mercat (cross), bothy (with bothy-book culture), moot (killed: 4chan founder's handle).
*[dig:]* blether.ing and natter.ing REGISTERED ("someone Scottish beat us to it");
smiddy/steamie/howff/snug/claver/mercat/bothy/ceilidh undelegated. *[Web search
blether/smiddy/bothy atproto: nothing.]* Sound-alike pass: smiddy≈Smitty (fails the eddy
test), steamie≈steamy, howff unsayable, mercat≈meerkat. Recommended **bothy** (the bothy
book = a stone-hut asynchronous forum that has existed a century) with **snug** as
runner-up.

## Round 5 — not in love; the register diagnosis

**USER (verbatim):**
> so far not in love with any of them

**ASSISTANT (condensed).** Diagnosis: the two loved names (graze, amble) are **ordinary
words with a pastoral second life** — recognition-plus-surprise — not museum dialect. New
list in that shape: glean, linger, yarn (stories + threads + wool; JS-package-manager
collision flagged), murmur (water + voices + murmuration), fold (the drystone sheepfold +
"back to the fold"; Fold bitcoin app flagged), heft (hefted hill flocks — belonging without
fences), roost, gather (Gather.town collision), patter, perch, saunter, stroll. *[dig:]*
glean/murmur/roost/patter/perch REGISTERED; linger, yarn, fold, heft, gather, saunter,
stroll undelegated (gather/fold premium-suspect). Recommended **linger**, then yarn, then
heft.

## Round 6 — the .exchange frame

**USER (verbatim):**
> what if we looked for a something.exchange as in conversation.exchange or something but
> on brand

**ASSISTANT (condensed).** Noted the frame is period-accurate: every market town had a Corn
Exchange; Bradford a Wool Exchange. Candidates: fair.exchange ("fair exchange is no
robbery" + the village fair), stile.exchange (over-the-stile talk + a Stack Exchange wink;
stile/style homophone flagged), corn/wool/yarn/tale/hearsay/village/gossip/tryst .exchange.
Caveat: `.exchange` reads crypto in 2026. *[dig:]* fair, corn, wool, gossip REGISTERED;
stile, yarn, tale, hearsay, village, tryst open. Recommended **yarn.exchange** (no homophone
traps; the threads pun).

## Round 7 — the creative-TLD survey

**USER (verbatim):**
> what tlds can we use in this vein like gossip or social but creative

**ASSISTANT (condensed).** Surveyed real TLDs by theme — third space: .pub (also
"publish"), .cafe, .place, .town, .club, .community, .social, .camp, .rest (wayside inn);
gossip/news: .buzz, .fyi, .news, .chat, .today; pastoral: .farm, .land, .green, .garden,
.market, .scot, .coop; flow: .stream (the only water TLD); time: .day. Noted nonexistent:
.gossip, .village, .folk, .inn, .tavern, .tea, .talk, .line. Then compound whole-phrase
checks. *[dig, with TLD-existence guard:]* **village.green, market.day, mill.stream,
snug.pub, drovers.rest, croft.town all undelegated**; common.land, third.place, story.camp,
corner.shop, blether.scot REGISTERED. Flagged village.green/market.day as
obvious-compound premium suspects. Recommended village.green (concept bullseye; common
grazing land rhymes with graze), market.day (names an event), mill.stream (keeps the water
brief + "rumor mill").

## Round 8 — the tweet test: nature behavior everyone understands

**USER (verbatim, first message interrupted by the user then continued):**
> what if we name it after a great social chat or chatting with a friend after a long time
> or something, but so far I'm pretty meh on these, some good starts and ideas
>
> or we look at nature and like tweet and twitter was about behavior everyone understands,
> something like that for conversation, interaction something

**ASSISTANT (condensed).** The sharpened brief: a behavior every human already knows that
maps to conversation. Candidates: **waggle** (the bee dance — a community member returns
and performs where the good stuff is; "not a vibe-match like tweet, a *functional*
match"), gaggle (Gaggle.net school-surveillance collision), grouse (moor bird + to vent),
lark (ByteDance Lark collision), chorus (dawn chorus), ripple (XRP-poisoned), purr;
already-claimed-in-spirit: huddle (Slack), hive, flock (Flock Safety), murmur. *[dig:]*
waggle.social, waggle.exchange, gaggle.buzz, grouse.social/.exchange, coo.ing open;
waggle.buzz, gaggle.social, lark.social, chorus.social, purr.ing REGISTERED. *[Web search
waggle/gaggle/grouse atproto: nothing.]* Recommended **waggle**.

## Round 9 — chorus, ripple, and the crow door opens

**USER (verbatim):**
> chorus is kind of good, ripple isnt bad, something with a crow wouldn't be terrible bc
> crow is to crow and a crow like bird

**ASSISTANT (condensed).** Corvid case made: crows are documented social
information-sharers (Marzluff facial-recognition studies; the ornithological
"information-center hypothesis" for communal roosts). Family tour: crow (crow.ing spells
"crowing" — but crowing = boasting; "a murder of crows"), **rook/rookery** (colonial
nesters; folklore "parliament of rooks"; flags: "to rook" = swindle (archaic), Victorian
slum sense (dead)), corbie ("The Twa Corbies" — two crows exchanging news), jackdaw,
magpie, raven (Odin's ravens; GoT "send a raven"; crowded namespace), corvid (dead:
covid-adjacent). *[dig:]* crow.ing, rookery.social, rookery.exchange, corbie.social/.ing,
jackdaw.social, chorus.exchange/.place/.town open; crow.social, caw.ing, rook.social,
magpie.social, raven.social, ripple.exchange, ripple.stream REGISTERED (ripple declared
dead — crypto fenced it). *[Web search rookery/chorus/jackdaw atproto: nothing.]*
Recommended **rookery** ("crows solved social long before we did"); warned crow.ing
canonizes boasting.

## Round 10 — warming to rook; testing the associations

**USER (verbatim):**
> rook and rookery isn't bad if the association isn't that strong, raven is pretty good,
> and I do like "rook" as part of rookery

**ASSISTANT (condensed).** Association strength assessed honestly: "to rook" archaic (live
associations = chess piece + "rookie," the latter a gift — new members are rookies);
rookery-as-slum dead, modern sense is thriving wildlife colonies. Raven: best mythology,
most crowded. *[dig on variants:]* rook.exchange/.place/.buzz, rooks.social,
rookery.place/.town, raven.place/.buzz, ravens.social open; rook.town/.chat, rookery.chat,
raven.exchange/.town/.community/.chat REGISTERED. *[Web search "Rookery" app/social:]*
**Two live adjacencies found:** (1) **Roost Social** (TechCrunch 2026-07-08, 100k+
installs) uses "your rookery" as in-app vocabulary; Android package literally
`com.rookery.courier`; (2) an early project **Rookery** — "social media for research
papers" — then at rookery.vercel.app. Verdict: rookery drops to "contested but takeable";
suggested **bare Rook** sidesteps both.

## Round 11 — Nevermore surfaces; domain hacks requested

**USER (verbatim):**
> nevermore is interesting depending on the meaning/usage
>
> can we look for short tld incorporated rook/rookery/

**ASSISTANT (condensed).** Nevermore read straight is Poe's "never again" (loss), and
Netflix's Nevermore Academy makes the trademark space hostile — suggested tagline, not
name. Hacks derived: **roo.ky** ("rooky" — Macbeth's "the crow makes wing to th' rooky
wood"), rook.ie (Ireland-restricted), rooke.rs ("rookers"), **brook.ing** (the brook with
a rook inside — water brief + corvid in four letters), nevermo.re.

**USER (verbatim, mid-turn):**
> what were the ravens names for oden?

**ASSISTANT (condensed).** **Huginn** (thought) and **Muninn** (memory) — fly out at dawn,
return with the world's news; noted both names already mined in software (Huginn
automation agent; Munin monitoring — both chosen for the same metaphor). *[dig:]* roo.ky,
rooke.rs, brook.ing, brooke.rs open; rook.ie, nevermo.re (active on Vercel) REGISTERED.
Recommended **brook.ing** ("closes the whole circle") with roo.ky as the deep cut; ccTLD
landlord caveat (.ky/.rs vs Google's .ing).

## Round 12 — Nevermore as the vow; Humm proposed

**USER (verbatim):**
> nevermore kind of works when the point of the social project is to nevermore be controled
> by enshittification through corporate fiat and extraction
>
> an d what about "Huginn" and "muinn" as "humm" as in "humm the news"

**ASSISTANT (condensed).** Endorsed the reframe — nevermore as a refusal aimed at the
platforms ("the rare name that carries a politics") — but recommended spending it on the
**tagline/manifesto slot**, not the product name ("Rook. Nevermore." / later: the
**Nevermore Charter** as the governance document's name). On Humm: **Hu**ginn + **M**uninn
→ Humm, landing on "hum" = the ambient sound of collective talk ("what's the hum?") — two
floors, surface meaning + Norse story. Collisions: Humm Group (ASX BNPL fintech), Hum by
Verizon. *[dig:]* humm.ing, humm.social, hum.social, hum.place REGISTERED; humm.exchange,
humm.place, humm.buzz, thehum.social open. Flagged "The Hum" = the Taos Hum (unexplained
tormenting drone) on search. Gave the eight-round running-shortlist table; top pick then:
Brook at brook.ing, else Rook + "Nevermore.".

## Round 13 — hum hacks; brook killed by the name test

**USER (verbatim, first message interrupted by the user, then):**
> is there a good "hum" tld combo, hum.ed or humm.ing or something
>
> unf brook is just too common of a name I feel like

**ASSISTANT (condensed).** Conceded brook by the user's own filter (Brooke the name — the
eddy flaw). Hum hacks: no .ed/.um/.an TLDs exist; **hu.ms** ("hums"), **hum.us**
("humus" — community as living soil; hummus jokes noted), hum.fm. *[dig:]* hu.ms and
hum.us REGISTERED; hum.fm open. Read the pattern aloud: every prime hum domain defended =
loved-but-crowded word; the corvid ground kept turning up open. Advised: stop generating,
start eliminating — sleep on Rook vs Rooky vs Rookery, then run full clearance on one.

## Round 14 — forage.fyi

**USER (verbatim):**
> what about forage.fyi

**ASSISTANT (condensed).** Recognized the return to the graze family: forage = graze's
sibling verb, the tweet-test behavior, tilted to information by `.fyi` itself ("forage,
for your information"). Sound-check clean. Collision noted: Forage / theforage.com
(ex-InsideSherpa, job-simulation edtech) — different class. *[dig:]* **forage.fyi
undelegated**; forage.social REGISTERED; forage.exchange/.place, graze.fyi open (graze
itself off-limits — the Graze company); rook.fyi and rookery.fyi REGISTERED (parked).
Framed the finalists: Forage (the behavior) vs Rookery (the place), noting a forum is more
place than activity but Forage is the cleaner single-purchase move.

## Round 15 — the targeted atproto/social sweep

**USER (verbatim):**
> look especially for projects in teh atproto and bluesky and social media/networking spaces

**ASSISTANT (condensed).** *[Four web searches + grepped the raw README of
bluesky-social/atproto-ecosystem (zero matches for forage/rook/rookery) + resolveHandle
checks.]* Findings: **Forage clean across the board** — no atproto/bluesky project, no
social-network startup, only the edtech company. **Rookery: the research-papers project
had graduated to rookery.network**, tagline "social media for research papers" — a live
same-category occupant, the graze/amble collision shape. Bare rook clean. Verdict: **Forage
at forage.fyi**; remaining unknowns mechanical (registrar price / premium flag, USPTO).

## Round 16 — USPTO knockout + price; "do ravens forage?"

**USER (verbatim):**
> run the uspto search and check the registrar price

**USER (verbatim, mid-turn while checks ran):**
> do ravens forage?

**ASSISTANT (condensed).** Ravens answer: emphatically yes — Bernd Heinrich (*Ravens in
Winter*): a juvenile raven that finds a carcass recruits the roost and leads others there
next morning; the best-documented information-sharing forager. "Your two finalists just
merged: Forage is the name, and the raven story is the brand."

*[Availability: RDAP 404 for forage.fyi via rdap.org and Identity Digital direct —
registry-confirmed unregistered. Porkbun public pricing: .fyi base $5.66/yr; premium flag
not determinable anonymously. USPTO tmsearch API unreachable (S3 static); Justia bot-walled;
fell back to targeted searches + TrademarkElite/uspto.report record pulls.]*

Trademark findings (knockout search, not legal advice):
- FORAGE serial 88470683 (Forage, Inc., LA — Class 9, restaurant-reservation software):
  **DEAD**, abandoned 2024-03-04, never registered.
- FORAGE, INC. serial 88794474 (Class 35 nonprofit-fundraising consulting): **DEAD**,
  abandoned 2022.
- FORAGE serial 90087783 (InsideSherpa Inc. / theforage.com): **LIVE** — internship/career
  placement + virtual work-experience training. Different services from social networking.
- Live FORAGE / FORAGE KITCHEN marks: food classes (preserves, restaurants, kombucha) —
  irrelevant.
- FORAGEPRO (76357396, filed 2001, Class 42 agri-analysis flavored): status unverified,
  goods unrelated either way.

Conclusion: **no live FORAGE occupant in the classes a social forum would file in (38/42/45)**.

## Round 17 — the cart screenshot; the 1-year question

**USER:** *[screenshot: Porkbun cart, forage.fyi flagged "Premium Registration", $8.47
first year (struck from $16.90), est. renewal $16.90, 1-Year term shown]*

**ASSISTANT (condensed).** Read the flag: mildest premium tier — ~$17/yr renewal forever
(never drops to base), a rounding error for a project domain. Recommended checkout +
defensive grabs (graze.fyi, forage.exchange/.place).

**USER (verbatim):**
> that only shows me 1 yr possible

**ASSISTANT (condensed).** Normal for premium-flagged names — single-year initial term at
checkout; extend from the management panel afterward (up to the 10-year window) at the
premium renewal rate; turn on auto-renew (lapsed premium dictionary words get drop-caught).

## Round 18 — a better TLD than .fyi?

**USER (verbatim):**
> any better tld that could go with forage?

**ASSISTANT (condensed).** The hacks: **fora.ge** (spells the whole word; left of the dot
is "fora," the plural of forum) and **forage.rs** ("foragers" — the members). *[dig:]*
fora.ge REGISTERED (parked — "someone knew exactly what they had"); forage.app, .club,
.day REGISTERED; **forage.rs open**, plus .chat/.news/.farm/.land/.garden/.buzz. Verdict:
.fyi keeps the crown (meaning + landlord stability); forage.rs recommended as companion
grab, forage.news runner-up.

## Round 19 — DECISION

**USER (verbatim):**
> I bought forage.fyi -- graze is done, already taken, forage it is and we can do /f/ for
> forum and forage too for the reddit style forum

**ASSISTANT (condensed).** Decision acknowledged: **Forage, forage.fyi (purchased
2026-08-21), `/f/` path convention** (f = forum and forage; e.g. forage.fyi/f/gardening),
members = "foragers." Saved the session memory
(`forage-social-forum-naming.md` in the workspace memory dir). Noted the unpurchased
defensive grabs (graze.fyi, forage.rs) and that the raven/rookery/Nevermore brand-world
material remains available but optional.

**USER (verbatim):**
> Can we go ahead and file this whole transcript in the normal Discovery Repo transcript
> filing kind of raw mechanism? that would be great.

*[This file is that filing. Same-session; §2b + §2c of the PLAYBOOK followed.]*

---

## Appendix — availability snapshot (2026-08-21, dig NS heuristic unless noted)

Open at check time (not purchased unless stated): leat.ing, eddy.ing, riffle.ing,
spate.ing, ghyll.ing, stile.ing, drove.ing, runnel.ing, wynd.ing, esk.ing, trod.ing,
swale.ing, freshet.ing, stravaig.ing, shiel.ing, syke.ing, rindle.ing, weir.ing,
smiddy.ing, steamie.ing, howff.ing, snug.ing, claver.ing, mercat.ing, bothy.ing,
ceilidh.ing, linger.ing, yarn.ing,
fold.ing, heft.ing, gather.ing, saunter.ing, stroll.ing, stile/yarn/tale/hearsay/village/
tryst .exchange, village.green, market.day, mill.stream, snug.pub, drovers.rest,
croft.town, waggle.social, waggle.exchange, gaggle.buzz, grouse.social, grouse.exchange,
coo.ing, crow.ing, rookery.social, rookery.exchange, rookery.place, rookery.town,
corbie.social, corbie.ing, corbie.rest, jackdaw.social, hrafn.social, eyrie.social,
chorus.exchange, chorus.place, chorus.town, rook.exchange, rook.place, rook.buzz,
rook.rest, rookery.rest, rooks.social, raven.place, raven.buzz, ravens.social, roo.ky,
rooke.rs, brook.ing, brooke.rs, humm.exchange, humm.place, humm.buzz, thehum.social,
hum.fm, bran.social, bran.place, bran.exchange, graze.fyi, forage.rs, forage.chat,
forage.news, forage.farm, forage.land, forage.garden, forage.buzz, forage.exchange,
forage.place.

**Purchased: forage.fyi** (Porkbun, premium tier, $8.47 yr-1 / ~$16.90 renewal).

Registered (notable): all prime graze/amble ground (pre-existing), wend.ing, ford.ing,
beck.ing, meander.ing, blether.ing, natter.ing, fair.exchange, corn.exchange,
wool.exchange, gossip.exchange, common.land, third.place, blether.scot, waggle.buzz,
raven.social + most raven.*, raven.rest, ravens.rest, perch.* (all), crow.social,
rook.social, rook.fyi, rookery.fyi, rookery.chat, magpie.social, humm.ing, humm.social,
hum.social, hum.place, hu.ms, hum.us, purl.ing, glean.ing, murmur.ing, roost.ing,
fora.ge, forage.app, forage.social, forage.club, forage.day, nevermo.re, rook.ie.
