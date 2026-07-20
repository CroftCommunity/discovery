# FINDINGS — hist-atproto-spike (RUN-HIST-01 §F-HIST-*)

Findings ledger for the HIST lane. FINDING = something learned that the
design must carry; FIX = a defect corrected during the run. Grade **Modeled**
unless stated otherwise.

## F-HIST-1 — Per-group DID announces group existence to a public, enumerable directory (RUN-HIST-01 Part A row 8; FINDING, recorded not solved)

Extension of the attest lane's F-AT-6 (PLC op-log timing, hosting, and
enumerability correlators), filed here rather than by editing that lane's
file. F-AT-6 records what the PLC log leaks about *personas*; what is new in
the HIST lane is what it leaks about *groups*:

Under HS OC-1's per-group-DID option (the communal namespace rendered
literally — one repo IS one group's dataset), creating the DID **announces
the group's existence** to the PLC directory, which is public, permanent, and
enumerable: "The full history of DID operations and updates, including
timestamps, is permanently publicly accessible"; "the set of all identifiers
is enumerable"; including "the full history of handle updates and PDS
locations (URLs) over time" (did:plc spec, fetched 2026-07-20 —
HIST-ATPROTO-MATCHUP.md §5-8). Concretely, an observer who never touches the
repo learns, at population scale and forever:

- **that the group exists** (the identifier is enumerable — no crawling
  needed), and its **creation time** (op-log timestamps);
- **every scribe supersession** (row 8's rotation events are PLC operations —
  the group's helper-replacement history is a public timeline);
- **its PDS hosting choice and hosting history** (an infrastructure
  fingerprint; groups co-homed on a small PDS cluster);
- **correlations across groups**: same-day creations, same-day rotations, and
  shared hosting joinable against F-AT-6's persona-side correlators.

This is all metadata about the *container*; content and envelope exposure are
governed separately (matchup rows 1, 11). But for a sealed-membership group
whose existence is itself sensitive, the per-group DID option leaks the one
bit no posture dial inside the repo can recover. Mitigations (named, not
solved, the F-AT-6 family): the service-DID option of HS OC-1 (groups as
collections under one service identity — existence hides in the service's
population, at the cost of the communal-namespace-primary rendering);
opaque/unlinkable handles; staggered creation and rotation; distinct PDS
hosts only where the group accepts the hosting fingerprint. The choice is
HS OC-1, pending. (evidence: HIST-ATPROTO-MATCHUP.md row 8 + §5-8 anchors,
RUN-HIST-01, Modeled-by-design)
