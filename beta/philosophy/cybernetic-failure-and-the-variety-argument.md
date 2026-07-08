# Cybernetic failure and the variety argument

`Status: philosophy layer (Layer 2, why it is right). Register: argument / framing. Resolution: the
philosophy-layer statement of the variety argument in its crispest form — the definition of a cybernetic
failure and why it relocates blame from people to architecture. The Drystone spec's Part 1 §3 realizes the
mechanics this frames (requisite variety made architectural, the algedonic channel, the hard-stop-and-
escalate posture); this document does not restate them. External facts (Beer, Ashby, Glushkov) are
AI-surfaced; quotations and the Glushkov figure carry verification flags and must be confirmed against
primary editions before external use.`

## Overview

The strongest thing the design can say about centralization is not that it is unfair, or badly run, or
staffed by the wrong people. It is that centralization *fails as regulation* — that under a formal law of
control, a single center cannot hold enough internal variety to match the variety of what it governs, so
some disturbances have no matching response and the system drifts out of control regardless of who is at
the top. This document states that argument in its crispest form and gives it a name: a *cybernetic
failure*. The load-bearing move is what the name buys — it **relocates blame from people to architecture**.
A system staffed by brilliant, honest people still fails cybernetically if the failure lives in the
channels and the topology. That is why, for Croft, the response is not "install better leadership" but
"put the invariant in the architecture": if the failure is structural, only a structural answer holds.

This is the philosophy-layer framing. The mechanics that carry the same conclusion into the wire protocol —
requisite variety made architectural, the algedonic escalation channel, and the *hard-stop-and-escalate* /
*label-not-enforce* postures — live in the spec (`../drystone-spec/part-1-reasoning-underpinnings.md` §3)
and are cross-referenced here, not repeated.

## Charter

- **In scope:** the definition of a *cybernetic failure* as a failure of regulation; the illustration that
  makes the variety mismatch visible (Glushkov's OGAS estimate); the intellectual lineage that generalized
  it from a Soviet particular into a structural law (Trotsky → Beer); *Cyberfolk* as the unbuilt boundary
  case of the argument; and the load-bearing conclusion for Croft (centralization failure is structural,
  not moral or a competence problem).
- **Out of scope (and where it lives):** the requisite-variety-as-architecture mechanics, Ashby's law
  as stated, the algedonic channel's design mechanics, and their mapping to the spec's escalation and
  labeling postures — all in `../drystone-spec/part-1-reasoning-underpinnings.md` §3. Also out of scope:
  the downstream conclusion that *peerhood is a locus of adjudication* (a separate argument that this one
  feeds but does not make here).
- **Boundary call:** this is the "why centralization fails, structurally" register — the argument a
  newcomer must be able to re-derive from first principles. The spec carries the mechanics the design
  realizes; this document carries the reasoning those mechanics exist to serve.

## The definition: a *cybernetic failure*

A *cybernetic failure* is a failure of *regulation*. A system loses the ability to keep itself viable —
not because anyone made a bad call, but because its information structure **physically cannot do the job
that control requires**. It is a structural diagnosis, not a moral or a competence one.

The argument runs through Ashby's Law of Requisite Variety (stated, with the primary quotation, in the
spec's §3; not repeated here). Control is the capacity to produce a matching response to each disturbance
the environment throws up. A regulator can only do that if it holds at least as much variety — as many
distinct internal states — as the thing it regulates. A *cybernetic failure* is precisely the case where
that condition breaks: the regulator sits below the variety it has accepted responsibility for, so some
disturbances have no matching response, and the system drifts out of control. Nobody has to be lazy or
corrupt. The math simply does not close.

This is the whole force of naming it. Because the failure lives in the channels and the topology rather
than in the people, **swapping in better leadership never fixes it** — a fresh apex inherits the same
variety deficit the moment it sits down. The only repairs that touch the actual cause are structural:
change where variety is absorbed, or change where adjudication lives. That is the sense in which the
argument is not a critique of any particular institution but a constraint on a whole class of designs.

## Glushkov's arithmetic: the mismatch made visible

The cleanest statement of the mismatch is not a slogan but a number. Viktor Glushkov, designing the Soviet
OGAS economic-planning network in the 1960s, calculated that continuing to manage the economy by hand would
eventually require on the order of **ten billion people computing by hand** `[confirm]`. The regulator — a
central planning apparatus — was being asked to embody a variety it could never physically hold. That gap
*is* the cybernetic failure, rendered as an arithmetic impossibility rather than a value judgment.

The figure is load-bearing precisely because it is a count. It converts "centralization is brittle" from
an aesthetic complaint into a claim about numbers of states, which is what makes the argument survive
disagreement about politics. Whatever one's view of central planning as an ideal, ten billion hand-computers
is a regulator that cannot be built, and requisite variety says a regulator that cannot match its
environment's variety cannot control it.

## Trotsky → Beer: bureaucracy as a *variety bottleneck*

Stafford Beer did not arrive at this from arithmetic alone. His stance was explicitly anti-authoritarian,
and he had read Trotsky's critique of Soviet bureaucracy — the argument that the bureaucracy had ossified
into a self-serving caste that strangled the system it ran `[confirm]`. Beer's contribution was to read
that not as a *political* accident particular to the Soviets but as a *cybernetic* one: the bureaucracy was
a *variety bottleneck*, a center whose channel capacity sat below the variety of the economy beneath it, so
the failure was structural and would recur in any design of the same shape.

He framed the stakes of getting this wrong in terms of freedom, not efficiency:

> "Where the wickedness lies is that ordinary folk are led to think that the computer is an expensive and
> dangerous failure, a threat to their freedom and their individuality, whereas it is really their only
> hope."
> S. Beer, *Designing Freedom* (1974) · **[UNVERIFIED, confirm against primary edition before publish]**

The generalization is what carries into the design: a centralized socio-technical system does not fail
because it is Soviet, or because its planners are venal. It fails because a single center is a variety
bottleneck by construction, and requisite variety is a law rather than a tendency. Trotsky gave Beer a
vivid early case; Beer turned it into the structural argument. For Croft, that is the whole point — the
indictment is of the *shape*, so no better occupant of the apex rescues it.

## *Cyberfolk*: the unbuilt boundary case

Beer's own designs answered the variety problem by pushing autonomy to the edge and reserving escalation
for the residue a center genuinely had to see. The furthest extension he imagined, and never built, was
*Cyberfolk*: a population-wide feedback channel that would let ordinary citizens signal satisfaction or
distress in real time — an algedonic channel for a whole society, a pleasure/pain wire running from the
edge to wherever adjudication sat `[confirm]`.

*Cyberfolk* matters here as the boundary case of the argument rather than as a proposal to emulate. It is
the point where the variety argument confronts its hardest instance: a whole population's variety is far
too large for any center to hold, so the only viable design is one that keeps most adjudication at the edge
and lets the center hear only the signal it must. That an algedonic channel for a society was conceivable
in principle but never built is itself instructive — it marks how much of this argument remained a
direction rather than a demonstration, which is exactly why Croft treats the conclusion as a design
constraint to satisfy structurally rather than a settled result to cite.

## Why this is load-bearing for Croft

The conclusion the whole argument delivers is a single sentence: **centralization failure is structural,
not moral or a competence problem.** If that is true, then three familiar responses are ruled out. You
cannot fix it by replacing the people at the center, because the deficit is in the channel, not the
occupant. You cannot fix it by demanding better intentions, because intentions do not add variety. And you
cannot fix it by adding sensors, because sensing is not adjudication — a center drowning in more inputs it
cannot match is more overloaded, not less.

What remains is the structural answer, and it is the reason the design puts the invariant in the
architecture rather than in policy: absorb variety at the edge, keep adjudication where the context that
makes a problem legible actually lives, and escalate only the residue a center genuinely must resolve. The
spec's §3 is where that becomes mechanism. This document is the reason the mechanism is not optional: if the
failure it guards against is a law of regulation rather than a lapse of character, then any design that
concentrates adjudication is choosing the failure, whatever it calls itself.

## What this establishes (and does not)

Establishes the crispest form of the variety argument: that a *cybernetic failure* is a failure of
regulation in which a regulator holds less variety than it has accepted responsibility for, so control is
lost for structural reasons no leadership change can repair; that Glushkov's ten-billion-hand-computers
estimate makes the mismatch visible as an arithmetic impossibility rather than a political opinion; that
Beer, reading Trotsky, generalized bureaucracy-as-*variety-bottleneck* from a Soviet particular into a
structural law; and that the load-bearing consequence for Croft is to place the invariant in the
architecture, because a structural failure admits only a structural answer.

Does **not** restate the requisite-variety, algedonic-channel, or Cybersyn-vs-OGAS *mechanics* — those,
with Ashby's and Beer's primary quotations and the mapping to the spec's escalation and labeling postures,
live in `../drystone-spec/part-1-reasoning-underpinnings.md` §3. Does **not** make the downstream
peerhood-as-adjudication argument (a separate thread this one feeds). Does **not** certify its external
facts: the Beer quotation carries `[UNVERIFIED, confirm against primary edition before publish]`, and the
Glushkov figure, the Trotsky influence, and the *Cyberfolk* description carry `[confirm]` — all must be
checked against primary sources before external use.
