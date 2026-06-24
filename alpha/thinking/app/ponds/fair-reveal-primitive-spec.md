# Fair-Reveal Primitive

author: research session

scope: a specification for the single shared commit-reveal module that provides fairness for secret-ballot voting, dice and other randomness, and hidden-information games, over pure P2P with no trusted server

date: 2026-06-21

why one module: voting, dice, the fair randomizer, and games like Two Truths and a Lie all need the same guarantee, that nobody can see or change a value until everyone has committed. Building it once and reusing it is high leverage. This is the primitive that unlocks a whole column of the catalog.

note on confidence: the cryptography here is standard commit-reveal (hash commitment), not novel. The transport assumptions are grounded in the iroh model verified earlier (gossip for broadcast, docs for persistence, per-entry author signatures). Anything platform-specific to confirm at build time is marked [VERIFY].

---

## The problem it solves

Pure peer-to-peer has no neutral referee. So any interaction where a value must be hidden-then-revealed is vulnerable to two cheats:

**Peeking:** seeing others' values before choosing your own (vote-watching, peeking at the die).

**Changing:** altering your value after seeing others' (changing your vote once the tally is visible, re-rolling a bad die).

Commit-reveal closes both with no server. You first publish a binding-but-hiding *commitment* to your value, then later publish the value itself. The commitment proves you cannot have changed the value (binding), and reveals nothing about it until you choose to (hiding). Everyone verifies independently and computes the same result.

---

## The core protocol

Three phases. The same shape regardless of whether the payload is a vote, a die roll, a hand of cards, or a secret statement.

### Phase 1 — Commit

Each participant `i`:

1. Forms their value `v_i` (a vote, a random seed, etc.).

2. Generates a high-entropy random nonce `r_i` (see "the nonce is load-bearing" below).

3. Computes a commitment `C_i = H(v_i ‖ r_i)` where `H` is a cryptographic hash (BLAKE3 is the natural choice since iroh already uses it) and `‖` is unambiguous concatenation (length-prefix each field so `("ab","c")` and `("a","bc")` cannot collide).

4. Publishes `C_i`, signed by their author key.

Everyone collects all commitments. **No value is readable at this point.** The set of commitments is frozen once the commit phase closes.

### Phase 2 — Reveal

Once all commitments are in (or a deadline fires, see "the abort case"), each participant `i`:

1. Publishes `(v_i, r_i)`, signed.

Everyone, for each revealed pair, checks `H(v_i ‖ r_i) == C_i` against the commitment they saw in Phase 1. A mismatch means that participant tried to change their value after committing: their reveal is rejected.

### Phase 3 — Compute

Every peer, having the verified values, computes the result **locally and deterministically**, so all honest peers arrive at the identical outcome:

- **Voting:** tally the verified votes.

- **Shared randomness (dice, shuffles, who-goes-first):** combine all verified seeds into one shared random value, `seed = r_1 ⊕ r_2 ⊕ … ⊕ r_n` (XOR), or feed the concatenation through a hash. The result is unbiased as long as at least one participant chose their seed honestly, because no single party can steer the XOR without knowing the others, which the commit phase prevented.

- **Hidden-info games:** reveal the committed hands/statements; play proceeds with values now provably un-tampered.

---

## The nonce is load-bearing (the most common implementation bug)

For **low-entropy values** the nonce is what provides hiding. A vote is one of a handful of options, so without a nonce an attacker simply hashes all possible votes and matches `C_i` to learn the vote, defeating secrecy entirely. The random `r_i` makes the commitment preimage-infeasible to guess.

Requirements:

- `r_i` must be **cryptographically random** and large enough to be unguessable (128 bits or more; reusing iroh's BLAKE3 and a CSPRNG is fine). [VERIFY] the platform CSPRNG (`getrandom`/OS RNG in the Rust core; `crypto.getRandomValues` in any web context).

- `r_i` must be **fresh per round.** Never reuse a nonce across votes or rolls.

- `v_i ‖ r_i` must be **unambiguously encoded** (length-prefixed fields), or two different (value, nonce) pairs could collide and create disputes.

For **high-entropy values** (a random 256-bit seed for dice) the value is already unguessable, but keep the nonce anyway for uniformity and to harden against any structure in `v_i`.

---

## The abort case (the real-world wrinkle, not the crypto)

The cryptography is the easy part. The operational hazard is a participant who **commits then disappears before revealing** (crash, dropped connection, or a deliberate withhold to stall or grief).

Mitigations, to be decided per use case:

- **Reveal deadline.** The commit phase and reveal phase each have a timeout. After the reveal deadline, the round proceeds with whoever revealed; non-revealers are dropped from this round.

- **Decide the semantics of a missing reveal explicitly:**

  - For *voting*: a non-revealer simply abstains; the tally proceeds over revealed votes. Low stakes, low harm.

  - For *shared randomness*: a non-revealer must not be able to bias or void the result by selectively withholding after seeing nothing (they have seen nothing, so withholding only removes their seed). Proceeding with the remaining honest seeds is safe as long as at least one honest seed remains. If *everyone* but one withholds, that one still cannot bias it alone. So "proceed with revealed seeds after deadline" is safe.

  - For *hidden-info games*: a missing reveal usually voids the round or forfeits the absentee, since play cannot continue with an unrevealed hand. Define this per game.

- **Grief resistance:** because a withholder learns nothing by withholding (commitments are hiding), there is no *informational* advantage to aborting; the only attack is denial-of-progress, which the deadline handles. Note this in the design: commit-reveal is robust against peek/change cheats and degrades only to "this round may need to restart," never to a fairness break.

[VERIFY] clock handling for deadlines: peers have skewed clocks (the same class as the shared-timer wrinkle). Use a relative timeout from a round-start marker rather than absolute wall-clock, or a simple round-leader heartbeat, to avoid skew-induced disputes.

---

## Transport mapping (onto your iroh layer)

The primitive needs a broadcast of small signed messages in two rounds, with optional persistence of the outcome. This maps directly onto the layer from Phase 0 of the build order.

- **Live rounds (commit, reveal):** iroh-gossip. Each commit and each reveal is a small broadcast message, signed by the author key. Gossip is ephemeral, which is correct, the rounds are transient and every peer wants every message.

- **Per-message authorship:** sign every commit and reveal with the participant's author key so the tally/compute step can attribute and de-duplicate, and so one participant cannot forge another's commitment. (This is the same author-signature property iroh-docs entries carry; reuse it.)

- **Optional durable outcome:** if a result should persist (a recorded group decision, a game outcome for a leaderboard), write the *final verified result* as an iroh-docs entry (or a Bluesky outcome record), not the raw rounds. The rounds evaporate; the conclusion persists.

- **Membership:** the set of eligible participants comes from the pond roster (the per-member keys from the shared store), so the compute step knows who was entitled to commit.

---

## The module interface (engine-agnostic shape)

Keep it generic over the payload so every caller reuses it. Illustrative, not prescriptive:

```
# A round is parameterized by the participant set, a payload codec, and timeouts.
start_round(round_id, participants, payload_codec, commit_deadline, reveal_deadline)

# Each peer commits locally; the module broadcasts the signed commitment.
commit(round_id, value) -> commitment            # stores (value, nonce) locally, broadcasts H(value‖nonce), signed

# On receiving all commitments (or commit_deadline), the module prompts reveal.
reveal(round_id)                                  # broadcasts (value, nonce), signed

# The module verifies every reveal against its commitment and yields the outcome.
on_round_complete(round_id) -> {
    verified:   [ (participant, value) ],         # passed H(value‖nonce)==commitment
    rejected:   [ participant ],                  # reveal mismatch (attempted change)
    no_show:    [ participant ],                  # committed but never revealed
}

# Callers compute their own result from `verified`:
#   voting  -> tally(verified)
#   random  -> fold(verified)  e.g. XOR of seeds / hash of concatenation
#   game    -> apply revealed hands
```

Design notes:

- The module's job ends at producing the verified value set; **each caller computes its own result** from those values (a vote tally, an XOR'd seed, a dealt hand). This keeps the primitive small and truly shared.

- The module **never trusts a reveal** until `H(value‖nonce)` matches the stored commitment. Rejection on mismatch is the entire integrity guarantee.

- Nonces are generated and held **inside** the module, never exposed to the caller, so a caller cannot accidentally leak or reuse one.

---

## What it unlocks (the leverage)

One module, reused across:

- **Secret-ballot voting** (the governance-grade mode, and the seed of cooperative decision-making for Croft).

- **Fair dice / shared randomness** (Backgammon, any dice game, bingo's draw, the fair randomizer / who-pays / who-goes-first).

- **Hidden-information games** (Two Truths and a Lie; the trustless-future version of card games that otherwise use a trusted dealer).

Building it in Phase 1 of the build order, right after the cold-open hook, means voting and several games in Phases 2-4 are mostly "call the primitive and compute," rather than each re-solving fairness.

---

## What it does NOT solve (boundaries, stated honestly)

- **It does not provide a trusted dealer for ongoing hidden state** (e.g. a shared deck dealt over many turns where players draw unknown cards repeatedly). That is mental-poker / commutative-encryption territory and is heavier. For friends-only play, a trusted-dealer model (one peer holds the deck and sends each player only what they may see) is simpler and sufficient; commit-reveal is for the discrete hidden-then-revealed moments, not continuous hidden state. Reach for the dealer model for continuous hidden state, this primitive for discrete reveals.

- **It does not prevent collusion** among a subset of participants who share their values out of band. No P2P scheme can. For a friends product this is a non-threat; note it for any adversarial future use.

- **It does not by itself handle identity/sybil** (one person committing as many). That comes from the pond membership and author keys upstream, not from this module.

### Verification notes

Commit-reveal with a hiding nonce is standard cryptographic practice; the security properties (binding via collision-resistance, hiding via the high-entropy nonce, unbiased shared randomness via XOR/hash of seeds with at least one honest contributor) are well established. The transport mapping reuses the iroh facts verified earlier (gossip for ephemeral broadcast, docs for durable outcomes, per-entry author signatures). Confirm at build time: the platform CSPRNG used for nonces, the exact BLAKE3 binding/encoding (length-prefixed concatenation), and the clock-skew handling for round deadlines (prefer relative timeouts over wall-clock). No existing webxdc/iroh commit-reveal library was located in the research, so treat this as build-it-yourself; it is small.
