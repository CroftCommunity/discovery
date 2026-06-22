# AT Proto's private-data architecture vs. Croft's host-untrusted stance

date: 2026-06-22

status: research deliverable (analytical lens). Source dialogue:
`../seeds/transcripts/raw/croft-atproto-pds-germ-privatedata-dialogue-2026-06-22.md`; every
substantive claim web-verified in its `-FACTCHECK.md` companion (2026-06-22). The related
**projects/tools** (PDS implementations, hosts, blob backends) are registered separately in
`../ECOSYSTEM.md` §5e — this doc is the *analysis*, that is the *register*; they overlap on purpose.

purpose: position Croft's design (host-untrusted MLS group state; the blind broker / content-blind
mule) against where AT Proto's own private/non-public-data work is actually heading, so we neither
over-claim novelty nor miss a place to build on. Headline: **atproto's private-data direction
*sharpens* Croft's differentiation rather than eroding it** — the atproto core team is deliberately
choosing the trusted-PDS side of the line Croft sits on the other side of.

---

## 1. The state of play (web-verified 2026-06-22)

AT Proto is **public-by-default**: writing a record signs it and broadcasts it to the global Relay
firehose; there are no ACLs or "private account" toggles in the core repository layer. The spec's
"Non-Public Data" section says private mechanisms are *planned* and explicitly **warns against
"bolting on" encryption** to the public content-addressed tree (a public, append-broadcast log is a
bad place to hide secrets). [CONFIRMED: atproto.com/specs/atp]

The design work is real and active, in three primary-source places:

- **GitHub Discussion #3363 — "Private, non-shared data in repo?"** — proposes private
  **"Namespaces"** (latest vocabulary has moved toward **"buckets"/"realms"**): data that does **not**
  use Merkle Search Trees, does **not** broadcast to the firehose, and sits in "a boring database" on
  the PDS, gated by authentication + ACLs. [CONFIRMED]
- **GitHub Discussion #121 — "Encryption for private content"** — the cryptography track: per-post
  Data-Encryption-Keys (DEKs), DEKs wrapped to recipient DIDs' keys, eventual MLS/Matrix/Signal, and
  the **key-revocation debate** (below). [CONFIRMED]
- **A community-led ATProto Private Data Working Group** — coordinated via atproto.wiki and
  discourse.atprotocol.community (organized by Boris Mann). **Important correction:** it is *not* an
  officially-chartered Bluesky body; Paul Frazee (Bluesky CTO) participates **informally** and has
  publicly hedged. This is **distinct from** the *fictional* "AT Messaging / MLS-standardizing working
  group" that the prior atproto fact-check REFUTED — that one is still fake; this one is real and
  modest. [PARTLY — see FACTCHECK error #4]

Frazee's own framing (leaflet.pub) splits the problem in two: **personal-private** (bookmarks, drafts,
preferences — unshared) and **shared-private** (private accounts, "circles", groups). [CONFIRMED]

## 2. The three contentions — and where Croft already stands on each

The engineering debate divides on three axes. On each, Croft has *already* taken the harder position,
which is why the atproto work clarifies rather than threatens our differentiation.

| Axis | atproto core team's lean | Croft's stance |
|---|---|---|
| **Trusted PDS vs. zero-knowledge** | PDS is a **trusted agent** (like a web browser — it already holds your repo-signing keys); forcing it out of the loop is "massive client-side friction." | **Zero-knowledge / host-untrusted.** The broker is **blind**; group confidentiality (MLS epoch keys) never rests on trusting the host. Croft sits on the side the atproto team is *reluctant* to take. |
| **PDS complexity vs. cheap self-host** | Keep the PDS cheap/simple (a $4 VPS); resist ACL/crypto middleware that bloats it. Private data via a parallel namespace + scoped OAuth replication stream. | Same low-overhead value (must "survive as small self-hosted nodes"), but confidentiality lives in the **client/MLS layer**, not in PDS-side ACLs — so the broker stays simple *and* blind. |
| **Key revocation ("cat out of the bag")** | Per-post DEK (secure, heavy) vs. one rotated master key per circle. Pragmatist lean: **rotate forward only**, don't force retroactive re-encryption (a leaked/scraped copy can't be recalled anyway). | Already proven: removal = **forward secrecy only** (PR #3 re-scoped "removal revokes access" to forward-only; can't claw back what was decrypted). Croft and atproto reach the **same honest conclusion** independently. |

The takeaway: **the atproto private-data effort is converging on access-controlled, PDS-gated private
data, with true E2EE/zero-knowledge explicitly deferred.** That is precisely the seam Croft's
lineage-groups MLS proof occupies — *real E2EE group state that does not trust the host*. Native-in-
protocol E2EE on atproto still does not exist; real atproto E2EE remains third-party (Germ/MLS, the
XMTP bridge — see `germ-xchat-features.md` and `../ECOSYSTEM.md` §6). Croft's answer is **more**
differentiated now, not less.

## 3. The "Anchor Key in the public profile" pattern (Germ) — a usable idiom

Germ (the closest atproto+MLS cousin, now the first native-launched private messenger from a Bluesky
profile) binds its E2EE identity by **publishing the current "Anchor Key" in the atproto public
profile text**, and watches the profile for changes. [CONFIRMED] This is a clean, shipped instance of
*"use atproto identity for discovery, do the E2EE off-repo."* It rhymes with Croft's cross-platform
identity-provenance work (publishing a stable anchor in a public profile field —
`../thinking/cross-platform-identity-provenance.md`): the public record carries the *pointer/anchor*,
never the secret. Worth treating as prior art for how Croft surfaces a verifiable key/anchor through a
public profile without leaking content.

## 4. PDS-as-selective-file-proxy — an idea for the content-blind mule (unverified, prototype-only)

The dialogue worked out how a PDS could serve a blob the network believes is PDS-native while the
bytes live in your own object store, **zero duplication** — a reverse-proxy that intercepts
`GET /xrpc/com.atproto.sync.getBlob?cid=…`, maps the CID to your external storage path, and streams it
from there, plus a row inserted into the PDS's blob-tracking table so the protocol validates it. This
**rhymes with Croft's content-blind mule** (the routing server reduced to a content-blind relay) and
the `../../experiments/encrypted-blob-share` spike.

**Caveat, loud:** this is **unverified and self-described fragile** — it depends on the official PDS's
internal SQLite schema (`blob`/`repo_blob` tables), which is not a stability contract and can break on
any PDS update. The atproto *constraints* it rests on are real (content-addressed CIDs; a signed repo
record is required; `getBlob` is the real sync endpoint; the official PDS is SQLite-backed). Treat it
as **an idea to prototype, not a recipe.** Tracked as ROADMAP_TODO **E23**.

## 5. What we take / what we leave

- **Take (build-on):** the Rust-PDS path (`rsky-pds`/Blacksky; Cocoon for the Postgres lesson); the
  blob-backend economics (Backblaze B2 + Cloudflare free-egress; R2 zero-egress) for any hosted Croft
  blob store; the Anchor-Key-in-profile idiom; CAR export/import as the "no data hostage" backstop.
- **Leave / watch:** the PDS-side-ACL approach to privacy (we keep confidentiality in the MLS/client
  layer, broker blind); enshrining any pricing (volatile); the file-proxy SQLite-injection recipe
  (fragile — prototype first).
- **Surface, don't resolve:** the **managed-PDS-host business model** is real demand but framed as
  for-profit SaaS; Croft's stance is cooperative/non-extractive — this feeds the existential
  sustainability ↔ cooperative-*mechanism* question (`../thinking/open-considerations.md` §8;
  ROADMAP_TODO E20/E22), and the framing must not silently become the answer.

## Design conclusions (pointers)

- The differentiation framing belongs in `../crystallized/principles.md` (the blind-broker / "different
  not weaker" line) and is tracked in `../COHESION.md` §26 + ROADMAP_TODO **E22**.
- Provenance/term hygiene for anything published from this: the WG is **community-led**; Germ's IETF
  draft is **`draft-xue-distributed-mls`**; the genuine 1-Click PDS host is **DigitalOcean** (not
  Vultr); don't assert `ger.mx` or `/android-waitlist`. Source of truth for atproto facts:
  `../seeds/transcripts/raw/atproto-atmospheric-web-iroh-mobile-FACTCHECK.md` (now carries a dated
  addendum for the private-data WG + Germ).
