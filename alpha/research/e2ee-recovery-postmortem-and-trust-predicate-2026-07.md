# The Recovery Problem: Did Account Recovery Really Kill E2EE Consumer Products, and What Should Drystone's Trust Predicate Learn From the Bodies?

author: Research agent (commissioned; claude.ai deep research, ~275 sources)

date: 2026-07

status: draft for review — NOT legal/security advice; informs but does not resolve the recovery-custody decisions

---

`Commissioned research deliverable, filed 2026-07 (content-faithful to the delivered report). It
tests the hypothesis that account-recovery friction killed dead E2EE consumer products, and derives
design lessons for Drystone's recovery/key-custody **trust predicate** — the "I9" open decision (who
may trigger release of a sealed key share when a person loses every device). Directly informs the
recovery-architecture decisions **A2 / A12** (ROADMAP_TODO; blind-relay vs revocable-delegate,
quorum and/or delegate) and the custody model in `beta/drystone-spec/` (§2.8, open-threads);
companion thinking in `thinking/governance-and-survivability.md` and `thinking/multi-device.md`. The
staged recommendation below is **surfaced, not resolved** — the default predicate + UX is the user's
call.`

## TL;DR

- **The hypothesis is refuted as a cause of death, but confirmed as a design tax.** The named E2EE consumer products that died (Keybase, Skiff, Wickr Me, Peerio, pEp, Wire's and Meta's consumer arms) died overwhelmingly of acquisition, strategic abandonment, and abuse pressure, not of recovery friction. But the survivors (Signal, WhatsApp, Apple, Proton, Matrix) all paid a heavy, ongoing engineering and custodial tax specifically to solve recovery, and the adjacent crypto-wallet domain shows recovery failure is catastrophic where there is no operator to catch you.
- **Recovery is a locksmith problem: whoever can drill your safe is your real security model.** Every shipped model is a different answer to "who is allowed to drill" — nobody (paper key), a hardened server enclave (Signal SVR, WhatsApp HSM), a trusted person (Apple/Proton recovery contacts), a quorum of guardians (Argent), or an admin (enterprise). Each answer buys convenience at the cost of a new attack surface.
- **For Drystone's default predicate the evidence points to a hybrid: a quorum-of-contacts recovery reusing the proven k-of-n counting, hardened by a mandatory wall-clock delay with owner-veto, plus a paper key as the sovereign escape hatch — and crucially, recovery must be OFF by default and opt-in, because the measured usability literature says most people cannot be trusted with a bare root secret, yet the abuse literature says any always-on recovery path is an always-on attack path.**

## Key Findings

1. **The bodies mostly died of business, not cryptography.** Keybase was acqui-hired by Zoom in May 2020 and left a "zombie." Skiff was bought by Notion in February 2024 and sunset. Wickr Me was killed by Amazon after a child-abuse-material scandal. Peerio was acquired by WorkJam. pEp simply ran out of operational life. In none of these post-mortems is "users couldn't recover their accounts" named as the cause of death. Recovery friction is a contributing wound to adoption, not the murder weapon.

2. **The survivors all built a custodian they swore they didn't have.** Signal introduced a PIN backed by Secure Value Recovery (SVR) on Intel SGX in 2020, triggering a real community revolt; it has since evolved to SVR3, splitting trust across three heterogeneous hardware enclaves on three clouds (Intel SGX on Azure, AMD SEV-SNP on Google Cloud, AWS Nitro) with a 2-of-3 threshold. WhatsApp stores backup keys in HSMs. Apple's Advanced Data Protection uses recovery contacts and 28-character recovery keys. Proton uses a BIP39 recovery phrase. Each is a bargain with custodial drift.

3. **The money domain proves the stakes of "nobody can drill."** A June 2020 Chainalysis report estimated roughly 3.7 million BTC — about 20% of supply — had not moved in at least five years and is likely lost. Social-recovery wallets (Argent, championed by Vitalik Buterin) emerged specifically to fix this — but even they carry documented griefing, collusion, and relationship-churn failure modes.

4. **The academy has measured that ordinary people cannot be trusted with a paper key.** From "Why Johnny Can't Encrypt" (1999) forward, usable-security research consistently finds most users fail at raw key management; social recovery reduces the memorability burden but introduces relationship-harm and availability failures the literature explicitly names.

5. **The abuse ledger is symmetric: every recovery path is an attack path.** SIM-swap fraud is the canonical demonstration that a recovery channel trusted by many systems becomes a skeleton key. Coercion ("recover your spouse's account"), insider risk at custodians, and guardian collusion all have analogues or documented cases.

6. **The enterprise variant openly abandons the E2EE claim.** Organizations reconcile admin recovery with "encryption" by switching to customer-managed keys (BYOK/EKM) or client-side compliance capture, which is not the same trust model as consumer E2EE.

## Details

### The empty password-reset email

Start with the thing that isn't there. In an ordinary consumer product, the account's court of last resort is a link in your inbox that says "reset your password." That link exists because the operator holds a copy of the keys to your house. In an end-to-end-encrypted (E2EE) system — one where data is encrypted on your device such that even the operator cannot read it — that link cannot exist by construction. If the operator could email you back into your account, the operator could also read your messages, and the "end-to-end" promise would be a lie.

So the E2EE designer faces a problem with no free move. The useful metaphor is the locksmith. You have a safe. The lock is genuinely good. But the same property that keeps a thief out — that nobody but you can open it — means that when you lose your key, nobody but you can open it either. Recovery is the question of who, if anyone, you pre-authorize to drill the safe. Hire a locksmith on retainer and you sleep easier, but that locksmith can be bribed, coerced, or impersonated for the rest of the year. Hire nobody and the safe is perfectly secure and occasionally a tomb. (Where the metaphor breaks: a physical safe can always be drilled with enough time and a diamond bit; a well-built E2EE lock genuinely cannot, so the "locksmith" in software is not someone who defeats the lock but someone pre-issued a spare key or a share of one. Hold that distinction; it matters for Drystone.)

Drystone's open design decision, internally "I9," is exactly the choice of who may drill. The lock is already built: threshold shares split across independent trust domains, with a BIP39 paper key in existence. What is undecided is the *trust predicate* — the rule for who may trigger release of a sealed key share when a person loses every device. Because there is no reset email, whoever satisfies that predicate is functionally the account's supreme court. This essay tests one hypothesis against the historical record: *every E2EE consumer product that died, died mostly at account recovery.*

### The taxonomy, told as characters

Before the autopsies, the cast. Each recovery model is a different answer to the locksmith question, and each assumes a trust it does not always admit to.

**The Hermit: nothing, or paper-key-only.** The user holds a single secret — a seed phrase or a long recovery key — and no one else can help. At loss time, the experience is binary: you have the paper, or you have a tomb. Proton's model leans here: a 12-word BIP39 recovery phrase that "doubles as an encryption key, so it unlocks your account and decrypts your data at the same time," in Proton's own words, but "if you don't have the file or the phrase, then sadly, your encrypted data cannot be recovered." Stated trust: none. Real trust: you are trusting yourself, and the usability literature says that trust is frequently misplaced.

**The Vault Keeper: PIN-hardened server-side vaults.** The user remembers a weak secret (a PIN); a hardened server component enforces a strict guess limit so the weak secret becomes strong. Signal's Secure Value Recovery and WhatsApp's HSM-backed Backup Key Vault are the exemplars. Signal's own description: SVR "introduces a computer that even Signal can't hack" via Intel SGX enclaves. WhatsApp stores the backup key in a "hardware security module (HSM) — specialized, secure hardware." Stated trust: none, because the enclave rate-limits even the operator. Real trust: you are trusting the hardware vendor (Intel, or the HSM maker) and the operator's honest operation of it, which is precisely what the Signal revolt was about.

**The Family Friend: recovery contacts / social recovery.** The user pre-designates trusted people who can collectively help. Apple's Advanced Data Protection recovery contacts and Argent's guardians are the archetypes. Apple splits keying material so that "at recovery time, after the user's device successfully obtains both the Recovery Contact Packet from their Recovery Contact and the AES key from Apple, it can combine the two." Stated trust: distributed, no single party. Real trust: your friends' availability, honesty, and their own device security — plus, quietly, Apple, which holds one of the two shares.

**The Concierge: platform keychain sync.** The user's keys ride along invisibly across their own devices via iCloud Keychain or Google's equivalent. At loss time, a new device signed into the same account inherits the keys. Trust: the platform's device-authentication and its enclave.

**The Notary: institutional or admin escrow (the enterprise variant).** An organization holds recovery power over its members' accounts. This is the dispatch-desk case, examined below; it usually stops being E2EE in the consumer sense.

**The Timelock: time-delayed recovery.** Any recovery action is subject to a mandatory waiting period during which the true owner can veto. Argent's guardian recovery imposes a delay (documented as 36 hours historically, and a 48-hour window in its current guardian flow) precisely so "unwelcome changes can be interrupted." Trust: your own vigilance during the window, plus a clock.

**The Hybrid Tiers:** most real survivors stack several. Proton alone offers a recovery phrase, recovery file, device backup, email/SMS reset, and (newer) emergency-access contacts. The stacking is the tell: no single model is trusted to stand alone.

### The bodies, case by case

Now the coroner's work. The hypothesis predicts recovery friction on the death certificates. It is largely absent.

**Keybase — died of acquisition.** Zoom acquired Keybase in May 2020, mid-pandemic, explicitly to buy E2EE expertise for Zoom's video product. Keybase's own founder Max Krohn framed it as bringing encryption to hundreds of millions. The community verdict, per one detailed retrospective, is blunt: "Since the ink dried on that acquisition, Keybase has been a zombie... active feature development is completely dead." Cause of death: acqui-hire and strategic redirection. Recovery friction: not mentioned. If anything, security architect Alex Stamos's defense of the deal — "The whole point of the Keybase design is that you don't have to trust who owns their servers" — is a statement about the *lock*, not the *locksmith*.

**Skiff — died of acquisition, twice over.** Skiff reached almost 2 million users by November 2023 (17 months after launch), having raised $14.2 million over two rounds from investors including Sequoia Capital, former Stanford president John Hennessy, former Yahoo CEO Jerry Yang, and Balaji Srinivasan. Notion announced the acquisition on February 9, 2024, and the service mostly shut down on August 9, 2024. The most incisive post-mortem is mechanical and has nothing to do with users failing at recovery: "An agent that reads, sorts and answers your email has to be able to read your email. End-to-end encryption is built to stop exactly that. So the moment Notion's direction became AI on top of the inbox, Skiff's encryption was already incompatible with the destination." The lesson the outlet draws: "Encryption that depends on one company surviving and keeping its strategy is a promise with a shelf life." Cause of death: acquisition and strategic incompatibility. Recovery: absent.

**Wickr Me — died of abuse pressure.** Amazon (which acquired Wickr in 2021) announced in November 2022 it would shut the consumer app Wickr Me by December 31, 2023, to "concentrate Wickr's focus on... business and public sector customers." The context reporters foregrounded was an NBC News investigation identifying "72 court cases from the past five years in which the defendant allegedly used Wickr" to trade child sexual abuse material. Cause of death: abuse pressure plus a pivot to enterprise. Recovery friction: absent.

**Peerio — died of acquisition, but with a recovery-relevant ghost.** Peerio was acquired by WorkJam in January 2019 and shut on July 15, 2019, wiping all user data. The interesting archaeology predates the death: co-founder Nadim Kobeissi left in 2016 alleging that Peerio planned to sell enterprises a version "that had administrative controls" allowing "their boss to recover user accounts and decrypt/read their messages." Kobeissi's warning — that "any backdoor could be exploited externally and amounted to no end-to-end encryption at all" — is precisely the institutional-recovery tension. So recovery *design* fractured the founding team, but recovery friction did not kill the product; acquisition did.

**pEp (pretty Easy privacy) — died of exhaustion.** Per Wikipedia, "As of January 2024, the company overseeing p≡p is not operational. Its website no longer functions, and development of the system has ceased." A March 2021 episode of paid fake reviews surfaced. This looks like slow commercial failure and reputational decay, not a recovery cliff.

**Wire and Meta's consumer arms — deprioritized, not recovery-killed.** Wire pivoted hard to enterprise ("no key escrow, no backdoors" for business), and Meta announced it would end Instagram DM E2EE after May 8, 2026, with the stated reason: "Very few people were opting in to end-to-end encrypted messaging in DMs." That is an adoption story, not a recovery story — though it is adjacent, because opt-in friction (of which recovery ceremony is a part) suppresses the numbers that make E2EE features survive budget reviews.

**The verdict from the morgue:** across the named bodies, the recurring causes are acquisition (Keybase, Skiff, Peerio), abuse-driven shutdown (Wickr), commercial exhaustion (pEp), and adoption starvation (Meta consumer E2EE). Recovery friction is a chronic condition that suppressed adoption and occasionally fractured teams (Peerio), but it is not the acute cause on any death certificate I could source.

### The survivors' bargains

If recovery didn't kill the dead, what did it cost the living? Here the hypothesis inverts into something truer: recovery is the tax the survivors pay to stay alive, and every one of them paid it in custodial drift, support burden, or user revolt.

**Signal: the required case study.** In 2020, Signal rolled out a PIN backed by Secure Value Recovery so users could recover their profile, settings, and contacts. The cryptographic goal was elegant: let a human-memorable 4-digit PIN protect data against brute force by having an SGX enclave rate-limit guesses. As Signal put it, SVR "introduces a computer that even Signal can't hack." The backlash was immediate and came from serious people. Johns Hopkins cryptographer Matthew Green warned that SGX "has known security vulnerabilities" and objected to the rollout being opt-out rather than opt-in: "If it's opt-out, you get more of your current users using it. But it's not clear that people are making an informed choice about their security." Users on the Signal forum and Twitter revolted; one widely shared complaint was simply "I didn't know what the PIN was for and never got around to figuring it out." Independent researcher Wladimir Palant's assessment was harsher still: "If you ask me, Signal should give up the complexity of their SGX-based solution. Instead, they should enforce strong passwords."

Signal's founder Moxie Marlinspike defended SGX publicly ("I think all the attention SGX is getting is great") but conceded ground: Signal committed to shipping an option to disable PINs for advanced users and added the "Disable PIN" advanced setting, which turns off SVR at the cost of losing contacts on reinstall. Signal did not abandon the architecture; it hardened it. SVR2 simplified the SGX design, and by 2024 SVR3 distributed trust across three heterogeneous enclaves — Intel SGX on Azure, AMD SEV-SNP on Google Cloud, and AWS Nitro — with a 2-of-3 threshold, described by its Signal-employee authors as "the first deployed secret key recovery system to split trust across heterogeneous enclaves managed by different cloud providers," so that an attacker must break all three trust domains. Signal characterizes recovery as "a rare operation" and reports that "SVR3 costs $0.0025/user/year and takes 365ms for a user to recover their key," at a capacity for over 500 million users. What it cost Signal: years of engineering, a public trust crisis, and the permanent awkwardness of running exactly the kind of trusted server component its brand is built on distrusting.

**WhatsApp: the HSM bargain at scale.** In 2021 WhatsApp shipped opt-in E2EE backups: a randomly generated 64-digit key, optionally protected by a user password stored in an HSM-based Backup Key Vault. A peer-reviewed security analysis of the protocol notes the HSM "can be programmed once with code and then 'locked'... This enables even the protection of a protocol against corruption of the party running the HSM." The cost: WhatsApp had to build "an entirely new system for encryption key storage," and the feature is off by default — meaning most users never get its protection, the same adoption tax that starved Meta's Instagram E2EE.

**Apple ADP: the recovery-contact and recovery-key bargain.** Apple's Advanced Data Protection makes recovery the user's problem by design: "With Advanced Data Protection turned on, Apple doesn't have the encryption keys needed to help you recover your end-to-end encrypted data." You must set up a recovery contact or a 28-character recovery key first. The friction is real enough that Apple explicitly warns a recovery key must be entered carefully because you may get very few attempts, and "if you can't provide your recovery key, you'll be locked out of your account permanently." An independent analysis of the recovery-contact scheme concluded it is "well-engineered" but that Apple's claim "your recovery contact can't access your data" understates an "elevated risk that your recovery contact would be able to gain access." Custodial drift hides here: Apple still holds one share of the recovery-contact secret.

**Matrix/Element: the usability reputation.** Matrix's cross-signing plus Secure Secret Storage (SSSS, "4S") lets users back up message keys to the server encrypted under a recovery passphrase or a printed security key. The documentation's own warning captures the stakes: "Losing the room keys will result in the loss of your messages... No admin can help with that." The usability reputation is poor: university help desks publish multi-step recovery rituals involving "red buttons," cache clears, and manual key re-imports, and Element's own changelog lists dozens of cross-signing verification bugs. A peer-reviewed cryptographic analysis (2023) even found "practically-exploitable" vulnerabilities in Matrix's key-sharing. The cost: a support burden and a reputation for confusion that suppresses adoption among non-experts.

**Proton: the recovery-phrase-plus-tiers bargain.** Proton is candid about the tradeoff: "This is good for your privacy but can make data recovery difficult if you forget your password." Its answer is a stack — recovery phrase, recovery file, device backup, email/SMS reset (which resets the password but leaves data locked), and contact-assisted recovery for up to 5 friends. Proton's support team "regularly receives requests to reset a forgotten password," and its honest answer is that some recovery methods are single-use and, without them, "your encrypted data cannot be recovered."

The pattern: none of the survivors escaped the locksmith. They hired one and spent enormous effort making him trustworthy-ish. The cost was engineering, support load, and — in Signal's case — a public credibility fight.

### The money domain's hard numbers

Cryptocurrency self-custody is the recovery problem with money bolted on, and because there is usually no operator to catch a falling user, the failure rate is visible and brutal. The most cited figure comes from a June 2020 Chainalysis report: roughly 3.7 million BTC — about 20% of supply — had not moved in at least five years and is likely lost. Estimates vary: some analysts put current lost supply at 1.5 to 1.8 million BTC (wallets untouched since 2014 or earlier), others as high as 3.8 to 4.2 million. The uncertainty is inherent, because a dormant wallet could hold a lost key or a patient holder. Buterin's own framing, citing an analysis that "1500 BTC may be lost every day," is that "an ecosystem whose only answer to losses and thefts is a combination of 12-step tutorials, not-very-secure half-measures, and the not-so-occasional semi-sarcastic 'sorry for your loss' is going to have a hard time getting broad adoption."

His January 11, 2021 essay, "Why we need wide adoption of social recovery wallets," is the field's foundational recovery document. The design he championed: a signing key for daily use plus a set of guardians who, at some threshold (e.g., 3-of-5), can rotate the signing key if it is lost — guardians who "cannot access your funds individually." Argent shipped this at scale, surpassing 500,000 registered users as of 2024, with a time-locked recovery window so unwanted recoveries can be cancelled; its off-chain recovery deliberately transfers the key-encryption-key only after a 48-hour delay so that a compromised iCloud account cannot complete a silent takeover.

But the money domain also catalogued social recovery's failure modes precisely because money attracts adversaries. The documented problems: guardian **availability** ("you may have to find people you can bestow sufficient trust and should know how crypto wallets work"); **relationship churn** ("a user's social circle morphs over time, and periodically evaluating whether a relationship remains trustworthy can be inconvenient"); **collusion** (the wallet "makes no guarantees for preventing collusion among guardians if they identify each other"); and **griefing/extortion** ("Guardians, whether individually or as a group, can lock your wallet and extort the user in a maneuver known as a 'griefing attack'"). The lesson the field learned, in Buterin's words, is that "security without usability is useless" — but the corollary, learned the hard way, is that usable recovery re-imports the very human adversaries cryptography was meant to remove.

### What the academy measured

The usable-security literature is unusually clear here, and it starts with a single devastating result. In "Why Johnny Can't Encrypt" (1999), Whitten and Tygar ran a lab study of PGP 5.0 and found that "only 4 out of 12 participants were able to correctly sign and encrypt an email message in 90 minutes; and one quarter of them accidentally sent the secret email in clear text." Their conclusion — "PGP 5.0 is not usable enough to provide effective security for most computer users, despite its attractive graphical user interface" — established that key management is a distinct, hard usability problem. The 2006 follow-up ("Why Johnny Still Can't Encrypt") found key management "was still a challenge for users" seven years later; a 2015 study ("Why Johnny Still, Still Can't Encrypt") found it still open after fifteen. The measured evidence on whether ordinary users can be trusted with a bare paper key is, essentially: mostly no.

On social recovery specifically, the recent literature names the failure modes formally. A 2025 arXiv study on social vault recovery observes that naive social recovery imposes a "memorability burden" (users must remember who holds their shares) and creates "social sensitivity of the choice of trustee set" — non-chosen contacts "might feel slighted." The threshold construction itself is well understood: t-of-n secret sharing means "even if the user loses access to her key, any qualified subset of guardians can help restore it," while "any t or fewer learn nothing." The tension the academy formalizes is exactly Drystone's: raising the threshold improves security against collusion but worsens the availability problem (more guardians must be reachable at once), and the CryptPad Blueprints project notes the practical wall bluntly — "using secret sharing is only possible when you can guarantee every participant can play their part—which isn't very practical."

### The abuse ledger

Every predicate class has an attack surface, and the abuse ledger is symmetric to the recovery ledger: whatever lets a legitimate user back in lets an adversary in through the same door.

- **The Vault Keeper (server enclave)** is attacked at the hardware root. The Signal SVR debate was precisely about SGX side-channel and attestation attacks (SGAxe, and the general concern that a malicious enclave could be attested with keys extracted from Intel). SVR3's 2-of-3 heterogeneous-enclave design is the direct mitigation: break one vendor and you still have nothing.
- **The Family Friend (recovery contacts / guardians)** is attacked by coercion and collusion. The "recover your spouse's account" scenario is the coercive analogue: a person with physical proximity and social leverage can pressure guardians. Argent's own literature names the griefing attack.
- **The Concierge / phone-number recovery** is attacked by SIM swap, the best-documented real-world case. SIM-swap fraud redirects a victim's number to an attacker's SIM, after which the attacker performs "password reset abuse, recovery flow hijacking," draining bank and crypto accounts "in minutes." One law firm reports handling "more than 130 consumer arbitrations against major mobile carriers, including Verizon, T-Mobile, and AT&T." SIM swap is the canonical proof that a recovery channel trusted by many systems becomes a skeleton key.
- **The Notary (admin escrow)** is attacked by insider risk — the boss, or a compromised admin, is a standing single point of failure, which is exactly Kobeissi's Peerio objection.
- **The Timelock** is attacked by denial (an attacker who repeatedly triggers recovery forces the owner into permanent vigilance) but it is also the strongest single mitigation, because it converts a silent theft into a noticed one.

The through-line: the delay-based design is the one abuse mitigation that helps almost everywhere, because it changes recovery from an instantaneous silent event into a contestable, observable one.

### The institutional variant

When E2EE products are sold to organizations, the "end-to-end" claim quietly changes shape. The honest ones say so. The industry pattern: "Most 'enterprise' platforms resolve this tension by using customer-managed encryption keys (BYOK/EKM) rather than true E2EE, allowing the organization itself to access message content for eDiscovery while blocking the vendor from reading it." Microsoft is explicit that E2EE "is not compatible with certain service-side functionality such as recording, transcription... [and] eDiscovery," and offers "Customer Key" as the escrow-flavored middle ground. Wire markets "no key escrow, no backdoors" for business but provides client-side compliance capture instead. The procurement reality is that regulated buyers require eDiscovery, legal hold, and retention — capabilities fundamentally at odds with "no one but the user can decrypt." So the enterprise answer to reconciling admin recovery with E2EE is usually to redefine the encryption model (BYOK, client-side capture) rather than to solve recovery within true E2EE. This is the dispatch-desk case's warning to Drystone: the moment an institution needs guaranteed access, the trust predicate has stopped being center-free.

## Recommendations

Drystone's architecture is already decided: center-free, no protocol-level privileged party, the predicate is per-deployment/per-persona policy. The task is the *default* policy and its UX. The evidence supports a staged, opt-in, delay-hardened hybrid.

**Stage 0 — Ship the Hermit as the floor, but do not celebrate it.** The BIP39 paper key already exists; keep it as the sovereign, always-available escape hatch. This is what Proton, Apple (recovery key), and every wallet fall back to. **But the academy's verdict is that most users cannot be trusted with a bare paper key**, so the paper key must never be the *only* option presented, or Drystone reproduces the Johnny result at scale.

**Stage 1 — Default recovery OFF, opt-in ON, framed as a deliberate choice.** The single clearest lesson from the Signal revolt is procedural, not cryptographic: Matthew Green's objection was that the feature was opt-out, so "it's not clear that people are making an informed choice." Make recovery enrollment an explicit, opt-in ceremony with plain-language stakes. The benchmark that would change this: if telemetry shows opt-in enrollment below a threshold where lockouts become a support crisis, consider a guided default — but never a silent opt-out.

**Stage 2 — Make quorum-of-contacts the recommended default predicate, reusing the protocol's k-of-n counting.** This is the candidate that best fits Drystone's existing machinery and the field's proven consumer model (Argent at 500k+ users, Apple recovery contacts). Recommended default shape, grounded in the secret-sharing literature: a modest threshold (e.g., 2-of-3 or 3-of-5) chosen to balance collusion resistance against the availability wall the CryptPad and arXiv work document. Guardians should not be able to discover one another (Argent's anti-collusion property, echoed in Buterin's essay).

**Stage 3 — Wrap every recovery in a mandatory wall-clock delay with owner-veto.** The provenance plane forbids trusted timestamps, but recovery is a utility-layer ceremony, so wall-clock delays are admissible there — and this is the single highest-value abuse mitigation in the entire record. A 48-hour delay (Argent's current window) converts a coerced or colluding recovery from a silent theft into a contestable, observable event. This directly defeats the SIM-swap-style "instantaneous silent takeover" and blunts the "recover your spouse's account" coercion, because the owner is notified and can cancel. The delay hurts exactly one legitimate case — the user in a genuine emergency who needs access *now* — which is why the paper key (Stage 0) must remain the instant, delay-free sovereign path.

**Stage 4 — Offer a minimal issuer/custodian only as an optional guardian slot, never as the protocol center.** Some deployments (and some personas who lack technically capable friends — Apple's and Argent's documented gap) will want an institutional guardian. Permit it as *one share among several*, subject to the same threshold and delay, so it can never unilaterally drill the safe. This preserves center-freeness while serving the user who has no Family Friend.

**What would change these recommendations:** (a) If Drystone's telemetry shows guardian availability failures dominate (users unable to assemble their quorum at loss time), lower the threshold or add a hardware-guardian slot (the Braavos secure-enclave-as-guardian pattern). (b) If griefing/denial attacks on the delay window become common, add rate-limiting and owner-authenticated cancellation. (c) If a deployment is institutional/regulated and demands guaranteed access, recognize that it has left the center-free E2EE model and should be told so explicitly, per the enterprise-variant evidence.

## Caveats

- **This is a distinction between cause and tax.** The strongest finding — that recovery friction did not *kill* the named products — should not be read as "recovery doesn't matter." It matters enormously as an adoption suppressant and support cost, and the survivors prove it by how much they spent on it. The hypothesis is refuted on cause of death and confirmed on cost of living.
- **Numbers are soft where it counts.** Neither Signal nor WhatsApp/Meta publishes an account-lockout rate or an encrypted-backup adoption percentage that I could source. Signal calls recovery "a rare operation"; an academic model (SafetyPin) estimates roughly 0.39 recovery events per user per year. The Bitcoin-lost figure (~20%, ~3.7M BTC) is a widely cited Chainalysis estimate with real methodological uncertainty, not a measurement.
- **Some sources are secondhand and labeled as such.** The Keybase "zombie" characterization and several shutdown framings come from reporting and retrospectives, not primary post-mortems; the Marlinspike "disable PINs" commitment is a Twitter statement captured by tech press (Slashdot, VICE), not a signal.org blog post. Where a claim rests on paraphrase or secondary reporting, I have flagged it. I could not locate a dedicated signal.org blog post announcing SVR2 or SVR3; those design details rest on Signal's GitHub README and the Signal-coauthored OSDI 2024 paper.
- **Conflicts I did not synthesize away.** Kobeissi and Peerio's executives gave directly conflicting accounts of Peerio's admin-controls plans and the surrounding dispute; I have presented both. Lost-Bitcoin estimates range widely (1.5M to 4.2M BTC) and I have shown the range rather than picking one.

### The verdict on the hypothesis

**Grade: REFUTED as stated, REFINED to a defensible successor.**

The claim "every E2EE consumer product that died, died mostly at account recovery" does not survive contact with the death certificates. The bodies died of acquisition (Keybase, Skiff, Peerio), abuse-driven shutdown (Wickr), commercial exhaustion (pEp), and adoption starvation (Meta consumer E2EE). Recovery friction appears in the archaeology — it fractured Peerio's founders and it is part of the opt-in friction that starved adoption — but it is not the murder weapon anywhere I could source.

The refined claim the evidence *does* support: **"Recovery is not what kills E2EE consumer products, but it is the largest recurring tax on the ones that survive, and in the adjacent domain where no operator catches you (self-custody), recovery failure is directly catastrophic. The survivors all built a custodian they promised they didn't have; the question is never whether to hire a locksmith, only which one and under what delay."** For a center-free system, that reframes I9 from "how do we avoid a court of last resort" to "how do we distribute the court of last resort across enough independent parties, behind enough delay, that no single one of them — and no one who coerces a single one of them — can convene it alone."

## Sources

- Cointelegraph, "Zoom Acquires Encryption Startup Keybase" (May 2020)
- Zoom blog, "Zoom Acquires Keybase..." (May 2020)
- schulz.dk, "The Cryptographic Zombie: How Keybase Went from Privacy Darling to Zoom's Cleanup Crew" (Apr 2026) — retrospective, secondary
- Hacker News, "Zoom Acquires Keybase" thread (2020)
- TechCrunch, "Notion acquires privacy-focused productivity platform Skiff" (Feb 9, 2024)
- emailexpert, "Notion Mail Shutdown Shows the Risk of Email Products Becoming AI Features" (2026)
- TechRadar, "Skiff gets bought by Notion—another lost battle for privacy?"
- Wikipedia, "Skiff (email service)"
- TechCrunch, "Amazon-owned Wickr is shutting down its free encrypted messaging app" (Nov 2022)
- CNBC, "Wickr Me, Amazon's encrypted chat app, stops accepting new users" (Jan 2023)
- Engadget, "Wickr's consumer messaging app will shut down next year"
- Wikipedia, "Peerio"; Peerio Support, "Peerio Service Closure FAQs"
- Forbes, "Peerio Co-Founder On Why He Left The Company" (Jan 2016)
- Wikipedia / HandWiki, "Pretty Easy privacy"
- The Hacker News; Euronews — Meta ending Instagram E2EE (2026)
- Matthew Green, "Why is Signal asking users to set a PIN..." cryptographyengineering.com (Jul 2020)
- Wladimir Palant, "Does Signal's 'secure value recovery' really work?" palant.info (Jun 2020)
- Signal blog, "Technology Preview for secure value recovery"; "Improving Registration Lock with Secure Value Recovery" (2020)
- Fast Company, "Signal's newest feature shows why putting privacy first is so hard" (2020)
- Slashdot / VICE, Marlinspike "disable PINs" commitment (Jul 2020) — Twitter statement via press
- Signal Support, "Signal PIN" help article
- signalapp/SecureValueRecovery2 GitHub README (2023)
- Connell, Fang, Schmidt, Dauterman, Popa, "Secret Key Recovery in a Global-Scale End-to-End Encryption System" (SVR3), USENIX OSDI 2024, IACR eprint 2024/887
- Engineering at Meta, "How WhatsApp is enabling end-to-end encrypted backups" (Sep 2021)
- IACR eprint 2023/843, "Security Analysis of the WhatsApp End-to-End Encrypted Backup Protocol"
- Apple Support: "How to turn on Advanced Data Protection for iCloud"; "Account recovery contact security"; "Set up a recovery key"; "iCloud data security overview"
- disgruntledcode.com, recovery-contact analysis of ADP (Aug 2024)
- Matrix/Element: matrix.org cross-signing docs; TU-Dresden and KIT Matrix FAQs; element.io cross-signing release blog; IACR eprint 2023/485 "Practically-exploitable Cryptographic Vulnerabilities in Matrix"
- Proton Support: "Recover lost account data after a password reset"; "Proton Account recovery explained"; "Recovery phrase"; "How data recovery works with end-to-end encryption"; "Device data backup"
- Vitalik Buterin, "Why we need wide adoption of social recovery wallets" (Jan 11, 2021)
- Argent Support, guardian recovery documentation; Dynamic.xyz, "Recovery Methods in Wallets"
- eco.com support articles on social recovery and smart wallet recovery (2026)
- Chainalysis-derived lost-Bitcoin estimates via Ledger, Bitget, Financhill, Unchained Capital, cryptoassetrecovery.com
- Whitten & Tygar, "Why Johnny Can't Encrypt" USENIX 1999; Sheng et al. "Why Johnny Still Can't Encrypt" SOUPS 2006; "Why Johnny Still, Still Can't Encrypt" arXiv 1510.08555
- arXiv 2507.19484, "Towards the ideals of Self-Recovery and Metadata Privacy in Social Vault Recovery"; SafetyPin, Dauterman et al., arXiv 2010.06712
- IACR eprint 2025/2089, "Traceable Bottom-Up Secret Sharing..."; Springer chapter on community social key recovery
- CryptPad Blueprints, "Social Secret Sharing"
- Wikipedia, "SIM swap attack"; iVerify; Group-IB; Proofpoint; Dilendorf Law Firm on SIM-swap
- secumeet.com, "Top 15 Secure Chat Platforms for Enterprises"; Wire vs MS Teams; Microsoft Teams encryption blog (Jun 2025); CS Disco and Consilio on ephemeral-messaging eDiscovery
