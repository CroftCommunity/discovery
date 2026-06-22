# Fact-check — AT Proto atmospheric-web / Iroh mobile dialogue

date: 2026-06-22 · companion to `atproto-atmospheric-web-iroh-mobile-dialogue.md`

purpose: the raw dialogue is AI-generated (Gemini, flagged by the user as sometimes unreliable).
Per the user's explicit request, every substantive assertion was fact-checked — Iroh claims
against the project's own pinned-`=1.0.0` verified source plus web; all other clusters against
web primary sources (atproto.com, iroh.computer/docs, GitHub, crates.io, Apple developer docs,
Wikipedia). Verdicts: **CONFIRMED** (exists & as-described) · **PARTLY** (real but
mis-described) · **REFUTED** (false / no such thing) · **UNVERIFIABLE** (no credible source).

## Headline

Far more grounded than a typical hallucination-heavy transcript. The hard technical mechanics
(AT Proto labelers/feeds/XRPC/DID, iroh range-based reconciliation, gossip HyParView/Plumtree,
custom-transports flag, QUIC-multipath migration, BLAKE3) are **accurate**. Gemini's failure
mode here is **provenance/packaging drift** (invented codenames, mis-attributed or invented
crate names, first-party-vs-community confusion) plus a few **overstated iOS capabilities** and
**mis-described products**. Treat existence/concept claims as reliable; verify every crate name,
codename, product mechanic, governance/standards framing, and exact API before relying on it.

## Two outright fabrications (REFUTED) — do not carry forward as fact

1. **"AT Proto encryption working group standardizing 'AT Messaging' with MLS."** No such
   official working group or product. Real AT-Proto E2EE is **third-party**: **Germ Network**
   (Germ DM, uses MLS — and already in this corpus: `seeds/transcripts/raw/germ-xchat-design-dialogue.md`,
   `research/germ-xchat-features.md`) and an **XMTP↔Bluesky bridge** (XMTP Labs `bluesky-chat`).
   ATProto's own docs list MLS/Matrix/Signal only as *possible future* work.
   (src: github.com/bluesky-social/atproto/discussions/121; blog.xmtp.org)
2. **"Keen" as an internal codename for `iroh-docs`.** No primary source links any "Keen" to
   n0/iroh. Almost certainly invented. (`iroh-docs` is simply `iroh-docs`.)

## Cluster 1 — AT Proto mechanics & atmospheric-web apps

| Claim | Verdict | Note (src) |
|---|---|---|
| "Atmospheric Web" is a real community term | CONFIRMED | atproto.com/blog/atmospheric-website |
| Standard.site = long-form publishing lexicon on PDS | PARTLY | Real, but **not Markdown-specific** (over-specified) |
| Semble = Linktree-style link curation | PARTLY | Real & on ATProto, but it's a **research knowledge network**, not a Linktree clone |
| Smoke Signal = decentralized Meetup, OSS, ~1yr | CONFIRMED | Name is singular "Smoke Signal"; 1yr on 2025-07-14 |
| npmx = npm browser w/ ATProto sign-in | PARTLY | Real npm browser + ATProto sign-in; "favorites" detail unconfirmed |
| Tangled (tngl.sh) = decentralized Git on ATProto | CONFIRMED | Now tangled.org; `user.tngl.sh` handles |
| Leaflet = long-form on PDS | PARTLY | Real; **block-based editor, not Markdown-primary** |
| WhiteWind (whtwnd.com) = Markdown blogging | CONFIRMED | OSS, data on PDS |
| Streamplace = livestream over ATProto | CONFIRMED | OSS, Livepeer-funded |
| Flashes = Instagram-like ATProto client | CONFIRMED | by Sebastian Vogelsang (3rd-party) |
| Automattic "ATmosphere" WordPress plugin | CONFIRMED | v1.0.0 May 2026 |
| Graysky defined `app.graysky.*` namespace | CONFIRMED | by @mozzius |
| Blacksky = independent ATProto infra | CONFIRMED | Rudy Fraser; Rust "rsky"; AppView in dev |
| Neocities, 2013, ~1.6M sites | CONFIRMED | ~1.61–1.65M |
| Puter = browser-OS w/ publish | CONFIRMED | OSS |
| Ozone / decoupled labelers / signed labels | CONFIRMED | atproto.com/specs/label |
| Feed Generators read firehose → URI skeletons | CONFIRMED | atproto.com/guides/feeds |
| did:plc + did:web, domain handles | CONFIRMED | atproto.com/guides/identity |
| XRPC = HTTP RPC layer | CONFIRMED | atproto.com/specs/xrpc |
| "open union" embeds; client shows placeholder | PARTLY | Open union real; client shows **empty embed**, generic fallback is *planned*; doesn't crash |
| Official client hardcoded to one AppView, ignores custom lexicons | PARTLY | Routes app.bsky.* to bsky AppView & won't render custom lexicons, but "hardcoded/ignores" is too strong — unknown fields are passed through |
| Zeppelin full-network AppView, ~16 TB | CONFIRMED | ~16 TB / ~$200/mo Hetzner mid-2025; **now decommissioned** |
| XMTP↔ATProto private-messaging bridge | CONFIRMED | XMTP Labs `bluesky-chat` |
| Relay/firehose free over WebSockets | CONFIRMED | com.atproto.sync.subscribeRepos |

## Cluster 2 — Iroh ecosystem (web + project's own `IROH-1.0.0-API-VERIFIED.md`)

| Claim | Verdict | Note (src) |
|---|---|---|
| iroh-docs uses **Merkle Search Trees** | **REFUTED** | Uses **range-based set reconciliation** (Willow/Meyer 2022). MSTs are AT Proto's structure — conflated |
| iroh-docs uses CRDT / last-writer-wins | CONFIRMED | eventually-consistent KV, LWW (matches our spike's "LWW" note) |
| iroh-docs metadata (BLAKE3 hash) split from blob bytes → iroh-blobs | CONFIRMED | docs.iroh.computer/protocols/documents |
| "Keen" = iroh-docs codename | REFUTED | hallucination (see above) |
| iroh-gossip = HyParView + Plumtree | CONFIRMED | matches our `altdrive-spike-gossip` |
| `unstable-custom-transports`, QUIC over ≥~1,200-byte datagrams | CONFIRMED | introduced iroh **0.97.0** |
| Iroh 1.0 added custom transports incl. **BLE in core** | PARTLY | Custom transports real; **BLE is a community crate (mcginty), not core/n0** |
| First-party Swift bindings, **mid-2026** | CONFIRMED | `n0-computer/iroh-ffi` shipped w/ iroh 1.0, mid-June 2026 (I had pre-flagged this suspect — web vindicated it) |
| Crates iroh-ble/tor/nym/webrtc-transport, iroh-pkarr-node-discovery | PARTLY | `iroh-pkarr-node-discovery` ✓; Tor = `n0-computer/iroh-tor` (not `-transport`, not on crates.io); Nym = `n0-computer/iroh-nym`; BLE = community `mcginty/iroh-ble-transport`; **`iroh-webrtc-transport` not found — likely hallucinated** |
| BlewChat + `blew` crate, GATT→L2CAP→QUIC | PARTLY | BlewChat, `blew`, GATT→L2CAP real; QUIC rides over the BLE link (not part of the upgrade ladder). ⚠️ BlewChat is an **unencrypted demo** |
| Aster = local-first P2P music on iroh | CONFIRMED | awesome-iroh |
| Obsiroh = Obsidian sync via iroh | PARTLY | Real; "uses iroh-docs" specifically unconfirmed |
| Fish Folk: Jumpy on iroh/iroh-net | CONFIRMED | Bones engine; "iroh-net" is the dated crate name (folded into `iroh`) |
| Production: distributed AI training + PoS | CONFIRMED | Nous Research (LLM training); Paycode (POS) |
| EndpointId public-key → QUIC session migration Wi-Fi↔cellular | CONFIRMED | QUIC-multipath; type renamed NodeId→EndpointId (0.94) |
| iroh relays were "DERP", now "relays" | CONFIRMED | DERP from Tailscale → relays |
| **transcript Rust code** (`connect_to_peer`, `iroh::docs::Doc`, `presets::N0` w/ `.idle_timeout`) | **PARTLY/REFUTED** | Real 1.0 API is `endpoint.connect(addr,&[u8])` — **no `connect_to_peer`**; it's `iroh_docs::api::Doc` (separate crate), not `iroh::docs::Doc`. `presets::N0` is real. Do **not** copy snippets as working code |

## Cluster 3 — Rust clients & UI frameworks (cleanest cluster — 12/16 fully CONFIRMED)

| Claim | Verdict | Note (src) |
|---|---|---|
| ATrium (atrium-rs) Rust AT-Proto framework | CONFIRMED | github.com/sugyan/atrium |
| atrium-lex + atrium-codegen (lexicon→Rust) | CONFIRMED | both real workspace crates |
| bsky-sdk in ATrium | CONFIRMED | workspace member |
| bsky_tui (Ratatui+Tokio+atrium) | CONFIRMED | github.com/ksk001100/bsky_tui |
| Ratatui = premier Rust TUI lib | PARTLY | leading, but "premier" is subjective |
| Tauri uses native OS WebView | CONFIRMED | tauri.app |
| Electron ~150MB+ vs Tauri ~3-10MB | CONFIRMED | in range |
| Tauri idle ~40-80MB RAM | PARTLY | more like **~30-50MB**; ceiling overstated |
| Tauri capability/allowlist + IPC | CONFIRMED | v2 ACL model |
| Slint formerly SixtyFPS | CONFIRMED | slint.dev |
| Slint .slint→native, Skia/OpenGL, <300KB | CONFIRMED | docs.slint.dev |
| Slint live-preview tooling | CONFIRMED | VS Code ext + LSP |
| Holepunch = Hypercore framework, Keet's makers | CONFIRMED | docs.holepunch.to |
| Hyperswarm DHT + hole-punching | CONFIRMED | github.com/holepunchto/hyperswarm |
| UniFFI = Mozilla cross-lang Rust bindings | CONFIRMED | github.com/mozilla/uniffi-rs |
| Keet = P2P video/chat (Holepunch/Tether) | CONFIRMED | serverless P2P |

## Cluster 4 — iOS background, P2P reference apps, history

| Claim | Verdict | Note (src) |
|---|---|---|
| SLC wakes app on movement/tower change | CONFIRMED | Apple CoreLocation docs |
| Geofence wakes app ~10s on crossing | PARTLY | ~10s right; understates ~20s dwell threshold; max 20 regions |
| CoreMotion walking/driving; bg-location API; `sharesLocationUpdatesWhenBackgrounded` | PARTLY | CoreMotion ✓; **property name invented** — real: `allowsBackgroundLocationUpdates` / `showsBackgroundLocationIndicator` |
| BGAppRefreshTask ~30s predictive | CONFIRMED | Apple BackgroundTasks |
| BGProcessingTask longer, charging+wifi+idle | CONFIRMED | Apple docs |
| **CoreBluetooth State Restoration relaunches on new advertiser UUID** | **REFUTED** | Relaunches only for **established/pending connections & subscriptions**, NOT on discovering a new advertiser. Weakens the "two locked phones passing auto-wake" scavenger story |
| Silent push + NSE ~30s window | PARTLY | conflates silent-push (content-available) with NSE (mutable-content on visible alert); both ~30s; silent pushes rate-limited |
| Extension/bg memory cap ~30-50MB | PARTLY | NSE cap is **24MB** (iOS 14+); overstated |
| NSFileProtectionCompleteUntilFirstUserAuthentication | CONFIRMED | bg r/w after first post-boot unlock |
| Delta Chat: IMAP/SMTP, deltachat-core-rust, Swift iOS | CONFIRMED | now `chatmail/core` |
| Delta Chat iOS NSE + silent push | PARTLY | push→bg-fetch confirmed; specific **NSE** detail unconfirmed |
| Berty: IPFS/libp2p, Go core, React Native | CONFIRMED | Wesh Protocol |
| Berty Multipeer + BLE; locked phones wake & sync | PARTLY | frameworks ✓, but Berty's **own blog says OS kills the node within seconds of backgrounding** — reliable wake-and-sync-while-locked is an *unsolved constraint*, not a feature |
| GeoCities 1994 "Beverly Hills Internet"; Bohnett & Rezner | CONFIRMED | both founder names correct |
| Themed Neighborhoods + Suburbs + numeric addresses | PARTLY | structure + SiliconValley/Hollywood ✓; the other neighborhood names real but not each individually re-confirmed |
| Yahoo shut US GeoCities 2009; Archive Team | CONFIRMED | closed 2009-10-26; ~641GB torrent 2010 |
| BitTorrent tit-for-tat; Gnutella free-riders | CONFIRMED | (optimistic-unchoking nuance) |
| Bitcoin double-spend via ledger+PoW, Sybil-resistant | CONFIRMED | |
| Early Skype from Kazaa team, supernodes | CONFIRMED | Zennström & Friis |
| Syncthing OSS P2P, TLS, local+global discovery | CONFIRMED | |

## Corrections that matter for Croft design

- **iroh-docs ≠ MST.** It uses **range-based set reconciliation** (the same family as Willow);
  AT Proto uses MSTs. Don't describe iroh-docs as MST-based in our docs.
- **No native AT-Proto E2EE/groups.** Private groups on AT Proto today are third-party
  (**Germ/MLS**, XMTP). This is *directly* relevant: Croft's own lineage-groups proof is the
  MLS-on-our-terms answer to exactly this gap. (Link the two in COHESION.)
- **The mobile "scavenger mesh" is shakier than the transcript implies.** CoreBluetooth
  restoration does **not** relaunch on discovering a new advertiser, and Berty's own engineering
  says background P2P is killed within seconds. The opportunistic-wake model is real (SLC,
  BGAppRefresh, BGProcessing, established-connection BLE restoration), but "two locked strangers'
  phones auto-wake and sync" should be treated as **aspirational/unproven**, not a given.
- **Real, usable primitives for us:** custom transports (`unstable-custom-transports`, iroh
  0.97+), first-party Swift bindings (`iroh-ffi`, iroh 1.0), iroh-gossip (HyParView/Plumtree),
  iroh-blobs streaming + QUIC-multipath migration (the music-over-iroh idea is sound), Tor/Nym
  community transports. BLE transport exists but is community + unencrypted-demo grade.
- **Crate-name hygiene:** `iroh-webrtc-transport` likely doesn't exist; Tor/Nym are
  `n0-computer/iroh-tor` / `iroh-nym`; BLE is `mcginty/iroh-ble-transport`.

## Provenance

Internal Iroh ground truth: `experiments/iroh/relay-lab-runs/IROH-1.0.0-API-VERIFIED.md` (iroh
pinned `=1.0.0`), `experiments/iroh/crates/altdrive-spike-{irohdocs,gossip,iroh}` (iroh-docs
0.100 / iroh-gossip 0.100 / iroh-blobs 0.102, LWW noted). Web verification: 2026-06-22 via four
parallel research passes; source URLs in the cluster tables above.
