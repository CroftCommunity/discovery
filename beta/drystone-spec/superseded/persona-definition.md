# Persona

status: normative definition. Reconciled to the final model of this session
(persona is the human layer's manifestation; the persona-to-human binding is
a group judgment the protocol cannot make; multiplicity is across-systems,
with one-per-human the within-group norm). Supersedes the earlier draft of
this file. For the full reasoning see `drystone-persona-session-summary.md`.

term: persona

plural: personae (Latin form, used throughout; the English "personas" is not used)

relation to "peer": peer is a relationship descriptor, not a noun for an entity (see Note 1)

---

## Definition

A **persona** is the human layer's manifestation in the system: a principal
by virtue of having a key pair, present through the lineage that descends
from that key pair and the verification of it. It carries the rights floor
and one flat unit of governance weight, and it is the locus at which the
social-utility calls the system cannot compute deterministically are
adjudicated, because a human stands behind it. Personae stand in **peer
relation** to one another; "peer" names that relation, not the entity.

A persona is represented, at any given moment, by exactly one root key pair.
The persona persists across rotation of that key pair: a sitting root key
pair MAY sign a successor, after which the successor becomes the persona's
current bearer and the prior key pair MAY be retired. The persona is the
entity; the root key pair is its current bearer; the chain of signed
successions is its lineage.

A human MAY hold any number of personae **across discrete systems** (one per
group is the intended shape), and no such persona is primary or derivative
with respect to another. **Within a single group** the intent is one persona
per human. The protocol cannot enforce that within-group binding (see Note 3);
where it is violated, the consequence is degraded governance, not a protocol
fault.

---

## Requirements

R1. A persona MUST be represented by exactly one root key pair at a time.

R2. Key rotation MUST preserve persona identity. A successor key pair takes
effect only when signed by the sitting root key pair, forming a verifiable
lineage.

R3. The protocol MUST NOT assign primary or derivative status between two
personae held by the same human across systems. Such personae are mutually
independent.

R4. The protocol MUST NOT (and structurally CANNOT) certify the binding
between a persona and the human who holds it: there is no fact for it to
deliver (the binding has no technical representation) and no authority tier
above the group from which to impose it (see Note 3). This does not forbid a
**group** from gating its own entry or recognizing personae to its own
standard; that is the group's recognition act, not the protocol enforcing a
binding.

R5. Weight MUST be flat: one unit per recognized persona, non-inflatable by
clients, devices, resources, or roles. The strength of binding a group
requires before recognizing a persona as a distinct human MAY vary,
proportional to the group's function and goals, but this proportionality is
on **recognition**, never on weight: a stronger binding requirement, never a
heavier vote.

R6. Peer relation between personae of a system MUST be symmetric: to be a
persona within a system is to hold equal standing with the other personae of
that system (see Note 1).

---

## Note 1: persona is the entity, peer is the relationship

"Peer" describes a symmetric relationship at the edge between two or more
things. It is used throughout this stack: personae are peers, clients are
peers, devices are peers. Each of those is a distinct kind of thing that
stands in peer relation to others of its kind.

Earlier drafts used "peer" as a noun for the entity itself (the human's
manifestation in a system). That overloads the word: it makes "peer" name
both the thing and the relation the thing enters into, which loses the
fidelity needed to talk about the parts separately. This document fixes the
entity sense as **persona** and reserves **peer** for the relation. The
adjudication-locus genus is the **principal** (of which persona is the human
kind); a single sync-protocol actor is a **participant**.

---

## Note 2: on the word

"Persona" is the Latin for the mask worn by an actor, the thing through which
(per + sonare, "to sound through") the actor's voice was carried to the
audience. The mask is not a disguise over a truer self; it is the channel
through which meaning is sounded into the scene.

Two properties of the concept follow the root rather than being bolted on:

A persona is a genuine bearer of standing, not a fiction. The connotation of
"persona" as a performed or inauthentic self is set aside here in favour of
the older sense: the means by which a participant genuinely speaks within a
system.

Personae arrive as a cast. A persona does not stand alone; the dramatis
personae are the set of voices that play against one another in the same
scene. Symmetric standing among personae (R6) is therefore latent in the
term, not an external stipulation.

A persona's name *is* one of its rights. The root is per + sonare, "to sound
through": a persona is the thing a human's **voice** sounds through. And
**voice** is one of the three fundamental rights (voice, tenure, exit; §5.3).
So the etymology and the rights floor are not two facts that happen to share
a word, the word's root names the very right it carries. The thing that
manifests a human is, at the root, the thing through which that human's voice
sounds; the protocol's floor guarantees exactly that voice. This is
congruence, not decoration: the name and the mechanism are the same claim in
two registers, the dramaturgical and the cryptographic. (It is also why
*voice* survives as the load-bearing right where a louder candidate like
*share* did not, §5.3: voice is what the manifestation is *for*.)

The seam: the root carries the "voice sounding through" sense well, and for
the *voice* right that congruence is exact, but the root does not by itself
imply key rotation, lineage, the across-systems multiplicity, or the
flat-weight guarantee. Those are properties this specification assigns (R1
through R6); they are not claims about the etymology.

---

## Note 3: the binding is a group judgment, and "recognize" means "treat as"

Whether a persona corresponds to one distinct human is a social-utility
judgment, never a protocol fact. The protocol **could not** make it from
either direction: there is no fact to deliver (the binding has no technical
representation), and there is no authority tier above the group from which to
impose or police it (attempting to would relocate adjudication and reintroduce
a center). Both are the same limit seen from the delivery side and the
enforcement side.

So the judgment does not get handed to the group; it **necessarily falls** to
the group, because that is the only place it can live. To **recognize** a
persona as a distinct human is to decide to **treat** a lineage as a distinct
person for the group's own purposes, never to **verify** it. The binding does
not become a fact even after recognition; it remains a standing judgment the
group can revise, and the revision path is withdraw-recognition-and-fork.

---

## Note 4: the full term lattice (the conformance oracle)

This section walks out every load-bearing noun and the exact relation each
holds, so the spec can be validated against it and misses reasoned about
reliably. A "miss" is any spec usage where a term appears in a relation this
table does not sanction, or where two terms are conflated.

### 4a. The entities and their genus

`principal` is the genus: a role-holding entity identified by one
key-lineage. Its kinds are exactly three:

- `persona` is-a principal. The kind that manifests a human. Carries the
  rights floor and one unit of weight. Adjudicates non-computable social
  utility.

- `group` is-a principal. A collective that can hold a role as a single
  principal. (Its key-establishment identity model is an open seam.)

- `delegate` is-a principal, as a **state** not a species: a persona or
  group currently holding a role delegated by another principal.

Not principals:

- `meer` is NOT a principal, NOT a member, NOT a persona. A blind
  store-and-forward node (infrastructure). Holds no role, no rights, no
  weight. Named by scope configuration, not enrolled as an identity.

- `relay` (iroh) is NOT a principal. A transport-layer blind packet
  forwarder for reachability. Distinct from a meer (different layer; meer is
  application-layer storage, relay is transport-layer forwarding). Holds
  nothing.

### 4b. The hosting / realization chain

```
human ── (manifests as, 1:N across systems; 1 per group is the norm) ──> persona
persona ── (rooted in, exactly 1 at a time) ──> root key pair
root key pair ── (descends to, by signed credential) ──> membership keys
persona ── (realized by, 1..N) ──> clients
clients ── (run on / hosted by) ──> devices
device ── (is-a) ──> node          [hardware]
device ── (hosts, 1..N) ──> clients
human ── (has, 1..N) ──> devices
```

Hosting chain in one line: human → devices → clients. A principal is
realized by one or more clients across one or more devices.

### 4c. The MLS-carried terms (kept verbatim from RFC 9420/9750)

- `client` = software on a device that is a member of a group: one MLS
  **leaf**, one **signature key**, one **credential**, authenticated as a
  **member** via the **AS** (Authentication Service).

- `member` = MLS's term; the client as enrolled in a group. The group
  recognizes its members (clients); **lineage** then resolves a group's
  member-clients to one persona.

- `leaf`, `signature key`, `credential`, `AS`, `DS` (Delivery Service) keep
  their RFC meanings.

Relation: `member` is the client-in-group; `persona` is what members fold
to by lineage. Counting members ≠ counting personae; the fold is the bridge.

### 4d. Lineage, and the count

- `lineage` is a **provenance object**: the cryptographic chain of signatures
  from a root key pair down to each membership key. Technically representable;
  the protocol verifies and counts it with certainty.

- the **fold** resolves all clients/devices sharing a lineage to **one
  persona per rooting key pair**. Governance counts personae (by lineage),
  never clients or devices.

Relation: `persona` is the human that a `lineage` is taken to manifest. The
binding "this lineage is one human" has NO technical representation; it is the
group's judgment (Note 3).

### 4e. The four equality-properties, and the non-property

Asked of personae: *in what ways may one persona differ from another?* Four
properties, two equal, two unequal, plus one that is not on the list.

- `right` (EQUAL). What a principal inherently holds: the floor of voice,
  tenure, exit/fork. Identical for every persona, unremovable, never
  delegated. Attaches to the **principal**, flows to its clients.

- `weight` (EQUAL, by necessity). How much a persona counts in governance.
  Flat, one per distinct persona (by lineage), non-inflatable. Attaches to
  the **persona**. The governance image of the equal right.

- `resource` (UNEQUAL, legitimately). What a node has (storage, uptime,
  reachability, radio). A property of the **device/node**, not of identity.
  Descriptive, expected unequal, not delegable. A fact about every node
  (including meers/relays), listed among persona-inequalities because across
  personae it is one of two legitimate differences.

- `role` (UNEQUAL, legitimately). In-group governance authority granted to a
  **principal** by member consent. Scoped, attenuating, revocable. Rides
  entirely above the two equalities; granting/revoking never touches the
  rights floor or weight.

- `capability` (NOT a fifth property). A Meadowcap data-access grant
  (read/write an area of a namespace), unforgeable, issued by the data's
  owner, attenuating. **Issued under a role**; lives in the data plane, one
  level below the equality question. Sits UNDER roles, not beside resources.

### 4f. The bundle

- `PrincipalSet` = a named, pinned, group-recognized **bundle** of roles and
  the capabilities they imply, bindable to **any principal** (persona, group,
  or delegate). Every PrincipalSet MUST be definable as
  `floor + [explicit role set] + [implied capabilities] + [expected resources]`.
  (Named PrincipalSet because its members are principals, not only personae.)

### 4g. The relation, and the two reassigned senses

- `peer` = the **relation**: symmetric standing between principals; each a
  locus the others must respect, none a center the others must obey.
  "Personae are peers." Also retained for transport wiring (peer-to-peer) and
  for the consensus sense ("every honest peer agrees") where it trades on the
  relating.

- `participant` = a single sync-protocol **actor** performing a mechanical
  step (accepting a write, folding, emitting a beacon). Used where no relating
  is in play and the entity is not specifically the human-manifestation.

- `node` = the hardware/box in the network; `device` is-a node. Used at the
  transport/resource layer.

### 4h. The relations that must hold (invariants to check the spec against)

I1. Only a `principal` holds a `role`, a `right`, or `weight`. A `meer`,
`relay`, `node`, `device`, or `client` holds none of these.

I2. `weight` attaches to `persona`; `right` attaches to `principal` (flows to
clients); `resource` attaches to `node`/`device`; `role` is granted to
`principal`.

I3. The fold counts `persona` per rooting key pair, never `client` or
`device`. Any threshold or quorum counts personae (by lineage).

I4. `capability` is issued under a `role` and lives in the data plane; it is
never one of the four equality-properties.

I5. The locus-of-adjudication is the `principal` (genus); `persona` is its
human kind. "What can adjudicate" should read principal (or persona where
specifically the human kind is meant), never the bare retired entity-"peer."

I6. The persona-to-human binding is a `group` judgment, never a protocol
fact; "recognize" = decide-to-treat-as, never verify.

### 4i. Sanctioned exceptions (where "peer" stays despite looking like the entity)

These were checked against the oracle and deliberately kept; a future reader
should not flag them as misses.

- §3.1 archetype, "thousands of nodes, zero peers in the sense that matters."
  Reads as the **relation**, not the entity: a pure sensor mesh has zero
  things standing in peer relation, because you cannot hold symmetric peer
  standing without adjudicating. The line names the absence of the relation,
  not the locus, so I5 is not violated. Kept for the rhetoric that plays off
  the section heading "system of peers."

- The consensus sense ("every honest peer agrees," "deterministic on every
  peer," "which a peer can defend"). Kept: trades on how peers relate
  (including across layers), per the session decision. Not the human entity.

- "peer protocol," "peer-to-peer," "peer-governed," "of peers,"
  "P-Peer-Equality." Relation, transport, or principle-name. Kept.
