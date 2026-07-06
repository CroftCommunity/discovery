# Drystone spec integration — work summary

Date: 2026-06-29

This summarizes the integration of the transport/identity/encryption research into the Drystone
specification, the consistency passes performed, and the source-grounded decisions made along the way.

---

## Inputs

- `drystone-part1.md` — Part 1: Reasoning Underpinnings (the "why")

- `drystone-part2.md` — Part 2: The Certifiable Design (the "what")

- `drystone-transport-section.md` — transport/identity/encryption research (a source input, folded into
  Part 2 §6; not emitted as a separate output)

- `drystone-exposure.svg` — Fig. 1, exposure map

- `drystone-catchup-flow.svg` — Fig. 2, returning-member catch-up

## Outputs

- `drystone-part1.md` — final (no content change; see note below)

- `drystone-part2.md` — final, with the research integrated and three passes applied

- `drystone-exposure.svg` — final, reconciled to spec terminology

- `drystone-catchup-flow.svg` — final (already consistent; unchanged)

- `drystone-integration.diff` — unified diff of all four files against the uploaded originals

- `drystone-integration-summary.md` — this file

---

## What was done

### 1. Integration (first pass)

The thin three-subsection `## 6. Transport` in Part 2 was expanded into a full
`## 6. Transport, Identity Planes, and the Encryption Stack` (§6.1–§6.8), folding in the research:

- two identity planes (peer/transport vs group/MLS)

- the two-layer encryption stack (iroh QUIC/TLS hop-by-hop vs MLS PrivateMessage end-to-end)

- discovery (DNS/Pkarr, Pkarr-on-DHT, mDNS), delivery (direct, relayed, meer), the HyParView+PlumTree
  gossip overlay, the two deployment modes, and real-time media

A returning-member catch-up subsection was added to §7.4 as the home for the `(G, D)` cursor and Fig. 2.

### 2. Consistency reconciliation

- **Terminology unified on `meer`.** The research's "mailbox" (17 uses) was Part 2's "meer" (the blind
  store-and-forward node). Standardized on `meer` throughout, including the figures. Zero "mailbox" remains.

- **Placeholder cross-refs resolved.** Every `§x.x` / `§3.x` now points at a real section. All internal
  cross-references resolve.

- **Appendix B, References, and the front-matter maturity note** were updated so the open-items ledger and
  mechanism lineage reflect the new material.

### 3. Source-grounded corrections (verified against primaries this session)

- **HyParView and PlumTree are two separate 2007 papers** (HyParView = DSN 2007; PlumTree = "Epidemic
  Broadcast Trees," SRDS 2007, pp. 301–310), not one. Corrected and cited.

- **MLS's security guarantees do not depend on the transport.** Sharpened the metadata framing to match
  RFC 9750 §8: the transport is assumed to add metadata privacy, but MLS holds even against a compromised
  transport.

- **Relay-vs-meer exposure correctness.** Corrected an overclaim that said "a relay or meer that terminates
  the QUIC connection sees everything Layer A protected." Per iroh's docs, the relay does not terminate the
  peer-to-peer session; it routes encrypted packets by `EndpointId` and cannot decode them. Only a meer you
  dial directly is a TLS endpoint of your connection.

### 4. iroh 1.0 differentiation (final pass)

iroh shipped 1.0 on 2026-06-15 with a wire-and-API stability guarantee. The version split is now itself a
differentiator the spec relies on:

- **Resolved to Verified (stable 1.0 core):** public-key (`EndpointId`/Ed25519) TLS identity that cannot be
  impersonated; the post-handshake point at which the remote identity becomes known (the §6.1 seam, via the
  infallible `Connection::remote_id()`); direct-first hole-punch with stateless blind relay fallback; relay
  routing by endpoint without decoding content; `subscribe(TopicId, bootstrap_peers)` shape.

- **Rescoped (NOT covered by the 1.0 guarantee):** `iroh-gossip` is a separate pre-1.0 crate, and discovery
  is split into `iroh-mainline-address-lookup` and `iroh-mdns-address-lookup` crates. Their internals
  (gossip event surface, view sizes, PRUNE/GRAFT, DHT republish interval, mDNS interface behavior) remain
  `[confirm]` against each crate's pinned version. The earlier "is mDNS mature / turnkey" question is
  resolved by the crate's existence as a maintained component.

- **`NodeId` → `EndpointId`** noted as the pre-1.0 → 1.0 rename so readers of older material aren't confused.

After this pass, §6 carries 15 Verified statements to 12 `[confirm]` flags, and every remaining flag is
legitimately open: gossip/discovery crate internals (pre-1.0), the Pkarr record-signing spec (a non-iroh
primary), RFC 8446 traffic-analysis limits, and RFC 9420 §16.9 DS-compromise specifics.

---

## Verification ledger (what was pulled from primaries this session)

Verified:

- RFC 9420 §2 (PrivateMessage/member/epoch terminology), §3 (trusted AS / untrusted DS trust model),
  §16.3/§16.4/§16.4.1/§16.4.3 (metadata exposure and the membership-inference mitigation), §6.3.2
  (SenderData AEAD-encrypted)

- RFC 9750 §8 (transport/MLS division of labor; the secure-transport recommendation; guarantees do not
  depend on transport)

- HyParView (DSN 2007) and PlumTree ("Epidemic Broadcast Trees," SRDS 2007) attribution and two-paper split

- iroh 1.0 (June 2026): Ed25519 `EndpointId` as TLS identity, post-handshake authentication
  (`Connection::remote_id()` infallible), direct-first/blind-relay-fallback, relay content-blindness,
  separately-versioned gossip and address-lookup crates, `subscribe(TopicId, bootstrap_peers)`

Still `[confirm]` (correctly, not pulled or genuinely version-pinned):

- iroh-gossip internals against its pinned pre-1.0 version

- address-lookup crate republish/expiry behavior; the Pkarr record-signing spec

- RFC 8446 padding mechanism / arXiv:2406.15686 traffic-analysis corroboration

- RFC 9420 §16.9 DS-compromise specifics beyond the §3 summary

---

## Consistency / clarity / correctness checks (final state)

- Every internal §N.M cross-reference resolves to a real heading.

- Zero em-dashes in either part (matches the no-em-dash preference).

- En-dashes only in legitimate ranges (K1–K8, T1–T7, R1–R6, page 301–310, 90–95%).

- All bullet lists carry blank-line separation between items.

- Code fences balanced; both SVGs well-formed and rendered for visual check.

- No stray "mailbox"; Appendix D term lattice already defined `meer` and needed no change.

## Note on Part 1

Part 1 needed no content change. Its only transport mention is a high-level pointer to Part 2 §10
("the substrate requirement-vs-realization treatment (MLS, iroh, and the primitives)"), which is
version-agnostic by design. The diff shows it as unchanged.

## Net change

- Part 2: +569 / −53 lines

- Part 1: no change

- Exposure SVG: +12 / −12 lines (terminology + vendor-neutral language)

- Catch-up SVG: unchanged

---

## Recommended next step before publication

Pin the `iroh-gossip`, `iroh-mainline-address-lookup`, and `iroh-mdns-address-lookup` crate versions in a
build manifest and clear the remaining §6 `[confirm]` flags against those exact versions, plus pull the
Pkarr record-signing spec, RFC 8446 §5.4 (record padding), and RFC 9420 §16.9 from their primaries.
