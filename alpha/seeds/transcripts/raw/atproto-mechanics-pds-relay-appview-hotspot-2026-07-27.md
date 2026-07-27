# Raw: atproto mechanics deep-dive (PDS/relay/AppView/blobs/auth/rev/inversion) + phone-hotspot P2P — Q&A (2026-07-27)

**Preservation status: preserved-condensed (cleaned-paste, content-faithful — NOT byte-pristine) —
PLAYBOOK §4.** Source: two claude.ai Q&A exchanges pasted 2026-07-27 (mid-session). Thinking-summary lines and
inline citation markers left as authored; no secrets. Two topically-paired exchanges filed together: (1) a
short networking Q&A on whether two phones can talk over one phone's Wi-Fi hotspot; (2) a long, heavily
citation-backed explainer on how atproto works at Bluesky (PDS ↔ relay ↔ AppView, account lifecycle, OAuth,
blobs, firehose, rev/inversion, proxying).

**FACT-CHECK.** The load-bearing quantitative/status claims are verified in
`../../../research/2026-07-27-social-tree-factcheck-2.md` (iOS-18 hotspot IPv4 client-isolation; Local
Network permission; firehose size limits 2 MB blocks / 1 MB record / 200 ops / 5 MB frame; relay
~2 vCPU/12 GB/30 Mbps; `PDS_CRAWLERS` default; getRepo = current-state-only; tooBig→#sync). Baseline atproto
facts also defer to the FACTCHECK SoT (`atproto-atmospheric-web-iroh-mobile-FACTCHECK.md`) and the earlier
`atproto-architecture-appview-relay-explainer-2026-06-22.md` — **this explainer is a deeper superset of that
one**; distil into a single atproto-mechanics reference rather than duplicating.

---

## Exchange 1 — two phones over one phone's Wi-Fi hotspot

**Q:** Can two phones communicate if one is tethering the other with a Wi-Fi hotspot?

**A (condensed).** Host↔client is the easy case and needs no internet: the hotspot phone is AP + DHCP + router;
the client gets an IP on the host's subnet with the host as gateway, so any TCP/UDP socket between client and
gateway is local link traffic (works with cellular off/unpaid — nothing leaves the phone). Three caveats:
(1) **client↔client is different** — two phones both joined to a *third* phone's hotspot may be isolated;
Apple DTS (developer forums) states that **as of iOS 18 you cannot use IPv4 between client devices on an
iPhone Personal Hotspot — must use IPv6 (link-local works)**; Personal Hotspot is a user feature, not an API,
focused on client→internet, so it's not a guaranteed surface (no equivalent authoritative Android statement
found). (2) **Discovery is flakier than the connection** — unicast to a known IP is reliable; mDNS/Bonjour and
broadcast/multicast over Wi-Fi silently fail; prefer handing over the gateway IP. (3) **iOS Local Network
permission** — since iOS 14 an app needs it before using the local network (`NSLocalNetworkUsageDescription`
in Info.plist; denials return `kDNSServiceErr_PolicyDenied`; ref TN3179 "Understanding local network
privacy"). For the two-phone one-hotspot case none of the isolation caveats apply — read the gateway address
off the client's Wi-Fi details and point at it.

---

## Exchange 2 — how atproto works at Bluesky (PDS/relay/AppView), with the user's mental model corrected

**User's initial model:** a PDS (content + metadata) plus some Bluesky object store; users write to their own
PDS; a PDS streams to a relay by default out of the box; the relay fans out over a pub/sub the giant Bluesky
AppView consumes and serves over the default API + Jetstream; content gets labeled somewhere; other AppViews
consume the same relay output for their own views; the Bluesky AppView stores everything forever, the relay
keeps limited backfill, and the PDS is ultimately authoritative. **A: mostly right — refinements below.**

**A (condensed, citation-backed to atproto.com/bluesky docs).**

**1. Account lifecycle / identity / auth / invites.** Identity (immutable **DID**) vs account (on a PDS:
repo hosting + authz/authn + blob storage); DID persists across hosts, account does not. Hosting status =
active boolean + a more specific status (`deleted`/`deactivated`/`takendown`/`suspended`/`desynchronized`/
`throttled`). **Account status is NOT self-certifying** — it propagates hop-by-hop, each downstream adopting
its upstream's status; when in doubt query `com.atproto.sync.getRepoStatus` against the account's current
PDS; a PDS *can* misrepresent activation state. Migration at the protocol level = DID doc points at the new
host + active there + valid repo with good signature and higher `rev` (seamless flow: create on new PDS
[often deactivated] → move data → update DID doc + new signing key [rotate PLC rotation keys] → flip status
on both → emit `#sync`/`#commit` signed with the new key at higher rev). Invites/email = deployment policy
not protocol (official PDS ships `PDS_INVITE_REQUIRED=true`); **email gates PLC operations** because the PDS
holds `PDS_PLC_ROTATION_KEY_...` and can sign PLC ops on your behalf (`identity.requestPlcOperationSignature`
email-token-gated before `signPlcOperation`/`submitPlcOperation`).

**2. Auth = two separate systems.** **Client↔server = OAuth**: client fetches PDS metadata to discover the
Authorization Server; **DPoP with mandatory server-issued nonces** binds tokens to a client instance; **PAR**;
confidential clients auth with a JWT signed by a secret key; **`client_id` = the HTTPS URL of the client
metadata JSON**, which must equal where it's served. **The AS is NOT necessarily the PDS** — may be the PDS
same-origin, or a separate "entryway" in large multi-PDS deployments (Bluesky: repos on `*.host.bsky.network`
shards, `bsky.social` is the AS). Discovery: `/.well-known/oauth-protected-resource` on the PDS (200, JSON,
`authorization_servers` array of exactly one origin) → `/.well-known/oauth-authorization-server` on that
origin (RFC 8414; `scopes_supported` includes `atproto`). **Mandatory client check:** resolve the DID doc →
extract declared PDS → fetch its resource metadata → confirm it resolves to the AS bound to the session +
issuer matches (else a rogue server could authenticate arbitrary DIDs). Chain of authority = DID doc → PDS →
AS, never the reverse. Refresh tokens generally single-use (lock to prevent concurrent refresh — bites
multi-tab SPAs). **Service↔service = a different mechanism**: the PDS mints a short-lived (≈<60s)
asymmetrically-signed JWT (`iss`=user DID, `aud`=target service DID, `lxm`=endpoint NSID) with the same key
that signs repo commits. The `rpc` OAuth permission (getServiceAuth / proxied requests) is parameterized by
`lxm`+`aud` and must be restricted by at least one.

**3. Blobs = two-phase commit.** Upload to `com.atproto.repo.uploadBlob` first (server can't know the lexicon
yet → generic limits only); blob sits in **temporary storage** (not downloadable, excluded from `listBlobs`);
on record creation the server extracts referenced blobs, checks they're referenced/in-temp, then makes them
public + validates against the referencing record's lexicon; on record deletion it refcounts within the same
repo and deletes if unreferenced (GC grace: several hours recommended, 1h floor). Integrity: the blob CID
lives in a signed record, so fetched bytes verify against a signature you already checked; **blobs do NOT
ride the firehose** (`#commit.blobs` deprecated → empty array; discover by parsing records). **Serving isn't
the PDS's job** — CSP+sandbox on `getBlob` effectively mandatory; the AppView plays CDN (thumbnails/resize on
separate untrusted infra); PDS should not resize/transcode.

**4. Firehose emission + requestCrawl.** The PDS is itself a firehose producer (`com.atproto.sync.subscribeRepos`
covers all its hosted accounts). Discovery is push-then-pull: PDS calls `requestCrawl` against each configured
crawler (`PDS_CRAWLERS`, default `https://bsky.network`), relay opens a `subscribeRepos` WS back (fresh deploy
auto-broadcasts → wipe+reinstall on same hostname desyncs PDS and relay). PDS must get right: **rev
discipline** (rev = TID from wall time, MUST strictly increase per repo even across migration/inactivity;
consumers ignore rev ≤ last-seen and reject rev beyond a few-min future window; empty commits legal;
`Atproto-Repo-Rev` header = read-your-writes); **inversion metadata** (`#commit` carries `prevData` = previous
MST root; consumers invert the ops list against the diff blocks and check it equals the previous data root —
the "inductive firehose"; `prevData` is NOT signed/authenticated → verify against your own recorded state);
**size limits** (blocks ≤ 2,000,000 bytes; per-record block ≤ ~1,000,000; ≤ 200 ops/commit; hard 5 MB WS
frame ceiling — e.g. 50 ops × 60 KB passes op/record limits but blows the blocks ceiling); event ordering on
lifecycle transitions (`#identity` once DID resolves to the current PDS; `#account` on reactivation validated
as from the current PDS). Sequence spaces are per-host (a PDS cursor means nothing to a relay); relays may
rate-limit/throttle.

**5. Proxying to AppViews.** Authenticated app requests go through the user's PDS and are proxied via the
`atproto-proxy` header (DID + service fragment, e.g. `did:web:api.bsky.app#bsky_appview`); the PDS resolves
the identity and forwards, attaching a fresh service-auth JWT so a third-party AppView can trust "on behalf
of this DID" without seeing user credentials. The **Bluesky AppView is a special case** — proxying to it was
still server-side-configured as of Fall 2024, so the header isn't required for `app.bsky.*`. Cost: proxying
every call adds read latency + blocks write-anticipation; public endpoints can hit `public.api.bsky.app`
(cached) directly.

**On "is the PDS constantly serving full history?" — three corrections.** (a) **There is no history to
serve**: `com.atproto.sync.getRepo` returns *all current* records + MST nodes + current signed commit in one
CAR — **no prior record versions**; you build history yourself by retaining firehose events. (b) **Not
constant**: steady state is the incremental push firehose; a full CAR fetch happens only on bootstrap
(never-indexed repo) or repair (broken chain) — per-repo sync status `desynchronized`/`in-progress`/
`synchronized`, diff CAR vs a record-state table to derive create/update/delete, queue events during repair.
(c) **Load steered off the PDS**: to avoid a thundering herd, consumers request the CAR from their direct
upstream (often a relay, which coalesces/caches) which may HTTP-redirect to the PDS; mirrors must honor
deletions/status within seconds-minutes (don't redistribute static repo snapshots as archival torrents);
`com.atproto.sync.listReposByCollection` finds DIDs holding a given collection for backfill. `tooBig` is
deprecated → replaced by `#sync` + the data limits (every commit self-contained/verifiable-in-isolation).

**Relay resource figure:** a full-network relay doing signature validation + MST inversion on every message
runs on roughly **2 vCPU / 12 GB RAM / 30 Mbps** (disk driven by the backfill window).

**The shape underneath:** the PDS is the *only* component holding a private key on the user's behalf and the
only one that can speak as them; relays/AppViews/labelers are readers and annotators — that asymmetry is the
whole security model, and is why "choose your PDS carefully" is not boilerplate.

---

## Relationship to the corpus
- Deep superset of `atproto-architecture-appview-relay-explainer-2026-06-22.md`; corroborates the FACTCHECK
  SoT (Merkle Search Tree, DID/PDS/relay/AppView, getRepo=CAR, Jetstream) and E48.
- The iOS-18 hotspot IPv4 client-isolation + Local Network permission facts feed the iOS-P2P watch-items
  (`beta/impl/ios-background-execution-and-the-ble-caution.md`; FACTCHECK SoT iOS section) — relevant to any
  Croft local-mesh / two-device durability story.
- The blob two-phase-commit + per-DID scoping detail is the mechanical basis for the E2EE-storage-contract
  insight in the companion compound transcript (see `croft-landscape-...-2026-07-27.md`).
