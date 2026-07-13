# Edge Table Representation: Composite Key vs MultimapTable

This document records the reasoning behind representing directed graph edges
as composite-key `TableDefinition` entries rather than as redb `MultimapTable`
entries.

---

## Background

The derived graph has two edge tables:

- `IDX_EDGES_OUT` — keyed by `source || edge_type || target`, value is `EdgeMeta`
- `IDX_EDGES_IN`  — keyed by `target || edge_type || source`, value is `EdgeMeta`

At the time this design was settled, redb offered two relevant storage primitives:

1. **`TableDefinition<K, V>`** — a B-tree table with unique keys.  Multiple
   values for the same "logical" key must be embedded in the key itself (hence
   "composite key").
2. **`MultimapTable<K, V>`** — a B-tree table that maps each key to an ordered
   set of values, with a distinct `(key, value)` pair stored per row.

---

## What Was Considered

### Option A: Composite Key `TableDefinition` (chosen)

Key = `source_typed_id(33) || edge_type(2) || target_typed_id(33)` = 68 bytes.

Every `(source, edge_type, target)` triple is globally unique, so the key is
unique by construction.  The value is `EdgeMeta` (42 bytes).

Range scans:

- All outbound edges from node X: prefix scan over `X[0..33]`.
- All outbound edges of type T from node X: prefix scan over `X[0..33] || T[33..35]`.
- Exact existence check for a specific edge: point lookup on the full 68-byte key.

### Option B: MultimapTable

Key = `source_typed_id(33) || edge_type(2)` = 35 bytes.
Value = `target_typed_id(33) || EdgeMeta(42)` = 75 bytes.

A `MultimapTable` stores one row per `(key, value)` pair.  To check whether a
specific `(source, edge_type, target)` edge exists, the caller would need to
scan all values for a given `(source, edge_type)` key and search for the target
in the value set, or rely on the table's internal ordering over value bytes.

---

## Why Composite Key Was Chosen

### 1. Exact edge lookup is O(log n) instead of O(k)

With Option A, "does edge (source, MemberOf, group) exist?" is a single point
lookup in the B-tree.  The fold engine calls this on every `MembershipRemove`
to overwrite the edge with `present=false`.  With Option B the lookup would
require iterating the value multiset for `(source, MemberOf)` until the target
is found, which is O(k) in the number of groups a principal belongs to.

The surface layer (`list_my_groups`, `request_rejoin`) also performs exact edge
lookups, and having O(log n) performance there simplifies the read path.

### 2. In-place overwrite of EdgeMeta is natural

When a `MembershipAdd` is followed by a `MembershipRemove`, the fold sets
`present=false` on the existing edge rather than deleting and reinserting.
With Option A this is a single `table.insert(full_key, new_meta_bytes)` that
overwrites the existing row atomically.  With Option B, a `MultimapTable` would
require a delete of the old value followed by an insert of the new value —
two operations instead of one, and more complex transactional reasoning.

### 3. The edge_type prefix scan pattern is important and works cleanly

The surface layer frequently scans `source || HasAttachment` or `source || References`
to list attachments and timeline entries for a group.  With Option A, a range
from `source(33) || edge_type(2) || [0x00×33]` to `source(33) || edge_type(2) || [0xFF×33]`
captures exactly those edges.  With Option B the same query would be a prefix
scan over the key `source(33) || edge_type(2)`, which is identical — this is
neutral for this access pattern.

### 4. Value-level ordering in MultimapTable is opaque

redb orders multimap values lexicographically over raw bytes.  `target_typed_id`
bytes do not encode any semantically meaningful order, so the internal ordering
of a `MultimapTable` would provide no benefit over the composite-key approach.

### 5. API complexity

`MultimapTable` has a distinct read API (`get_all`, `iter_all`, etc.) that
differs from `ReadableTable`.  Using `TableDefinition` for both edge tables and
all other tables means the entire storage layer uses a single uniform API.  This
reduces the surface area for encoding bugs and makes the key/value contract
explicit in the `TableDefinition` type parameter rather than implicit in the
value bytes.

---

## Conditions Under Which This Decision Should Be Revisited

1. **Very high fan-out nodes.** If a group routinely has tens of thousands of
   attachments or messages, the 68-byte composite key for each edge consumes
   more space than necessary.  A `MultimapTable` with a 35-byte key and a
   shorter value might win on storage.  The threshold depends on redb's page
   layout; benchmark before switching.

2. **Append-only edge semantics become dominant.** The current design relies on
   in-place overwrite of `EdgeMeta` (for `present` flag transitions).  If the
   design shifts to immutable edge records (e.g., one record per event rather
   than one record per edge), `MultimapTable` becomes the more natural fit.

3. **redb multimap performance improvements.** If a future redb release provides
   O(log n) value-level point lookup in `MultimapTable`, the primary advantage
   of Option A (exact edge lookup) is eliminated.

4. **Schema evolution requiring the edge key to carry additional discriminants.**
   If the key needs to encode more than `source || edge_type || target` (e.g.,
   a lamport-keyed history of edge states), the composite key approach naturally
   extends by appending the new field.  At that point, evaluating whether
   `MultimapTable` would be simpler is worthwhile.
