# Raw: Drystone human-adjudication language — design-intent voice dialogue (2026-07-20)

**Preservation status: preserved-condensed (cleaned-paste, content-faithful — NOT byte-pristine) —
PLAYBOOK §4.** Source: a voice-transcribed claude.ai session pasted 2026-07-20. The user flagged
**bleed-in audio from the TV show *Schitt's Creek* (~season 2)**; those interleaved TV lines are
removed and marked `[TV audio bleed removed]`, per the user's request to separate them. No secrets.
The session's *outputs* (the survey + the Claude Code instruction file) are frozen at
`../../human-unpacked/`; the adjudication-language pass they specify is **executed on main**
(conventions §A.11 + §B.8, Part 2 §7.6/§7.6.1, Part 1 §3 RFC corroboration). This raw preserves the
user's spoken reasoning — the design-intent behind that pass.

---

## Thread 1 — the fork-not-verdict case, and the RFC-precedent question (user)

On Drystone: there is an interesting part where there are **legitimate questions the machine is not
meant to answer** — here, mostly about social dynamics. Example: two legitimate group quorums trying
to take mutually-exclusive governance operations over a single user at once — say, delegate them a
role and ban them simultaneously. In that case the **ban wins** [the safety-conservative op]. But the
social-utility question — the *conflicting rule-users* — has **no winner**: there's a small **fork**.

`[TV audio bleed removed]`

It's likely the **accident case** — plausibly a higher-level governance SNAFU (adding someone to a
team and removing them at a higher level) rather than anger and spite. But **there is no way to tell
the motivation from a provenance point of view.** So you have to build in the possibility for
**infinite variety** — which means it **cannot be part of any finite protocol by design**, because
the center is a **glue whose job is to keep honest seams, not to solve every problem.** What we're
really designing is the **bridging of gaps between social events.**

`[TV audio bleed removed]`

**The publishing question:** if I wanted to pursue formally publishing this, are there **other
examples of RFCs written with this sort of need** — designing a system where there's a required
**human input**? To my mind it's like a **telephone switch operator**: it's **not a value/utility
judgment**, it's a **mechanical, deterministic judgment** — which is exactly why it was a great job
to give a machine. The closer analogy: a human hits **File → Save**. The machine can write the bytes,
but the **file's name and where it should live logically are utility judgments**. It's funny to think
of social adjudication as that kind of question, but ultimately it is — because from a
machine-deterministic point of view it is **unknowable**, so you **call out to a separate authority.**
I've looked at the example of **the guy in Chile** [Stafford Beer / Cybersyn] and his word for calling
out to a human adjudicator [the *algedonic* channel], but I didn't know whether that had ever been
**reconciled inside formal specifications in the internet age.**

> Session answer (captured, now in the spec): yes — RFC 8126 (IANA "Expert Review" / designated-expert
> role), W3C Permissions/Geolocation (user-consent gates), RFC 7282 (IETF rough consensus / "humming"),
> and Beer's algedonic channel. Drystone's inversion: distribute the designated-expert role to every
> principal. Added beside the Beer entry in Part 1 §3.

## Thread 2 — the operator is NOT the villain (the load-bearing correction)

Correcting the survey's first framing: **"operator" is not the villain**, even though "no operator to
trust" reads as loaded. As the protocol notes, **trust is a gradient — trust *for what*.** The point
is there is **no operator to trust *for social utility*** — no path for an operator to **cross both
planes** and influence unduly, transparently or not, against the wishes of group members or a person's
social graph. **Operators are necessary, even good;** a good operator is aligned to the fact that these
are **separate planes of authority.** "Operator-needed recovery" just means reducing the transaction to
the technological — there is a social component *above* it, but **not intra to the groups themselves**,
and that has to be a **hard line.** The operator is **a role in the system** — like group roles that
are mutually exclusive, and system roles that are mutually exclusive. Not a villain.

Also: the **LWW / conflict-resolution-by-timestamp** language is a **no-go** — a rogue; clean it out so
it stops confusing people. And: update the human-adjudication language based on this feedback.

> This correction is what reshaped the vocabulary: the reason one word can't cover both roles is **role
> exclusivity across planes**, not operator badness. Landed as conventions §A.11 (DR-4/DR-10 plane-
> separation, "the local authority" coined for escalation's receiving side, "planes of authority"
> coined) + the §7.6 naming + the iroh LWW cleanup. See `../../human-unpacked/` for the survey +
> instruction file, and `beta/drystone-spec/conventions-and-decisions.md` §A.11/§B.8 for the landed pass.
