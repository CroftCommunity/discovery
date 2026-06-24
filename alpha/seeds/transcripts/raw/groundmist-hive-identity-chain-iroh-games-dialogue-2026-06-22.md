# Raw transcript — Groundmist · Steem/Hive · identity-chain (did:webvh↔did:plc) · iroh games + odds-and-ends (Gemini)

source: sprawling explainer dialogue, pasted 2026-06-22 · **cleaned-paste, content-faithful — NOT
a byte-pristine export** (PLAYBOOK §4). One long conversation covering: **(a)** **Groundmist**
(grjte / Ink & Switch — local-first × atproto); **(b)** Bluesky/atproto research datasets + key
figures; **(c)** the **Steem→Hive / Justin Sun** takeover saga, **Hard Fork 23**, **TRON**, **Hive**
deep-dive, **coops / PBC / DAO**; **(d)** a **did:webvh ↔ did:plc identity chain** design (bidirectional
`alsoKnownAs`, the #atproto subkey, and the validation that atproto can't resolve did:webvh); **(e)** a
long **odds-and-ends** tail — Missouri coop member-sponsorship law, **iroh voice/video** (callme,
iroh-live), **WebRTC-over-iroh**, **iroh-gossip games** (libmarathon, godot-iroh, webxdc, ascii-royale,
iroh-lan), **SNES netplay over iroh**, iroh-as-relay/metadata-passer, **DataBeam/croc/sendme**, **Ostrom
"Governing the Commons"**, and the **"corporation vs. person / held the door open"** anecdote.

> **Provenance & verification status (READ FIRST).** AI-assistant (**Gemini**) dialogue; user flags
> Gemini as sometimes unreliable. Fact-checked 2026-06-22 — companion
> **`...-FACTCHECK.md`**. **This transcript heavily overlaps already-filed parallel intakes; those
> fact-checks are the source of truth and are cited rather than redone:**
> - Sun / Steem / Hive / coops → `cooperative-social-union-governance-dialogue-2026-06-22-FACTCHECK.md` + `thinking/cooperative-social-union-model.md`
> - Groundmist / grjte / open-social → `croft-atproto-sovereign-appview-open-social-dialogue-2026-06-22-FACTCHECK.md`
> - did:webvh / did:tdw / `alsoKnownAs` chaining → `thinking/cross-platform-identity-provenance.md`, `plc-identity-resilience.md`
> - iroh voice/video (callme, iroh-live), godot-iroh → `thinking/realtime-media-over-iroh.md`, `iroh-realtime-media-references.md`, `iroh-quic-localfirst-ecosystem-dialogue-...`
>
> **Net-new corrections carried forward (see FACTCHECK):**
> - **Hard Fork 23** confiscated **~$6.3M (23.6M STEEM) from 64 accounts** (May 20 2020) — the
>   transcript's **"$5M" is low**; the rest (64 accounts, Hive-supporters, Notestein/Blocktrades) is correct.
> - **atproto resolves `did:plc` and `did:web` ONLY — NOT `did:key`** (the transcript lists three;
>   did:key is the error). The overall conclusion stands: **did:webvh is not atproto-resolvable**, so the
>   bidirectional-`alsoKnownAs` link to a did:plc account is the correct workaround (W3C-standard; note
>   `equivalentId` is a stronger-but-less-supported form).
> - The **"held the door open / did you ask for money first" anecdote**: the **film (*The Corporation*,
>   2003; Achbar/Abbott; Bakan's book) and Robert Hare (PCL-R) are real**, but the **exact exchange
>   cannot be sourced** — attribution is UNVERIFIABLE. The **Israeli daycare "A Fine is a Price"** study
>   (Gneezy & Rustichini 2000), the **Thurlow** "no soul to damn" quote, **Ariely** social-vs-market
>   norms, and **Ostrom** (1990 / 2009 Nobel) all CONFIRMED.
> - **All named iroh games/tools CONFIRMED real:** libmarathon (Bevy+iroh+gossip+CRDT), ascii-royale
>   (Chad Fowler), iroh-lan (Hamachi-like), DataBeam (croc=schollz, sendme=n0), callme, iroh-live.
> - **Hive user counts UNVERIFIABLE** (Splinterlands DAU fell to ~8–16k by 2024); **TRON "8M daily tx"**
>   unconfirmed (rest of TRON: 27 SRs, $70M 2017 ICO, ~$85B USDT, 3s blocks — confirmed).
>
> iroh/atproto facts: cite the project source of truth. Inline citation source-name lines + embedded
> video references preserved as pasted; user/Gemini typos verbatim.

---

## The dialogue (verbatim content)

> Tell me about the ground most local first bluesky related project, motivations, players, key points, current state, outcome, quotes, related projects

You are likely thinking of Groundmist (a slight phonetic mix-up from voice-to-text or translation). Groundmist is a highly regarded ecosystem experiment that explicitly bridges the AT Protocol (atproto)—the open, federated infrastructure powering Bluesky—with the principles of local-first software.
WhiteWind

[Motivations] Traditional local-first software focuses heavily on offline functionality and device-level data ownership using Conflict-free Replicated Data Types (CRDTs). However, it often hits a wall regarding public distribution, standardized data formatting, and identity management. Conversely, the AT Protocol excels at open public data, global schemas, and cryptographic identity portability, but it naturally lacks an offline-first, private data layer. Groundmist was conceived to mash these two paradigms together.
WhiteWind

[The Players] grjte: The mononymous developer, researcher, and cryptographer (formerly at Bain Capital Crypto) who created and developed Groundmist. Ink & Switch: The prominent independent industrial research lab behind the local-first movement. Following the public traction of Groundmist, grjte was recruited into Ink & Switch to work on their collaborative software frameworks (GAIOS and Patchwork).

[Key Points — The Three Experiments]
1. Distribution — 1-click publishing of local-first data to a public Bluesky PDS (Groundmist Library).
2. Legibility — applies Bluesky's Lexicon schema system to private local-first data so apps can read each other's local databases (Groundmist Editor & Notebook; uses the WhiteWind blog lexicon to draft essays offline).
3. Interoperability — replicates the idea of a Bluesky PDS but private-by-default, a personal data lake (Groundmist Sync; self-hosted, cryptographically owned by your Bluesky identity).

[Current State & Outcome] A highly successful proof-of-concept and technical blueprint. The individual code repos (like groundmist-sync) remain early prototypes with incomplete, experimental authentication blocks, but the overarching design pattern was widely validated. It demonstrated that a user's Bluesky identity (their DID) can act as the root authority for private, offline data syncing. It served as a major technical springboard for grjte, anchoring discussions on platforms like the Zero Knowledge Podcast.

[Key Quotes & Philosophy]
"Groundmist echoes the design of AT Protocol, which emphasizes user ownership and interoperation, but it applies these ideas and select elements of the protocol to local-first software and data instead of to global public data."
It frames the AT Protocol not merely as a backend for a microblogging platform like Bluesky, but as potential "public infrastructure" (akin to DNS or HTTP) for the entire future of local data storage.

[Related Projects] WhiteWind (lexicons adapted for Groundmist's offline editors); Automerge (the JSON CRDT engine under Groundmist's hood); y-atproto & Jake Lazaroff's work (Yjs CRDT explorations turning the Bluesky firehose into serverless real-time sync); Keyhive (capabilities-based local-first access control by Ink & Switch).

---

> [On the bluesky/atproto research + key figures — the URL-prompted overview:]

The URL provided implies an interest in the public architecture of Bluesky and its foundational Authenticated Transfer (AT) Protocol. Bluesky launched its invite-only beta in early 2023 and opened fully to the public in early 2024, rapidly scaling past 10 million registered users by late 2024 (Kleppmann et al., 2024).

[Datasets] Total Network Histories: a dataset of the complete post history of over 4 million Bluesky accounts, 235 million posts (Failla & Rossetti, 2024). Temporal/Feed: BlueTempNet (Jeong et al., 2024).

[Foundational Goals] Decentralized Identity (DIDs, account migration); Algorithmic Choice (Feed Generators); Composable Moderation (labeling services, stackable).

[Key Figures] Jay Graber (CEO of Bluesky); Paul Frazee (protocol engineer); Martin Kleppmann (distributed-systems researcher who helped formally map the protocol); Jack Dorsey (initiated Bluesky in 2019).

[Ways to get involved] Explore bluesky-social/atproto (TS + Go); host a PDS (Docker); build a Feed Generator (TS/Python starter kits); tap the firehose (com.atproto.sync.subscribeRepos, no auth).

References: Failla & Rossetti (2024) PLoS ONE 19(11):e0310330; Jeong et al. (2024) BlueTempNet, IEEE Data Descriptions 1:71–79; Kleppmann, Frazee, Gold, Graber, Holmgren, Ivy, Johnson, Newbold, Volpert (2024) "Bluesky and the AT Protocol: Usable Decentralized Social Media," ACM CoNEXT-2024 Workshop, 1–7.

---

> Tell me this story as a narrative with references, quotes, key figures, then and now reality
> [+ pasted block: In March 2020 a contentious hard fork split Steem after Justin Sun (TRON) acquired Steemit Inc.; community launched Hive, airdropped equivalent tokens, froze the acquired stake. + the Hive/Steem technical reference list.]

[Act I: The Premise and the Ninja Mine] Steem = DPoS + "Proof of Brain" social consensus, automated token-inflation rewards. Steemit Inc. held a massive "ninja-mined" stake; a gentleman's agreement said it would never vote for the 21 elected witnesses.

[Act II: The Takeover] Feb 2020: Justin Sun (TRON) acquired Steemit Inc. + the token cache. Witnesses executed a soft fork deactivating the Steemit Inc. voting power. Sun coordinated with Binance, Huobi, Poloniex to use user-deposited STEEM. On March 2, 2020, Sun used exchange stakes + the ninja-mine to vote out all 21 community witnesses, replacing them with his own.
"Steemit has successfully defeated the hackers and safely taken control of the STEEM ecosystem back to the community." — Justin Sun via Twitter.

[Act III: The Great Escape] On March 20, 2020, Hive was born — a direct code fork of Steem's DPoS engine, with an airdrop mirroring all balances EXCEPT the corporate stake held by Sun and the puppet witnesses, whose funds were frozen.

[Key Figures] Justin Sun (TRON); Ned Scott (co-founder/former CEO of Steemit Inc., sold to Sun); the Steem Witnesses; centralized exchange execs (Binance/Huobi later backpedaled).

[Then vs Now] Hive = decentralized, community-run, the apps + users migrated (Splinterlands), proven case study in community resilience. Steem = effectively centralized under TRON-backed entities, a quiet echo chamber.

---

> Ok so how did the Sun dude respond?

[1. Rhetorical: framing Hive as "hackers"] Sun framed the Hive fork as a malicious cyberattack / IP theft; claimed to contact law enforcement.

[2. Financial retaliation — Hard Fork 23] In May 2020 the Sun-controlled Steem network executed Hard Fork 23, directly confiscating roughly $5 million of STEEM from 64 specific accounts who supported/launched Hive (incl. Blocktrades founder Dan Notestein), moving the funds to a corporate-controlled wallet.
CoinGeek

[3. Censorship + exchange pressure] Pro-Hive accounts downvoted/banned on Steem; withdrawal delays / "wallet maintenance" freezes on Sun-tied exchanges.

"The Hive blockchain was illegally created by hackers who stole Steem's community assets. We are working with law enforcement to protect our ecosystem." — Summary of Justin Sun's public stance.

[The Ultimate Irony] By seizing $5 million from private wallets, Sun proved the Hive community's point: under his control, Steem was no longer decentralized or safe.
CoinGeek

---

> So they stole 5mil bc they didn't like the communities partial consensus actions? That's actually wild
> What did that ding dong think he was buying, capitalism meets collective action

Hard Fork 23 broke the ultimate unwritten rule of public ledgers: immutability. By coding 64 specific wallet addresses into the core update and transferring their balances, Sun ran a localized authoritarian regime via code.

[The Corporate Miscalculation] Sun viewed it through a TradFi M&A lens — buy the company (Steemit Inc., ~20% of token supply), buy the controlling stake. But a blockchain's value resides in the social contract of the nodes and community, not the legal entity. He thought he bought a digital media conglomerate; he bought a server cluster and a blueprint. When the community said "we'll take a copy of the database and leave you and your 20% behind," he treated a voluntary ideological exit as theft.
Medium

---

> So where is that Sun guy now? How did it end?

[Steem vs Hive ended] Hive won the ecosystem (community, dApps, devs stayed). Steem became a ghost town of bots under TRON-backed entities.

[Where Sun is now — 2026] Still runs TRON. Traded blockchain feuds for geopolitical/legal battles:
1. **World Liberty Financial** (Trump-family-linked) — April 2026 Sun filed a 52-page lawsuit alleging $WLFI misrepresentations; alleges WLF added a secret "blacklist" function to freeze/burn his tokens when he refused more capital. WLF countersued for defamation (May 2026), alleging a bot-driven smear campaign.
Dynamis LLP / The Japan Times / The Defiant / Banking Dive
2. **SEC settlement** (March 2026) — settled the 2023 charges (unregistered TRX, wash-trading).
Dynamis LLP
3. **Tech pivot** — keynotes (DC Blockchain Summit, Paris Blockchain Week) pitching TRON for "Agentic AI" + a post-quantum security upgrade.
Unchained Crypto / CryptoRank

"You can buy the tokens, but you can't buy the crowd."

---

> Ok, so tell me about TRON

[Paradox] Ideologically disdained (centralized, Sun baggage), practically one of the most heavily used chains. "If Ethereum is a luxury toll road, TRON is a gritty public interstate."

[1. Architecture] Launched 2017, ~$70M ICO; a modified EVM clone; DPoS via exactly **27 Super Representatives** elected by TRX holders. Sun's whale stakes effectively dictate the 27.
CBS News / Eco

[2. King of USDT] ~$85B circulating USDT on TRON (TRC-20) — nearly half of all Tether. ~8M daily transactions; 3-second blocks; the dominant rail for remittances in high-inflation regions (LatAm, SE Asia, Africa). A USDT transfer costs ~$2–4 of TRX energy vs $5–20+ on Ethereum.
Eco / Bitcoin Foundation / Coincub

[3. Burn mechanics] Bandwidth/Energy via freezing or burning TRX → deflationary; market cap well over $30B.
Eco

[Takeaway] A centralized machine for processing digital dollars at lightning speed — does that job better than almost anyone, even if the guy running it is constantly in court.
Binance / Coincub

---

> Convenience wins the first mile and the last often

It absolutely does. In tech and finance, friction is the ultimate user killer. The most philosophically pure system loses if buying a sandwich takes forty-five minutes and $15 in gas. That's how Sun built a multi-billion-dollar empire despite being disliked by the crypto intelligentsia: the First Mile (easy fiat→stable asset) and the Last Mile (send instantly, cheaply, reliably). TRON traded decentralization for predictability. "Purism builds great manifestos, but convenience builds monopolies."

---

> So how to balance them for humanity, and tell me about Give, what it's for

[Balance via layered architecture] Like banking: a rigid/secure/purist base settlement layer (Fedwire), with convenient centralized layers on top (Visa/Apple Pay/Venmo). In blockchain: keep the base layer decentralized + un-capturable, build fast/cheap "Layer 2" on top; fall back to the base if the top censors you.

[Assuming you meant **Hive**] An open-source decentralized social blockchain + fast data ledger, built to host dApps/content/Web3-social.
1. Zero-fee, 3-second transactions via a **Resource Credit** system (hold the token → regenerate bandwidth).
2. **"Proof of Brain"** content monetization (upvotes route a daily rewards pool; PeakD, Ecency).
3. **Censorship resistance** (21 consensus witnesses elected by a distributed community; data immutable on-chain).
4. **Sovereign identity** (your username = a universal Web3 passport; Splinterlands, 3Speak).

---

> Does give move money?

Yes — both Hive and "Give" protocols move money differently.
1. **Hive**: HIVE (liquid token), **HBD** (native ~$1 stablecoin), and the content rewards pool (newly minted HIVE/HBD to creators by upvotes).
2. **"Give" protocols** (Giveth.io, Givepact, The Giving Block): philanthropy/charity — zero-fee crypto donations to non-profit smart contracts; instant fiat conversion (Givepact/The Giving Block); on-chain accountability.
Coinbase / Giveth.io / Givepact

---

> And both can be converted easily to fiat currency?

Yes, with different "ease." **Hive route**: send liquid HIVE/HBD to an exchange that lists them (Huobi, Gate.io, Upbit, or cross-chain swaps), sell for cash, bank transfer. **"Give" protocols route**: donors contribute mainstream crypto (ETH/BTC/USDC); auto-conversion widgets (MoonPay, Transak, Zero Hash) swap to USD behind the scenes; fiat batch-deposited to the charity's bank account.

---

> Bitcoin and crypto currency is very mercantile minded even when it's used for completely unrelated use cases

You've hit the deepest truth about the space. Even building a decentralized Twitter or charity, the underlying engine is market-driven — every human interaction can/should be optimized by a financial incentive. Baked in three ways:
1. **Game theory as a substitute for trust** — cooperation via economic incentive, not shared values. Adam Smith's invisible hand turned into an immutable algorithm.
2. **The financialization of everything** — every action has a fluctuating cost (gas / energy / Resource Credits). It forces a merchant mindset.
3. **The tokenization of alignment** — to give people a voice, we give them tokens; the moment a token has a price ticker, "community member" and "investor" blur.
The ultimate paradox: an un-hackable tool for collective organization that forces the whole system into a 24/7 marketplace.

---

> Huh, makes me think it has a valid place but is not the whole story

Exactly why "crypto will replace everything" faded. Blockchain is exceptional for **mechanized trust** — moving value across borders, a permanent ledger no dictator can erase, coordinating strangers through math. But humanity needs more:
- **Grace and Forgiveness** — a ledger is mercilessly immutable; human systems need empathy, arbitration, "that was an honest mistake."
- **Intangible Shared Values** — true collective action comes from culture/connection/duty, not resource credits; a price tag crowds out intrinsic motivation.
- **Nuanced Governance** — "whoever holds the most tokens wins" → oligarchy (as Sun proved).
"It's a foundational layer, not the whole house."

---

> This is the crux for me
> [+ the corporation/person door-holding anecdote: "have you ever held the door open for someone?... did you ask for money first?... no... That's the difference."]
> This smells like that to me

That is a beautiful analogy. The phenomenon is the **"Overjustification Effect"** (motivation crowding-out): introduce an external financial reward for something people do out of decency/curiosity, and the intrinsic motivation dies. If you pay someone to hold the door, they stop viewing it as connection and start viewing it as a job.
- **Web2 corporate model**: owns the door, lets you through "free" while tracking/selling/manipulating.
- **Web3 mercantile solution**: tokenize the door — get paid per hold, pay a fee per walk-through.
- **The human failure**: nobody talks/shares because they care; they post to extract value. A third space becomes a trading floor.
"It can secure the building, but it can never be the reason people actually want to hang out inside."

---

> Can you search for the origin of that anecdote?

That exact dialogue doesn't trace to a single published Socratic text; it's "philosophical folklore" distilling the classic argument against corporate personhood, with two roots:
1. **Legal**: Edward Thurlow (18th-c. Lord Chancellor) — "Corporations have neither bodies to be punished, nor souls to be condemned; they therefore do as they like." Later John Poynder: "A corporation has no soul to damn, and no body to kick."
2. **Economic**: Dan Ariely's **social norms vs market norms** (introduce money → destroy the magic; offering your Thanksgiving host $50 ruins the relationship).
The person pointing out that a human holds the door without asking for a dollar is naming the border where the market should stop and humanity begins. A corporation can only exist in the market norm.

---

> How do pbc fit? And dao?

Both bridge the market-vs-social gap from opposite directions.
1. **PBC (Public Benefit Corporation)** — legal patch. Standard corp law: a board has a fiduciary duty to maximize shareholder value (putting humanity before profit is legally actionable). A PBC hardwires a **dual mandate** (Delaware PBC statute): stockholder interests + a declared public benefit + materially-affected stakeholders. It legally shields altruism (e.g., **Anthropic** uses a PBC so safety can't be legally sidelined).
Mintz / Skala / Harvard Law Forum / Herzog
2. **DAO** — bypasses legal frameworks with blockchain code/tokens; token holders vote transparently. The vision: ultimate collective action. The glitch: inherits the mercantile mind — issue a governance token → speculation → whales (Sun types) hijack votes → plutocracy.
[Comparison table: PBC = legal code / legal system enforces / risk = still a corporation; DAO = smart contracts / crypto+math enforce / risk = financializes coordination → plutocracy.]
"You still need human beings who are willing to hold the door open simply because they want to live in a world where doors are held open."

---

> Coops?

Co-ops are the original, tried-and-true model (19th-c. **Rochdale Pioneers**) that aligns economic survival with social norms. The structural difference: **one member, one vote** regardless of capital (vs 1-share/1-token = 1-vote, which collapses into plutocracy/takeover — like Sun buying the Steem stake).
1. **Removing the external predator** — customers/workers ARE the owners; the goal is sustainable service to themselves, not extraction for an outside whale.
2. **Preserving the social contract within commerce** — a **Credit Union** operates in market norms (loans, interest) but can waive a fee for a member in hardship because its members are neighbors, not capital.
[Comparison table: Traditional Corp / PBC / DAO / Cooperative — who rules, how they vote, core vulnerability. Co-op vulnerability: harder to scale (can't take predatory VC).]
Ocean Spray, regional electric co-ops — keeps power localized; everyone owns the building together.

---

> Tell me about hive what you can do with it, how it's used, governance, issues, central figures

[What you can do / how used] Universal social identity (username = wallet/storage/login); Web3 social + blogging (PeakD, Ecency, LeoFinance); decentralized video (3Speak); Web3 gaming (Splinterlands microtransactions, owned assets).
Hive - The Blockchain & Cryptocurrency for Web3

[Governance — DPoS] Consensus Witnesses (top 21 elected by HIVE stake → produce blocks, update core params); the **Decentralized Hive Fund (DHF)** — a programmatic treasury; proposals voted on by HIVE holders → top-voted get automated block-by-block funding.

[Central figures] Dan Notestein (Blocktrades) — core dev/infra, executed the Steem fork; the Hive Witnesses Collective (@arcange, @good-karma, @roelandp); community app teams (Splinterlands, 3Speak).

[Challenges] On-boarding friction (managing owner/active/posting keys); the **governance oligarchy loop** (Hive Power = sway → whales dominate witnesses + DHF); exchange-liquidity limitations after the 2020 Sun fallout.

Note: the decentralized **Hive blockchain** is distinct from **"HIVE Digital Technologies"** (a publicly-traded Bitcoin-mining / NVIDIA-GPU-AI company).

---

> Why vote for a witness and why run?

[Why vote] Every staker (Hive Power) gets up to 30 witness votes; voting costs no tokens and doesn't lock them. Securing infrastructure (uptime/hardware), protecting the governance shield (17 of top-20 must upgrade for a hard fork), aligning monetary policy (price feeds → HIVE/HBD conversion + HBD savings interest).
Hive.blog

[Why run] Demanding (24/7 hardened Linux, NVMe, community activity). Economic incentive: ~10% of annual inflation goes to block-signing witnesses (top-21 produce on a rotating loop; backups by stake). Building social capital — witnesses compete on funding/building ecosystem utilities, self-funding open-source via block rewards.
Ecency / Gate.com / Binance

---

> Other block chains like hive?

[Hive's three pillars] App-layer (social/identity/zero-fee micro-tx); consensus (DPoS, elected producers); origin (anti-corporate rebellion).
1. **Graphene relatives** (Dan Larimer's framework, 3s blocks, account-name addresses, zero gas): **Steem** (the parent / dark mirror under Sun); **WAX** + **EOS** (DPoS/Graphene variants — WAX → Web3 gaming/NFTs, social-login).
2. **Modern open-social-identity**: **Farcaster** (Optimism/ETH L2; on-chain registry; Warpcast); **Lens Protocol** (Polygon/Momoka; social graph as modular primitives, by the Aave team).
3. **Sovereign app-chains (self-funding treasury)**: **Cosmos** (Tendermint/CometBFT; app-chains via IBC; on-chain governance + community pools — echoes the DHF).

---

> Define user count and quantify activity here, hundreds or hundreds of thousands! Etc

[Hive] Lifetime: ~2.3–2.6M registered accounts (many dead/abandoned from the 2020 fork). True active: tens of thousands DAU (~30,000–60,000). Raw tx in the millions/day driven by **Web3 gaming (Splinterlands)** — every battle move written on-chain. Social layer at human speed: ~5,000–10,000 posts/day, ~20,000–50,000 comments, hundreds of thousands of upvotes.

---

> Ok and the others?

[Steem] ~2.5M+ accounts; ~1,000–3,000 DAU (abandoned). [WAX] ~14–15M wallets; 10–15M tx/day (automated, Alien Worlds); ~20,000–50,000 real human DAU. [Farcaster] ~550,000 registered ($5/yr storage rent); ~40,000–55,000 nominal DAU; under 10,000 true power-users (~4,500–5,000 "Power Badge"). [Lens] ~300,000–400,000 profiles; few hundred to 5,000 DAU (infra layer, scattered front-ends).
WAX.io / BlockEden.xyz
Takeaway: nobody has cracked the mainstream 100M mark; the closest anomaly is **Bluesky** (atproto) — 40M+ registered, 1.5–3M DAU — which skips the blockchain/token model entirely.
Sprout Social

---

> Biggest block chains and user quantities?

[By DAU] TRON ~3.5–4.2M daily active addresses (USDT rail); BNB Chain ~3.6–4.3M (retail DeFi/opBNB); Solana ~1.8–3.5M (HFT/meme/DEX, many bots); NEAR ~2.0–2.5M ("AI-native"/sharding). [Value giants, lower DAU] Ethereum L1 ~500–600k (institutional settlement; retail on L2s like Base +400k); Bitcoin ~400–500k (digital gold).
Alchemy / Token Terminal / Binance / NOWNodes
[vs Hive's ~40,000 daily human users.]

---

> Explain the use case relationship between these and how a user experiences then

A digital global economy where each chain is a district/utility.
1. **Bitcoin & Ethereum** — capital core / federal reserve (vault + high-security business district); UX = deliberate, stressful, expensive (Metamask gas $5–50+).
2. **Base & Solana** — retail high streets; UX = instantaneous/frictionless (Phantom/Coinbase Wallet; account abstraction covers gas).
KuCoin / Base / Across / Quicknode
3. **TRON** — cross-border financial rail (USDT in LatAm/Asia/developing economies); UX = paste a "T..." address, send stable dollars; like a high-speed wire transfer.
4. **Hive & decentralized social** — sovereign outpost (data/blogging/identity, no fees, staking credits); UX = like Web2 (Hive Keychain, human-readable username), collaborative not transactional.

---

> Ok now tell me how each of those is funded originally and now

[Bitcoin] Originally: zero funding, no pre-mine/VC/ICO. Now: programmatic inflation + tx fees; devs via grants/sponsorships (Brink, HRF). [Ethereum] Originally: 2014 ICO (~60M ETH, ~$18M → Ethereum Foundation). Now: foundation war chest + gas (fraction to validators, rest burned). [Base] Originally: Coinbase corporate R&D, no token. Now: the sequencer "tax" (bundles L2 → ETH, small markup; a cash cow). [Solana] Originally: $350M+ VC (a16z, Multicoin, Polychain). Now: Solana Foundation treasury + token inflation + sub-penny fees. [TRON] Originally: $70M 2017 ICO (TRON Foundation held ~34% supply). Now: energy/bandwidth market + a massive treasury. [Hive] Originally: the Steem fork (no capital raised; Sun's tokens coded out). Now: the **DHF** (programmatic inflation → smart-contract vault → community-voted proposals streamed daily).
Saxo / Clay / Solana Foundation / CoinStats

---

> Could a crypto key pair be used to generate a did or crypto identity for blue sky and base or hive?

Yes — that's exactly how they function. Every DID relies on a public/private key pair.
1. **Bluesky (atproto)**: handle is a wrapper; the anchor is a W3C DID. **did:plc** (the transcript glosses it "Public Ledger of Credentials" — keys via secp256k1/P-256; genesis op signed; SHA-256 hash → identity string). Signing key separate from recovery/rotation key; supports **did:web** too.
2. **Base (EVM)**: identity = an Ethereum address (Keccak-256 of pubkey → last 20 bytes). Maps to **did:pkh** / **did:ethr** (did:pkh:eip155:8453:0x... ; 8453 = Base). UX: Smart Wallets + Passkeys (secure-enclave key + biometrics).
3. **Hive**: maps keys to a readable username; not native W3C DID notation, but wraps to **did:key**. Key hierarchy: **Posting** (low-risk) / **Active** (transfers) / **Owner** (master reset).
W3C CCG / David Buchanan

---

> Could I use an origin key pair to generate a key pair to use with did:web to change domains but keep crypto provenance?

Yes — you can derive child key pairs from an origin/root key. BUT standard **did:web** shatters provenance on a domain change (the identifier string itself changes → blank slate). The fix: **did:tdw / did:webvh** (Trust DID Web / Verifiable History). Instead of a static did.json, you host a **DID Log** (did.jsonl):
1. **Genesis root** — your master origin key signs Version 1; the hash → a permanent **SCID** (Self-Certifying Identifier).
2. **Domain move** — write a Version 2 entry changing the domain params, signed by the origin key; the log is a sequential hashed chain, so a resolver traces back to V1 and verifies the move. The SCID stays constant → provenance unbroken.
[Key architecture diagram: Master Origin Seed → Root Update Key (offline; signs moves/rotations) + Child Operational Key (daily signing).]
Decentralized Identity Foundation

---

> Explain did:tdw to did:web and moving domains
> Sure but could I combine them so tdw is my identity and did:web is an address I can be found at?
> An address is correct until it's not and that's fine, an address also isn't 1:1 with a person even if they can be reached there

You captured the W3C distinction: separate the **Identifier** (who you are) from the **Service Endpoint** (where to reach you). Use **did:tdw/did:webvh** (SCID-anchored) as the permanent core, and did:web (a domain) as a disposable readable mailbox.
1. **`alsoKnownAs` (mirror link)** — bidirectional: your did:tdw doc lists did:web:current-address.com; the did:web doc points back to did:tdw. Sets up bidirectional cryptographic proof.
2. **Service endpoint router** — a `service` block (type LinkedDomains) tells apps where to send data *now* without confusing location with identity.
[When the address changes: your true identity is untouched (you hold the master key matching the SCID); you publish a new routing entry on a new domain, update serviceEndpoint + alsoKnownAs. Anyone verifying an old signature traces the log, sees the authorized migration, confirms validity.]

---

> Explain setting up a did:webvh key pair identity

[Step 1] Generate two key pairs (Ed25519 or secp256k1): an **Update Key** (root, offline; signs structure changes) and a **Verification/Operational Key** (live in the DID doc; daily signing).
[Step 2] Structure the genesis DID doc with a literal `{SCID}` placeholder.
[Step 3] Compute the SCID — wrap the draft into a Log-Entry-Parameters schema, hash (SHA-256) → permanent SCID.
[Step 4] Replace every `{SCID}` with the hash; author the first JSON-Lines log entry (single compact line, whitespace removed), with a Data Integrity Proof signed by the offline Update key.
[Step 5] Save as `did.jsonl`; publish to `https://yourdomain.com/.well-known/did.jsonl`.
[Live] A resolver fetches the file, reads Line 1, re-hashes, confirms it equals the SCID, validates the Update-key signature.
Vidos / DID:webvh / Decentralized Identity Foundation

---

> Ok now how to create a bluesky subkey for a true identity chain?

To plug a did:webvh identity into atproto, issue a **delegated subkey** to your PDS — control the master ledger (did:webvh), delegate daily-signing to Bluesky's rails.
[Step 1] Generate an atproto subkey (P-256/secp256r1 or K-256). [Step 2] Append it to the did.jsonl log mapped to the W3C fragment **`#atproto`**. [Step 3] Route data streams via the `AtprotoPersonalDataServer` service block (serviceEndpoint https://bsky.social or your PDS). [Step 4] Add the handle bidirectional proof via `alsoKnownAs: ["at://yourdomain.com"]`. [Step 5] Flatten, sign with the offline Update key, append. [Step 6] Inform the network (input your did:webvh:... during account setup/migration).
AT Protocol

---

> Do some deep research to validate all of this actually works

[Verdict: the cryptographic design works; it CANNOT currently power a Bluesky account.]
[What validates] did:webvh ledger mechanics (didwebvh-rs Rust impl); the `{SCID}` placeholder + offline-root signing; key delegation (adding a #atproto verification method).
[Roadblocks]
1. **atproto code restriction** — the network only resolves three DID methods: **did:plc, did:web, did:key**. did:webvh isn't compiled in → relays/AppViews reject it.
Docs.rs
2. **String mutation conflict** — atproto binds an account to a single immutable string; did:webvh's resolving string mutates when you move domains (the SCID is constant, but the location is baked into the string), so Bluesky treats the two strings as separate accounts and can't remap the social graph.
[Verdict table: did:webvh self-containment = Validated; subkey delegation (#atproto) = Validated; live Bluesky integration = Fails (no webvh/tdw resolver + immutable-string demand).]
If you want domain portability on Bluesky now, use **did:plc** + a DNS TXT record for the handle.

---

> Ok so is there a standards compliant way to link the two crypto identities?
> Describe this in detail
> Summarize the overall goal and approach to our key provenance plan here a with approach for validation

[The compliant mechanism: Bidirectional Verification / Equivalency Assertion]
1. **Forward link** (in your did:webvh ledger): add your did:plc string to `alsoKnownAs`; sign the state update with the offline Update key.
2. **Inverse link** (in your did:plc directory): submit a signed PLC operation inserting your did:webvh URI into the account's `alsoKnownAs` (alongside at://yourdomain.com), serialized DAG-CBOR, signed with your PLC Rotation Key, POSTed to plc.directory.
[Why it works] Bluesky keeps did:plc as the unchangeable DB key (relays don't break); external trust apps read your did:webvh ledger, see the linked did:plc, and prove the same entity. Abandon Bluesky → sign a new entry stripping the did:plc alias.
[Validation strategy] (1) Ledger & op integrity (run did.jsonl through didwebvh-rs; GET plc.directory/did:plc:[id], confirm 200 + did:webvh in alsoKnownAs). (2) Automated handshake graph-walk (resolve webvh → extract plc → resolve plc → confirm it contains the original webvh; pass = bidirectional loop closes). (3) App-layer run-time check (log into a Bluesky client via the domain handle; posts/likes/followers index correctly → the extra alsoKnownAs URI doesn't degrade function).
PLC Directory

---

## -- odds and ends --

> Under coop law in MO can a member sponsor other nonmemors inclusion if the charter allows it? Like membership is 10 dollars and includes sponsorship of 10 other people who have equal access to resources but not voting or governance power? It would be a shared resource subsidy

Yes. Under Missouri cooperative law, a co-op can let a paying member sponsor non-members for resource access, if the model is explicitly defined in the articles + bylaws. The sponsor is the **Member** (voting/governance); the sponsored are **Non-Member Patrons** (use services, no ownership/voting).
[Legal framework] **Missouri Cooperative Associations Act (Chapter 351)** (2011; RSMo 351.1030 — broad bylaw authority over member qualifications/rights) OR traditional **Chapter 357** (RSMo 357.130). Either recognizes the structure if the bylaws say so.
[Structuring] Severance of governance and access (sponsored = strictly non-member patrons); revocability (access contingent on the sponsor's good standing); allocation of rights (define "equal access" — unlimited vs a capped shared pool).
[Operational considerations] Liability/indemnification (make the sponsor a guarantor); federal tax (**Subchapter T** member-vs-nonmember income; get a co-op CPA); capacity limits (board authority to cap sponsorships/waitlist).
Justia Law
Disclaimer: I am an AI, not an attorney. Have a Missouri-licensed business attorney review your bylaws.

---

> Is it possible to use iroh p2p to make calls, voice and video?

Yes — n0 has built components/experimental tools for real-time P2P audio/video over QUIC.
1. **P2P Voice: callme** — an Iroh endpoint piping P2P audio; stack = **iroh-roq (RTP over QUIC)** + **Opus** + **cpal** (cross-platform mic/speaker); optional echo cancellation.
Iroh Docs / GitHub
2. **P2P Video: iroh-live** (+ CLI `irl`) — live-streaming over Iroh using **Media over QUIC (MoQ)**: capture/encode/transport/decode/render; `irl call` generates a ticket/QR for a direct 1-on-1 video call; optional relay server bridges to browsers via WebTransport.
GitHub
[Challenges] Iroh only provides transport (you handle codecs/hardware, H.264/AV1/Opus); adaptive bitrate over raw QUIC needs custom logic; media capture/render stability varies across Desktop/Android/iOS. Still experimental vs WebRTC.

---

> Could I do webrtc over iroh!
> So iroh backend and webrtc direct?
> Walk through the design and give me a feasibility rating

[Two patterns]
1. **Iroh as the signaling server for WebRTC** (most practical) — use iroh-gossip / a direct Iroh stream to exchange WebRTC SDP offers/answers + ICE candidates P2P, eliminating a centralized signaling backend; WebRTC then handles the direct media stream.
2. **WebRTC as a custom transport inside Iroh** — for native apps; plug a WebRTC transport so an Iroh node uses RTCDataChannel to route Iroh/QUIC through a browser sandbox (browsers block raw UDP). Community/team prototyped iroh-webrtc-transport layers.
Iroh Docs
[Design: WebRTC Direct + Iroh control plane] Signaling via iroh-gossip (peers find each other by Node ID, swap SDP/ICE); media via WebRTC direct (codecs, jitter buffers, echo cancellation). [Feasibility: **8.5/10** — Architectural elegance 9, media quality 10, hole-punching 7 (symmetric NATs need a STUN/TURN fallback), dev velocity 8.] Verdict: gold-standard for sovereign low-latency calls; budget a tiny TURN server for globally restricted networks.

---

> Ok but in the case of symmetric NAT, as iroh is already creating a dialing ticket what would it take to get webrtc to reuse the same details or be legit tunneled if needed, we already will have a public peer relay for iroh
> Feasibility?

WebRTC and Iroh run on different local UDP ports → WebRTC can't reuse Iroh's ticket/hole-punch state; symmetric NAT blocks WebRTC's separate mapping. Two options:
1. **Legit tunneling (WebRTC over Iroh datagrams)** — treat Iroh as an E2EE virtual wire; intercept WebRTC's outbound RTP packets in memory, feed into Iroh's unreliable **Datagram API** (`send_datagram`); if hole-punch fails, Iroh auto-falls-back to your DERP relay. Pros: 100% reliability, no STUN/TURN, low latency (unreliable datagrams = skip not buffer). Cons: double packet overhead; MTU (~drop to ~1150 bytes to avoid fragmentation); WASM/browser needs WebTransport.
2. **Single-port multiplexing (shared socket)** — bind one UDP socket, pass the fd to Iroh, demultiplex inbound (QUIC→Iroh, STUN/SRTP→WebRTC). But symmetric NATs still struggle unless both peers mirror the multiplexing.
[Rust code blueprint for Option 1: endpoint.connect → on_output_packet → conn.send_datagram; receiver loop reads datagrams → inject_rtp_packet.] [Feasibility: **9/10, production-ready** with a public Iroh DERP relay.] Recommendation: Option 1.
Iroh Docs / Crates.io

---

> Can iroh gossip be used for multiplayer game?
> Tell me about libmarathon ... And other games that exist

Yes — depends on genre. iroh-gossip uses epidemic broadcast trees (HyParView + Plumtree).
[Shines] Massively-multiplayer lobby/world-chat; slow/turn-based games; co-op sandbox/local-state sync (CRDTs). [Fails] Twitch FPS/racing/fighting — hop delay + at-least-once redundancy. [For fast games] hybrid: gossip for control plane (matchmaking/lobby/chat), direct Iroh QUIC datagrams for gameplay; or a P2P host (one machine authoritative).
[**libmarathon / Marathon**] open-source offline-first multiplayer framework on **Bevy + Iroh + iroh-gossip**, **CRDTs** under the hood; demo = a replicated 3D cube simulator on macOS/iOS, sharing cursors/selections/physics P2P.
Crates.io
[Other Iroh games] **godot-iroh** (Godot extension; host shares a Node ID, friends connect, no port-forwarding); **Delta Chat webxdc** real-time P2P apps (P2P Pong with custom clock sync; Color Pixel collaborative canvas); **Desktop VTTs** (Roll20/D&D-Beyond alternatives; players join via the GM's Node ID).
Mastodon / Delta Chat / GitHub

---

> How hard is it to build one of these games? (Webxdc) And tell me about more iroh games

Building a Webxdc game over Iroh is shockingly easy (**1/10 difficulty**) — no networking code; the host app (Delta Chat, Iroh under the hood) distributes data P2P. Core: shared state via event broadcasting (`window.webxdc.sendUpdate()` / `setUpdateListener()`). Workflow: write index.html/css/js → zip → rename `.xdc` → drop into a Delta Chat room. Local dev: **webxdc-dev** (splits the browser into multiple mock players).
Delta Chat / Webxdc
[More Iroh games] **ascii-royale** (Chad Fowler; 16-player ASCII battle royale in Rust; host prints an Iroh ticket; direct QUIC); **iroh-lan** (Hamachi-like; encrypted P2P virtual-LAN tunnel so old LAN-only games — Minecraft Java, StarCraft, CS 1.6 — work over the internet); **godot-iroh-multiplayer** (swaps Godot's default socket for an Iroh endpoint).
GitHub

---

> Could I combine webxdc-dev and iroh to make a fairly simple p2p game app to play with friends? And can iroh call me do group calls?

[Combining webxdc-dev + Iroh] Yes — webxdc-dev is the local simulator; for real friends, swap it for an Iroh-powered host. Architecture: game widget (HTML/CSS/vanilla JS, `window.webxdc.sendUpdate`) → an Iroh host wrapper (Tauri + the iroh crate, loads the widget in a webview) → connection bridge (Tauri intercepts sendUpdate, broadcasts over iroh-gossip; peers' setUpdateListener gets it). Zero-config frontend, Iroh routes P2P.
[Can callme do group calls?] **No — callme is strict 1-on-1** (iroh-roq optimized for two endpoints; no mesh coordination, no SFU audio mixing). For multi-party: use **iroh-gossip as an audio bus** — compress voice to small Opus fragments, broadcast to a shared gossip topic; every node decodes/plays locally.

---

> Are there godot based games then that slot in?
> Would it be possible to use an emulator for say snes and use iroh for multiplayer?

[Godot games that slot in] Via the godot-iroh plugin (in the Godot Asset Library), swap the networking layer of peer-hosted games: co-op survival/sandbox (LibreSoma, Minecraft clones); local-multiplayer converted to online (brawlers, bomberman, 2D racers — sync RPCs over Iroh); headless-server hybrids.
Reddit
[SNES emulator multiplayer over iroh] Yes — highly practical. Snes9x/RetroArch/bsnes support **Netplay** (lockstep/rollback). Problem: standard netplay needs port-forwarding (UDP 6073) and fails behind symmetric/CGNAT. Solution: tunnel netplay over Iroh (like iroh-lan) — no source changes; wrap the traffic. Host listens on 127.0.0.1:6073; the Iroh wrapper pipes a friend's inbound Iroh connection into local 6073; the client points Snes9x at 127.0.0.1. Iroh's QUIC keeps latency low; DERP relay fallback works even behind strict NATs.

---

> In an iroh peer network can a public ip peer with w nat buried nodes and help them directly connect by acting as a metadata passer for setup?

Yes — that's the foundation of Iroh's hole-punching/signaling (an **Iroh Relay**, conceptually STUN + stateless DERP relay).
1. Both buried peers open outbound UDP to your public helper node (NATs allow outbound); the server sees their mapped public IP/port (STUN role).
2. Peer A asks the server to reach Peer B (by Node ID); the server passes A's networking details to B and vice-versa (metadata passer).
3. Both fire UDP at each other simultaneously; each router opens a temporary hole for the other's IP → the cross-firing packets slip past → direct QUIC established; the helper steps out of the data path.
[If hole-punch fails — symmetric NAT] the public node becomes an **encrypted fallback relay** (forwards raw binary packets). Security: Iroh's E2EE TLS 1.3 over QUIC means the relay sees only encrypted noise + headers.
Iroh Docs / DEV Community

---

> Https://github.com/vinay-winai/DataBeam — What is croc mentioned here?

In DataBeam (a desktop GUI combining two terminal file-transfer tools), **croc** = the open-source P2P file-transfer tool **schollz/croc** (celebrated for convenience — share files with a simple human-readable code phrase). The other is **sendme** (Iroh-based; fast, resumable across network shifts). DataBeam wraps that workflow in a GUI built on modern protocols like Iroh.

---

> Link? Ostrom, "Governing the Commons"

Elinor Ostrom's 1990 *Governing the Commons: The Evolution of Institutions for Collective Action* — challenged the "tragedy of the commons" inevitability by studying long-surviving community-governed resources (Swiss alpine meadows, Japanese forests), establishing **eight design principles** for stable common-pool-resource management; contributed to her **2009 Nobel Prize in Economic Sciences**.
[Links: Cambridge University Press; Google Books; Iowa State faculty PDF (Ch. 3); UNM repository (Natural Resources Journal review); Beyond Intractability summary; Internet Archive lending library.]
AbeBooks

---

> This anecdote was on tv or movie, can you help me find it? [the corporation/person door-holding anecdote]

That specific interaction is from the **2003 documentary film *The Corporation*** (co-directed by **Mark Achbar and Jennifer Abbott**, based on **Joel Bakan's** book). The speaker is **Dr. Robert Hare**, forensic psychologist and creator of the **Psychopathy Checklist (PCL-R)**. The segment discusses the legal definition of a corporation as a "person" (14th Amendment jurisprudence); Hare uses the analogy to show a corporation behaves on a purely transactional, algorithmic level of self-interest.
[The crowding-out / overjustification effect.] A famous real-world study: **Israeli daycare centers** introduced a cash fine for late pickups → late pickups *doubled* (parents reframed it as a price to buy extra time, not a social obligation).

---

*(End of pasted dialogue.)*
