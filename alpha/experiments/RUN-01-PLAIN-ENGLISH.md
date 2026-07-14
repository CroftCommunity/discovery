# RUN-01, in plain English

`A jargon-free companion to RUN-01-SUMMARY.md. Same facts, told for someone who hasn't been living in
the spec. Branch: claude/experiments-run-01, 2026-07-14.`

## The point of this run

We're building a group-messaging system where there's **no central server** deciding who's in a group
or what the rules are. Every device works it out for itself from a shared log of signed facts. The
promise is that if two honest devices see the same facts — even in a different order — they end up
agreeing. This run was a batch of five experiments to *check whether that promise actually holds* in
places we'd only assumed it did, and to be brutally honest when it doesn't.

The golden rule we worked under: **a passing test is only worth anything if it's honest about what it
really proved.** A green checkmark that quietly leaned on a shortcut is worse than a red one. So every
time we leaned on a stand-in, we wrote it down in a public register.

Five experiments. Three went well, one uncovered a genuine bug, one we deliberately stopped before
guessing at something we shouldn't guess at.

---

## EXP-1 — Does the cost stay sane as a group grows? ✅ (with an honest caveat)

**The question.** When more people are in a group, does the messaging cost grow gently, or does it blow
up? We spun up 2, then 4, then 8, then 16 copies of the app on one machine, all talking over the real
networking layer, and measured.

**What we found.** Each device's own workload grows in a **straight line** with the group size — tidy
and predictable, exactly what we wanted. Everyone reliably ended up agreeing on the conversation at
every size we tried.

**The honest caveat.** There's one operation — a device *catching up* the moment it reconnects — where
the "host" device does a lot of extra work, and that work grows **much faster** than the group size.
Past about 8 devices, the group agreed on the *content* but hadn't fully finished tidying up its
bookkeeping in the time we gave it. This isn't a disaster (nobody disagreed), but it's a real signpost:
it points straight at a smarter catch-up mechanism the design already has on its wish-list, and gives us
hard numbers for *why* it's worth building.

---

## EXP-2 — Does a key third-party library still behave on its newest version? ✅

**The question.** We rely on a library called Automerge for one subtle, safety-critical behavior: if a
device receives a puzzle piece but is *missing* the piece it depends on, it must hold the new piece back
and show **nothing** — never a half-built, misleading picture. We'd only ever proven this on an old
version of the library because the newer one wouldn't build on our old toolchain.

**What we found.** The toolchain is modern now, so we upgraded to the shipping version and re-ran all
four checks. **All four still pass, identically.** The safety behavior we depend on holds on the version
we'll actually ship. One less "we're pretty sure" in the corpus; one more "we checked."

---

## EXP-3 — Are the security-critical checks actually tested? ✅ (mostly — see the footnote)

**The question.** There's a technique called *mutation testing*: a tool deliberately introduces small
sabotages into the code — flip a "greater-than" to a "less-than," make a security check always say
"yes" — and reruns the tests. If the tests still pass, that line of code isn't really protected by any
test. We aimed this at the code that decides **who is allowed to do what** and **whether enough people
approved a change** — the trust core.

**What we found — the good part.** The code that **counts approvals** ("did 2 distinct people really
sign off?") is rock-solid: every sabotage we tried was caught by a test. That's the part where a
silent bug would be most dangerous, and it's well-guarded.

**What we found — the footnote.** A lot of the *permission* checks ("is this person an admin?") looked
*unprotected* by this tool. But that's a known illusion: those checks are tested — just by tests that
live in a **different part of the codebase** that this particular tool run couldn't see. We proved this
concretely: we hand-sabotaged one such check and watched the "other" test suite catch it immediately.
So there's no real hole — but the tooling that would show this automatically, in one pass, is still on
the to-do list.

---

## EXP-4 — Two small fixes... and one real bug 🐞

**The good half. ✅** When the system hits a genuine deadlock — two people kick each other out at the
exact same moment, and there's no fair way to decide who wins — it's supposed to **freeze and ask a
human**, and label that frozen state with a name that both devices compute *identically* (so they agree
they're stuck on the *same* thing). We confirmed the naming is exactly right and order-independent.

**The bug. 🐞** Now the real find. Imagine two separate groups of admins, at the same moment, each
properly vote through a **conflicting rule change** — one says "set the limit to 5," the other says "set
it to 9." Both votes are legitimate. The system is *supposed* to freeze and ask a human, because there's
no fair way to pick. Instead, **it silently picks a winner based on pure luck** — whichever message
happened to be processed last wins, and the two devices can end up with *different rules and no warning*.
That's exactly the kind of silent disagreement this whole system is meant to prevent.

We wrote a test that pins this bad behavior in place (so it can't sneak back, and so a future fix has
something to turn green), and flagged it loudly in the register. We did **not** fix it, on purpose:
*how* to fix it is a genuine design decision (which kinds of conflict should freeze, exactly?), and the
rules of this run say we don't make design decisions on our own — we write down the options and hand
them to a human. This is the most valuable thing the run found.

**The third item** (should approvals require a minimum rank?) turned out to also be an undecided design
question, so we wrote up the options and left it for a human rather than guessing.

---

## EXP-5 — The big one we deliberately didn't start ⛔

**The question.** Replace two remaining "pretend" pieces with the real thing: (1) really distributing
group encryption keys over the network, and (2) making "revoke this device's access" require a real
*group* of authorizers signing off, instead of the current placeholder.

**What we found.** Piece (1) turns out to be **already built** in an earlier experiment — good news.
Piece (2) runs straight into the **single biggest open design question** in the whole project: *who is
allowed to revoke access, how many of them does it take, and how do devices know whose signatures to
trust in the first place?* That's not a coding task; it's a foundational decision about identity and
recovery that has to be made deliberately by a human.

So we **stopped** — as the instructions explicitly told us to when we hit exactly this kind of gate. We
wrote up three concrete options (with a recommendation for the one that *doesn't* depend on solving the
whole identity problem first) and left the decision where it belongs.

---

## The one-paragraph version

Group-messaging costs scale sanely per device (EXP-1); a safety-critical library still behaves on its
shipping version (EXP-2); the trust code's approval-counting is genuinely well-tested and nothing is
secretly broken (EXP-3); the "freeze on a genuine deadlock" behavior is correct in one case but **has a
real hole for competing rule-change votes** — which we found, pinned, and flagged rather than papered
over (EXP-4); and the last, largest experiment was correctly **not started** because it depends on a
human-only design decision (EXP-5). The headline for next time is the EXP-4 bug: two rival votes can
silently disagree, and fixing it needs a design call.
