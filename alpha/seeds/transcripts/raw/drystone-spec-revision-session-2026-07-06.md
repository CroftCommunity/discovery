# Drystone spec revision session — 2026-07-06

**Status:** `preserved-condensed (cleaned-paste)`
**Fidelity note:** content-faithful cleaned-paste of a multi-turn design dialogue; no byte-pristine
export exists. UI chrome ("Searched the web", "Fetched:", "Edited a file", "Read a file", tool-call
dividers) stripped. Substantive content reproduced faithfully. Per PLAYBOOK §4.

**Session type:** Design dialogue + spec revision pass  
**Output artifacts:** Updated `discovery/beta/drystone-spec/part-1-reasoning-underpinnings.md`,
`part-2-certifiable-design.md`, `CHANGELOG.md`, `review-handoff.md`  
**Commit:** document-pass-1 (2026-07-06)

---

## Session summary

This session was a focused revision and grounding pass on the three Drystone documents (Part 1,
Part 2, changelog). The work fell into six moves:

1. Resource / capability / role correction + group-as-principal
2. Personhood reframed as contextual group judgment
3. Peer-vs-personhood keystone, device-group dial, dial-discipline principle
4. Equality model corrected to four properties (two equalities, two inequalities)
5. Control-plane / data-plane timestamp lesson + architecture-relativity
6. Two review passes + primary-source confirmation pass

---

## Dialogue (content-faithful cleaned-paste)

**User:** [Opens with the pasted transcript header — Drystone Part 1 status/beta doc, defines
P-Local-Truth, P-Knowable-Truth, P-Peer-Equality, P-Durable-Enablement]

**User hypothesis (opening):** Attaching a protocol I am developing, one time I am seeing is that
the distributed systems space has been seemingly focused on ironically solving for the best view of
canonical truth rather than solving for the reality of the supremacy of local truth reality. A node
can only ever know its own state really and all decisions are by nature made against local state,
even when in comparison.

And also that the functional intersection for the above seems shockingly underrepresented for
solving for augmenting actual human social structures and interactions esp without a presumed single
truth or central state at the heart. Beer is one exception.

How does my hypothesis hold up for scrutiny? Who are my aligned folks? Am I missing something that
changes that narrative? It seems surprising to me but so far consistent.

**Assistant:** [Confirmed the hypothesis holds — the field has split into two camps: (1) the data
layer (CRDTs, local-first, CALM theorem) that abandoned global coordination but stops at data; (2)
the governance layer (Matrix, blockchains) that kept a coordinator. The synthesis — carrying the
causal-and-cryptographic, no-coordination premise up into governance — is what Drystone occupies,
and it is genuinely underrepresented. Beer is correctly identified as an exception. The nearest
neighbor in *intent* is Modular Politics (Schneider et al.), which shares the ambition but roots
authority in a platform operator and brackets the resolution mechanics as future work. The honest
novelty claim is synthesis and terminus, not mechanism invention.]

---

**Key decision: architecture-relativity of the timestamp claim**

**User:** [After reviewing the draft text that stated "the path that decides governance outcomes
must not depend on a value an adversary can forge — and a timestamp is such a value — Matrix's..."]

At least in a system where all nodes are responsible for their own canonical view and likely in
none other but only our case is under design.

**Assistant:** Right — that qualification matters, and it's the difference between an overclaim and
a true claim. [Explained the two distinct reasons Matrix can tolerate what Drystone cannot:]

Your point: The claim is not a universal law. It follows from Drystone's specific architecture:
every node is responsible for its own canonical view, with no shared authority to appeal to. Matrix
has a recovery mechanism Drystone structurally lacks, so the Matrix tolerance is rational for their
architecture, not a mistake.

The sharpening (two reasons, not one):
- **Reason one — the homeserver tier.** Matrix has servers that hold authoritative room state;
  other servers plus room admins can act against misbehavior (de-op, ban, new epoch). Drystone has
  no such layer by construction.
- **Reason two — the after-the-fact override.** Even a bad timestamp resolution is reversible
  through continued authority action ("start a new epoch"). Drystone's remedy is exit (the fork),
  not override.

The full chain: no authority tier → cannot override in place → only remedy is heavyweight fork →
cannot afford routinely-corruptible ordering the way a system with cheap in-place override can →
authority-ordering path must exclude forgeable values. The constraint is architecture-relative: "a
system whose nodes each hold their own canonical view and whose only remedy is exit rather than
in-place override must" — which is Drystone's case and any design sharing those two properties,
not a universal.

**User:** Keep both points bc they are two sides of the same coin and yes that's what I meant.

**Assistant:** [Confirmed — folded into both §2.0.1 and §7.3.1, with the changelog entry. The
Matrix contrast is now explicitly a difference of architecture and remedy, not of correctness.
Matrix's timestamp tiebreak (power_level → origin_server_ts → event_id) confirmed verbatim against
Matrix State Resolution v2 and MSC1442; the authors' own admission quoted ("basically trusting
servers not to lie about the time"); the §7.3.1 marker for this specific claim cleared.]

---

**Session closeout: handoff document**

**User:** Ok our context is getting pretty cluttered, I want you to give me our framework and
conclusions laid out here that I can use to have another session review documents if needed for
clarity and consistency please.

**Assistant:** [Created `drystone-review-handoff.md` — a standalone reviewer brief covering: what
Drystone is in one paragraph; the razor and four principles; the equality model with the canonical
sentence and explicit terminology-trap list; the identity model and peer-vs-personhood keystone;
contextual personhood with the credential-service caveat; the control-plane/data-plane timestamp
lesson with architecture-relativity; group-as-principal; the dial-discipline principle; prior-art
grounding; and a precise confirmation-status ledger.]

**User:** Give me a summary of what we have done in this session with the latest copy of all
deliverables.

**Assistant:** [Summary of the six moves (see session summary above). Four deliverables: Part 1
(~780 lines), Part 2 (~1,975 lines), changelog (~490 lines), review handoff (new, 217 lines).]

**User:** Those spec files should update our local spec part 1 and 2. And let's commit after each
update document pass of which this is one to get a clean history, but also you may find contents
that need to be folded into other layers to separate out and we file all transcripts in raw form as
well for reference.

---

## Key decisions / artifacts from this session

| Decision | Where it landed |
|---|---|
| Architecture-relativity of timestamp constraint (two sides of one coin) | Part 1 §2.0.1, Part 2 §7.3.1 |
| Four-property equality model: two equalities (rights, weight) + two inequalities (resources, roles) | Part 1 §2.3, Part 2 §5.0–§5.3, §5.6 |
| Capability demoted: data-access mechanism under roles, not a peer property | Part 2 §5.0, §5.5 |
| Peer-vs-personhood keystone: peer = provenance object, personhood = social judgment | Part 2 §5.2 |
| Three crypto-trust examples (TLS CA, PGP, SSH TOFU) grounding social-trust bottom-out | Part 2 §5.6 |
| Device-group dial correction: even own device pool is a dial, not fixed high-trust | Part 1 §2.3 |
| Dial-discipline principle: 80/20 defaults, 20% must stay representable | Part 1 §2.3 (new) |
| Matrix timestamp tiebreak confirmed verbatim (MSC1442); §7.3.1 marker cleared | Part 2 §7.3.1, CHANGELOG |
| Review handoff doc created for reviewer brief across fresh sessions | drystone-spec/review-handoff.md |
| Layer cake restructure proposed for beta/: drystone-spec/ impl/ croft/ governance/ history/ socialization/ | User decision — to be executed across sessions |
| Themes 04, 05, 06 confirmed as impl/ layer | User decision |

## Confirmation status (as of this session)

**Cleared this session:**
- Matrix State Resolution v2 timestamp tiebreak (power_level → origin_server_ts → event_id; MSC1442)
- RFC 9420 / RFC 9750 (MLS)
- Meadowcap/Willow capability definitions (willowprotocol.org)
- Spritely / ActivityPub (spritely.institute)
- PGP web-of-trust model (GnuPG docs)
- SSH TOFU trust model
- Mill, Hayek, Ashby, Popper — verified in prior sessions, stable

**Still [confirm before publish]:**
- Matrix MSC/CVE specifics (CVE-2025-49090, CVE-2025-54315, MSC4289, MSC4291, MSC4297)
- CALM theorem statement (Hellerstein & Alvaro primary paper)
- CAP theorem statement (Gilbert & Lynch primary)
- CRDTs (Shapiro et al. primary)
- Lamport "Time, Clocks" verbatim
- Beer: "specify only somewhat" verbatim (Brain of the Firm); Cybersyn/OGAS capacity figures
- Ostrom: principles 6/7 verbatim (1990); subsidiarity verbatim (Wilson/Ostrom/Cox 2013)
- iroh relay-blindness and public-key-identity claims (iroh 1.0 primary docs)
- TLS/X.509 RFC 5280
- decentralized-MLS / FREEK / draft-xue claims
