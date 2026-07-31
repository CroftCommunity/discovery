# Prompt: process the Amble naming + cooperative-layer transcript

Copy this into a fresh session (after a clear). It is a **thin pointer** by design — the filing
rules live in `discovery/AGENTS.md` and `discovery/PLAYBOOK.md` (the source of truth). Follow them;
this prompt only carries the task-specific state a fresh session can't otherwise know.

---

You're in the CroftC workspace (`/Users/cpettet/git/chasemp/CroftC`). I have **one long transcript** to
process and discuss — it covers (a) a **naming move for the forum pad (Graze → "Amble by Croft")** and
(b) **effective ideas for the cooperative ("coop") layer**. I'll paste it next. **Don't start filing
or discussing until I've given it to you.**

## Orient first (source of truth — read before acting)

- `discovery/AGENTS.md` and `discovery/PLAYBOOK.md` — filing discipline: classify → preserve raw
  verbatim (a pasted chat is usually **cleaned-paste, content-faithful, NOT byte-pristine** — say so,
  PLAYBOOK §4) → distill → update the connective tissue. **Don't commit/push unless I ask.**
- The **behavior-scale** methodology (the reusable method these pads are built with — renamed from
  "behavioral twin" on 2026-07-31): `discovery/alpha/thinking/behavior-scale/README.md` (index +
  registry + build pipeline + the mock→small→medium→large continuum), plus
  `behavior-scale-methodology.md` and `persona-switch-prototype.md` beside it.

## State you need to know (this is why the transcript matters)

- The **forum pad was built as the "Graze" behavior-scale mock** — `graze.ing`, repo
  `CroftCommunity/graze`, spec `discovery/alpha/thinking/app/build-specs/graze-persona-switch-spec.md`,
  research `discovery/alpha/research/reddit-aggregation-ux-slashdot-baseline-2026-07.md`. Naming home:
  `NAMING.md` → "Forum-layer naming". `graze.ing` was acquired (ROADMAP_TODO E73) and it superseded the
  earlier `forum.croft.ing` domain.
- **DECISION to fold in (user, 2026-07-31): "Graze / Graze by Croft" was premature and is being
  superseded by "Amble by Croft."** The field is dense and there is already an existing **"Amble"** in
  this space → treat clearance as **CONTESTED-until-cleared**, exactly like Stellin. Mirror the Stellin
  discipline: `research/stellin-name-clearance-2026-07.md` (the clearance report shape) and the NAMING
  gate — **do not propagate "Amble" into durable structure until the clearance clears; surface the
  gate, don't resolve it** (PLAYBOOK §5). Also surface: what happens to the just-acquired **`graze.ing`**
  domain (retire / redirect / hold) and whether an `amble.*` domain is wanted.
- The forum pad has an **open architecture fork already tracked** (`COHESION.md` §63, ROADMAP_TODO E80):
  the Graze/Amble build spec assumes a **Next.js + Postgres** large-tier backend, while the corpus's
  active forum plan (**E62**, `plans/2026-07-27-read-first-forum-mvp.md`) chose a **read-first lens over
  the public Bluesky AppView**. Keep that fork in view; the coop-layer ideas may bear on it.
- The **cooperative layer already has homes** — fold the incoming coop ideas into these, don't start a
  parallel doc: `thinking/cooperative-social-union-model.md`, `thinking/foundation-and-ip-stewardship.md`,
  `thinking/governance-and-survivability.md`, `SOVEREIGN-COMMONS-DOSSIER.md`, and the **D5 sustainability
  ↔ cooperative-mechanism** item (the existential open gate; legal-review is the user's call — see
  `beta/DECISIONS.md` and ROADMAP_TODO). Surface, don't resolve, the legal-review gate.
- Git state: the behavior-scale filing is committed (`discovery` HEAD `c01ef04`, pushed) on branch
  `claude/games-catalog-filing`. Confirm the branch before any commit; ask me where a new commit should
  land.

## Task, once I paste the transcript

1. **File it** per PLAYBOOK §2b: raw → `discovery/alpha/seeds/transcripts/raw/` (cleaned-paste, §4
   header); distill the two threads to their right homes (naming → `NAMING.md`; coop ideas → the
   cooperative homes above).
2. **Naming:** update `NAMING.md` "Forum-layer naming" to Graze → **Amble by Croft** (working name,
   not-final, clearance gate); reconcile the `graze.ing` domain; add/refresh `COHESION.md` +
   `ROADMAP_TODO.md` (E73/E80 and a name-clearance gate row like Stellin's A18). Rename the
   forum-pad build-spec / registry references only **after** you and I agree the name holds — until
   then, note "Amble (pending clearance)" without churning the built `graze` repo.
3. **Clearance:** set up (or run) an "Amble" name-clearance pass in the Stellin mold — dense field,
   existing "Amble", "Amble by Croft" lockup — and file it to `research/`.
4. **Coop layer:** fold the effective ideas into the cooperative homes; flag what's strong, what needs
   a decision, and how it ties to D5 (sustainability) and the legal-review gate. Then **discuss** it
   with me.
5. Update connective tissue (COHESION / ROADMAP_TODO / ECOSYSTEM if new orgs/projects are named /
   RAW-ARTIFACTS-MANIFEST / NAMING). Don't commit or push unless I ask.

Start by confirming you've oriented, then wait for the transcript.
