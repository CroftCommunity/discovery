# The Achilles Heels of Group Chat

author: research pass, run against the lineage-based model

date: 2026-06-14

A note before the findings. You asked for adversarial, sourced, and honest over reassuring. The single most important thing this pass turned up is that your worry in Question 2 is not paranoia, it is the consensus view of the people building decentralized MLS. The ordering authority is not a dirty secret you might be hiding. It is a named, published, unsolved problem, and the part you are most worried about (returning a forked group to one agreed state) is explicitly called out as unsolved even in the academic work that fixes the easier half. That reframes what you can honestly claim, so I have put it first in the head-to-head.

I have separated protocol facts from product facts where it matters, and flagged the partisan sources (Wire's anti-Matrix posts are marketing, not neutral analysis).

---

## Part 1: The field's recurring failure modes, and why each is structurally hard

### 1. Multi-device

The root difficulty is not key management or history sync taken alone. It is that "a person" is a social abstraction and "a keypair" is the cryptographic primitive, and these two never line up cleanly. Every system pays for the gap somewhere.

Briar refuses the problem outright. Its own developers describe multi-device as a problem they have not solved because using one account from two devices creates synchronization complications, and an account is stored only on the one device with no transfer mechanism.

SSB could not make one feed span devices either. Each device must keep its own append-only feed because two devices writing to one log forks it, and a forked append-only log is a protocol violation, not a merge. Fusion Identity was the attempt to paper over this by linking separate feeds into one logical identity at the presentation layer. It was specified but stalled before becoming the default experience, and the audit found that even the tombstone-and-redirect path required out-of-band human voting to decide which device to trust.

Signal solves it by centralizing. The server holds the linked-device list and brokers the sender-key distribution, which is exactly what buys the clean UX, and exactly what a serverless design cannot copy.

Keybase is the closest prior art to your model and worth studying hard. It treats each device as a distinct key in an append-only per-user sigchain, then layers a per-user key (PUK) on top so any device can read shared content. This is per-device-as-member done in production. The lesson for you is in how they made revocation safe, covered under failure mode 3.

So the map of approaches, and what each costs:

- Shared key across devices: simple, but divergence and the inability to revoke one device without rekeying everything.

- Primary plus server-linked: clean UX, but the server is now load-bearing for identity (Signal).

- Per-device-as-separate-member: revocation becomes natural, but the member list and tree carry every device, and something has to fold them back into one displayed person (Keybase, and your model).

- Transfer-not-sync: sidesteps live multi-device by making it a one-time handoff, then letting the two diverge (Delta Chat's second-device model, Session's seed sharing).

### 2. Device loss and identity recovery

This is the hardest usability-versus-security collision in the field, and every point on the spectrum either weakens the threat model or reintroduces a trusted party. There is no known third option.

- "You are just gone": Briar and SSB. Lose the device, lose the identity. Maximally safe, minimally usable. Briar is explicit that uninstalling or forgetting the password means no recovery.

- Mnemonic seed: Session. The recovery password is the private key. It is the only restore path and also a single stealable secret, and sharing it across devices is literally how Session does multi-device. Note the sharp edge: Session V1 encrypts everything under one long-term key that does not rotate unless you make a new account, which is why it has no forward secrecy at all. V2 (in development as of late 2025) is trying to re-add PFS and proper device management.

- Server-backed PIN: Signal. Recoverable, but a server holds the recovery-enabling material.

- Key backup: Matrix. Recoverable, but the backup is a new asset to protect and a new attack surface.

The structural reason is simple and unforgiving. Recovery means reconstructing secret-holding authority after the secret holder is gone. Either another party held a copy (trusted party reintroduced) or a human-memorable secret stood in for the key (weaker, stealable). Decentralization removes the party that every usable recovery story leans on.

### 3. Membership-change handling and group key agreement

Add or remove triggers a rekey, and the hard part is concurrency. "Two people changed membership at the same time while partitioned" is genuinely hard, not an oversight, because membership change in MLS is a state transition on a single linear epoch chain. Two valid commits for the same epoch are a fork, and the protocol has no merge operation.

How the main designs handle change, and what each gives up:

- Sender keys (Signal, WhatsApp): each sender distributes a symmetric key to current members; removal forces redistribution. Cheap to send, but removal security leans on the server coordinating who is current.

- Megolm (Matrix): per-sender ratchet shared with the group. Membership change means new sessions. It provides only partial (block-level) forward secrecy and no post-compromise security by itself; the application has to rotate sessions to limit exposure. (The "Megolm has no forward secrecy" line you will see is Wire marketing. The precise version is: forward secrecy in blocks, no PCS.)

- MLS (RFC 9420): TreeKEM gives proper FS and PCS and scales logarithmically, but it assumes a delivery service that orders commits. This is the assumption your whole design has to live with.

Keybase's solution to concurrent membership ordering is the most instructive. They needed to prove "key was used to change the team after it was provisioned and before it was revoked," an a < b < c ordering across two different sigchains. Their answer was not pure cryptography. It was an append-only structure plus a server that commits to one true view, plus server-side access control. They had your exact problem and reached for a trusted ordering party.

### 4. The ordering / consensus assumption

This is the center of the field, and the honest finding is blunt: decentralized group membership is fundamentally a consensus problem, and the always-needed ordering authority is the thing most "decentralized" group chat quietly smuggles in.

RFC 9420 requires commits to be sequenced. RFC 9750 (the architecture doc) says the delivery service can be abstract or peer-to-peer, but then the clients themselves must implement the delivery guarantees the DS would have provided. The requirement does not disappear when you remove the server. It moves onto the peers, who now need to agree, which is consensus.

The IETF's own Decentralized MLS draft states the tradeoff in plain terms. In decentralized settings, the only way to run a functional DS is to let members retain key material so they can process commits out of order. That retention violates MLS's deletion schedule and significantly reduces forward secrecy. The FREEK-based DMLS work recovers most of that FS loss. But here is the line that should stop you cold: even with those fixes, once a fork has been created, they do not help return members to a single agreed group state. There is, in their words, no one semantic for resolving it.

That is your "deterministic survivor selection" step, named by the people who study this, and marked unsolved.

### 5. History on join, backfill, and forward secrecy tension

Readable history fights forward secrecy directly, because forward secrecy is the property that old keys cannot decrypt old messages, and "let the new member read the backlog" requires exactly that capability to exist for someone. The systems split predictably: Signal and MLS lean toward no history on join (protect FS), while history-sharing designs keep a re-encryptable copy and accept the weaker property. You cannot have full backfill and strong FS in the same transcript. Something gives.

### 6. Scaling

SSB is the cautionary tale: unbounded append-only logs that every replicating peer stores forever, with partial replication and Fusion Identity arriving late as attempts to bound the cost. MLS scales the rekey better (logarithmic in group size via the tree), but the tree grows with members, and in your design it grows with members times devices. Replication overhead and state growth are where ambitious P2P designs quietly hit a wall well before the crypto does.

### 7. Onboarding and the social/cryptographic identity gap

Keypair identity is hard for normal people (SSB, Briar, Session all struggle with "your identity is a key you can permanently lose"), and phone-number identity is easy precisely because it outsources identity and recovery to the carrier and the centralized server, which is the privacy and centralization liability. Your DID lineage sits on the hard side of this line. Be honest that it does.

### 8. Governance and moderation

This is the least-studied failure mode and I agree it is an under-appreciated killer. The pattern across the field: moderation powers were modeled as server or admin privileges, and when you remove the server you have to encode "who can kick whom" into the protocol or social layer, where it becomes a live attack surface. SSB notoriously cannot delete or globally moderate content because there is no authority to enforce a takedown and every peer holds its own copy. Briar's private groups punt to "only the creator invites." Nobody in the pure-P2P space has a convincing answer to a captured quorum, ban evasion, or a malicious majority, because those are governance problems and the field treated them as access-control features.

---

## Part 2: Your model against each failure mode

Escaped / partially escaped / relocated / inherited.

**1. Multi-device — partially escaped, by relocation.** Per-device-as-member genuinely makes "my laptop is a member" a first-class concept instead of a hack, and counting lineages not leaves is the right instinct (it is what stops you from manufacturing a quorum from your own devices, which is a real attack). But you have relocated the cost into the member list and the lineage fold, and made that fold load-bearing for every client. Keybase shipped this shape; see the costs in Part 3.

**2. Recovery — inherited, and you know it.** You said you do not have a total-loss story and I am not going to let you hand-wave it. If every device under a lineage is gone, nothing anchors recovery, which puts you exactly where Briar and SSB are: gone. Options in Part 4, all of which cost you something.

**3. Membership change — partially escaped.** Treating partition forks as clean attributable forks rather than errors is a real and good idea, and "hard-stop and escalate to a human on genuine membership conflict" is more honest than auto-merging. But see failure mode 4: the enactment step is where the consensus you are avoiding comes back.

**4. Ordering / consensus — relocated, not escaped, and this is the load-bearing finding.** Your governance tree forks and heals; your MLS chain needs ordered commits; the binding rule says a governance op is enacted only when realized as an MLS commit. That enactment is a single linear-epoch transition. Two concurrent enactments are an MLS fork, and the IETF's own work says fork-to-single-state has no general resolution. So your "deterministic survivor selection" is doing the consensus work, and the question is whether it is doing it honestly. If the superpeer is what makes survivor selection deterministic and available, then yes, the superpeer is partially your ordering service. That is not fatal. Keybase's server played exactly this role and Keybase was a real product. But it changes the claim from "no ordering authority" to "an ordering authority that is cryptographically blind and optional, at the cost of staleness when absent." Claim that, not more.

**5. History on join — escaped by redefinition, honestly.** You sidestep the FS tension by never merging into one transcript and making backfill consensual and lineage-gated. This is a legitimate escape because you changed the requirement (no single transcript) rather than beating the tradeoff. The cost is that "complete history across my devices" is now explicitly a non-goal, which you have already accepted.

**6. Scaling — inherited, possibly amplified.** Per-device-as-member means tree and governance-log growth scale with devices, not just people. Device churn (every phone upgrade is an add plus a revoke) writes to the governance log forever. This is the SSB-shaped risk wearing a different hat.

**7. Onboarding — inherited.** DID lineage is keypair identity. It is on the hard side. No escape claimed or available.

**8. Governance — partially escaped, and this is your most original contribution.** Immutable genesis thresholds grounding the "who decides who decides" regress at the root is a genuinely good idea and more principled than anything in the pure-P2P field. But immutability is also a trap: a captured quorum under the genesis rules is permanent, because you fixed the rules at genesis specifically so they could not be changed. See Part 3.

---

## Part 3: The strongest case against your design, ranked

**1. The superpeer is your ordering service, and the pure-P2P tier is a demo.** The most likely way this fails is that everything works well with the superpeer and degrades to unusable without it, so the "two tiers" become one real tier and one marketing tier. The history here is brutal: every system that claimed P2P-or-server-your-choice found that the server tier was the only one anyone used (this is effectively SSB's pub-node story and Matrix's homeserver story). If survivor selection, rekey carrying, and snapshot storage all concentrate on the superpeer, you have rebuilt a delivery service and the honest framing is Keybase's, not Briar's. Test the no-superpeer tier early and adversarially, because it is where the design's central claim lives or dies.

**2. Immutable genesis thresholds cannot survive contact with a real group.** Groups change. The threshold that made sense at genesis (boot=2) becomes wrong when the group grows from 3 to 30, and you have deliberately made it unchangeable. A captured quorum, a founder who turns malicious, or a simple bad initial parameter is now permanent. Your fork-as-feature mechanism is the only escape valve (fork away from the captured group), but that means the answer to "our governance is broken" is "abandon the group and re-form," which is a heavy and lossy operation to make routine. The field's moderation failures (failure mode 8) teach that governance needs to be amendable under its own rules, and you have traded that away for the clean regress-grounding.

**3. Composition leaks at the lineage-credential-on-the-MLS-leaf seam.** You have six clean pieces (lineage identity, governance tree, MLS ratchet, CRDT history, consensual backfill, optional broker). The messiest seam is where the lineage credential has to ride on the MLS leaf so that "thresholds count lineages not leaves" can be evaluated. That means the MLS layer (which only knows about leaves and keys) has to carry and expose governance-layer identity, and your governance evaluator has to trust the binding between leaf and lineage that lives in MLS credentials. Two clean layers, one leaky coupling that both depend on. Ambitious P2P designs tend to die at exactly these seams, not inside the well-specified pieces. This is the one to prototype first because it is where the integration cost hides.

**4. Recovery has no anchor and you will be forced to add a trusted party under pressure.** Lowest of the four because it is a known gap rather than a hidden one, but real: the field shows that "no recovery" is acceptable only to activists and journalists (Briar's actual audience) and that every consumer-facing system eventually bolted on a recovery party. If you want anyone but high-threat users, you will face the same pressure, and the cleanest place to absorb it is a lineage-level social recovery (threshold of other lineages re-attest a new device), which reintroduces a quorum-of-trusted-parties at exactly the layer you wanted to keep pure.

---

## Part 4: Open questions and realistic options

**Total-device-loss recovery.** Realistic options, each with its cost:

- Social recovery at the lineage layer: a genesis-defined threshold of other lineages can attest a fresh device into your lineage. Cost: a quorum of contacts can collude to seize your identity, and it only works while the group is alive.

- Offline lineage root key held in cold storage (paper, hardware): cost: it is Session's seed phrase by another name, a single stealable and losable secret, and it sits outside your nice per-device story.

- Accept "gone": cost: you are Briar, usable only by people who accept that, which contradicts any broader ambition.

There is no option that keeps the threat model pure and gives normal users recovery. Pick which group you are for, and say so.

**Does survivor selection need the superpeer to be deterministic?** This is the question to answer before anything else, because the honest framing of your entire project depends on it. If two fully-partitioned peers with no superpeer can independently compute the same survivor epoch from the governance logs alone, you have something genuinely new. If they need the superpeer to break ties or to even learn that a fork happened, the superpeer is the ordering service. Write the survivor-selection function and check whether it is a pure function of the two logs or whether it needs an external tiebreaker. The DMLS work suggests the latter is likely, so prove yourself right or wrong here first.

**Amendability versus regress-grounding.** Can you allow genesis thresholds to be amended under a higher genesis-fixed threshold (e.g. "thresholds may change only by unanimous lineage consent") and still claim the regress is grounded? This might recover most of immutability's safety while fixing its brittleness. Worth designing before you commit to hard immutability.

**The lineage-credential-on-leaf binding.** Specify exactly what the MLS layer must expose and what the governance evaluator must trust, and treat that interface as a first-class threat-modeled boundary rather than an implementation detail. This is where a clean composition becomes a messy one.

---

## The Achilles heel you did not name

You asked what kills systems like yours that you are not even asking about. Based on the histories above, it is not a cryptographic failure. It is **governance-log noise from device churn making the member-list fold unmaintainable at the social layer.** Here is the shape of it.

Per-device-as-member plus consensual backfill plus forks-as-feature means the governance log accretes device adds, device revokes, lineage attestations, fork markers, and re-key enactments, forever, with no compaction authority (you removed the server that would have compacted it). Every client must replay and fold all of it to display "who is in this group right now." This worked for Keybase because Keybase had a server committing to one view and doing the heavy lifting. In your no-superpeer tier, every client does it alone, over a log that grows with people times devices times time times fork-count.

This will not show up in your experiments, because experiments run on small groups over short windows with few devices. It shows up at month eighteen, in a thirty-person group where everyone has upgraded phones twice and there have been four partitions, when a new member's client has to fold a five-thousand-entry governance log to render the member list and gets it subtly wrong, and now two members disagree about who is in the room. That is SSB's unbounded-log death and the field's governance-is-not-access-control lesson arriving together, at the one layer (the fold) you have made load-bearing for everyone. Put a synthetic high-churn, multi-partition, long-horizon group in your test suite now, because it is the thing that will be too late to fix once people depend on it.
