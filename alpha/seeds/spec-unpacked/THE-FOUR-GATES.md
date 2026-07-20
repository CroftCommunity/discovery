# The Four Gates — a walkthrough for consideration

`Same shape as the feasibility verdict: assessment is mine, facts inside it are the earned ones.
For each gate: what it is in plain terms, why it gates, the option space with costs, what opens
when it opens, my recommendation, and what would change my mind. The gates in one line each:
I9 is a decision, the resolution-ACL is a design, X1 is a purchase, the release pass is scheduled
work. Two need your pen, one needs your card, one needs a calendar.`

---

## Gate 1 — I9: the recovery trust predicate (a decision)

**Plain terms.** A person loses every device. Their key still exists, split into shares held
across independent trust domains, and the lock provably round-trips (the BIP39 spike). The open
question is the predicate: under exactly what conditions do the share-holders release? That
question IS the product's recovery experience, because in an E2EE system there is no
password-reset email — whoever can trigger recovery is the account's court of last resort.

**Why it gates.** Directly: threshold-revoke over the wire, conformance half two (the
revoke-authority distribution and the co-sign-versus-vote ordering), croft-group L2's authority
half, and the recovery ceremony itself. Indirectly: any consumer launch, because "what happens
when I drop my phone in a lake" is a first-week support question, not an edge case.

**The option space.** The architecture is already decided (predicate = per-deployment/per-persona
policy, never a protocol constant), so this is a choice of DEFAULT plus its UX:
- *Paper key only.* Zero new trust; the crypto-wallet literature says a large fraction of ordinary
  people will lose it; honest but brutal. Right as a floor, wrong as a default.
- *Quorum of contacts (social recovery).* Drystone-native: it can reuse the same k-of-n
  content-bound counting R7 already made `Verified` — the recovery request is a proposal, the
  releases are approvals. Costs: availability (friends unreachable), relationship churn (your
  quorum is your ex), coercion ("help me recover *his* account"). Mitigable with per-share
  wall-clock delays and notification-to-the-old-persona — both admissible because recovery is a
  utility-layer ceremony, where the no-timestamp discipline does not bind.
- *Minimal issuer/custodian.* One professional share-holder (the nonprofit from classroom Ch. 8,
  or a vendor) whose release requires the user's own proof tier. Costs: the custodial-drift
  gravity every E2EE survivor exhibits, and a target painted on the custodian.
- *Hybrid tiers (my expectation for the default).* k-of-n where the n mixes kinds — e.g., 2 of
  {paper key, three contacts counted as one lineage-quorum, one institution} — with delay and
  notify as standard riders. The mixture is the point: no single domain can act alone, which is
  the protocol's own independent-trust-domains posture surfacing as UX.

**What opens.** The largest queued cluster in the program, most of it mirroring machinery already
proven, plus the consumer-launch precondition.

**Recommendation.** Take this gate first; it is pure thinking, and the research essay (the
companion prompt) is designed to arm exactly this decision. Decide the *default* and the
*ceremony* (what the user sees at loss time), not just the predicate algebra.

**What would change my mind.** Evidence from the research that social recovery fails in practice
at rates that make the contact-quorum leg unsound — in which case the default shifts custodial-
plus-delay and the design burden moves to keeping the custodian provably weak.

---

## Gate 2 — the resolution-ACL (a design; croft-group L3)

**Plain terms.** Layer 1 already detects every fork totally and names it objectively. The
resolution-ACL is each person's private policy for what their world does with a fork: follow,
multi-home, drop, escalate. The verb set is sketched and closed; undesigned are the match
language, the floors meta-dial, and — the part that actually decides product success — the
DEFAULT policy and its presentation, because most users will never open a policy editor, so the
default IS the product.

**Why it gates.** The product cost of the protocol's honesty lands here. Contradiction banners,
quiescence delays, and fail-closed stalls are *correct*; users read unexplained pauses as
breakage. Whether "your removal is pending corroboration" renders as calm, legible process or as
a bug is entirely this design plus the DR language rules. It also gates croft-group L3+ and the
projection/multi-home experience.

**The option space (staging, not alternatives).**
- *v1 minimal:* ship exactly two behaviors — `follow_default` for everything below the floors,
  `escalate` for floor-grade forks — with the banner language done to DR standard. No editor.
  This is buildable the week the design doc is signed.
- *v2:* the match→verb ACL with a curated set of named policies ("strict," "big-tent,"
  "archivist") rather than a raw DSL.
- *v3:* the full editor, multi-home rendering, history-as-lens.

**What opens.** The adjudication UX, the classroom's Layer-2 chapters becoming honest, croft-group
beyond L2, and — strategically — the differentiator, because no incumbent can even express "here
is exactly what your group disagrees about, with proof."

**Recommendation.** Design v1 immediately after (or interleaved with) I9; it is small, and its
core deliverable is *language*, which is the work you're already best set up to do (the DR block
exists; the shape-of-disagreement taxonomy exists). Resist designing v3 first.

**What would change my mind.** If early real-group testing shows even floor-grade escalations are
rare, v1 might be the permanent product and v2/v3 become library features for institutions.

---

## Gate 3 — X1: real-NAT hardware (a purchase)

**Plain terms.** Two boxes (or VPSes plus a NAT'd endpoint) that let the suites run over the real
relay-and-holepunch path instead of loopback. The last Active register row names a router; this
buys the router.

**Why it gates.** It is the only thing between the current evidence and retiring the final
divergence, and it triggers a one-time *re-grade campaign*: every "loopback grade" bound in the
spec lifts in a single sweep when the transport-bounded suites go green on real NAT. It also
unlocks honest hot-N measurement later.

**The option space.** Trivial: buy/rent now versus keep parking. The cost is small and fixed; the
risk it retires is the one place where "very likely fine" (iroh's proven path) is still not a rung
on your own ladder.

**Recommendation.** Do it in parallel with the I9 thinking — it needs no design attention, and
having it done means the release pass (Gate 4) pins encodings against real-transport evidence
rather than scheduling a second re-grade after.

**What would change my mind.** Nothing realistic; the only argument for waiting is cash-flow
timing.

---

## Gate 4 — the `[gates-release]` pass (scheduled work)

**Plain terms.** The door between "feasibility demonstrated" and "someone else can implement it
from the documents." One coordinated pass that pins every byte encoding (canonical fact,
checkpoint, cursor, the horizon manifest — including the deferred frontier-head decision — and
SubspaceId E.1), re-proves §4 end-to-end on BLAKE3, brings the governance-resolution vectors into
the conformance suite, and lands the emitter integration you already decided (Option C) here.

**Why it gates.** Publication, third-party implementation, and any credible RFC-style submission.
Nothing *product-internal* strictly blocks on it — your own crates interoperate with themselves —
but every external ambition does.

**Sequencing logic (the one real decision here).** Run it LAST of the four, and specifically
after I9, because the revoke-authority and recovery-ceremony vectors should be pinned once, not
re-pinned after the trust predicate lands; and preferably after X1, so encodings are pinned
against real-transport evidence. Inside the pass, the frontier-head decision (converged-state
digest versus DAG-tips set) gets made on the record.

**Recommendation.** Schedule it as its own run-series with the same FIX/FINDING and audit loop;
it is the highest-volume, lowest-uncertainty work remaining, and the loop you've built is exactly
shaped for it.

**What would change my mind.** An external implementer showing up early — in which case a partial
pass (fact encoding + conformance vectors only) jumps the queue.

---

## The order, stated once

**I9 (pen) and X1 (card) in parallel → resolution-ACL v1 (pen, small) → the release pass
(calendar).** Two decisions, one purchase, one campaign — and after the four, what remains between
Drystone and a shipped product is product work proper: the client, the operators, and the humans.
