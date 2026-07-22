# Raw: Croft encrypted-prefs record + repo-mirror + the account.croft.ing kernel — design dialogue (2026-07-22)

**Preservation status: preserved-condensed (cleaned-paste, content-faithful — NOT byte-pristine) —
PLAYBOOK §4.** Source: a claude.ai design dialogue pasted 2026-07-22. UI render chrome stripped
(duplicated thinking-summary header "Architecting local-first sync leveraging atproto's native hash-tree
infrastructure"; "Added to memory" confirmations). No secrets. Part of the 2026-07-22 batch (plan:
`alpha/plans/2026-07-22-transcript-intake-ponds-kernel-ens.md`). Weaves into the card-maker/push thread
(`croft-card-maker-webxdc-packaging-and-push-2026-07-22.md`). **Typo note:** the user wrote
`account.croft.io` and `*.Croft.omg`; the user confirmed these are typos for `croft.ing` — normalized in
distillation, preserved-with-note here.

**FACTCHECK alignment.** This dialogue uses "Merkle Search Tree (MST)" for atproto's repo structure. The
source-of-truth FACTCHECK (`atproto-atmospheric-web-iroh-mobile-FACTCHECK.md`) confirms MST *is* atproto's
structure; the only refuted claim is calling **iroh-docs** MST-based (it uses range-based set
reconciliation). This dialogue does not do that, so the usage is correct — cite the FACTCHECK, do not
re-verify. "Sync v1.1 streams the log in causal order" is FACTCHECK-aligned (relay non-archival under Sync
v1.1). DRISL/DAG-CBOR is treated in `beta/drystone-spec/dag-cbor-and-content-addressing.md`.

---

## Opener (user)

We started making a greeting-card online thing today. So many utilities are just overcharged — a virtual
card costs ~$4.95 and has basically no overhead. Other ideas — not necessarily greetings, but online
utility and social things — we could stand up a version of that really rarely changes and is useful.

## Part 1 — encrypted prefs as a repo record (assistant)

**Encrypted prefs as a repo record: yes, and it's the right first use of the pattern.** The design: an
`ing.croft.account.prefs` record whose payload is ciphertext, encrypted in the broker with a key that
never leaves broker-origin storage. Sync across devices rides the PDS itself, which you already trust for
exactly this job, and the helper loses its prefs hat entirely, shrinking to push and session-attestation
only. Framing it openly as "our workaround for Bluesky having special-cased their own preferences" is
honest and accurate, and "as safe or safer" holds on the axis that matters: Bluesky's private preferences
are plaintext their server can read, while yours are ciphertext nobody but your devices can read.

**Why prefs are the ideal training-wheels case for encrypted records.** The two standing objections to
encrypt-in-public-repo both collapse when the payload is a theme toggle:
- *Objection one is key loss*, but losing your prefs key costs you re-picking dark mode, so the recovery
  problem that made the vault (idea 16) hard is trivial here; new-device key distribution can be a QR pass
  from an existing device or PRF-derived, with "just reset prefs" as the always-available fallback.
- *Objection two is harvest-now-decrypt-later*, since repo history persists in mirrors and ciphertext
  written today is exposed if the key ever leaks — but the future decryption prize is your notification
  settings.

The residual leaks to name in the docs and accept: the record's existence (Croft user), and update
timestamps (when you fiddle with settings) — both minor for prefs and both reasons this pattern still
shouldn't graduate to bookmarks or anything sensitive. The official posture matches this calculus from the
other side: Bluesky has said real private content will be a major protocol component precisely because it
is "not a simple bolt-on to the current public repository data structure." They're right for sensitive
data; prefs are the case where the bolt-on is fine. And the lexicon is a good field regardless: when
protocol namespaces land, the same record type moves into private storage unchanged.

## Part 2 — the local mirror is a re-derivation of a PDS (assistant)

The satisfying answer to the IPFS thought is that atproto already speaks IPFS's data language, so you get
the whole design without IPFS-the-network. From the repository spec: repositories are "a graph of data
objects, encoded in DRISL CBOR and referencing each other by content hash (CID Links)," the record mapping
is held in a Merkle Search Tree — a "content-addressed deterministic data structure" — commits are signed
and verified against the account's DID document, and "complete repositories can be exported as CAR files
for synchronization, offline backup, account migration, or other purposes." CAR is literally the IPLD
transport format, "a serialized representation of any IPLD DAG as the concatenation of its blocks." The PDS
is a hash-crypto-tree sync store already. That was the design intent.

So the browser mirror is an assembly job, not an invention:
- **Fetch:** `com.atproto.sync.getRepo` for the full CAR, or `getBlocks`/per-collection `listRecords` for
  partial mirrors of just the collections an app cares about. Sync semantics recently improved for exactly
  this: under Sync v1.1, diffs stream the log of records in causal order rather than requiring MST
  tree-diffing, which makes incremental catch-up cheap.
- **Verify locally:** check the commit signature against the DID document's key and every block's CID
  against its hash. This is the part that quietly upgrades trust: the data is self-authenticating, so your
  mirror doesn't have to believe the PDS, it proves the content. A Croft app can render from mirror with
  cryptographic assurance even if the PDS were compromised.
- **Store:** a blockstore keyed by CID. Persistence toggle is just which backend the blockstore interface
  uses: OPFS for a durable mirror, memory for ephemeral, and "disable local persistence" falls out for
  free.

Where Helia (browser IPFS) would genuinely add something versus not: the shared CID format means your
blocks are IPFS-pinnable, which is interesting later for community archival — repos that outlive their PDS,
pinned by co-op members. But as the sync transport it subtracts: browser IPFS peer discovery is weak, and
the PDS is already the always-on, authoritative host with purpose-built sync endpoints. Use the formats,
skip the network.

Two practical seams: blobs (images, media) are referenced by content hash but not stored in the
repository, so the mirror fetches them separately via `getBlob` and they need their own cache policy; and
a heavy Bluesky account's repo CAR can be large, so partial-by-collection is the default posture, full
mirror an option.

**The compounding part.** This one library — call it `repo-mirror` — is the storage layer for half the
estate. `pdsview` gets offline browsing and verified rendering, which makes it the natural first home and
proof. Every Croft app gets read-your-own-data offline from the mirror plus the write outbox you already
planned, which together complete the local-first loop with the PDS as the sync authority. And your
encrypted prefs record from the first half syncs through it like any other record. One primitive, bought
once.

## Part 3 — one shared account origin for the whole estate (user)

Could we have this be at an `account.croft.ing` [source wrote `account.croft.io`, a typo] and shared
across the `*.croft.ing` estate [source wrote `*.Croft.omg`, a typo] rather than each subdomain managing
directly?

## Part 3 — response: the kernel (assistant)

Yes, and it's strictly better than per-subdomain mirrors. It's the same move as the auth broker, applied
to data: browser storage is per-origin, so if each subdomain ran its own mirror you'd get N copies of the
same repo, N sync loops, N quota bills, and N different ideas of "current." Hosting the mirror inside the
shared `account.croft.ing` origin, reached through the same embedded iframe, gives you one blockstore, one
sync loop, one truth, visible to every app. And the same-site logic that saves the auth iframe saves this
one: every `*.croft.ing` subdomain shares the `croft.ing` site, so the iframe keeps its unpartitioned
storage under modern tracker blocking.

Mechanics worth getting right:
- Apps read over **postMessage RPC** (`getRecord`, `listRecords`, `query`), which is cheap for
  record-sized payloads; blobs move as **transferable ArrayBuffers** or streams — zero-copy handoff rather
  than serialization, so even media stays fast.
- **One sync loop for the whole browser, not per tab.** A SharedWorker is origin-scoped, and every Croft
  tab's `account.croft.ing` iframe shares that origin, so a single worker owns the blockstore and the sync
  connection no matter how many apps are open. That also solves OPFS write contention, since sync access
  handles are exclusive and a single-writer design sidesteps locking entirely. Modern Safari restored
  SharedWorker; where it's absent, Web Locks leader election among the iframes gets the same single-writer
  effect.
- **Change events fan out over BroadcastChannel:** a record updates once, every open Croft app hears
  "`at://…/collection/rkey` changed" and re-renders. Live consistency across the estate for free.
- **The outbox moves in too.** Queued writes belong next to the keys that sign them and the mirror they'll
  eventually reflect, so the shared origin becomes: session broker, prefs cache, repo mirror, write outbox.
  One kernel.

That last word is the real consequence. The subdomains stop being apps-with-state and become **stateless
skins**: static bundles that hold nothing, talking to a client-side kernel origin that holds identity,
cache, and queue. Which strengthens every promise made so far. LTS gets easier because app bundles with
zero storage can be replaced or bit-rot harmlessly; clearing one app's site data destroys nothing; a new
Croft tool joins the estate already signed-in, already synced, on its first load, with nothing to build but
UI.

The seams, honestly: all eggs sit in one origin's quota and eviction policy, but the mirror is a cache
rebuildable from the PDS and the outbox is the only truly precious content — small and worth flagging to
`navigator.storage.persist()`. Safari's storage eviction applies to the shared origin, though use of any
Croft app refreshes the site's clock, as before. postMessage adds a hop per read, so the API should favor
batch queries over chatty single-record calls. And the kernel origin becomes the one bundle where a
compromise matters most, which argues for keeping it tiny, dependency-light, and the most carefully
reviewed code in the estate — the opposite of a kitchen sink; a kernel in the disciplined sense too.
