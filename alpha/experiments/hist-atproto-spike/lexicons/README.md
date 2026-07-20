# hist-atproto-spike draft lexicons — DRAFT, NON-NORMATIVE

Status: **DRAFT** (RUN-HIST-01 Part A §3; sketch mirrors live in
`../src/record.rs` and `../src/checkpoint.rs`). These are shapes, not shipped
schemas: nothing consumes them at runtime except the B1 mapping spike, which
mirrors them in pure code. The canonical encoding of anything here is NOT
pinned (`[gates-release]` untouched).

Design rules carried in (attest-lane precedent, `attest-family/lexicons/`):

- **Closed sets would use lexicon `enum`, never `knownValues`** (`enum` is "a
  closed set of allowed values"; `knownValues` is advisory-open — lexicon
  spec, matchup §5-6). These two sketches happen to carry no string
  vocabularies; the rule is recorded so a later field addition inherits it.
- **Core content is embedded, not hash-referenced** — the envelope fields ride
  in the record body; the rkey rendering (subspace prefix + padded counter) is
  an index convenience and the body stays authoritative.
- **No wall-clock anywhere.** The §G exclusions (Willow path, wall-clock,
  capabilities, raw subspace id) are schema absences here, deliberately.
- **Fields with NO lexicon home are listed mechanically** in
  `HIST-ATPROTO-MATCHUP.md` §3 (the T-A3.8 discipline).
- `ing.croft.hist.checkpoint` is **shape only**: §L's checkpoint construction
  stays open; the `commitment` field is a declared placeholder.

`OWNER-CALL: HS OC-5 pending` — the sealed-posture backend shape (matchup
row 11) decides whether these records appear with full envelope fields
(public-envelope posture), as opaque-rkey minimal references, or with sealed
bodies; no verdict here.
