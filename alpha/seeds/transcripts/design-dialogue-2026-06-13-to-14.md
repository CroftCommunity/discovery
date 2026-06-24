# Raw transcript — GroupDynamics design dialogue (2026-06-13 to 06-14, verbatim)

source: the original design conversation, re-dropped 2026-06-15 to close the provenance gap.
verbatim. This is the richest single seed — the conversation that produced THESIS.md,
MULTI_DEVICE.md, SOCIAL_LAYER.md, and messaging-solutions-landscape.md (the four docs filed in
seeds/groupdynamics-unpacked/), and that ANALYSIS.md distilled. It also contains the two
generated prompts (already filed in seeds/generated-prompts/) and the scale-axis / two-regime
material that SOCIAL_LAYER.md's header promised but never had written in (sections 8–10).

> Inline assistant source citations from the research phase ("Delta Chat", "Wikipedia", etc.)
> are preserved as they appeared. "Searched the web" / "Created a file" markers denote tool
> use in the original session.

---

## Brief: Comparative Analysis of Messaging & Group-Chat Solutions

(The full research brief — Delta Chat, Secure Scuttlebutt, Signal, and other prominent systems
along usability/security/capability lines — is preserved verbatim as the task header of
seeds/generated-prompts/ context and produced messaging-solutions-landscape.md. Summary: a
research+analysis assignment to investigate prominent chat/group-chat solutions across three
axes and map what each teaches for building an encrypted local-first messaging stack;
deliverable = experiments/competitive-analysis/messaging-solutions-landscape.md with exec
summary, per-solution investigation, three axis tables + a "lesson for us" table, synthesis,
and sources; cite inline, mark [UNVERIFIED], separate protocol from product facts. Required
anchors: Signal, Delta Chat (Rust+iroh cousin), Secure Scuttlebutt (the cautionary tale);
also Matrix, Briar, Session, a mainstream baseline. Our stack = iroh P2P + optional blind
superpeer broker, AT Protocol DIDs, MLS via openmls per-epoch rotation, automerge CRDTs,
iroh-blobs media, public via atproto / private on encrypted P2P.)

---

## The dialogue

**[research phase]** I'll research the current state of these systems before writing, since
several have changed in ways that matter (Delta Chat's iroh integration, SSB's decline,
Matrix's MLS progress). [Multiple web searches: Delta Chat iroh usage; Delta Chat multi-device;
SSB current state; Manyverse "My last update" April 2024 + abandoned PPPPP; SSB fusion-identity
spec v1 limits; Matrix MLS (MSC4256/4244, arewemlsyet.com); Signal usernames + linked devices;
Briar/Session specifics; Session moved off Signal Protocol, dropped then re-added PFS via
Protocol V2 Dec 2025.]

Done — experiments/competitive-analysis/messaging-solutions-landscape.md (~6,900 words, four
tables). Key findings where I pushed back on the brief: Session premise was stale (now its own
Session Protocol; dropped PFS for years; reintroduced PFS + ML-KEM in V2 Dec 2025 — lesson:
don't trade forward secrecy for decentralization, which vindicates choosing MLS). Delta Chat's
iroh use is narrower than implied — ephemeral forward-secret realtime channels for webxdc
(joinRealtimeChannel(), random 32-byte topic id, lazy bootstrap) + a one-time local-network
"Add Second Device" QR transfer leaving devices independent; durable multi-device sync rides
SMTP deliberately (so the broker is de-facto mandatory for good multi-device, not optional);
iroh ignores proxy settings (privacy gotcha). SSB is in decline (Manyverse/PPPPP lead stepped
away Apr 2024; Patchwork deprecated; fusion-identity + partial-replication specs archived,
never finished — could add devices but only tombstone, never cleanly remove, and required
sharing a private key across devices). Honest verdict: broker+CRDT dissolves SSB's onboarding
and log-replay pain, MLS fixes the group-rekey/forward-secrecy gaps, but multi-device and
total-device-loss recovery remain unsolved in our design too — DID device-key
revocation-with-propagation is the exact operation fusion identity botched, and no recovery
anchor is specified. Briar (strongest threat model) simply refuses multi-device — a signal that
our claim to offer both is the claim others declined to make. [SimpleX [UNVERIFIED], deferred.]

> **How does delta chat do multi device securely and yet functionally?**

Short version: it doesn't do "multi-device" the way Signal or Matrix do. No live continuously-
synced multi-device; two separate mechanisms, and the security comes mostly from sidestepping
the hard sync problem. [Verified current mechanism.] Core choice: shared identity (your OpenPGP
private key IS your identity), not synced devices — copy the whole account, then run
independently. Mechanism 1 "Add Second Device" (iroh local transfer, QR carries the connection
secret out-of-band, iroh+QUIC, relay helps find but isn't trusted with content, no third party
holds the key; since v1.36 transfers the full account). Mechanism 2 backup export/import
(fallback). After either, devices don't stay in lockstep — ongoing self-sync rides SMTP
deliberately (a device can be offline for days; email queues reliably where opportunistic P2P
wouldn't). Costs: local-network dependency for the live transfer (documented friction); no live
convergence. For our stack: the SecureJoin protocol got an ETH Zurich formal analysis →
20 issues fixed in v1.44 (data point on how much scrutiny key-transfer flows need). Contrast:
Delta = one shared key + device independence; our DID+MLS = distinct per-device keys under one
identity with live convergent Automerge state — strictly more capable, but reintroduces the
synchronization/revocation complexity Delta sidestepped.

> **Wouldn't multi devices sync the same way in p2p as a non-same-user peer, so it's really
> down to crypto key provenance and attribution? Seems more solvable in p2p + automerge + iroh
> because peers are equal.**

You're onto something real but the framing collapses two problems that stay separate even when
peers are equal. Where you're right: transport + state-merge is symmetric — to iroh your laptop
and phone are two nodes; to Automerge two writes from your devices merge like writes from two
people (CRDT doesn't care about authorship for convergence). The thing that wrecked SSB
(one identity bound to one append-only feed, so a second device forks it irreparably) doesn't
exist for you — Automerge is multi-writer by construction. So "multi-device state sync" is not
the hard part. Where symmetry breaks (three surviving problems): (1) MLS doesn't see your
devices as equal peers — every leaf is a distinct member; adding your phone is an Add+Commit
that rotates the epoch and every other member sees N→N+1; reconciling MLS group state with
Automerge doc state across a fluid device set is glue code you own (same class Matrix is still
solving for MLS-in-federation). (2) Provenance/attribution is necessary but not sufficient —
revocation is the hard half; the negative operation (removing a device) is what killed fusion
identity; stolen phone needs revoke-in-DID + propagate + MLS Remove/epoch rotation in every
group, and (c) requires someone online — the cleanest argument the broker isn't optional.
(3) Recovery is total-loss, and equal peers makes it worse — no asymmetric anchor means losing
all devices loses the identity (SSB/Briar dead-end); recovery requires breaking symmetry
(mnemonic seed, social-recovery quorum, broker-held backup). Honest reframing: state
convergence solved by Automerge; attribution solvable by DID; group-membership coherence =
MLS-meets-CRDT reconciliation you own, unsolved; revocation+recovery the actually-hard part,
requiring broken peer symmetry.

> **In a durable e2ee setup some nodes need authority to decide who's in the club, or you slow
> down, or risk key divergence, or constrain peers. What about a sliding window of consensus —
> 2-3 people sign off on membership changes; if dissent goes deep the group de facto dissolves
> (≤10); an offline forward-only chain like scuttlebutt records offline group changes; the only
> real risk is the group very noticeably splinters, clear but not insulting.**

This maps to real research; separating the sound part from the sharp edge. Sound: you've
arrived at threshold/quorum-based group administration — m-of-n admins is strictly better than
single-admin and cheaper than full-group consensus (Matrix MSC work + DCGKA literature circle
this); small-groups-self-police has teeth (<~10, social and cryptographic consensus nearly
coincide). The hard problem: "2 or 3 somehow sign off" — that "somehow" is the difficulty, and
an offline forward-only log relocates it, doesn't dissolve it. Unavoidable failure mode:
concurrent contradictory membership commits under partition (group of 8 splits 5/3, each with
quorum; side A signs "remove Dave," side B signs "Dave stays, add Erin"; both valid, both imply
different MLS epoch secrets — MLS structurally refuses to merge; you can only pick). Your own
conclusion is right and undersold: "the group very noticeably splinters, clear but not
insulting" is the correct resolution promoted to a feature — the log doesn't merge the
contradiction, it makes the fork detectable and attributable without blame. Two things to pin:
(1) the merge rule when forks reconnect (deterministic tiebreak heals transient partitions;
visible-splinter reserved for sustained contradictory intent); (2) who has standing in the
quorum, and does that set fork too (anchor the admin set to something slower-moving — the DID
layer / designated signers — so the decider set is more stable than what it decides).

> **[voice note]** Encrypted group + a separate chain controlling thresholds of group change,
> by operation type (add takes 1, boot takes 2, leave-self any time). Circle members decide;
> outcomes very clear. Git/Merkle-tree-like — bring a feature branch back or it becomes its own
> thing. A chat could have a sixteenth-great-grandparent with none of the original people, fine,
> costs nothing. Thresholds set at group setup, immutable. Three tiers / or just ask "how strict
> do you want membership." Defaults must be the no-surprise option. There's a recursion problem
> but you ground it at genesis — not turtles all the way down; self-referential but immutable, so
> fine.

Your style came through fine. You resolved the recursion: the group's constitution (thresholds
by op type) is fixed at genesis, immutable, the root of the tree — converts the unbounded "rules
for agreeing on rules" into bounded "agree once at creation." Git has the same property (first
commit has no parent). Per-operation thresholds track blast radius (leave-self=1, add=low,
boot=higher, dissolve=highest) — usable not bureaucratic. UX framing nailed: lenient
no-surprise default keeps the rare fork interpretable; three tiers (Casual/Normal/Locked) or
the single "how strict?" question. The unsolved seam: two trees — the governance chain (SSB-style
signed forward-only log, forks/heals, attributable) and the MLS epoch chain (single, linear,
refuses to merge). Binding rule: a governance event is real only once enacted as an MLS commit;
under partition you get validly-signed-but-not-yet-enacted ops; at heal you deterministically
pick a survivor epoch and re-enact the loser's still-valid ops onto it, or let the branch become
its own tree. Two things undecided: the heal rule (what picks the survivor; transient→heal,
schism→branch) and re-enactment semantics (rebased ops that now conflict surface to humans).
Confidence: genesis-immutability + per-op thresholds sound; the governance-rebases-onto-MLS
binding depends on openmls's actual reinit/external-commit surface — verify against the library.

> **[voice note]** Which side's MLS epoch is the survivor — which history becomes authoritative?
> It's more like inviting one group to abandon the other and join ours, not merging — you can't
> have two keys occupy the same space, so maybe a third in all cases. Whose history is lost? Can
> we forcibly rekey old data? Could you farm history-merge to peers, folding by timestamp,
> rekeyed into a third key, deciding how much history is kept? The re-enactment conflict (booted
> here, re-added there) — just refuse, hard stop on conflict, escalate to the human; it's a
> social problem, cap it.

You've talked yourself to the right answer, the boring one. "Inviting one group to abandon the
other and join ours" — yes; merge is impossible (one ratchet), so every "merge" is one side
adopts the other's epoch or both adopt a fresh third. Survivor: Group Zero forks to A/B; at
reconnect one chain's current key becomes the group's key going forward and the other side gets
re-keyed in as if joining (nobody kicked out absent a conflict). Whose history is lost? Nobody's
— each side already holds its decrypted messages locally; switching the live key forward doesn't
erase them; what's "lost" is only the unified view — and that's a CRDT/display problem, not MLS.
Forward key: one survivor (MLS's rule). Backward transcript: mergeable by peers because it's
just data. Farming history-merge to peers is the right place (no central authority, not
security-critical). The "threshold of sustained contradiction" question is mostly moot under
your rule "always attempt merge, hard-stop only on actual membership conflict." Algorithm: pick
survivor deterministically; re-key the other side in; check membership ops for contradiction —
no conflict → heal silently; conflict → stop, escalate, optional quorum to force. Escalating is
the only honest move (the disagreement is the content; any auto-resolution takes a side). Tiers:
heal silently / conflict→human / forced-merge→fresh genesis inheriting both as ancestry /
rejected→stay two groups (resting state, not error).

> **I actually think this probably makes no sense — "peers fold the two message logs together by
> timestamp" — because it'd be a weird jumble like 6 tapes playing at once. Instead use
> local-first peer thinking: let someone search and manage across their histories, or ignore, or
> discard at leisure.**

Correct, the mistake is mine. Interleaving by timestamp is the wrong primitive — two diverged
branches are two different conversations sharing an ancestor; splicing by clock produces the
six-tapes garble. Better model: don't unify the transcript at all. Keep branches distinct,
linked by the fork point (lineage, not interleaving — the Git insight done right). The
individual gets agency: their client searches across branches they hold, surfaces "you have a
related branch here," read/ignore/drop per-person. Kills the messy questions (whose history is
lost / who rekeys old data / how far to fold) — they were artifacts of trying to produce one
true transcript. Only the forward key converges (MLS); the past is never merged, just navigable
per-person. Fits local-first better. Open UX question: standing in A while B exists — (a)
nothing until you go looking, or (b) a quiet non-insistent marker; given your no-surprises
through-line, (b)/silence.

> **Could we even allow people to fill in other people's histories? One exception to
> history-merging: if you accidentally splinter, you could merge back in if they share a common
> parent. Forking is harder than add/remove; otherwise just start a new chat (vacation 2026, no
> link). Intentional histories — a curated archive of text+images, shareable between members.
> Standing in A while B exists: nothing, but if you rejoin you could be sent the history (the
> recombined key everyone signs off on). In a group chat you expect members to see it, full
> stop.**

You found a real distinction that makes it cleaner. What you're describing is backfill as a
consensual gift, not an automatic merge — directional, voluntary, per-person; nothing global,
no transcript rewritten. The consent model is your last line: in a group chat you already expect
members to see what's said. The boundary that matters is "were you ever party to a group on this
lineage" — which shared genesis proves; backfill within the lineage is catching someone up on a
room they had standing in; backfill to a stranger is the violation, prevented by the
common-parent requirement (that's why it's the privacy boundary, not a nicety). The recombine is
the one legitimate exception, gated by the recombined-key consent ceremony — you can't
accidentally recombine. The gem you tossed off: intentional histories — a curated archive is
just a branch someone deliberately froze rather than one that forked by accident; falls out for
free. Distinction: forking carries the common parent (the link is the point); starting a new
chat is the clean break (no link). Flag for later: backfill means a recipient gains messages
authored under epochs they weren't a member of, so their client must verify inherited history
(signed by parent-lineage members) without having witnessed those epochs — provenance on a
gifted branch; tractable (signatures + genesis anchor) but confirm what openmls gives for
validating an epoch chain you weren't present for.

> **Common ancestry is the key to safety — the only way to know is if you know where you branched
> from reliably. Then like code-editor folds — it's there if you want it but not in your way.
> Teenage gossip heaven, but a better experience than how messaging infrastructure thinks; groups
> are interconnected and fluid; a client reflecting that, grounded in human expectations and
> ironically cryptographic abilities.**

You're pulling me back to the point. Common ancestry is the safety primitive — knowing reliably
where you branched from is the root fact that makes everything legible; prove "these two groups
share genesis G" and you can reason about standing, recombine legitimacy, relatedness; lose fork
provenance and you can't say anything safe about the relationship between two conversations. The
code-fold metaphor is right (better than my "branches"): history always there, folded away;
unfold is the deliberate act; crypto guarantees the fold is real, lineage guarantees the unfold
is legitimate. The deeper thing: messaging infra has a broken model — real groups are fluid and
interconnected (vacation crew overlaps work friends overlaps family; people drift; side-convos
splinter and rejoin); tools model a group as a rigid flat eternal container, so software fights
social reality. What you've arrived at is a thesis: model groups as a navigable lineage of
conversations grounded in cryptographic provenance, and you get a client that's both more secure
and more true to how people socialize — and those two virtues are the same virtue ("knowing
reliably where things came from" is simultaneously the security property and the
social-legibility property — one fact, two payoffs). The mechanisms are implementation in
service of it; the thesis stands on its own.

> **Give me a detailed technical thesis and set of experiments to validate this with Claude
> Code.** [Q: prove out first? A: all three, sequenced as phases. Q: how real transport in v1?
> A: mix — sim for logic, real iroh for one integration spike. Q: where? A: inside experiments/.]

[Checked openmls external-commit builder + reinit; both expressible.] Done —
experiments/lineage-groups/THESIS.md. One claim with two payoffs; five sub-claims; two-tree
model; ten falsifiable invariants I1–I10; three phases with Phase 1 (crypto feasibility) the
make-or-break gate (can openmls express "pick a survivor epoch, re-key the other side, or mint a
third" with PCS intact); sim for Phases 0–2, real iroh in Phase 3; §6 paste-ready Claude Code
brief; honesty notes (total-device-loss recovery the largest residual risk; what a green run
would NOT establish).

> **[multi-device voice note]** What if the same user across devices is, cryptographically, a
> different user in the group — different key per device, same provable lineage. UI makes it
> discernible. Use the p2p history-sharing backend for self-sync, no server source of truth.
> Remove a device via the admin forward-only chain (keys and identity are not 1:1). Without
> multi-device this isn't usable, but it doesn't have to be perfect — I want the comms reachable
> and to manage devices individually.

This is the right idea and the natural consequence of what you built, not a new thing. You're
applying the same flattening to your own devices: each device a distinct MLS member, recover
"same person" at the DID-lineage layer, not the key layer (keys are not identity). Buys three
things: (1) device revocation = a normal governance op (revoke one actor under your lineage,
epoch rotates, stolen device goes dark); (2) no server source of truth (self-sync = backfill
between two actors sharing a genesis — your own devices trivially share genesis; point the
existing mechanism at a new case); (3) drift becomes honest (your laptop 3 days behind = a member
offline 3 days; one mechanism). Where it bites: N-devices×M-members scaling (40-leaf tree for 10
people×4 devices; logarithmic but governance log gets noisier — segregate device-events from
human-membership-events); the other-members'-view problem (everyone must fold leaves-sharing-a-
lineage into one displayed person — protocol-visible, the lineage proof must travel with the
leaf); quorum math (thresholds count lineages not leaves, or you manufacture a quorum from your
own devices); self-removal (leave-this-leaf vs leave-all-under-lineage). Classic trilemma
(shared key / server source-of-truth / single-device); yours is a genuine fourth corner. Slots
in as Phase 2.5 (data-model + threshold logic, not new crypto): E2.9 lineage-fold, E2.10
thresholds-count-lineages (adversarial), E2.11 device revocation, E2.12 self-sync-as-backfill,
E2.13 leave-one-vs-all. Dependency to confirm: openmls letting the lineage credential ride on
the leaf.

> **[refinements]** Collapse devices to one actor in history presentation, fine that under the
> covers it's not. Quorum = two members not two devices — but for an op on a common-ancestor
> member (one of my devices), one sign-off if by another device of the same ancestor, two if not.
> Remove your laptop: the moment you remove it you have the rights to modify the group, it just
> stops being a participant; history left behind is a separate concern. Two tiers (with/without
> superpeer); availability and administrative clarity are the two compromises you would NOT make.

Your refinements are sharper than mine. The self-add asymmetry: same-lineage device op = one
sig (the lineage is the authorization, devices self-organize); cross-lineage op = full social
threshold (a lost device gets cleaned up by others). Answers "2 humans or 2 devices" with a
third option — it depends on who's asking relative to the target's lineage. Self-removal
ordering: act while you still have standing, last act drops standing. Revoking participation ≠
wiping local history (no protocol claws back what was decrypted) — say "can't participate going
forward," never "forgot everything." Two non-negotiables (availability, administrative clarity)
vs two deliberate compromises (history completeness across devices, real-time sync perfection) —
the load-bearing prioritization; degradation must be visible, not silent (fail-early/fail-
clearly). Done — experiments/lineage-groups/MULTI_DEVICE.md, E2.9–E2.16, gated on E2.10 + E2.12;
§8.1 separates the one protocol hook (lineage credential on the MLS leaf).

> **Generate a comprehensive prompt to dig deeper into the classic issues for existing attempts
> vs our protocol — the Achilles heel of group chat dynamics; what are we not thinking of.**

[Produced the adversarial research prompt — preserved verbatim in
seeds/generated-prompts/achilles-heel-research-prompt.md.] Built to surface recurring structural
failure modes, framed adversarially toward our design; Question 2 (is the superpeer secretly the
ordering authority) is the sharpest "what are we not thinking of."

> **[social-tree voice note]** An internetwork of these groups with statuses (family/adults),
> message all my grandparents' children except X. A social-media-esque app showing N layers from
> your core circle. Identity becomes "someone vouches for you"; if one person vouches for 7,000
> dicks you prune them; my social world is frozen unless I say otherwise; no extraction model
> riding alongside. A Facebook-2010 bare-bones experience. But maybe it doesn't exist because
> it's by nature invasive — shadow profiles get built whether you like it or not, and that's the
> thing to invert.

Two distinct ideas. Idea one (internetwork of groups with status/longevity) is your lineage
model pointed outward — clean, ships on what you have; the grandparents'-descendants example is a
real unmet need (Google+ Circles is the corpse — built by an extractor who needed the graph).
Idea two (vouching as identity-propagation, pruning as control) is a web of trust — deeper and
more dangerous. The "frozen unless I say otherwise" move is the whole safety property, not an
aside — promote it to core invariant; it converts "trust is transitive, clean up the mess" into
"trust is transitive only as far as you personally walk it." Visible-but-opaque is harder than
Kevin Bacon suggests — topology deanonymizes (the shape of connections is a fingerprint); offer
scoped visibility, not opaque structure. The inversion is the actual thesis: you hold the graph
of your life, see it, shape it, nobody monetizes it, invisible to anyone you haven't shown — the
absence of an extraction model is the point. But the invasiveness cuts at your design too (a
family graph is a map; abusive members, estrangements, custody) — handle the adversary INSIDE
the graph: quiet membership / asymmetric visibility (be reachable without being mapped). Three
invariants to make central: freeze-by-default, scoped visibility, asymmetric/quiet membership
(the unsolved one). Treat the social layer as its own thesis (different threat model).

> **The labeling thing is a separate opt-in system. The graph is what it is; you choose to
> advertise groups or not; names/labels/roles ride on top with their own visibility. Scanner's
> best friend, but worse than scrolling phone contacts? Not really, but much more useful.
> Multi-identity has to be part of life. Composable + smooth enough for the subtle safety case
> while usable enough to build meaningful usage. Dream a minute about what you could do and how
> to protect it — let people build it for themselves.**

The architecture in one line: the graph and the labels are different layers, only the label
layer is social — keep them separate (most platforms fuse them, and the fusion is the
invasiveness). Calibration validated: exposes ~what a contact list does but makes it useful;
danger scales with usefulness, so safeguards scale too — they decide which friend you built it
for. Done — SOCIAL_LAYER.md (the inversion; graph/label separation; safety invariants S1–S4),
kept separate (different threat model). Then dreaming: the graph as a LENS you point, not a
PLACE you go (apps are views; leaving costs nothing — inverts lock-in); vouching as
renewable/revocable/scoped/decaying (a graph that gracefully forgets is more humane and more
private); protection through deliberate poverty of data (insultingly thin — minimalism as the
security model); the frozen world as a feature people want (restful), not just a safety default;
quiet membership unlocks the genuinely new primitive (support group reachable but mutually
invisible; whisper network real but unmappable); the family is the wedge (trust + offline
verification free; slow grounded growth is itself a safety property). The honest tension: every
one gets less powerful as it's safer, and there's no growth engine by design — solvable by being
genuinely better for the small grounded case, growing at the pace trust grows.

> **This also makes management of big supergroup broadcast-only things — a top layer with
> discussions slotted under, Reddit/forum style. As you register your link you can see everyone
> linked. Woodworking to White Lotus, same mechanism. Bluesky threads underneath would make it
> public — never mind, don't know how you'd mix those. Family bulletin board; school's-out-for-
> weather notice that doesn't run through a multibillion-dollar corporation — I hate having to
> look at Twitter for civic things. Local-first makes a lot possible — your local history is what
> matters.**

You found the scale axis, the missing piece — the same primitive stretched to broadcast.
Broadcast-with-discussion-under-it is a lineage where the root is write-restricted and branches
are open — Reddit structure falls out, not a forum engine. Self-selecting membership is genuinely
different: a public topic board inverts intent — visibility is the point, not the danger. So TWO
REGIMES on one substrate: intimate (default-private, quiet membership, visibility is opt-in
exposure) vs public/topic (visible-to-co-members, affiliation is the feature). Genesis carries a
visibility regime; the UI must make crystal-clear which you're in (the danger is confusing them).
The civic-notice use case is the strongest thing here: school closures / boil-water notices ride
private ad-funded can-disappear platforms; a locally-held, signed, offline-readable, no-
corporation civic notice board is more reliable than the Twitter version — and the thing that
makes it ethical (no extractor) and resilient (local-first, signed, offline) is the same property
again (third time: the ethical choice and the technical strength are the same choice; extraction
requires centralization, refusing it frees you to decentralize, which makes it resilient and
private — causally linked). Your "never mind" flinch located a real boundary: mixing private and
public content is the dangerous seam. The rule: **content is born into a regime and cannot
silently change regime.** Public-to-public flows freely; private stays private; crossing is
never a forward — it's a deliberate REPUBLISH (a new authored public act that references but does
not carry the private original). You mostly don't mix them; you reference across, never flow
across, and republishing is an explicit authored act with its own consent moment.

> **You'd need to make how many layers you share very conservative — maybe impossible to share
> more than one layer for a big/open/public group. Make it literally break everything if needed.
> Content is born into a regime and cannot silently change it.**

The instinct to break rather than warn is right, but sharper: make it **structurally
unrepresentable, not runtime-warned.** A warning is a speed bump; a thing the data model can't
express is a wall. If a public group's genesis fixes max-shareable-depth at one hop, a two-hop
share isn't discouraged — it's unconstructable, and every honest client rejects it the way it
rejects an under-threshold boot (enforced by everyone's verification, not the sharer's good
behavior — the only kind of limit worth having, since the safety case is the hostile sharer).
**Share-depth is a regime parameter that scales inversely with openness:** intimate small group
maybe a few hops; big open public group collapses to one or zero — a public group is a
**visibility sink, not a conduit** (connections terminate at it, don't flow past). Subtlety:
**depth (outward) and inward-visibility are separate, independent parameters** — outward (how far
my affiliation propagates) is the deanonymization risk, cap it hard; inward (what I see of the
group's structure) can be permissive for a public board (seeing co-members is the point). So a
public group can be transparent looking in while being a hard wall looking out through any
member. The whole safety geometry of the scale axis: regime is born-in and immutable; content
can't change regime silently; share-depth collapses as openness rises; inward-visibility and
outward-propagation controlled separately.

> **I'm gonna pause here. Let's update everything so it's current.** [Response interrupted.]

> **Actually can you give me a prompt to copy to add structural tests for this new thinking to
> the prior.**

[Produced the V1–V9 structural-tests prompt — preserved verbatim in
seeds/generated-prompts/structural-tests-visibility-regimes-prompt.md. Visibility regimes +
propagation geometry; V5/V8 the adversarial gate under a hostile sender; structurally-
unrepresentable-not-runtime-warned; openness/size classes left for the session to propose.]

> **Can you give me whatever documents we've created here again so I don't miss any?**

Four files: messaging-solutions-landscape.md, THESIS.md, MULTI_DEVICE.md, SOCIAL_LAYER.md. Note:
SOCIAL_LAYER's status line points toward the scale-axis sections, but the broadcast/civic/regime
content (sections 8–10) and the propagation-geometry material aren't written into it yet — we
paused before that update. Two things not captured in any file: the scale-axis / two-regime model
(intimate vs public, content born-into-a-regime, republish-not-forward, depth collapses with
openness, inward-visibility vs outward-propagation as separate genesis params), and the two
prompts (the failure-mode research prompt and the structural-tests prompt) — they live only in
chat. [These are now all preserved: the two prompts in seeds/generated-prompts/, the scale-axis
model in this transcript + thinking/social-layer.md's intended §8–10, and the regime tests were
walked out in Proofs/lineage-group-model (V1–V9, SOCIAL_LAYER_FINDINGS.md).]
