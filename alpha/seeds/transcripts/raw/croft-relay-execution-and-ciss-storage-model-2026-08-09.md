# croft-relay execution + the CISS storage model — working sessions, 2026-08-09 → 2026-08-11

**Provenance:** preserved-condensed (§4 of the manifest's vocabulary). This is the working dialogue
that *executed* the tiered-admission plan (Phases 1–4, 6) and produced the CISS storage-model
taxonomy (ADR 0005). The session ran across context compactions, so the early dialogue is condensed
from working state rather than verbatim; **owner statements quoted below are verbatim** (with
transcription artifacts kept, marked [sic] where confusing). Ground truth for every technical claim
is the commit record: croft-stack `relay-source-move` (merged, PR #5) + `relay-bin` (PR #6), CISS
PR #35 + `main` through `70106df`, and this repo's plan
(`alpha/plans/2026-08-07-1-plan-croft-relay-tiered-admission.md`), whose Review Log carries the
per-event detail. Direct continuation of
`croft-relay-tiered-admission-fork-vs-embed-2026-08-07.md`.

---

## Part 1 — The nine-question walk-through (2026-08-09)

All plan open questions closed with the owner, one at a time. Rulings:

1. **`insert_relay` reconnect** — probe first, either answer workable: "that's fine, lets just find
   out either way, either could be reasonable answers depends on the context."
2. **`IdIndex`** — kept; key custody reframed as a dial: "we could choose to do a remote restore on
   startup type thing or another way to protect a salt or pepper for that and just treat it like a
   password since it's sucha strong identity proxy, that's a dial in itself and we should settle on
   an early simple default while preserving the shaope of more complex options when cheap but
   clear." → the L0–L3 ladder, L1 shipped, `KeySource` seam.
3. **Store engine** — "the ciss is basically a key value store where the hash is the key bc it's
   chained ... we already need to run it and have an HA story." After a clarifying exchange (the
   agent initially over-read this as the shared customer-facing instance): a **private CISS instance
   over localhost** — "the reusable paradigm that we can sort of build up wisdom around"; three
   shared-instance concerns raised and withdrawn as inapplicable, recorded in the plan so they are
   not re-derived.
4. **DID cache** — phase-gated; "I would even think 24h is reasonable, and probably worth it but it
   depends on teh shape of hte cache" → two ages (~1h refresh / 24h hard max-stale).
5. **`ciss-auth` dependency** — git pin; elevated to a standing rule: "this is generally my rule if
   the dependency is ours: git dependency pinned to a commit; otherwise I would want to bundle it
   and build in CI checks."
6. **Device selector** — one cap per callee with a device scope minted into the token; "when I'm
   giving you this thing, it's up to me where you can contact me. And honestly, isn't that just
   another endpoint ID?" — the insight that made scope need no new concept.
7. **Repo shape** — "let's go ahead and put this in a directory in the crop stack repo [sic:
   croft-stack] ... a source and a Ansible directory ... History gets mixed, but this is fairly
   straightforward, that might not ever be a problem."
8. **Metrics** — aggregates only; "be sure to document what and why here and let's do at least 30
   days for now."
9. **Release cadence** — "confirmed" (automated dependency PRs against the new gate).

A free-form coherence pass then reconciled contradictions the amendment history had left (Phase 6's
pre-CISS store language; the grant record's "nothing else" vs its device scope; the Phase-7
cache silently breaking Phase-8's revocation claim; the unwired usage-record process boundary).

## Part 2 — Plain-English review before execution (2026-08-10)

The full design retold with user stories; three owner corrections folded in as plan §1.2:
"carried **when needed**" (membership doesn't route through the relay); the storage/visibility table
stated precisely ("nobody can see who you've invited" is true; "that you invite people" is not);
and — "it feels like we should name croft-admit croft-relay-admit for clarity" — the crate rename,
riding the Phase-1 move. Owner confirmed the two-artifact framing ("this is us building
croft-relay-admit that wraps up and doesn't need to fork the iroh relay code and ends up in
croft-stack and athoritative for now?") and said go.

## Part 3 — Phases 1–2 (2026-08-10)

**Phase 1**: gate-before-code in croft-stack (its only workflow had been a `workflow_call` deploy —
a notification, not a gate); a pre-existing red bats assertion fixed first (cert path had moved to
tmpfs); the move + rename in one commit; discovery's smoke matrix entry removed; ADR-0006 written;
ADR-0004's mechanism superseded in place; all five legacy OPEN-QUESTIONS closed. PR #5 merged green.

**Phase 2**: `croft-relay-bin` RED-first through the spawned binary (valid/absent/malformed/
wrong-key tokens; fail-loud config naming the missing field). One stated deviation: no `[tls]`
section — untested TLS is worse than absent TLS; Phase 5 owns it; `deny_unknown_fields` makes a
premature `[tls]` block a loud abort. Established the §8.3 logging convention (tracing → stderr;
stdout carries exactly one contract line).

## Part 4 — Phase 3: the airlock (2026-08-10)

The phase's verify-in-phase bullet **refuted the design before a line was written**: the relay
downcasts post-upgrade IO to exactly `TokioIo<MaybeTlsStream>` (`http_server.rs:96`), whose variants
are concrete `TcpStream` types — an in-line `CountingStream` breaks every relay connection at
runtime. Not untested: impossible. Revised to the **loopback airlock** (count the public side, pump
over a loopback hop, serve the relay a genuine `TcpStream`; bypass guard; the token join gains a
port map). Wiring test measured a 9.5× byte ratio for a 10× push; one field bug fixed in the binary
not the parser (tracing emitted ANSI to piped stderr, so `bytes_in=` was never a literal substring).
Owner: "let's make sure we note this in teh plan and documentation and continue" → §3.4, ADR-0006,
README, DESIGN.md all corrected.

## Part 5 — Phase 4: budget-and-drop (2026-08-10)

Opened with the probe the plan gated it on. **Verdict: `insert_relay` with a changed token does NOT
reconnect** — the connection persists on the old token (empirical, matches source; evidence +
probe source preserved in `relay/source/evidence/insert-relay-probe.txt`). Consequence adopted:
**disconnect-to-upgrade**. Then: `budget_for`/`Budget::exceeded_by` with boundary rows (at-the-budget
is NOT exceeded), mutation run zero survivors (the `>`→`>=` mutant died to exactly that row); the
supervisor (100ms scan, idempotent drop, `reason=budget_exhausted` one endpoint per line, spent-token
refusal running BEFORE verification so reconnect can't flap; near-miss DEBUG). The product wiring
test: dropped → refused on the spent token → admitted on a fresh one; Broker survives what kills
Coordination.

## Part 6 — The guide, and Phase 6 (2026-08-10)

Owner: "this is great for understanding the feature and functionality, let's make sure we document
all of this alongside the artifacts. we might need a guide markdown that walks out the entire
setup" → `relay/source/docs/GUIDE.md` (built-vs-pending marked throughout).

**Phase 6** recon found CISS's kind registry **closed by design** ("kinds are code, not data") — so
the store's kinds were added *as code*, vocabulary-free: `kv.flag` + `kv.counter` (CISS PR #35,
RED-first in CISS's own tier; two course corrections: kinds use dots not slashes; the owner GET
wraps `{assertion, ack}`). Owner on the pin: "tis is fine but we need to note it on both sides" →
CISS README "Downstream consumers" + the relay-side mirror. Then: `IdIndex` at L1 (initial mutation
survivors killed, not explained); the service binary; `CissStore` (one tenant `id:` keypair, Model-A
signing reused from the pin); the **both-processes restart** wiring test against the real `ciss`
binary (fails, never skips, when absent; CI builds it from the same pin Cargo.toml declares — one
source of truth).

## Part 7 — "Is this a we-aren't-actually-deleting issue?" → ADR 0005 (2026-08-11)

The Phase-6 migration gap (no enumeration endpoint; `remove()` refusing) prompted the owner's
question, verbatim: "I'm not sure I understand, is this a 'we aren't actually deleting' issue?"
Checked at source: CISS assertions are `ON CONFLICT DO UPDATE` — removal-by-overwrite is real; what
lingers is an unreadable peppered tombstone row; true row-erasure has no endpoint.

Owner's design call, verbatim: "I think we need to scope an accounting kind of chain into ciss
vocabulary then potentially so we can solve this problem correctly. we can do a lot with this
crytpo based k, v store and each kind use case is similar with a few muutually exclusive needs."

→ **ADR 0005 (Proposed)**, then a section-by-section walk-through in which the owner added two
axes: "I think there's also how things are hashed and how they're sized and that kind of thing" and
"we settled on Blake Three for the File Transfer One ... codify the hash algorithm sort of use case
broadly"; "sizing is a problem. Definitely can't assume anything's gonna be infinite." The
chain-counter checkpoint/rollup was walked through on the bank-statement analogy (compaction only
behind an **acknowledged** checkpoint — no shredding before agreement); owner: "I do understand
fine grain history behind the checkpoint is gone. So yes, that all makes sense." Classification
calls: `kv.flag` erasable + listable; `kv.counter` **removed** when the chain lands (no deprecated
2am traps).

## Part 8 — The cross-inspection (2026-08-11)

Owner: "Can you do a sweep of not only Discovery but mainly the CISS repo ... and just bear out
this kind of five axes storage model ... if this holds up a cross inspection we should probably
update that there because this is a much better framing than we've had with the sort of iterative
build by use case."

Sweep of every CISS storage surface (blobs, manifest, receipts, statements, ledger, `did_total`,
`meta`, assertions, seal). **Held; nothing resisted classification; four refinements:** retention
widened to `setting | immutable | log | chain`; **authorship** surfaced as a sixth axis
(`derived | owner-signed | provider-signed | co-signed`); `merkle-rooted` joined the hashing
postures; and the checkpoint design turned out to be a **port of shipped practice**
(`purge_receipts_settled_through` already compacts receipts behind a settled co-signed statement;
the seal tombstone tier is the erasure axis at its extreme). `CISS/docs/ARCHITECTURE.md` §5a is now
the stated storage model.

---

## Standing state at filing

Executed: plan Phases 1, 2, 3, 4, 6 (+ CISS PR #35; ADR 0005 Proposed with owner-agreed axes).
Open: ADR 0005 final acceptance → implementation; PR #6 merge; Phase 0 (owner's second network);
Phase 5 (production deploy, owner-gated); Phases 7–12; `com.atproto.repo.listRecords` still
unverified before Phase 10. The plan's Review Log is the authoritative per-decision record.
