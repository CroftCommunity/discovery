# Raw transcript: DAG-CBOR and the Merkle-tree framing (primer), 2026-07-07

`Provenance caveat (PLAYBOOK §4): content-faithful cleaned paste, not a byte-pristine export. A primer on
DAG-CBOR and how it realizes a Merkle tree, relevant to the Drystone data model (Part 2 §4: content
addressing, canonical-on-the-wire, the signed message as the unit of history). CBOR-DAG is also tracked in
the cairn field list.`

## What DAG-CBOR is

DAG-CBOR is a strict, deterministic binary serialization used in the IPLD ecosystem (IPFS, Filecoin,
Ceramic, and, relevant here, the atproto repository model). It takes CBOR (a compact binary alternative to
JSON) and constrains it so the exact byte layout is fixed, which is what content-addressing requires.

Core features:

- **Strict determinism.** Standard CBOR/JSON can serialize the same data multiple ways (e.g. map keys in
  any order), producing different bytes and therefore different hashes, which breaks content-addressing.
  DAG-CBOR enforces canonical rules (map keys sorted by length then bytes), so a given dataset always
  serializes to the same byte string and the same Content Identifier (CID).

- **Native links via Tag 42.** IPLD's defining feature is that a block can point to another block by CID.
  DAG-CBOR reserves CBOR Tag 42 for this: on Tag 42, the parser knows the following bytes are a logical
  link (a CID) to another block, not a random string. This is what stitches isolated blocks into a
  traversable graph (a DAG).

- **Stripped for predictability/security.** DAG-CBOR bans features standard CBOR allows: no `undefined`
  (only `null`); no floating-point edge cases (NaN, Infinity, -Infinity forbidden); no indefinite-length
  arrays/maps (the parser must know collection length upfront, avoiding streaming vulnerabilities).

Comparison: JSON (text, keys can shift, no native links) vs standard CBOR (binary, determinism optional,
no native links) vs DAG-CBOR (binary, determinism enforced, native CIDs via Tag 42).

## The Merkle-tree framing (plainly)

A Merkle tree is a pyramid where leaves hold data and each parent holds the combined hashes of its
children, up to a single Root Hash / CID. Change one leaf and the hashes change all the way up, altering
the root; knowing the root lets you verify the whole tree is untampered.

DAG-CBOR is the concrete building block used to write the parent nodes. A parent (e.g. a folder listing
files by hash) is serialized in DAG-CBOR, and **Tag 42** is what tells the computer that "Hash_ABC_123" is
not text but a cryptographic **link (CID) to a child node** further down the tree. Because DAG-CBOR is
deterministic (sorted keys, banned flexible formatting), whoever builds that parent node gets the exact
same bytes and therefore the exact same hash.

In summary: the Merkle tree is the abstract network of data pointing to data via hashes; DAG-CBOR is the
format that renders that network into flawless, unchangeable binary, ensuring links are explicitly
recognized as links and the structure cannot be subtly altered by formatting quirks. (Same shape underlies
a blockchain block or a Git commit.)

Design relevance for Drystone: the spec's content-addressed history DAG (forward-only mode, Part 2 §7.7,
§11.1) and its "canonical on the wire, free in local storage" rule (§4.6) are the same determinism-for-
content-addressing discipline DAG-CBOR embodies; the ENABLING wire encodings gated for release (Appendix B)
are where Drystone pins its own canonical byte layout.
