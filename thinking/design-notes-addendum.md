# Design Notes Addendum: Roll-Ups, Two Modes, and the Capabilities-Not-Rights Principle

author: design discussion capture

date: 2026-06-14

This addendum captures decisions and framing developed after the initial failure-mode analysis. It extends, and does not replace, the three prior files. It covers three connected ideas that emerged in sequence: splitting the governance log into settled history and live tail, running the system in two modes (superpeer-assisted and pure peer-to-peer), and the governing principle that ties them together.

---

## 1. The problem this addresses

The initial analysis flagged a sleeper risk: in pure peer-to-peer mode, every client has to replay an ever-growing governance log (every device added and removed, every fork, every rejoin) just to render the current member list, with nothing compacting it. This is the SSB unbounded-log death, landing at the one step every client depends on. It will not show up in short, small-group tests, and it becomes expensive to fix once people depend on it.

The response developed here turns a continuous, mandatory, grows-forever verification cost into a periodic, optional, bounded one.

---

## 2. Split the governance log: settled history and live tail

Separate the admin chain into two parts.

- Settled history: the portion of the past the group agrees is final. This gets compacted into roll-up checkpoints.

- Live tail: the recent, still-churning, possibly-forking portion. Not yet compactable.

Two distinct concerns ride on this split, and they fail differently, so they must be reasoned about separately.

### The display concern: solved, no tension

Folding settled membership into a current-state snapshot and keeping the churn history behind a "show more" affordance is a standard, safe UI pattern. The analogy is Git: `git log --oneline` by default, full detail via `git log -p` on demand. The member list renders from the rolled-up current node; the deep look-back is one tap away for the rare case (an audit, a dispute, "when exactly was this person removed").

There is no cryptographic tension on the display side. The noise isolation is legitimate and routine.

### The cryptographic concern: the real design decision

When a client accepts a roll-up instead of replaying the full chain behind it, something has to make that roll-up trustworthy without the replay. There are only three options, and choosing among them is the actual decision:

- Authority-signed checkpoint: the superpeer signs "as of epoch N, members are X." Cheap, but this makes the superpeer the source of truth for membership. This is the referee problem in a new coat, and it is the path of least resistance, so it is the one to consciously avoid.

- Threshold-signed checkpoint: a genesis-defined quorum of lineages co-signs the roll-up. Keeps it decentralized and is consistent with the genesis-thresholds model. Costs a live coordination event to mint each checkpoint, which reintroduces a small, infrequent, batched consensus moment.

- Accumulator or recursive proof: the roll-up carries a cryptographic proof that it is the honest fold of everything behind it, checkable in one shot without the underlying data. The only option that introduces no trusted party. Heaviest to build. Merkle Mountain Ranges are the tractable end (used by Mina, some certificate-transparency systems); recursive SNARKs are the ambitious end. Real engineering, not a config flag.

### What roll-ups do not fix: forks

Compaction makes the honest, settled past cheap to verify. It does nothing for a forked past. You cannot roll up across an unresolved fork, because there is no single state to roll up to. So compaction only applies to settled, un-forked segments, and the live tail (the churning, possibly-forking part) is exactly what cannot be compacted yet. This is natural rather than a flaw, but it means the roll-up boundary must sit where the group agrees the past is settled, and "the group agrees" is itself a small consensus event.

### Honest framing of the roll-up

It converts a continuous, mandatory, unbounded verification cost into a periodic, optional, bounded one, at the price of a trustworthy checkpoint mechanism, and it applies only to the settled un-forked portion of history. Good trade. It moves the sleeper risk from fatal to managed.

The discipline: do not let "roll-up" quietly degrade into "the superpeer signs the current state." If the decentralization claim is to survive, checkpoints must be threshold-signed or accumulator-proven, not authority-signed.

---

## 3. Two modes, one convergence requirement

The system runs in two modes that reach the same governance ends by different paths.

- Superpeer-assisted: an always-on larger peer does floor-sweeping (compaction, roll-up minting, carrying rekey and revocation commits when human peers are not co-present) opportunistically, whenever it is available. Faster, less coordination pain.

- Pure peer-to-peer: no superpeer. Governance is harder and slower, and compaction becomes a deliberate periodic ceremony, but everything remains possible.

The binding rule is a convergence requirement: both modes must reach the same governance state. The superpeer is an accelerant, not a source of truth. The crucial constraint, in the design's own words: the superpeer does sweeping, but never sweeping that only it can do.

### The conformance test

For every superpeer feature, ask one question: is there a governance outcome reachable with the superpeer that is not reachable without it? Not slower, not more annoying, not more coordination. Unreachable.

If the answer is ever yes, that capability has secretly become referee-only and the design has leaked. For each thing the superpeer does, write down the no-superpeer path that achieves the same end, even if that path is "five lineages come online together and it takes a day." If you cannot write that path, the feature does not ship in the superpeer until you can.

### Compaction in pure-P2P mode

Floor-sweeping is real work needing a trigger and an authority to mint. Without the superpeer, the minter is a threshold of lineages, as a deliberate periodic event. This means in pure-P2P mode, compaction is itself a governance act subject to genesis thresholds. Consistent and elegant, but it must be stated plainly to users: periodically the group must collectively agree "the past up to here is settled," or the log grows unbounded again. The superpeer does not remove this need; it lets the group skip the ceremony when the superpeer happens to be reachable.

### The mode-toggle risk: keep the path warm

Real groups will toggle. A group runs superpeer-assisted for a year, the superpeer does all the sweeping, then access is lost (shutdown or partition). Now the group is in pure-P2P mode but has never run the threshold-checkpoint ceremony, because the superpeer always did it. Does the group have the muscle? Is the threshold-mint path exercised, or only theoretically present?

This is the failure mode where "possible in principle" rots into "possible but no code path has ever run it." Mitigation: run the pure-P2P checkpoint path on a schedule even when the superpeer is present, occasionally, to keep it warm. Same logic as testing backups by actually restoring them.

### Mode philosophy

Peer-only is the correctness floor and defines what is possible. The superpeer is a scenario optimization that makes the possible fast and painless but can never make the impossible possible. This inverts the usual P2P relationship: the server is not a crutch the design leans on and apologizes for, it is a bonus the design exploits when present. A cleaner story than Briar's purism or Signal's centralization.

---

## 4. The governing principle: different in capability, equal in rights

The framing that ties it all together: peers differ in capabilities but not in rights. Exploit the former when present; rest on the latter when it is not.

### Why this is load-bearing, not decorative

A bigger peer can do more (storage, uptime, throughput) but cannot decide more. The moment a capability difference becomes a rights difference, the design has leaked. This gives the conformance test its sharpest form: does this superpeer feature exercise a capability, or a right? Sweeping the floor faster is capability. Being the one whose signature settles who is in the group is a right. The first is fine; the second is forbidden.

### Grounding: data plane vs control plane

This is the distinction between the data plane (moving, storing, relaying, accelerating) and the control plane (deciding, authorizing, ordering). Capabilities live in the data plane; rights live in the control plane. The studied failures are mostly cases where a data-plane actor (an SSB pub, a Matrix homeserver, an MLS delivery service) quietly accreted control-plane authority because it was convenient. The principle is a precommitment against that drift, and it names the exact mechanism by which the others failed.

### Why it makes the blind superpeer coherent

"Equal in rights" is what makes a cryptographically blind superpeer coherent rather than merely a nice-to-have. If the superpeer had rights, blinding it would be self-defeating: you would be asking a decision-maker to decide on data it cannot read. Because it has only capabilities, blindness is natural: you do not need to read contents to relay, store, or sweep. The blindness and the rightlessness are the same commitment from two angles, and they reinforce each other, which is a sign the principle is real.

### The seam to guard: availability masquerading as a right

The subtle leak. If the superpeer is the only peer reliably online, and a governance action requires someone online to carry the commit, then the always-on peer becomes the de facto authorizer, not because it holds the right but because it is the only body present to exercise everyone's rights through. It did not take a right; it absorbed one through presence. This is the soft version of how "the server just relays" becomes "the server coordinates."

Defense: the no-superpeer path must stay exercised, so the group never structurally depends on one peer's presence to act, even when it usually uses that peer. Rights that cannot be exercised without one specific peer present are not held equally; they are escrowed to that peer. Keeping the pure-P2P ceremony warm is what keeps rights actually distributed rather than nominally distributed.

### The principle, tightened

Peers differ in capability, never in rights. Exploit capability when it is there; rest on rights when it is not. Guard the seam by ensuring no right can be exercised only through a single peer's presence, because availability is the back door through which capability tries to become authority.

---

## 5. Summary of current state

- The sleeper risk (unbounded governance log replayed by every client) has a managed answer: split the log into settled history and live tail, compact the settled part into verifiable roll-ups, and make deep look-back optional at the UI layer.

- Roll-up trust must come from threshold signatures or cryptographic proofs, never from an authority signature, or the superpeer becomes the referee. Roll-ups apply only to settled, un-forked history.

- The system runs in two convergent modes. The superpeer accelerates; it is never the sole source of any outcome. Conformance test: every superpeer feature must have a written no-superpeer path to the same end.

- Pure-P2P mode pays for independence with an occasional collective compaction ceremony, gated by genesis thresholds. This path must be exercised on a schedule even when the superpeer is present, so it does not rot.

- The governing principle is capabilities-not-rights: peers differ in what they can do, never in what they may decide. It is grounded in the data-plane / control-plane split, it makes the blind superpeer coherent, and its one guarded seam is availability quietly impersonating authority.

### Open items still to resolve

- Which roll-up trust mechanism to commit to (threshold signature now, accumulator or recursive proof later, or build toward the proof from the start).

- The deterministic survivor-selection question from the prior analysis is still the thing to settle before building: can two disconnected, superpeer-less clients independently compute the same surviving branch from their histories alone? The roll-up and two-mode work assumes the answer can be made yes; it has not yet been proven.

- Total-device-loss recovery still has no anchor. Unchanged from the prior analysis.

---

## 6. The social-vs-cryptographic division of labor

A posture shift developed after the sections above: stop trying to solve social complexity with clean technology. No cryptographic scheme, validated or not, can prevent an unexpected or undesired case from occurring. So the design goal is not prevention. It is recovery, plus an explicit continuum of adversarial posture.

### The continuum, not a fixed point

Groups fall somewhere on a continuum from prioritizing cohesiveness and inclusion to prioritizing member fidelity and exposure-resistance. This is not a fact the protocol should decide. It is one or more dials the group tunes.

Compared to the upstream research, this is a third position neither camp occupied:

- The purist camp (Briar, SSB) welded the dial to maximum fidelity and paid in usability: no recovery, no multi-device, gone means gone. Usable only by activists and journalists who accept the terms.

- The convenience camp (Signal, Matrix in practice) welded the dial to maximum inclusion and paid by putting a server in the control plane. "Decentralized" became a deployment detail rather than a property.

Refusing to weld the dial is novel relative to the corpus, and the research supports the premise: every system that made the protocol enforce a single answer to a social question either failed at it (SSB cannot enforce a takedown) or delegated it to a server. MLS explicitly defines key mechanics but not membership policy. The social layer kept leaking through the cryptographic one across all eight failure modes. Making crypto provide provenance and recovery while the group provides policy is closer to what the systems were forced into anyway, made intentional instead of accidental.

### The dial caution

Tunable dials reintroduce the regress that genesis-thresholds closed. A dial governing "how easy is it to remove someone" is itself capturable: a captured quorum turns exposure up and inclusion down to entrench itself. So "who can turn the dials, and is that itself genesis-fixed" must be answered. Some dials probably want to be genesis-fixed precisely because they govern the others.

### The guarantee actually made

The system's correctness guarantee is not "the group converges to one membership." It is "every party can always see a true, attributable history, and always has a clean exit." Those are different guarantees. The second is the honest one for a system that respects social governance. The failure mode here is not technical: it is claiming convergence while delivering legibility, which would land the design back in the exact gap the upstream systems fell into.

---

## 7. The one-mechanism unification

Merkle-style ancestry is the substrate, and several problems previously treated as separate are one operation: two histories that share an ancestor reconciling.

- Multi-device sync is two of my own devices converging on shared ancestry.

- Group merge after an inadvertent split is two branches converging on shared ancestry.

- Splinter is the same operation declined.

- Re-formation (the group minus the removers) is a branch off the shared ancestor that some members follow and others do not.

All of it is: find the common ancestor, replay forward, converge, stay local-first. The admin tier scoped to each group stays immutable, so ancestry is provable rather than asserted. When one mechanism explains things previously thought separate, that is a sign of a correctly factored architecture.

### The internal boundary: complementary vs contradictory

Merkle ancestry gives verifiable convergence on content. It does not give convergence on conflicting policy decisions. These are different, and the boundary is the "hard stop" case.

- Complementary divergence (multi-device, clean splits, non-conflicting concurrent change): the branches each hold a piece of the truth, the union is well-defined, convergence is automatic. "Agree on the ancestor, bring the groups back" is the literal algorithm. This is the commutative case, the same restriction that makes CRDTs work.

- Contradictory divergence (member ejected on one branch, re-added on the other): both branches are valid, ancestry is shared and provable, and they make opposite claims about a single fact. There is no correct answer to compute. The mechanism detects and presents the conflict with full provenance ("everyone can see and understand the issue") but cannot heal it. Healing is a human choosing a branch, or the branches staying separate as a resting state. This is the non-commutative case, where no auto-merge is valid.

The architecture is, in effect, a CRDT for the complementary cases and a human-escalation for the non-commutative ones, riding on one ancestry structure. The error to avoid is believing the ancestry structure makes the contradictory case auto-resolvable. It makes it detectable and provable, which is the most any structure can do and exactly what is needed.

### Immutable substrate, subjective view

The admin tier (membership operations, ancestry, votes) is immutable and append-only: that is what makes provenance provable. The view (which branch a client currently renders as "the group right now") is local-first and can differ between members during a split. Immutable substrate, subjective view. This is the SSB insight done right: SSB had immutable feeds but no principled story for diverging views; this design keeps immutable provenance and adds both a convergence mechanism for complementary cases and honest escalation for contradictory ones.

---

## 8. The closed system: every failure path terminates somewhere legitimate

The keystone is the trap door: fork and start new history, related to an ancestor or not, is always an available move. It sits under everything as the guaranteed escape, which is what lets every higher mechanism be partial without anything ever being truly stuck. A system with a guaranteed-available escape at the bottom can afford partial mechanisms above, because no case falls through.

The local-first view is what makes the trap door non-catastrophic. Even on divergent history, the view renders something palatable, navigable, and searchable (the code-fold model: present but folded, unfoldable on demand). A fork is not a cliff, it is a branch you can still live inside and search. The architecture's worst outcome is still a working chat with legible, searchable history. That is a high floor, and it is the specific thing SSB lacked, where divergent state was an unusable mess and forking therefore felt like breakage.

The final principle: hard breaks are always a social authoritative problem, never a cryptographic one. A hard break (a removal, a ban, a contested membership) is not a fact the protocol can establish. It is a judgment a social authority renders, and the protocol's only job is to give that authority a true, legible, searchable account to judge from. This is the category error the upstream research circled for eight failure modes and never named cleanly. Crypto serves the social decision; it does not make it.

### The completed division of labor, top to bottom

- Crypto provides immutable provenance and verifiable ancestry. The rigid substrate.

- The convergence mechanism auto-heals every complementary (commutative) divergence: multi-device, clean splits, non-conflicting concurrent change.

- The local-first view renders every state, convergent or divergent, as a palatable searchable experience, so no outcome is unusable.

- Contradictions are detected and presented with full attribution, never auto-resolved, because they are non-commutative and have no answer the tree can compute.

- Hard breaks are escalated to social authority, whom the protocol exists to inform, not replace.

- The trap door (fork and new history, always available) sits under all of it as the guaranteed escape.

Every failure path terminates somewhere legitimate. That is what makes the system closed.

### The sentence to be able to say without flinching

"My system guarantees that any two members can always determine their common ancestor and converge wherever their histories are complementary, and can always see a fully attributable account of where and why they are not, but it does not guarantee they end up in the same group, because that is theirs to decide."

If that sentence feels right, the architecture is sound. The spot to stay suspicious is any temptation to claim more than it.
