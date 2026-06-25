# Raw: Rights-vs-capabilities definitional dialogue (2026-06-24)

**Source:** claude.ai design dialogue, pasted into a CroftC filing session.
**Date:** 2026-06-24.
**Preservation status:** `preserved-condensed (cleaned-paste)` — **content-faithful, NOT a byte-pristine
export** (PLAYBOOK §4). UI render chrome stripped (the "Created 2 files, ran a command, read a file"
indicator, the "Defining the terms the boundary depends on" artifact-render frame, the `Jun 24` dividers,
the "How should I draft this?" Q/A widget). The pasted excerpt of `beta/01-epistemic-foundation.md` that
opened the session is the *current beta state* (for reference) and is **not** re-filed here — it lives at
`../../../../beta/01-epistemic-foundation.md`. Distilled to
`../../../thinking/rights-vs-capabilities-definitions.md`.

**Context:** continues the Drystone "why" work. Grounds 01 §5's boundary ("no right to remove the rights of
others") by making the rights-vs-capability line sharp. Connects to 01 §6.1 (capabilities-not-rights / K6),
the 05 rights-floor, 06 label-not-enforce, and the 04 survivor re-key.

---

## USER

Need to think defining these rights concretely so "cannot remove the rights of others" is grounded — what is
a right vs what is a capability.

## ASSISTANT

This is a definitional problem, and the document already gives you the seam to pull on. It distinguishes
rights ("standing: tenure, exit, voice, share") from roles ("governed delegation"), but it never defines
*capability*, and the boundary rule ("no right to remove the rights of others") can't be load-bearing until
that line is sharp.

**The discriminating test: does removal cancel the conditions of its own contestation?** The wolf test
already supplies the criterion. A right is any standing whose removal is **self-amplifying toward collapse**,
because it lowers the variety available to resist the next removal. A capability is a power whose removal
leaves contestation intact. So the definition is a property, not a list:

- **A right is standing that must survive in order for any dispute about it to remain contestable.** Remove
  it and the holder loses the means to object to the removal.
- **A capability is an ability to act or affect outcomes that does not, when removed, disable the holder's
  standing to contest the removal.**

This stays consistent with the local-first derivation: rights are the things that, being local-first state
the person holds, cannot be reached into; capabilities are mediated through relationships and roles, so they
move.

**The four rights, defined by irreversibility-from-the-center.** Each defined by what its removal would
foreclose:

- **Tenure** — your standing to remain a peer. Identity-as-local-first-state (the 05 rights-floor: you cannot
  be cleared because your standing is not held elsewhere). Removal = erasure of the unit.
- **Exit** — the right to fork. Local-first's native move when peers disagree, and the dignified alternative
  to being silenced (the Mill grounding). Removal = the dissenter has nowhere to go (the thing *On Liberty*
  names as theft).
- **Voice** — standing to assert into the record and be corroborated or refuted. Removal = your assertions
  are made invisible, which defeats provenance itself.
- **Share** — your claim on the collective's commons. Removal = expropriation, the enclosure inversion 02 is
  built against.

**What capabilities look like, by contrast.** Roles, delegated authority, moderation powers, admin
functions, write-access to a shared resource, a vote weighting in a particular governance scheme. Each can be
granted, narrowed, rotated, or revoked without disabling the holder's standing to object. If you lose admin
on a collective, you can still contest that loss using tenure, voice, and exit. That is the tell that it was
a capability.

**One edge case worth resolving now.** Voice is the hard one, because the line between "removing voice"
(rights violation) and "declining to amplify / labeling / refusing to corroborate" (legitimate, per 06's
label-not-enforce) is exactly where the content-blind safety posture lives. The clean cut: **voice is the
right to assert into your own record and reach willing peers. It is not a right to compel any specific peer
to carry, host, or amplify you.** Removing your ability to assert at all = rights violation. A peer declining
to relay you = that peer exercising their own standing. This keeps voice from collapsing into "a right to an
audience," which would itself be a clearance of others' standing not to listen.

**Two things to verify before this hardens into the spec.** First, confirm that "share" is actually a right
in the cooperative model and not a role-like entitlement that varies by membership class — if share can be
diluted by legitimate governance, it behaves partly like a capability and you'll want to say which portion is
the inviolable floor. Second, check tenure against the re-key / survivor mechanism in 04: if a survivor
re-key can in practice strand a peer, then tenure has an implementation-level exception you need to name
explicitly rather than leave the rule absolute.

Want me to draft this as a section for 01, or as a standalone definitions block 05 and 07 can both cite?

## USER

Sure

## ASSISTANT

Which fits better? A definitions block is cite-able from multiple themes but reads as reference material. A
prose section in 01 carries the derivation but means 05 and 07 cite "01 §X" rather than a named artifact.

Q: How should I draft this? — A: **Both: block + a short framing para in 01.**

I'll draft both — the standalone definitions block first, then a short framing paragraph that sits in 01 and
points to it.

The definitions block leads with the discriminating test (removal-cancels-contestation) so the distinction
is a property rather than a memorized list, then gives the four rights as a closed set and capabilities as an
open class, resolves the voice edge case, and parks the two verify-before-hardening items as explicit open
work for 05 and 07. The framing para is one paragraph for 01 §5 that states the cut and the §3 connection,
then hands off to the block.

One thing left as a deliberate open item rather than decided: whether **share** is fully a right. The block
flags it for 07 because the cooperative model may dilute share by membership class, which would make part of
it behave like a capability. If you already know the answer, that's a quick edit to close.

### Generated artifact: the framing paragraph for 01 §5 (verbatim)

> **Defining the terms the boundary depends on.** The boundary ("no right to remove the rights of others")
> only bites once "right" and "capability" are sharp, and the line between them is not a list but a property.
> A right is standing that must survive for any dispute about it to stay contestable: remove it and the
> holder loses the means to object to the removal, which is why rights-removal is the only self-amplifying
> move toward collapse. A capability is a power to act or affect outcomes whose removal leaves the holder's
> standing to object intact. This is the same cut as the local-first derivation in §3 — rights are
> local-first state the person holds and so cannot be reached into, while capabilities are mediated through
> roles and relationships and so move freely. The four rights (tenure, exit, voice, share) are each defined
> by what their removal would foreclose, not by what they grant, and capabilities are the open remainder:
> roles, delegation, moderation powers, write-access, vote weighting, relay. The full definitions, the
> resolved voice-versus-amplification edge case, and two open items for 05 and 07 live in the
> rights-and-capabilities definitions block.

[The standalone definitions block was also generated; its content is reproduced/distilled at
`../../../thinking/rights-vs-capabilities-definitions.md`.]
