# Research prompt — "The Recovery Problem": did account recovery really kill E2EE consumer products, and what should Drystone's trust predicate learn from the bodies?

## Role and mission

You are a research writer producing a sourced narrative essay for the owner of Drystone, a
center-free peer protocol for social governance. Drystone's one open design decision (internally
"I9") is the **recovery trust predicate**: the rules for who may trigger the release of a sealed
key share when a person loses every device. The *lock* is built (threshold shares across
independent trust domains; a BIP39 paper key exists). The *predicate* — quorum of friends? an
institution? a paid custodian? a waiting period? — is undecided, and it is also the product's
recovery UX, because in an end-to-end-encrypted (E2EE) system there is no password-reset email:
whoever can trigger recovery is, functionally, the account's court of last resort.

The working hypothesis you must TEST, not assume: **"Every E2EE consumer product that died, died
mostly at account recovery."** Treat this as a claim under audit. If the evidence says products
died of other causes (funding, acquisition, network effects, abuse pressure) with recovery as a
contributing wound, say so with the same care you'd use to confirm it. A refuted or refined
hypothesis is a fully successful outcome.

## Research questions (answer all, in this order of importance)

1. **The taxonomy.** What recovery models have shipped in E2EE consumer products? Build the full
   map: nothing/paper-key-only; custodial escrow; PIN-hardened server-side vaults (secure-enclave
   / HSM-backed); platform keychain sync; recovery contacts / social recovery; institutional or
   admin escrow (the enterprise variant); time-delayed recovery; hybrid tiers. For each: at least
   one named shipping example, what the user experiences at loss time, and what trust it actually
   assumes (stated vs real).
2. **The bodies.** For E2EE or E2EE-adjacent consumer products that shut down, pivoted, or
   stagnated — investigate at minimum: Keybase, Skiff, Wickr (consumer), pEp, Peerio, Wire's
   consumer arm, and any others the evidence surfaces — what actually killed each one? Where does
   recovery/key-management friction appear in the post-mortems, reviews, and support-forum
   archaeology, and where is it absent? Quote the post-mortems.
3. **The survivors and their trades.** Signal (the 2020 PIN/SVR controversy is a required case
   study: what was changed, why, what the community said, what Signal said back); WhatsApp's
   encrypted backups; iMessage/iCloud Advanced Data Protection (recovery contacts and recovery
   keys); Matrix/Element's cross-signing + secret storage (and its usability reputation); Proton.
   For each: what recovery model, what it cost them (custodial drift? user loss? support burden?),
   and any published numbers on lockouts or recovery usage.
4. **The adjacent domain with hard data.** Cryptocurrency self-custody is the same problem with
   money attached: seed-phrase loss rates, the fate of social-recovery wallets (Argent and the
   Ethereum social-recovery lineage, including Vitalik Buterin's writing on it), and what that
   field learned about ordinary people holding root secrets. Extract the numbers where they exist.
5. **The academic spine.** The usable-security literature from "Why Johnny Can't Encrypt" forward,
   plus SOUPS/CHI/USENIX studies on key management, recovery, and secret-sharing usability. What
   does the measured evidence say about what fraction of users can be trusted with a paper key,
   and about social recovery's real-world failure modes (unavailable friends, coercion,
   relationship churn)?
6. **The abuse ledger.** For each predicate class: its attack surface (SIM-swap analogues,
   coercive "recover your spouse's account," insider risk at custodians, collusion thresholds) and
   any documented real-world exploitation.
7. **The institutional variant.** How do E2EE products sold to organizations (the dispatch-desk
   case) square admin recovery with end-to-end claims? Who escrows, who audits, what do the
   procurement documents actually require?

## Drystone-specific constraints the synthesis must respect

- Center-free: no protocol-level privileged party; the predicate is per-deployment/per-persona
  POLICY, not a protocol constant (that direction is already decided — you are informing the
  default policy and its UX, not reopening the architecture).
- The lock is threshold shares across independent trust domains; candidate predicate shapes
  include quorum-of-contacts (which could reuse the protocol's proven k-of-n counting) and a
  minimal issuer/custodian; hybrids and time-delays are open.
- The provenance plane forbids trusted timestamps, but recovery is a utility-layer ceremony, so
  wall-clock delays are admissible THERE — note where delay-based designs help or hurt.

## Source and quote discipline (non-negotiable)

- Source priority: primary post-mortems, official design docs and blogs, peer-reviewed papers and
  arXiv, then reputable reporting; SEO listicles and secondhand blogs are corroboration only and
  must be labeled as such when leaned on.
- A direct quote is verbatim from a retrieved source, attributed to both speaker and document with
  a link and date. Never quote from memory. If only a paraphrase exists, write "this is the
  outlet's paraphrase, not their words." Never attribute your own gloss to a source.
- Mark claim status throughout: sourced claims carry the source inline; if a specific can't be
  sourced, write "I could not find a source for X" and move on. No unverified specific presented
  as known.
- Where sources conflict (they will, especially on why products died), show the disagreement;
  do not synthesize it away.

## Output: a narrative essay, plain English, with metaphors

- Shape: a through-narrative (suggested arc: "the empty password-reset email" → the taxonomy told
  as characters → the bodies, case by case → the survivors' bargains → the money domain's hard
  numbers → what the academy measured → the verdict on the hypothesis → what this means for a
  center-free system choosing its default predicate). Prose, not bullet-dump; headers are fine,
  bullets sparing.
- Every technical term defined at first use, in the same paragraph, connected to the story.
- One sustained metaphor per major concept (e.g., recovery as the locksmith problem: who is
  allowed to drill your safe, and what does hiring them cost you the rest of the year) — then mark
  where the metaphor breaks so it isn't over-trusted.
- End with: (a) an honest verdict on the hypothesis, graded (confirmed / refined-to / refuted,
  with the refined wording if applicable); (b) a one-page "what the evidence recommends for
  Drystone's default predicate," separating what the evidence supports from the author's judgment;
  (c) the full source list.
- Length: whatever the evidence needs; depth beats brevity, but every page must be earning it.
