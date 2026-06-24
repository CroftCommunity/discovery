# P2P / Local-First Projects: Founder Motivations & User Adoption

Research compiled June 2026. Sources linked inline. Gaps explicitly noted. Industry research /
comparison feeding the cooperative P2P social-utility design — examines *why founders built* and
*whether anyone crossed the chasm*, to inform the adoption strategy. Self-reported user numbers are
flagged.

---

## 1. Secure Scuttlebutt — Dominic Tarr

**Founder motivation.** Tarr was living on a sailboat in New Zealand's Hauraki Gulf with unreliable
connectivity, needing to communicate with friends (fellow Node.js hackers living unconventionally)
without stable infrastructure. He described dual motivations: a personal "adventure" motive ("build
a thing no one has built before… name it") alongside "a sort of moral optimistic Sci-Fi
motivation… liberating us from perceived tyrannies." On computers arriving "via military, then
corporate accounting systems… about centralizing and empowering a single perspective, but that
isn't how people are. Everyone has their own story." Also cited disillusionment with orgs "whose
managers did not understand or care about software quality." The name = sailor slang for
water-cooler gossip; gossip replication mirrors how info spreads among people in physical proximity.

**Adoption.** Small dedicated community of OSS devs, off-grid/solarpunk-adjacent, some real sailing
use. Gained some popularity on the Facebook–Cambridge Analytica wave. No reliable active-user count
(no central registry by design).

**Status.** Declining. Core dev slowed; key members departed; append-only log scaling problems led
to Willow as successor.

## 2. Earthstar / Willow — Cinnamon, Sam Gwilym, Aljoscha Meyer

**Founder motivation.** Earthstar begun 2020 by Cinnamon during Mozilla Spring Labs; work "put user
safety at the heart of everything." Cinnamon passed away in 2022; Gwilym and Meyer (as "worm-blossom"
since 2022) carry it forward. Willow's motivation over SSB: fragility — "a single component failure
can bring the whole service down, and users regularly lose access to their data." Willow adds
deletability (solving SSB's append-only tombstone accumulation) and private set intersection for
discovery without revealing interests. Goal: make Willow "as boring as possible… PVC piping under
the sink boring."

**The "resignation" story commonly misremembered here is André Staltz, not an Earthstar contributor
(see §3).** Cinnamon *passed away*; did not resign.

**Status.** Active but early. Spec mostly stable; Rust impl underway (NGI Core funding); Iroh (n0)
has its own Willow impl in progress.

## 3. Manyverse / André Staltz — the resignation story

**Founder motivation.** 2016, motivated by "the worrisome rise of Donald Trump and the far right,
assisted by surveillance capitalist methods," specifically Facebook's weaponization of comms
channels. Found SSB via Patchwork, "amazed… genuine friendly interactions, where no one was running
the place." Goal: bring SSB to mobile for "underconnected populations in developing countries."

**The departure (April 2024).** The often-cited farewell: "Building social media or social networks
is not interesting to me anymore. These platforms seem like they're mostly strangers talking to
strangers… huge attention routers, mostly useful for those seeking attention, and mostly detrimental
to everyone else… The SSB community is sweet, but I don't actually know these people." Explicitly
*not* burnout: "Working on PPPPP has been low-stress… but I have been worried about the purpose
behind all of it. I don't believe in the same mission as I did in 2016." After 6,400 hours / 7
years, moved to consulting and cello. The paradox: years building tools for intimate community
connection while his actual intimate community was the 10 closest people in his physical life.

**Status.** On life support. Jacob took over; Mix Irving continues; community small.

## 4. Briar — Michael Rogers & Eleanor Saitta

**Founder motivation.** Began 2011. Rogers (prior Freenet/LimeWire P2P experience) and Saitta
(security consultant). Explicit goal from the start: protect "activists, journalists and civil
society." Unusually specific threat model (adversary can deploy unlimited devices running Briar;
limited ability to persuade users to trust its agents) — reads designed for authoritarian
surveillance states, not Silicon Valley data harvesting. Aspiration: "as simple to use as WhatsApp,
as secure as PGP, and that keeps working if somebody breaks the Internet." No specific personal
inciting incident found; Freenet/LimeWire background suggests long-standing anti-censorship conviction.

**Adoption.** OTF-funded ($361,100 as of 2018), Cure53 audit (2017). Documented use in Myanmar
post-2021 coup; reviews mention cruise ships / poor-coverage areas. No public active-user count.
Android-focused; no iOS planned.

**Status.** Active, niche, stable. Small team, fills a real gap for high-threat-model users.

## 5. Reticulum — Mark Qvist

**Founder motivation.** Dedicated to public domain in 2016. Comms infrastructure over any physical
medium (LoRa, packet radio, WiFi, serial, Ethernet), encryption by default, no central coordination.
No explicit "I built this because…" statement found; the design *is* the statement (128-bit keys as
network addresses, 500-byte MTU).

**The departure (December 2025), per a FOSDEM 2026 presentation.** Apr 2025: license changed to add
anti-AI and "no Harm" clauses (blocking Debian/F-Droid/Alpine main). Summer 2025: break. Fall 2025:
RNS 1.0.0. Dec 2025: "Carrier Switch" — full withdrawal; issue tracker hidden (years of discussion
gone). Parting line: "The protocol is public domain. The code is open source. Everything you need is
right here." Reasons for withdrawal not publicly stated beyond "Carrier Switch."

**Adoption.** Sideband skews off-grid / amateur radio / preparedness. Meshtastic-adjacent but
distinct. No reliable count.

**Status.** In transition; community self-organizing post-Qvist; license change is a distribution
barrier.

## 6. Nostr — fiatjaf

**Founder motivation.** Pseudonymous Brazilian dev (worked at Zebedee), launched March 2020.
"Frustrated by Twitter's increasing bans" — core problem: you couldn't switch to a competitor while
retaining followers; identity + social graph locked in. Protocol for portable identity. Grassroots
ethos: when a dev messaged Jack Dorsey about Nostr, fiatjaf was displeased ("Your approach is too
dangerous… we're trying to run a grassroots movement"). Dorsey later donated 14 BTC, then $10M.

**Adoption.** Claimed 18M users (May 2023) — likely total key registrations, not active. Heavily
Bitcoin/crypto-skewed. Multiple clients (Damus, Amethyst, Primal). Extended to GitHub/Wikipedia
alternatives.

**Status.** Growing but Bitcoin-centric; limited evidence of breakout beyond that community.

## 7. Veilid — Cult of the Dead Cow (DilDog / Medus4)

**Founder motivation.** Launched DEF CON 31, Aug 2023, after 3 years' dev. DilDog (Christien Rioux,
Veracode co-founder): exists to "develop, distribute and maintain a privacy-focused communication
platform and protocol for the purpose of defending human and civil rights." Bio: "if you want to
change the present you need to build the future, and is very sorry for having helped create InfoSec
from hacking, and would like to undo the damage." cDc: the internet "began as an open realm of
possibility" then commercialized with no opt-out. Trigger appears to be governmental attacks on
cryptography (UK Online Safety Bill, Australian hostility). "We cannot have a profit motive because
once you enter a profit motive, you are already done."

**Adoption.** VeilidChat early; no public numbers. Positioned as infrastructure for others. Rust.

**Status.** Active but pre-adoption.

## 8. Keet / Holepunch — Mathias Buus (mafintosh)

**Founder motivation.** Self-taught Danish JS dev, deep in Node.js (600+ npm modules), core
maintainer of Hypercore (evolved from DAT, originally for sharing scientific datasets). Evolution:
DAT → Hypercore → Holepunch/Keet. "Big believer in open source and the role it will play in
liberating communication channels for billions." **Tether is the elephant in the room:** Holepunch
co-founded by Tether, Bitfinex, Hypercore (July 2022); Paolo Ardoino CSO; USDT positioned as the
micropayment layer — a P2P comms platform carrying its stablecoin without centralized infra
regulators could shut down. Buus: "If we stop working, the apps will still work."

**Adoption.** "Downloaded millions of times" (CIO, Apr 2026); earlier 1M+. No demographic breakdown.

**Status.** Active, well-funded via Tether; the connection raises independence/purity questions the
P2P community cares about.

## 9. Matrix — Matthew Hodgson & Amandine Le Pape

**Founder motivation.** Born inside Amdocs (~2014). Hodgson had 15+ years across IRC/IM/NNTP/IMAP/
SIP/XMPP. Founding frustration = fragmentation: closed silos, analogous to pre-web AOL/CompuServe.
The internet's breakthrough came from shared standards; messaging lacked one. On not extending XMPP:
neither IRC nor XMPP ever invested in a competitive client experience — "Nobody sat down and said
'I'm gonna write the Netscape equivalent for XMPP.'" Element built to "look and smell like Discord,
or Slack, or WhatsApp." Blunt: "People don't give a stuff about decentralization… or the open web."

**Adoption.** 25+ countries deploying for government/public-sector digital sovereignty (Germany's
ZenDiS/openDesk, European Commission). 750K+ accounts on Matrix.org (older figure). The de facto
standard for government-sovereign messaging in the EU. **Foundation not financially sustainable:**
needs ~$1.2M/yr, brings ~$561K; bridges threatened with shutdown early 2025.

**Status.** Growing institutionally, financially precarious. Closest any decentralized protocol has
come to institutional mainstream — via *government mandate, not organic consumer choice*.

## 10. Automerge / Local-First — Martin Kleppmann & Ink & Switch

**Founder motivation.** 2019 "Local-First Software" essay (Kleppmann, Wiggins, van Hardenberg,
McGranaghan): "both the convenient cross-device access and real-time collaboration provided by cloud
apps, and also the personal ownership of your own data embodied by 'old-fashioned' software."
Kleppmann (Cambridge associate prof, author of *Designing Data-Intensive Applications*) motivation
reads intellectual/academic, not crisis-driven. Ink & Switch = research lab; Automerge (CRDT) its
most significant output.

**Adoption.** Movement grew since 2023: WIRED coverage, Discord 1,600+, first Local-First Conf
(Berlin, May 2024), Linear adopting local-first. Primarily developers/product builders, not end
users directly.

**Status.** Growing as an architectural movement; Automerge itself in maintenance mode (Kleppmann
advisory). Ideas spreading; specific tools niche.

## 11. Spritely / OCapN — Christine Lemmer-Webber

**Founder motivation.** Co-authored ActivityPub. Spritely motive: ActivityPub a "great success" but
"a few things bother me" — it "isn't decentralized enough to survive the threats coming for it,
including age-verification laws rooted in anti-queer politics, hardware-level lockdown… and
data-centre concentration that turns information infrastructure into a kill switch." Four years'
research before launch; vision = next-gen fediverse as a distributed game on object-capability
(OCapN) security.

**Status.** Active research; no consumer product yet. Goblins framework exists; path to adoption long.

## 12. Signal — Moxie Marlinspike

**Founder motivation.** Cryptographer/anarchist/sailor (real name Matthew Rosenfeld). Co-founded
Open Whisper Systems 2013 on TextSecure/RedPhone. Core position: "privacy through policy" (TOS
promises) is inadequate → E2EE as default, not option. Kyiv forum (2026): "It is possible to create
technology that protects privacy while remaining user-friendly… security often benefits from
simplicity." 2018: Signal Technology Foundation formed with Brian Acton's $50M.

**Adoption.** 100M+ installs. Mass-adoption events: WhatsApp privacy-policy change (Jan 2021),
surveillance scandals. Ukrainian military adoption ("Blue"). **The only project here that crossed
the chasm to mainstream non-technical users** — by making encryption invisible (default, no config)
and mimicking incumbent UX.

**Status.** Mainstream, financially stable (foundation), actively maintained. The only true
mainstream success in this landscape.

## 13. BitChat — Jack Dorsey / Block

**Founder motivation.** Announced July 7, 2025; a weekend project to "learn about Bluetooth mesh
networks, relay and store-and-forward modes, message encryption models." "Protocols, not platforms."
Criticized Bluesky for "repeating all the mistakes we made as a company" by becoming "another app."
Evolution: Twitter (centralized) → Bluesky (federated) → Nostr (protocol) → BitChat (physical mesh).
Donated $10M to Nostr (2025); BitChat (local BLE mesh) + Nostr (global) complementary.

**Adoption.** ~70,000+ downloads in one week during Madagascar civil unrest (late 2025). Removed from
China App Store (Feb 2026) by the CAC as a "public opinion-influencing" service.

**Status.** Active, early, attention-grabbing. China removal arguably validates the threat model.

## 14. Offline Protocol / Fernweh

**Founder motivation.** No explicit founder origin story found. $1.1M pre-seed (Portal Ventures).
Infrastructure for the "4.6 billion people affected by connectivity shutdowns."

**Adoption.** Claimed 300K OfflineID holders across 80+ countries; Fernweh launched after 500+ beta
users. **Self-reported, unverified.** Crypto/Web3 framing (OfflineID as a "non-intrusive KYC
alternative competing with Worldcoin") — the incentive layer makes it hard to distinguish genuine
communication users from speculative registrations.

**Status.** Active, VC-funded, crypto-adjacent. Mesh tech may be real; user counts warrant skepticism.

## 15. Quiet

**Founder motivation.** "We are building Quiet to sharpen the tools open societies use to hold power
accountable." Core dilemma (README): comms software needs servers, so "who runs the server?" becomes
a free-software dilemma — project? users (only hobbyists have servers)? an org (which then controls
the data and relationships, limiting the freedom to fork and flee)? No named founders found; small
team; they dogfood it as their own Slack replacement.

**Status.** Active, early, unaudited. Warns it "should not be used when privacy and security are
critical." Tor + IPFS; Android app has battery/stability complaints.

## 16. Berty — Manfred Touron / Berty Technologies

**Founder motivation.** French nonprofit NGO. "A world where free and secure communications are
common and fear of censorship or surveillance are not." Explicit org goal: "to progressively
relinquish control over Berty and to make it become a truly global community project." Touron tied
to Paris P2P / 42 ecosystem. No specific personal inciting incident found. libp2p + Go chosen
pragmatically (networking stack + mobile cross-compile via gomobile); built go-orbit-db, contributed
BLE transport to libp2p.

**Status.** Active but pre-audit. iOS + Android exist. Small team.

---

## Research Question 2: adoption analysis

**Who crossed the chasm?** Only **Signal** (100M+). Succeeded by making encryption invisible and
matching incumbent UX.

**Politically driven adoption.** Briar (Myanmar post-coup), BitChat (70K in a week, Madagascar;
removed from China), Signal (Ukraine "Blue"; spikes track surveillance events), Fernweh (claims 80+
countries, unverified).

**Practically driven (P2P is the only option).** SSB (sailing/off-grid), Meshtastic/Reticulum
(off-grid, amateur radio, preparedness), BitChat (festivals, cruise ships — BLE range fits).

**Ideologically driven.** Nostr (Bitcoin/crypto; 18M keys, far fewer active), Matrix (OSS devs +
government sovereignty), Keet (1M+ downloads, motivation unclear, Tether-adjacent).

**Has any P2P tool crossed the chasm without becoming centralized?** **No.** The three-condition
hypothesis holds: (1) **product parity** — only Signal; (2) **sustainable org model** — Signal
(foundation + Acton $50M), Matrix (foundation, precarious), Keet (Tether patronage, not a co-op),
everyone else donation/grant-funded and struggling; (3) **inciting event** — WhatsApp policy change
(Signal), Myanmar (Briar), Madagascar (BitChat) — real, but produce *spikes, not sustained migration*.

**Where the hypothesis may be incomplete — institutional mandate.** Matrix's 25+ country adoption was
top-down (governments mandating sovereign infra), not bottom-up. A possible **fourth bridge** for the
cooperative model: make the co-op's infrastructure adoptable by *institutions* (NGOs, unions,
cooperatives) that bring members along, rather than waiting for individuals to choose P2P.

**The Staltz paradox.** After 6,400 hours building a tool for intimate community connection, Staltz's
actual intimate community was 10 physical people; SSB was "sweet" but strangers. Implication: the
*social* infrastructure problem may be harder than the technical one. P2P tools that succeed may need
to be **already embedded in existing trust networks** (a union, neighborhood, family, professional
community) rather than expecting trust networks to form around the tool.

---

## Summary table

| Project | Founder story sourced? | Active users (est.) | Growing? | Crossed chasm? |
|---|---|---|---|---|
| SSB | Yes (Tarr) | Small, declining | No | No |
| Earthstar/Willow | Partial (Cinnamon deceased) | Developers only | Slowly | No |
| Manyverse | Yes (Staltz) | Very small | No (departed) | No |
| Briar | Partial (Rogers/Saitta) | Small, niche | Stable | No |
| Reticulum | Partial (no explicit origin) | Small, hobbyist | Uncertain | No |
| Nostr | Yes (fiatjaf) | 18M keys, fewer active | Yes | No (Bitcoin niche) |
| Veilid | Yes (DilDog/Medus4) | Minimal | Early | No |
| Keet | Partial (Buus) | 1M+ downloads | Yes | No |
| Matrix | Yes (Hodgson) | Large (gov't) | Yes | Partially (institutional) |
| Automerge | Partial (Kleppmann) | Developers only | Yes (as movement) | N/A (infra) |
| Spritely | Yes (Lemmer-Webber) | Developers only | Early | No |
| Signal | Yes | 100M+ | Yes | Yes |
| BitChat | Yes (Dorsey) | Early, spike-driven | Uncertain | No |
| Fernweh | No founder story | 300K claimed, unverified | Unknown | No |
| Quiet | Partial (mission page) | Very small | Early | No |
| Berty | Partial (Touron, NGO) | Small | Early | No |

## Gaps & what couldn't be sourced

Briar: no specific personal inciting incident for Rogers. Reticulum: no origin statement from Qvist;
withdrawal reasons unstated beyond "Carrier Switch." Fernweh: no founder story; 300K unverified;
crypto framing. Quiet: no named founders. Berty: Touron identified, no personal-motivation essay.
Keet: no geo/demographic breakdown of downloads. Earthstar/Cinnamon: motivations beyond "user safety"
undocumented. DAT origin: how Buus specifically got involved not in a single sourced statement.
