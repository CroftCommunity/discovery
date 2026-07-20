# ap-ambassador — the ActivityPub ambassador receipt lane

An **aligned offering** in the Croft product lane (NOT drystone-spec surface).
The ambassador ingests inbound ActivityPub activities, verifies them at the
HTTP-signature layer, and mints **evidence-complete receipt records** — gateway-
attested facts that state exactly what was received, from whom, verified how.

**Governing principle** (canonical statement in `AP-AMBASSADOR.md` §0): the
ambassador respects the customs of the protocol federated with. It is a
delivery-plane role in the A.7 sense: no ordering authority, no membership
authority, and — pinned this run — **no governance conductivity**. Where the
two cultures' semantics differ, the remedy is clear provenance.

**Status:** experiment-grade, red-first, RUN-AP-01 (2026-07-20). No live
fediverse leg (see the gated live leg in the run brief); every activity in the
test suite is a fixture.

- Charter + verdicts: `AP-AMBASSADOR.md`
- Findings ledger: `FINDINGS-AP.md`
- Lexicon drafts: `lexicons/` (empty this run — AP-OC-7 deferred until the
  outbound-delivery run)
