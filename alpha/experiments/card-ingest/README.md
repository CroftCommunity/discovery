# Experiment: content-blind ingest + scheduled read-grant (the card/guestbook net-new pieces)

A hermetic Rust spike for ROADMAP_TODO **E43** (virtual cards / guestbooks). It proves the two
mechanics the design note flagged as net-new
(`../../thinking/app/ponds/virtual-cards-and-guestbooks.md`); everything else in that design reuses
already-proven substrate.

**Product consumer (2026-07-21):** the link-key tier is being productized as `CroftCommunity/greetings_site`
(greetings.croft.ing) — the 1:1 server-blind card slice, reimplemented in the browser with WebCrypto
AES-256-GCM (a second faithful implementation of the same encrypt-client-side / store-ciphertext /
key-in-the-fragment shape; the Rust `card-seal` proof here is the model's evidence). Plan:
`../../plans/2026-07-21-greetings-croft-ing-mvp.md`. The anon-multi-write ingest arm this spike proves
is NOT yet productized — greetings ships the 1:1 slice first.

```
cargo test                                   # 17 tests green (incl. 5 capability probes)
cargo tree -p card-service --edges normal    # the content-blind boundary, as a dependency fact
ATP_IDENTIFIER=<h> ATP_PASSWORD=<app-pw> python3 live/pds-writetarget-probe.py  # the live PDS leg
```

Capability reasoning is filed in `CAPABILITIES.md` (probes CAP-1..5 + the live leg).

## The two mechanics

1. **Content-blind ingest.** A service accepts an already-encrypted contribution from anyone holding
   the card's bearer link, content-addresses it, and appends it via a `WriteTarget` port. The service
   has no key, no `open`, and no dependency on the decrypt crate, so "the service cannot read what it
   stores" is a **compile-time fact**, not a convention. This is the RUN-14 EXP-B content-blind
   pattern applied to the write side.

2. **Scheduled read-grant (the reveal).** The recipient is withheld the (still-encrypted) records
   until a logical reveal time AND the viewer is the recipient. Both failure conditions return the
   same opaque `Withheld`, so the gate leaks neither the existence of records nor which condition
   failed (the "one flat refusal" property). Delivery (the gate) and reading (the key) are independent
   layers: the reveal offers ciphertext, and only the fragment-key holder can read it.

## Result: validated

- **17 tests green** (`card-seal` 3, `card-sign` 2, `card-service` unit 5, end-to-end 2, capability
  probes 5), clippy `pedantic`-clean, no `unwrap`/`expect` in library paths (only in tests).
- **The content-blind boundary is proven by `cargo tree`.** The service's runtime dependency graph is
  `blake3` only (content-addressing, a hash, not a cipher):

  ```
  card-service v0.1.0
  └── blake3 v1.8.5
      ├── arrayref ├── arrayvec ├── cfg-if └── constant_time_eq
  ```

  No `chacha20poly1305`, no `card-seal`, no AEAD anywhere on the normal edges. The decrypt capability
  (`card-seal` -> `chacha20poly1305`) appears only on the dev/test edges, where the test plays the
  client. So the not-reading is enforced by the dependency graph, exactly as the design intends.
- **End-to-end**: two signers seal contributions client-side under one key and append them with no
  login (bearer link only); the service holds only ciphertext (asserted: no plaintext substring
  survives in the store); the recipient is withheld before the reveal and for a wrong viewer
  (identical opaque error); at the reveal the recipient is offered the ciphertext and opens it with
  the key; a party without the key cannot read the offered ciphertext.

## Grades (honest)

- **Content-blind ingest + scheduled read-grant**: `green`, hermetic, in-memory `WriteTarget`.
  Content-blindness is a compile boundary (dependency-graph proof), which is stronger than a runtime
  check.
- **Crypto path** (seal/open; the stored envelope carries `nonce || ciphertext`, never the key):
  ChaCha20-Poly1305 directly; the same encrypt-then-content-address shape the `encrypted-blob-share`
  spike validated on real `iroh-blobs`.

## Stand-ins and what is NOT built (blocked or out of scope)

- **The live atproto write adapter (storage leg now validated).** `WriteTarget` is a port with an
  in-memory fake for the hermetic tests; `live/pds-writetarget-probe.py` additionally validated the
  storage leg against a **real bsky PDS** (create/read/delete of an encrypted contribution round-trips
  as opaque bytes; the PDS holds only ciphertext; custom NSID needs no pre-registration). What that
  live leg does NOT cover, and stays modeled: the **OAuth + DPoP per-collection scoped-delegation**
  path (the live run used the legacy app-password Bearer flow acting AS the account, not a mediating
  service holding a delegated `repo:<NSID>` scope), and the anonymous-contributor mediation. The
  atproto OAuth facts that shape the delegation are verified in the design note (DPoP-bound tokens,
  short-lived, single-use refresh, per-collection scopes, sessions bounded by revocation not time).
- **The per-card OAuth session lifecycle** (create -> delegate scope -> ingest -> auto-revoke on
  delivery) is design, not code here.
- **Bearer issuance/verification** is a simple equality check standing in for a real unguessable
  capability token; **abuse/rate limiting** is not modeled.
- **Single symmetric key**: read + produce-content, per-card blast radius. No per-signer authorship
  and no revocation (the design note's honest edges); acceptable for a card, wrong for a chat.

## The generalization (why this is bigger than cards)

The shape here is **delegated per-collection write + content-blind ingest + optional scheduled
grant**. Nothing about it is card-specific. The same shim serves any "many contributors -> one owner's
repo, mediated, optionally server-blind" collaborative surface: shared lists, collaborative notes,
polls, RSVP rosters, group playlists, a public suggestion box. The card is simply the first instance;
the ingest is a reusable collaborative primitive.

## References

- Backlog: ROADMAP_TODO **E43**; design note:
  `../../thinking/app/ponds/virtual-cards-and-guestbooks.md`.
- Proven pieces reused/mirrored: `../encrypted-blob-share/` (encrypt -> content-address -> decrypt,
  ref carries hash+nonce not key); RUN-14 EXP-B (content-blind offer against a verified caller).
- atproto write-path facts (verified against the primary at edit time): `https://atproto.com/specs/oauth`,
  `https://atproto.com/guides/permission-sets`.
