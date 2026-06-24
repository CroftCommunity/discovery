# The foundation + IP-stewardship layer — code / brand / coop, and how the marks are held

date: 2026-06-23

source: distilled from `seeds/transcripts/raw/croft-foundation-coop-ip-naming-dialogue-2026-06-23.md`
(the user's own design thinking; treat the *reasoning* as primary). **Sibling to**
`thinking/cooperative-social-union-model.md`: that doc is the *economic/governance mechanism* (the MO
Chapter 351 Limited Cooperative Association); this doc is the *IP-stewardship + foundation* layer — how
the code, the brand, and the coop relate, and how the marks are held so the ecosystem stays plural.
Together they answer the existential **D5** (cooperative *mechanism*) and partially walk **D8**
(centerless-meets-center: the legal entity / name registrar / the money). Naming work → `NAMING.md`.

> **NOT LEGAL OR TAX ADVICE.** Every legal/financial specific here (incorporation costs, IRS forms,
> assignment-with-goodwill, DCO/CLA, certification-mark obligations, fiscal-sponsor terms) is
> dialogue-sourced from web search and **requires confirmation by a nonprofit/IP attorney before any
> filing or reliance.** The user explicitly flagged this whole area as "pending legal advice."

## Problem statement

Croft is a *package*: code + brand/trademark + a cooperative operator, deliberately combined (the thesis
that no single layer is the product — big-tech social is the *fusion* of codebase, funding model,
trademark, and contract). Two failure modes threaten the package. **Top-heaviness:** standing up a
foundation and a coop on day one — before the thing they'd govern exists — burns scarce 0→1 energy on
governance scaffolding for an idea that will change shape. **Capture:** if the brand that signals trust
is owned by the operator, or if any single party can relicense or close the code, the "alternative"
quietly becomes the thing it opposed. The structure has to defer entity overhead *and* make capture
structurally impossible.

## Approach — three decoupled layers, opposite tools for opposite goals

```
   LAYER          TOOL                         RULE                      HELD BY
 ┌──────────┬───────────────────────────┬──────────────────────────┬──────────────────┐
 │ Code     │ AGPL-3.0-or-later + DCO   │ maximally OPEN, forkable │ nobody (license   │
 │          │                           │ — the license gives away │  governs it; the  │
 │ (commons)│                           │   control of the code    │  foundation holds │
 │          │                           │                          │  your copyright)  │
 ├──────────┼───────────────────────────┼──────────────────────────┼──────────────────┤
 │ Brand    │ trademark → (later) a     │ maximally CONTROLLED —   │ the neutral       │
 │          │ certification/compat. mark│ only conformant software │ FOUNDATION        │
 │ (trust)  │                           │ carries the name/badge   │ (licensed out)    │
 ├──────────┼───────────────────────────┼──────────────────────────┼──────────────────┤
 │ Operation│ a cooperative (one of     │ runs the canonical       │ the COOP (under   │
 │          │ potentially many)         │ instance under license;  │ a free, condition-│
 │ (model)  │                           │ earns the brand          │ ed mark license)  │
 └──────────┴───────────────────────────┴──────────────────────────┴──────────────────┘
```

**Code and brand are opposite tools and must never touch.** AGPL-3.0 says *take this, run it, change
it, fork it* — a regional/interop fork is the point, and the license guarantees it can happen. The
trademark says *but you cannot call your version by our name unless it meets the standard* — a fork is
free to exist, not free to impersonate. The fork is free; the *confusion* is what the mark prevents.
This is the lever that makes "brand as a trust signal for the average user" actually function: the user
can trust the name precisely because the name means one specific, conformant thing. Framed honestly, the
mark is **anti-counterfeiting for a community asset** — defensive of users' trust, not mercantile —
which matters most exactly when success makes the project a target for scammers wearing its face.

**Two tiers of mark.** Tier 1 = the **house mark** (the product name/logo, e.g. Croft), held by the
foundation, licensed tightly to the canonical coop-run instance; a fork must rename. Tier 2 = a
**compatibility / interoperability badge** for the published protocol (e.g. "Speaks Drystone") — the
honest place for forks: a fork can't use the house mark but can fly the compatibility badge if it
implements the protocol. The protocol spec itself is the **unowned commons** (permissive/open-spec
license — open spec, controlled badge); independent implementations are encouraged.

**The compatibility badge starts as an honor system, not a certification program.** Publish a plain
"what this badge asserts / we do not independently verify" page; keep a revocation clause; let
transparency be the enforcement (a lie is visible). A formal USPTO **certification mark** carries real
obligations (control the standard, don't use it on your own goods, don't discriminate among qualifiers)
— the heavyweight path to defer. What starts is a lighter regular-mark/published-badge under a simple
usage policy, upgradeable later. Open sub-question (deferred): does flying the badge assert only
*technical* interop, or also adherence to the **non-negotiable principles**? — i.e. is the badge where
the principles get their teeth? (Ties to the genome-vs-strategy question in
`crystallized/principles.md` Tier 1 and `cooperative-social-union-model.md`.)

## The structural openness lock — AGPL-3.0-or-later via DCO

Decided (recorded as the user's design intent, not a legal filing): license the code
**AGPL-3.0-or-later** and take contributions under a **DCO** (Developer Certificate of Origin), *not* a
CLA. A CLA would collect relicensing rights in one party — a power that sits uneasily with "control
dilutes." A DCO grants no relicensing power, so **no party — not a future founder, not a captured board,
not an acquirer — can relicense or close the source.** The inability to relicense becomes the feature: a
credible commitment in the game-theory sense (you remove your own ability to defect, which is what makes
the promise trustworthy downstream). "-or-later" (vs "-only") defends the *principle* (network-use
copyleft stays closed to capture) against future loopholes, at the cost of trusting the FSF with forward
power — lean "-or-later". Write the commitment, and the *why*, into `GOVERNANCE.md` / the foundation
charter. A quiet benefit: this also simplifies the eventual asset transfer — the code stays AGPL in
everyone's hands regardless, so the foundation holds the brand and your copyright, not a code monopoly.

## Entity phasing — don't build a governor before the thing it governs exists

- **Phase 0 (now — proving fit):** no legal entities. Code AGPL-3.0-or-later; operate as an individual
  / informal group. Cheap to stay here. Do the near-free brand-separation now: a `TRADEMARK.md` stating
  the name/logo are NOT covered by the AGPL and forks must rename (refs: Model Trademark Guidelines,
  FOSSmarks); use the name with **™** (common-law rights accrue from use); get the **logo copyright
  assigned in writing by the artist** day one (the Debian-logo trap — they couldn't act on their own
  logo for years because the artist kept copyright). Instrument the load-bearing question: do users who
  *could* run an anonymous fork instead choose the *branded* instance, and why?
- **Phase 1 (need to take money / hold the mark):** a **fiscal sponsor replaces the need for your own
  foundation** — rent the back office. **Aspiration is the interim foundation** (see below). Key
  distinction: a fiscal sponsor (charitable 501(c)(3)) stands in for the *foundation* function, NOT the
  *coop*. The coop is a member-owned business entity and is not what a charitable sponsor houses.
- **Phase 2 (model is real, others want to replicate):** spin up the **foundation** to hold the marks
  neutrally + form the **first coop** as the reference operator. The "other coops on even footing"
  promise is credible *because* the foundation is separate from the first coop. Form the **coop when the
  service is real and has members** (the operational vehicle); graduate to your **own foundation** when
  the charitable/brand side justifies the overhead. **Rent the foundation function for years; you cannot
  rent the coop — build it when it's real.** The `.coop` TLD (gated/verified, cooperative-only) becomes
  available to the coop here and is itself a verified trust signal.

## How the marks move, and how the coop uses them

- **Transfer = a charitable contribution of property, not a "gift"** (gift tax = transfers to
  individuals). To one's own foundation: a contribution agreement / assignment; keep the flow
  one-directional to avoid private-benefit/inurement.
- **Per asset class:** trademarks via **assignment WITH the goodwill** (an assignment "in gross" can
  invalidate the mark; record with USPTO if registered); logo **copyright** via a separate written
  assignment; **domains** via registrar transfer noted in the agreement; **code** mostly stays put (the
  DCO/AGPL-only choice means the foundation holds your copyright + the marks, not a code monopoly).
- **Timing:** build the mark through use now (™) → assign to the foundation at Phase 2
  (with-goodwill) → foundation files for **®** as owner (born already owned; no post-registration
  assignment to record). ~half-day of attorney time if the paper trail is clean — which is the argument
  for getting the logo assignment + consistent use + clean contributor terms right in Phase 0.
- **The coop's license:** the foundation licenses the house mark to the coop **royalty-free but WITH
  quality-control conditions** (free ≠ uncontrolled — the active-control requirement is what preserves
  the mark). The same terms are **repeatable for future coops** → even-footing made mechanical: no coop
  owns the name; each operates under license from the neutral foundation, and the first has no
  structural privilege over the second. This is the whole reason the foundation must be separate from,
  and named independently of, the first coop (→ `NAMING.md`: the foundation name is mission-flavored,
  not flagship-fused).

## Fiscal-sponsor landscape (the Phase-1 interim foundation)

Tech-native sponsors beat a local generalist because holding/licensing trademarks is central here.
(Rows registered in `ECOSYSTEM.md`; verify all terms — dialogue-sourced.)

- **Aspiration** — the recommended **interim foundation**. 501(c)(3); short-term (1–2 yr)
  **grantor/grantee** model → the project keeps its own board + IP (friendly to holding your own marks
  and graduating to your own 501(c)(3)). Sponsors "mission-aligned technology, data, and digital rights
  projects, focusing on open and equitable approaches to community, governance, and intellectual
  property" — near-exact fit. Fees 5–15%; apply via form + $150 fee + 30-min interview. **Open item:**
  verify they'll sponsor a *coop-operated* project (a non-standard shape) in the free exploratory call.
- **Software in the Public Interest (SPI)** — the **proof-of-concept for the whole plan**: SPI *owns*
  the Debian trademark (registered US mark), *managed by* the Debian project, and has licensed it to
  outside orgs. The asset-holding-shell / community-runs-it model, exactly Croft's intent. Also holds
  ArduPilot's mark. Best long-term asset-holding match; fits Phase 2+.
- **Software Freedom Conservancy** — 501(c)(3); can hold copyrights, trademarks, domains and provides
  trademark registration/policy/licensing/enforcement (word-for-word the plan), plus serious legal
  muscle. But a strict FOSS-purist bar (OSI/DFSG licenses, mature public community), slow rolling
  review, "not a service provider." The permanent neutral mark-holder if the project becomes a major
  FOSS effort.
- **Open Source Collective** — a **501(c)(6)** (trade association), so donations generally are **not
  tax-deductible** — a real limit if charitable grants/donations are central.
- **Open Collective Foundation** — the (c)(3) that filled that gap — **dissolved Dec 31 2024** (growth
  without infrastructure; not viable). Do not plan around it.
- **FSF Working Together Fund** — only if aligned with free-software advocacy specifically; 10% fee.

## Reasoning — why this is the IP half of the sustainability answer

- **It makes capture structurally impossible, not merely promised.** The DCO/AGPL lock removes anyone's
  ability to close the source; the foundation-held-mark-licensed-to-coops makes the brand un-hoardable;
  the separation of foundation from coop makes even-footing real. These are the IP-layer expression of
  `crystallized/principles.md` Tier 1 ("equal in rights, not capabilities"; "no right to remove the
  rights of others") and the candidate non-negotiable "neutral trust anchor."
- **It defers overhead without deferring the guarantees.** Phasing rents the foundation function
  (Aspiration) for years and builds the coop only when real — answering the top-heaviness risk the user
  named — while the near-free Phase-0 brand-separation (TRADEMARK.md, ™, logo assignment) preserves the
  asset the whole trust thesis rests on.
- **It is the IP/legal-entity half of D5 and the named answer to part of D8.** `cooperative-social-
  union-model.md` supplies the *economic/governance* mechanism; this supplies *who holds the brand and
  code, and how*. Together they give D8's "legal entity to sue/bill, name registrar, the money" a
  concrete shape that is arranged to stay plural (many coops) rather than growing a center at the seam.
- **The oppositional read is recorded.** The brand-as-trust thesis is the most novel and least-proven
  claim — instrument Phase 0 for it (branded-vs-fork preference, and *why*). And "experience as a moat"
  is honest only as a moat against *extractive competitors*, never against *aligned forks* — real exit
  (portable identity/data) must keep the fork-path open, or the structure becomes the lock-in it opposes.

## Open items / decisions to surface (not resolve)

- **[decision] Legal-clearance + entity gate** — a nonprofit/IP attorney confirms the structure, the
  assignment-with-goodwill mechanics, and the trademark clearances (incl. the foundation name; see
  `NAMING.md` / ROADMAP_TODO). The user owns this.
- **[decision] Fiscal-sponsor selection** — the free exploratory call with Aspiration (confirm the
  coop-operated shape). Relates `cooperative-social-union-model.md` (the 501(c)(3) tandem).
- **[decision] Foundation-vs-coop entity structure + form** — single co-op vs co-op + 501(c)(3) tandem
  (shared with the cooperative-model doc's open item).
- **[deferred] The load-bearing-few principles (genome vs strategy)** — the real founding act; the
  compatibility badge may be where they get teeth. Revisit fresh.
