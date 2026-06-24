# Raw transcript — Foundation / coop / IP structure + the foundation-name search (cleaned-paste)

source: research dialogue (claude.ai), pasted 2026-06-23 · **preserved-condensed (cleaned-paste,
content-faithful — NOT a byte-pristine export, PLAYBOOK §4)** · the governance/legal/IP-stewardship
thinking and the long foundation-name exploration that lands on **Noria** as the leading candidate.

> **What this is.** A working session covering (1) how to stand up a nonprofit + coop to own/steward
> the Croft software and brand, the fiscal-sponsorship path, and the three-layer code/brand/coop IP
> architecture; and (2) a long naming exploration for the *foundation* that ends on **Noria** (the
> river-turned water wheel) as the front-runner. Distilled to
> `thinking/foundation-and-ip-stewardship.md` (structure) and `NAMING.md` (the naming work). Advances
> the existential **D5** (cooperative *mechanism*) and partially walks **D8** (centerless-meets-center:
> the legal entity / name registrar / the money). See COHESION §35.
>
> **NOT LEGAL OR TAX ADVICE.** Every legal/financial specific below (MO incorporation $25; IRS 1023-EZ
> $275 / 1023 $600; charitable-solicitation $15; director minimums; assignment-with-goodwill; DCO/CLA;
> certification-mark obligations; the dollar/timeline figures) is **dialogue-sourced from web search and
> requires confirmation by counsel.** The user explicitly flagged the naming + structure as "pending
> legal advice." Treat as orientation, not reliance.
>
> **Naming status (non-negotiable for downstream docs):** **Noria is a leading *candidate*, NOT a
> decision.** The user said "Likely one of these, which I would want legal advice." Do NOT propagate
> "Noria" into structure (doc names, code, the spec) as if adopted — same convention as the unsettled
> app/brand names in `NAMING.md`. **Drystone** (protocol) and **Croft** (umbrella/app) remain the
> settled names.
>
> **Provenance caveats:** cleaned-paste — UI render chrome (search notices, date dividers) stripped;
> user turns preserved closely, assistant turns condensed to their substantive claims (verbatim
> repetition compressed), per the long-dialogue convention. Inline source attributions preserved as
> they appeared (Chamber of Commerce, 501c3.org, Linux Foundation, SPI, Aspiration, etc.) — secondary
> web sources, verify before reliance. No atproto/iroh/iOS claims bear on the FACTCHECK source-of-truth.

---

## SECTION A — Foundation / coop / IP structure

### Q1: most cost-effective way to start a small MO nonprofit to own/sponsor tech-for-good projects?

**User:** What's the most cost effective time and money wise way to start a small nonprofit in MO to
act as the owner and sponsor for a set of tech projects for social and community good?

**Assistant (substance):** First decide if you need your own nonprofit. To *be* the sponsoring entity
you need your own 501(c)(3); to just get tax/legal cover you can use a fiscal sponsor (faster, cheaper,
less autonomy, ongoing fee). Given "owner and sponsor," assume your own entity.

DIY MO 501(c)(3) minimum path: ~$315 hard costs to start. MO incorporation $25; IRS Form 1023-EZ $275
(or full 1023 $600); AG charitable-solicitation registration $15 (required if raising >$10k/yr).
Timeline: state incorporation 1–2 wks; 1023-EZ 2–4 wks; full 1023 3–6 mo; ~4–8 mo end-to-end. MO
requires min 3 directors [later corrected]; you can self-serve as registered agent. The MO Articles
template does NOT meet IRS 501(c)(3) language requirements — must add purpose + dissolution clauses.
Annual: MO annual report (~$20 online), charitable-reg renewal $15, IRS Form 990 (990-N for <$50k).
Use 1023-EZ if eligible (projected <$50k/yr first 3 yrs, assets <$250k). Pay for one thing: a 1–2 hr
attorney review of Articles + bylaws for the IRS-required language. Resources: MOSourceLink, Midwest
Center for Nonprofit Leadership.

### Q2: does a MO nonprofit need a corporate suffix? "association"? and the trademark-ownership model

**User:** A nonprofit needs these in its name? [the "corporation/company/incorporated/limited"
requirement] What about association? And I want the nonprofit to own trademarks, logos, branding etc so
the projects are free to use (as in beer) but also so the particular naming/version published is not
user-confused, and the name/branding would be licensed out to a coop or several. Unless there's a
better model.

**Assistant (substance):** Correction: unlike many states, **MO does not require** a corporate
identifier in a nonprofit's name — "Foo Foundation" / "Foo Collective" is fine. You can also organize
as an unincorporated **association**, a trust, or an LLC, though incorporating as a nonprofit
corporation is the common path.

On the trademark model: what you describe is exactly how serious open-source foundations operate. The
Linux Foundation pattern: trademarks owned by the neutral foundation for use by the project community,
so no single company can capture control via the mark. Open-source licenses let anyone fork the *code*;
the *trademark* is the control point the community holds. Caveat: trademark law requires **active
control** by the owner or you lose the mark — you need a written trademark policy + license agreements
defining permissible use. The "free code / controlled brand" split is exactly Kubernetes, Python,
Firefox. Coop licensing is contract law (flexible). A tech-IP attorney can template it in a few hours.

### Q3: working with a fiscal sponsor instead of forming a foundation — how, and what do you need?

**Assistant (substance):** Mechanics: an existing 501(c)(3) (the sponsor) extends its tax-exempt status
to your project; it accepts tax-deductible donations/grants on the project's behalf and — critically —
the IRS requires the sponsor have **complete discretion and control over the funds** (so it's not a
mere pass-through). That shapes everything: the sponsor legally controls the money, you operate with
oversight, not full autonomy.

What you need before approaching: (1) confirm the work is actually charitable (educational/charitable
public-benefit articulation); (2) a written project proposal (mission, scope, budget, team, fundraising
plan); (3) know your domain + timeframe. Process: find aligned sponsors → apply → sign the agreement
(between sponsor and project *leaders as individuals*, since the project isn't a legal entity; states
the sponsor's complete discretion/control; reporting, payment mechanism, permissible-activity limits)
→ fundraise/operate (donors pay the sponsor; funds earmarked as restricted; you report). Fees ~8–25% of
project revenue (open-source/tech sponsors lower end). **Two things specific to you:** the
trademark/branding question is the friction point — under comprehensive (Model A) sponsorship the
sponsor owns project assets incl. IP, so to hold/license your own marks you need **Model C** (grant
relationship) or explicit IP-carve-out language; raise it early. And you can **graduate** later to your
own 501(c)(3) — negotiate clean spin-off terms (esp. IP/assets) up front.

### Q4: local MO software-for-good umbrella? smaller SPI projects? (the SPI/Debian-marks premise)

**User:** Is there a local MO software-for-good nonprofit umbrella to talk to? Examples of smaller
projects under Software in the Public Interest? I think they work with Debian, yet don't own those
trademarks.

**Assistant (substance):** Premise correction: **SPI *does* own the Debian marks.** "Debian" is a
registered US trademark of Software in the Public Interest, Inc., *managed by* the Debian project. SPI
owns trademarks/money/property but the *management* sits with the projects — SPI is the asset-holding
shell, the community runs the project. SPI has licensed the Debian trademark to outside orgs via formal
license agreements. **This is a working proof-of-concept for your exact plan.** Smaller SPI projects:
ArduPilot (registered trademark of SPI, managed by the project); roster historically incl. PostgreSQL,
Jenkins, OpenWrt, Arch Linux.

Other tech-specific sponsors more relevant than a generic community foundation:
- **Software Freedom Conservancy** — fiscal sponsor + legal assistance, licensing help; Git joined them.
- **Open Source Collective** — nonprofit fiscal host for 2,500+ OSS projects (accounting, invoicing,
  legal); lowest-friction via the Open Collective platform.
- **Aspiration** — closest to "software for social good." Short-term (1–2 yr) sponsorship for projects
  becoming 501(c)(3)s; usually a **grantor/grantee** relationship (project keeps its own board + IP);
  fees 5–15%; apply via form + $150 non-refundable fee + 30-min interview + board decision. Grantee
  model preserves independence/IP.
- **FSF Working Together Fund** — only if aligned with free-software advocacy; 10% admin fee.

No MO-based software-for-good fiscal sponsor found; the established ones (SPI, Conservancy, OSC,
Aspiration) are national and sponsor regardless of state, so geography isn't a constraint. Given the
trademark-holding goal, a tech-native sponsor beats a local generalist. **SPI is the cleanest match for
the asset-holding/trademark model; Aspiration for incubation-toward-independence.**

### Q5: didn't Open Collective close over money?

**Assistant (substance):** Two similarly-named orgs; only one closed. **Open Collective Foundation
(OCF)** — the 501(c)(3) charitable host — **dissolved effective Dec 31 2024** (tripled during COVID
without infrastructure; determined not financially viable; 600+ collectives had to transition). **Open
Source Collective (OSC)** — the one recommended — **did NOT close**; it's a **501(c)(6)** that hosts
only OSS projects, in good financial health. The (c)(6) vs (c)(3) matters: donations to OSC-hosted
projects generally are **not tax-deductible** the way a (c)(3)'s are. So if deductible donations/grants
are central, OSC's (c)(6) status is a real limit and the (c)(3) that filled that gap (OCF) is gone →
pushes back toward **Software Freedom Conservancy** or **Aspiration** (both (c)(3) charitable hosts).

### Q6: SFC vs Aspiration — standing, projects, process

**Assistant (substance):**
**Software Freedom Conservancy** — heavyweight legal-and-infra home for FOSS; founded 2006; 501(c)(3);
40+ member projects; serious legal muscle (GPL compliance/enforcement since 2007, Vizio lawsuit).
Projects: Git, Samba, Inkscape, Mercurial, Wine, Selenium, BusyBox, PyPy, Boost, Twisted, coreboot,
OpenWrt, etc.; formal Debian relationship (Copyright Aggregation Project). **Trademark fit (your core
need): can hold copyrights, trademarks, domain names for projects; provides trademark registration,
policy development, licensing, enforcement** — almost word-for-word your plan. But the bar is strict
and FOSS-purist (exclusively FOSS, OSI-approved/DFSG licenses, developed in public, proven community);
slow rolling review; explicitly "not a service provider." Best trademark-holding fit, hardest entry,
favors mature projects.

**Aspiration** — smaller, social-good rather than FOSS-purist; 501(c)(3); ~1000 events over 22 yrs.
Projects: Coko, Tahoe-LAFS (decentralized storage — maps to your P2P interests), Open Referral. Model:
short-term (1–2 yr) grantor/grantee (project keeps own board + IP); fees 5–15%; advises on governance/
licensing/501(c)(3) incorporation (much pro bono). Open Referral chose them specifically to establish
**stewardship of the project's IP**. Process: application + $150 fee + 30-min interview + board.

Choosing: Conservancy = mature project wanting permanent legally-aggressive neutral mark-holder (best
trademark fit, hardest/slowest entry, permanent). Aspiration = emerging project/leader wanting
incubation toward its own 501(c)(3), keeping board + IP, social-good framing (easier entry, time-boxed,
bridges to the standalone foundation). Both (c)(3) → both fix the deductibility gap OSC can't. **Given
early stage + eventual own-the-marks + cooperative/social-good framing → Aspiration is the natural
start; Conservancy later if it matures into a major FOSS effort.**

### Q7: Aspiration's alignment

**Assistant (substance):** Their fiscal-sponsorship page: they sponsor "mission-aligned technology,
data, and digital rights projects, focusing on open and equitable approaches to **community,
governance, and intellectual property**" — which maps almost exactly onto the user's cooperative-
governance + license-to-coops plan. Broader frame: social/racial/climate justice; power/privilege
analysis. Manifesto principles that screen a project like this: control of technology destiny; power
dynamics + accountability (tech expertise ≠ disproportionate authority — same concern as trademark
neutrality); free-and-open as default; data over software + portability; inclusion/real-world impact
("technology doesn't make a better world, people do"). One tension: "nonprofits should almost never
write their own software" — but they carve out projects *whose mission is building software*; be ready
to justify why the tooling must exist. Strongest pitch framing: equitable user-control mission;
cooperative governance separating brand/asset stewardship from individual contributors; neutral mark-
holding licensed to coops without capture; a clear path to standing independently. Cheapest next step:
a no-fee exploratory email to fiscalservices@aspirationtech.org.

### Q8: the thesis + the three-layer separation (code / brand / coop), and is it viable

**User (substance):** Working out proof-of-concept + "proof of disability" (= proof the problem is real
enough that the ideology is load-bearing). Long game: a foundation to support the software side + a coop
that runs it as a model to others, the two separate-but-linked, with other coops on legally even
footing. Software AGPL-3.0; want to test solution/market fit. Is it viable/interesting/effective?
There's a governance gradient starting from just working-in-the-open, but the thesis is that brand and
trademark are a valuable part of the equation for the average user. Software + branding/trademark + coop
are interrelated but top-heavy if not managed simply. Recommendations, esp. separating open-source code
from branding to start?

**Assistant (substance):** Viable — not hypothetically: foundation-holds-marks / separate-operator-runs
is the SPI/Debian + Linux Foundation pattern; the coop-operator-with-other-coops-on-even-footing twist
is close to platform-cooperative / data-commons structures. Interesting — the genuinely novel part is
**brand/trademark as a load-bearing trust signal for the average user** (not just defensive); the
right model is the **certification mark** (USDA Organic / Fair Trade). Effective — that's the open
question and the right thing to test.

**Top-heaviness is the real risk: don't create legal entities until the thing they'd govern exists.**
Phasing: **Phase 0 (now, proving fit):** no entities, code AGPL-3.0, operate as individual/informal
group, trademark deferred-but-not-ignored. **Phase 1 (need to take money / hold the mark):** a fiscal
sponsor (Aspiration) replaces the need for your own foundation — rent the back office. **Phase 2 (model
is real, others replicate):** spin up the foundation to hold marks neutrally + form the first coop; the
"other coops on even footing" promise is credible *because* the foundation is separate from the first
coop. The separation you want at the end is the reason you can defer building it.

**Separating code from brand from the start (do now, nearly free):** AGPL-3.0 code = maximally open
(forks legal/easy). Trademark = maximally controlled (only conformant software carries the name; a fork
is free to exist, not free to use your name → renames). In Phase 0: put AGPL-3.0 on the code; add a
**TRADEMARK.md** stating the name/logo are NOT covered by the AGPL, are held by you (→ foundation), and
forks must rename (refs: Model Trademark Guidelines, FOSSmarks); use the name with ™ (common-law rights
accrue from use); keep the logo's **copyright assignable** — the artist assigns copyright to you in
writing day one (the Debian-logo cautionary tale). The **certification-mark** idea fits the
brand-as-trust thesis (license to others who meet a published standard; owner doesn't use it on its own
goods) — Phase 2 target.

Mental model — three decoupled layers: **Code** (AGPL-3.0, open, forkable) = the commons; **Brand**
(trademark → eventually certification mark, held by the neutral foundation, licensed on conformance) =
the trust signal; **Operation** (the coop, one of potentially many) = the replicable model. The
foundation's only real job is to hold layer 2 neutrally so layer 3 can be plural — which is why it must
be separate from the first coop and can stay tiny. Honest caution: instrument Phase 0 for the actual
load-bearing question — do users who *could* run an anonymous fork instead choose the branded instance,
and why?

### Q8b: the full thesis (the user's own statement — preserved closely)

**User:** Basically, social and community digital ownership of experience and assets. A p2p protocol
built on the idea that all peers are equal in rights, but not abilities — so a peer to hold longer-term
state and be HA is ideologically compatible to smooth the classic p2p hard cases, but peers are never
more powerful than others in a group. They are delegated roles that are removable and have capabilities
to fulfill them but there is no captive audience. The data is local-first in integrity and durability
but that can include a delegate peer to hold data at a user's discretion. The main app is a composable
platform application plugged into the distributed ecosystem with Bluesky and a private appview (run
operationally by the coop along with the infra needed to operate it) and a group-management protocol
published that is the foundation of the above. So there is the code, the branded released version, the
foundation to accept donations to support it, the coop to run a home for the members. Data is either
E2EE-encrypted or seen only as long as a delegation is valid. The UX is meant to be cohesive,
personable, prioritize long-term UI/UX stability as a user need, and never extractive. The model is a
combination of product thinking, user-rights-enabling but pragmatic on experience being the moat in
both directions; needs protection on identity specifically to build that asset and mindshare; needs a
sustainable fiscal coop that operates on behalf of members but has a core technical driving group whose
control is gradually lessened over time — but the thing is always forkable. The thesis: all these
elements have been seen, but they are needed in combination. Just like a big-tech social is not just a
funding model, or a trademark, or a contract, or a codebase. This is an alternative mechanism, human-
first, not aimed at winning but at preserving and resilience. The option must exist for technology to
serve humanity rather than capital extraction in a loop of lost community assets and an arc of hope and
enshittified despair.

**Assistant (substance — the pushes):** Coherent and consistent; the layers reinforce. Load-bearing
technical idea (symmetric-rights / asymmetric-ability P2P with revocable, scoped, non-capture
delegation) is the right shape — it solves the classic P2P failure (availability reintroduces a server
→ power). Bluesky/atproto choice is consistent (identity separated from host). Five pushes:
1. **"Experience as moat" vs "always forkable" tension** — the honest version: the experience is a
   moat against *extractive competitors*, not against *aligned forks*; data/identity portability keeps
   exit real even when UX is sticky. Test: when someone forks and takes their community, does the stack
   help or resist? It must help.
2. **Brand-as-trust is the most novel + least proven claim** — instrument for it: given a branded
   coop-run instance and an unbranded fork, does the average user prefer the branded one, and is it
   trust/provenance or just default-discovery?
3. **"Gradually lessening core control" fails silently most often** — the dilution must be encoded
   structurally + costly-to-reverse early (board composition shifting founder→member-elected on a
   defined arc), not promised.
4. **"Preserving not winning" is the right soul + a real risk** — resilience without enough adoption
   is a well-architected ghost town; grow only as fast as governance stays honest.
5. Name the actual tiebreaker.

### Q8c: the means determine the end + "every revolution has a maintenance phase"

**User (substance):** The moat point: UX keeps folks out but protects the ecosystem from big-tech
feature-theft — if the UX is good enough that the *model* is valued, it's hard to compete with from an
extraction POV. On brand-as-trust: why do folks go to McDonald's in Paris/Milan? Mental ease, trust,
known quantity. The P2P/distributed space is fighting for *mindshare* more than user base; too much
splinter, complexity, niche privacy/security-only solutions. To socialize ideologically-driven
pragmatism you have to lean into a brand and an identity, or your mindshare gets polluted by less
principled, less UX-focused actors — and success itself makes you a target for scammers/opportunists
who squander the goodwill. Brand is identity the way crypto is identity. On gradual control: yes, get it
right at the start — governance can't be overlaid in hindsight. But 0→1 needs genuine drive/ownership,
so the operating model needs an offramp and a genuine fiscal story that funds itself *and the
maintainers* — not the charity-martyr model (burnout, least-bad choices). Care-giver fatigue is ever-
present; reject that doing good must cost your well-being. The theory: this can ONLY be built this way —
**the means determine the end.** Motto: **every revolution has a maintenance phase.** Better to
demonstrate honest failure than to succeed despite itself. The true principles must be few, load-
bearing, and non-negotiable — that's what makes it what it is, otherwise it's just something else.

**Assistant (substance):** Conceded brand point (predictability-under-uncertainty; the unproven part is
narrower — bootstrap speed of trust in a distrustful audience). Reframed the mark as **anti-
counterfeiting for a community asset** (defensive of users' trust, not mercantile). UX-moat refinement
accepted (asymmetry: competitors can copy the *what*, not the *how-it's-governed*) with the caveat that
the non-extraction must be *legible* as the source of the experience. **The fiscal resolution
(important): separate the economic stake from the governance stake.** Maintainers can have durable fair
compensation for *labor* (employment/contracts) that does NOT dilute on the same schedule as governance
*control*. "Compensation for labor is not capture; control over direction is." The coop is the right
vehicle (worker-member earns a living + has one vote). The offramp is "keep doing the paid work, let go
of the steering on a bound schedule" — survivable, not martyrdom. Proposed a candidate set of
**load-bearing non-negotiables** (1: symmetric rights / asymmetric ability, revocable non-capture
delegation; 2: real exit — portable identity/data, forks can leave; 3: control dilutes, livelihood need
not; 4: non-extraction — data visible only while delegation valid, fast revocation; 5 candidate: neutral
trust anchor — mark held by the foundation, never one operator). Four-or-five, not fifteen. Test: each
must be a knife that forbids a tempting "pragmatic" decision. [User deferred finalizing the list.]

---

## SECTION B — The foundation-name search

### Setup

**User (substance):** Able to build to demonstration-of-value then transition assets to the nonprofit
(mine or others); create code under an open license (AGPL-3.0 — "aggressively protects downstream but
expects the unexpected"; a regional/interop fork is great); principles as the glue ("like the 4 noble
truths"). Wants to talk trademark/assets and how to brand with compatibility, practically/structurally.

**Assistant (substance):** License correction: "APL" is ambiguous — assume **AGPL-3.0** (closes the
SaaS loophole, the strongest *unambiguously-open-source* license; don't go past it or you lose the
open-source guarantee the model rests on). Core structural insight: **license the code, trademark the
name, never let them touch** — AGPL gives away control of the code; the trademark keeps maximal control
of the identity. **Two tiers of mark:** Tier 1 = the house/product mark (Croft), held by the foundation,
licensed tightly (canonical coop-run instance); a fork must rename. Tier 2 = a **compatibility/
interoperability mark** = a **certification mark** (USB-IF Certified / Works-with-Bluetooth / USDA
Organic): owner publishes the standard, doesn't use it on its own goods, licenses it to anyone who
conforms. So a regional fork can't use Tier 1 but can apply for Tier 2 ("Speaks [Protocol]") if it
implements the published group-management protocol. Protocol = the unowned commons; compatibility mark =
the owned-but-openly-licensed badge; house brand = the tightly-held trust anchor.

**Assets the foundation must hold (enumerate so none orphans):** trademarks (house + certification);
the **logo's copyright** (separate from the trademark — assigned in writing by the artist; the Debian-
logo trap); domain names; the **protocol spec** (permissive/open-spec license — open spec, controlled
badge); code copyright / contributor rights. **Contribution rights — decide before outside
contributions:** a **CLA** (broad license incl. relicensing — max flexibility, distrusted) vs a **DCO**
(certify right-to-contribute under the project's license — lighter, more trusted, no relicensing power,
locks you to AGPL forever). Given the values: **DCO + AGPL-only commitment** turns "we can't relicense"
into a principle ("we structurally cannot betray the license").

### Q: "How? As a gift?" (moving assets to the nonprofit)

**Assistant (substance):** Not a "gift" (gift tax = transfers to individuals); it's a **charitable
contribution of property** to a qualified charity (deduction often limited to your *basis* for
self-created IP). To your own foundation: a **contribution agreement / assignment**; watch
private-benefit/inurement (keep the flow one-directional). Instruments per asset class: **trademarks —
assignment WITH the goodwill** (an assignment "in gross" can invalidate the mark; record with USPTO if
registered); **logo copyright — separate written assignment**; **domains — registrar transfer + noted
in the agreement**; **code — your DCO/AGPL-only choice makes this simpler** (code stays AGPL regardless;
foundation holds your copyright + the trademarks). **Timing:** build the mark through use now (™),
transfer to the foundation at Phase 2 via assignment-with-goodwill, foundation files for **®** as owner
(born already owned — no post-registration assignment to record). ~half-day of attorney time if the
paper trail (logo assignment, consistent use, clean contributor terms) is clean.

Confirmed decisions: **unchangeable license — AGPL-3.0-or-later via DCO** (no party, incl. a future you
or captured board, can relicense; "-or-later" defends the *principle* against future loopholes vs
"-only" defends the *text"; lean -or-later; write it into GOVERNANCE.md/charter with the *why*).
**Honor-system compatibility badge** to start (publish a "what this badge asserts / we don't
independently verify" page; keep a revocation clause; transparency is the protection; a formal USPTO
certification mark = the heavyweight path to defer; what you're starting is a lighter regular-mark/
published-badge under a simple usage policy — upgradeable later).

### Q: sequencing — coop first, then foundation via Aspiration?

**Assistant (substance):** Almost right, with a wrinkle: **Aspiration is a stand-in for the
*foundation*, not the *coop*.** A fiscal sponsor (charitable 501(c)(3)) houses the donation/brand-
stewardship function; a **coop** is a member-owned business entity (often not tax-exempt) and is NOT
what a charitable sponsor houses. So: form the **coop when the service is real and has members** (the
operational vehicle); use **Aspiration as the interim foundation** (donations + brand stewardship)
before building your own; graduate to your own **foundation** when the charitable/brand side justifies
the overhead (Aspiration helps spin it out). Verify Aspiration will sponsor a coop-operated project (a
slightly non-standard shape) in the no-fee exploratory call. **"Rent the foundation function for years;
you cannot rent the coop — build it when it's real."**

### Q: the name "Croft," + transfer-and-license-to-coop

**Assistant (substance):** Croft is on-thesis (a croft = a small enclosed holding, individually worked
but communally regulated, with shared common grazing — individual sovereignty + cooperative commons; a
resilience-not-maximizing structure). **Risk to check:** "Croft" is a common surname + famous fictional
(Lara Croft); per-class likelihood-of-confusion means a game character won't block P2P-social infra,
but check USPTO/TESS in software classes (9, 42, 38, 45), web/app-store/domain, for *mindshare*
collision. **Transfer-and-license: yes, exactly the structure** — build the mark now (™), assign to the
foundation at Phase 2 (with goodwill), foundation files ®, foundation **licenses the house mark to the
coop royalty-free WITH quality-control conditions** (free ≠ uncontrolled — the active-control
requirement preserves the mark), and the same terms are **repeatable for future coops** (even-footing
made mechanical: no coop owns the name). Do now: artist's copyright assignment in writing; run TESS +
web + domain checks before the name carries weight.

### Croft clearance check (run in-session)

**User:** I have done some legwork — it was only "Lara Croft" as a unit that was reserved.

**Assistant (substance — the checks):** Legwork close but not complete. Software "Croft" activity:
- **vitali87/croft** — a Rust terminal IDE on Codeberg (docs.croft.software); FOSS, dev-facing, exact
  lowercase styling — the most relevant *mindshare* collision among technical early adopters. Not a
  legal block (dev tool, not social infra).
- **croft.so** — a no-code data-mining/scraping SaaS.
- **withcroft.com** — Croft Technology, Inc., an H-2A ag-labor compliance SaaS (Purdue-born), real
  traction. Different lane.
- **croft-it.com** — a UK managed-IT/software shop.
- Trademark: "TECFRAME" owned by an individual *named* Joseph Croft (Croft as owner, not the mark) — no
  block. No live federal registration for standalone "CROFT" in the class found in this pass (not a
  substitute for a real TESS clearance).
Read: **none is an obvious legal block** (relatedness/likelihood-of-confusion), but the name is **not
clean in the software/dev space** for the brand-as-trust thesis; obvious domains (croft.so/.software/
withcroft.com) are taken. Legally survivable, strategically compromised. Recommendation: either clear it
properly first or pick a more distinctive variant carrying the crofting meaning.

**User (decision):** Still like Croft for the app name; looking at **croft.ing** for the domain; the
foundation would be something like "Techne Croft Foundation"; the protocol is the **peer-to-peer
Drystone protocol** (others demonstrate/claim interop). The software-project collision is small and not
a concern from my POV.

**Assistant (substance):** Drystone for the protocol = the best name in the set (a drystone wall: no
mortar, held by the fit/weight of the stones — structure-without-a-central-binder; durable, repairable;
"Speaks Drystone" works as a compatibility badge; verify the **Dhrystone** CPU-benchmark non-collision —
different spelling). croft.ing = clever (reads "crofting"); caveats: .ing is a newer Google TLD, often
premium-priced (verify *renewal*), slightly less familiar to normies → grab a plain fallback redirect.
Techne Croft Foundation: pushback — "Techne" is heavily used (weak distinctiveness), and **the
foundation name shouldn't foreground "Croft"** — a neutral steward named after one of its assets
undercuts the even-footing/neutrality once it stewards other things (Drystone governance, future
projects). Lean **mission-flavored** for the foundation, vivid for product (Croft) + protocol
(Drystone). [Confirmed .coop is gated/verified — available to the coop once it legally exists; a verified
trust signal; timing lines up with Phase 2.]

### The naming exploration (condensed) — toward the foundation name

The user explored many candidates with the assistant; the through-line was a search for a
**mission-flavored, approachable, ownable** steward name, kin to Croft/Drystone's rural-built landscape,
that allows broad advocacy without fusing to the Croft product. Trajectory:

- Rejected/early: Commonage (mouthful), Commons-alone (generic), Township/Outfield (overloaded),
  Grazings/Marches/Cairn (Cairn: pronunciation trap "care-n"), Husbandry (baggage), Moot (modern
  "moot point"), Techne (overused), Keystone (strong meaning but a known OSS project + crowded),
  Lintel (sounds like "lentil"), Truss (truss/trust echo), Trellis (growth-support, runner-up for a
  while), Spine/Backbone/Keel/Trunk/Bedrock (Bedrock = AWS collision), Foothold (liked imagery, didn't
  land), Loam/Tilth/Seedbed (soil/growth vein), Basin (disliked the word), Artesian (echoes
  "artisanal"), Confluence (Atlassian collision), Headwater (didn't love), Wellspring (loved the
  imagery — but **crowded** in the nonprofit world AND the prominent "Wellspring Philanthropic Fund" =
  a billionaire-backed secretive grantmaking network = the worst-possible association for a
  transparency-first project → dropped), Upwell (verb, awkward as a noun-anchor).

- **Watershed** — the strongest candidate for a long stretch (synthesis of the user's two instincts:
  the Ridge high-dividing-line + life-giving water; "watershed moment" = the ideology; the whole shared
  catchment many flows belong to = the commons). **Rejected on clearance:** "Watershed" is the single
  most common word in literal water-conservation nonprofits (adjacent-field misread), AND — fatally —
  **Watershed Technology, Inc.** is a ~$1.8B venture-backed enterprise-climate-software unicorn (Series
  C $100M; customers incl. BlackRock, Stripe, Walmart) holding **watershed.com**. A direct mindshare
  competitor in the user's own medium → undercuts the clean-trust-anchor thesis; also faintly ironic for
  an anti-extraction project to share a name with a VC unicorn. Set aside, keep the *concept*.

- The register shift that unlocked it: the user kept hunting *natural features*; the assistant noted the
  project is a **built thing that serves people** (like Croft the holding, Drystone the wall) → look at
  **built water infrastructure**. Aqueduct (right register, "too long in the tooth") → the **water
  wheel turned by the river**.

### Landing on Noria

**User:** Is there a name for one of those water wheels that turns with the river?

**Assistant (substance):** A water wheel sits *in* the flow and is turned by it — takes only motion,
gives useful work, lets the water continue undiminished = a near-perfect non-extraction metaphor.
Candidates: Mill (shared communal infra; common/overloaded), Weir (gentle built shaper of flow; short,
rural, ownable), and **Noria** — an undershot bucket-wheel turned by the river's current that lifts
water into a channel to irrigate the fields. Two syllables ("NOR-ee-uh"), warm, distinctive, genuinely
uncrowded, on-thesis (river-turned, lifts the life-giving water to feed growth = "grow your own"),
*built* (kin to Croft/Drystone, signals tech-through-image), water (the through-line). Front-runner.

**User:** I like both [Noria / Weir]. Do research on Noria — meaning, etymology, use.

**Assistant (substance — the research):** **Meaning:** a hydropowered scoop wheel that lifts water into
a small aqueduct for irrigation / town supply; runs purely on the river's kinetic energy (no fuel, no
animals, no human effort once built). Crucially, **unlike a watermill it provides NO mechanical power to
another process** — its sole job is to lift the life-giving water to the people (≈ the foundation's
sole job: bring the sustaining resource to the community, not extract value for something else).
**Etymology:** via Spanish from Arabic *nā‘ūra*, from Syriac, from a root "to roar/growl/bray" — named
for its *voice* ("the growler"): the wooden bearings + falling water + stresses make a "symphony" as it
turns → "the one that sings as it works." **Endurance/resilience:** mostly replaced by motorized pumps
but some survive as early renewable-energy examples; the **Hama, Syria** norias reach ~20 m / 120 boxes,
some reportedly turning **1,000+ years**. **Commons rhyme:** multiple norias share one low dam; excess
flows over when water is high. Landscape coherence: Croft (holding) / Drystone (wall) / Noria (wheel
that lifts the river's water to feed the holding) = one working self-sustaining rural landscape.
**Collision — assessed, far milder than Watershed:** **Noria Corporation** (since ~1998,
machinerylubrication.com) — a niche B2B industrial-machinery-lubrication/reliability content+training
company, a completely different universe from a cooperative digital-rights foundation; near-zero
consumer/tech-audience mindshare overlap. noria.com likely held (→ noria.org / noriacommons.org; .org is
the right TLD for a foundation anyway). **Still requires a real trademark clearance in the user's
classes** (Noria Corp holds marks in *their* classes; different goods → likely navigable, verify).

**User (decision-direction):** Likely **The Noria Foundation** or **Noria Commons Foundation** (leaning
the latter — "Commons" names the cooperative tradition without constraining) — **"which I would want
legal advice."** The image is great.

**Assistant (substance):** Right call to get a real clearance before committing (don't DIY it; Noria
Corp holds marks). The assembled system: **Croft** (app/holding/"grow your own"), **Drystone**
(protocol/unmortared wall/structure-from-mutual-fit), **Noria** (The Noria Foundation / Noria Commons
Foundation — the neutral steward: the wheel the river turns to lift the living water to the community;
holds the marks neutrally so the protocol stays plural and many coops operate on even footing). Open
threads to revisit: the form choice (Foundation vs Commons Foundation) can wait for counsel; everything
underneath still holds (AGPL-3.0-or-later + DCO; foundation-held mark licensed free-with-conditions to
the coop; honor-system compatibility badge; assignment-with-goodwill at Phase 2; phasing with Aspiration
as interim foundation); and the deferred **load-bearing-few principles / genome-vs-strategy** question
remains the real founding act, to revisit fresh.
