# Experiment: Encrypted file/photo share in a private group over real `iroh-blobs`

A **self-contained** Rust experiment that shares a photo inside a private group
end-to-end, with the binary **encrypted before it enters a content-addressed
blob store** and transferred peer-to-peer over **real `iroh-blobs` QUIC**. It
depends on no other experiment; it builds the minimal MLS + AEAD + Automerge
substrate it needs from scratch.

```
cargo run
```

The single most important outcome — **validated**: a real photo, encrypted under
an MLS-exporter epoch key, stored and transferred byte-for-byte over real
`iroh-blobs` QUIC **as ciphertext**, referenced from an Automerge document by its
**ciphertext** BLAKE3 hash, is fetched by a second group member and decrypted
back to a plaintext whose hash matches the original exactly — with the blob store
and transport never seeing plaintext.

---

## Summary (for reasoning about this in relation to other work)

A one-screen view. Detailed report follows below.

### Goal
Prove the **large-binary media path** of the private-group architecture works on
**real networking** (not a stubbed transport): can a file/photo be shared inside a
private group end-to-end — encrypted, content-addressed, referenced from a CRDT
document, fetched by another member, and decrypted — with the store and wire never
seeing plaintext? This complements the small-state CRDT path that other slices
exercise; here the headline is **real `iroh-blobs` over QUIC** plus the
**encrypt-then-content-address** property.

### Approach
Build a **standalone** crate that reconstructs the minimal real substrate from
scratch (no dependency on sibling experiments): MLS group + per-epoch exporter key
(`openmls`), AEAD (`chacha20poly1305`), a CRDT reference doc (`automerge`), and the
real blob transport (`iroh` + `iroh-blobs`). Flow: derive a per-epoch content key
via the MLS exporter → encrypt the photo → put **ciphertext** in an `iroh-blobs`
store (hash is over ciphertext) → record an atproto-`blob`-style reference (hash +
nonce, **no key**) in the Automerge doc → a second member fetches the ciphertext
over a real QUIC connection and decrypts with the key it already holds via MLS.
Two genuine `iroh` endpoints connect directly (no relay/DNS). The only deliberate
shortcut: the small reference doc is handed to the second member directly rather
than run through full CRDT sync — the **entire blob path is real**.

### Effort
~700 lines across 6 modules + a real 778 KB PNG asset and a lexicon file. Most of
the effort went into **version archaeology** against a moved ecosystem (the brief
assumed an older `iroh-blobs` line; current is `iroh 1.0.0-rc.1` + `iroh-blobs
0.102`) and adapting to the real APIs (production MLS Welcome path since
`into_welcome` is test-gated; ratchet-tree extension; the current blobs
fetch/connect surface). See the **friction log** below for specifics. Builds and
runs green on `rustc 1.94`.

### Result — validated ✅
Every step passes, including the core assertion: recovered plaintext BLAKE3 ==
original, ciphertext hash ≠ plaintext hash, transfer integrity confirmed by
BLAKE3-verified streaming over real QUIC, and MLS epoch rotation (add a 3rd member)
rotates the content key with all members agreeing. The large-binary media path is
**proven, not assumed**.

### What this validates / does NOT validate
- **Validates:** encrypt-before-store; content-addressing over ciphertext; nonce
  riding in the CRDT reference while the key never leaves MLS; real P2P blob
  transfer with integrity; per-epoch key rotation on membership change; that the
  current `iroh`/`iroh-blobs`/`openmls`/`automerge` versions interoperate.
- **Does NOT validate:** full CRDT document sync (shortcut taken); persistence
  across restarts (`MemStore` only); NAT traversal / relay / discovery (endpoints
  dial directly on one host); multi-device; old-epoch key retention (rule is
  described, not implemented); production-grade error handling and key management.

### Key tradeoff to carry into other work
Encrypting before content-addressing means the BLAKE3 hash is over **ciphertext**,
so **cross-user dedup is lost** — two members encrypting the same photo under
different nonces/keys produce different content hashes. This is inherent to doing
encryption correctly; whether it matters depends on the use case. See the
encrypt-then-content-address section.

---

## Architecture being validated

A private group shares a rotating symmetric **group key** managed by **MLS**.
Small mutable state lives in an **Automerge** CRDT document. Large immutable
binaries live in a **content-addressed blob store** (`iroh-blobs`) and are
referenced from the CRDT document by hash. Binaries are **encrypted before
entering the store**, so the store and transport only ever see ciphertext. This
experiment proves the large-binary path works on real networking.

| Layer | Crate | Role here |
|-------|-------|-----------|
| Group key | `openmls` 0.8 | Ed25519 members, add→commit→Welcome join, per-epoch exporter key |
| AEAD | `chacha20poly1305` 0.10 | encrypt-before-store / decrypt-after-fetch |
| CRDT doc | `automerge` 0.10 | holds the attachment *reference* (not the bytes) |
| Blob transport | `iroh` 1.0.0-rc.1 + `iroh-blobs` 0.102 | real QUIC, BLAKE3-verified streaming |

Source layout:

```
src/aead.rs      ChaCha20-Poly1305 encrypt/decrypt
src/mls.rs       MLS group + per-epoch content key (exporter)
src/doc.rs       Automerge doc, AttachmentRef record, Willow 4-tuple address note
src/lexicon.rs   targeted validator for the attachment record
src/blobnet.rs   iroh endpoints + iroh-blobs store + real transfer
src/main.rs      the 8-step demonstration
lexicon/com.example.groupshare.attachment.json   record lexicon
assets/sample-photo.png                           real 512x512 RGB PNG (~778 KB)
```

---

## Resolved versions + version-line decision

`rustc 1.94.1`. Exact resolved crate versions (pinned with `=` in `Cargo.toml`,
locked in `Cargo.lock`):

| Crate | Resolved |
|-------|----------|
| `iroh` | **1.0.0-rc.1** |
| `iroh-blobs` | **0.102.0** |
| `openmls` | 0.8.1 |
| `openmls_rust_crypto` | 0.5.1 |
| `openmls_traits` | 0.5.0 |
| `openmls_basic_credential` | 0.5.0 |
| `automerge` | 0.10.0 |
| `chacha20poly1305` | 0.10.0 |
| `rand` | 0.8.5 |
| `blake3` | 1.8.5 |
| `tls_codec` | 0.4.2 (transitive, via openmls) |
| `curve25519-dalek` | 4.1.3 |
| `ed25519-dalek` | 2.2.0 |

### The iroh / iroh-blobs decision

The brief, written against an earlier ecosystem snapshot, framed the choice as
**0.35.x (stable)** vs **0.9x (canary)** and asked to *prefer the current 0.9x
line*. **The ecosystem has moved well past that snapshot.** As of this build the
current line is **`iroh-blobs` 0.102.0 paired with `iroh` 1.0.0-rc.1**. The 0.9x
series no longer exists as the bleeding edge; 0.102 *is* the current canary line,
and its own README still carries the same caveat the brief quoted:

> **NOTE: this version of iroh-blobs is not yet considered production quality.
> For now, if you need production quality, use iroh-blobs 0.35.**

**Decision:** follow the brief's intent — prefer the current canary line to
exercise the current API surface — which today means **0.102.0 + iroh
1.0.0-rc.1**, not 0.9x. `0.35.x` remains the documented stable fallback.

* **(a) Which line:** current canary, `iroh-blobs` 0.102.0 + `iroh` 1.0.0-rc.1.
* **(b) Compatible-pair confirmation:** the pair resolves together (446 packages,
  no conflict) and compiles and runs. `iroh-blobs` 0.102 depends on exactly
  `iroh` 1.0.0-rc.1.
* **(c) Trouble:** none that forced a fallback. The current API matches the
  shapes in the brief almost exactly (see friction log for the small deltas), so
  0.35.x was not needed.

---

## Checkpoint: trivial `iroh-blobs` add/get round-trip

Run **before** building the feature. Result: **PASS** — a 16-byte blob was added
to a `MemStore` on one endpoint and fetched over real iroh QUIC by a second
endpoint, bytes identical, hash
`24912896e8871417d3a68b3c58ba69382775bf3c5f49fb530c4073d83bc5eb24`.

---

## Per-step results (a representative run)

Hashes/nonces are per-run for the nonce; plaintext/ciphertext hashes are stable.

| Step | Result | Key numbers |
|------|--------|-------------|
| 1. Group setup + epoch key | PASS | epoch `1`; Alice and Bob both derive content-key hash `4aebe0d8…b5e7` (identical) |
| 2. Real binary | PASS | real PNG, **plaintext 778 149 bytes**, BLAKE3 `086e076c…33ca` |
| 3. Encrypt | PASS | nonce `11a0e21c9c4882720ea5aa86`, **ciphertext 778 165 bytes** (+16-byte tag) |
| 4. Store ciphertext | PASS | **ciphertext BLAKE3 `1a1bd7d2…661f`** ≠ plaintext hash (encrypt-then-address) |
| 5. Author reference record | PASS | record validated against lexicon; inserted at `/attachments/3l5xqphoto001` |
| 6. **Transfer over real iroh** | PASS | fetched 778 165 bytes; received-bytes hash == stored ciphertext hash (BLAKE3-verified streaming) |
| 7. **Bob decrypts** | **PASS (CORE)** | recovered 778 149 bytes, BLAKE3 `086e076c…33ca` == original, valid PNG |
| 8. Epoch rotation | PASS | epoch `2`, new key `1e58e259…2c5f` ≠ old; Alice/Bob/Carol agree; Carol decrypts a new-epoch blob; old-epoch ciphertext is *not* decryptable with the new key |

### How the two iroh endpoints connected

Both endpoints bind with `Endpoint::bind(presets::N0)`. The provider mounts
`BlobsProtocol` on a `Router` under `iroh_blobs::ALPN`. **Bob dials Alice's full
`EndpointAddr` directly** — node id plus direct socket address(es) — so no DNS
publish or relay round-trip is needed; the connection is a direct QUIC dial. In
this sandbox the single direct address observed was `192.0.2.2:49632` (a
TEST-NET-1 documentation address the environment assigns); the direct dial
succeeded over it on the same host. No `BlobTicket` was needed because we hand the
full address over in-process; `BlobTicket` is the equivalent for out-of-band
sharing.

### The attachment reference record (as authored)

Modeled on atproto's `blob` convention (`$type: "blob"`, a `ref`, `mimeType`,
`size`). **Divergence:** `ref` here is the BLAKE3 hash of **ciphertext**
(`ciphertext_ref`), we **add** the AEAD `nonce`, and we **never** store the key.

```json
{
  "$type": "blob",
  "ciphertext_ref": "1a1bd7d26aa31e1f32f94949de349832a08be28d2d1147225808693a8290661f",
  "nonce": "11a0e21c9c4882720ea5aa86",
  "mime_type": "image/png",
  "size": 778149,
  "filename": "sample-photo.png"
}
```

A conceptual **Willow 4-tuple address** is recorded alongside each write:
`namespace=private-group-001` (group id), `subspace=alice` (author),
`path=/attachments/3l5xqphoto001`, `timestamp=<unix>`. No full addressing engine
is built — see `src/doc.rs`.

### Epoch rotation + old-epoch access rule

Adding Carol advances the MLS epoch `1 → 2` and rotates the exporter content key
(`4aebe0d8… → 1e58e259…`). All three members (Alice, Bob who processed the
commit, and Carol who joined from the Welcome) derive the **same** new key. Carol
fetches and decrypts a blob shared under the **new** key. The experiment also
verifies concretely that Carol's new-epoch key **cannot** decrypt the old-epoch
ciphertext.

**Old-epoch rule:** a blob encrypted under epoch *N*'s key requires that retained
key to decrypt. A member who joined at epoch *N+1* cannot derive epoch *N*'s
exporter secret, so she can only open older attachments if the old content key is
explicitly retained and handed to her — the same forward-secrecy tradeoff that
governs message history. Old-key retention is not implemented (out of scope); the
new-epoch share is demonstrated and the rule is described.

---

## The encrypt-then-content-address property (the genuinely new thing)

Because encryption happens **before** the bytes enter the store, the BLAKE3
content hash is computed over **ciphertext**, not plaintext (step 4 proves
`ciphertext_hash ≠ plaintext_hash`). Consequences, made concrete:

* the store and transport only ever hold/see **ciphertext**;
* the **AEAD nonce must ride in the CRDT reference record** — the content hash
  alone is not enough to recover plaintext. We store *pure ciphertext* in the
  blob and the nonce solely in the record, so this is unambiguous;
* the **key never travels** anywhere — members already hold the epoch content key
  via MLS;
* **cross-user dedup is lost**: two members encrypting the same photo under
  different nonces (or different epoch keys) produce different ciphertext, hence
  different content hashes, so the store cannot dedupe them. This is the genuine
  tradeoff the architecture inherits from doing encryption correctly. Whether it
  matters depends on the use case (per-group media is rarely re-shared
  byte-identically across groups).

---

## Honest friction log

* **iroh version line moved past the brief.** The brief's "0.9x canary vs 0.35
  stable" framing is stale; the current canary is `iroh-blobs` 0.102 + `iroh`
  1.0.0-rc.1. Followed the brief's *intent* (prefer current canary) rather than
  its literal version numbers. No fallback to 0.35 was needed.
* **iroh-blobs API vs the brief's described shapes — matches, with small deltas.**
  `Endpoint::bind(presets::N0)`, `MemStore::new()`, `BlobsProtocol::new(&store,
  None)`, `Router … .accept(iroh_blobs::ALPN, …)`, `add_*` returning a tag with
  `.hash`, and `BlobTicket` are all present as described. Deltas observed:
  - the tag type is `TagInfo` and `add_bytes`/`add_slice` return an `AddProgress`
    you `.await` to get it (rather than a bare tag);
  - for the **fetch** side this experiment uses
    `endpoint.connect(addr, ALPN)` + `iroh_blobs::get::request::get_blob(conn,
    hash).bytes()` rather than `store.reader(hash)`. The `reader(hash)` API the
    brief mentioned does exist on the *store* (for reading a blob you already
    hold), but the requester needs a network fetch; `get_blob` is the direct,
    store-less requester path and gives BLAKE3-verified streaming for free. A
    `store.downloader(&endpoint).download(hash, providers)` path also exists but
    resolves providers by node id via discovery, which would need the address
    book populated; connecting to the full `EndpointAddr` is simpler for a local
    two-endpoint test;
  - node identity is `EndpointId` / `EndpointAddr` (renamed from the older
    `NodeId` / `NodeAddr`).
* **openmls `into_welcome()` is `#[cfg(test-utils)]`-gated** — exactly as the
  brief warned. Production path used: serialize the commit/welcome to
  `MlsMessageOut` via `tls_serialize_detached`, deserialize on the receiver as
  `MlsMessageIn::tls_deserialize_exact`, then `msg_in.extract()` and match
  `MlsMessageBodyIn::Welcome(w)`.
* **Welcome needs the ratchet tree.** The first join attempt failed with *"No
  ratchet tree available to build initial tree."* By default the Welcome does not
  carry the tree. Fixed by `MlsGroupCreateConfig::builder().use_ratchet_tree_extension(true)`
  so the tree travels inside the Welcome's GroupInfo (the alternative is
  delivering the ratchet tree out of band to `StagedWelcome::new_from_welcome`).
* **openmls companion-crate versions.** No 0.4.x/0.5.x `openmls_traits`
  duplicate-trait pain occurred: `cargo add` resolved the companion crates
  (`openmls_rust_crypto` 0.5.1, `openmls_traits` 0.5.0,
  `openmls_basic_credential` 0.5.0) on the 0.5.x line cleanly against `openmls`
  0.8.1. They are `=`-pinned to keep it that way. Likewise the dalek dependency
  set resolved to a single consistent version (`curve25519-dalek` 4.1.3,
  `ed25519-dalek` 2.2.0) shared across iroh and openmls — no duplicate-dalek
  conflict despite an early resolver preview suggesting newer pre-releases.
* **TLS codec traits not surfaced by the prelude glob.** `openmls::prelude::*`
  re-exports `tls_codec` but does not bring `Serialize`/`Deserialize` into method
  scope; imported them explicitly as
  `use openmls::prelude::tls_codec::{Deserialize as _, Serialize as _}`.

### Simplifications taken (labeled)

* **Document sync.** The Automerge document holding the attachment reference is
  handed to Bob directly (`save()` → `load()`) rather than run through a full
  CRDT sync protocol. This is the **one** deliberate simplification — Automerge
  sync is well-trodden and the blob path is the subject here. The **entire blob
  path (encrypt → store → transfer over real iroh → fetch → decrypt) is fully
  real.**
* **In-memory store.** `MemStore` is used; `iroh_blobs::store::fs::FsStore` is the
  persistent option (no persistence across restarts is in scope).
* **Targeted lexicon validator.** `src/lexicon.rs` checks exactly the
  `com.example.groupshare.attachment` record constraints rather than implementing
  a general-purpose lexicon engine.

## Scope discipline

No PDS, OAuth, atproto network, Jetstream, relay/AppView, superpeer, push,
discovery beyond connecting two local endpoints, offline queuing, multi-device,
persistence, or UI. Purely the private encrypted-blob path.
