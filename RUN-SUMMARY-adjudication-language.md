# Run summary: human-adjudication language codification

`Docs-only pass. Branch: claude/human-adjudication-language-54bkm2 (see "Branch note" below). Date: 2026-07-14.`

> **Historical record (point-in-time docs pass, landed).** This pass's results are landed (conventions
> A.11). The current run sequence and roadmap live in `alpha/experiments/RUN-0N-SUMMARY.md`,
> `alpha/experiments/MASTER-INDEX.md`, and `alpha/experiments/EXPERIMENT-BACKLOG.md`. Retained for provenance; no deletions.

Codifies the settled human-adjudication vocabulary into a named convention
(conventions A.11) and repairs drift across the spec set and the experiment
corpus. Source of decisions: `human-adjudication-language-survey-and-proposal.md`
(2026-07-13). One commit per edit block, all prefixed `docs(adjudication):`.

## Branch note

The run brief asked for a fresh branch `docs/human-adjudication-language` off
`main`. The session's standing branch requirement designates
`claude/human-adjudication-language-54bkm2` and forbids pushing elsewhere
without explicit permission. The system-level designation wins: all work
landed on `claude/human-adjudication-language-54bkm2` (which already carried
the RUN-02 spec set this pass builds on). No other deviation from the brief.

## Per-edit status

| Edit | What landed | Status |
|---|---|---|
| **E1** | conventions-and-decisions.md: new `### A.11` (the human-adjudication vocabulary + ten description rules DR-1..DR-10) inserted after A.10; Part B changelog entry `### B.8`. | **Landed** (commit `ac71a3d`) |
| **E2** | Part 1 §3: Internet-governance corroboration entry (IANA Designated Expert / W3C powerful features / IETF rough consensus), adjacent to the Beer block; three reference-list entries; §3 back-map line updated; part-1-changelog entry. | **Landed** (commit `1a67963`) |
| **E3** | Part 1 §2.5: one forward-pointer sentence to the Designated Expert precedent; part-1-changelog entry. No back-map change (map line meaning unchanged). | **Landed** (commit `81f0ce3`) |
| **E4** | Part 2 §7.6/§7.6.1: shapes-vs-parties terminology note; running-example fix ("escalation set" → "escalation parties"); local-authority coinage on the surfacing sentence; §0 map §7.6 line updated; part-2-changelog entry. | **Landed** (commit `707b724`) |
| **E5** | conformance-suite.md cat 6: "verdict" → "disposition" (item text + vector-file comment) with the disposition-not-verdict note. | **Landed** (commit `a5cb1bc`) |
| **E6** | iroh corpus: DESIGN §7 rewritten as a superseded block; one-line superseded pointers at every other listed LWW/timestamp mention (DESIGN, threat-model, README). | **Landed** (commit `3f2bc64`) |
| **E7** | hard-stop spelling normalization in experiment prose (three files). | **Landed** (commit `8a14e13`) |
| **E8** | doc-writing-method.md §12: one-sentence pointer to the A.11 convention set. | **Landed** (commit `25778de`) |
| **E9** | SPEC-DIVERGENCE-REGISTER.md: resolved-divergence row `iroh-lww-language`; this run summary. | **Landed** (this commit) |

## Anchor drift

None. Every anchor string in the brief resolved uniquely in the current tree:
A.10 → Part B boundary (E1); §3 Beer/Cybersyn block and the reference list
(E2); §2.5 fork-not-verdict paragraph (E3); §7.6.1 heading, "they differ only
in how they are detected", the beat-E5 running example, and the "surfaces the
conflict to the affected Group" sentence (E4); conformance cat 6 and the
vector-file comment (E5); DESIGN §7 "DECIDED" plus the enumerated line refs
(E6); the known hard-stop hits (E7); doc-method §12 end (E8). Nothing was
approximated or skipped for drift.

## Fetch outcomes for the E2 primaries

All four primaries were fetched during this run and the quotes are verbatim
from them (each in-text entry carries `Verified against the primary at edit
time`). No quote slot was left `[confirm-verbatim: fetch primary]`.

| Primary | URL | Verbatim quote used |
|---|---|---|
| RFC 8126 (BCP 26) §5.2 | rfc-editor.org/rfc/rfc8126 | "IANA forwards requests for an assignment to the expert for evaluation, and the expert (after performing the evaluation) informs IANA as to whether or not to make the assignment or registration." / "The list of designated experts for a registry is listed in the registry." |
| W3C Permissions (TR) | w3.org/TR/permissions/ | "A powerful feature is a web platform feature (usually an API) for which a user gives express permission before the feature can be used." |
| W3C Geolocation (current CR) | w3.org/TR/geolocation/ | "Geolocation is a powerful feature that requires express permission from an end-user before any location data is shared with a web application." (Flagged: the 2016 REC predates the powerful-feature framing; wording differs from the CR quoted.) |
| RFC 7282 §3 | rfc-editor.org/rfc/rfc7282 | "Rough consensus is achieved when all issues are addressed, but not necessarily accommodated." |

## Judgment calls

- **E7 idiom case (left as-is):** `alpha/experiments/appview-validation/README.md:452`
  ("Invisible when designing against docs; a hard stop in practice") is the
  English idiom for a blocking `rustls` panic, **not** the reconcile hard-stop
  mechanism. Left unchanged per the brief's guidance and noted here.
- **E7 verbatim quote (left as-is):** `alpha/seeds/transcripts/design-dialogue-2026-06-13-to-14.md:157`
  ("just refuse, hard stop on conflict, escalate to the human") is inside a
  `>`-quoted dialogue transcript — a verbatim record of speech. Not normalized,
  to avoid altering a quote. (The brief's explicitly-listed META-TRANSCRIPT hit
  *was* normalized, as instructed.)
- **E6 scope:** the many LWW mentions in `roadmap.md`, `phase-0-spikes.md`,
  `transport-layers.md`, `TEST-LOG.md`, `TESTING-DESIGN.md`,
  `NEXT-SESSION-2026-06-16.md`, and `Cargo.toml` were **left untouched**: they
  characterize iroh-docs' *actual observed* flat-LWW behavior as an empirical
  finding (and the "LWW too weak for the lineage model" conclusion supports the
  invariant). Only the sites that presented LWW-by-timestamp as Drystone's
  *decided* resolution model were superseded. Added the `threat-model.md:514`
  open-question site (beyond the brief's snapshot line list) because it phrases
  the forbidden model as "our model."
- **E6 commit discipline:** the iroh (Alt.Drive) `CLAUDE.md` carries a "the user
  makes commits, not Claude" rule. That is a snapshot of the original
  standalone project's discipline; this reconciliation pass is governed by the
  operator run brief (one commit per edit block), so commits were made as the
  brief directs.
- **E9 placement:** the iroh LWW divergence is a doc-language contradiction, not
  a test stand-in, so it went into the register's **Reconciled** table (Was /
  Reconciled to / Evidence) as "superseded in place" rather than the active
  `SPEC-DELTA` table.

## Verification

- Docs-only: no `.rs`, no code, no test files touched. No code identifiers or
  test names renamed (`ForkStatus::Contradiction`/`UnderDetermined`, `HardStop`,
  `e2_4_conflict_hard_stops`, `rule_change_approval_subject` all untouched).
- Every touched Part 1 / Part 2 section carries its back-map update (where the
  map meaning changed) and a changelog entry, inside the same commit.
- Corroboration quotes verbatim from the fetched primaries.
