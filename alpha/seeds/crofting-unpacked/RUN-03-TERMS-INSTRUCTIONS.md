# RUN-03: crofting_site — /terms/ and /terms/ens/

Target repo: `CroftCommunity/crofting_site` (main). REQUIRES RUN-02 merged:
STOP if `checks/check_site.py` or `arecipe/index.html` is absent, report, and
halt (the runs are ordered; this one extends the harness RUN-02 created).

Execute on a fresh branch `run-03-terms` off `main`. Write `RUN-03-SUMMARY.md`.
Do not merge; leave for review.

Guardrails: identical to RUN-02 (plain HTML + the existing styles.css, no JS,
no new dependencies, zero loaded external requests, copy below is FINAL and
placed verbatim, TDD red-first).

## Phase 1 (RED): extend checks/check_site.py

Add assertions, run, and record the red before building anything:

- `terms/index.html` and `terms/ens/index.html` exist.
- The ENS page contains, in order: THE SIGNPOST, THE SURFACE, THE SOIL,
  THE BEDROCK; and the strings "A polite acronym for how platforms rot." and
  "ENS is Not Service".
- The terms index contains the entry line linking to `/terms/ens/`.
- `library.html` links to `/terms/`.
- Extend the external-URL allowlist with `pluralistic.net`.

## Phase 2: the pages

Same header, footer, favicon, tier-label styling, and `.course` dividers as
the rest of the site; absolute-root asset paths (`/styles.css`).

### terms/index.html — copy verbatim

Title: `Terms — croft.ing`

Heading: **Terms**

> Croft's pages use a small working vocabulary. Each term gets a page: a
> one-liner, a citable definition, and the full reasoning. Link whichever
> depth the conversation needs.

Entry list (one entry for now, same markup shape as the growing section:
bold term, description, no dashes):

**ENS** — links to `/terms/ens/`. Description: A polite acronym for how
platforms rot.

### terms/ens/index.html — copy verbatim

Title: `ENS — croft.ing`

Kicker, small caps, `--granite`: A CROFT TERM

# ENS

THE SIGNPOST

A polite acronym for how platforms rot.

THE SURFACE

ENS, sometimes pronounced "ins," is Croft's shorthand for enshittification,
Cory Doctorow's name for the arc by which platforms decay from serving their
users to consuming them. Deliberately, ENS has no single expansion: in the
old free-software tradition of self-referential acronyms, it reads
differently depending on where you stand, and every reading describes the
same decline.

THE SOIL

**The word problem.** Doctorow named the pattern in 2023, and the word stuck
because everyone recognized the thing it named: a platform is generous to its
users until they are locked in, then squeezes them for its business
customers, then squeezes the business customers too, keeping the extracted
value for itself. The concept is indispensable. The word, though, is a
room-clearer in exactly the rooms where the pattern gets decided: board
meetings, procurement reviews, syllabi, budget lines. ENS is a passport. It
carries the full concept across that border with its analytical content
intact and its shock value checked at the door.

**The tradition.** ENS takes its shape from free software's habit of acronyms
that refuse to sit still: GNU's Not Unix, WINE Is Not an Emulator, YAML Ain't
Markup Language. The habit looks like a joke and functions as a philosophy:
the name defines itself by what it refuses to be. So the root definition of
ENS is recursive.

**ENS is Not Service.** Software as a Service makes a claim in its own name:
that what you are buying is service. A platform that has crossed into ENS has
quietly forfeited that claim. The subscription still bills, the uptime page
is still green, but the system's real product is no longer the service
rendered to you; it is the value extracted from you. ENS is Not Service is
the recursive root, and the three expansions below are the same forfeiture
seen from three chairs.

**From the boardroom: Extractive Neglect Syndrome.** This is the cause. At
some point a platform's operators shift strategy from value creation, making
the product better so people choose it, to value extraction, monetizing the
people who can no longer easily leave. Extraction gets the engineers, the
roadmap, and the OKRs; the core product gets neglect, and the neglect is not
an accident but a reallocation. If you sit where budgets are set, ENS is the
name for that reallocation.

**From the machine room: Eventual Non-utility State.** This is the mechanic,
and it is a deliberate play on eventual consistency, the distributed-systems
promise that replicas, left to themselves, converge on agreement. Offered in
the same spirit as the industry's other satirical laws: a centralized,
investor-fed platform, left to itself, converges too, on a state of zero
utility for the person using it, as each quarter's re-engineering serves the
payer a little more and the user a little less. Engineers watch this
convergence from the inside, one migration at a time. If you sit at the
architecture diagram, ENS is the name for the attractor.

**From the interface: Erosive Nudge Syndrome.** This is the symptom. Design
research gave us the nudge, a gentle default that helps a person do what they
already meant to do. ENS inverts it: the erosive nudge wears the user down
instead. Consent walls rebuilt weekly, sponsored rows dressed as results, the
cancel flow that takes six screens while the upgrade takes one, the setting
that resets itself. No single cut is worth complaining about, which is the
design. If you sit in front of the product, or in front of users, ENS is the
name for death by a thousand nudges.

**One arc, three chairs.** The three expansions are not competing
definitions; they are the same failure observed from the boardroom, the
machine room, and the chair in front of the screen. That is the diagnostic
use of the term: notice which chair the person across from you is sitting in,
and offer the expansion that describes their view. The executive hears a
strategy critique, the engineer hears a systems law, the designer hears a
pattern audit, and all three are now discussing the same disease, in a word
that can be said in any of their meetings.

**Why a vocabulary at all.** The point of the polite acronym is not
politeness; it is admissibility. A pattern that cannot be named in the rooms
where it is decided cannot be refused in those rooms either. And naming is
only half of Croft's interest. The other half is construction: the pattern
has structural preconditions, and software can be built without them.
arecipe is the working experiment, and its treatise is that one app's account
of its own ENS-resistance, walked precondition by precondition.

[Link "its treatise" → https://arecipe.croft.ing ]

THE BEDROCK

- The origin: Doctorow's essay that named the pattern →
  https://pluralistic.net/2023/01/21/potemkin-ai/
- One experiment in ENS-resistance: the arecipe treatise →
  https://arecipe.croft.ing
- The ground this grows on: https://croft.ing

## Phase 3: discovery

`library.html` gains one line in its link list, before the discovery-repo
links: The working vocabulary → `/terms/`. Nothing else on any existing page
changes.

## Phase 4 (GREEN) and summary

`python3 checks/check_site.py` to green. RUN-03-SUMMARY.md records red and
green outputs, files created/edited, and the allowlist addition.

## Appendix: one-line amendment to the arecipe_treatise repo (separate session)

In the treatise's Act I, the parenthetical "(ENS from here on, and across
Croft's pages)" gains a link: the text "ENS" links to
https://croft.ing/terms/ens/ . Fold this into TREATISE-RUN-01 if that run has
not executed yet; otherwise it is a one-line follow-up commit there. The
treatise's own framing does not change: it is that one app's account of its
own ENS-resistance, and the terms page is where the acronym itself lives.
