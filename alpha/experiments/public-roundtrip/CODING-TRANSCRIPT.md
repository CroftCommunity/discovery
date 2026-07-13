# Coding transcript — public atproto round-trip, chain-of-custody + moderation (PR #4)

source: AI coding session, pasted verbatim 2026-06-15.

context: this log captures the later arc of the public-roundtrip experiment — the
cryptographic chain-of-custody capstone and the moderation-label validation. The capstone
fuses identity (V2) and content-integrity (V3) into one verifiable chain; the moderation
work (written up separately in MODERATION.md) stacks label-signature verification on the
same crypto machinery. The honest diagnostic arc in the moderation step (two failed
hypotheses before the real root cause) is preserved.

---

> **what could we validate from here now that we've come this far?**

We've got the crypto primitives in hand (DAG-CBOR, CID, sha256, EC keys from the DPoP work),
which unlocks the deepest validations — the ones that tie identity (V2) and content-integrity
(V3) into a single chain of trust.

**The capstone — cryptographic chain of custody (stacks in 3 steps):**

1. Export the repo as a CAR (`com.atproto.sync.getRepo`) and parse it — validates data
   portability and gives the signed commit + MST blocks.

2. Extract the signing key from the DID document (`verificationMethod` → the atproto
   `did:key` secp256k1) — binds identity → key.

3. Verify the signed commit (secp256k1 over the DAG-CBOR commit block) and walk the MST to a
   record's CID — proving this specific record is provably part of a repo whose signed root
   chains to a verified identity. Fuses V2 + V3 into end-to-end "who-said-what-provably."

Quicker wins: blob round-trip; TID/rkey ordering; PLC audit log; Jetstream replay window.

> **it's do it all stacked**

[Pinned crates for repo verification (CAR parse, IPLD/CID, multibase, k256 0.13 stable, not
the RC). Wrote repo_verify.rs; added the getRepo XRPC method + verify-repo command.]

**The capstone works end-to-end, live — the crown-jewel result:**

- CAR export: 1185 bytes, 6 blocks, 1 root

- 6/6 blocks content-address correctly (sha256 == CID)

- Signed commit decoded (DID matches, rev, MST root)

- secp256k1 signing key from the DID doc verified the commit signature ✓

- Our anchored record CID present in the signed MST ✓

The full chain holds: verified identity → signing key → signed commit → MST root → record
CID → record bytes — the record is provably part of this identity's signed repo.

**Four quick wins, all live, all pass:**

| Validation | Result | Significance |
|---|---|---|
| V6a Blob round-trip | upload→embed→fetch, byte-identical | media path, content-addressed |
| V6b TID/rkey ordering | monotonic; lexical sort = chronological | "order by rkey" is a free correct sort |
| V6c PLC audit log | time-ordered; latest key = current DID-doc key | identity is a signed auditable history |
| V6d Jetstream replay window | replays from a 5s-past cursor | restart guarantee is genuine replay |

Architectural takeaway: **atproto gives you cryptographic trust (identity + integrity) for
free, but zero semantic trust.** Lean on signatures/CIDs/MST for who-said-what-provably; own
all schema validation, threading, and policy in your AppView (matches V1's "the PDS won't
validate your schema").

---

## Moderation labels (→ MODERATION.md)

> **can you design an experiment for moderation labels here ... execute it and write out our
> work goal, attempts, results, and conclusions with a summary to a readme for this pr**

Design: M1 self-labeling round-trip; M2 labeler discovery; M3 queryLabels (pull); M4
subscribeLabels + signature verification.

Recon reshaped it: labeler discovery works (`#atproto_labeler` → mod.bsky.app, an
`#atproto_label` secp256k1 key distinct from the repo key); queryLabels returns signed labels;
**subscribeLabels → 404 — the Bluesky labeler is pull-only, no public label firehose** (a
finding). M4 became "verify signatures of labels obtained via queryLabels."

The honest diagnostic arc:

1. First run: only 1/10 verified (the cid-less label).

2. Hypothesized labeler key rotation; pulled the labeler's PLC key history (reusing V6c) and
   tried every historical key — still failed on cid-bearing labels.

3. The failure correlated with **presence of `cid`, not age** → real root cause: a label's
   `cid` is a lexicon **string** field, not a CBOR CID link like in the repo MST. I'd encoded
   it as a link.

4. Fixed → **10/10 verify** repeatably.

Conclusions: moderation is decoupled, additive, and cryptographically sound but semantically
opt-in. Labels are separate signed assertions by independent labeler identities about a
subject; two distinct keys (repo vs. label) chain to the same DID; verification has sharp
edges (string `cid`, key rotation by `cts`); distribution is pull-based, not a guaranteed
stream. atproto lets you trust a label's *origin*; which labelers to trust and what to *do*
about a label is yours to own.

Responsible handling: label subjects aggregated by value; samples redacted; no specific
flagged user exposed.
