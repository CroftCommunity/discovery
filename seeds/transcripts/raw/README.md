# Raw transcripts archive (reference & provenance)

date: 2026-06-15

These are **verbatim** raw transcripts as provided, kept for reference and provenance — the
unedited source behind the condensed `CODING-TRANSCRIPT.md` summaries that live next to each
proof/experiment. Where a transcript embedded a brief that is already saved verbatim
elsewhere in the repo (the lineage thesis, the experiment-suite spec), this archive points to
that canonical copy instead of triplicating it, and preserves the session log verbatim.

## Provenance status

- **Code:** verbatim. Each proof/experiment tree was `git clone`d from its croftc
  SecurityPolicy PR branch and copied unchanged; `diff -rq` against the branches is empty
  (only the added PR-CONVERSATION/CODING-TRANSCRIPT files and excluded SecurityPolicy
  plumbing differ).

- **PR conversations:** verbatim, pulled from `gh` into each `PR-CONVERSATION.md`.

- **Coding transcripts:** verbatim here in `raw/`; condensed/readable renderings in each
  artifact's `CODING-TRANSCRIPT.md`.

## Files

| File | PR | Artifact | Embedded brief (saved elsewhere) |
|---|---|---|---|
| `pr6-appview-validation.md` | #6 | experiments/appview-validation | — |
| `pr9-lineage-group-model.md` | #9 | Proofs/lineage-group-model | experiment-suite spec → in lineage-group-model |
| `pr8-lineage-groups.md` | #8 | Proofs/lineage-groups | lineage thesis → thinking/thesis-lineage-groups.md |
| `pr4-public-roundtrip.md` | #4 | experiments/public-roundtrip | — |
| `pr3-encrypted-local-first.md` | #3 | Proofs/encrypted-local-first-atproto | — |

### Non-PR narrative / research transcripts

| File | Subject | Distilled into |
|---|---|---|
| `croft-crofting-research.md` | Scholarly crofting history + naming argument | `../../NAMING.md` |
| `croft-crofting-narrative.md` | Popular narrative re-telling of the crofting story (quotes/anecdotes `[UNVERIFIED]`) | `../../NAMING.md` "Vivid grounding"; COHESION §16 |
| `germ-xchat-design-dialogue.md` | Germ / xChat design dialogue | research/germ-xchat-features.md |
| `p2p-architecture-origin-dialogue.md` | P2P architecture origin dialogue | ANALYSIS.md / thinking/ |
| `atproto-atmospheric-web-iroh-mobile-dialogue.md` (+ `...-FACTCHECK.md`) | AT-Proto atmospheric web / Neo-GeoCities / Iroh opportunistic mobile P2P (Gemini; fact-checked 2026-06-22) | COHESION §17; thinking/ + ECOSYSTEM distillation pending |
| `croft-drystone-protocol-naming-dialogue-2026-06-22.md` (+ `...-FACTCHECK.md`) | Naming the P2P protocol (Gemini): the candidate sweep (isonomy / pares / souming / …), the **Drystone** decision, and the **"Princeps Problem"** anti-pattern (nominal equality masking capability asymmetry). **Cleaned-paste, content-faithful** (§4); heavily fact-checked (1 fabrication "Skartsia and Tomi", 4 PARTLY; substance grounded). | `NAMING.md` "Protocol-layer naming"; COHESION §24; partially closes ROADMAP_TODO A7 |
| `croft-atproto-pds-germ-privatedata-dialogue-2026-06-22.md` (+ `...-FACTCHECK.md`) | AT Proto / PDS deep-dive (Gemini): Bluesky federation limits + DMs, **Germ Network** (MLS, AC Protocol, Anchor-Key-in-profile, `draft-xue-distributed-mls`), self-hosting a PDS (ElfHosted/DO/Hostinger; Cocoon/rsky-pds Postgres), **PDS-as-selective-file-proxy** shims, object-storage cost map, PDS/Relay/AppView, and the **real community-led ATProto Private Data Working Group** (#3363/#121; trusted-PDS-vs-ZK; key-revocation). **Cleaned-paste, content-faithful** (§4); heavily fact-checked — **updates the source-of-truth** `atproto-atmospheric-web-iroh-mobile-FACTCHECK.md` (dated addendum). | COHESION §26; ECOSYSTEM §6 (Germ enriched) + the "no native WG" note; ROADMAP_TODO; research/ distillation pending |
| `croft-atproto-sovereign-appview-open-social-dialogue-2026-06-22.md` (+ `...-FACTCHECK.md`) | The **sovereign PDS/AppView "club"** (Gemini, flagged "esp good"): private blocking (inbound-effective / outbound-impossible → experience-shaping "local shadow ban"), off-repo private feeds + encrypted blobs, asymmetrical federation, private Labelers, multi-source AppView (AT+AP+Nostr+RSS), CAR/MST offline mesh, ready-to-hack AppViews (AppViewLite/Blacksky/Zeppelin/Groundmist) + client bases (Ouranos/Heron/atcute); **plus** the AP↔AT bridge (Bridgy Fed / A New Social / Bounce), Meta-Threads-on-ActivityPub, Bluesky VC + Aggregation Theory, the open-social naming sweep (Till/Tillage), and the **Twitter Circles → Communities → Group DMs** cautionary trilogy. **Cleaned-paste, content-faithful (§4); DUPE/FORK consolidated** (superset; tail diverged). Heavily fact-checked — unusually accurate (all named projects real). | `research/atproto-sovereign-appview-club.md`; ECOSYSTEM §5f; `NAMING.md` reservoir (Till/Tillage); COHESION §29; overlaps §27/§26/§25 |
| `croft-app-design-dialogue-2026-06-20-to-22.md` | The Croft **app/client layer**: architecture, honest-seams ponds/pads, stack (Rust core + Tauri/Leptos, web-first), iroh transport tiers, atproto appview routing, the games pond, super-apps, palette/brand, session-review, Phase 0-1-2. **Cleaned-paste, content-faithful — not a pristine export** (no canonical export existed; PLAYBOOK §4). | `thinking/app/` (+ artifacts frozen at `../../multiecosystemapp-unpacked/`); COHESION §18; ROADMAP §12-15; research/ECOSYSTEM distillation pending |
| `crypto-wars-to-p2p-pds-economics-dialogue-2026-06-22.md` (+ `...-FACTCHECK.md`) | Three-body Gemini dialogue: **(a)** the crypto-wars / digital-liberty lineage (PGP/Zimmermann, the MIT-Press-book export loophole, Bernstein, Barlow, the 1970s NSA pressure campaign, surveillance capitalism, Proton v. Apple, Apple-fees/AltStore/**Ma Bell→Carterfone** parallel); **(b)** the **mobile-P2P four-property impossibility** (moderation + multi-device + PFS + offline-mesh need an unequal peer) + protocol landscape (MLS RFC 9420, Matrix, Briar, Cwtch, Quiet, SimpleX, Keet/Pear, Berty/Wesh, **Peat/Defense Unicorns**); **(c)** a **PDS-hosting + P2P-blended business model** w/ enterprise-compliance grounding. **Cleaned-paste, content-faithful** (§4); **heavily fact-checked** — unusually accurate for Gemini (3 REFUTED: a fabricated Zimmermann "Stalin" quote, "Voskop", the Meyer-letter quote; several quote-wordings UNVERIFIABLE; **Peat / Proton case-no. / China 30→25% / Deloitte+Velox fines all CONFIRMED-despite-suspicion**). | COHESION §25; ECOSYSTEM §1 (Peat) + §6; ROADMAP_TODO E20-E21; distilled → `thinking/ios-opportunistic-p2p.md` (four-property impossibility + Peat), `thinking/open-considerations.md` §8 (PDS-host business model, surfaced-not-resolved), `crystallized/principles.md` (Bazelon/Carterfone legal ancestor) |
| `atproto-architecture-appview-relay-explainer-2026-06-22.md` (+ `...-FACTCHECK.md`) | AT-Proto **architecture explainer** (Gemini): AppView = relational index over the firehose; PDS→Relay→AppView→client split; Lexicon=schema; **did:web vs did:plc**; long-form on shared identity; PDS↔Relay `subscribeRepos` WebSocket + CAR/DAG-CBOR; **rev(TID)/seq** dedup; backfill (`getRepo`); `requestCrawl` discovery; stale-endpoint handling; **feed-generator (skeleton) vs AppView (hydration)**. **Cleaned-paste, content-faithful** (§4); fact-checked — **unusually accurate, mostly restates settled atproto facts.** 1 REFUTED (did:plc ≠ "Public Liaison Corporation" → "Public **Ledger** of Credentials"); 1 OUTDATED (relays no longer store every repo — Sync v1.1 non-archival); CONFIRMED-despite-suspicion: 2 vCPU/12 GB relay (current), **Tap** (real official tool). | COHESION §27; **updates source-of-truth** `atproto-atmospheric-web-iroh-mobile-FACTCHECK.md` (Addendum 2); ECOSYSTEM §5b (Tap); low distill yield (teaching restatement) |
| `solid-pds-webid-scalingtrust-dsnp-dialogue-2026-06-22.md` (+ `...-FACTCHECK.md`) | Comparative-landscape explainer (Gemini), 4 bodies: **(a)** Solid (Berners-Lee Pods/WebID/Solid-OIDC/DPoP) vs the **Bluesky PDS**; **(b)** WebID + Solid-OIDC login flow; **(c)** Atlantic Council DFRLab **"Scaling Trust on the Web"** (Jun 2023 — T&S as public infra, **middleware**, **C2PA**); **(d)** **DSNP** (Project Liberty — social graph as public utility, token-free core, delegation; reference chain = Frequency). **Cleaned-paste, content-faithful** (§4); fact-checked — **well-grounded, no fabrications.** PARTLY: Bluesky "public-by-default"; Scaling-Trust exact recs UNVERIFIED (real, on-topic; PDF wouldn't render); added DSNP→Frequency. | COHESION §28; ECOSYSTEM §5 (Solid, DSNP) + §7 (C2PA, Scaling-Trust); low distill yield (landscape) |
| `groundmist-hive-identity-chain-iroh-games-dialogue-2026-06-22.md` (+ `...-FACTCHECK.md`) | Sprawling Gemini dialogue: **Groundmist** (grjte/Ink&Switch local-first×atproto), the **Steem→Hive/Justin Sun** saga + **Hard Fork 23** + **TRON** + **Hive** + **coops/PBC/DAO**, a **did:webvh↔did:plc identity chain** (bidirectional `alsoKnownAs`, #atproto subkey, the "atproto can't resolve webvh" validation), and a long **odds-and-ends** tail (MO coop member-sponsorship, **iroh voice/video** callme/iroh-live, **WebRTC-over-iroh**, **iroh-gossip games** libmarathon/godot-iroh/webxdc/**ascii-royale**/**iroh-lan**, SNES netplay over iroh, iroh-as-relay, **DataBeam/croc/sendme**, **Ostrom**, the **corporation-vs-person** anecdote). **Cleaned-paste, content-faithful (§4); heavy overlap w/ already-filed intakes** (cites their FACTCHECKs). Net-new: Hard Fork 23 ≈ **$6.3M not $5M**; atproto resolves **did:plc+did:web only (NOT did:key)**; door-holding anecdote attribution **UNVERIFIABLE**; all iroh games/tools **real**. | COHESION §30; ECOSYSTEM §5d (iroh games); **seeds `narrative/messaging-and-quotes.md`**; cites cooperative-social-union / sovereign-appview / cross-platform-identity FACTCHECKs |
| `croft-app-ponds-games-dialogue-2026-06-20-to-22.md` | The Croft **ponds & pads / games deep-dive** (the run of the games-pond research prompt): games hunt + inclusion pathways, webxdc security model (Cure53), the moat-from-not-having-things, maintenance-vs-attention + cooperative economics, utility + presence/ritual ponds, complexity/UX + deep-linking, on-device-LLM feasibility, build-order, fair-reveal. **Cleaned-paste, content-faithful** (PLAYBOOK §4); did real in-session web verification (verdicts in the artifacts). | `thinking/app/ponds/` (8 artifacts frozen at `../../apps-unpacked/`); COHESION §19; ROADMAP_TODO E8-E11; ECOSYSTEM §5d |

## Still outstanding (see ../RAW-ARTIFACTS-MANIFEST.md)

The original **design-dialogue transcript** (the first large paste — the messaging research
and the multi-device/social-layer/lineage-fork conversation) is still only distilled into
ANALYSIS.md, not preserved verbatim. Re-drop it to save as
`../design-dialogue-2026-06-13-to-14.md`.
