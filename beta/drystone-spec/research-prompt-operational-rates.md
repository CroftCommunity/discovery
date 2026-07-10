# Research prompt: quantifying per-group operational rates for community messaging

`Companion artifact to Drystone Part 2 §11.14. This is a research task specification, not a result. Its job is to extract the maximum defensible signal about three per-group operational rates from a landscape where the official per-group data does not exist, while refusing to launder low-quality sources into false precision.`

`Status of what this produces: best-estimate ranges with explicit confidence and explicit source-quality tags. Nothing this task produces is a verified figure; the best possible outcome is a well-triangulated range whose basis a reader can inspect and dispute.`

---

## What to find, and why these three

Three quantities govern the cost model of a large-group messaging design, and all three are unpublished at per-group granularity on every consumer platform. Each is a per-group-per-unit-time rate or a ratio, not a platform-wide aggregate.

1. **Member-ban rate per group per day.** How often a *person* (an account/member) is removed from a single community per day, in steady state and in bursts. This is distinct from content removal and must be kept distinct throughout (see Guardrail 3). Steady-state central estimate, plausible range, and burst behavior all wanted.

2. **Member add/join rate per group per day.** How fast new members join a single community per day, for a mature (non-viral, non-launching) community in steady state, and how it spikes on external linking or virality.

3. **Live fraction as a function of total roster.** Of a community's total members, what fraction are *active or present* (however the source operationalizes it: recently-visited, recently-posted, recently-online) versus dormant, and how that fraction changes as the roster grows. The design's central premise is that this fraction is small and shrinks with size; the task is to quantify how small and how it scales.

For each, the design needs: a central estimate, a plausible range, how the quantity varies with community size, and burst/spike behavior where relevant.

## The source landscape, and the priority order

Official per-group data is largely absent, so triangulation across source tiers is mandatory. Use this strict priority, and label every figure with the tier it came from.

- **Tier 1, primary platform disclosure.** Platform transparency reports, official engineering blogs, official API documentation and hard limits, investor/regulatory filings (10-Ks, DSA reports). Highest trust. Note that these are almost always *platform-aggregate*, not per-group, so their main use is to bound per-group figures from above (a platform total divided by a plausible active-group count gives an order-of-magnitude ceiling) and to anchor the distribution shape.

- **Tier 2, peer-reviewed and academic datasets.** Papers analyzing open platform data (Reddit via historical bulk datasets, moderation-action-log studies, community-structure analyses), and pre-print archives. High trust for the quantities they directly measure. These are the best per-group source that exists, because researchers computed per-community rates the platforms never published. Prioritize finding these; they are the backbone of any defensible estimate.

- **Tier 3, ancillary and practitioner sources.** Community-management blogs, mod-tooling vendor data and dashboards, platform-community forum posts, moderator self-reports, "state of moderation" surveys, subreddit/server directories that publish activity stats. Admissible *only as corroboration* (see Guardrail 2). Their value is real: practitioners running large communities have direct operational knowledge that leaks into how-to content and tooling defaults, and mod-tool vendors sometimes publish aggregate customer data. But they are also where fabricated and recycled statistics live, so they never stand alone.

## The guardrails, non-negotiable

These exist because the specific failure mode of this research is swallowing invented precision. Several widely-circulated "statistics" pages present fabricated figures with false specificity (named reports that do not exist, oddly precise percentages with no methodology). The guardrails are how the task stays honest.

**Guardrail 1, distribution shape before point estimates.** Establish the *shape* first (heavy-tailed, most communities small, activity concentrated in a small core), because the shape is well-established across independent platforms and it tells you which summary statistic is even meaningful. A "mean group size" or "average bans per group" is nearly useless under a heavy tail; report medians, ranges, and how the quantity varies across the size distribution, not a single headline number. If a source reports only a mean, treat it as low-information.

**Guardrail 2, Tier 3 corroborates, never founds.** A figure may rest on Tier 3 only if a Tier 1 or Tier 2 source independently supports the same range. A number appearing solely in blogs or vendor pages is reported as *indicative, ancillary-only* and explicitly excluded from any figure that would feed a design SLA. When multiple Tier 3 sources agree, check whether they are independent or are recycling one origin (SEO content frequently copies a single fabricated figure across dozens of pages, which is not corroboration, it is amplification). Trace agreement to independent origins or discount it.

**Guardrail 3, member-removal is not content-removal.** The single most important distinction. Most published moderation figures are *content* removal rates (posts/comments removed), which are frequent (single-digit percent of content). *Member* bans (removing a person) are rare and are a small minority of moderation actions. Never let a content-removal figure stand in for a member-ban figure; they differ by orders of magnitude and only the latter is the quantity sought. Any source that blurs them is used only for the content figure, and its member-ban implication is discarded.

**Guardrail 4, active is not member-count.** For the live-fraction quantity, watch which denominator a source uses. Platforms increasingly report active-visitor metrics precisely because raw member count overstates the active population, so a source's "members" and "active users" can differ by an order of magnitude. Record the exact operational definition each source uses (visited-in-7-days, posted-in-30-days, online-now) and do not merge figures across incompatible definitions without saying so.

**Guardrail 5, fabrication tells.** Flag and exclude sources exhibiting: named studies or reports that cannot be located as primary sources; suspiciously precise figures with no stated methodology or sample; round-number "statistics" presented without provenance; and figures that recur verbatim across many low-quality pages with no traceable origin. When in doubt, exclude and say so, rather than include and hedge.

**Guardrail 6, cite every figure to a resolvable locator, and tag its tier and confidence.** Every number in the output carries: its source (resolvable, not "studies show"), its tier (1/2/3), and a confidence (high/medium/low/indicative-only). A figure without all three does not appear in a conclusion.

## The inference method

Because no single source gives a per-group rate directly, the estimates are triangulated, and the triangulation is shown, not hidden.

- **Bound from above with Tier 1 aggregates.** A platform-wide moderation total from a transparency report, divided by a defensible count of active communities, gives an order-of-magnitude per-group ceiling. State the division and its assumptions explicitly; it is a sanity bound, not a point estimate.

- **Anchor with Tier 2 direct measurements.** Where a paper computed a per-community rate, that is the anchor. Report its sample, method, platform, and date, because per-group rates vary by platform and community type and a single paper's number is not universal.

- **Corroborate the range with Tier 3.** Practitioner and tooling figures that fall inside the Tier-1-bounded, Tier-2-anchored range raise confidence; ones outside it are noted as tension to explain, not averaged away (Rule: show disagreement rather than paper over it with a false synthesis).

- **Report as a range with a shape, per size band.** The output for each quantity is not a number but a range that varies across the size distribution: what a small community sees, what a median-active community sees, what a large community sees, plus burst behavior. This matches how the design consumes it (costs scale on live-N across size bands, so the rate-vs-size relationship is what matters).

- **State the residual explicitly.** Where the evidence genuinely cannot pin a quantity (member-ban rate per group per day is the likeliest to stay thin), say so plainly and report the widest defensible range with low confidence, rather than manufacturing a central estimate the evidence does not support.

## Specific leads worth pursuing

Concrete starting points, to be verified at their primary source rather than trusted from memory:

- **Moderation-action-log studies** on open community data are the most promising Tier 2 source for member-ban rate, because they categorize action types (including user-bans distinct from content-removals) across many communities. Find these and extract the ban-specific figures; they are the closest thing to a real per-group member-ban rate that exists.

- **Platform transparency reports** (the major consumer platforms publish these biannually/quarterly) give Tier 1 aggregate accounts-actioned and communities-removed totals, useful as upper bounds and for the platform's own stated shift toward warnings-over-bans, which bears on ban-rate trends.

- **Community-size distribution studies** and platform investor disclosures anchor the distribution shape and the live-fraction denominator problem; one major platform's public shift from member-count to active-visitor reporting is itself strong evidence for the activity/membership divergence.

- **Purpose-built-platform hard limits** (API documentation for platforms that publish server-size caps, role caps, and "large group" thresholds) give external engineering reference points for where scaled systems draw their lines, useful for the live-fraction-vs-size question and as corroboration that ~250 is a real inflection.

- **Mod-tooling vendor data and community directories** (Tier 3) sometimes publish aggregate activity and moderation stats across their customer base; admissible as corroboration under Guardrail 2, valuable when it traces to real operational data rather than marketing.

## Output format

Produce, for each of the three quantities:

1. A one-line **best-estimate range** with its confidence and dominant source tier.

2. The **distribution-aware breakdown**: the quantity for small / median-active / large communities, and its burst behavior, each with source and tier.

3. The **triangulation shown**: the Tier 1 upper bound (with its division assumptions), the Tier 2 anchor(s) (with sample/method/date), and the Tier 3 corroboration (with independence checked).

4. An explicit **residual statement**: what the evidence cannot pin, and the widest defensible range where it stays thin.

5. A **source table**: every figure used, with resolvable locator, tier, date, and confidence, and a separate list of sources **excluded** under the fabrication guardrails, with the reason.

Close with a one-paragraph honest summary: which of the three quantities the evidence supports well, which stay inferential, and where a design consuming these figures should therefore keep the widest safety margin. The quantity most likely to remain thin is the member-ban rate per group per day; say so if the search confirms it, rather than overclaiming.
