# Raw: Croft landscape — small/medium/big-world positioning · tree-as-relational-UI · did:webvh watchers · blob-dedup↔E2EE · co-op service menu · clients · auth/MFA · Lightning/Nostr · responsive filters · Rust — design dialogue (2026-07-27)

**Preservation status: preserved-condensed (cleaned-paste, content-faithful — NOT byte-pristine) —
PLAYBOOK §4.** Source: a long claude.ai session pasted 2026-07-27. The paste **triplicated its opening third**
(the small-world monologue + the OPFS/continuous-export re-verification + the tree-as-UI research were each
pasted three times) — **reproduced once here**, per §4. Some in-line research-report bodies were long;
condensed content-faithfully with the load-bearing claims + numbers retained. No secrets.

**FACT-CHECK.** Load-bearing claims verified in `../../../research/2026-07-27-social-tree-factcheck-2.md`
(companion to the first Social-Tree fact-check). **Cite it, do not re-verify.** Headline: unusually accurate;
key corrections — iOS-18 hotspot client-IPv4-isolation is forum-only/UNVERIFIED; Riseup pads = 60 days (not
~30); DBSC is *shipped* now (Chrome 146 GA Windows, Apr 2026), not forward-looking; the TOTP resync-window
figures are the validator's choice not RFC-mandated; `popover` is Baseline *Newly* (Jan 2025) not *Widely*;
Path's "50" is inspired-by-Dunbar (150) not literally Dunbar's number; MIMI's protocol is a WG draft not an
RFC. atproto/iroh/iOS baseline facts defer to the FACTCHECK SoT.

**Session pause-note (author's own, top of paste):** "the tree is your actual data structure rather than a
metaphor; heartwood lands on the did:webvh lineage; the tree-as-UI has forty years of prior art with a known
adoption problem; the compliance path turned out to be relying-party rather than issuer, with a browser-native
API that happens to fit a PWA. Threads to pull next: whether Safari exposes the Digital Credentials API to
web content (the iOS story depends on it); and MIMI's current state (it sits directly on the stack)."

---

## Thread 1 — Positioning: small / medium / big world (the spine)

The user is working out how to explain Croft. **Bluesky/atproto = "big world"** by its own declaration:
built for global scale, so it trades toward *easily-consumable-public* data — "these are the things I want
publicly consumable about me." Even permissioned/scoped data on atproto is **scoped *open* sharing** (open =
"some, not everyone"), never confidential *from the operator/AppView domain*. **ActivityPub = "medium world",
a different trust model**: the locus of trust is the *instance operator* — you hang off their reputation and
operational reality, and all trust to the outside world is brokered at that layer. **There is no cohesive
"small world" option**, especially not one that *folds up into* the others. Croft = the small-world case:
**two people is the base case, working up.** The fractured status quo (iMessage/Google Messages don't fit;
Facebook does both roles poorly, locked-in and extractive). The user wants **non-pejorative framings for
"small world"** ("small" reads as lesser but it's just *different* — like the **life-world vs system-world**
distinction, not a 0-to-1 binary; different forms/scopes of social interaction mirroring real life). Open
question: is there a novel, useful place for Croft on this continuum? (Working answer: yes.)

## Thread 2 — OPFS + File System Access "continuous export": the honest correction

Re-examines the transcript-1 (E62) "continuous export" claim and **corrects it precisely**: "the parts are
real, the wiring is not, and the failure lands on the platform you care about." OPFS is real + Baseline
(widely available since ~Mar 2023, sync access handles in Web Workers, Safari has it, SQLite-in-a-worker
legit) — **but OPFS is not durability**: not user-visible, subject to the same eviction/quota regime as
IndexedDB, deleted on clear-site-storage → it buys *speed, not ownership, not persistence*.
**`showDirectoryPicker()` is desktop-Chromium-only** (Chrome/Edge/Opera 86+); Firefox + Safari ship only
OPFS, no disk pickers; Chrome-for-Android = OPFS only; global picker usage ~27% → **the continuous-export-to-
your-own-folder story does not exist on iOS or Android at all.** The Service-Worker step is **wrong twice**:
pickers require a user gesture (SWs have none) and background wake needs Background/Periodic Sync
(Chromium-only, absent on iOS). The "72h-export-is-a-dark-pattern-for-ad-indexing" close is **editorial, not
analysis** — unsupported; export queues are usually genuine batch jobs also serving GDPR Art. 20; flagged as
credibility-costing. **Usable resolution:** use OPFS for local perf (incl. iOS) but don't sell it as
ownership; ship folder-mirrored continuous export **on desktop** where it genuinely works (a real
differentiator); keep a **separate honest mobile durability story** — second device, peers who can re-admit
you, explicit identity export — because the file-tree answer isn't available there.

## Thread 3 — Tree structure as the substrate for a social relational UI/UX (research + design criteria)

User frame: the tree hasn't taken off, but that's *good prior art* not a deal-breaker; this is the canonical
use case; the point is **legibility to the user, not metaphor for its own sake**.

**The reframing finding (from the failure literature):** the 1995 hyperbolic-tree browser (Lamping, Rao &
*Pirolli*, Xerox PARC, CHI'95) found *no significant performance difference* for finding nodes; the follow-up
(Pirolli, Card & Van Der Wege, **ACM TOCHI 2003**, "effects of information scent on visual search in the
hyperbolic tree browser") found the **conditional**: on **high information-scent** tasks the tree was faster;
on **low-scent** tasks the plain file browser won. Every historical test used the *worst-scent* labels (file
paths, URLs, org charts). **Croft's labels are faces and names — the highest-scent material the human visual
system processes.** The condition under which the tree always failed is structurally absent here; the
condition under which it wins is permanent.

Other empirical gifts: a **2024 ego-network study** ("Me! Me! Me! Me!", *Computers & Graphics* 2024) — no
notable perf difference among 3 node-link layouts, adjacency matrices rated hardest, **layered node-link most
preferable** (rings, not blobs, not a matrix). **Path** (2010; 50-friend cap *inspired by* Dunbar, raised to
150 in v2.0, users settled ~50, sharing went up; Morin: "the home inside Facebook's city" — the thesis
verbatim, 14 years early). Personal-CRM rule: "if it expects 2,000 contacts on day one you'll never use it;
the good ones let you start with ten."

**10 design criteria:** (1) **No node without a face** (scent is the mechanism; text-only node = you lose to
a list; identicons floor, real avatars goal). (2) **The tree is a launcher, not an analysis tool** (wins
"find & open"; loses the moment you ask users to compare/count/audit). (3) **Layered & quantized, never
force-directed** (discrete concentric rings; organic physics = the Obsidian-graph failure mode). (4) **Ring
one = 5–9 nodes, full stop** (never render 150; expansion outward explicit/on-demand). (5) **Distance is
derived, never declared** (position from behavior — recency/frequency/group-age/size; no user-assigned tiers;
kills the Circles problem + the intimacy-leaderboard hazard). (6) **Re-rooting is the core interaction, needs
a home key** (tap a person → re-center; one gesture back to you — the part users genuinely preferred). (7)
**Pads attach as leaves — the tree's justification** (a list can't show *this game belongs to this group*; if
attachment isn't visible the tree is decoration). (8) **Shared people = one node, two edges** (Multitrees,
Furnas & Zacks CHI'94; duplicated nodes destroy recognition→scent). (9) **Search is always a bypass** (never
the only path; covers the low-scent case). (10) **Full-parity list view** (a11y/small-screens/opt-out; if the
list is fully capable the tree is a choice not a tax — the same "better-not-privileged" rule as clients).
**Prototype ideas:** recency-as-a-physical-force (new activity brightens & pulls a group inward, quiet groups
drift out — makes "distance is derived" legible without explaining it); pinch = quantized depth/ring control
(not free zoom → can't get lost); long-press a group to fan its pads out as leaves. **Sequencing:** large
custom-UI investment; build order says prove one pond before three → **list + search + great avatars ships
first and tests the actual thesis; the tree earns phase two** once there's real structure to render.

## Thread 4 — did:webvh: versionTime, watchers, and federation-of-availability

User's stance (verified against did:webvh v1.0 spec): **under-verify, not misverify.** Verdict: `versionTime`
is "as asserted by the DID Controller" (spec concedes the epistemics) — but resolvers DO constrain it: every
entry's versionTime **MUST be strictly greater** than its predecessor's (equal rejected), and **no entry may
be more than ~5 min** in the resolver-clock future (else resolution fails); it also drives the `?versionTime=`
query. **But those checks establish self-consistency, not truth** — a fabricated log can be uniformly
backdated and pass. So **timestamps are load-bearing for *validity* and the version index, never for
provenance**; authenticity rests entirely on **SCID + entry hashes + data-integrity proofs**. Treat
versionTime as an index, never as evidence.

**Impossibility:** "nothing newer exists" is never provable non-interactively (absence of a newer head is
only visible to an observer who's seen one). Best achievable = **bounded staleness** (a standing heartbeat:
the controller periodically re-signs the current head). Safety/liveness split: signatures + hash chains make
**integrity unconditional** (no hostile channel makes you accept a false state) → all residual error is
**omission** (true-but-old). Sharp edge: **staleness of authority is exploitable even when every fact served
is true** — an old head containing a since-rotated key verifies perfectly and is exactly what a key-thief
wants you to see → remedy = a **per-use staleness budget** (casual resolution tolerates hours; credential
acceptance tolerates one heartbeat).

**Settlement:** you need *some agreed lookup authority* canonical for "latest" (directory / DNS / DHT); since
you can't prove latest, you settle for "latest here" — **never wrong, sometimes stale.** This is in the spec:
**watchers** = URLs published in the DID's own log params, surfaced in resolution metadata, usable as the DID-
data source in place of the HTTPS location, caching verified state, **persisting it even after the controller
deletes the log**, and able to detect a controller republishing altered history. (Fact-check nuance:
"watchers reaching consensus" is the author's gloss — the spec frames watchers as distributed caches / a
malicious-controller-detection purpose, distinct from **witnesses** = approve-before-publish.)

**The spiral (federation readmitted on better terms):** split federation's two jobs — **fidelity** (is this
true) vs **availability** (can I get it / is it current). Classic ActivityPub federation *bundles* both →
every replica is a fidelity risk, adding a node = extending trust. Self-certification **unbundles** them:
once the log verifies against nothing but its own name, fidelity leaves the network and lives in the artifact;
what returns as "federation" (watchers, mirrors, relays, PLC replicas, pinned providers) is **federation of
availability only** — defanged (can't forge/tamper/impersonate; the entire remaining attack surface is
omission = eclipse/stale). Economics flip: trust-federation needs vetting/governance/reputation per node;
**availability-federation needs only a disk.** This is atproto's wager vs ActivityPub in one line: *move
fidelity into signed Merkle artifacts so the federated layer only ever answers "where" and "how fresh," never
"whether true."* **The DHT axis:** watchers/relays/PLC/DHT are all one axis — *who answers the availability
question* — differing only by membership + discovery, since fidelity is already answered by the artifact.
`did:web` = degenerate (one origin); watchers = named social set (in the log); PLC = single global
institutional answerer; a true DHT (pkarr / `did:dht`) = open membership + algorithmic placement (max
censorship-resistance, weakest freshness). Same fidelity everywhere; the trade is availability politics.
The one inescapable pinch: **bootstrap** — even a DHT needs a first node to ask, so discovery keeps a stubborn
bit of convention no cryptography dissolves (it just pushes one layer down).

## Thread 5 — Auth-read PDS + Bluesky blob storage → the E2EE storage contract

User: "What if I want a PDS that requires auth to read, in Rust, self-hosted — does it exist?" (framing
question; the substance is the blob thread.)

Surprise that Bluesky PDS store actual blob content (precludes storage-layer dedup at scale). Assistant:
dedup isn't *precluded* by the design, it was **deliberately traded away** in the reference impl. Per Newbold
(atproto discussion #1756): they moved from PDS-global CID-keyed blobs to **repo-local (per-DID)** blobs,
mainly for **lifecycle management** (deleting referencing records / whole accounts) and **per-context
moderation** (a blob subject to takedown in one post's context but fine in another → per-repo moderation state
is desirable). The spec: blobs are universally CID-content-addressed but **always referenced/managed per
account (DID)** → the `(DID, CID)` tuple everywhere in the sync API. Empirical kicker: **very little actual
dedup observed** even with meme reuse; backend *could* dedupe with a secondary index if worth it. (Fact-check:
the "comes for free with a CID-indexed store" phrasing was DavidBuchanan314's, the by-CID proposer, not
Newbold's; the "uploadBlob by CID" shortcut was **not adopted**, over the exact abuse concern below.)

**Dropbox/Dropship (explained):** early Dropbox cross-account dedup **trusted the client's hash claim**;
Dropship (Wladimir van der Laan, **April 2011**) weaponized it — spoke the sync API, "pretended" to have a
file, and since the only proof required was the block hashes (4 MB blocks, SHA-256), the server attached the
real content to your account → arbitrary anonymous transfer by hash-list (piracy with Dropbox as CDN). Plus a
**side channel**: instant upload reveals a file already exists in *someone's* account (Harnik et al. 2010 —
cross-user dedup as side/covert channel; Halevi et al. 2011 — "proofs of ownership" as the fix). Dropbox
killed the trust-the-client-hash path (+ legal takedowns); whether it kept purely-server-side dedup after is
**not well documented** (Newbold's "ripped it out" is his memory, not a Dropbox source). **Relevance:** a PDS
never trusts a client hash (`uploadBlob` takes the bytes) → Dropship-style injection doesn't apply.

**The insight (client-side E2EE):** cross-account dedup and client-side E2EE are **mutually exclusive** unless
you cheat with **convergent / message-locked encryption** (key derived from a hash of the plaintext →
identical ciphertext across users), and that determinism is the flaw (a plaintext-holder can confirm you have
a file; low-entropy fields brute-forced) — safe dedup only survives *within one user's key or a shared group
key* (Douceur et al.; Bellare et al. MLE). So by **scoping blobs per-DID + taking full bytes on upload**, the
PDS "accidentally-on-purpose" landed on the **storage contract E2EE needs**: blobs are opaque per-account
byte strings, the CID hashes whatever arrived (ciphertext hashes fine), **no cross-account equality oracle or
instant-upload timing channel**. Newbold's own aside: "we'll probably need to do something different anyways
for private/group-private data." Two seams: (a) layer separation — the storage-layer property doesn't make
atproto blobs private today (`getBlob` is unauthenticated public sync; the PDS still sees ciphertext sizes,
upload timing, the public record graph); (b) the same opacity removes moderation affordances (content
matching, contextual takedowns).

**User's correction (the durable framing):** the two rationales don't pull against each other — they **define
differing data contracts for different material in one storage engine.** Assistant concedes and relocates the
tension: the spec deliberately makes the engine **contract-agnostic** (restrictions can't be enforced at
upload — different lexicons need different blob types → blobs land in temp storage, validated only at *record
reference time* per the referencing record's lexicon). So the **contract attaches at the reference layer, not
the byte layer**: an encrypted blob under a future private-data lexicon and a public JPEG under
`app.bsky.feed.post` are just two `(DID,CID)` entries, each governed by its referencing lexicon; per-repo
moderation state is already contract-machinery of this kind. **Relocated tension:** not between contracts, not
in the engine — it lives *inside the encrypted contract at the operator layer* (a PDS hosting opaque bytes has
fewer response options: no content-match, no contextual takedown — only delete-by-CID / rate-limits / account
action; a known E2EE-hosting cost, not a design incoherence). The one real engineering requirement the framing
surfaces = **dispatch**: the engine can stay ignorant, but moderation/lifecycle tooling needs a reliable way
to know which contract a blob is under → reference-time lexicon validation + per-repo state is atproto's
existing answer. Status caveat: the private-data side is **prospective** (all atproto blobs today are the
public contract).

## Thread 6 — Co-op services: low-maintenance, high-utility menu

Question: "Could I make a Google Reader PWA/SPA clone?" → generalizes to **what a Croft-style co-op should
host**, ranked by the user's "as little durable state as possible" stand. Precedents: **Disroot** (email,
Nextcloud, XMPP, Etherpad, PrivateBin, SearXNG, Forgejo), **Riseup** (email, lists, VPN, chat; ephemeral
**share ~12h TTL**, **pads removed after 60 days** inactivity), **CHATONS** (French ethical-hoster federation,
initiated by **Framasoft, 2016**), **CryptPad** (browser-E2EE, admins can't read).

**The maintenance filter (the sharp part):** ops labor comes from four places — an abuse/moderation desk
(anything members can publish to strangers), reputation battles (anything touching email/exit traffic),
support tickets (per-user config), release churn. Dodge all four → set-and-forget. **Shortlist (best ratio
first):** password-vault sync (Vaultwarden — server holds only client-encrypted blobs; near-zero ops); feed-
reader backend (**Miniflux** — single static Go binary, only Postgres); CalDAV/CardDAV (last Google
dependency; tiny state); encrypted backup target (labor = quota mgmt); cron/uptime dead-man's-switch;
ephemeral encrypted file-drop + pastebin (self-cleaning by TTL = the maintenance strategy); push-notification
relay (ntfy/UnifiedPush). **The dividing line = charter material:** *server can't read it, members can't
publish to strangers.* **Restructuring idea:** one **generic encrypted sync service** — vault, estate prefs,
bookmarks, shopping lists, feed read-state, the `account.croft.ing` kernel are all the *same* per-member
encrypted KV store with small blobs → build once, every estate app rides it, "server holds only ciphertext"
enforced in one place. Second tier: stateless caching proxies (CORS feed-fetcher generalizes to link-unfurl/
oEmbed, image-resize, geocode/weather cache — member-authed so no open-proxy abuse desk); a **timestamp
notary** (RFC 3161 / OpenTimestamps-style — a pure trust good, zero per-user state); member DNS/subdomain
grants (doubles as the p2p rendezvous layer); public artifact pinning (makes the survivability clause
tangible); newsletter-to-feed bridge (highest-maintenance — inbound email = some spam). **Honest boundaries
(labor sinks):** full email mailboxes, photo libraries, Nextcloud-scale file sync, Matrix homeservers — heavy
authoritative state, where volunteer collectives burn out. **SMTP relay + VPN fail the filter** (pooled
reputation / exit-traffic abuse desk = perpetual labor → "fund a role," not a set-and-forget utility). The
flagship hiding in plain sight: **the static SPA catalog itself** — cheapest thing to run; the differentiator
isn't the hosting, it's the **LTS promise** only an institution can credibly make (the 10-year-domain
reasoning institutionalized).

## Thread 7 — Best Bluesky clients (→ borrow PWA patterns for Skylite)

Landscape (directory/corroboration-tier): **web/desktop** — deck.blue (TweetDeck-style multi-column, dev
since Aug 2023), **TOKIMEKI** (browser, multi-column + multi-account, **PWA-installable, MIT, self-hostable**),
**SkyFeed** (custom feed builder), Ouranos (lightweight friendly web), DarkSky (Windows-native OSS);
**mobile** — Skeets (iOS, paid Pro = bookmarks/drafts/thread-unroll), Graysky (maintenance status unconfirmed),
Sora (multi-network Mastodon+Bluesky). **User note:** "we may want to heavily borrow from the good PWA ones
for **skylite**" → TOKIMEKI + SkyFeed are the closest comparables to a PWA web client, both open source →
read them for AppView-interaction patterns.

## Thread 8 — Auth: HSM alternatives, passkey↔DPoP, DBSC, flat-cost group MFA + mass invalidation

**Passkey can't be the DPoP key** (format incompatibility, not engineering): a DPoP proof is a **JWS** signing
arbitrary chosen bytes; a WebAuthn assertion signs a **fixed envelope** = `authenticatorData ‖
SHA-256(clientDataJSON)` (the authenticator wraps your challenge in its own `clientDataJSON` and signs its own
structure) → no WebAuthn op emits a JWS over content you chose; and DPoP signs *per request* while passkeys
need a *per-operation gesture*. **The legs:** DPoP binds **broker↔PDS** (the "device" the PDS sees is the
broker, not the phone); a passkey would bind **user↔broker** (a leg the PDS never sees); **DBSC** binds a web
**session↔device** (user↔broker leg). For hardware DPoP-key binding: native apps can put the DPoP key in
Secure Enclave/Keystore (no per-use gesture) — platform key storage, not a passkey; the web has no API to
force the DPoP key into the TPM. **DBSC** (Device Bound Session Credentials): TPM/Secure-Enclave non-
exportable key signing server challenges (~5 min, *implementation-defined not spec-mandated*) with no user
interaction, at the HTTP layer — **shipped: GA on Windows in Chrome 146 (Apr 2026), wider rollout May 2026,
macOS Secure Enclave next**. DBSC spec caveat (verbatim): won't prevent temporary session use while an
attacker is *resident* on the device; no guarantee about the specific device/state → same containment-after-
not-during ceiling as DPoP/broker.

**Mass invalidation = already built, two flat kill switches:** (1) PDS-layer, spec-mandated — pull a key from
your published **JWKS**; atproto OAuth binds each session to the client key used at start and **MUST revoke
the session + reject refreshes once that key leaves the JWKS** → since it's your `client_id`/JWKS, "everyone
on that key" = your whole userbase (rotate per-cohort/per-window to scope it); (2) your-layer — flush the
broker's in-memory session store. Having both = defense in depth (one works even if the broker is compromised,
the other even if you can't reach the PDS). **Flat-cost MFA = TOTP** (**RFC 6238**, built on HOTP **RFC 4226**,
HMAC-SHA1): verification is a **local HMAC** — no gateway, no per-message fee, no per-MAU license → zero
marginal cost at any scale. (SMS = per-message; hosted MFA = per-MAU; passkeys = also zero-marginal + phishing-
resistant but harder enroll/recovery — the strongest flat option.) Correctness (all local): bounded resync
window (validator's choice — small, e.g. ±1 step), single-use (track last consumed step), rate-limit. **The
catch that ties back:** a TOTP secret is long-lived, never rotates, must persist → it **reintroduces the at-
rest secret problem** the ephemeral-session design eliminated; if it sits in the SQLite that replicates to R2,
the **Litestream-cleartext finding** (from the measurement kit) means a bucket leak = every second factor,
permanently. So the TOTP-secret table is the one thing that genuinely earns **envelope encryption** — hold it
like the client-assertion key: one encrypted-at-rest blob, loaded into broker memory at boot, decrypted only
to run the HMAC, never written back cleartext. Money cost stays flat; the real cost is operational (QR-scan
enrollment + a recovery path).

## Thread 9 — Lightning / Nostr zaps for small-scale community value transfer

**Zaps = a receipt protocol on top of Lightning** (NIP-57): the payment is a normal Lightning payment between
wallets; Nostr only (a) advertises where to pay and (b) publishes a signed attributable record. Primitives:
kind 0 profile metadata carries `lud16` (Lightning address); **kind 9734 = "zap request"** (signed, **NOT**
published to relays — sent to the recipient's LNURL callback); **kind 9735 = "zap receipt"** (published to
relays by the recipient's wallet server). Flow: resolve `lud16`→LNURL-pay URL (GET) → if response has
`allowsNostr:true` + a valid `nostrPubkey` (BIP-340), build the signed 9734 and POST to the callback →
server returns a **description-hash invoice** committing to the zap request (binds who/whom/for-what into the
payment) → pay with any Lightning wallet → on payment the LNURL server publishes the 9735 to the requested
relays; clients aggregate + validate (signed by the advertised `nostrPubkey`). **Trust seam:** a receipt is
an **attestation by the recipient's wallet provider, not cryptographic proof of payment** (a malicious/
compromised server could mint fakes or drop reals; the description-hash stops *third-party* forgery, but the
provider is trusted). **Privacy:** the announcement is a *separable layer* — skip the zap request and pay the
Lightning address directly and the money moves identically with **no public 9735** (spec: clients SHOULD
request a receipt when supported; the plain path still exists). The public receipt is the *product* (social
proof / spam-deterrence). Caveats: custodial wallets on each end see the payment; once a 9735 is published,
sender/recipient/amount/comment are on public relays permanently; encrypting zap requests to the target was
left out of the initial draft. **NWC = NIP-47** (a client instructs a remote wallet to pay).

**Two fresh phones (onboarding):** Path A all-in-one = **Primal** (generates a Nostr keypair; built-in wallet
via **Strike** custody, KYC-lite = name/email/DOB/country + email code; auto-publishes a `name@primal.net`
Lightning address; ~5 min; the price is **custody**). Path B separated = a client (**Damus** iOS / **Amethyst**
Android) + a separate Lightning wallet wired via **NWC**; custodial address instant (**Wallet of Satoshi**),
self-custodial address = real work. iOS wrinkle: Apple pressured **Damus (June 2023)** to remove post-level
zaps as selling digital content outside IAP (kept profile-level tips). Summary: A is Venmo-grade onboarding,
every convenience purchased by letting Strike hold the balance; A-first-then-graduate is the realistic pilot
sequence. Transfer-rail and announcement-rail are independent — each payment can be a quiet gift or a public
signal.

## Thread 10 — Responsive filter/search UI (arecipe) — the 261-source playbook

Question: how to balance one interface across mobile web + desktop when a filter toolbar has different real
estate (e.g. 3 side-by-side dropdowns — meals/cuisine/cook-time — at desktop vs no horizontal room on mobile);
is collapsing standard? **Answer: yes, per-breakpoint presentation is standard; the specific instinct is
mainstream — but do NOT merge distinct facets into one combined dropdown.** **Recommended pattern (one state
model, two presentations):** desktop = a sticky **horizontal toolbar of per-facet disclosure dropdowns**
(live-update, a count in each trigger, applied-filter chips + "Clear all" below); mobile = collapse to a
single **"Filters (n)"** button opening a **native `<dialog>.showModal()` bottom sheet** with the three facets
as labelled accordion sections + a sticky **"Show X results"** batch-apply button; applied chips in a scroll-
snap row above results. Evidence: **Baymard** (2025 benchmark: **58% desktop / 78% mobile** "poor-to-mediocre"
product-list UX; **28%** show no applied-filters overview, **72%** desktop do; truncate value lists >4–8;
per-option counts are the single highest-impact fix + a zero-results guard), **NN/g** (batch-filter on mobile,
tray over results, sticky accordion controls), **Material 3** (chips under search → side/bottom sheet),
**GOV.UK/MoJ `moj-filter`** (accessible applied-filters + clear links), **Polaris `Filters`** (promoted +
"More" sheet). **Behavior:** live desktop / batch mobile; per-option counts; applied chips removable (real
`<button>`, "Remove [facet]: [value]"); sort ≠ filter; **URL state** (`URLSearchParams` + History API —
`pushState` for filter changes, `replaceState` + ~300ms debounce for typing; restore on `popstate` + load).
**Baseline-safe native impl:** `<dialog>`/`showModal` (Baseline widely; free focus-trap/backdrop/Esc/inert);
`popover` attribute (Baseline **Newly** Jan 2025 — progressive-enhance) or ARIA disclosure (button
`aria-expanded`/`aria-controls` + checkbox group; Roselli: prefer this over `<select multiple>`); chip rows
via `scroll-snap`; **container queries** on the toolbar (Baseline widely Aug 2025) so it switches by its own
width, not the viewport. **A11y:** WCAG **2.5.8** target size (24×24 CSS px, AA), **2.5.1** pointer gestures
(swipe-to-dismiss must not be the only close); APG Dialog + Disclosure patterns; 44px touch targets. **Perf:**
**MiniSearch** (build index at build time, ship as static asset, cache in IndexedDB/SW; debounce ~300ms; move
to a Web Worker only if it janks). Rollout: ship the core pattern first; refine (cross-session memory, promoted
chips); Web-Worker only if the catalog grows. Thresholds that flip the recommendation: >5–7 facets or any
value list >8 options → desktop switches to a left sidebar + search-within-filter.

## Thread 11 — Rust single binary like Go (reference)

Yes — `cargo build` = one executable with all Rust crate deps statically linked; only the **C runtime**
differs (per-target, per rust-lang **RFC 1721 "C runtime static linking"** + the `-C target-feature=+crt-
static` knob): `x86_64-unknown-linux-gnu` links glibc **dynamically**, `x86_64-unknown-linux-musl` links musl
**statically** (the Go-style "scp anywhere" binary: `rustup target add …-musl` + `cargo build --target …-
musl`), windows-msvc links MSVCRT dynamically. Go's default isn't fully static either (cgo in `net` DNS +
`os/user`); `CGO_ENABLED=0` / `netgo`/`osusergo` → static. Gotchas: fully-static kills `dlopen`/glibc NSS;
musl's allocator is *reportedly* slower under multithreaded pressure (measure, don't assume). On glibc
stability: the ABI is stable **forward only** — old-built runs on newer via versioned symbols
(`printf@GLIBC_2.2.5`); **new-built fails on older** (`version 'GLIBC_2.38' not found`). So "it's a Linux box"
isn't the constraint — "its glibc ≥ my build machine's" is (CI drift; Alpine/scratch = musl, gnu-target won't
run; RHEL/Debian-oldstable lag). Cheap hedge: the musl target removes the whole question (why so much Rust CLI
tooling ships musl Linux builds).

---

## Relationship to the corpus (routing)
- **Positioning (T1)** + **tree-as-UI (T3)** → E63; feeds naming/positioning + `thinking/social-layer.md`,
  extends **E62** (the Social Tree client — the tree UI is the small-world made legible).
- **did:webvh / federation-of-availability / DHT axis (T4)** → E64; ties **E30/E31** (Drystone), heartwood,
  `research/atproto-sovereign-appview-club.md`.
- **blob-dedup ↔ E2EE storage contract (T5)** → E65; ties **E24/E42**, `research/atproto-private-data-
  architecture.md`.
- **co-op service menu (T6)** → E66; ties **D5** (cooperative/sustainability), E24 club, the account kernel.
- **Bluesky clients → Skylite (T7)** → E70; ties Skylite, ECOSYSTEM.
- **auth/MFA/mass-invalidation (T8)** → E68; ties the auth-helper spike (`spike/auth-helper`), account kernel
  (`plans/2026-07-22-account-kernel-spike.md`, K1/KC1), and the measurement-kit **Litestream-cleartext**
  finding.
- **Lightning/Nostr zaps (T9)** → E67 (candidate, Phase-2, custody/legal-gated); ties D5.
- **responsive filters (T10)** → E69; ties the arecipe cluster (E53–E59).
- **OPFS/continuous-export correction (T2)** → updates **E62** (honest desktop-only + separate mobile
  durability story).
- **Rust single-binary (T11)** → reference only (no dedicated item); lives here + the fact-check doc.
- MIMI + DBSC + Lightning/Nostr/co-op/client orgs → ECOSYSTEM §5j.
