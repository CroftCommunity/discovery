# Capability register: the collaborative-ingest model (E43)

`Companion to the card-ingest spike. Each row is an executable capability probe (a test), so the
reasoning about what this model can/cannot do, and what it trusts, is evidence rather than prose.
Probes CAP-1..5 are hermetic (card-service/tests/capabilities.rs, 5 tests); the live leg is a real
PDS run (live/pds-writetarget-probe.py). Grades are honest: hermetic characterization vs live-verified.`

## Summary

| Probe | Question | Finding | Grade |
|---|---|---|---|
| CAP-1 | What does a content-blind writer still learn? | count + per-entry length + byte-identical duplicate; NOT content. Nonce discipline controls the plaintext-dup leak. | hermetic |
| CAP-2 | What does the bearer link authorize? | append only; content-address binds ref to content (no clobber, idempotent); no edit; delete is a separate scope to withhold. | hermetic |
| CAP-3 | What does the reveal trust? | the clock source. Authority `now` gates; a viewer-supplied `now` bypasses. Monotonic. | hermetic |
| CAP-4 | Authorship: who can forge? | a signature over ciphertext binds authorship, verifiable without decrypting; the no-login path has none (impersonatable). Opt-in, orthogonal to confidentiality. | hermetic (real ed25519) |
| CAP-5 | Revocation / rotation? | all-or-nothing: rotating the key kills the old link for everyone; no per-holder revocation. Escalates to the MLS-sealed tier. | hermetic |
| LIVE | Does the storage leg work on a real PDS? | create/read/delete of an encrypted contribution round-trips as opaque bytes; the PDS holds only ciphertext; custom NSID needs no pre-registration. | live-verified (real bsky PDS) |

## The probes, in detail

### CAP-1 - metadata residue of a content-blind writer

A writer that cannot read still observes structure. From what it stores it can derive the **number of
distinct contributions**, each **ciphertext's byte length** (hence a message-length band), and
**byte-identical duplicates** (identical ciphertext yields an identical content address). It cannot
derive content. The plaintext-duplicate leak is controlled by nonce discipline: the same message under
a **reused** nonce produces identical ciphertext (collides, leaks "these two are the same"); under a
**unique** nonce it does not. Mitigations: unique nonce per contribution (correct client behavior);
pad to length bands to blunt the length signal. This is the honest metadata boundary, the analogue of
RFC 9420 section 16.4 for MLS.

### CAP-2 - the bearer link authorizes append only

Holding the link is an **append** capability, not edit or delete. Content addressing binds the record
ref to its exact bytes, so a link holder cannot clobber another entry (different bytes always get a
different ref) and a re-append of identical bytes is idempotent (same ref, no growth). Existing entries
are therefore immutable by ref. The `Ingest` surface exposes no edit or delete at all; over a real PDS,
collection **delete** is a distinct capability the shim must withhold (the open `[confirm]` on whether
the delegated scope can be create-only).

### CAP-3 - the reveal trusts the clock source

The gate is `now >= reveal_at`, a pure function of the `now` it is handed. Whoever supplies `now`
controls the reveal. The probe makes the trust dependency explicit: a **viewer-supplied** `now` would
let the recipient claim it is already reveal time and open early, so `now` **must** come from a trusted
authority (the service clock), never a viewer claim. The gate is monotonic (once open for a given
`reveal_at`, it stays open); un-revealing requires issuing a new reveal, which is the organizer's
capability. This names the reveal's one trust assumption: an authority time source.

### CAP-4 - authorship is opt-in and content-blind-verifiable

A `card-sign` ed25519 signature over the **ciphertext** binds "the holder of key K produced this exact
opaque contribution." Because the signed message is the ciphertext, verification needs no decryption
key, so a reader or even the content-blind service can check authorship without reading the message
(content-blind-safe). Forgery fails: another key's signature does not verify, and a tampered ciphertext
breaks a genuine signature. The **no-login / bearer path has no such binding**, so the service accepts
any link-authorized append regardless of claimed author, and impersonation is possible by construction.
Authorship is therefore an opt-in capability (a signature; real identity additionally costs a login),
orthogonal to the fragment-key confidentiality. A "signed-only" card can reject unsigned entries while
staying content-blind.

### CAP-5 - revocation is all-or-nothing in the symmetric-key tier

Rotating the key makes everything sealed under the old key unreadable under the new one at once, and
there is no operation that revokes one holder while keeping others: all K holders are equivalent, so
selective revocation requires moving to a new key (which no old-link holder can derive) and
redistributing the link. Selective, forward-secret revocation is exactly what escalates the design to
the **MLS-sealed tier** (croft-group L2a can remove one member). This confirms the honest capability
limit of the link-key tier.

## The live leg: WriteTarget on a real PDS

`live/pds-writetarget-probe.py` ran against a real bsky PDS (test account
`ngvalidation2112.bsky.social`, PDS `stropharia.us-west.host.bsky.network`) over real network:

- **create** an encrypted-contribution record in `ing.croft.cardtest.entry`: HTTP 200, real AT-URI + cid.
- **read back**: the ciphertext round-trips byte-identically; the stored record is `$type`,
  `ciphertext`, `createdAt` only, with no key and no plaintext (content-blind by construction). The
  custom NSID propagated with no pre-registration.
- **delete**: HTTP 200, and read-after-delete is HTTP 400 (gone, cleaned up).
- the session is revoked at the end; no credentials are stored in the repo.

**What this validates:** the storage/transport leg of the content-blind ingest is real, at real-PDS
grade. **What it does NOT validate (honest boundary):** (a) the ChaCha20-Poly1305 crypto, proven
separately in the hermetic spike with a compile-time content-blind boundary (the live envelope is
deliberately opaque bytes to avoid faking crypto); (b) the **OAuth + DPoP per-collection
scoped-delegation** path, this run uses the legacy app-password/createSession Bearer flow acting AS the
account, not a mediating service holding a delegated `repo:<NSID>` scope; (c) the anonymous-contributor
mediation. Those remain modeled / spec-verified (the design note's atproto facts are verified against
the primary spec, but the delegated-scope write and the mediation are not exercised live).

## Reasoning payoff

Read together, the probes bound the model's trust surface precisely: it is **confidential from the
server** (fragment key, content-blind writer proven by dependency graph and by the real PDS holding
only ciphertext), **append-integral** (content addressing), and **honest about what it leaks**
(count/length/dup metadata) and **what it cannot do** (no authorship or revocation without opting up a
tier). The two escalations are named and mechanical: **sign** for authorship (proven, ed25519), and
**MLS-seal** for forward-secret selective revocation (proven elsewhere, croft-group L2a). The live leg
shows the storage substrate is real; the delegated-scope and mediation legs are the remaining
modeled-to-real gap.

## References

- Design note: `../../thinking/app/ponds/virtual-cards-and-guestbooks.md`; backlog: ROADMAP_TODO E43.
- Crates: `card-service` (content-blind ingest + reveal), `card-seal` (confidentiality), `card-sign`
  (authorship); live adapter: `live/pds-writetarget-probe.py`.
- atproto facts verified at edit time: `https://atproto.com/specs/oauth`,
  `https://atproto.com/guides/permission-sets`.
