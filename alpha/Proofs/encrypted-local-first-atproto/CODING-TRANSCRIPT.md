# Coding transcript — encrypted local-first + atproto validation (PR #3)

source: AI coding session, pasted verbatim 2026-06-15.

context: this log captures the closing arc — restructuring the accounting as a proper
hypothesis-driven research record, then building (but not running) the live-bsky-validate
crate and persisting a RESUME.md handoff because live egress could not be enabled in-session.
The earlier phases (1–12, the 14 sub-experiments) are written up in README.md.

---

## Restructuring the accounting as hypothesis-driven

> I'll restructure it as a proper research accounting: the central hypothesis, decomposed
> testable claims, per-experiment hypothesis→method→result→finding, confirmed conclusions,
> revised/refuted assumptions (the surprises), and what the experiments deliberately do not
> prove.

`experiments/README.md` became a full accounting:

- **Central hypothesis** — the one architectural bet: a private, encrypted, local-first
  group (MLS + CRDT) can interoperate with public atproto such that the public/private split
  is a **policy + transport boundary, not a data-model fork** — on the real stack.

- **H1–H9 → verdicts** — each claim mapped to its phase(s): Confirmed / Confirmed-with-caveat
  / Refuted→re-scoped.

- Method & discipline (real stack, prove-novel-stub-known, offline-by-construction,
  falsifiable framing); architecture under test (MLS keys only; exporter-secret-keyed AEAD
  over CRDT content); per-phase Hypothesis · Experiment · Result · Finding; confirmed
  conclusions; assumptions revised/refuted; open design decisions; threats to validity.

The honest core of the accounting:

- **One hypothesis refuted and re-scoped:** "removing a member revokes access" → forward
  secrecy only; redaction needs re-encryption, bounded to the stored copy.

- **One required an addition the early briefs waved off:** concurrent membership forks — a
  sequencer is **load-bearing, not optional**.

- **Two revised:** rkeys must be pinned for stable identity; a public reference to a private
  record is itself a leak.

- Everything else confirmed, with live-network and transport limits stated plainly as
  fidelity boundaries, not silent gaps.

## The live validation that couldn't run in-session

> **I have two bluesky test accounts, could we validate some of this now while we are here?
> I am using it elsewhere but should only overlap at the publish relay.**

The relay overlap is expected (atproto repos are public). Re-probed egress: `bsky.social`,
`plc.directory`, and the Jetstream host all return **Host not in allowlist**, and no cred env
vars are set. **The gate is the sandbox's egress policy, not credentials** — configured on the
user's side, not changeable from here.

Built `live-bsky-validate` (compiles, ready to run the moment egress + creds are in place):

- createSession for both accounts (app-password auth)

- createRecord of a custom-lexicon post on the real PDS

- **CID parity** — PDS-assigned CID vs. locally-computed CIDv1 (a mismatch is itself a finding
  about DAG-CBOR canonicalization)

- strongRef reaction from the second account; getRecord readback + lexicon re-validation

- best-effort firehose appearance (filtered to our collections + DIDs); deleteRecord cleanup

Two honest caveats baked in: publishing is public (cleanup can't un-emit firehose history;
clearly-experimental NSID + "test record" label used); identity binding (Phase 8/11) is **not**
live-validatable on bsky-hosted accounts — it needs the DID's verification key, which an app
password doesn't grant (would need a self-hosted PDS or did:web).

> **I am not going to be able to flip that switch in this session so I need you to persist
> this to a file so we can pick it back up elsewhere.**

Persisted `live-bsky-validate/RESUME.md` (linked from the README): where we left off + the
verified egress blocker evidence + branch/PR pointers; prerequisites (hosts to allowlist:
bsky.social, plc.directory, the Jetstream host; the app-password env vars); how to run;
per-step pass criteria; what it deliberately does not validate (identity binding needs
self-hosted PDS / did:web); likely live findings to watch (custom-NSID acceptance, CID parity
from DAG-CBOR canonicalization, Jetstream timing, handle→PDS routing); and what to capture
back to flip H2/H3/H5 from "validated locally" to "validated against the live network."

Nothing left in flight; no secrets involved.

---

## Resume pointer

To run the deferred live validation: allowlist the three hosts, set
`ATP_IDENTIFIER`/`ATP_APP_PASSWORD` (+ `_2`) as env vars (Bluesky app passwords, never
committed), `cargo run -p live-bsky-validate`, and fold the results back into the README
accounting. Full instructions in `live-bsky-validate/RESUME.md`.
