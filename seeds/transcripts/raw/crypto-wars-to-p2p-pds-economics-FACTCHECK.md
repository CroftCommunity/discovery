# Fact-check — Crypto-wars → mobile-P2P → PDS-economics dialogue (Gemini)

date: 2026-06-22 · companion to `crypto-wars-to-p2p-pds-economics-dialogue-2026-06-22.md`

purpose: the raw dialogue is AI-generated (Gemini, flagged by the user as sometimes unreliable).
Per the user's standing request, every substantive assertion was fact-checked. Method: parallel
web verification (court dockets/CourtListener, SEC.gov/FINRA.org, EFF, Wikipedia, Stanford
Magazine, National Academies, philzimmermann.com, project repos/docs, RFC editor, Apple Developer,
vendor sites) plus the project's own iroh/atproto source of truth for those facts. Verdicts:
**CONFIRMED** (exists & as-described) · **PARTLY** (real but mis-described) · **REFUTED**
(false / no such thing) · **UNVERIFIABLE** (no credible source for the *exact* claim, usually a
quote). Cluster B (Ma Bell + surveillance-capitalism quotes) was verified from model knowledge
plus two targeted live checks (Bazelon, Zuboff); its remaining quote verdicts are flagged
accordingly.

## Headline

**Unusually accurate for a Gemini transcript** — markedly better than the companion atproto
dialogue. The hard skeleton across all three bodies (1970s/90s crypto-wars history, the Ma
Bell→Apple antitrust parallel, the current Proton/Apple/AltStore facts, the P2P-protocol
landscape, and the enterprise-compliance enforcement record) is **overwhelmingly CONFIRMED**.
Several claims that *smelled* fabricated checked out on inspection — call these out so they aren't
wrongly discarded: the Proton case number **4:25-cv-05450**, Judge **Araceli Martínez-Olguín**,
the **China 30%→25%** App-Store cut, the **Andy Yen** donation quote, **Peat by Defense
Unicorns** (a real Rust + Iroh + Automerge-CRDT + MLS stack), and the **Deloitte $200k / Velox
$1.8M** off-channel fines.

The failure mode is narrow and concentrated:
1. **Fabricated or unverifiable QUOTES attributed to historical figures** — the signature Gemini
   tell. Treat every direct quotation in the history section as suspect.
2. **Two invented/over-stated technical labels** — "Voskop" (not a Matrix term) and Pear-as-the-
   "browser-replacing core runtime" (Pear is a dev platform on the **Bare** runtime; Keet's
   transport is **Hypercore**).
3. **Staleness** on a couple of fast-moving legal items (the Epic appeal was largely decided
   Dec 2025; the EU CTF→CTC fee is sequential, not two names for one thing).

## REFUTED / fabrications — do NOT carry forward as fact

1. **Zimmermann's "Stalin" quote.** "If we do nothing, new technologies will give the government
   new automatic surveillance capabilities that Stalin could never have dreamed of…" is **not** in
   his Oct 12 1993 House testimony. The real testimony makes the same point in different words
   ("This can be done easily, routinely, automatically, and undetectably on a grand scale").
   (src: irp.fas.org/congress/1993_hr/931012_zimmerman.htm) — **classic Gemini hallucinated quote.**
2. **"Voskop" as a Matrix group-crypto protocol.** No such term. Matrix group crypto is **Megolm**
   (1:1 is Olm); the Rust crypto library is **Vodozemac**. "Voskop" is invented.
   (src: matrix.org E2EE docs; github.com/matrix-org/vodozemac)
3. **The Meyer-letter exact quote** ("These modern weapons technologies, uncontrollably
   disseminated, could have more than academic effect…"). The letter is **real** (J.A. Meyer,
   an NSA employee, to the IEEE) but it was **July 1977** (the dialogue says August), and this
   exact wording is **not** in the FOIA-released material. Treat as UNVERIFIABLE quote on a real
   event. (src: cryptome.org NSA-Meyer FOIA; National Academies App. E)

## Quotes that are UNVERIFIABLE or PARTLY (real source, wording not confirmed)

| Quote | Verdict | Note (src) |
|---|---|---|
| Keane case-closing letter, Jan 11 1996, verbatim text | UNVERIFIABLE | Case **was** closed Jan 1996 by AUSA William Keane (CONFIRMED); the exact quoted paragraph isn't sourced (philzimmermann.com; IEEE *Cipher* 1996) |
| Zimmermann "…they did not indict me, because we beat them" | UNVERIFIABLE | No match in quote databases; plausible paraphrase |
| Zuboff: "Surveillance capitalism is an assault on human autonomy…" | **PARTLY** | First clause is the **verbatim Guardian headline**, 4 Oct 2019; the trailing sentences are paraphrase-likely. "Economies of action" **is** her term (theguardian.com; Harvard Gazette 2019) |
| Acquisti/Brandimarte/Loewenstein, *Science* 2015 quote | UNVERIFIABLE wording | Paper is **real** ("Privacy and Human Behavior in the Age of Information," *Science* 348:6221, 2015); exact sentence not confirmed |
| Solove, "The Myth of the Privacy Paradox" (2021) quote | UNVERIFIABLE wording | Article is **real** (GW Law); exact sentence not confirmed |
| Ma Bell "catastrophic failure… general deterioration" FCC-argument quotes | UNVERIFIABLE wording | The *argument* is historically accurate; the exact phrasing isn't pinned |

## Cluster A — 1970s/90s crypto-wars history

| Claim | Verdict | Note (src) |
|---|---|---|
| PGP name ← "Ralph's Pretty Good Grocery" (Keillor, *A Prairie Home Companion*) + catchphrase | CONFIRMED | Lake Wobegon store (Wikipedia: PGP) |
| First cipher "BassOmatic" ← SNL (Aykroyd) blender sketch; later replaced by IDEA | CONFIRMED | (Wikipedia: BassOmatic) |
| PGP free June 1991; crypto = "munitions" under ITAR | CONFIRMED | (Wikipedia: PGP) |
| 3-yr criminal investigation 1993→Jan 1996 (US Customs / USAtty N.D. Cal); **no indictment** | CONFIRMED | Feb 1993 grand jury; closed Jan 1996, AUSA Keane (Wikipedia; IEEE *Cipher*) |
| MIT Press printed the full PGP C source as a ~600-page book (1995); OCR-friendly | CONFIRMED | "*PGP Source Code and Internals*," OCR instructions on cover (philzimmermann.com) |
| …with **per-page cryptographic checksums** | UNVERIFIABLE | OCR strategy confirmed; per-page-checksum detail not sourced |
| Snowden used PGP (2013); Zimmermann founded PGP Inc. early 1996 | CONFIRMED | (The Intercept; Wikipedia) |
| Motivation: 1991 **Senate Bill 266** backdoor ("sense of Congress") clause | CONFIRMED | S.266, 102nd Cong.; later dropped (Congress.gov) |
| "postcards vs envelopes" / "driftnet" surveillance analogy | CONFIRMED | "Why I Wrote PGP" (philzimmermann.com) |
| Bernstein (UC Berkeley), "Snuffle"; State Dept → register as arms dealer; *Bernstein v. US*; 1999 9th Cir: code = speech | PARTLY | Was a PhD **candidate** at design time; suit = *Bernstein v. US DOJ* (1995); 1999 panel 2-1 win later **vacated for en banc**, but govt relaxed rules first (EFF; 176 F.3d 1132) |
| Barlow: Dead lyricist + WY rancher, co-founded EFF 1990; "Declaration of the Independence of Cyberspace," Davos/WEF, 1996 | CONFIRMED (nuance) | Issued **Feb 8 1996** in **response to the Communications Decency Act** (Title V of the Telecom Act), not generically "after" it (Wikipedia; EFF) |
| Diffie–Hellman, "New Directions in Cryptography," 1976 | CONFIRMED | IEEE Trans. Info. Theory, Nov 1976 |
| Meyer letter: NSA employee warned IEEE re ITAR; RSA via Gardner's *Scientific American* Aug 1977 (Rivest/Shamir/Adleman, MIT) | CONFIRMED (event) | Letter **July** 1977 (quote unverifiable — see above) (Stanford Mag; Cryptome) |
| George Davida: UW-Milwaukee; NSA Secrecy Order on his crypto patent (Invention Secrecy Act 1951); rescinded after backlash | PARTLY | Patent filed Oct 1977; secrecy order **April 1978** (not 1977); rescinded June 1978 (Wikipedia: Davida; SCU Law Rev.) |
| 1978 "voice scrambler" patent also hit with an NSA secrecy order | PARTLY | Carl Nicolai's **"Phasorphone,"** 1978; "Seattle" not specifically sourced; order ~7 wks then rescinded |
| 1975: NSA pressed NSF to stop funding civilian crypto, claiming sole statutory authority | CONFIRMED | (National Academies App. E; Schneier 2014) |
| VAdm Bobby Ray Inman took NSA late 1977; shifted intimidation→engagement | CONFIRMED | (Wikipedia: Inman; National Academies) |

## Cluster B — Ma Bell → Apple antitrust parallel + surveillance capitalism

| Claim | Verdict | Note (src) |
|---|---|---|
| AT&T "foreign attachment" tariffs; leased phones; Western Electric = AT&T's equipment arm | CONFIRMED | standard telecom history |
| Hush-A-Phone (non-electrical mouthpiece cup); *Hush-A-Phone Corp. v. US*, D.C. Cir. **238 F.2d 266 (1956)**, reversed FCC/AT&T | CONFIRMED | (Justia; casemine) |
| **The "privately beneficial without being publicly detrimental" quote, written by Judge David L. Bazelon** | **CONFIRMED** | Bazelon authored the opinion; wording verified ("…without being publicly detrimental") (casemine 238 F.2d 266) |
| Carterfone (Thomas Carter, acoustic coupler); 1968 FCC decision opened the network → modems/internet | CONFIRMED | (FCC; comp-comms history) |
| AT&T broken up by 1982 consent decree (divestiture 1984) | CONFIRMED | *US v. AT&T* |
| Zuboff, *The Age of Surveillance Capitalism* (2019); "economies of action" | CONFIRMED | quote = PARTLY (see quote table) |
| Acquisti et al., *Science* 2015 — paper exists | CONFIRMED | quote wording UNVERIFIABLE |
| Solove, "Myth of the Privacy Paradox" (2021) — article exists | CONFIRMED | quote wording UNVERIFIABLE |

## Cluster C — Proton v. Apple, Apple fees, AltStore (web-verified, mid-2026)

| Claim | Verdict | Note (src) |
|---|---|---|
| *Proton AG v. Apple, Inc.*, **4:25-cv-05450**, N.D. Cal., filed ~Jun 30 2025 | **CONFIRMED** | real docket; originally 3:25- then reassigned to Oakland 4:25- (CourtListener 70671824) |
| Joined earlier Korean-developer suit (~May 23 2025) | CONFIRMED (nuance) | consolidated w/ *In re Apple App Developer Antitrust Litig.* / KPA, 4:25-cv-04438 (proton.me/blog/apple-lawsuit) |
| Judge **Araceli Martínez-Olguín** assigned | **CONFIRMED** | sitting N.D. Cal. judge (cand.uscourts.gov) |
| Andy Yen "donate to organizations fighting for democracy and human rights" quote | **CONFIRMED** | verbatim (Cohen Milstein) |
| Proton: Apple policies "favor the surveillance capitalism business model employed by companies like Meta and Google" | CONFIRMED | verbatim on Proton blog |
| DOJ antitrust suit; MTD denied ~2025 | CONFIRMED | **Judge Julien X. Neals, D.N.J.** (not N.D. Cal.), denied MTD Jun 30 2025 (Mintz) |
| Epic v. Apple: "willful contempt" Apr 2025; 27% backdoor; US link-out at 0%; "Apple appealing" | PARTLY (stale) | 9th Cir **Dec 11 2025** affirmed contempt but **vacated the 0% portion**, remanded for a "fair commission"; SCOTUS declined pause May 2026 (Perkins Coie; SCOTUSblog) |
| EU DMA: flat 30% → unbundled incl. "Core Technology Commission/Fee" | PARTLY | CTF (per-install €0.50) → **CTC 5%**, fully replaced Jan 1 2026; sequential, not synonyms (RevenueCat) |
| Small Business Program 15% (<$1M/yr); subs 30%→15% after yr 1 | CONFIRMED | (Apple Developer) |
| **China storefront 30%→25%; small-biz 15%→12%** | **CONFIRMED** | effective **Mar 15 2026** (developer.apple.com/news) — smelled fake, is real |
| Patreon forced onto Apple IAP | CONFIRMED | (MacRumors; TechCrunch) |
| AltStore PAL via Safari (EU); **also Japan** | CONFIRMED | Japan launch **Dec 2025**, iOS 26.2+ (9to5Mac) |
| AltStore Classic: AltServer desktop, free cert, 7-day expiry, Wi-Fi refresh, "Sources" repos, Delta | CONFIRMED | (AltStore FAQ) |
| No US DMA equivalent; **AICOA** is the relevant US bill | CONFIRMED | reintroduced Jun 11 2026 (Grassley/Klobuchar), 3rd attempt (Senate Judiciary) |

## Cluster D — P2P protocol landscape + iOS background mechanics (web-verified)

| Claim | Verdict | Note (src) |
|---|---|---|
| Delta Chat = email/IMAP + Autocrypt; iOS kills IMAP IDLE; **Chatmail** servers + notification **proxy** holding an Apple push cert | CONFIRMED | (support.delta.chat; chatmail docs) |
| iOS: no 3rd-party SMS read except a filter extension; iMessage non-replaceable; `MFMessageComposeViewController`; Android default-handler (`SMS_DELIVER_ACTION`) lets Textra/Pulse replace SMS | CONFIRMED | (Apple/Android docs) |
| Screen Time → Communication Limits → Contacts Only; bypass unless "Account Changes" locked | CONFIRMED | (Apple support) |
| iOS VPN = NetworkExtension/Packet Tunnel Provider (background, sandboxed to packet routing); VoIP since iOS 13 = PushKit + CallKit | CONFIRMED | (developer.apple.com) |
| NetworkExtension memory cap; "15MB–32MB" | PARTLY | ~**15MB** documented (content filter); 32MB not in docs (Apple Dev Forums) |
| MLS = **RFC 9420**; TreeKEM; mandatory Delivery Service for ordering; PFS+PCS; O(log n) | CONFIRMED | (datatracker RFC 9420) |
| Matrix: Olm (1:1, Double-Ratchet) + Megolm (group); Megolm trades some PCS via shared ratchet; **"Voskop"** | PARTLY / **"Voskop" REFUTED** | Olm/Megolm correct; "Voskop" invented (real Rust lib = Vodozemac) (matrix.org) |
| Briar: Tor + Wi-Fi + Bluetooth mesh, no servers; peers online to sync; append-only groups | CONFIRMED | Bramble protocol (briarproject.org) |
| Cwtch (Open Privacy Research Society), Tor onion; group chat uses an (untrusted) server/host node | CONFIRMED | (docs.openprivacy.ca) |
| Quiet: Slack-alt on IPFS/OrbitDB + Tor; needed a centralized component for iOS notifications | CONFIRMED | (github.com/TryQuiet) |
| SimpleX Chat: no user IDs/phone #s; unidirectional queues; relay servers (not pure P2P); QR setup | CONFIRMED | (simplex.chat/faq) |
| Keet (Holepunch); Tether-backed; El Salvador; Hypercore; **Pear** = Holepunch runtime | PARTLY | Tether/El Salvador/Hypercore CONFIRMED; **Pear is a P2P *dev platform* on the Bare runtime, not a "browser-replacing core runtime"; Keet's transport is Hypercore** (docs.pears.com; tether.io) |
| Berty: French non-profit; **Wesh** protocol on libp2p + IPFS; Go core, React Native UI; now maintenance/stasis | CONFIRMED | (berty.tech; weshnet) |
| **Peat by Defense Unicorns** — open-source Rust P2P data-sync; **Iroh** transport + **Automerge** CRDTs + **MLS**; denied/degraded/ATAK; `peat-gateway`→Okta/Keycloak | **CONFIRMED** | real, active repos; Defense Unicorns is a real defense-tech firm (github.com/defenseunicorns/peat, /peat-gateway) — **smelled fabricated, is real; strongest prior-art find** |
| Four-property impossibility (moderation + multi-device + PFS + offline-mesh need an unequal peer) | CONFIRMED (reasoning) | consistent w/ the protocol table above + Croft's own MLS findings; not a citable "theorem" but a sound synthesis |
| Secondary "dials": MLS/TreeKEM wire overhead, metadata-vs-scale (mixnets/Loopix), eviction-lag, log-pruning, Energy-Depletion attacks, Sybil (PoW/invite-chains), traffic-analysis/cover-traffic, DHT warm-up lag | CONFIRMED (all real concepts) | standard distributed-systems/secure-messaging tradeoffs |

## Cluster E — enterprise-compliance demand (web-verified — the section the user wanted "proof" on)

| Claim | Verdict | Note (src) |
|---|---|---|
| SEC Rule **17a-4** + FINRA Rule **4511**; WORM retention | CONFIRMED | (SEC.gov; FINRA.org) |
| FINRA Rule **2210**: principal pre-approval of retail comms | CONFIRMED | (FINRA Rule 2210) |
| **>$3.5B** combined SEC/CFTC(+FINRA) off-channel penalties, ~2021–2026 | CONFIRMED | JPMorgan $125M (2021) → 16 firms $1.8B (2022) → $549M (2023) → more FY24/25 (Alston & Bird; CFO Dive) |
| **Deloitte Corporate Finance $200k FINRA** fine; ~**676k** (sources: "over 650k") unarchived iMessages; iOS-update bypass (only **4 of 99** phones had iMessage disabled) | CONFIRMED | (Global Relay GRIP; FX News Group) |
| **Velox Clearing $1.8M** (**$1.3M FINRA + $500k SEC**, Jun 2025) over WeChat; >10k unretained msgs | CONFIRMED | (Steel-Eye; GRC Report) |
| **FINRA 2026 Annual Regulatory Oversight Report** flags off-channel comms | CONFIRMED | published **Dec 9 2025** (FINRA; Sidley) |
| Smarsh / Global Relay / Pagefreezer real; multi-channel WORM + surveillance | CONFIRMED | Smarsh = 2025 Gartner leader (vendor sites; Gartner) |
| Pricing ~$10–30/user/mo basic, $50–150+/user/mo surveillance tier | UNVERIFIABLE (plausible) | per-seat SaaS model is the confirmed norm; exact public pricing scarce |

## Cite-don't-reverify (project source of truth)

iroh / atproto / iOS-P2P mechanics: see
`seeds/transcripts/raw/atproto-atmospheric-web-iroh-mobile-FACTCHECK.md`. Anchors that touch this
dialogue: iroh `1.0.0`; iroh QUIC/relay/hole-punch model; iroh-gossip = HyParView/Plumtree; PDS =
public-by-default append-only repo; CAR export real. This dialogue does **not** re-assert the
Merkle-Search-Tree / "Keen" errors from the companion (it doesn't discuss iroh-docs internals).

## What to carry forward into the corpus

- **Prior art (ECOSYSTEM):** **Peat / Defense Unicorns** — a *production, CONFIRMED* stack that is
  essentially Croft's protocol bet (Rust + iroh + CRDT + MLS) proven in denied/degraded field
  conditions. Strong learn↔ / build-on signal. Also register/confirm SimpleX, Briar, Cwtch, Quiet,
  Keet/Holepunch-Pear, Berty/Wesh, Matrix(Olm/Megolm), Delta Chat/Chatmail as the secure-messaging
  comparison set.
- **Design grounding (thinking):** the **four-property impossibility** + the secondary-dials
  catalogue is a clean, accurate articulation of why Croft's lineage-groups accept an MLS Delivery
  Service / "unequal peer" — backs the existing protocol work and `ios-opportunistic-p2p.md`.
- **Sustainability body (ROADMAP_TODO / open-considerations):** the **PDS-hosting + P2P-blended
  business model** (consumer/creator/operator/**enterprise-compliance** tiers) is real demand —
  the compliance cluster is CONFIRMED. This feeds the top open problem (sustainability ↔ the
  cooperative *mechanism*); **surface as input, do not let the model's for-profit framing silently
  become Croft's answer** (the cooperative/non-extractive stance is the user's call).
- **Civic "why" (narrative/principles):** the Bazelon **"privately beneficial without being
  publicly detrimental"** standard is a precise legal ancestor of Croft's **"no right to remove
  the rights of others"** razor; the Carterfone/Ma-Bell→Apple arc and the crypto-wars lineage
  reinforce the design-imperative body.
