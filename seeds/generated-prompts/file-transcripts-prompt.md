# Prompt: File new transcripts into the CroftC discovery library

Copy this into a fresh session to file one or more transcripts. It is a **thin
pointer** by design — the actual filing rules live in `discovery/AGENTS.md` and
`discovery/PLAYBOOK.md`, which are the source of truth. Follow them; do not
re-derive them here (so this prompt stays correct as those docs evolve).

---

You're in the CroftC workspace (`/Users/cpettet/git/chasemp/CroftC`). I have
transcripts to file — I'll paste them or give you paths next. **Don't start
filing until I've given them to you.**

ORIENT FIRST (read before filing — these are the source of truth):

- `discovery/AGENTS.md` → "Reference indexes, filing & the backlog" — where
  everything goes and which standing indexes to reach for.
- `discovery/PLAYBOOK.md` — the canonical filing process (classify → preserve
  raw verbatim → distill → update connective tissue), the preservation statuses
  (verbatim / cleaned-paste / condensed / distilled), the header convention, and
  the commit rules (§3b).

TASK — for each transcript I give you, follow the PLAYBOOK to:

1. Preserve the raw under `discovery/seeds/transcripts/raw/` with the correct
   header and preservation status (a pasted dialogue is usually **cleaned-paste,
   content-faithful — NOT byte-pristine**; say so in the header). Keep secrets
   out; mark volatile facts `[UNVERIFIED]`.
2. Update the standing indexes the content actually warrants — **add to the
   existing ones, never start a parallel list**:
   - `seeds/transcripts/RAW-ARTIFACTS-MANIFEST.md` — always (provenance status).
   - `ROADMAP_TODO.md` (new open items), `ECOSYSTEM.md` (orgs/projects/tools
     named), `COHESION.md` (seams) — where earned.
3. atproto / iroh / iOS-P2P claims: align to the FACTCHECK source of truth
   (`seeds/transcripts/raw/atproto-atmospheric-web-iroh-mobile-FACTCHECK.md`) —
   **cite it, do NOT independently re-verify** (e.g. iroh is `1.0.0`). Flag
   genuinely new claims rather than inventing verification.

RULES:

- Git identity: chasemp (`chase@owasp.org`, `github-personal`); the `discovery`
  repo.
- Show me the filing plan first — where each transcript lands, which indexes
  you'll touch, and any new index rows — and **wait for my commit approval**
  (PLAYBOOK §3b). Don't resolve my open decisions; surface them.
