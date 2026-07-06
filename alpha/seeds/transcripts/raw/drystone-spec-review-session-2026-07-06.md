# Drystone spec review session — 2026-07-06 (document-pass-2)

**Status:** `preserved-condensed (cleaned-paste)`
**Fidelity note:** content-faithful cleaned-paste of a multi-turn design dialogue; no byte-pristine
export exists. UI chrome (tool-call dividers, "Searched the web", "Viewed a file", "Edited a file",
"Ran a command") stripped. Substantive design reasoning, rulings, and verification steps preserved
faithfully. Per PLAYBOOK §4.

**Session type:** Structured review + model corrections + em-dash pass
**Output artifacts:** Updated `discovery/beta/drystone-spec/part-1-reasoning-underpinnings.md`,
`part-2-certifiable-design.md`, `CHANGELOG.md` (prepended), `open-items.md` (new companion)
**Commit:** document-pass-2 (2026-07-06)

---

## Session summary

This session was a structured consistency review of Parts 1 and 2 against the review-handoff.md
brief. The work fell into three phases:

1. **Initial review pass** — five rounds of model corrections (A1–A5 mechanics corrections,
   B1–B8 open-items review)
2. **Author rulings** — user supplied decisions and positions against the open items in a voice
   transcript, converted to written rulings and applied
3. **Em-dash removal** — role-aware pass eliminating all 562 em-dashes from both parts

---

## Phase 1: model corrections

The reviewer identified inconsistencies across the five areas:

**A1 (logical clock).** Per-client is correct. A device can host multiple clients in multiple
scopes (Linux namespace/cgroup analogy: one device, multiple time representations). Confirmed.

**A2 (peer/member/lineage).** "Group-recognized peer" was wrong terminology. A group recognizes
*members* (clients); lineage resolves member-clients to one peer. This is a meaningful model
correction, not just wording.

**A3 (meer).** Retained colloquialism for a blind store-and-forward node. The earlier model (full
group member with no local history) abandoned; language cleaned to match.

**A4/A5 (meer/DS/store-forward).** Verified against RFC 9750: a central Delivery Service is not
required. Clients can communicate directly peer-to-peer. Because MLS is asynchronous (no two
clients need be online at once), a message for an offline recipient must be held somewhere — the
meer is the optional store-and-forward augmentation. Distinct from the iroh relay (transport-layer
NAT forwarder). Three-way distinction applied: direct p2p / optional meer / iroh relay.

---

## Phase 2: author rulings (voice transcript, paraphrased)

**On device/client/peer model (A2):**
"A client is a piece of software on a device that is a mechanism a member uses to communicate with
the group. A device is a node with storage and RAM and resources. A device could run n number of
clients — in a Linux namespace/cgroup, one device with multiple representations of time with
clients in each. A group recognizes a member, and a member is a client. Multi-client is how most
people think of multi-device. The group has a member, and that member has a lineage, and that
cryptographic lineage says one peer across n devices could have n clients, but in governance every
peer is still one vote."

**On meer (A3/A4):**
"Meer is a retained colloquialism for a store-and-forward node, just for clarity. This was
originally proposed as a different mechanism where it was literally a full member of the group and
just didn't have local history of its own. We've moved away from that. What we're cleaning up is:
a meer is a store-and-forward node for the MLS messaging substrate, but not a required one —
our protocol peers can directly talk to other peers. [Need to] verify that's correct. The meer
is [the MLS] mechanism, which allows devices to persist a message even when they're offline for
another device. It's not necessary — we can have two clients directly connect and communicate
via MLS. We need to definitely verify that's correct. [Distinction between] the meer (store-and-
forward) vs the iroh relay."

**On rights set (B1):**
"For the full four rights, I'm just not sure. I don't know what 'tenure' even is supposed to
mean there — I guess tenure meaning existence in the group already, voice meaning communication.
Yeah. I think it's... I don't know what share would mean there. Is it tenure, exit, and voice — I
guess those are three? I'm not sure how share got in there, to be honest. And then can the
survivor re-key path strand a peer's tenure, leaving it formally a member but unable to re-
establish its standing after re-key? If yes, tenure is not yet a clean right. I'm not sure, to be
honest. We probably need to just clarify what we need to walk out there so I can run the test in
a technical sense."

**On group-as-principal/communal namespace (B2):**
"My conundrum was: when forking and merging is cheap but you have an asset in a group that's being
collaborated on, in order to honor the fork you have to say the asset is owned by the clients and
thus the peers as well as the group, so that when the fork happens it's like an open-source fork.
And as I was thinking through that problem, we were refining the definition of capability, and we
came down to a communal namespace fitting that model of group asset that can survive the fork and
merge. So right now, okay, that's what group-as-principal would be implemented with — that
Meadowcap communal namespace concept — but I haven't worked out the rotation scheme. It could be
that the communal namespace is secondary and only established at a fork or a merge. I'm not totally
sure. What we need to do is dig into Meadowcap and align or see if it does align with MLS to see
if assets associated with groups fork and merge sanely."

**On escalation tolerance default (B5):**
"The spec deliberately declines to pick the value — I think you mean the default value. And
really, that's a matter of we know this is a thing and it needs to be looked at in actual
implementation to determine the granularity of the knobs and also sane defaults."

**On capability mechanism / Track A vs B (B6):**
"I was kind of working through nomenclature, ended up preserving capability because we've
foreshadowed that Willow and likely Meadowcap are the path forward and so setting out not to
collide is the stronger choice. It's not impossible — Keyhive is a better option. This is an open
design question. So the deferral is still correct, and if we can define what our needs are to work
out the fit and how to integrate, that would be useful."

**On hash function (B7):**
"I'm not sure. I think we ran the testing with SHA-256 but all of it should work fine with any
reliable hashing function. I'm not too worried about that. It's really just a note on follow-up
for proofs to go along with the spec."

**On grounds of authority (B8):**
"The idea is the rights floor is variety-enabling, which is system-sustaining, and that's why
rights-negation is the move toward system collapse, and that's what makes a peer's authority
necessary. And then what grounds it — you need some mechanism to mint and bind the human identity
with a cryptographic fixed peer identity. And if we're saying what grounds that, really it's
contextual. That should be spelled out: a family group is simpler and higher trust to bind versus
larger groups of disconnected people. And then the enabling wire encoding set — we know we need
to specify this at this level, it's just not there yet."

**On em-dashes:**
"I don't care for the em-dash, so getting rid of that would be great."

---

## Phase 3: em-dash removal

The reviewer wrote role-aware scripts (dedash.py, dewrap.py) to classify and replace all em-dashes:
- Bullet labels and headings → colons
- Appositives → commas
- Independent-clause joins → semicolons
- Parentheticals → commas or parentheses
- Attribution lines → clean (dash dropped)

562 em-dashes removed (219 in Part 1, 343 in Part 2). Several artifacts caught and fixed manually:
bibliography "., Grounds:" sequences; stray leading-comma in wrapped line; reference-block
attribution lines.

---

## Key decisions / positions from this session

| Item | Decision / position |
|---|---|
| Logical clock | Per-client (not per-device); confirmed |
| Peer/member recognition | Group recognizes members (clients); lineage resolves to peer; "group-recognized peer" dropped |
| Meer | Optional blind store-and-forward node (MLS layer); distinct from iroh relay (transport layer) |
| DS (Delivery Service) | Optional; verified RFC 9750; clients can communicate directly peer-to-peer |
| Rights floor | Three: tenure, voice, exit. `share` dropped. |
| Tenure-under-rekey | Open check; test shape written into Appendix B; user to confirm test shape |
| Group-principal / communal namespace | Motivated by fork/merge asset ownership; Meadowcap/MLS alignment the decisive next step; rotation unworked |
| Escalation tolerance | Default value left to implementation (knob granularity + sane defaults = deployment tuning) |
| Capability mechanism | Deferral correct; Keyhive preferred on revocation immediacy; needs-definition precedes selection |
| Hash function | Not a design worry; SHA-256 tested, any reliable hash works; follow-up for proofs |
| Grounds of authority | Rights floor = variety-enabling = system-sustaining; rights-negation = self-amplifying collapse; mint-and-bind is contextual |
| ENABLING encodings | Known, needs filling after nomenclature settles; gates publication-final |
| Em-dashes | Removed everywhere, including headings (heading title-separator → colon) |

## Confirmation status changes (this session)

**Cleared:** RFC 9750 §6.4 (DS optionality); MLS asynchronous delivery model.

**Still [confirm before publish] (unchanged):** Beer verbatim + Cybersyn/OGAS; Ostrom 1990 + 2013;
decentralized-MLS/FREEK/draft-xue; TLS/X.509 RFC 5280; SSH; Keyhive; Lamport; CALM; CAP; CRDTs.
Matrix CVE/MSC specifics (CVE-2025-49090, MSC4289, MSC4291, MSC4297) and the capped-root soundness
remain the priority security open item (B4).
