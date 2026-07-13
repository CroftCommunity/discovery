# Experiment: AT Protocol Moderation Labels

A self-contained experiment, run live on real infrastructure, validating how
moderation **labels** work in AT Protocol — how they're authored, who issues
them, how they're distributed, and whether they're cryptographically
attributable. It builds directly on the signature/CID machinery from the main
experiment (`src/repo_verify.rs`, `src/cidv1.rs`).

Run it: `cargo run -- moderation` (uses the test account from `.env`).

## Goal

Answer, concretely and on live infra:

1. **Authoring** — can a record carry author-applied (**self**) labels, and do
   they survive a round-trip intact?
2. **Issuers** — how is a third-party **labeler** discovered, and what identity /
   keys does it expose?
3. **Distribution** — how are labels obtained (pull vs. firehose), and what does
   a label object actually look like?
4. **Trust** — are labels **cryptographically signed** and verifiable against the
   labeler's key (the moderation analogue of the repo chain-of-custody)? Where is
   the trust boundary?

Subject matter is sensitive, so a guardrail: **no specific flagged user is
exposed** — label subjects are aggregated by value and any sample is redacted.

## Method / attempts

Implemented as `cmd_moderation` (`src/main.rs`) + `src/moderation.rs`, in four
parts (M1–M4) against the Bluesky moderation labeler
(`did:plc:ar7c4by46qjdydhdevvrndac`).

- **M1 — self-labeling:** publish a record with a
  `com.atproto.label.defs#selfLabels` value (`!no-unauthenticated`, a benign,
  self-protective label), fetch it back, confirm the label survived.
- **M2 — labeler discovery:** resolve the labeler's DID document; extract its
  `#atproto_labeler` service endpoint and `#atproto_label` signing key.
- **M3 — queryLabels (pull):** query for labels on our own clean account (expect
  none), then a broad sample to obtain real signed labels.
- **M4 — signature verification:** reconstruct each label's signed bytes
  (canonical DAG-CBOR of the label minus `sig`) and verify against the labeler's
  key, reusing `repo_verify::verify_signature`.
- **M4b — subscribeLabels:** probe whether a public label *firehose* exists.

### Attempts that failed first (and how we diagnosed them)

The signature step did **not** work on the first two tries, and the path to the
answer is itself a result:

1. **First run: 1/10 verified.** Only an account-level `!takedown` label (no
   `cid`) verified; nine older `porn` labels (which carry a `cid`) failed.
2. **Hypothesis: labeler key rotation.** Plausible — historical labels would be
   signed by a prior key. We pulled the labeler's **PLC audit log** (reusing the
   V6c capability) to gather *all* historical `#atproto_label` keys and verified
   each label against every candidate key.
3. **Hypothesis disproved.** A second sample (account-level `!takedown` labels +
   one cid-bearing `nudity` label) verified **9/10** — every cid-*less* label
   passed, the one cid-*bearing* label failed, even against historical keys. The
   failure correlated perfectly with the **presence of `cid`**, not with age.
4. **Root cause found.** In the repo MST, a CID is a CBOR **link** (tag 42); we
   had encoded the label's `cid` the same way. But the label lexicon
   (`com.atproto.label.defs#label`) defines `cid` as a **string**. Encoding it as
   a plain string (not a link) made cid-bearing labels verify.
5. **Fix → 10/10**, repeatably, including cid-bearing labels.

## Results (live)

```
M1 self-labeling:    self-label '!no-unauthenticated' survived round-trip: YES ✓
M2 labeler:          handle at://moderation.bsky.app
                     #atproto_labeler → https://mod.bsky.app
                     #atproto_label   → zQ3shmV1BNcX17coaDbfen6zArEad6SCLT3jVWCbC6Y9iinTa (secp256k1)
M3 queryLabels:      labels on our clean account: 0 (as expected)
                     sample of 10, value distribution: {"!takedown": 9, "nudity": 1}
M4 signatures:       verified 10/10 label signatures (all by the current key)
                     sample: val='!takedown' cts=2025-02-15 → signature VALID
M4b subscribeLabels: GET .../subscribeLabels → HTTP 404 (no public firehose)
```

- **M1 ✓** Self-labels round-trip intact and live inside the signed record, so
  they inherit the integrity guarantees proven in the main experiment (V3 /
  capstone): an author's self-labels are as tamper-evident as the post itself.
- **M2 ✓** A labeler is a first-class atproto identity: a DID with a declared
  `#atproto_labeler` service and a dedicated `#atproto_label` **secp256k1** key,
  distinct from its repo-signing `#atproto` key.
- **M3 ✓** Our clean test account has zero labels (expected). Labels are obtained
  by **pull** (`queryLabels`), keyed by URI/DID patterns. The label object:
  `{ver, src (labeler DID), uri (subject), cid?, val, neg?, cts, sig}`.
- **M4 ✓** All sampled labels' signatures verify against the labeler's
  `#atproto_label` key — labels are **cryptographically attributable** to the
  issuing labeler, the moderation analogue of the repo commit signature.
- **M4b — finding:** the Bluesky labeler exposes only the **pull** interface;
  `subscribeLabels` (a label firehose) returns **404**. There is no public
  push-stream of its labels.

## Conclusions

1. **Moderation is decoupled and additive.** Labels are *not* part of the labeled
   record or its repo. They are separate, signed assertions made by independent
   labeler identities about a subject (a DID or an AT-URI/CID). Anyone can run a
   labeler; consumers choose which to trust and subscribe to.
2. **Two distinct trust roots, two keys.** A labeler signs labels with a
   dedicated `#atproto_label` key, separate from the `#atproto` repo key. Both
   chain to the same DID, so the same identity machinery (V2, V6c) anchors both
   "what this account published" and "what this labeler asserted."
3. **Labels are verifiable, but verification has sharp edges.** They're signed
   and independently checkable — but the canonical bytes must be reconstructed
   *exactly*: `cid` is a **string** in a label (vs. a CID link in the repo MST),
   and (per the broader spec) verification must use the key valid at the label's
   `cts`, since labelers rotate keys. Naive verification silently fails on a
   subset; we hit and fixed exactly that.
4. **Distribution is labeler-dependent.** Pull (`queryLabels`) is the reliable
   public interface; a public firehose (`subscribeLabels`) is not guaranteed
   (Bluesky's returns 404). An AppView integrating moderation must call labelers
   and merge labels at hydration time, not assume a stream.
5. **Self-labels vs. third-party labels are fundamentally different trust-wise.**
   Self-labels are author claims inside the signed record (trust = the author);
   labeler labels are external claims about it (trust = the labeler). A client
   must treat them as separate signals.

### Implications for an architecture built on atproto

- **Validation boundary holds (cf. V1):** as with custom lexicons, the network
  won't apply *your* policy. Moderation is a set of opt-in, signed opinions you
  choose to consume and enforce in your own AppView/client.
- **You own labeler selection and label merging.** Decide which labeler DIDs to
  trust, fetch their labels (pull), verify signatures against the correct
  (time-valid) key, and apply your own display/visibility rules.
- **Self-labels are cheap and trustworthy-by-construction** for author-driven
  content warnings; they need no labeler and ride the record's own signature.

## Summary

The public moderation-label path works end to end and is **cryptographically
sound but semantically opt-in**. Authors can self-label (M1); labelers are
discoverable signed identities with a dedicated key (M2); labels are pulled, not
necessarily streamed (M3, M4b); and every sampled label's signature verifies
against the labeler's key (M4) — once the canonical bytes are reconstructed
correctly. The headline lesson mirrors the rest of this experiment: **atproto
gives you the cryptographic tools to trust a label's origin, but the policy —
which labelers to trust and what to do about a label — is entirely yours to own.**
The one real gotcha we hit and documented: a label's `cid` is a string, not a
CBOR link, and labeler keys rotate — verify against the right key over the right
bytes, or it fails silently.
