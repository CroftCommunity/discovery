# Fact-check (part 2) — the 2026-07-27 Croft-landscape + atproto-mechanics batch

date: 2026-07-27
purpose: verify the load-bearing claims in the second/third/fourth transcripts of the 2026-07-27 batch —
`seeds/transcripts/raw/croft-landscape-small-world-tree-ui-webvh-blob-coop-2026-07-27.md` and
`seeds/transcripts/raw/atproto-mechanics-pds-relay-appview-hotspot-2026-07-27.md` — against primary sources.
Method: four parallel research passes (atproto/did:webvh/blob; auth/crypto/payments; UX research;
infra/ecosystem/iOS/atproto-quant). **Not committed until reviewed.** Companion to
`2026-07-27-social-tree-factcheck.md` (part 1, the Social-Tree PWA transcript). atproto/iroh/iOS baseline
facts defer to the FACTCHECK SoT.

**Headline: unusually accurate across all four domains.** Of ~40 load-bearing claims, the large majority are
CONFIRMED verbatim against primary sources. The corrections are small and specific; none collapses a thread.

---

## Corrections that matter

1. **UNVERIFIED — iOS-18 Personal-Hotspot client-to-client IPv4 isolation is forum-only.** The claim ("as of
   iOS 18 you cannot use IPv4 between two client devices on a Personal Hotspot, must use IPv6 link-local") is
   supported *only* by an Apple Developer **forum** thread relaying a Feedback-Assistant reply — not an
   official DTS/eskimo post or a TN. The "host↔client IPv4 works fine" half is **not documented anywhere**.
   Treat as UNVERIFIED / needs-device-test before any Croft local-mesh story leans on it. (The iOS-14+ Local
   Network permission / `NSLocalNetworkUsageDescription` / `kDNSServiceErr_PolicyDenied` / TN3179 facts ARE
   CONFIRMED against official docs.)

2. **CORRECTION — Riseup pads are removed after 60 days of inactivity, not ~30.** (The ~12h share TTL and the
   Disroot/Riseup/CHATONS/CryptPad service facts are CONFIRMED; CHATONS = Framasoft 2016 confirmed.)

3. **UPDATE — DBSC is shipped now, not forward-looking.** As of 2026-07-27, Device Bound Session Credentials
   is **GA on Windows in Chrome 146 (April 2026)**, wider rollout began ~May 25 2026, macOS Secure-Enclave
   support next. Nuance: the "~every 5 minutes" refresh cadence is **implementation-defined, not
   spec-mandated** (spec says browsers MAY refresh proactively); "Safari/Firefox still evaluating" is not
   confirmable from a primary source. The resident-attacker containment caveat is CONFIRMED verbatim.

4. **NUANCE — TOTP resync-window figures are the validator's choice, not RFC-prescribed.** RFC 6238 (TOTP) /
   RFC 4226 (HOTP) / HMAC-SHA1 basis, local-HMAC verification, single-use, and rate-limiting are all
   CONFIRMED — but the specific "±1 step / ~89s max drift / 2 backward steps" numbers overstate what the RFC
   fixes (it recommends keeping the window small; the exact size is the validator's).

5. **CORRECTION — `popover` is Baseline *Newly available* (Jan 2025), not *Widely available*.** `<dialog>`/
   `showModal` (Mar 2022) and CSS container size queries (Aug 2025) ARE Baseline widely available; the blanket
   "all widely available" is wrong only for `popover` → use it as progressive enhancement.

6. **NUANCE — Path's "50" is inspired-by-Dunbar, not Dunbar's number.** Dunbar's number is 150 (which became
   Path's v2.0 cap); the original 50 cap was a stricter intimacy limit inspired by Dunbar's research. Launch
   Nov 2010, 50→150 in v2.0 (2012), users settled ~50, Morin "the home inside Facebook's city" — all CONFIRMED.

7. **NUANCE — the 1995 hyperbolic-tree paper is Lamping, Rao & *Pirolli* (CHI'95).** The claim omitted
   Pirolli. The "no significant performance difference" result is corroborated; the *causal attribution*
   (label overlap + ill-structured WWW hierarchy) and the "users preferred it" line are not confirmable
   against the paywalled primary text (much of the scent nuance is actually the 2003 TOCHI follow-up, which is
   fully CONFIRMED).

8. **NUANCE — MIMI's protocol is an IETF WG Internet-Draft, not an RFC.** `draft-ietf-mimi-protocol` (-04,
   July 2025) + `draft-ietf-mimi-content`; the WG is chartered on MLS (RFC 9420). "Builds on MLS" ✓, "RFC" ✗.

9. **ATTRIBUTION — the blob dedup "comes for free with a CID-indexed store" phrasing was DavidBuchanan314's**
   (the by-CID proposer in atproto discussion #1756), responding-to, not Newbold's; and the "uploadBlob by
   CID" shortcut was **not adopted**, over the abuse concern. Newbold's per-DID-scoping rationale (lifecycle
   + per-context moderation) and "very little dedup observed" ARE his and ARE confirmed.

10. **NUANCE — did:webvh "watchers reaching consensus" is the author's gloss.** The spec frames watchers as
    distributed caches / an alternative resolution source / a malicious-controller-detection purpose —
    distinct from **witnesses** (approve-before-publish). No consensus protocol among watchers is specified.
    Everything else in the watcher/versionTime thread is CONFIRMED verbatim (versionTime "as asserted by the
    DID Controller"; strictly-increasing/equal-rejected; ~5-min non-futurity; `?versionTime=`; authenticity
    rests on SCID + entry hashes + data-integrity proofs, not versionTime).

11. **NUANCE — atproto `tooBig` deprecation.** `tooBig` is deprecated (set false / ignore) and the size limits
    supersede the old too-big flow; but framing `#sync` as a 1:1 "replacement" overstates it — `#sync` asserts
    current repo status. (Minor: Vaultwarden stores a few metadata fields in plaintext; modern Go 1.17+ folds
    `netgo` into `CGO_ENABLED=0`.)

---

## Full verdicts by domain (compact)

### atproto / did:webvh / blobs / MIMI — all CONFIRMED except the nuances above
did:webvh versionTime + monotonicity + ~5-min non-futurity + `?versionTime=` ✓; authenticity = SCID + entry
hashes + data-integrity proofs (versionTime = index not evidence; uniform-backdating passes checks) ✓;
watchers (log-param URLs, resolution metadata, cache/persist-after-deletion, detect altered history) ✓,
distinct from witnesses ✓ (consensus = gloss); MIMI = IETF WG on MLS, protocol = WG draft ✓; blob per-DID
scoping / Newbold #1756 (lifecycle + per-context moderation; "very little dedup") ✓ (attribution nuance);
`(DID,CID)` tuple ✓; `getBlob` unauthenticated/public ✓; upload → temp storage → validated at record-
reference time per lexicon ✓; `uploadBlob` takes bytes → Dropship injection N/A ✓.

### auth / crypto / payments — all CONFIRMED except DBSC-status/TOTP-window nuances
passkey ≠ DPoP key (WebAuthn signs `authenticatorData ‖ SHA-256(clientDataJSON)`, not chosen JWS bytes;
per-gesture vs per-request) ✓; DBSC mechanism ✓ (shipped Chrome 146 GA Windows Apr 2026; 5-min = impl-
defined); DBSC resident-attacker caveat ✓ verbatim; TOTP=RFC 6238 / HOTP=RFC 4226 / HMAC-SHA1 / local-HMAC /
single-use / rate-limit ✓ (window = validator's); atproto OAuth binds session to client JWKS key + MUST
revoke on key removal ✓ verbatim; NIP-57 zaps (kind 0 lud16 / 9734 unpublished→LNURL / 9735 published;
allowsNostr + nostrPubkey; description-hash invoice binding; receipt = provider attestation not proof; plain-
pay bypasses 9735) ✓; BIP-340 Schnorr + NWC=NIP-47 ✓; Damus/Apple June 2023 zap removal ✓.

### UX research — CONFIRMED except the Path/Pirolli/popover nuances above
1995 hyperbolic browser = Lamping/Rao/Pirolli CHI'95, no-perf-diff ✓ (causal attribution UNVERIFIED); Pirolli/
Card/Van Der Wege TOCHI 2003 information-scent high/low conditional ✓; 2024 ego-network "Me! Me! Me! Me!"
(*Computers & Graphics*) layered-node-link-preferred / matrix-hardest ✓; Path 2010 / 50→150 / Morin quote ✓;
Multitrees = Furnas & Zacks CHI'94 ✓; Baymard 58%/78% (2025) ✓; 28%/72% applied-filters ✓; `<dialog>` +
container-queries Baseline-widely ✓ (popover = Newly); WCAG 2.5.8 (24px AA) + 2.5.1 (pointer gestures) ✓;
MiniSearch client-side ✓.

### infra / ecosystem / iOS / atproto-quant — CONFIRMED except iOS-hotspot (UNVERIFIED) + Riseup-pads (60d)
Disroot services ✓; Riseup (share ~12h ✓, pads 60d not 30); CHATONS/Framasoft 2016 ✓; CryptPad browser-E2EE ✓;
Vaultwarden Bitwarden-compatible client-encrypted ✓ (few metadata fields plaintext); Miniflux single Go binary
+ Postgres ✓; deck.blue (Aug 2023) ✓; TOKIMEKI (MIT ✓, PWA/self-host = corroboration); SkyFeed feed builder ✓;
Ouranos/Skeets-Pro/Graysky/Sora exist ✓; `cargo build` static crate linking ✓; per-target CRT (glibc dynamic /
musl static / MSVC dynamic) ✓; RFC 1721 "C runtime static linking" + `crt-static` knob ✓; glibc forward-compat-
only ✓; Go cgo/CGO_ENABLED=0/netgo/osusergo ✓; iOS Local Network permission + TN3179 ✓; iOS-18 hotspot client
IPv4 isolation = forum-only UNVERIFIED; firehose limits 2 MB blocks / 1 MB record / 200 ops / 5 MB frame ✓;
relay ≈ 2 vCPU / 12 GB / 30 Mbps ✓; `PDS_CRAWLERS` default `https://bsky.network` + requestCrawl ✓; getRepo =
current-state-only CAR ✓; tooBig deprecated (nuance: size limits supersede; #sync asserts current status).

---

## What this closes / feeds
- Clears the fact-check caveats on both new raws (their headers point here).
- The corrections are baked into ROADMAP_TODO E63–E70 and the E62 update, and into ECOSYSTEM §5j / COHESION
  §60 — the corpus carries the corrections, not the errors.
- Reinforces existing lines: the blob→E2EE-storage-contract insight corroborates `research/atproto-private-
  data-architecture.md`; the did:webvh federation-of-availability result feeds heartwood/Drystone (E30/E31);
  the auth cluster feeds the auth-helper spike + account kernel; the iOS-hotspot UNVERIFIED flag is a watch-
  item for any local-mesh durability story.
