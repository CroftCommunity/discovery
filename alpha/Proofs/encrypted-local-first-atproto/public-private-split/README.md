# Public/Private Split (Phase 4)

The architectural keystone every prior phase deferred: **selective mirroring**
from the encrypted private group into a **public, cleartext atproto space**.
Records authored in the encrypted group are projected to a public repo *only*
when explicitly marked public — never otherwise, and never in a way that leaks
the existence of a private record. What crosses is valid atproto lexicon data
under the author's public DID, and it plugs into the same AppView ingest
contract from Phases 3a/3b.

Fully offline (no live PDS/network), consistent with every prior phase.

```
cargo run     # 5-step lifecycle: private group -> mirror -> non-leakage -> validity -> ingest
cargo test
```

## The two privacy rules (in `mirror.rs`)

1. **Default-deny.** Visibility is client-side metadata (`visibility.rs`), keyed
   by the record's private AT-URI — deliberately *not* a field on the record
   (the lexicon record types are closed schemas, and a `visibility` field would
   itself be published). Any record not explicitly `Public` stays private.
2. **No dangling references to private records.** A public reaction whose
   `subject` strongRef points at a record that is *not* public is **redacted
   (dropped)** — publishing it would leak the existence and AT-URI of a private
   record. This is a real cross-boundary integrity rule, not cosmetic.

Public records are validated against their lexicon before crossing — what
becomes world-readable must be conformant atproto data.

## Lifecycle (all 5 steps PASS)

1. **Private group**: an MLS group with 4 mixed-visibility records — a public
   post, a **private** post (`"internal: Q3 budget is 50000 USD"`), a public
   reaction on the public post, and a public-tagged reaction on the *private*
   post. The group doc is encrypted at rest; the assertion confirms the secret
   does **not** appear in the ciphertext.
2. **Mirror**: `considered:4, mirrored:2, kept_private:1, redacted_refs:1` — the
   public post and the public→public reaction are published; the private post is
   withheld; the reaction referencing the private post is redacted.
3. **Non-leakage (keystone)**: the private secret text, the private post's
   AT-URI, and its rkey appear **nowhere** in the serialized public projection.
4. **Validity + identity bridge**: every public record validates against its
   lexicon, and each author DID bridges back to an MLS member identity
   (`did:plc:<hex>` derived from the member's signature key = the 4-tuple
   subspace).
5. **AppView ingest**: the public projection emits exactly 2 `RecordEvent`s (the
   same contract Phases 3a/3b consume), and none carry private data — a public
   AppView would index these and only these.

## The single most important outcome

A record authored in the encrypted private group **never crosses into the public
space unless explicitly marked public, and never leaks the existence of a
private record** — while public records emerge as valid atproto lexicon data
under the author's public DID, ready for the public read path. The public/private
core of the architecture holds end to end on the real stack.

## Resolved versions

rustc 1.94.1 · automerge 0.7.4 · openmls 0.8.1 · chacha20poly1305 0.10.1 ·
cid 0.11.3 · serde_ipld_dagcbor 0.6.4 · serde_json 1.0.150. iroh 0.98.2 /
iroh-blobs 0.102.0 resolvable, not linked.

## Deviations & friction (honest)

- **Visibility is out-of-band metadata, not a record field.** This is the right
  model (closed lexicon schemas; you must not publish the visibility tag itself),
  but it means a real client needs a durable, synced visibility store — a small
  design surface this PoC keeps in memory.
- **Redaction = drop, not transform.** The cross-boundary rule here drops a leaky
  reaction entirely. A richer policy could *rewrite* it (e.g. keep the reaction
  but null its subject), but for a privacy boundary, drop is the safe default and
  the clearest to verify. The `mirror` function is the single place to add
  field-level transforms later.
- **Non-leakage is checked by substring scan** of the serialized public repo for
  the secret / private URI / private rkey. This catches content and identifier
  leaks; it does not attempt to reason about *traffic-analysis* leaks (e.g. that
  a public record exists at all, or its timing) — out of scope for a data-model
  PoC, worth noting as a real-world concern.
- **One private namespace, single committer**, as in prior phases. Multi-group
  fan-out and per-record transforms are natural extensions the structure already
  supports.
- **The split composed cleanly with every prior layer**: the private side is the
  unchanged encrypted CRDT stack; the public side reuses the lexicon validator
  and emits the unchanged AppView `RecordEvent` contract. No layer had to bend to
  accommodate the boundary.

## Where this sits in the arc

Private encrypted CRDT core (Phase 1) → lexicon-valid records (Phase 2) →
source-agnostic AppView (Phase 3a) → real-wire Jetstream ingest (Phase 3b) →
**public/private split (Phase 4)**. The private and public read paths now both
terminate in the same AppView ingest contract; the boundary between them is a
single, auditable mirror policy.
