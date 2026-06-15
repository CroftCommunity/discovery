# The Achilles Heels of Group Chat, in Plain English

author: research pass, run against the lineage-based model

date: 2026-06-14

This is the same analysis as before, rewritten so each idea comes with a concrete example or a comparison to something familiar. The findings did not change. The translation is the point.

One thing to say up front, in plain terms. You were worried (Question 2) that your optional "superpeer" might secretly be doing the job of a central server while you tell everyone it's decentralized. That worry is correct, and it's not just your worry. The people writing the official standards for this exact problem say the same thing in their own documents. So the headline is: you are not hiding a flaw, you are bumping into a wall that the whole field has bumped into. The honest move is to describe the wall accurately rather than claim you walked through it.

---

## Part 1: Why group chat keeps breaking, with examples

### 1. Using the same account on your phone and laptop

**The plain problem:** You'd think "let me read this group from both my phone and my laptop" would be easy. It is one of the hardest things in this entire field.

**Why it's hard, by analogy:** Imagine your identity is a diary that you can only ever add pages to, never tear pages out or reorder them. That's an "append-only log," and it's how these systems prove nobody tampered with your history. Now give two people (your phone and your laptop) the same diary and let both write in it at the same time. Page 47 now exists in two different versions. The diary is corrupted, because the one rule was "pages only get added, in order." That's the core reason one identity can't easily live on two devices.

**How each system dealt with it:**

- **Briar just said no.** Briar's own developers describe multi-device as an unsolved problem, because using one account from two devices creates exactly that two-versions-of-page-47 mess. Your Briar account lives on one phone, full stop.

- **Secure Scuttlebutt (SSB) gave each device its own diary, then tried to staple them together.** Because two devices can't share one append-only feed, every device got a separate one. "Fusion Identity" was the staple: a way to say "these three diaries are all me." It was designed but never really shipped as the normal experience, and the security audit found that when a fusion identity went bad, sorting it out required people to vote, by hand, over some other channel, about which device to trust.

- **Signal made it easy by keeping a boss in the middle.** Signal's server keeps the list of your linked devices and hands out the keys. That central coordinator is *why* it feels seamless, and it's exactly the thing you don't have.

- **Keybase did the closest thing to your design and shipped it.** Each device is its own key, listed in a public append-only record, with a shared "master key" layered on top so any of your devices can read everything. This matters a lot for you, because it's your "each device is a member" idea, in production, years ago. The interesting part is how they made it safe, which comes up in section 3.

**The four options, plainly:**

- Everyone shares one key: simple, but you can't kick out one lost phone without re-keying the whole group.

- A server tracks your devices: easy to use, but now the server knows who you are (Signal).

- Each device is its own member: kicking out a phone is natural, but the group's member list now includes every phone, tablet, and laptop, and something has to quietly group them back together as "one person" (Keybase, and you).

- Hand off once, then drift apart: you copy your account to the new device once and after that they're independent (Delta Chat does this on purpose; Session does it by sharing the seed phrase).

### 2. Losing your device, and getting back in

**The plain problem:** If your identity is a secret key on your phone and your phone falls in a lake, how do you get back? Every answer either makes you less safe or quietly puts someone in charge of rescuing you. Nobody has found a third way.

**The spectrum, with who sits where:**

- **"You're gone."** Briar and SSB. Lose the phone, lose the identity. Briar tells you this plainly: uninstall or forget your password and there is no recovery. Safe, but harsh.

- **A backup phrase.** Session gives you a 13-word recovery phrase that *is* your key. Anyone who reads it becomes you, and you copy it between devices to do multi-device. There's a sharp edge here worth knowing: Session's first version locked everything with one permanent key that never changes unless you start over, so it had no forward secrecy at all. Their version 2, still being built as of late 2025, is trying to fix that.

- **A PIN backed by a server.** Signal. You can recover, but only because Signal's servers hold the material that lets you.

- **A key backup.** Matrix. Recoverable, but now the backup itself is a thing thieves can target.

**Why it's unavoidable, by analogy:** Recovery is like getting back into a house after losing your only key. Your choices are: someone else kept a spare (now you trust that person), or you hid one under a rock that you can describe from memory (weaker, and someone else might find the rock). "Decentralized" means you fired the locksmith who normally holds the spare. The need for the spare didn't go away.

### 3. Adding and removing people, especially at the same time

**The plain problem:** When someone joins or leaves, the group has to change its shared locks (a "re-key"). The nightmare case is two admins changing the membership *at the same time while the group is split in two* (say, a conference Wi-Fi outage cut the room in half). This isn't sloppy engineering, it's genuinely hard.

**Why, by analogy:** Picture the group's history as a single train track. Each membership change lays the next section of track. If two people, on two sides of a tunnel they can't see through, both lay "the next section," you now have two contradictory next-sections and no switch to join them. The standard (MLS) has no merge operation. Two valid changes for the same moment is a split, not a sum.

**What each approach trades away:**

- **Signal / WhatsApp ("sender keys"):** cheap to send messages, but removing someone leans on the server to coordinate who's currently in.

- **Matrix ("Megolm"):** here's a nuance to keep straight. You'll read "Megolm has no forward secrecy." That's Wire (a competitor) doing marketing. The accurate version: Megolm protects messages in *blocks*, so a stolen key exposes a chunk of recent messages, not all past ones, and it doesn't on its own recover security after a break-in. The app has to keep starting fresh sessions to limit the damage.

- **MLS (the modern standard):** proper security and it scales well, but it assumes something puts the membership changes in a definite order. That assumption is the thing you can't escape.

**The Keybase lesson, in plain terms:** Keybase needed to prove "this device made a change *after* it was added and *before* it was removed." Their solution was not clever math. It was a public append-only ledger *plus a server that everyone agrees to treat as the official record*. They had your precise problem and the answer they shipped was a trusted referee. Hold that thought.

### 4. The dirty secret: someone has to be the referee

**The plain finding, no hedging:** Deciding group membership in order is, deep down, the same problem as getting a committee to agree on what happened and when. That's "consensus," and it's famously hard without someone (or some quorum) acting as referee. Most systems that call themselves decentralized quietly keep a referee and don't put it on the poster.

**The evidence, in plain English:** The official MLS architecture document admits that if you remove the central "delivery service," the *peers themselves* now have to provide everything that service did, including agreeing on order. Removing the referee doesn't remove the job. It just makes the players do it, which means they have to agree, which is consensus.

The official Decentralized MLS draft is even more direct, and this is the sentence that should grab you. To run MLS without a central orderer, members have to *hold onto old keys longer* so they can deal with changes arriving out of order. Holding old keys longer is exactly what forward secrecy tells you never to do, so you lose security. Newer research (FREEK) wins most of that security back. But the draft says plainly that even after that fix, once the group has split into two versions, none of this tells you how to merge them back into one agreed version. In their words, there's no single rule for it.

**Why this is the whole ballgame for you:** That unsolved "merge two versions back into one" is precisely your "deterministic survivor selection" step. The standards bodies have a name for it and a flag on it saying "unsolved." So when your governance op only counts once it's written as an MLS change, and two of those happen at once during a split, you are standing exactly where the experts have planted their "here be dragons" sign.

### 5. Letting new members read old messages

**The plain problem:** Forward secrecy means "old keys can't open old messages," which is what protects you if you're robbed later. But "let the newcomer scroll up and read last month" requires old messages to *still be openable* by someone. These two wants are in direct conflict.

**By analogy:** It's the tension between "shred every document the moment it's read" (safe) and "keep a readable archive for new hires" (convenient). You can lean one way or the other, but you can't fully have both in the same filing cabinet. Signal and MLS lean toward shredding. History-keeping apps keep a re-readable copy and accept the weaker guarantee.

### 6. Things getting too big

**The plain problem:** SSB is the ghost of Christmas future here. Its "diaries" only ever grow, and every friend's device stores a full copy of everyone's, forever. It got heavy and slow. MLS handles the locks more efficiently, but its internal "tree" still grows with the number of members, and in your design it grows with members *times every one of their devices*. More on why that's the sleeper problem at the end.

### 7. Why "your identity is a key" is hard for normal people

**The plain problem:** Telling a normal person "you are this 64-character key, don't lose it, there's no reset" is a tough sell (SSB, Briar, and Session all feel this). Phone numbers are easy precisely because the phone company and the app's server handle your identity and your recovery for you, which is the privacy cost. Your "DID lineage" sits on the hard, key-based side. That's a real adoption tax, and worth naming honestly rather than wishing away.

### 8. Who gets to kick people out

**The plain problem, and why it's underrated:** This is the least-studied killer and I think you're right to worry about it. Most apps treated "moderation" as an admin button on a server. Take away the server and you have to bake "who can kick whom" into the protocol itself, where it becomes something attackers poke at. SSB famously can't really delete or globally moderate anything, because there's no authority to enforce a takedown and everyone already has their own copy. Briar dodges by saying "only the group's creator can invite." Nobody in the pure peer-to-peer world has a good answer to a hijacked majority or someone making new accounts to evade bans, because those are governance questions and the field kept treating them as settings screens.

---

## Part 2: Your design, graded against each problem

Quick key: **escaped** (you genuinely beat it), **partially escaped** (you soften it), **relocated** (you moved the pain somewhere else), **inherited** (you have it too).

**1. Multi-device — partially escaped.** Making each device a real member is the honest version of this, and "count people, not devices, when you need a majority" correctly stops someone from out-voting a group using their own three gadgets. The catch: you've moved the mess into the member list and the step that re-groups all those devices back into "one person." Keybase shows this works, but also shows what it costs (Part 3).

**2. Recovery — inherited.** You already admitted you don't have a "lost every device" story. You don't, and that drops you next to Briar and SSB: gone. Options in Part 4, none free.

**3. Add/remove people — partially escaped.** Treating a split as a clean, labeled "fork" instead of a crash is genuinely nice, and "if two sides truly disagree about who's a member, stop and ask a human" is more honest than silently guessing. But the moment of *enacting* the change is where the referee problem (4) sneaks back.

**4. The referee problem — relocated, and this is the big one.** Walk the chain: your governance decision only counts once it's written as an MLS change; an MLS change is one step on that single train track; two at once during a split is the unmergeable fork the standards call unsolved. So your "pick the surviving version automatically" rule is quietly doing the referee's job. If the superpeer is what makes that pick reliable and available, then the superpeer *is* your referee, at least part-time. That's survivable. Keybase had a referee server and was a real product. But it changes your honest claim from "no referee" to "a referee that can't read your messages and is optional, and when it's gone things get stale and slow." Say that version.

**5. Old messages for new members — escaped, fair and square.** You dodge the conflict by saying there is no single shared history to begin with, and backfill is something a person chooses to hand over. That's a real escape, because you changed the rules of the game rather than claiming to beat them. The price (no guaranteed full history across your own devices) is one you already accepted.

**6. Getting too big — inherited, maybe worse.** Because every device is a member, your records grow with devices, not just people, and every phone upgrade writes a permanent "added, then removed" entry. This is the SSB growth problem wearing your hat.

**7. Key-based identity is hard — inherited.** No way around it, and no claim that there is.

**8. Moderation — partially escaped, and your best original idea.** Locking the voting rules at the group's birth, so there's a fixed answer to "who decides who decides," is more principled than anything in the pure-P2P field. But locking them is also a trap, covered next.

---

## Part 3: The strongest ways your design could fail, ranked

**1. The superpeer becomes the only tier anyone actually uses.** The likeliest failure is mundane: everything works great with the superpeer and turns painful without it, so your "two options" quietly become "the real one and the brochure one." History is unkind here. SSB had optional "pub" servers that became load-bearing. Matrix has optional self-hosting that almost nobody does. When the convenient centralized-ish option exists next to the purist option, people pick convenience every time. If picking the surviving version, carrying re-keys, and storing snapshots all happen on the superpeer, you've rebuilt a central server with extra steps, and Keybase's honest framing fits you better than Briar's. Test the no-superpeer mode early and try hard to break it, because that mode is where your headline claim lives or dies.

**2. Locking the rules at birth won't survive a real group's life.** A group of 3 sets "it takes 2 people to kick someone." It grows to 30 and that rule is now absurd, and you deliberately made it unchangeable. A founder who goes rogue, a clique that captures the vote, or just a bad initial guess is now permanent. Your only escape hatch is "fork away and start fresh," which means your answer to "our rules are broken" is "abandon ship and rebuild," a brutal thing to make routine. The field's moderation failures teach that rules need to be changeable *under their own terms*, and you traded that away for tidiness.

**3. The two cleanest pieces have one messy weld.** You've got six neat parts. The messy weld is where each device's "I belong to this person" credential has to ride inside the MLS layer, because MLS only understands devices-and-keys, but your voting rule needs to understand people. So the low-level crypto layer has to carry around high-level identity info, and your voting logic has to trust a link that lives down in the crypto. Two clean rooms, one leaky pipe between them that both depend on. Ambitious P2P projects usually die at the welds, not inside the well-built rooms. Build this weld first, because it's where the hidden cost is.

**4. Recovery has no anchor, and you'll get pressured into adding a rescuer.** Lowest because it's a known gap, not a hidden one. But "no recovery" only flies for activists and journalists (Briar's actual users). Want anyone broader and you'll face pressure to add a rescue path, and the natural spot is "a few of your trusted contacts can vouch a new device back in," which drags a trusted quorum right into the layer you wanted kept pure.

---

## Part 4: Open questions, and your realistic options

**Getting back in after losing everything.** Your real choices, with the catch on each:

- **Trusted contacts vouch you back in** (a set number of them, fixed at the group's birth). Catch: those same contacts could gang up and steal your identity, and it only works while the group still exists.

- **A master phrase in a drawer or hardware key.** Catch: that's Session's backup phrase with a new name. One thing to lose, one thing to steal, and it sits outside your tidy per-device story.

- **Accept "you're gone."** Catch: you're now Briar, usable only by people who accept that, which kills any mainstream hope.

There's no option that stays purist *and* lets normal people recover. Decide which crowd you're building for and say it out loud.

**The question to answer before any other:** Can two phones, fully cut off from each other and with no superpeer, *independently arrive at the same answer* for "which version of the split survives," using only the histories they each hold? If yes, you've got something genuinely new. If they need the superpeer to break the tie or even to notice the split happened, then the superpeer is your referee and you should say so. Write that "pick the survivor" rule down and check whether it's pure math on the two histories or whether it needs an outside tiebreaker. The research suggests it'll need a tiebreaker, so find out which it is before you build anything else.

**Can you have it both ways on the locked rules?** Maybe let the birth-rules be changed, but only under a stricter birth-rule (for example, "the voting rules can only change if every person agrees"). That might keep the "grounded at the root" tidiness while fixing the brittleness. Design this before you commit to hard locking.

**The messy weld.** Write down exactly what the crypto layer has to expose and what the voting layer has to trust, and treat that handoff as a guarded border with its own threat model, not a detail. That's where clean becomes messy.

---

## The trap you didn't ask about

You asked what kills designs like yours that you're not even thinking to test. From these histories, it's not a broken lock. It's this: **your group's records pile up forever from device churn, and the step that turns those records into "here's who's in the room right now" quietly buckles.**

In plain terms. Every device is a member, so every new phone is an "added," every old phone is a "removed," every split leaves a marker, every re-join leaves a marker, and nothing ever sweeps the floor (you removed the server that would have). To show a simple member list, every person's app has to read and replay that entire growing pile. Keybase survived this *because a server did the sweeping and kept the official tally.* In your no-superpeer mode, every phone does it alone, over a pile that grows with people times devices times time times number-of-splits.

Why you won't catch it in testing: your tests will use small groups, for short stretches, with few devices, and it'll look fine. The failure shows up around month eighteen, in a 30-person group where everyone's swapped phones twice and there've been a handful of network splits, when a newcomer's app has to digest a 5,000-entry pile to draw the member list, gets it slightly wrong, and now two people genuinely disagree about who's in the room. That's SSB's "the logs ate us" problem and the field's "moderation isn't a settings screen" problem arriving hand in hand, at the one step you've made every single client depend on. Put a nasty test group in your suite *now*: lots of device swapping, several splits, running for a long simulated time. That's the thing that's expensive to fix once people are relying on it.
