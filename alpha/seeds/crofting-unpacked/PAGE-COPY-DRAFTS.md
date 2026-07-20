# Page copy drafts — for review before instruction files

Three pages. Copy is written to be placed verbatim; notes to you are in
[bracketed italics] and will not ship. Grounding: arecipe's README,
docs/PHILOSOPHY.md, docs/SECURITY.md, docs/STACK.md; the discovery repo's
socialization and fenced layers; the live croft.ing conventions.

---

# PAGE 1 — croft.ing/arecipe  (the user guide / about)

*[Same header/footer as the rest of croft.ing. Tier labels THE SIGNPOST and
THE SURFACE at top, THE BEDROCK at bottom; the middle sections carry their own
names instead of THE SOIL.]*

## THE SIGNPOST

A recipe box whose recipes are actually yours.

## THE SURFACE

arecipe is a recipe box and meal planner for your people: a family, a supper
club, a neighborhood. Your recipes live as records in your own account on an
open network, not in the app. arecipe is just a good kitchen window onto them.

*[Button: **Open arecipe** → https://arecipe.app , the one ruddy button on the
page.]*

## THE FIELD GUIDE

**Start without an account.** Open Alchemy, the drafting workspace, and start
writing recipes. No sign-up wall stands between you and the page.

**Sign in when you want to keep them.** arecipe signs in with your Bluesky
account (or any AT Protocol account). Signing in is what moves your recipes
into your own data store, where they belong to you rather than to the app.

**Browse and cook.** Browse shows recipes from the network. Your Cookbook
gathers your own recipes plus those of the cooks you follow, and it has a
share link, so handing someone your whole cookbook is one URL.

**Compare versions of a dish.** Every dish can hold alternative versions side
by side: your grandmother's and the internet's, on one screen, no winner
declared.

**Plan the week.** Meals is a weekly planner: assign recipes to days, repeat
good weeks onto the calendar. Meal plans get public share links too, so the
household can just look. For the committed, an opt-in publishing step turns
your plan into a calendar feed you can subscribe to from Google Calendar.

## BUILT TO BE YOURS

There is no arecipe server. The app is a static bundle in your browser; your
recipes are records in your own data store; the same records already render on
recipe.exchange, a separate project, with no coordination between the two.
If arecipe disappeared tomorrow, your recipes would not. That is not a promise,
it is an architecture, and the full argument has its own page:
**The treatise** at arecipe.croft.ing.

## ITS PLACE IN THE VALLEY

arecipe is the first working crop on Croft land: proof that the plot model
holds, with something as unglamorous and beloved as the family recipe box.
The a is for Amanda.

## THE BEDROCK

One line of framing: The app, the argument, and the source, in that order.

- Use it: https://arecipe.app
- Why it is built this way: https://arecipe.croft.ing
- Source: https://github.com/CroftCommunity/arecipe

---

# PAGE 2 — croft.ing/skylite  (early basic version)

## THE SIGNPOST

A window to the stars, sized for the youngest neighbors.

## THE SURFACE

Skylite is a read-only window into Bluesky for kids and other gentle
audiences: no algorithm, no ads, no posting, no strangers. A grown-up tends a
garden of accounts and feeds worth seeing; the explorer just looks up.

## WHAT IT IS TODAY

Skylite is under active cultivation. The bones are live at skylite.croft.ing:
a sponsor (the grown-up) curates what appears, and the explorer (the kid, or a
tech-shy grandparent) gets a calm, chronological view of just that, honoring
the network's own content labels. No account is needed to look, and in its
default mode nothing about the explorer leaves the device.

## ITS PLACE IN THE VALLEY

Every valley has households, and households have children. Skylite is the
gate in the wall built at their height: a way to see the sky without being
seen by the machinery. It is the same Croft conviction, that the person is
not the product, applied to the people most often treated as one.

## THE BEDROCK

- The window itself: https://skylite.croft.ing
- Source: https://github.com/CroftCommunity/skylite

---

# PAGE 3 — arecipe.croft.ing  (the treatise, new repo `arecipe_treatise`)

*[One long page, Croft visual identity, generous whitespace, drystone-course
dividers between acts. Reads as a written-out conference talk.]*

*[Kicker, small caps:]* A CROFT FIELD REPORT

# The recipe box that cannot turn on you

*[Lede:]* arecipe is a small recipe app built as a working argument: that the
slow decay of beloved software is a design choice, and that a different design
makes it structurally impossible. This page is the talk version of that
argument.

## Act I. A short story you already know

A recipe app launches. It is fast, kind, and free, and you pour a decade of
family cooking into it. It gets popular, then acquired. Ads arrive in the
search results, then between your own recipes. The features you rely on drift
one by one behind a subscription. The export button technically exists and
half works. Eventually a sunset notice thanks you for being part of the
journey, and your grandmother's handwriting, transcribed one winter, is a
row in a database someone else is deleting.

Nobody in that story is a villain. Cory Doctorow named the pattern
enshittification: a platform is good to its users until they are locked in,
then value is clawed back, stage by stage, because the operator answers to
someone other than the users. The word is crude; the mechanism is not. It is
structural, and that is the actually hopeful part, because structures can be
refused.

## Act II. The four preconditions

arecipe's design docs distill the pattern into four structural preconditions.
The decay needs all of them:

an operator with unilateral control over how the platform behaves; data
lock-in that makes leaving expensive; the users' aggregated data held as a
proprietary asset; and network effects that make individual defection
irrational, because leaving means leaving everyone.

Remove any one and the pattern stalls. Remove all four and it cannot begin.
arecipe removes all four.

**No unilateral operator control.** An operator exists: someone owns the
domain, maintains the code, signs releases. But the levers are
minimum-authority, and anything the operator publishes, users can decline.

**No lock-in.** Recipes are records in your own data store on the AT Protocol,
in an open schema. Any conforming app renders them. The proof is live, not
promised: the same records already appear on recipe.exchange, a separate
project, with no coordination between the two.

**No data asset.** The application holds nothing. No database, no user table,
no analytics. There is nothing to monetize because there is nothing there.

**Network effects stay at the protocol layer.** Your people are your Bluesky
graph, not an arecipe roster. Adopt a competing app and your recipes, your
follows, and your moderation choices all come with you.

## Act III. The shape: a PWA with no back half

Two terms, then what they mean here. A single-page application (SPA) is an app
that runs entirely in your browser as scripts and pages, rather than asking a
server to think for it. A progressive web app (PWA) is a website that your
phone or desktop can install and treat like a native app: an icon, an offline
shell, a full window. arecipe is both, and the mix is the point: SPA means all
the logic ships to you and runs on your device; PWA means it feels like an app
worth living in.

What arecipe deliberately is not, is served. There is no application server
anywhere: the whole app is a static bundle of files, built in the open and
hosted as plain pages. Each destination is its own document rather than one
mega-app with a router, so the surface area stays small and legible. Your
browser talks directly to your own data store; nothing of yours passes through
an arecipe machine, because there isn't one.

## Act IV. Security, honestly

No server means no server to breach and no password database to leak, and it
also means every defense must ship inside the pages themselves. Sign-in is
the network's own OAuth, run entirely in the browser, and the resulting
credentials are sender-constrained: a stolen token is inert off the device it
belongs to, because using it requires a proof signed by a key that never
leaves that device.

That moves the realistic threat to one place: malicious script running inside
the app's own origin. So that is where the security budget goes: a strict
content-security policy carried in every document, integrity hashes on the
code the pages load, zero third-party scripts, and a build gate that loads
every page under the enforced policy and refuses to ship on any violation.

And the honest residuals, stated rather than hidden: a fully compromised
device is out of scope for any browser app, and script that does get inside
the origin could use keys in place even though it cannot steal them. The
controls shrink that surface; nothing reduces it to zero, and a page that
claimed zero would be exhibiting the exact confidence this project exists to
refuse.

## Act V. What you actually get

Practically, the architecture cashes out like this. Your recipes render in
other apps today, so exit is a live demonstration, not a clause in a promise.
The app can be installed and used like any other, shared cookbook links and
weekly meal plans included, and an opt-in step publishes your plan as a
calendar feed your household can subscribe to. There are no ads and there
never can be, not as policy but as plumbing: there is no data to target with
and no chokepoint to rent. And the project's ambition is deliberately
countercultural: reach a good interface, then let it be. Software that has
nothing to extract has no reason to churn.

## Act VI. What this does not claim

Bounded claims are the whole posture, so here are the bounds. This design does
not resist an author walking away; abandoned software stays abandoned. It does
not resist a court order against the domain; the frontend would end, though
the records would survive. Public records can be indexed by anyone, which is
the price of credible exit. And the largest honest residual is governance:
one person still owns the domain, holds the signing key, and decides what
ships. That is not enshittification, since there is no data, rent, or
chokepoint to extract through, but it is concentrated authority, and the
design keeps the door open to sharing it out over time.

## The gate

*[Bedrock-style link block:]*

- The app itself: https://arecipe.app
- The user guide: https://croft.ing/arecipe
- The homestead this grows on: https://croft.ing
- Source, philosophy, security posture, in full: https://github.com/CroftCommunity/arecipe
- The same recipes, rendered elsewhere: https://recipe.exchange

*[Footer: same as croft.ing plus the AGPL line.]*
