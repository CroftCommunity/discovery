# discovery / beta / fenced: group-chat failure modes (the descriptive map)

date: 2026-07-07

register: descriptive field knowledge. This doc maps *how existing group-chat systems fail* at
scale: the recurring failure modes the field has named and paid for, and the operational lessons a
production E2EE deployment (Matrix) has already absorbed. It is the field a large-group design
measures itself against. It draws that map; it does not argue a design. Drystone's responses to these
modes live in the spec (`drystone-spec/` Part 2 §7, synchronization and governance-conflict
resolution; §11, large-group scaling, dormancy, and re-entry) and in the implementation, and are
cross-referenced here, not reproduced.

## Overview

Group chat breaks in a small number of structurally hard places, and the same places recur across
every serious system (Signal, WhatsApp, Matrix, Briar, Secure Scuttlebutt, Session, Keybase, Delta
Chat). The failures are not implementation bugs; they are consequences of the primitives. Two bodies
of field knowledge sit here. The first is a taxonomy of the recurring failure modes and why each is
structurally hard, drawn from a sourced adversarial research pass (2026-06-14) that separated protocol
facts from product facts and flagged partisan sources. The second is the operational-lessons record
from Matrix, the one federated system that shipped decentralized encrypted group chat in production
and paid for the hard lessons at consumer scale.

Boundary calls:

- This doc carries the *lessons* (what breaks), not the *systems*. Matrix as a building block, and its
  place in the open field, are catalogued in `cairn/adjacent-systems.md`; this doc reads only the
  operational scars off it.

- The E2EE-versus-scale *tradeoff* and the capability / ceiling map live in the sibling
  `group-scale-versus-e2ee.md`; the per-group operational rates and platform economics live in
  `operational-rates-and-platform-economics.md`. This doc does not restate either; it cross-references
  them.

- Drystone's *answers* to every mode below live in the spec and impl. Where a mode is exactly the thing
  a large-group design must answer, the pointer is given inline.

## The failure-mode taxonomy

### 1. The person-versus-keypair gap (multi-device)

The root difficulty is that "a person" is a social abstraction and "a keypair" is the cryptographic
primitive, and the two never line up cleanly; every system pays for the gap somewhere. Append-only
identity logs fork if two devices write to one log (a forked append-only log is a protocol violation,
not a merge), so a single feed cannot span devices. The field's four approaches, each with its cost:

- Shared key across devices: simple, but divergence and the inability to revoke one device without
  rekeying everything.

- Primary plus server-linked: clean UX, but the server becomes load-bearing for identity (Signal holds
  the linked-device list and brokers key distribution).

- Per-device-as-separate-member: revocation becomes natural, but the member list carries every device
  and something must fold them back into one displayed person (Keybase shipped this shape years ago,
  with a per-user key layered over per-device sigchain keys).

- Transfer-not-sync: sidesteps live multi-device by making it a one-time handoff, then letting the
  copies diverge (Delta Chat's second-device model, Session's seed sharing).

Briar refuses the problem outright; its developers describe multi-device as unsolved because an account
lives on one device with no transfer mechanism. Secure Scuttlebutt gave each device its own feed and
tried to staple them with Fusion Identity, which was specified but stalled and (per its audit) required
out-of-band human voting to decide which device to trust when a fusion identity went bad.

### 2. Device loss and identity recovery

This is the hardest usability-versus-security collision in the field, and every point on the spectrum
either weakens the threat model or reintroduces a trusted party; there is no known third option.

- "You are just gone": Briar and Secure Scuttlebutt. Lose the device, lose the identity. Maximally
  safe, minimally usable.

- Mnemonic seed: Session. The recovery phrase *is* the private key, a single stealable secret, and
  sharing it is how Session does multi-device. Sharp edge: Session V1 encrypts everything under one
  long-term key that does not rotate, so it has no forward secrecy at all; V2 (in development as of late
  2025) is re-adding it.

- Server-backed PIN: Signal. Recoverable, but a server holds the recovery-enabling material.

- Key backup: Matrix. Recoverable, but the backup is a new asset to protect and a new attack surface.

The structural reason is unforgiving: recovery means reconstructing secret-holding authority after the
secret holder is gone, so either another party held a copy (trusted party reintroduced) or a
human-memorable secret stood in for the key (weaker, stealable). Decentralization removes the party
every usable recovery story leans on.

### 3. Membership-change and concurrent group-key agreement

Add or remove triggers a rekey, and the hard part is concurrency. In MLS, membership change is a state
transition on a single linear epoch chain; two valid commits for the same epoch are a fork, and the
protocol has no merge operation. The main designs and what each gives up:

- Sender keys (Signal, WhatsApp): cheap to send, but removal security leans on the server coordinating
  who is current.

- Megolm (Matrix): a per-sender ratchet shared with the group; membership change means new sessions. It
  provides block-level forward secrecy and no post-compromise security by itself. (The "Megolm has no
  forward secrecy" line is competitor marketing from Wire; the accurate version is forward secrecy in
  blocks, no post-compromise security.)

- MLS / TreeKEM (RFC 9420): proper forward secrecy and post-compromise security, scaling
  logarithmically, but it assumes a delivery service that orders commits.

Keybase is the most instructive case: to prove an ordering across two sigchains ("key was used after it
was provisioned and before it was revoked") they reached not for pure cryptography but for an
append-only structure plus a server committing to one true view plus server-side access control, that
is, a trusted ordering party.

### 4. The ordering / consensus assumption (survivor-determinism and covert-ordering)

This is the center of the field, and it is the consensus, published view rather than any one project's
opinion (high confidence). Decentralized group membership is fundamentally a consensus problem. RFC 9420
requires commits to be sequenced; RFC 9750 (the architecture document) says the delivery service can be
peer-to-peer, but then the clients themselves must implement the delivery guarantees the service would
have provided. The requirement does not disappear when the server is removed; it moves onto the peers,
who now must agree, which is consensus. The IETF Decentralized MLS draft is blunt: in decentralized
settings the only way to run a functional delivery service is to let members retain key material so they
can process commits out of order, which violates MLS's deletion schedule and significantly reduces
forward secrecy; FREEK-based work recovers most of that loss, but even with those fixes, once a fork has
been created there is no single semantic for returning members to one agreed group state.

Two named modes fall out of this:

- Survivor-determinism: the "return a forked group to one agreed state" step is explicitly flagged as
  unsolved by the people who study it. A design that promises deterministic survivor selection is
  claiming that independently-partitioned peers can compute the same surviving state from the shared
  logs alone; whether that selection is a pure function of the logs or needs an external tiebreaker is
  the load-bearing question.

- Covert-ordering: the always-needed ordering authority is the thing most "decentralized" group chat
  quietly smuggles in. The history is unkind here: every system that offered peer-to-peer-or-server
  found the server tier was the only one anyone used (Secure Scuttlebutt's pub nodes, Matrix's
  homeservers). An optional broker that makes survivor selection deterministic and available is, to
  that extent, an ordering service; the honest claim becomes "an ordering authority that is
  cryptographically blind and optional, at the cost of staleness when absent," not "no ordering
  authority."

Drystone's response lives in the spec (§7, synchronization and governance-conflict resolution) and impl.

### 5. History-on-join versus forward secrecy

Readable backfill fights forward secrecy directly, because forward secrecy is the property that old keys
cannot decrypt old messages, and "let the new member read the backlog" requires exactly that capability
to still exist for someone. The systems split predictably: Signal and MLS lean toward no history on join
(protect forward secrecy), while history-sharing designs keep a re-encryptable copy and accept the
weaker property. You cannot have full backfill and strong forward secrecy in the same transcript;
something gives.

### 6. Scaling and the churn-fold Achilles heel

Secure Scuttlebutt is the cautionary tale: unbounded append-only logs that every replicating peer stores
forever. MLS scales the rekey logarithmically via the tree, but the tree grows with members, and in a
per-device-as-member design it grows with members times devices. The specific, easy-to-miss mode is the
churn-fold: membership-log noise from device churn (every phone upgrade is an add plus a revoke) accretes
forever with no compaction authority once the server is removed, and every client must replay and fold
the whole log to render "who is in the room right now." This does not show up in short, small-group
tests; it surfaces at long horizon, high churn, and multiple partitions, when a client folds a large log,
gets the member list subtly wrong, and two members come to disagree about who is in the room. Keybase
survived this precisely because a server did the compaction and held the official tally; a
no-authority tier has every client doing the fold alone over a log that grows with people times devices
times time times fork-count.

Drystone's response lives in the spec (§11, and specifically §11.4 cost scales on the live set not the
roster, §11.7 re-entry, §11.8 the governance chain and standing resolution) and impl.

### 7. The social-versus-cryptographic identity gap (onboarding)

Keypair identity is hard for normal people (Secure Scuttlebutt, Briar, and Session all struggle with
"your identity is a key you can permanently lose"), and phone-number identity is easy precisely because
it outsources identity and recovery to the carrier and a centralized server, which is the privacy and
centralization liability. DID-based identity sits on the hard, key-based side of this line, which is a
real adoption tax worth naming rather than wishing away.

### 8. Governance, moderation, and genesis amendability

This is the least-studied failure mode and an under-appreciated killer. The field's pattern: moderation
powers were modeled as server or admin privileges, and when the server is removed, "who can kick whom"
must be encoded into the protocol or social layer, where it becomes a live attack surface. Secure
Scuttlebutt cannot delete or globally moderate content because there is no authority to enforce a
takedown and every peer holds its own copy; Briar punts to "only the creator invites." No pure-P2P
system has a convincing answer to a captured quorum, ban evasion, or a malicious majority, because those
are governance problems and the field treated them as access-control features.

Genesis amendability is the sharp secondary mode. Grounding the "who decides who decides" regress at an
immutable genesis threshold is more principled than anything in the pure-P2P field, but immutability is
also a trap: a threshold that fit at genesis (say, two of three) becomes wrong as a group grows to
thirty, and a captured quorum under fixed rules is permanent. The field's moderation failures teach that
governance needs to be amendable under its own rules; hard immutability trades that away for the clean
regress-grounding, and leaves "abandon the group and re-form" as the only escape valve.

Drystone's response lives in the spec (§7, governance-conflict resolution, and §11.8, the governance
chain, bans, and standing resolution) and impl.

## Matrix E2EE operational lessons

Matrix (with the Element client) is the one federated system that shipped decentralized encrypted group
chat in production, so its operational scars are the highest-value transferable knowledge: they are what
a large-group E2EE design meets at consumer scale. Matrix as a system and building block is catalogued
in `cairn/adjacent-systems.md`; only the lessons are here. Today's production crypto is Olm (1-to-1 key
agreement) plus Megolm (the group ratchet); MLS is in development, not production, and "MLS in a
decentralized setting" is the bookkeeping problem Matrix is still solving.

### The UTD "unable to decrypt" invariant

The dominant E2EE complaint is the "unable to decrypt this message" (UTD) tile, and the key finding is
that it is a key-availability problem, not a crypto-strength one. It occurs when a device lacks the
session for an event: the key was never shared to that device, the sender's device was unknown or
unverified, or session state broke. The invariant the field learned to hold is: can every current
member decrypt every current-epoch message. The dead UTD tile is the anti-pattern; the healthy path is
an explicit, friendly key-request and healing flow rather than a dead tile. (MLS narrows one whole class
of this, because group membership is itself the key-distribution mechanism and every member of the
current epoch can derive the key, but the equivalent risk persists at epoch boundaries and for messages
sent while a device was offline.)

Drystone's response lives in the spec (§11.9, delivery under scale, the race and graceful degradation)
and impl.

### Mandatory-recovery onboarding

Key backup and recovery is a cliff, not a slope. The documented Matrix failure is severe: a user with
encrypted messages, logged into only one session, who logs out without having set up a recovery
passphrase or key, loses access to those messages outright, because no server holds plaintext to fall
back on (which is the whole point and also the whole danger). The lesson: recovery setup is a
near-mandatory part of onboarding, and a user must not reach a single-device, no-recovery state without a
loud, blocking warning. Device verification and cross-signing (the master, self-signing, and
user-signing hierarchy held in Secure Secret Storage) are powerful but confusing, and an
unverified-but-legitimate device silently fails to receive keys, which is its own recovery-adjacent trap
and a driver of user attrition.

### The expectation-gap

The gap between the E2EE users are promised and the UX they actually get is itself a failure mode. Users
carry a centralized-cloud mental model ("log in on a new phone and see all my history") that E2EE cannot
cheaply satisfy: giving a newly added device old history is exactly where backfill and forward secrecy
collide, and Matrix's answer (server-side encrypted key backup) adds a passphrase burden and a homeserver
dependency. Instant full-history search is assumed, but a server that cannot read plaintext cannot index
it, so search can only be client-side and device-scoped (Element's local Seshat index is the working
precedent). Presence and typing indicators are assumed ubiquitous, but they are metadata-heavy and
expensive; Matrix disables presence at scale for mere performance reasons. Read as silence, each of these
gaps gets blamed on the app being broken. The lesson is to name each gap early and set expectations,
rather than let the promised-versus-delivered gap surface late.

The E2EE-versus-scale tradeoff underneath these gaps is mapped in the sibling `group-scale-versus-e2ee.md`;
this doc records only the operational lesson (what breaks, and how it was learned).

## What this establishes (and does not)

Establishes the descriptive map of how group-chat systems fail at scale: the recurring failure-mode
taxonomy (the person-versus-keypair gap, device-loss recovery, concurrent membership change, the
ordering/consensus assumption with its survivor-determinism and covert-ordering modes,
history-versus-forward-secrecy, the churn-fold scaling Achilles heel, the identity gap, and
governance/genesis-amendability) and the Matrix E2EE operational lessons (the UTD key-availability
invariant, mandatory-recovery onboarding, and the promised-versus-delivered expectation-gap). This is the
field a large-group design measures itself against.

Does **not** argue a design or reproduce Drystone's responses: those live in the spec (Part 2 §7 and §11)
and the implementation, cross-referenced inline. Does **not** catalogue Matrix as a system or any other
building block (that is `cairn/`). Does **not** restate the E2EE-versus-scale tradeoff or the per-group
operational rates and economics (those are the sibling docs `group-scale-versus-e2ee.md` and
`operational-rates-and-platform-economics.md`).
