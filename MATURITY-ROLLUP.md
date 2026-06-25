# Maturity rollup — the repeatable method for maturing one stage into the next

date: 2026-06-24

purpose: how we mature a stage into the next one in the lifecycle `alpha → beta → rc → publish` (see
`AGENTS.md` → "Maturity stages"). This is **cross-stage process** — it describes *how we work*, not the
project — so it lives at the `discovery/` root alongside `AGENTS.md` and `PLAYBOOK.md`. It is written to
be reused at **beta → rc** and **rc → publish**, and to accumulate learnings we can reflect on at each
transition rather than re-deriving the process.

> Relationship to the other process docs: `PLAYBOOK.md` is **intake** (new material → the current
> stage). This doc is **promotion** (a matured stage → the next stage). `AGENTS.md` is orientation.

---

## 1. Principles (the invariants of any rollup)

1. **The next stage is cohesive within itself.** A doc in the higher stage references only *other docs
   in that same stage* for its forward narrative. A reader walks the stage end-to-end without dropping
   down a level.

2. **The prior stage is frozen.** Existing content in the lower stage is never moved, edited, or
   deleted (PLAYBOOK §4). The *only* additive file permitted in the prior stage is the **rollup ledger**
   (below). Raw stays frozen forever.

3. **The audit trail lives at the prior level, not the higher one.** The detailed source-by-source
   trace — what was lifted, how it was treated, where it landed — lives in the prior stage's rollup
   ledger so the higher stage stays a clean forward narrative. The higher-stage docs carry only a
   concise sources footer + a one-line pointer to the ledger. **Do not pollute the higher tier with
   provenance machinery.**

4. **"Do not carry forward" means absent, not annotated.** Material excluded from the higher stage
   simply does not appear there — that is what not carrying it forward *means*. *What* was excluded and
   *why* is recorded once, in the prior-level ledger (treatment `excluded`), never as a list inside a
   higher-stage doc.

5. **Verification travels with every carried claim.** A claim that is carried up keeps its status inline
   (`green-real` / `green-model` / `spec` / `[UNVERIFIED]` / `NOT-LEGAL-ADVICE` / …). **Direct quotes
   are preserved whole** — block quote + citation + per-quote verification flag — and tied to the
   conclusion(s) they ground. Quotes and references are first-class; never paraphrase a verified quote
   away. For settled-fact domains (atproto/iroh/iOS) cite the source-of-truth FACTCHECK, do not
   re-verify.

6. **Surface the user's decisions; do not resolve them.** Decision-gated themes carry a banner; the
   gates are listed, not decided.

## 2. The per-doc template (every higher-stage theme doc)

1. Title + status + verification line.
2. **Theme narrative (overview)** — prose summary of the whole doc and the arc it walks, before sections.
3. **Charter** — explicit *in scope* / *out of scope (and where it lives)* / *boundary calls* vs. adjacent docs.
4. **Body** — the resolved synthesis; verbatim quotes preserved whole and tied to conclusions.
5. **What this establishes (and does not).**
6. **Provenance trace** — one-line pointer to this doc's section in the prior-level rollup ledger.
7. **Sources (prior stage)** — concise footer.

No "do not carry forward" list and no detailed rollup table inside a higher-stage doc — both live in the
ledger.

## 3. The rollup ledger (the trace, at the prior level)

Lives in the prior stage as `<NEXTSTAGE>-ROLLUP.md` (e.g. `alpha/BETA-ROLLUP.md`; later
`beta/RC-ROLLUP.md`). It is a process artifact peer to `COHESION.md` / `RAW-ARTIFACTS-MANIFEST.md`, not
corpus content. Per higher-stage doc: a table mapping each prior-stage source → **treatment** → the
section it **landed in**. Treatment codes:

- **synthesized** — lifted and recomposed.
- **collapsed** — a contradiction/seam (often a `COHESION.md` entry) resolved into one statement.
- **harvested** — a conclusion buried in the prior stage, surfaced into the synthesis layer.
- **carried-flag** — verification status carried forward unchanged.
- **excluded** — deliberately not carried; reason named.
- **deferred** — real material not yet pulled up; tracked so it is not silently dropped.

The ledger also keeps a **coverage view**: prior-stage sources not yet dispositioned by any drafted
higher-stage doc. Closing that list to zero is how we confirm nothing slipped through.

## 3b. The open-threads ledger (the staging queue, at the higher level)

The rollup ledger's `deferred` code says "not yet pulled up." When deferred material is **actively
queued for the higher stage but blocked on specific settling work** — and would *pollute the settled
narrative* if asserted now — it gets a richer home: an open-threads ledger living in the **higher**
stage as `<STAGE>/OPEN-THREADS.md` (e.g. `beta/OPEN-THREADS.md`; later `rc/OPEN-THREADS.md`). The
settling happens at that stage's gate, so the queue sits there; the prior stage **references** it
(`ROADMAP_TODO.md` / `COHESION.md` / the rollup ledger point to it) so the need is never lost.

It is a process artifact, not a theme doc, and may point down into the prior stage freely. Per thread:
**status** (`surfaced` / `gated` / `ready-to-promote`), **what it is**, **promotion target** (which
higher-stage doc/section), **gates** (the explicit decisions / `ENABLING` spec work / fact-confirmation
that must clear), and **prior-stage provenance**. **Promotion rule:** a thread enters a theme doc — and
earns a rollup-ledger row — **only when its gates clear**; until then theme docs may not assert it. This
is what lets DRAFT / decision-gated / fact-unconfirmed material be tracked toward the next stage without
contaminating the resolved synthesis.

## 4. The steps (one transition)

1. **Re-validate the blueprint** against the *current* frozen prior stage (it may post-date the plan).
   Deliver a short "what changed" delta + the validated theme list.
2. **Propose the higher-stage structure** (a README/map + one doc per theme, reading order, scope,
   sources). Surface the open structural calls. **Stop for approval before drafting.**
3. **Draft theme-by-theme, least-gated first.** Apply the per-doc template; preserve quotes whole;
   carry verification status.
4. **Maintain the rollup ledger** at the prior level as you draft — fill each theme's trace, update the
   coverage view.
5. **Surface decisions, don't resolve them**; banner the gated themes.
6. **Do not commit/push** until the user approves (PLAYBOOK §3b).

## 5. Learnings log (append at each transition; reflect before the next)

### alpha → beta (2026-06-24, in progress)

- The 2026-06-22 narrative-architecture plan **predated several intakes**; re-validation (step 1) was
  load-bearing, not a formality — N4 had doubled, a vertical was already drafted, a new node
  (membership-vs-access) and a media sub-thread had appeared. *Lesson: always re-comb the current frozen
  stage; never draft straight from a plan older than the corpus.*
- First instinct was to put the rollup trace and a "do not carry forward" list **inside** the beta docs.
  The user corrected both: the trace belongs at the **prior level**, and not-carrying-forward means the
  material is simply **absent** from beta (recorded as `excluded` in the ledger, not annotated in beta).
  *Lesson: keep the higher tier a clean forward narrative; all provenance/exclusion bookkeeping lives
  down a level.*
- **Verbatim quotes + references matter a great deal to the user.** Paraphrasing a verified quote is a
  regression. *Lesson: preserve quotes whole, with citation + verification flag, tied to the conclusion
  they ground.*
- The per-doc template (theme narrative + charter + quote-anchored body) emerged from review of 01 and
  was then propagated. *Lesson: nail the template on the first one or two docs, get a read, then
  propagate.*

### beta → rc (future)

- (reflect on the above first; record what the rc transition changes about the method.)

## 6. Reflection checkpoints

At the start of each transition, re-read §1 and §5, then ask: did the prior transition's learnings hold?
What about this stage is different (rc adds real-validation/hardening pressure; publish adds
external-audience and legal/clearance pressure) and what does that change about scope, the charter
boundaries, or the verification bar? Record the answer in §5 before drafting.
