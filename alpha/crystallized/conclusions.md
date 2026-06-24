# Conclusions — what we tested, what we concluded, where it sits (plain English)

date: 2026-06-16

purpose: the plain-English synthesis. No jargon walls. What we set out to prove, what actually
held, what we concluded, the honest open risks, and how it all maps to the roadmap. For the
table-of-record version see `proof-ledger.md`; for the campaign-level write-up see
`experiments/iroh/CAPABILITIES.md`; for the sequenced remaining work see `../TEST-PLAN.md`.

---

## The one sentence

We are building Croft — an open, member-owned, peer-to-peer social/messaging platform that
refuses to extract from you — and the big risky technical bet underneath it has now held up
under real testing, including on genuinely separate machines across a real network.

## The bet we had to test first

The whole design rests on one unusual idea: **knowing reliably where something came from
(provenance) is at the same time the security guarantee and the thing that makes the social
experience honest.** Instead of one eternal chat room, a group is a navigable family tree of
conversations that can split and rejoin. The scary part was the cryptography: could a real,
standards-based encryption library actually support a group that splits during a network
outage and then cleanly rejoins — without ever leaking to people who were removed?

If that one thing failed, nothing else mattered. So we tested it first.

## What we actually proved (in plain terms)

**1. The make-or-break crypto works.** Using the real, audited MLS encryption library
(openmls), we showed a group can split under a network partition and rejoin: one side is
chosen as the survivor, the other side is re-keyed back in, and anyone who was removed stays
locked out. This is the result everything else depended on, and it passed.

**2. Disagreements are handled honestly, never faked.** If two halves of a split group make
contradictory decisions (one keeps a person, the other removes them), the system does **not**
silently pick a winner. It stops and asks a human. If there's no real contradiction, it heals
quietly on its own. We proved both behaviours, and proved that three separate computers,
given the same facts, independently reach the **byte-for-byte identical** verdict — no central
referee needed.

**3. The "always-on helper" is a convenience, not a boss.** A always-on relay node ("superpeer")
makes things faster and lets messages wait for offline people, but we proved it can only ever
see encrypted gibberish, and that every outcome it helps reach is also reachable without it. It
holds no special powers. (Honest caveat below.)

**4. History is preserved as separate, navigable branches — never blended into a fake single
transcript.** When branches reconcile, you get distinct, attributable threads, not six
recordings spliced into one misleading timeline. Forged or outsider history is rejected.

**5. The plumbing is real, not theoretical.** Big encrypted files transfer, resume after
interruption, and pull from multiple sources; a phone behind a real home-internet connection
(NAT) successfully connected from outside the data center via a relay — the actual path a real
phone would use. Gossip-style message spreading works even when a relaying node is killed
mid-run. An Android app proved the engine runs on a phone.

**6. The privacy-by-design social rules are enforceable in the data, not just promised.** A
group is born "intimate" or "public" and can't silently change; private content can't be
auto-forwarded into a public space; how far your membership can ripple outward is capped by
the rules and enforced by *everyone's* software, not just the sender's.

## What we concluded

- **The ethical choice and the strong-engineering choice keep turning out to be the same
  choice.** Refusing to extract data forces decentralization; decentralization is what delivers
  the privacy and resilience. This stopped looking like a slogan and started looking like a
  structural fact the proofs keep re-demonstrating.

- **"Forks are a feature" is real, not a rationalization.** Splitting and rejoining cleanly is a
  genuine capability we can build on, including for re-forming a group minus the people who were
  removed, with the lineage history intact.

- **The honest version of the "no servers" claim** is: there *is* a minimal ordering role, but it
  is blind, minimal, and holds no rights. We say that plainly rather than pretending it's pure
  serverless magic — because pretending would be the kind of dishonesty the whole project exists
  to avoid.

## The honest open risks (what we have NOT settled)

1. **Losing every device.** If you lose every device tied to your identity, how do you come
   back? We have now *decided* the approach (your other devices normally re-authorize a new one;
   an optional offline recovery seed covers the lose-everything case) but we have **not yet
   proven** it. This remains the single biggest residual risk. (Roadmap: gate G1 decided, proof
   T12 pending.)

2. **"Quiet membership."** Being in a group without that group being able to map all your other
   relationships — protecting the person whose safety depends on it — is designed but unsolved.
   It's where the hardest privacy problem lives. (Roadmap: T6, behind a design pass G5.)

3. **A human can still copy-paste.** Our rules stop *automatic* leaks of private content into
   public, but they cannot stop a person from manually retyping something private into a public
   post. That's a user-interface safeguard we still owe. (Roadmap: T8.)

4. **A few "prove it for real" items remain modeled.** The multi-device identity-on-the-leaf
   mechanism and threshold-signed history checkpoints are proven in simulation and now being
   proven against the real library (T1 is in progress as of today). The deliberately adversarial
   "try to break our design" research has been written but not yet run (T4).

## How this maps to the roadmap

Two tracks run in parallel:

- **Validation track (de-risk the protocol).** Phases 0–3 plus the adversarial passes are all
  **GO**. The cross-machine campaign moved many results from "simulated" to "ran on real
  separate machines across a real network." What's left is sequenced in `TEST-PLAN.md`: finish
  the real-library items (multi-device on the leaf → starting now), run the adversarial research,
  widen the merge/split case coverage, prove total-device-loss recovery, and characterize scaling
  and metadata leakage. A separate relay-capacity lab (how big a community node must be) is
  half-done.

- **Product track (ship value).** M0 is a single-user encrypted vault that's useful on day one;
  M1 is secure group chat (gated by the validation track above); M2 is the social graph you hold;
  M3 the consumer-pull economic inversion; M4 the cooperative that owns and maintains it all.
  Roughly a 3–5 year build, co-op incorporated early, built in public.

**Where we are right now:** the foundation bet is proven, the plan for the remaining proofs is
comprehensive and mapped onto our actual test machines, and we have just begun executing the
first remaining real-library proof (multi-device identity riding on the encryption leaf). The
next decisions that are genuinely *ours* (not the experiments') — recovery details, the
open-source license question, the quiet-membership design — are flagged as explicit gates so
they don't get silently skipped.
