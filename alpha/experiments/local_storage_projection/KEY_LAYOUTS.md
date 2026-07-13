# Composite Key Byte Layouts

Every table in `local_storage_projection` uses raw `&[u8]` keys and values.
This document specifies the exact byte layout of every composite key.
Single-field keys (32-byte raw IDs) are included for completeness.
All multi-byte integers are **big-endian** unless noted otherwise.

The encoding functions live in `src/tables.rs`; the authoritative table names
are the string literals passed to `TableDefinition::new`.

---

## AUTH_ASSERTIONS (`auth_assertions_v1`)

**Purpose:** Primary store for every `AssertionEnvelope`.  One row per unique
content hash.

### Key

| Field | Bytes  | Width | Endian | Notes                     |
|-------|--------|-------|--------|---------------------------|
| hash  | 0..32  | 32    | —      | BLAKE3-256 envelope hash  |

Total key width: **32 bytes**.

No range scans are performed on this table; the key is always a point lookup.

### Value

`u8 version=0x01 || canonical_bytes_with_sig()`

---

## AUTH_ASSERTIONS_BY_DEVICE (`auth_assertions_by_device_v1`)

**Purpose:** Index supporting per-device Lamport-order scans and monotonicity
enforcement.

### Key

| Field     | Bytes  | Width | Endian | Notes                              |
|-----------|--------|-------|--------|------------------------------------|
| device_id | 0..32  | 32    | —      | Raw DeviceId bytes                 |
| lamport   | 32..40 | 8     | BE     | u64 Lamport clock at assertion creation |

Total key width: **40 bytes**.

Encoding function: `encode_by_device_key(device, lamport) -> [u8; 40]`
Decoding function: `decode_by_device_key(bytes) -> (DeviceId, u64)`

**Field order rationale:** `DeviceId` is the range-scan prefix; `lamport` is
the sort field within that prefix.  Placing `DeviceId` first allows a prefix
scan with `range(dev||0 ..= dev||u64::MAX)` to retrieve all assertions for a
device in monotonically increasing Lamport order.  Big-endian encoding of
`lamport` is required so that redb's lexicographic byte ordering preserves
numeric ordering.

**Example:**

```
DeviceId = [0xAB; 32]
lamport  = 42 (0x000000000000002A)

Key bytes:
  [0xAB×32] [0x00 0x00 0x00 0x00 0x00 0x00 0x00 0x2A]
   ^-- 32 bytes ---^  ^---------- 8 bytes ----------^
```

---

## AUTH_GOV_LOG (`auth_gov_log_v1`)

**Purpose:** Append-only governance sequence log; one entry per governance
assertion per group, in insertion order.

### Key

| Field   | Bytes  | Width | Endian | Notes                                    |
|---------|--------|-------|--------|------------------------------------------|
| group   | 0..32  | 32    | —      | Raw GroupId bytes                        |
| gov_seq | 32..40 | 8     | BE     | u64 governance sequence number (0-based) |

Total key width: **40 bytes**.

Encoding function: `encode_gov_log_key(group, gov_seq) -> [u8; 40]`
Decoding function: `decode_gov_log_key(bytes) -> (GroupId, u64)`

**Field order rationale:** Same as `AUTH_ASSERTIONS_BY_DEVICE`: `GroupId` is
the prefix for group-scoped scans; `gov_seq` is the sort-within-group field.
Big-endian `gov_seq` preserves insertion-order sort under lexicographic
comparison.

The governance sequence number is derived during the fold (it is the count of
existing entries for the group at the moment of ingest) and is not carried in
the assertion itself.  This means `gov_seq` is a derived value, but the key
layout is nonetheless frozen because `AUTH_GOV_LOG` is an authoritative table
(its keys are used as stable references for fork detection).

---

## AUTH_GENESIS (`auth_genesis_v1`)

**Purpose:** Stores the raw genesis payload for each group; one row per group.

### Key

| Field    | Bytes | Width | Endian | Notes               |
|----------|-------|-------|--------|---------------------|
| group_id | 0..32 | 32    | —      | Raw GroupId bytes   |

Total key width: **32 bytes** (point lookup only).

---

## AUTH_ARTIFACTS (`auth_artifacts_v1`)

**Purpose:** Stores typed artifact records.

### Key

| Field | Bytes | Width | Endian | Notes                    |
|-------|-------|-------|--------|--------------------------|
| hash  | 0..32 | 32    | —      | BLAKE3-256 artifact hash |

Total key width: **32 bytes** (point lookup only).

---

## IDX_EDGES_OUT (`idx_edges_out_v1`)

**Purpose:** Outbound edge index.  A scan with prefix `source` returns all
outbound edges from that node.  A scan with prefix `source || edge_type` returns
all outbound edges of a specific type.

### Key

| Field          | Bytes  | Width | Endian | Notes                              |
|----------------|--------|-------|--------|------------------------------------|
| source KindTag | 0      | 1     | —      | KindTag byte of the source node    |
| source Hash    | 1..33  | 32    | —      | Hash portion of source TypedId     |
| edge_type      | 33..35 | 2     | BE     | EdgeType discriminant (u16)        |
| target KindTag | 35     | 1     | —      | KindTag byte of the target node    |
| target Hash    | 36..68 | 32    | —      | Hash portion of target TypedId     |

Total key width: **68 bytes**.

Encoding function: `encode_edge_out_key(source, edge_type, target) -> [u8; 68]`
Decoding function: `decode_edge_out_key(bytes) -> (TypedId, EdgeType, TypedId)`

**Field order rationale:**

1. `source` (33 bytes) is first because the dominant scan pattern is "all
   outbound edges from node X" — a 33-byte prefix covers it.
2. `edge_type` (2 bytes) follows source so that "all edges of type T from node
   X" is a 35-byte prefix, enabling efficient sub-range scans without scanning
   unrelated edge types.
3. `target` (33 bytes) is last; it is the tie-breaker and is never used as a
   range-scan prefix in this direction.

**EdgeType discriminants:**

| Variant       | Value  |
|---------------|--------|
| MemberOf      | 0x0001 |
| HasAttachment | 0x0002 |
| References    | 0x0003 |
| Vouches       | 0x0004 |

**Example (MemberOf edge, Principal → Group):**

```
source  = TypedId(Principal=0x02, Hash=[0x11×32])
edge_type = MemberOf = 0x0001
target  = TypedId(Group=0x01, Hash=[0x22×32])

Key bytes:
  [0x02] [0x11×32] [0x00 0x01] [0x01] [0x22×32]
   ^ 1 ^  ^-- 32 -^ ^- 2 B --^  ^ 1 ^  ^-- 32 -^
```

---

## IDX_EDGES_IN (`idx_edges_in_v1`)

**Purpose:** Inbound edge index (mirror of `IDX_EDGES_OUT` with source and
target swapped).  A scan with prefix `target` returns all inbound edges to that
node.

### Key

| Field          | Bytes  | Width | Endian | Notes                              |
|----------------|--------|-------|--------|------------------------------------|
| target KindTag | 0      | 1     | —      | KindTag byte of the target node    |
| target Hash    | 1..33  | 32    | —      | Hash portion of target TypedId     |
| edge_type      | 33..35 | 2     | BE     | EdgeType discriminant (u16)        |
| source KindTag | 35     | 1     | —      | KindTag byte of the source node    |
| source Hash    | 36..68 | 32    | —      | Hash portion of source TypedId     |

Total key width: **68 bytes**.

Encoding function: `encode_edge_in_key(target, edge_type, source) -> [u8; 68]`

This table stores the same `EdgeMeta` value bytes as `IDX_EDGES_OUT`.  Both
rows are written atomically by `write_edge_atomic` in a single redb write
transaction.

**Field order rationale:** The structure mirrors `IDX_EDGES_OUT` with target
and source swapped.  Scans that ask "who vouches for principal P?" or "which
groups does principal P belong to (inbound membership)?" use the target prefix.
The `edge_type` field in position 33 allows sub-range filtering by type without
scanning all inbound edges to the node.

---

## IDX_NODES (`idx_nodes_v1`)

**Purpose:** Derived node cards for all known typed entities.

### Key

| Field     | Bytes | Width | Endian | Notes           |
|-----------|-------|-------|--------|-----------------|
| TypedId   | 0..33 | 33    | —      | KindTag(1) || Hash(32) |

Total key width: **33 bytes** (point lookup only).

---

## STATE_GROUP (`state_group_v1`)

**Purpose:** Current projected group governance state.

### Key

| Field    | Bytes | Width | Endian | Notes             |
|----------|-------|-------|--------|-------------------|
| group_id | 0..32 | 32    | —      | Raw GroupId bytes |

Total key width: **32 bytes** (point lookup only).

---

## STATE_CHECKPOINTS (`state_checkpoints_v1`)

**Purpose:** Periodic fold checkpoints for fast restart.

### Key

| Field    | Bytes  | Width | Endian | Notes                        |
|----------|--------|-------|--------|------------------------------|
| group_id | 0..32  | 32    | —      | Raw GroupId bytes            |
| ckpt_seq | 32..40 | 8     | BE     | u64 checkpoint sequence (0-based) |

Total key width: **40 bytes**.

Encoding function: `encode_checkpoint_key(group, seq) -> [u8; 40]`

**Field order rationale:** `GroupId` prefix enables group-scoped checkpoint
scans; `ckpt_seq` big-endian preserves sequence order under lexicographic sort,
so the latest checkpoint for a group is always the last entry in the range scan.

---

## Notes on Endianness

All multi-byte numeric fields in composite keys are big-endian.  This is
required because redb sorts keys lexicographically (byte-by-byte), and
big-endian encoding is the only encoding that preserves numeric ordering under
lexicographic comparison.  Little-endian values in composite keys would
produce incorrect range scan boundaries and incorrect "last entry" detection.
