# DAG-CBOR and content addressing (reference primer)

`Status: reference primer, companion to Part 2 §4 (data model) and the forward-only history DAG (§7.7, §11.1). Not a normative change to the spec. DAG-CBOR / IPLD facts are Established primitives; Drystone pinning its own canonical byte layout is a [gates-release] item (Part 2 Appendix B), not yet frozen.`

---

## Overview

Drystone's data model is content-addressed: the signed message is the unit of history, and identical content must hash to the same identifier no matter who serializes it or when. This primer explains DAG-CBOR, the encoding the IPLD ecosystem uses to get that property, and the plain Merkle-tree framing behind it. The purpose is to make the discipline legible, so a reader understands why Part 2 §4.6 requires canonical bytes on the wire and why the exact encoding is held as a release-gating item rather than assumed.

This is a reference primer, not a specification of Drystone's wire format. Drystone is CBOR-DAG-shaped: it inherits the same determinism-for-content-addressing discipline, but its exact canonical byte layout is an open, release-gating item (Part 2 Appendix B), not a frozen artifact.

## What DAG-CBOR is

DAG-CBOR is a strict, deterministic binary serialization in the IPLD family, used by IPFS, Filecoin, Ceramic, and, relevant here, the atproto repository model (Established). It takes CBOR, a compact binary alternative to JSON, and constrains it so the exact byte layout is fixed. That fixed layout is what content addressing requires: the identifier of a block is a hash of its bytes, so the bytes must be reproducible.

Three properties make it content-addressing-ready.

- **Strict determinism.** Standard CBOR and JSON can serialize the same data several ways (map keys in any order, for one), producing different bytes and therefore different hashes, which breaks content addressing. DAG-CBOR enforces canonical rules (map keys sorted by length, then by bytes), so a given dataset always serializes to the same byte string and therefore the same Content Identifier (CID). Identical data yields identical bytes yields the same CID.

- **Native links via CBOR Tag 42.** IPLD's defining feature is that one block can point to another block by CID. DAG-CBOR reserves CBOR Tag 42 for exactly this: on encountering Tag 42, the parser knows the following bytes are a logical link (a CID) to another block, not an arbitrary string. This is what stitches isolated blocks into a traversable graph, a DAG.

- **Stripped features, for predictability and safety.** DAG-CBOR bans constructs that standard CBOR allows: no `undefined` (only `null`); no floating-point edge cases (NaN, Infinity, and negative Infinity are forbidden); no indefinite-length arrays or maps (the parser must know a collection's length up front, which removes a class of streaming ambiguity). Fewer degrees of freedom means fewer ways for two encoders to disagree on the bytes.

The contrast that isolates the value: JSON is text, its keys can shift, and it has no native links; standard CBOR is binary but its determinism is optional and it too has no native links; DAG-CBOR is binary, its determinism is enforced, and it carries native CIDs via Tag 42.

## The Merkle-tree framing, plainly

A Merkle tree is a pyramid where the leaves hold data and each parent holds the combined hashes of its children, up to a single root hash (a CID). Change one leaf and its hash changes, which changes its parent's hash, and so on up to the root; knowing the root lets you verify the whole tree is untampered.

DAG-CBOR is the concrete building block used to write the parent nodes. A parent (say, a listing that names its children by hash) is serialized in DAG-CBOR, and Tag 42 is what tells the reader that a given value is not text but a cryptographic link (a CID) to a child node further down the tree. Because DAG-CBOR is deterministic (sorted keys, no flexible formatting), whoever builds that parent node produces the exact same bytes and therefore the exact same hash.

In summary: the Merkle tree is the abstract network of data pointing to data via hashes; DAG-CBOR is the format that renders that network into fixed, reproducible binary, so links are explicitly recognized as links (Tag 42) and the structure cannot be subtly altered by formatting quirks. The same shape underlies a blockchain block or a Git commit.

## Relevance to Drystone

Drystone's content-addressed history DAG (the forward-only mode, Part 2 §7.7 and §11.1) rests on the same discipline. The spec's rule that state is canonical on the wire and free in local storage (§4.6) is the determinism-for-content-addressing rule stated in Drystone's own terms: a message must serialize to one fixed byte string so that every participant computes the same identifier for it and the forward-only history converges rather than fragmenting on encoding differences.

The place this discipline is pinned is Appendix B: the byte-level wire encodings carry the `[gates-release]` flag, meaning they must be frozen before a publication-final release and before two independent implementations can interoperate. Drystone does not adopt DAG-CBOR's bytes wholesale; it pins its own canonical byte layout there. Until that item is closed, Drystone is CBOR-DAG-shaped in intent (Established discipline) but its exact encoding is not yet frozen (`[gates-release]`).

## What this establishes (and does not)

- It establishes what DAG-CBOR is, and why strict determinism plus native Tag 42 links plus stripped features make an encoding content-addressing-ready. These are Established primitives inherited from the IPLD ecosystem.

- It establishes the plain Merkle-tree framing that content addressing rests on, and connects it to Drystone's canonical-on-the-wire rule (§4.6) and its forward-only history DAG (§7.7, §11.1).

- It does not change the spec. Nothing here is normative.

- It does not freeze Drystone's wire encoding. That Drystone pins its own canonical byte layout is a `[gates-release]` item (Part 2 Appendix B), still open. Drystone is CBOR-DAG-shaped; the exact bytes are not yet frozen.
