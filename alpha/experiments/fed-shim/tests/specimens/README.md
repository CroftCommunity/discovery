# fed-shim wire specimens — SINGLE SOURCE OF TRUTH

Each file in this directory captures a **byte-shape** the shim must
match. When the shim's tests assert equality against a specimen, they
are anchoring the shim's fidelity to a piece of observed / documented
Mastodon behavior.

**Discipline** (`FED-SHIM.md §0 rule 3, §1`):

1. Every claim in the charter's §1 fidelity table cites a specimen
   here (or a spec citation if the specimen was not obtainable
   in-env this run).
2. If the attended live leg (§4) observes a byte-level discrepancy
   against real Mastodon, the resolution is: update the specimen
   (recording the observation), update the shim to match, and name
   what changed in the run summary that observed the discrepancy.
3. Specimens are DATED at the top of each file and cite the source
   (Mastodon source-tree path, spec section, or observed instance
   response).
4. **Nothing here is heuristic.** If a specimen's byte shape isn't
   directly extractable from a captured artifact or a spec text,
   the corresponding shim behavior is not modeled — it goes into
   `FED-SHIM.md §3` (firm non-goals) instead.

Specimens filed this run:

- `mastodon-follow-observed-shape.md` — the JSON body of a Mastodon
  outgoing Follow activity.
- `mastodon-undo-follow-observed-shape.md` — nested-Follow shape.
- `mastodon-delete-actor-observed-shape.md` — Delete of an actor
  (account deletion).
- `mastodon-actor-doc-observed-shape.md` — actor-document JSON-LD.
- `mastodon-http-signature-header.md` — the Signature header shape
  Mastodon emits.
