# Reference index: governance (Layer 7, the manifestation)

Every source cited across the governance layer — the foundation-plus-cooperative structure, the
IP-stewardship and open-publication posture, the improvement-paradox model, and the cooperative
prior-art register — grouped by type, with the doc that relies on it, its epistemic-status tag, and
a primary/secondary marker. This is the per-layer source-of-record for the four governance content
docs; it does not introduce new claims and it carries only external sources.

The governance layer is the *manifestation*: it builds the form. The *argument* for the form (and
the case law behind it) lives in the philosophy layer and is inherited by cross-reference, not
re-derived here — which is why the "Legal cases" group below is a pointer, not a native list (see
Coverage / gaps).

Marker key:

PRIMARY = the canonical artifact itself (the license text, the statute, the published paper or book,
the founding essay, the timestamping service).

SECONDARY = a living institution, movement, or practice named as an existence-proof or precedent and
cited by description, not from the organization's own primary records.

CROSS-LAYER = the load-bearing source is cited and carried in a *different* beta layer (here:
philosophy); this index points to it rather than re-attributing it to governance.

Verification flags are preserved inline exactly as the source docs carry them:
`[verified]`, `[dialogue-sourced — verify before reliance]`, `[UNVERIFIED]`.

NOT-LEGAL-ADVICE = legal or financial material carried as design *reasoning*; every statute section,
fee, and figure requires a cooperative, nonprofit, and IP attorney before any filing or reliance.
The statute names below are the reasoning's referents, not resolved citations.

Sibling docs referenced by short name:
`foundation-cooperative-and-sustainability.md` (the manifestation),
`open-publication-and-ip-stewardship.md` (IP + publication),
`making-preventative-work-visible.md` (the improvement paradox),
`cooperative-and-governance-prior-art.md` (the prior-art register).

---

## Legal cases

The governance layer cites **no case law of its own**. The legal foundation of the form —
*eBay Domestic Holdings v. Newmark* (Del. Ch. 2010), *Dodge v. Ford Motor Co.* (Mich. 1919), and the
rights-lineage line *Hush-A-Phone Corp. v. United States* (D.C. Cir. 1956) and *Carterfone* (FCC,
1968) — is cited and carried in the **philosophy layer** (`peer-standing-and-the-cooperative-form.md`,
`structural-argument-principles.md`, `the-peer-rights-razor-and-its-lineage.md`), and the governance
docs inherit it by cross-reference ("it inherits that basis from philosophy and does not re-derive
it"). CROSS-LAYER. NOT-LEGAL-ADVICE. Indexed here as a pointer so the layer's legal grounding is
locatable; these cases belong in the philosophy layer's reference index, not this one.

---

## Statutes & licenses

NOT-LEGAL-ADVICE applies to the statute row: the governance docs deliberately carry the *reasoning
and shape*, not the statute sections, fees, or forms, which are gated to counsel.

Missouri Chapter 351 (the Missouri limited-cooperative-association statute the *Social Union* would
incorporate under). Statute, dialogue-sourced, carried as reasoning only. PRIMARY (statute) / carried
NOT-LEGAL-ADVICE — `[dialogue-sourced — verify before reliance]`. Relied on by
`foundation-cooperative-and-sustainability.md` (the four-pillar legal-vehicle discussion and the
legal-review decision gate) and echoed in `cooperative-and-governance-prior-art.md` (the
legal-review-gate section, which names the "limited cooperative association" form without resolving
the statute). Locator: Missouri Revised Statutes, Chapter 351.

CC0 1.0 Universal (public-domain dedication). Spec-text license, the current decided choice
(user-approved 2026-06-25), chosen over CC-BY 4.0 for maximal "no one can claim or restrict the idea."
PRIMARY (canonical license text). documented-fact. Relied on by
`open-publication-and-ip-stewardship.md` (license posture; the CC0 dedication in the
defensive-publication notice). Locator: https://creativecommons.org/publicdomain/zero/1.0/

CC-BY 4.0 (attribution-only). Named as the earlier research recommendation and a coherent alternative
to CC0, not the chosen license. PRIMARY (canonical license text). documented-fact. Relied on by
`open-publication-and-ip-stewardship.md`. Locator: https://creativecommons.org/licenses/by/4.0/

Apache-2.0 (permissive license with an express patent grant + retaliation clause). **Superseded 2026-07-09
(C13): was the earlier reference-code choice (A14); the reference implementation is now AGPL-3.0-or-later +
DCO.** Retained here only as the rejected permissive option (its patent grant is preserved under AGPLv3
§11); MIT and BSD (permissive, patent-silent) were the further-rejected permissive alternatives *within*
that superseded Apache-2.0 evaluation, and the C13 rewrite removed them from the content docs, so they no
longer carry a standalone citation. PRIMARY (canonical license text). documented-fact. Locator:
https://www.apache.org/licenses/LICENSE-2.0

AGPL-3.0-or-later (network-copyleft license, with an express patent grant at §11). **The reference-code
license (decided C13, 2026-07-09) and the structural-openness lock on the code / neutral stack**, paired
with a DCO. PRIMARY (canonical license text). documented-fact. Relied on by
`foundation-cooperative-and-sustainability.md` (the three-decoupled-layers table and the "-or-later"
copyleft-loophole reasoning), `open-publication-and-ip-stewardship.md` (reference-code license), and named
in `cooperative-and-governance-prior-art.md` (the "AGPL+DCO lock"). Locator:
https://www.gnu.org/licenses/agpl-3.0.html

MPL-2.0 — Mozilla Public License 2.0 (weak/file-level copyleft). **A mandatory, non-substitutable license
constraint on the neutral stack**, not a chosen preference: the `hpke-rs` substrate dependency is MPL-2.0,
HPKE is mandatory for MLS per RFC 9420, and there is no pure-permissive substitute — both the rust-crypto
and libcrux HPKE providers route through MPL-2.0 code (Decision A1). PRIMARY (canonical license text).
documented-fact. Relied on by `open-publication-and-ip-stewardship.md` (license posture, the substrate
dependency at :40). Locator: https://www.mozilla.org/en-US/MPL/2.0/

DCO — Developer Certificate of Origin (contribution instrument, not a license). Chosen over a CLA
because a CLA collects relicensing rights in one party (a capture vector); a DCO grants none.
PRIMARY (canonical text). documented-fact. Relied on by
`foundation-cooperative-and-sustainability.md`. Locator: https://developercertificate.org/

CLA — Contributor License Agreement (named as the rejected instrument). documented-fact. Relied on by
`foundation-cooperative-and-sustainability.md`.

---

## Cooperative & governance precedents

Existence-proofs and governance-DNA. All are cited by description as living institutions or documented
mechanisms; none is cited from its own primary corporate records, so all are SECONDARY with the
source docs' inline verification flags preserved. Formal scholarly/founding texts behind some of these
are listed under Papers & scholarship and cross-referenced here.

The named movement:

Platform Cooperativism Consortium (Trebor Scholz, The New School) — the research hub and movement for
user- and worker-owned platforms; supplies the vocabulary and scholarly home for the entire form.
SECONDARY. existence-proof / named-movement. `[verified 2026-06-22]`. Relied on by
`cooperative-and-governance-prior-art.md`. Founding text under Papers & scholarship (Scholz 2014).
Locator: platform.coop.

Existence-proofs (the form is buildable):

Stocksy United (Colorado limited cooperative association) — profitable multi-class platform coop; the
direct model for the Social Union's multi-class design. SECONDARY. existence-proof. `[verified]`.
`cooperative-and-governance-prior-art.md`. Locator: stocksy.com.

The Drivers Cooperative (NYC) — driver-owned ride-share; proves a coop can out-margin a venture
monopoly by paying contributors more. SECONDARY. existence-proof. `[verified]`.
`cooperative-and-governance-prior-art.md`. Locator: drivers.coop.

Resonate — music-streaming cooperative; stream-to-own; proves value can route to ownership without a
speculative token (the internal-credit precedent). SECONDARY. existence-proof. `[verified]`.
`cooperative-and-governance-prior-art.md`. Locator: resonate.coop.

Social.coop — member-owned, dues-funded Mastodon instance via Open Collective; the closest living
relative to the Social Union. SECONDARY. existence-proof. `[verified]`.
`cooperative-and-governance-prior-art.md`. Locator: social.coop.

Mondragon — worker-owned cooperative federation at industrial scale; proves the form scales into a
durable federation. SECONDARY. existence-proof. `[verified 2026-06-22]`.
`cooperative-and-governance-prior-art.md`. Locator: mondragon-corporation.com.

Green Bay Packers — the only community-owned team in the major US leagues (subsequently banned by
league rule); proves community ownership can run a major competitive institution. SECONDARY.
existence-proof. `[verified 2026-06-22]`. `cooperative-and-governance-prior-art.md`.

The credit-union lineage (Schulze-Delitzsch → Raiffeisen → Desjardins → Filene) — the centuries-durable
member-owned financial-institution model; source of the "Not for profit, not for charity, but for
service" motto. SECONDARY. existence-proof / historical lineage. Lineage `[verified 2026-06-22]`; the
motto attribution `[dialogue-sourced 2026-06-22 — verify attribution before reliance]`.
`cooperative-and-governance-prior-art.md`.

The rug-pull disarmed (unsellable ownership / credible exit as existence-proofs):

Signal / the Signal Foundation — 501(c)(3) nonprofit (launched Feb 2018, ~$50M contribution from WhatsApp
co-founder Brian Acton) that wholly owns Signal Messenger; architecturally centralized but uncapturable
because the entity legally cannot be sold ("governance, not decentralization"). Whittaker's structural
logic and Marlinspike's no-VC posture are carried as attributed synthesis; the verbatim Whittaker line is
held for a later pass. Proves the ownership-structure defense. SECONDARY. existence-proof.
`[documented history — the $50M / $38M expenses / $29M revenue / $19B figures research-sourced, confirm
against Signal Foundation filings before external reliance]`. Relied on by
`cooperative-and-governance-prior-art.md`. Also used in `../activism/platform-extraction-and-captured-labor.md`
as the "contingent not intrinsic" counterweight. Locator: signalfoundation.org.

Dreamwidth — open-source fork of the LiveJournal codebase with a full account importer (posts, comments,
tags, icons); the credible-exit existence-proof used when LiveJournal moved its servers to Russia (Dec
2016). Proves a forkable codebase plus a working importer is a real exit that disarms the rug-pull.
SECONDARY. existence-proof. `[documented history]`. Relied on by
`cooperative-and-governance-prior-art.md`. Locator: dreamwidth.org.

Archive of Our Own (AO3) / the Organization for Transformative Works — nonprofit, non-commercial,
open-source, donation-funded, ad-free archive built in 2007 as a direct response to a rug-pull (FanLib +
LiveJournal's "Strikethrough" mass deletion, May 29 2007); tens of millions of works, 8M+ users, 2019 Hugo
Award. Proves a burned community rebuilds durably on infrastructure it owns and cannot be sold. SECONDARY.
existence-proof. `[documented history]`. Relied on by `cooperative-and-governance-prior-art.md`. Also used
in `../activism/platform-extraction-and-captured-labor.md`. Locator: archiveofourown.org / transformativeworks.org.

Governance DNA (how a commons stays governed at scale):

The Rochdale Principles (Rochdale Society of Equitable Pioneers, 1844) — the founding governance code
of the modern cooperative movement, still the backbone of the International Co-operative Alliance's
principles; the "Education of the Members" principle is load-bearing. SECONDARY. established history.
`[established history; Rochdale Pioneers, 1844]`. `cooperative-and-governance-prior-art.md`. Locator:
International Co-operative Alliance, ica.coop (statement of cooperative identity).

Elinor Ostrom's commons work (Törbel, Valencia huertas, Bali Subak, Maine lobster grounds) —
polycentric governance + subsidiarity as the scale answer. SECONDARY. established scholarship.
`[established scholarship; Ostrom, Governing the Commons, 1990]`.
`cooperative-and-governance-prior-art.md`. Formal work under Papers & scholarship.

Liquid Feedback / liquid democracy (German Pirate Party; comparable Google Votes) — instantly-revocable
per-topic vote delegation; carried *with* its documented failure mode (delegation concentration) and
antidotes (decay, per-delegate caps, bounded chains, expiry, visibility). SECONDARY.
governance-mechanism. `[dialogue-sourced 2026-06-20 — verify before reliance]`.
`cooperative-and-governance-prior-art.md`.

Commons-DAO research (De Filippi, Rozas, et al.) — Ostrom-grounded alternative to code-is-law; treats
forking as legitimate pressure. SECONDARY here for the governance-DNA framing; the paper itself is
PRIMARY under Papers & scholarship. `[dialogue-sourced; Frontiers DOI verified]`.
`cooperative-and-governance-prior-art.md`.

Seed capital without capture:

The Purpose Foundation / steward-ownership — ownership held in stewardship for the mission and
unsellable; separates control from economic rights. SECONDARY. seed-capital-instrument. `[verified]`.
`cooperative-and-governance-prior-art.md`. Locator: purpose-economy.org.

Revenue-Based Financing / the Demand Dividend — capped-return investor models (financial right, no
governance right, stake vanishes at the cap); documented instances Haferkater and Start.coop. SECONDARY.
seed-capital-instrument. `[verified]`. `cooperative-and-governance-prior-art.md`. Locator: start.coop.

The fiscal-sponsor analysis (the concrete Phase-1 vehicle):

Software in the Public Interest (SPI) — owns the Debian trademark (managed by the Debian project) and
holds other projects' marks (ArduPilot among them); the trademark-stewardship proof-of-concept for the
foundation-holds-the-marks / cooperative-operates-under-license intent. SECONDARY. fiscal-sponsor /
mark-holding vehicle. `[dialogue-sourced 2026-06-23 — verify before reliance]`.
`cooperative-and-governance-prior-art.md`. Locator: spi-inc.org.

Software Freedom Conservancy (SFC) — permanent neutral 501(c)(3) mark-holder with GPL-enforcement
muscle; the Phase-2-and-beyond permanent-home option. SECONDARY. fiscal-sponsor vehicle.
`[dialogue-sourced 2026-06-23 — verify before reliance]`. `cooperative-and-governance-prior-art.md`.
Locator: sfconservancy.org.

Aspiration — 501(c)(3) fiscal sponsor with a short-term grantor/grantee model where the project keeps
its board and IP; the recommended interim Phase-1 foundation. Open seam: whether it will sponsor a
cooperatively-operated project. SECONDARY. fiscal-sponsor vehicle.
`[dialogue-sourced 2026-06-23 — verify before reliance]`. `cooperative-and-governance-prior-art.md`.
Locator: aspirationtech.org.

Code for Science & Society (CS&S) — 501(c)(3) fiscal sponsor of public-interest tech and open-science
projects; a fourth same-class option for the grant-funded code side. SECONDARY. fiscal-sponsor vehicle.
`[dialogue-sourced — verify before reliance]`. `cooperative-and-governance-prior-art.md`. Locator:
codeforsociety.org.

Open Collective (correction carried so it is not reached for by mistake): the Open Collective
*Foundation* dissolved end of 2024; the Open Source Collective is a separate 501(c)(6), not a
charitable 501(c)(3). SECONDARY. correction / documented-fact. `[verified]`.
`cooperative-and-governance-prior-art.md`.

The anti-pattern failure lineage (the negative specification each pillar answers):

Ello (VC capital for an anti-ad mission, no revenue engine, went quiet and wiped data); Ampled
(uncompensated-volunteer musician coop, closed 2023 citing terminal burnout); Steemit (tokenized every
interaction, bot-farmed, hostile takeover); Diaspora (decentralized social network undone by
architectural naivety); Coomappa (owned a cooperative but actually owned a technological dependency,
captured by its white-label platform). SECONDARY. failure-pattern. `[verified 2026-06-22 as a failure
pattern]`. Relied on by `cooperative-and-governance-prior-art.md` and named in
`foundation-cooperative-and-sustainability.md` (the failure-lineage section; the private-chat-platform
labor-extraction figures there are `[UNVERIFIED]` — private company — while the labor reasoning is the
load-bearing part).

---

## Papers & scholarship

Repenning, N. P., & Sterman, J. D. (2001). "Nobody Ever Gets Credit for Fixing Problems That Never
Happened: Creating and Sustaining Process Improvement." *California Management Review*, Summer 2001.
The improvement-paradox model (Work-Harder vs Work-Smarter loops; capability as an eroding stock).
PRIMARY (the paper). management-scholarship. The paper, authors, and year stand as cited; the *figures
and quotations* drawn from it are `[UNVERIFIED]` and to be confirmed against the source before
publication. Relied on by `making-preventative-work-visible.md` (the whole doc). Locator: *California
Management Review* 43(4), Summer 2001 (search title via CMR / JSTOR).

Scholz, T. (2014). "Platform Cooperativism vs. the Sharing Economy" (founding essay). Names the
pattern the entire form adopts. PRIMARY (the essay). founding-text. `[verified 2026-06-22]`. Relied on
by `cooperative-and-governance-prior-art.md` (the named-movement section). Locator: essay, 2014,
widely mirrored (Medium, Trebor Scholz).

Ostrom, E. (1990). *Governing the Commons: The Evolution of Institutions for Collective Action.*
Cambridge University Press. The empirical case for polycentric commons governance. PRIMARY (the book).
established scholarship. `[established scholarship]`. Relied on by
`cooperative-and-governance-prior-art.md` (governance DNA). Locator: Cambridge University Press, 1990
(ISBN 978-0521405997).

De Filippi, P., Rozas, D., et al. (2023). "DAO design for the commons" (Ostrom-grounded DAO governance).
*Frontiers in Blockchain.* PRIMARY (the paper). peer-reviewed-finding. `[Frontiers DOI verified]`.
Relied on by `cooperative-and-governance-prior-art.md` (governance DNA). Locator:
https://doi.org/10.3389/fbloc.2023.1287249

Schelling, T. C. (1960). *The Strategy of Conflict.* Harvard University Press. The source of the
*credible commitment* concept — a commitment becomes believable by binding your own hands so reneging is
no longer an available move — invoked in the DCO-over-CLA reasoning ("remove your own ability to defect").
PRIMARY (the book). established scholarship. `[attribution to be confirmed against the primary edition
before external use]`. Relied on by `foundation-cooperative-and-sustainability.md` (the AGPL+DCO
structural-openness lock). Locator: Harvard University Press, 1960.

---

## Publication infrastructure & venues

The prior-art timestamping vehicles and the per-layer venue map from
`open-publication-and-ip-stewardship.md`. The venue-map rows are target communities / standards bodies
rather than cited claims; they are indexed here as the layer's publication surface.

Timestamping & archival (the prior-art vehicles):

Zenodo — CERN-run archival repository; the primary prior-art timestamp vehicle (a Concept DOI across
versions plus a version DOI per frozen draft, with GitHub-release integration). PRIMARY (service).
publication-infrastructure. `[UNVERIFIED]` (Zenodo specifics dialogue/web-sourced 2026-06-24). Relied
on by `open-publication-and-ip-stewardship.md`. Locator: https://zenodo.org

OpenTimestamps — document-hash anchor as a complementary tamper-evident witness. PRIMARY (service).
publication-infrastructure. `[UNVERIFIED]` (mechanics dialogue/web-sourced 2026-06-24).
`open-publication-and-ip-stewardship.md`. Locator: https://opentimestamps.org

GitHub release tags + Wayback Machine capture — complementary dated witnesses. PRIMARY (services).
publication-infrastructure. `open-publication-and-ip-stewardship.md`. Locators: github.com,
https://web.archive.org

arXiv — cs.NI primary (cross-list cs.CR, cs.DC) preprint for prior-art strength, reach, and academic
credibility. PRIMARY (service). publication-infrastructure. `open-publication-and-ip-stewardship.md`.
Locator: https://arxiv.org

IETF Internet-Draft — named as the *reconsidered* earlier timestamp vehicle (set aside for the
Zenodo-first posture). documented-decision. `[UNVERIFIED]` (IETF Trust draft-reuse terms
dialogue/web-sourced 2026-06-24). `open-publication-and-ip-stewardship.md`.

Venues & standards bodies (the per-layer venue map; each a target community, not a cited finding):

iroh / number 0 community (transport); OpenMLS maintainers + IETF MLS WG (group crypto); W3C Credentials
Community Group (device identity / DIDs); IETF Authenticated Transfer WG (atp) + W3C Social Web WG +
atproto dev community (public-social side); Willow / Earthstar / p2panda peers + Malleable Systems forum
(data model / membership / sync); IRTF DINRG + ANRW workshop (decentralization thesis); Local-First Conf
(flagship public moment); PoPETs and NSDI (peer-reviewed full-paper venues). SECONDARY (named venues).
standards-venue / target-community. Relied on by `open-publication-and-ip-stewardship.md` (the venue
map and publication sequence).

---

## Coverage / gaps

**AGPL-vs-Apache internal inconsistency — ✅ RESOLVED 2026-07-09 (C13 → AGPL-3.0-or-later + DCO).** The
user's licensing call was the copyleft lock over permissive spread: the neutral reference implementation is
AGPL-3.0-or-later + DCO; the earlier Apache-2.0 (A14) is superseded; both governance docs are now aligned
and the spec README / OPEN-THREADS / this index updated. The historical description of the conflict is
retained below for the record.

The two governance docs had assigned
different licenses to what each calls the reference implementation.
`foundation-cooperative-and-sustainability.md` licenses "the code" / the neutral stack (which it
defines as "the protocol, the reference implementation, and the product flavor") as
**AGPL-3.0-or-later + DCO**, in its three-decoupled-layers table and the "-or-later" reasoning.
`open-publication-and-ip-stewardship.md` licenses "the reference code" as **Apache-2.0**, chosen over
MIT/BSD for the patent grant. As written these conflict on the reference code's license. A plausible
reconciliation — AGPL-3.0-or-later for the deployed application / product (network copyleft), and
Apache-2.0 for the reference *protocol* implementation (permissive + patent grant, so others can
implement freely) — is **not stated in either doc**. This should be resolved and the two docs made
consistent before external publication. **Update 2026-07-09 (C13):** the conflict is now **surfaced in
both docs** with a `[decision-gate]` note (no longer a silent contradiction), but it is **not resolved** —
it is a genuine strategic fork (permissive-for-spread vs copyleft-lock for the neutral reference
implementation) and is the user's licensing call, tracked as ROADMAP C13. Neither license is settled for
the reference implementation until then.

```
                    foundation-cooperative-and-sustainability.md
                              │
                    "the code" / neutral stack
                    (protocol + reference impl + product)
                              │
                        AGPL-3.0-or-later + DCO
                              │
                              ▼
                        ┌───────────┐   CONFLICT   ┌───────────┐
                        │  AGPL-3.0 │  ◄────────►  │ Apache-2.0│
                        └───────────┘  on the      └───────────┘
                              ▲        reference          ▲
                              │        code               │
                    "the reference code"
                        Apache-2.0
                              │
                    open-publication-and-ip-stewardship.md
```

**Legal cases not native to this layer.** The task's expected case law — *eBay v. Newmark*, *Dodge v.
Ford*, *Hush-A-Phone*, *Carterfone* — and **Delaware DGCL Subchapter XV (§§365–367)** are **not cited
in any governance content doc**. They are cited and carried in the **philosophy layer**
(`peer-standing-and-the-cooperative-form.md`, `structural-argument-principles.md`,
`the-peer-rights-razor-and-its-lineage.md`) and inherited here by cross-reference only. They belong in
the philosophy layer's reference index; indexed above as a CROSS-LAYER pointer, not re-attributed to
governance. NOT-LEGAL-ADVICE.

**MPL-2.0 is present and mandatory — ✅ corrected 2026-07-10.** An earlier version of this index stated
MPL-2.0 was "not cited anywhere in the governance layer." That was wrong: `hpke-rs` is MPL-2.0 and is a
**mandatory, non-substitutable** constraint on the neutral stack (HPKE is required by RFC 9420 / MLS and
has no pure-permissive substitute — Decision A1), cited in `open-publication-and-ip-stewardship.md`:40 and
now indexed under Statutes & licenses above. The license families actually named across the layer are: CC0
1.0 (chosen, spec text), CC-BY 4.0 (alternative), **AGPL-3.0-or-later + DCO (chosen, reference code and
product code per C13)**, Apache-2.0 (superseded 2026-07-09, the earlier reference-code choice A14, retained
only as the rejected permissive option), MIT/BSD (the further-rejected permissive alternatives inside that
superseded Apache evaluation, no longer in the content docs after C13), and **MPL-2.0 (mandatory via
`hpke-rs`)**.

**Statute detail is deliberately withheld.** Only Missouri Chapter 351 is named, and only as reasoning;
the statute sections, tax codes, fees, and dollar figures are excluded by design and gated to counsel
(NOT-LEGAL-ADVICE). Stocksy's "Colorado limited cooperative association" is described, not cited as a
statute.

**Dialogue-sourced rows needing a refresh pass before external reliance** (flags preserved from the
source docs): the credit-union motto attribution; Liquid Feedback; all three phased fiscal-sponsor rows
(SPI/Debian, SFC, Aspiration) and CS&S; Commons-DAO framing. The Zenodo, OpenTimestamps, and IETF Trust
mechanics carry `[UNVERIFIED]`. The Repenning & Sterman figures/quotations and the CVSS-9.8 example in
`making-preventative-work-visible.md` carry `[UNVERIFIED]` pending confirmation against the paper; the
private-chat-platform financial figures in `foundation-cooperative-and-sustainability.md` are
`[UNVERIFIED]` (private company). The bulk of the existence-proofs are `[verified]` or
`[verified 2026-06-22]`.

**Design-intent rows (no external locator by nature).** Much of
`foundation-cooperative-and-sustainability.md` is marked "Verification: design intent" or "settled
values" — the two-body structure, the two-tier mark, entity phasing, the bankruptcy-remote steward, and
the non-mimicry moat. These are the layer's own design and carry no external citation; they are not
indexed here as sources, only their external supports are (the licenses, the statute, and the
prior-art register above). The one external grounding inside that doc is the overjustification /
crowding-out effect (the day-care late-pickup study), carried as "externally grounded" but without a
specific citation in-doc — a candidate to pin to a primary (Gneezy & Rustichini, 2000) before external
use.
