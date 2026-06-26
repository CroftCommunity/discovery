# The field as honest trades, and the ordering tension (not an impossibility)

date: 2026-06-26

status: thinking (new) — the synthesis + fact-check layer for the field-comparison framing in `beta/03`,
correcting an overstated claim and two outdated facts, and surfacing the decentralized-MLS prior art
(DMLS/FREEK, draft-xue-distributed-mls) that is a **sibling to the Drystone spec's serverless ordering**.
Source: `../seeds/transcripts/raw/field-trades-four-property-impossibility-dmls-and-redb-dialogue-2026-06-26.md`.

All facts here are **web-verified-in-dialogue, not yet in the FACTCHECK SoT → [confirm before publish]**.

---

## 1. The framing holds — and is *stronger* corrected

The field is **honest trades, not a ladder**: no deployed system delivers usability + decentralization +
metadata-protection at once. SSB (pure P2P, paid in multi-device hell / recovery dead-ends / unbounded
logs) and Briar (strongest threat model, *refuses* multi-device + recovery by design) **confirmed on every
point**. Two claims needed correction and one needed softening — and the softening makes the
honest-trades thesis survive *better* than the strict-impossibility version did.

## 2. Two corrections (folded into 03, flagged)

- **Signal is phone-rooted only at *registration*.** A phone number is still required to register
  (Sybil/registration layer), but since **usernames (2024)** it is no longer the contact-graph identifier
  (you can hide your number even in groups). "Phone-rooted identity" flat is imprecise.
- **Delta Chat no longer "inherits email's metadata leak."** True historically over classic email
  (To/CC reveal who-talks-to-whom). But **2.48+ (2026)** moves `To`/`Subject`/`References`/`In-Reply-To`/
  group-membership into the encrypted part — full **Header Protection, RFC 9788** — and randomizes the outer
  `Date` on a 5-day window; with a **chatmail relay** no contact/group metadata is stored server-side.
  Residual: a **relational-metadata exposure at the relay** (no Sealed Sender yet), not the full header
  leak. Caveat: this is the chatmail config, not classic email.

## 3. The load-bearing fix: the four-property "impossibility" is a *trade with a quantified cost*

The claim "group moderation + multi-device + PFS + offline-mesh cannot all hold without an unequal,
privileged peer" is **not a proven impossibility** — it is a real engineering tension with a specific
mechanism that is being actively dissolved.

- **The mechanism.** MLS needs a **Delivery Service** to agree on Commit *ordering*. Remove it →
  concurrent same-epoch commits **fork** (the epoch chain becomes a DAG) → members must **retain key
  material to process commits out-of-order**, which **degrades forward secrecy** (a compromise can
  propagate down the epoch DAG). So the ordering authority and PFS trade against each other server-free.
- **It is quantitative, bought back by construction.** **DMLS** (Phoenix R&D) + **FREEK** (Fork-Resilient
  CGKA; Alwen/Mularczyk/Tselekounis) recover most of that FS via a **puncturable PRF (PPRF)**: clients
  *puncture* retained key material per-commit — deleting the direct-path secrets so the same output can't be
  re-derived (the FS property), keeping co-path secrets so other commits still process. **Cost-shifting, not
  magic:** the cost is **storage** (~8 kB per PPRF eval; scales with retention window, group size, key size,
  and *fork frequency*). The authors frame it modestly — a building block that *meaningfully improves* FS
  where forks are inevitable, not a full restoration of server-ordered MLS's deletion-schedule FS.
- **The ordering role need not be a *privileged* peer.** Plain MLS already resolves forks with a
  **deterministic tie-break**; a decentralized DS can be clients transmitting over P2P. "Unequal, privileged
  peer" smuggles in a centralization conclusion the cryptography doesn't require — it can be a
  **deterministic protocol role**.
- **Deployment status (airtight phrasing).** **No production deployment ships the escape.** Every shipping
  MLS system is **server-ordered** (Webex, Wire, Discord; Google/Apple **RCS MLS-E2EE, May 2026**). The
  serverless variants — **DMLS/FREEK** (IETF draft + PoC OpenMLS fork) and **`draft-xue-distributed-mls`**
  (per-member "Send Groups", PCS+FS without global ordering consensus) — are **drafts/PoC as of mid-2026**.
  So say **"no *production* deployment delivers all four,"** not "impossible," and exclude in-progress
  protocols explicitly.

**Net:** the privileged ordering peer is **empirically universal in deployment, theoretically escapable, and
nobody has shipped the escape.** That is an honest trade, stated precisely.

## 4. Why this matters for Drystone (the spec is on the decentralized side of this exact trade)

The Drystone spec's §7 governance **is** a decentralized-DS / decentralized-MLS approach: **timestamp-free
deterministic ordering**, **fork-by-construction**, **deterministic tie-break**, no privileged orderer — the
meer is "unequal-in-capability, **equal-in-rights**." So:

- **DMLS/FREEK and draft-xue-distributed-mls are sibling prior art** the spec should cite as related work
  (and as `[confirm before publish]` comparisons, like the Matrix contrast). Drystone is *another* point in
  the same design space.
- **FREEK's out-of-order-commit FS cost is the exact cost Drystone's fork/reconcile model must reckon
  with.** The spec/social-graph synthesis says "forward secrecy constrains keys, not history; undesired
  forks self-heal by deterministic tie-break in the retention window" — FREEK quantifies the *price* of
  holding that window open (retain-and-puncture key material). This **couples T29** (MLS↔governance-log
  consistency) and **T22** (survivor/re-key vs tenure): the FS cost of reconciling a fork is a real,
  storage-scaling quantity, not free.
- **redb confirmation:** the dialogue's Part A confirms the redb facts the build prompt relies on (1.0
  Jun 2023, savepoints, MVCC single-writer/multi-reader, per-transaction durability, **stable file format**,
  no Jepsen-grade crash-safety evidence). The redb **3.x-specific API** remains web-sourced.

## Where this lands

- **Corrections → `beta/03`** (narrative + §1 + field map), flagged `[confirm before publish]`.
- **DMLS/FREEK + draft-xue prior art + FREEK FS-cost → OPEN-THREADS T29** (the MLS↔log binding it sharpens),
  **`drystone-spec` Part 2 Appendix A** (related-work note), and **ECOSYSTEM** (the decentralized-MLS
  frontier + the MLS-deployment-status register).
- **redb confirmation** noted (build prompt facts corroborated).
