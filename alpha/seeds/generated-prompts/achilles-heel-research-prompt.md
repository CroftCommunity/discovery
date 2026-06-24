# Research Prompt: The Achilles Heels of Group Chat — Failure Modes Across Decentralized Messaging, Tested Against a Lineage-Based Model

source: generated 2026-06-14 during the design dialogue; preserved verbatim as a seed

status: not yet run — this is a prompt to paste into a fresh research session

intent: surface recurring structural failure modes (not a feature catalog), then adversarially test the lineage model against them

---

## What I want from you

I'm designing a group-messaging protocol and I want to understand, deeply, why group chat and group-membership management have been so hard across the history of attempts — especially decentralized and P2P ones. I don't want a feature comparison chart. I want the recurring structural failure modes: the things that keep killing these systems or making them unusable, the pain points that show up again and again across different designs, and the tradeoffs nobody has escaped.

Then I want you to test my emerging model against that history and tell me what I'm not thinking of. Be adversarial. I can find my own design's strengths; I need you to find where it inherits the same diseases that killed the others, or introduces new ones. A reassuring answer is worth less to me than an honest one.

Verify current state with sources rather than relying on memory — several of these systems have changed or declined, and dates matter. Separate protocol-level facts from client/product-level facts. Mark anything thin as unverified.

## The systems to mine for lessons

Dig into the group-chat and membership-management history of at least these, weighting the ones with the richest failure record:

- Secure Scuttlebutt (SSB) — weight this most heavily. Pure P2P, append-only logs, identity-as-keypair. Study its multi-device problem, the fusion-identity effort and why it stalled, log-growth/replication scaling, onboarding friction, and the overall ecosystem decline. This is the canonical cautionary tale.

- Matrix — federated E2EE group chat (Olm/Megolm), and the long-running, still-incomplete MLS migration. Study what makes group encryption hard in a distributed topology, history-on-join vs forward secrecy tension, and device/cross-signing complexity.

- Signal — the centralized benchmark. Study what centralization buys for group management (and what its sender-keys group model costs), and the PIN/registration-lock recovery story.

- Briar — P2P, Tor, strong threat model. Study specifically why it refuses multi-device and has no account recovery, and what that refusal tells us about the security/usability tension.

- Session — decentralized, no-phone-number, and its history of dropping then re-adding forward secrecy. Study its mnemonic-seed recovery and multi-device model.

- Delta Chat — Rust + iroh in production, email-substrate identity, "Add Second Device" transfer model. Study its multi-device-as-transfer-not-sync approach and what it deliberately declined to do.

- MLS / RFC 9420 itself — as a protocol, study its known hard edges: single linear epoch chain, no merge, the "who orders commits" delivery-service assumption, tree growth with membership, and what MLS explicitly does not solve (it assumes an ordering service; it doesn't define group membership policy).

Add others if they teach a distinct lesson (MLS-based products like Wire or Matrix's MLS work; XMTP, Keybase teams, Cwtch, SimpleX, Tox, classic XMPP MUC/OMEMO). Don't pad — only include a system if it surfaces a failure mode the others don't.

## The failure modes I specifically want excavated

For each, I want: how it manifests, which systems suffered it and how, why it's structurally hard (not just "they didn't get to it"), and what the partial solutions cost.

- Multi-device. Why is "use the same group from my phone, laptop, and browser" so persistently hard? Map the approaches: shared key (and its divergence/security cost), primary-plus-linked with a server (and its centralization cost), per-device-as-separate-member (and its scaling/UX cost), and transfer-then-diverge. What is the actual root difficulty — is it key management, history sync, the member-list explosion, or the social-vs-cryptographic identity mismatch?

- Device loss and identity recovery. The hardest usability×security collision. Map the spectrum from "you're just gone" (SSB, Briar) to mnemonic seed (Session) to server-backed PIN (Signal) to key-backup (Matrix). Why does every recovery mechanism either weaken the threat model or reintroduce a trusted party?

- Membership-change handling and group key agreement. Add/remove triggering rekey, who is authorized to do it, and what happens under concurrency or partition. Why is "two people changed the membership at the same time while partitioned" a genuinely hard problem and not just an engineering oversight? How do sender-keys (Signal/WhatsApp), Megolm (Matrix), and MLS each handle membership change, and what does each give up?

- The ordering / consensus assumption. MLS assumes a delivery service that orders commits. Most "decentralized" group chat smuggles in a server or ordering authority right here. Dig into this: is decentralized group membership fundamentally a consensus problem, and is the always-needed-ordering-authority the dirty secret of the whole field?

- History on join, backfill, and forward secrecy tension. What new members can see, how history is (or isn't) shared, and how that fights forward secrecy. Why does keeping history readable undermine the security property, and how have systems chosen?

- Scaling. Log/state growth over time (SSB's unbounded logs), rekey cost as group×devices grows, replication overhead. Which systems hit a wall and where.

- Onboarding and the social/cryptographic identity gap. Why keypair-identity systems are hard for normal people, and why phone-number identity is easy but a privacy/centralization liability.

- Governance and moderation. Who can kick whom, admin models, and what happens to "admin powers" with no server. This is the least-studied one and I suspect it's an under-appreciated killer — dig hard here.

## My emerging model (the thing to test against the history)

Now take everything above and stress-test this design. The point is not to validate it — it's to find where it inherits the field's diseases or hides new ones.

The model, briefly:

- Two separate structures. (a) A governance tree: a forward-only, signed log of membership operations (add/remove/leave/dissolve/fork/recombine), evaluated against threshold rules that are fixed immutably at group genesis. (b) An MLS epoch chain (openmls, RFC 9420) for the actual key ratchet. The binding rule: a governance op is only "enacted" once realized as an MLS commit.

- Identity is a DID lineage; keys are per-device. Each device is a distinct MLS member with its own key, carrying a credential proving it belongs to a lineage. "Same person across devices" is recovered at the presentation layer (fold leaves sharing a lineage into one displayed actor) and at the threshold layer (thresholds count lineages, not leaves, so you can't manufacture a quorum from your own devices). Device add/remove within a lineage costs one signature; cross-lineage ops pay the full social threshold.

- Membership thresholds are per-operation and set at genesis (e.g. add=1, boot=2, dissolve=N, leave-self=always). Immutable genesis grounds the "who decides who decides" regress at the root.

- Forks are a feature, not a failure. Under partition, contradictory-but-valid membership commits are treated as a clean, attributable fork. Reconnect picks a survivor epoch deterministically and re-keys the other side in via MLS external commits; on genuine membership conflict (someone booted on one side, kept on the other) it hard-stops and escalates to a human rather than auto-resolving. A rejected merge just leaves two valid groups (resting state, not error). A forced clean merge mints a fresh genesis inheriting both prior logs as read-only ancestry.

- History never merges into one transcript. Forked branches stay distinct and navigable (a "code-fold" model: present but folded away, unfoldable on demand). Reconciliation across branches — and self-sync across my own devices — is consensual backfill: a lineage member gifts a branch, the recipient verifies it chains to a shared genesis and imports it. No server-side source of truth. Self-sync is literally the same operation as catching up a forked branch.

- Optional always-on "superpeer" broker that is cryptographically blind (sees ciphertext + routing only). It provides rendezvous, queue, and snapshot storage, and can carry revocation/rekey commits when human peers aren't co-present. Two tiers: with-superpeer (prompt, palatable) and without (pure P2P, accepts stale history and slower sync). The non-negotiables are forward availability and administrative clarity; the deliberate compromises are history completeness across devices and real-time sync perfection.

## The questions I most need answered

- Where does this model inherit the field's known failure modes anyway? Be specific. For each of the eight failure modes above, does my design actually escape it, partially escape it, or just relocate it somewhere I haven't looked?

- The ordering/consensus dirty secret (failure mode 4) is the one I'm most worried about. My governance tree forks and heals; my MLS chain needs ordered commits. Am I smuggling a consensus authority into the "deterministic survivor selection" or the "who enacts the commit" step without admitting it? Is my superpeer secretly the ordering service that makes me no better than the systems that quietly rely on a server? Push hard here.

- Governance/moderation (failure mode 8) with no server — is my immutable-genesis-thresholds + signed-op-log model actually workable, or are there moderation dynamics (spam, a captured quorum, a malicious majority, ban evasion via new device leaves) it can't handle? What do existing systems' moderation failures teach me that my model ignores?

- The per-device-as-separate-member choice — I think it elegantly turns device revocation into a normal governance op and self-sync into backfill. What does it cost me that I'm underestimating? (Tree size, governance-log noise from device churn, the member-list fold being load-bearing for everyone's client, the lineage credential having to ride on the MLS leaf.) Has anyone tried per-device-as-member and what happened?

- Recovery (failure mode 2) — I genuinely don't have a total-device-loss story yet (if you lose every device under a lineage, what anchors recovery?). Given the whole field's struggle here, what are my real options and what does each cost my threat model? Don't let me hand-wave this.

- The composability claim. I think the value here is that the pieces (lineage identity, governance tree, MLS ratchet, CRDT history, consensual backfill, optional broker) are composable and separable. Is that actually a strength, or is it complexity that will collapse under its own integration cost the way ambitious P2P designs tend to? Where does the composition leak — where do two clean pieces produce a messy seam?

- What is the Achilles heel I haven't named? Given the history, what kills systems like mine that I'm not even asking about? What's the failure mode that won't show up in my experiments until it's too late?

## Output

Structure it as: (1) a synthesis of the field's recurring failure modes with the structural reason each is hard, drawn from real systems with sources; (2) a head-to-head mapping of my model against each failure mode — escaped / partially escaped / relocated / inherited, with honest reasoning; (3) the strongest case against my design — the three or four most likely ways it fails, ranked; (4) the open questions I haven't resolved and the realistic options for each. Prose over bullets where reasoning matters. Don't soften the negative findings.
