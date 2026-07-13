# What we demonstrated, in plain language

date: 2026-06-15

This is a plain-language companion to the detailed `TEST-LOG.md` and the findings docs. It explains,
without jargon, what we actually ran across four real machines (three AWS boxes in different data-
center zones, plus a laptop behind a normal home/office NAT) and what it means for what Croft can do.
Every claim here is backed by a logged run; pointers are at the end.

---

## The two questions we were really asking

1. **Can a group run itself with no one in charge?** If two phones are out of contact and each makes a
   change to "who's in the group," can they later come back together and agree on what happened —
   without a central server deciding for them? And when there *is* an always-on helper, is it just a
   convenience, or does it quietly become a boss?

2. **Does the plumbing actually work?** Can these devices find each other and move real data — even a
   phone behind a home router that can't be reached directly — and keep a group chat's history in a
   way that's flexible instead of rigid?

We got clear answers to both.

---

## 1. A group really can run itself (no server in charge)

We put the real "who's in the group" logic on all three boxes and cut them off from each other. Each
box independently made a conflicting decision in the same moment — one kept a member, one removed
that member, one added someone else. Then we let them exchange their histories and each one worked
out the result **on its own**.

**What happened:** all three machines reached the *byte-for-byte identical* conclusion, with no
server and no "tie-breaker" node involved. Where the decisions simply added up, they merged
automatically. Where they genuinely contradicted (one kept a person, another removed them), the
system **stopped and flagged it for a human** — it kept *both* versions, clearly labeled with who did
what, and refused to silently pick a winner. And the answer didn't depend on the order they
reconnected in.

**Why it matters:** this is the hard part of "decentralized" that usually gets hand-waved. We showed
that disconnected devices can converge on the same truth from only the histories they hold. Crucially,
the system never makes a *social* decision for you — it surfaces the conflict honestly and lets people
decide, then faithfully records what they chose. A removed member can even re-form a group with the
people who still want them, and the shared lineage makes that legible rather than a mystery.

## 2. The always-on helper is a *convenience*, not a *boss*

A real product wants an always-on node (a "superpeer") so phones that are rarely online can still
sync. The worry: does that node secretly become an authority you depend on?

We tested it directly. With the helper present, an offline phone's missed messages were held and
delivered when it came back — useful. Then we **turned the helper off** and the two phones reached the
exact same end state on their own, just with more back-and-forth. We also handed the helper a genuine
contradiction and confirmed it has **no special power to resolve it** — it stops exactly like any
ordinary peer would. And if the helper tries to alter a message it's holding, the tampering is caught
immediately.

**Why it matters:** the helper speeds things up and improves availability, but it can't do anything
the peers couldn't do themselves, and it can't forge or rewrite anything. In the language of the
project: it's a **capability, not a right**. That's the difference between "convenient" and "captured."

## 3. The plumbing works — including the hard NAT case

- **Big files, verified, resumable:** we moved a 1 GiB file between boxes, confirmed it arrived
  bit-for-bit correct, then killed the transfer halfway and restarted it — it **resumed** from where
  it left off instead of starting over.
- **More than one source:** two boxes offered the same file and a third pulled it; when we killed one
  source mid-transfer, the download **finished from the other** without missing a beat.
- **The real-world phone case:** the laptop behind a home NAT — which the cloud boxes *cannot* reach
  directly — still fetched a file and verified it, by routing through a public relay. This is the
  scenario every "phones talking to phones" product lives or dies on, and the earlier same-network
  tests never actually exercised it. (It took one fix: the invite has to carry the relay address, not
  just an ID — which is exactly the bootstrap pattern proven mobile apps already use.)
- **Group broadcast that heals itself:** three nodes joined a shared topic where two of them only knew
  how to reach the third. Messages still reached everyone (they spread across the mesh), and when we
  **killed the node in the middle**, the other two kept talking — the network had already learned
  other paths.

## 4. History is flexible, not rigid — and it's one mechanism for "my devices" and "the group"

This is the piece that sets the design apart. Most systems force everyone onto one official
transcript; if histories diverge, something is "broken" and has to be reconciled rigidly.

We do the opposite. Each device (and each person) keeps **its own signed history**, all anchored to a
shared starting point. Syncing is **voluntary**: you pull in someone else's history and it's kept as a
**separate, navigable thread** beside yours — never blended into one timeline, never overwriting what
you have. You can **fold** a thread away (it disappears from your daily view without being deleted) and
unfold it later. And the *same* mechanism works whether the other history is **your own second phone**
or **another member of the group** — there is no separate "device pairing" vs "group sync" machinery.

We proved this on the boxes: each device absorbed the others' histories as separate threads;
tampered history was **rejected** (a flipped signature is caught), and an outsider with no claim to the
group was **rejected** too. So anyone can offer you history without it becoming a way to attack you.

**Why it matters:** divergence becomes a *resting state* you can live in, not a failure. Your phone,
your laptop, and your friends' devices are all just branches off a shared root that you choose to weave
together — or not. That flexibility is the intended escape from the rigidity seen elsewhere.

---

## Honest limits (so the claims aren't oversold)

- The "who's in the group" logic and the history sync were tested by exchanging their data between the
  real machines and computing the result locally; we proved the *computation* is identical and correct
  across machines, but the *partition/reconnect* was modeled as file exchange, not yet run over the
  live network transport. (The transport itself is separately proven in the file/relay/gossip tests.)
- The encryption-group key mechanics (MLS) are modeled, not yet the real key schedule.
- Identity and key **recovery** — what happens when you lose all your devices — is still an open
  problem we did not solve here.
- One file split *simultaneously* across two sources needs a different internal representation than we
  used; we demonstrated source **redundancy/failover** instead, which is the property that matters for
  a flaky-phone world.

---

## Where the evidence lives

- `experiments/iroh/TEST-LOG.md` — the detailed, chronological run log (commands, outputs, findings).
- `Proofs/lineage-groups/PART_A_RECONCILE_FINDINGS.md` — group self-governance + capability-vs-right.
- `Proofs/lineage-groups/LOCAL_FIRST_HISTORY_FINDINGS.md` — multi-device & group voluntary history.
- `Proofs/lineage-groups/part-a-evidence/`, `…/local-first-history-evidence/` — raw artifacts.
- `experiments/iroh/TESTING-DESIGN.md` — the campaign plan and per-test status.
