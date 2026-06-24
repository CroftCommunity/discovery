# How the Lineage-Based Model Holds Up

author: research pass summary

date: 2026-06-14

## The short verdict

The design is genuinely strong on the problems most systems fumble, and honest about its own gaps. But it does not escape the field's central wall, the referee problem. It relocates that wall onto the superpeer. Fair grade: a real contribution, with one load-bearing claim that needs softening and one sleeper risk that needs testing.

You are building on the honest side of nearly every tradeoff, which is more than most of the field can say.

## Scorecard against the eight problems

**Beats outright (1)**

- Old messages for new members. You sidestepped the forward-secrecy conflict by deciding there is no single shared transcript, and backfill is something a person chooses to hand over. A true escape, because you changed the rules rather than claiming to beat them. The price (no guaranteed full history across your own devices) is one you already accepted.

**Softens (3)**

- Multi-device. Each device as a real member is the honest version, and counting people not devices correctly blocks the "out-vote a group with my own three gadgets" attack.

- Add and remove people. Treating a network split as a clean labeled fork instead of a crash, and stopping to ask a human on genuine conflict, is more honest than silently guessing.

- Moderation. Locking the voting rules at the group's birth, so there is a fixed answer to "who decides who decides," is your best original idea and more principled than anything in the pure-P2P field.

**Moves the pain elsewhere (1)**

- The referee problem. The important one. Covered below.

**Still has it (3)**

- Recovery after losing every device. You admitted this. No anchor yet.

- Getting too big. Records grow with devices, not just people, and every phone upgrade writes a permanent entry.

- Key-based identity is hard for normal people. No way around it.

## The one claim to fix

You cannot say "no central ordering authority."

The chain: your governance decision only counts once it is written as an MLS change; an MLS change is one step on a single track; two at once during a network split is the unmergeable fork the standards explicitly call unsolved. If the superpeer is what makes "pick the surviving version" reliable and available, the superpeer is your referee, at least part-time.

This is survivable. Keybase shipped your exact design (each device a member) and solved the ordering with a trusted server, not cryptography, and it was a real product. But the honest claim changes from "no referee" to "a referee that cannot read your messages and is optional, and things get stale and slow without it." Make that claim, not the stronger one.

## The single most useful test, before building anything else

Can two phones, fully cut off from each other and with no superpeer, independently arrive at the same answer for "which version of the split survives," using only the histories they each already hold?

- If yes: you have something genuinely new.

- If they need the superpeer to break the tie, or even to notice the split happened: that is your answer to the claim question, and you should say so.

Write the "pick the survivor" rule down and check whether it is pure math on the two histories or whether it needs an outside tiebreaker. The research suggests it will need the tiebreaker. Find out which before you build the rest.

## The sleeper risk

In the no-superpeer mode, every client has to replay an ever-growing pile of device-churn records (every phone added, every old phone removed, every split, every rejoin) just to draw the member list, with nothing sweeping the floor.

It will look fine in short, small-group tests. It breaks around month eighteen, in a churned and partitioned 30-person group, when a newcomer's app digests a 5,000-entry pile, gets the member list slightly wrong, and two people genuinely disagree about who is in the room.

Add that nasty test group to your suite now: heavy device swapping, several splits, long simulated runtime. It is the thing that is expensive to fix once people depend on it.

## Net

The work is not to find a cleverer escape from the referee problem. The experts say there is not one. The work is to state the superpeer's role plainly, and to harden the no-superpeer tier so it is a real fallback rather than a brochure.
