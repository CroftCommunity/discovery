# The cooperative "Social Union" model — a sustainability *mechanism* for non-extractive infrastructure

date: 2026-06-22

source: distilled (CONFIRMED-only) from `seeds/transcripts/raw/cooperative-social-union-governance-dialogue-2026-06-22.md`
(Gemini; fact-checked — see its `-FACTCHECK.md`). This is the corpus's first deep treatment of the
**sustainability ↔ cooperative *mechanism*** open item flagged as existential in `ROADMAP_TODO.md` /
`open-considerations.md` §8. It is the user's (chasemp) own design thinking; treat the *reasoning* as
primary.

> **NOT LEGAL OR TAX ADVICE.** The legal scaffolding below cites Missouri statute sections and IRS
> codes from a model-generated dialogue. The *framework* is verified real; several *specifics are
> wrong* and are flagged inline. **Every section number and fee requires confirmation by a Missouri
> cooperative attorney / CPA before any filing or reliance.** See the FACTCHECK for the full verdicts.

## Problem statement

Every prior attempt at a non-extractive, user-owned social platform has died one of three deaths
(all verified — FACTCHECK §11): **VC capture** (Ello — took $5.5M VC for an anti-VC mission, no
revenue engine, silent shutdown 2023, user data wiped); **volunteer burnout** (Ampled — pure co-op on
≥80 uncompensated hours, couldn't fix payment bugs, closed end-2023 citing terminal burnout);
**hyper-financialization** (Steemit — tokenized every click into a bot-farmed casino, hostile crypto
buyout 2020); plus **architectural naïveté** (Diaspora — leaked metadata to cloud middleware) and
**white-label dependency** (Coomappa, Brazil — "owned a cooperative but actually owned a technological
dependency"). Meanwhile the survivors that *advertise outside their own circle* are VC-subsidized and
extractive. The closest non-extractive things (Mastodon, Social.coop) are engineering-culture-bound
and hit the **UX & Cultural Inertia Wall**. The gap: nobody has combined **operational founder speed
early** + a **hardcoded legal democratic sunset** + a **non-speculative internal economy** + **paid
professional builders** in a consumer-grade package.

## Approach — the four pillars

**1. Legal vehicle: a Missouri Chapter 351 Limited Cooperative Association (a "Social Union").**
RSMo §§ 351.1000–351.1228 (Senate Bill 366, 2011) — verified real. It uniquely supports, in one
entity: multi-class membership (§ 351.1090), nonpatron/investor capital with restricted voting,
sweat-equity contributions (§ 351.1135 + contribution agreement § 351.1138), indivisible reserves
(§ 351.1147), internet/cryptographic balloting, and "any lawful purpose" (software). The same LCA/
ULCAA framework exists in Colorado (C.R.S. § 7-58 — Stocksy United's home), Minnesota (Ch. 308B),
Wisconsin (Ch. 193), Wyoming, Illinois, Maryland (Oct 2026). Delaware notably has **no** LCA statute.
*Caveats (FACTCHECK): the "powers of a natural person" provision is `§ 351.1036`, not `§ 351.1015`;
Articles (Form CA 41) fee is `$100` not `$105`; name reservation `$25` not `$20`.*

**2. Governance: progressive decentralization with a hardcoded sunset.** Class A (founder/dev,
super-voting + code veto early); Class B (patron/user, one-member-one-vote, credit-union style); Class
C (nonpatron investor, capped return, no/limited vote). Milestones written as "facts ascertainable
outside the bylaws" (§ 351.1090.2) auto-strip founder power across phases (Alpha → Open Beta → Credit
Union State, e.g. 10k members / reserve threshold / 100k members or 5 years). A supermajority lock
(§ 351.1111) requires ~90% Class B vote to alter the sunset. A person can hold two classes (founder +
investor); governance attaches to the person, financial rights stack separately.

**3. Economy: a closed, non-speculative loop.** Three sources: member dues (at-cost), a capped
revenue-share/royalty for seed capital (RBF / "Demand Dividend" — e.g. 2% of gross, terminating at a
3x cap via § 351.1141; verified real — Purpose Foundation, Haferkater, Start.coop), and tax-deductible
donations routed through a **parallel 501(c)(3)** (fiscal sponsor or two-entity Mozilla model). Value
is locked in **internal utility credits** (offset dues, no token to dump → no bot incentive — the
anti-Steemit) and an **indivisible Unallocated Reserve** (the co-op "endowment," § 351.1147). The
nonprofit owns the open-source code + indexers (funded by grants); the co-op owns the app, nodes, and
member ledger (funded by dues). *Fiscal-sponsor note: valid 501(c)(3) tech sponsors are SPI, SFC,
CS&S; Open Source Collective is a 501(c)(6); the Open Collective Foundation dissolved end-2024.*

**4. Labor as a first-class moral good.** "Burnout is a failure of governance." Bylaws prioritize
market-rate cash salaries for core builders *above* any patronage/investor payout, with a 6-month
salary runway before dividends. Pay labor in **cash** (avoid FLSA pitfalls, no mission-guilt
coercion); reserve patronage/credits for participation. This is the explicit antidote to the Ampled
death.

## The charging mechanism (2026-07-31): metered PDS-hosting, "meter the boundary, not the machine"

source: distilled from `seeds/transcripts/raw/amble-naming-coop-metering-dialogue-2026-07-31.md` (claude.ai;
the "second half" of that conversation). **Proven in code** — the full protocol was built as a
deterministic, adversarial experiment suite that lives on `main`:
`experiments/item-storage-protocol/` (E0–E14 + `SPEC.md` + an independent funder-side verifier + a green
test suite + `RUN_REPORT.md`) and the dependency-free `experiments/item-storage-protocol-standalone/`
(E0–E11). Status **green-real** (assertions pass end-to-end); see `crystallized/proof-ledger.md` +
`test-narrative.md`. This is the concrete answer to what **Pillar 3** left abstract — *how* a
non-extractive co-op actually charges for infrastructure, honestly and verifiably.

**The service (the wedge).** The co-op's flagship non-extractive offering is **PDS-shaped object-store
hosting**: the reference atproto PDS takes any S3-compatible blobstore via environment variables (the
Frontpage-architecture config seam), so an object store built for PDS-shaped needs and offered by a
co-op is a clean wedge. It is the digital analogue of the doc's own **Rural Electric Cooperative** model
— the line the community owns because incumbents won't build it non-extractively.

**Meter the boundary, not the machine** (the user's formulation; see `crystallized/principles.md`). Two
boundary-observable units, both computable by the *member*:
- **Postage = bytes transferred** — bilaterally signed transfer receipts per fixed increment. Walkaway
  exposure is exactly one increment.
- **Rent = byte-days at rest** — computed from the member's **own signed repo**, since a repo plus its
  referenced blob CIDs *is* the storage manifest. The member writes the bill's source of truth in their
  own handwriting.
- **Monthly balance-forward statements**, co-signed by both sides and hash-chained — disputes bound to
  one period; this month only explains the difference from a last month already agreed.

The "trucks and trips vs load" razor (the user's): you cannot cryptographically prove a provider's
internal compute/bandwidth, and you do not need to — meter the **size of the package delivered**,
regardless of how it moved. That makes the provider's internal costs (hops, retries, hardware) *their*
problem to optimize under a price, not something to prove to you.

**Assurance is a declared, cost-priced dial.** Anyone can spot-check: pick items at random, retrieve,
re-fingerprint, compare against the signed manifest. Detection probability is the closed form
`1 − (1 − f)^k` (drop-fraction `f`, sample `k`); checking costs real work, so the **audit dial** (how
often you check) is a member-chosen tier priced **at cost** — "your paranoia, your bill, no judgment
either way." Challenges are seeded by **public randomness** so no arbitrary party can make you burn
cycles.

**Cold storage where the plan is no movement and verification proves it.** The **sealed** tier pins a
root and destroys the provider's write credential (writes fail closed; any direct mutation is caught by
audit against the pinned root; a legitimate unseal is a customer-signed rotation the monitor classifies,
not an alarm). The **tombstone** tier destroys the customer's rotation capability too — frozen for all
parties, audits still verify. ("The tombstone tier is a feature.") This is the missing *thinking*-layer
articulation of the "PDS-as-cold-storage with cryptographic receipts" tier the corpus had only in
`cairn/` as atproto substrate mechanics.

**Mercy is on-book, not off-book — the grace ledger.** Fee waivers, the **deceased-member hold** (rent
accrues to the co-op's own grace account for a fixed term), and throttle-instead-of-cutoff are
**first-class signed ledger entries** that net to zero against a grace account, so the books still
balance and grace is *reportable*. "The receipts make fairness legible; the margin makes mercy
affordable." This is the mechanism under the doc's digital-legacy/estate-trust and
maintenance-first commitments.

**The two-plane split (public/private, always a pair).** A member's **public repo** and their **private
household-effects storage** live on **separate hosts, fail-closed by topology** — privacy is structural,
not a runtime flag (the same "make private data structurally private" lesson the corpus carries as
invariant S5). The **LTS "we won't move the doorknobs" promise** (remodels only by member vote) is the
governance-and-survivability tie.

**The financing side makes Pillar 3's capped royalty mechanical.** E11 encodes an **extinguishing-royalty**
instrument (invest P, receive m·P then stop — no interest, no accrual in loss years, no residual) — the
RBF/"Demand Dividend" in Pillar 3, as another balance-forward ledger: "the return is bounded because the
extraction is bounded; the ledger is how we keep that promise." E12–E14 are the **funder-side diligence
machine**: an outside cooperative-fund lender, holding **no keys**, underwrites the co-op **from the
published ledger files alone** — revenue is **co-attested** (not asserted), audits are replayable from
public randomness, covenants are machine-checkable, and the diligence report is a **build artifact**
("the loan application is a build artifact"). This is a concrete, non-extractive path to *capital* that
doesn't require trusting the co-op's word — a direct advance on the D5 sustainability gate.

**The honest boundary (E14, recorded not hidden).** The whole standard holds **only where the unit is
countable at the boundary by both sides**. The counterexample — a signed "consulting hours" ledger — must
be classified *attested-but-unverifiable*, distinct from *verified*: "the signature attests the count,
and the count disciplines the signature; without a countable unit, it's signed vibes." State this
wherever the mechanism is pitched.

**The delivery-cadence "slow lane" (adjacent, same ethos).** In the tree-out (pull-based) architecture,
refresh cadence is just how often you poll your follows' PDSes — so the novel move is to **declare** it
(a small `delivery cadence: daily` record in your repo, visible to your tree), turning latency from an
inferred slight into a stated fact "like a mailbox that says mail runs once a day." **Slow-lane users
are nearly free to serve, so the economics and the humanity point the same direction** — the
anti-extraction ethos, made cheap.

**Two gates to surface, not resolve (both the user's):**
- **[open, naming] The cooperative and its storage service are unnamed.** The dialogue used "Drystone"
  as a working name for the object-store co-op, but that was a **miscommunication / recall-drift**
  (user, 2026-07-31) — **Drystone is the P2P protocol** (`NAMING.md` → Protocol-layer) and stays that.
  The co-op and its storage service are **yet to be named**; a naming item for the lane, not a blocker.
- **[decision] Legal-review gate (D5).** The receipts-as-contract, the E11 extinguishing-royalty
  instrument, and the deceased-member hold add to the existing Pillar-3 legal-review gate — a
  cooperative attorney/CPA must review before reliance. **NOT LEGAL ADVICE.**

## Reasoning — why this answers what the corpus left open

- **It is the missing sustainability mechanism.** `open-considerations.md` §8 and `ROADMAP_TODO`
  named the cooperative *mechanism* as existential and unresolved. This supplies a concrete,
  legally-grounded one that is **anti-fragile** (a 10% growth drop is leaner spend, not a death
  spiral — no VC burn-rate to feed) where the extractive model is brittle (relies on exploitation;
  churn-and-burn). The mental model for users is the **Rural Electric Cooperative**: build the lines
  the community owns when incumbents won't.
- **It closes the loop with the technical thesis.** Federating on **AT Protocol day-one** solves the
  "empty ghost town" problem (import the Bluesky graph) while running **own PDS + AppView + custom
  Lexicons** gives the "cozy union" private layer — and the credible-exit/forkability promise is the
  strongest co-op selling point. The private-union channels are exactly where Croft's **lineage-groups
  MLS** proof and the **group-privacy-lanes** design plug in (atproto's own Private Data WG does
  in-transit ACLs, *not* E2EE-at-rest — the gap our MLS proof fills). See COHESION.
- **The failure-case lineage hardens the principles.** Ello/Ampled/Steemit/Diaspora/Coomappa give a
  verified anti-pattern set the design explicitly answers: capped exit (not VC veto), grant-paid
  builders (not martyrdom), internal credits (not a speculative token), owned client on open
  standards (not white-label dependency), native indexers (not leaky middleware). These feed
  `crystallized/principles.md` (non-extractive / infrastructure-first / "non-mimicry moat" /
  growth-apathetic / labor-as-first-class).
- **The oppositional read is recorded, not buried.** The biggest real risk is **operational over-
  extension** (the "Complexity Tax" of two entities + the event-app "consultancy trap" + grant-tied
  salary precarity). The user's own resolution: a **Minimal Viable Cooperative** first — single PDS +
  clean AppView + a "cozy" local feed + a founding-member ledger; one revenue engine, one entity, for
  the first ~18 months; events deferred. A **DBA → IP-assignment** path lets a solo founder build the
  working model first, then capitalize the co-op with the finished product at incorporation.

## Growth-apathetic, non-mimicry features (only possible non-extractively)

Protocol-level ad-blocking; transparency-as-a-feature (open-book financial dashboard); a community
micro-grant treasury (member-voted capital allocation); non-algorithmic "human-choice" feeds; inter-
coop SSO/barter; maintenance-first budgeting (≥50–60% to tech-debt/security); user-controlled data
TTL ("digital living room," lower hosting cost); hardware "Node Kits" (members as the backbone);
community-jury moderation (atproto Labeling Service); digital-legacy/estate trusts; a "right-to-
repair" forkability promise. Onboarding antidotes to the inertia wall: soft anonymous "guest pass,"
protocol abstraction (drop jargon), human-curated starter-pack scaffolding, and (later) an **event-app
Trojan Horse** exploiting the big-venue-small-pipe problem with local P2P mesh.

## Open items / pending connective tissue (deferred — concurrent agent active)

- **ROADMAP_TODO:** add the cooperative-mechanism work-list (legal review gate; MVC build; entity-
  structure decision) with origin `file:line`.
- **COHESION:** new entry linking this mechanism ↔ lineage-groups MLS / group-privacy-lanes / the
  atproto Private Data WG finding ↔ the `social-layer` and `governance-and-survivability` docs.
- **`crystallized/principles.md`:** fold the verified failure-case razors (non-extractive, anti-
  fragile, labor-as-first-class, non-mimicry moat).
- **ECOSYSTEM.md:** rows for Platform Cooperativism Consortium, Stocksy, Drivers Coop, Resonate,
  Social.coop, the Purpose Foundation, SPI/SFC/CS&S (relationship: learn↔), flagged dialogue-sourced
  where not independently re-verified.
- **`seeds/transcripts/RAW-ARTIFACTS-MANIFEST.md` + `raw/README.md`:** register the four new raw files
  + FACTCHECKs.
- **The user's decision points to surface, not resolve:** entity structure (single co-op vs co-op +
  501(c)(3) tandem); state of incorporation (MO confirmed-fit); and the lawyer-review gate on all § /
  fee specifics.
