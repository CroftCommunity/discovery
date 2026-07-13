# Frozen vs Fluid Boundary

This document classifies every persistent artefact in `local_storage_projection`
as either FROZEN (a breaking change if altered) or FLUID (safe to change and
re-fold without a wire-protocol bump).

---

## FROZEN — Breaking If Changed

These items are embedded in persisted bytes that cross process boundaries or
that other nodes depend on for verification.  Changing them requires a new wire
version, a coordinated migration, and invalidation of all existing stored data.

### Envelope Shape (`AssertionEnvelope::canonical_bytes`)

The canonical byte layout is the signed surface; the BLAKE3 digest of
`canonical_bytes_with_sig` is the content address used everywhere as a key.
Any reordering, width change, or endianness flip produces a different hash for
the same logical assertion and breaks signature verification retroactively.

Fixed layout (all multi-byte integers big-endian):

| Field              | Offset (variable part fixed) | Width         |
|--------------------|------------------------------|---------------|
| version            | 0                            | 1 byte        |
| assertion_type     | 1                            | 2 bytes (BE)  |
| author_device      | 3                            | 32 bytes      |
| author_principal   | 35                           | 32 bytes      |
| group              | 67                           | 32 bytes      |
| antecedents count  | 99                           | 4 bytes (BE)  |
| antecedents[0..n]  | 103                          | n × 32 bytes  |
| lamport            | 103 + n×32                   | 8 bytes (BE)  |
| timestamp          | 111 + n×32                   | 8 bytes (BE)  |
| payload length     | 119 + n×32                   | 4 bytes (BE)  |
| payload            | 123 + n×32                   | payload_len   |

`canonical_bytes_with_sig` appends `sig_len(4 BE) || sig_bytes` after the
payload.

The `version` byte is always `0x01` for this generation.  A future generation
would use a different version byte and is an entirely separate format.

### KindTag Discriminants

`KindTag` values are encoded as a single byte in every `TypedId` and in
`NodeCard` values.  Existing discriminants must never change.

| Variant       | Value |
|---------------|-------|
| Group         | 0x01  |
| Principal     | 0x02  |
| Device        | 0x03  |
| ArtifactChat  | 0x04  |
| ArtifactNote  | 0x05  |
| ArtifactLink  | 0x06  |
| ArtifactGame  | 0x07  |

New variants may be added with a fresh byte value.  Unknown discriminants are
decoded as `None` and must be treated as forward-compatible unknowns, not errors,
in read paths.

### AssertionType Discriminants

`AssertionType` values are encoded as a big-endian `u16` in the envelope wire
format and in `AUTH_GOV_LOG` values.

| Variant           | Value  |
|-------------------|--------|
| GroupGenesis      | 0x0001 |
| MembershipAdd     | 0x0002 |
| MembershipRemove  | 0x0003 |
| RoleGrant         | 0x0004 |
| RoleRevoke        | 0x0005 |
| RuleChange        | 0x0006 |
| AttachmentAdd     | 0x0007 |
| ArtifactRef       | 0x0008 |
| Message           | 0x0009 |
| Vouch             | 0x000A |

Gaps (0x0000, 0x000B+) are reserved.  Unknown values must be treated as
forward-compatible unknowns in read paths and must be rejected by the fold
engine.

### TypedId Scheme

`TypedId` is 33 bytes: `KindTag(1) || Hash(32)`.  This is the primary key for
`IDX_NODES` and both edge tables.  The concatenation order and widths are frozen.

### BLAKE3 as Hash Primitive

`compute_hash` and `envelope_hash` are backed by `blake3::hash`.  The hash
function itself is frozen: switching to SHA-256 or any other algorithm would
change every content address for every assertion ever stored.

### Authoritative Table Key Layouts

The following table keys are used as primary lookup addresses and are therefore
frozen (see KEY_LAYOUTS.md for full byte-level detail):

- `AUTH_ASSERTIONS` — 32-byte raw Hash key
- `AUTH_ASSERTIONS_BY_DEVICE` — `DeviceId(32) || lamport_be(8)`
- `AUTH_GOV_LOG` — `GroupId(32) || gov_seq_be(8)`
- `AUTH_GENESIS` — 32-byte raw GroupId key
- `AUTH_ARTIFACTS` — 32-byte raw Hash key

---

## FLUID — Safe to Change and Re-Fold

These artefacts are entirely derived from the authoritative assertions stored
in the `auth_*` tables.  They can be dropped and recomputed by calling
`fold_derived::rebuild()`, which replays all assertions from `AUTH_ASSERTIONS`
in `merge_cmp` order.

### Entire Derived Family

| Table                | Notes                                              |
|----------------------|----------------------------------------------------|
| `idx_nodes_v1`       | NodeCard projections; rebuilt from fold            |
| `idx_edges_out_v1`   | Outbound edge index; rebuilt from fold             |
| `idx_edges_in_v1`    | Inbound edge index (mirror of `idx_edges_out`); rebuilt |
| `state_group_v1`     | Projected GroupState; rebuilt from governance log  |
| `state_checkpoints_v1` | Checkpoint records; rebuilt from fold            |
| `state_blob_presence_v1` | Ephemeral fetch-state; NOT replicated, local-only |
| `auth_gov_log_v1`    | Sequence-indexed governance log; rebuilt from assertions |

Note: `state_blob_presence_v1` is local-only ephemeral state and is explicitly
excluded from rebuild — it represents fetch intent, not assertion history.
`auth_gov_log_v1` is included in rebuild because it is derived (sequence numbers
are computed during the fold, not embedded in assertions).

### View-Models

All structs in `surface.rs` (`GroupSummaryView`, `MemberView`, `RulesView`,
`GroupListItemView`, `AttachmentCardView`, `TimelineView`, `TimelineEntry`,
`TrustSignalsView`, `TrustSignal`, `PrincipalView`, `NodeCardView`) are
application-layer projections.  Fields can be added, renamed, or removed
without any impact on stored state.

### EdgeMeta and NodeCard Wire Formats

`EdgeMeta` (42 bytes, `version || since_lamport_be || since_assertion || present`)
and `NodeCard` (variable, `version || kind || present || title_len || title ||
created_by || created_at || [blob_hash]`) are derived-table value formats.
Because these tables are always rebuilt from authoritative assertions, the
wire format of their values can be changed in a new table version without
coordination — simply bump the table name suffix and rebuild.

### Edge Table Representation

Whether `IDX_EDGES_OUT` / `IDX_EDGES_IN` are implemented as composite-key
`TableDefinition` or as `MultimapTable` is an internal implementation detail.
Callers never interact with redb directly.  See EDGE_TABLE_DECISION.md.

---

## Migration Guidance

### Wire-Breaking Change (FROZEN items)

A change to any FROZEN item (envelope field order, KindTag or AssertionType
discriminant, hash primitive, authoritative key layout) requires:

1. A new `version` byte in `AssertionEnvelope` (currently `0x01`).
2. New table name suffixes for all `auth_*` tables that store the affected bytes
   (e.g. `auth_assertions_v2`).
3. A one-time migration tool that reads the old table, re-encodes every
   assertion in the new format (re-signs with new keys if the signing scheme
   changed), and writes to the new table.
4. Removal of all peer nodes running the old format before the new format is
   deployed — or a dual-version ingest path during the transition window.

### Re-Fold-Only Change (FLUID items)

A change to any FLUID item (derived table schema, NodeCard fields, EdgeMeta
fields, view-model shapes) requires only:

1. Bump the affected table name suffix (e.g. `idx_nodes_v2`).
2. Call `fold_derived::rebuild()` on startup to repopulate from `AUTH_ASSERTIONS`.

No coordination with other nodes is needed.  Assertions are never re-sent;
only local derived state is invalidated.
