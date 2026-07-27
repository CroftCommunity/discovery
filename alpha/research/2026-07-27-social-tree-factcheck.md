# Fact-check — the 2026-07-27 "Social Tree" forum/reader PWA dialogue

date: 2026-07-27
purpose: verify the load-bearing technical claims in the 2026-07-27 Social Tree dialogue
(`seeds/transcripts/raw/bsky-reddit-forum-social-tree-pwa-2026-07-27.md`) against primary sources, so the
corpus does not carry dialogue-sourced errors forward. Method: three parallel research passes against
primary/authoritative sources — the `bluesky-social/atproto` GitHub lexicons + docs.bsky.app + Bluesky blog
(atproto claims); W3C/WHATWG specs, MDN, developer.chrome.com, WebKit blog, caniuse (browser/Web-Platform
claims); IETF datatracker RFCs + official project docs (crypto/protocol/infra claims). **Not committed until
reviewed.** atproto/iroh/iOS baseline facts defer to the FACTCHECK SoT
(`seeds/transcripts/raw/atproto-atmospheric-web-iroh-mobile-FACTCHECK.md`) and the E48 client-side-search
research (`research/atproto-clientside-search.md`) where they overlap.

**Headline: unusually accurate for a long AI product dialogue.** Of ~40 load-bearing claims checked, the
large majority are CONFIRMED. **Five are REFUTED / materially wrong; several are PARTLY with caveats that
matter to the design.** Notably, the single most consequential protocol error (the "native atproto E2EE DMs
/ Privately Shared Data" claim) is one the dialogue itself *partially self-corrects* two turns later, when
the user catches the plaintext-firehose leak ("Definitely not invisible at all") — that exchange re-derives
the corpus's own invariant **S5** (semi-private-on-a-public-plane leaks → make private data *structurally*
private).

---

## Corrections that matter (owner-relevant)

1. **REFUTED — "ATProto has native Privately Shared Data and native E2EE DMs."** Bluesky's native DMs are
   **server-side, NOT end-to-end encrypted** (visible to Bluesky PBC); there is **no shipped private-data
   primitive in atproto**. The only E2EE messaging in the Bluesky world is **Germ** (Feb 2026), a
   third-party **MLS-based layer on top of** atproto that does not modify the protocol. *Design impact:* the
   r=1 "Group-Private Data Enclave" cannot rest on a native atproto feature. The dialogue's *later* turns
   already reach the correct architecture — decouple identity from transport (sidecar MLS/Matrix, "Dark PDS,"
   or ephemeral WebRTC), and treat "Graft to Global" as a literal cryptographic threshold. Honor **S5**.
   (Sources: github.com/bluesky-social/atproto/discussions/121; the Germ launch coverage.)

2. **REFUTED — there is no `app.bsky.feed.hideReply` XRPC method.** Hidden replies are the **`hiddenReplies`
   array (of AT-URIs, max 300) on the `app.bsky.feed.threadgate` record** — write/update the record, there is
   no method of that name. (docs.bsky.app/docs/tutorials/thread-gates.)

3. **REFUTED — `app.bsky.feed.threadgate` has no native "mutuals only" (bidirectional-follow) rule.** The
   actual rules are `mentionRule`, `followingRule` (people the author follows), `followerRule` (people who
   follow the author), and `listRule`. "Mutuals" must be **composed client-side** (following ∩ follower).
   *Design impact:* the r=1→threadgate mapping (and the P3 "dual-write threadgate" trade-off) must state that
   the network-enforced gate can only approximate mutuals via following/follower rules, or use a `listRule`
   over a client-maintained mutuals list — it cannot enforce true bidirectionality at the PDS.
   (github.com/bluesky-social/atproto lexicons/app/bsky/feed/threadgate.json.)

4. **REFUTED — Privacy Pass is not "W3C Blind Signatures / RFC 9152."** RFC 9152 is an unrelated NSA
   certificate-delivery profile (SODP). Privacy Pass is **IETF** (not W3C): **RFC 9576** (Architecture),
   **RFC 9577** (HTTP Authentication Scheme), **RFC 9578** (Issuance Protocols), all June 2024. The
   anonymous-metering idea stands; the citation was wrong on both the number and the standards body.

5. **REFUTED — `PublicKeyCredential.supportsPrf` is not a real API.** The WebAuthn **PRF extension** and
   symmetric-key derivation from a passkey are real; detection is via
   `PublicKeyCredential.getClientCapabilities()` (look for `extension:prf`) or by reading
   `getClientExtensionResults().prf` on the create/get result. (w3c/webauthn explainer; MDN.)

6. **FRAMING — `site.standard.*` is real but COMMUNITY, not an official atproto/Bluesky namespace.** All four
   NSIDs exist (`site.standard.document`, `.publication`, `.graph.recommend`, `.graph.subscription`) and are
   used by Leaflet / pckt.blog / Offprint; the atproto blog promoted it, but it is **not a Bluesky-governed
   standard.** The other long-form lexicons the dialogue names are also real (WhiteWind `com.whtwnd.blog.
   entry`, Leaflet `pub.leaflet.*`). *Design impact:* the long-form layer can lean on `site.standard.*`, but
   as a community convention that could drift — carry a `$version`/renderer-registry hedge (which the dialogue
   already proposes). (standard.site/docs; atproto.com/blog/standard-site-bluesky-timeline.)

7. **PLATFORM CAVEAT that reshapes the "PWA is the floor" thesis — several key APIs are Chromium-only / not
   on iOS Safari.** This is the most important cross-cutting caveat:
   - **File System Access API** (`showDirectoryPicker`/`showSaveFilePicker`) — **Chromium only**; Firefox and
     Safari ship only **OPFS** (sandboxed), not arbitrary local-folder access. → The "continuous export to
     your Obsidian vault" story works on Chromium desktop; on Safari/Firefox the floor is OPFS + a
     download/`showSaveFilePicker`-less export, not live folder mirroring. **(Directly answers the user's
     "this is possible? please verify" — YES on Chromium; OPFS-only on Safari/Firefox.)**
   - **`share_target`, `protocol_handlers`, `file_handlers`** — real manifest members but **not supported on
     iOS/iPadOS Safari** (Chromium/Android-centric). → The "Add to Social Tree from any share sheet" and
     `web+at://` handoff are Android/desktop features, not iOS.
   - **Background Sync + Periodic Background Sync** — **Chromium only** (not Safari/iOS/Firefox). → The
     offline-outbox auto-replay and 4h background polling degrade to foreground-only on iOS.
   - **`navigator.connection`** (saveData/downlink/rtt) — **Chromium only.** → adaptive throttling needs a
     fallback on Safari/Firefox.
   - **WebKit 7-day eviction** of script-writable storage is real (IndexedDB included), **exempted by
     `storage.persist()` and Home-Screen install** — so the PWA must request persistence and prefer
     Home-Screen install on iOS, and treat the PDS + exported files as the source of truth (which the
     dialogue's defensive-strategy turn already says).

8. **PARTLY — the Bluesky "Top" tab ranking formula is unverified.** Bluesky documents only that Top is
   "algorithmically ranked"; the specific likes+reposts+replies blend the dialogue asserts is **not confirmed
   by any primary source** — treat as `[UNVERIFIED]`. (`since:`/`until:` YYYY-MM-DD and `from:` ARE confirmed.)

9. **PARTLY — WebRTC is not "E2EE by default" once an SFU is in the path.** DTLS-SRTP is mandatory but
   **hop-by-hop**; an SFU terminates it and sees plaintext media. True E2EE through an SFU requires **SFrame**
   (an IETF **draft**, `draft-ietf-sframe-enc`, not yet an RFC) over **WebRTC Encoded Transform**. The
   commercial-helper "the relay literally cannot decode" claim holds **only** with SFrame layered on;
   plain DTLS-SRTP through an SFU does not give it. Frame the relay tier accordingly.

10. **PARTLY — Web Monetization is a WICG Community-Group draft, not a "W3C standard."** `<link
    rel="monetization">` + Interledger streaming are correct; the standards status was overstated.

---

## Full verdicts by domain

### A. ATProto / Bluesky (primary: atproto GitHub lexicons, docs.bsky.app, bsky.social blog)

| Claim | Verdict | Note |
|---|---|---|
| `since:`/`until:` (YYYY-MM-DD) + `from:@handle` search operators | CONFIRMED | bsky.social search blog (also UTC-timestamp form). |
| "Top" tab = blended likes+reposts+replies | PARTLY / `[UNVERIFIED]` | Only "algorithmically ranked" documented. |
| `getFeed`, `searchPosts`, `getPostThread(depth)` XRPC methods | CONFIRMED | `getPostThread` also has `parentHeight`. |
| Post view has `replyCount` + `repostCount` + `likeCount` + `quoteCount` | CONFIRMED | `postView` in `app/bsky/feed/defs.json` (also `bookmarkCount`). |
| `site.standard.*` long-form family (4 NSIDs) | CONFIRMED but COMMUNITY | Leaflet/pckt.blog/Offprint; not official atproto. |
| `app.bsky.feed.threadgate` gates replies | CONFIRMED | rules: mention / following / follower / list. |
| threadgate native "mutuals only" rule | REFUTED | compose following∩follower client-side. |
| `hiddenReplies` field on threadgate | CONFIRMED | array of AT-URIs, max 300. |
| `app.bsky.feed.hideReply` method | REFUTED | no such method; use the threadgate field. |
| `getMutes` / `getBlocks` methods | CONFIRMED | plus `getListBlocks`/`getListMutes`. |
| `graph.list` (modlist purpose) + `graph.listitem` = subscribable mod lists | CONFIRMED | |
| `cdn.bsky.app/img/{feed_thumbnail,feed_fullsize,avatar_thumbnail}/plain/...` | CONFIRMED | imgproxy-backed. |
| `public.api.bsky.app` unauthenticated CORS reads, no key | CONFIRMED | AppView read endpoint. |
| `app.bsky.actor.profile.pinnedPost` | CONFIRMED | `ref` → `com.atproto.repo.strongRef`. |
| Jetstream `wss://jetstream1.us-east.bsky.network/subscribe?wantedCollections=` | CONFIRMED | 4 public instances; JSON firehose. |
| "native E2EE DMs / Privately Shared Data" | REFUTED (aspirational) | native DMs server-visible; Germ = 3rd-party MLS layer. |
| atproto OAuth granular per-lexicon scopes | CONFIRMED (Aug 2025) | e.g. `repo:app.bsky.feed.post`; broad `transition:*` still exists alongside. |
| `app.bsky.embed.external` / `.images` / `.video` | CONFIRMED | embed union members. |

### B. Browser / Web-Platform APIs (primary: W3C/WHATWG, MDN, developer.chrome.com, WebKit, caniuse)

| Claim | Verdict | Note |
|---|---|---|
| Digital Credentials API `navigator.credentials.get({digital})`, OpenID4VP + ISO mdoc | CONFIRMED | protocol string `org-iso-mdoc`. |
| "stable in Chrome 141 and Safari 26, late 2025" | CONFIRMED | Chrome 141 default ~Sept 2025; Safari 26.0 Sept/Oct 2025. Timing was right. |
| WebAuthn PRF derives symmetric keys | PARTLY | PRF real; detect via `getClientCapabilities()` / `getClientExtensionResults().prf` — **not** `supportsPrf`. |
| OPFS sandboxed + sync access handles in Workers, Chrome/Safari/Firefox | CONFIRMED | Chrome 86+, FF 111+, Safari 15.2+; sync handles Worker-only. |
| File System Access (`showDirectoryPicker`) — NOT Firefox/Safari | CONFIRMED | Chromium 86+ only. |
| `chrome.storage.sync` ~100KB total / ~8KB per item | CONFIRMED | QUOTA_BYTES 102400 / PER_ITEM 8192. |
| `chrome.bookmarks` extension-only (`onCreated`); PWA cannot | CONFIRMED | needs `"bookmarks"` permission. |
| Web Locks `navigator.locks.request` cross-tab leader election | CONFIRMED | all evergreen, secure context. |
| BroadcastChannel same-origin tab sync | CONFIRMED | Baseline Mar 2022; not persisted. |
| `share_target` / `protocol_handlers` / `file_handlers` | PARTLY | real but Chromium/Android; **not iOS Safari**. |
| Background Sync + Periodic Background Sync | CONFIRMED Chromium-only | not Safari/iOS/Firefox. |
| iOS Web Push only via Home-Screen PWA (16.4+) | CONFIRMED | not in-Safari. |
| `storage.persist()`/`estimate()`; WebKit 7-day eviction | CONFIRMED | persist() + Home-Screen exempt from the 7-day counter. |
| `navigator.connection` (saveData/downlink/rtt) | CONFIRMED Chromium-only | not Safari/Firefox. |
| Web Monetization `<link rel=monetization>` + Interledger | PARTLY | WICG CG draft (Mar 2025), **not** a W3C standard. |
| WebGPU cross-browser; WebNN draft; Transformers.js quantized client-side | CONFIRMED | WebGPU Baseline Jan 2026; WebNN = W3C Candidate Rec / experimental. |

### C. Crypto / networking / protocol (primary: IETF datatracker, W3C, project docs)

| Claim | Verdict | Note |
|---|---|---|
| Privacy Pass = "W3C Blind Signatures / RFC 9152" | REFUTED | IETF RFC 9576/9577/9578; RFC 9152 is unrelated NSA SODP. |
| MLS = RFC 9420, E2EE group messaging | CONFIRMED | Standards Track; FS + PCS. |
| WebRTC media E2EE by default via DTLS-SRTP | PARTLY | hop-by-hop; SFU sees plaintext unless SFrame added. |
| SFrame over WebRTC Encoded Transform for E2EE-through-SFU | CONFIRMED (SFrame = draft) | `draft-ietf-sframe-enc` -09; Encoded Transform is a real W3C spec. Not an RFC. |
| STUN + TURN(~15-20%); coturn + LiveKit real | CONFIRMED (% approximate) | coturn = TURN server; LiveKit = SFU. |
| ThumbHash + BlurHash | CONFIRMED | both real placeholder encodings. |
| AES-256-GCM + PBKDF2 via Web Crypto `crypto.subtle` | CONFIRMED | W3C Rec 2017; `deriveKey()`. |
| Yjs + Automerge (CRDT) + Hybrid Logical Clocks | CONFIRMED | HLC = Kulkarni/Demirbas 2014. |
| OpenID4VP + cross-device VP over BLE/QR (or ISO 18013-7) | CONFIRMED | dedicated "OpenID4VP over BLE" draft; 18013-7 Annex C. |

---

## What this closes / feeds

- Clears the whole-document fact-check caveat on the raw
  (`seeds/transcripts/raw/bsky-reddit-forum-social-tree-pwa-2026-07-27.md`) — the raw header points here as
  source of truth for its load-bearing claims.
- The five REFUTED items and the iOS-Safari platform caveats are folded into the ROADMAP_TODO E-item for the
  Social Tree so the design carries the corrections, not the errors.
- Corroborates E48 (client-side search substrate) and E24/E42 (sovereign AppView: private data must be
  *structurally* private, not namespace-hidden — S5). The "native E2EE DM" refutation reinforces that the
  private-comms plane needs a sidecar (MLS/Matrix/Dark-PDS/WebRTC), consistent with the beta MLS work.
