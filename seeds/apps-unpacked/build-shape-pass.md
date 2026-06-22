# Build + Shape Pass

author: research session

scope: (1) license verification for the wrap-ready webxdc apps, (2) an iroh-docs schema for split-the-check, (3) a design for local real-time voting, (4) a UX model for ponds, pinning, and deep-link sharing that keeps complexity off the user

date: 2026-06-21

note on confidence: claims about how iroh-docs works are grounded in the n0 crate docs and verified below. Anything I could not fully confirm is marked [UNVERIFIED].

---

## 1. License verification (wrap-ready webxdc apps)

These now live on Codeberg, but the GitHub mirror (public archive, pre-move) carried explicit SPDX license metadata, which is the authoritative record of what they were released under. Codeberg's repo cards do not render a license badge, and each repo does contain a LICENSE file, so the values below are from the GitHub mirror and should get a final one-line confirmation by opening each LICENSE file directly before bundling.

| App | License | Source of truth | Notes |
|---|---|---|---|
| editor (Yjs + ProseMirror) | **MIT** | GitHub mirror, archived | In maintenance mode; uses y-webxdc provider. Permissive, clean to fork. |
| checklist (Automerge CRUD) | **Unlicense** | GitHub mirror, archived | Public-domain-equivalent. LICENSE file present on Codeberg since 2022. |
| corkboard (CRUD, no CRDT) | **Unlicense** | GitHub mirror, archived | "Sharing lists with a group." The plain-CRUD counterpart to checklist. |
| poll (1 question, 5 answers) | **Unlicense** | GitHub mirror, archived | The decision-tool starting point. |
| (one early example app) | **MPL-2.0** | GitHub mirror, archived | MPL is weak copyleft (file-level); only matters if you reuse that specific app. |

Bottom line: the collaboration references you actually want (editor, checklist, corkboard, poll) are **MIT or Unlicense**, which is as permissive as it gets. No copyleft obstacle, no attribution-art trap like the chess set. The only caveat is process, not risk: confirm each LICENSE file at bundle time, because a repo can be relicensed after a move and Codeberg recently widened its allowed-license terms.

Caveat I want to be honest about: Codeberg was not reachable from the research sandbox for cloning, and its web UI does not surface an SPDX badge, so I am leaning on the GitHub archive metadata. High confidence, but [UNVERIFIED] against the live LICENSE file contents as of today.

---

## 2. iroh-docs schema for split-the-check

### How iroh-docs actually works (verified, because the schema depends on it)

Confirmed against the n0 crate docs and docs.rs:

- The unit is a **Document** (a.k.a. Replica). Its identity is a **NamespaceId**, the public key of a keypair. The **namespace secret key is the write-capability token**: holding it lets you write to the doc.

- An **Entry** is identified by the triple **(namespace, author, key)**. Its stored value is the **32-byte BLAKE3 hash** of the content, plus size and a **timestamp**. The actual content bytes live in iroh-blobs, not in the doc. The doc syncs metadata; blobs moves content lazily on request.

- **Authors** are separate keypairs (AuthorId = public key). Any number of them, meaning is application-defined. They are **proof of authorship**, signed per entry.

- Conflict resolution is **last-write-wins per (author, key)**, by timestamp. Sync is **range-based set reconciliation** (peers exchange fingerprints, converge in a few messages; fully-synced peers exchange a single fingerprint). **iroh-gossip** carries live "new entry" notifications so peers update in near-real-time, not just on reconnect.

Two consequences that shape the design, and that are easy to get wrong:

**Consequence A: the namespace secret is shared write-capability, not per-user.** Everyone who can write holds the same namespace secret. Authors give you *attribution and signatures*, but they do not by themselves stop someone from writing under a different key, last-write-wins is per (author, key), so two authors writing the "same" logical fact write to *different* entries and both survive. This is good for an append-style ledger and a thing to handle deliberately for any "one canonical value" field.

**Consequence B: LWW is fine for facts that are owned by one author, dangerous for shared mutable totals.** Never store a running balance as a single mutable key that everyone overwrites; the last writer would clobber. Store the immutable *events* (each authored by its creator) and *derive* balances by folding the event log locally. This is event-sourcing, and it is the natural grain of iroh-docs.

### The schema

Model the ledger as an **append-only event log of immutable expense entries, each authored by its creator, with balances computed client-side**. One document per expense group (per pond instance).

Document (Replica): one per group. Namespace secret = the group's shared write capability, distributed in the invite ticket.

Author: each member's device gets an AuthorId. This is who-recorded-this, signed.

Entries (key naming uses a sortable, namespaced convention):

```
# An expense event. Immutable. Key embeds a ULID so keys are unique and time-sortable,
# which sidesteps the LWW-clobber problem entirely: no two events share a key.
expense/{ulid}            -> blob: {
                               id:        ulid,
                               payer:     memberId,         # who paid
                               amount:    integer_minor_units,   # cents, never floats
                               currency:  "USD",
                               splits:    [ {member, weight} ],  # how to divide
                               note:      "dinner",
                               created:   iso8601,
                               # author signature is intrinsic to the entry, not in the body
                             }

# A correction or void references a prior event rather than mutating it.
# You never edit expense/{ulid}; you append a reversal.
void/{ulid_of_target}     -> blob: { target: ulid, reason, created }

# A settlement (someone paid someone back). Also an immutable event.
settle/{ulid}             -> blob: { from: memberId, to: memberId, amount, currency, created }

# Member roster. This one is small mutable state; keep one entry PER member,
# keyed by memberId, so each member owns their own row (no cross-member clobber).
member/{memberId}         -> blob: { displayName, joinedAt, leftAt? }

# Optional group metadata, owned by the creator author.
meta/group                -> blob: { name, baseCurrency, createdAt }
```

Balance computation (client-side, deterministic fold over the log):

```
for each expense:   payer credited (amount), each member debited (amount * weight / total_weight)
for each void:      remove the referenced expense's effect
for each settle:    from credited, to debited (the payback moves the needle the other way)
net[member] = sum of credits - sum of debits
```

Because every event is immutable and authored, the fold is order-independent except for display, and every peer computes the identical balance from the same entry set. No shared-counter race, no float drift (integer minor units only), no server.

### Why this grain fits

- **Immutable events + LWW** never collide, because each event has a unique key. You only use mutable-key LWW for the member roster, where each member owns exactly one key, so there is no contention.

- **Corrections are appends** (void/settle), which preserves an auditable history. That suits a money ledger and matches the cooperative-transparency ethic: everyone can see how a balance was derived.

- **Content vs metadata split** means the doc stays tiny (hashes only) and syncs instantly; the small JSON blobs fetch lazily. For a text-only ledger this is all fast.

Open design choices to decide, not assume:

- **Membership and write-capability revocation.** iroh-docs gives one shared namespace secret. If someone leaves a group, you cannot un-issue their write capability without rotating the namespace (new doc, migrate state). For a friends-splitting-dinner group this is acceptable; flag it for the threat model. [UNVERIFIED] whether current iroh-docs exposes finer-grained capability revocation; the crate docs describe a single namespace write token.

- **Multi-currency.** Store amounts in minor units with an explicit currency per event; do conversions only at display time, never store a converted total.

- **Simplify-debts** ("who should pay whom to settle up") is a pure client-side computation over net balances; it needs no protocol support.

---

## 3. Local real-time voting (this is genuinely the interesting one)

You are right that it is fascinating, and the reason is specific: **real-time voting is the rare utility where watching it happen is the product.** A poll you submit and check later is a form. A poll where the bars move as your friends tap, live, is a small piece of theater. The "it's alive" energy is maximal here because votes are social signals and seeing them land in real time changes how people vote.

### Two modes, two transports, two feels

The key design fork is whether the live tally is **open** or **secret**, and it maps cleanly onto iroh's two relevant layers.

**Mode A: open / live-tally (iroh-gossip).** Everyone sees every vote as it lands. Built on the ephemeral gossip layer: each vote is a broadcast message, the UI tallies live. This is the fun one: ranked-choice for "where do we eat," dot-voting on ideas, a live applause meter, a reaction poll during a shared moment. No persistence needed; the result is the moment. The drama (a late vote flipping the lead) is a feature.

- transport: iroh-gossip. tempo: real-time. wrinkle: none for fairness, since open voting is by definition not secret. Watch for **bandwagon dynamics** as a *design* property, not a bug: live tallies cause herding. If you want considered votes, hide the tally until close; if you want fun, show it.

**Mode B: secret-ballot (commit-reveal over iroh-docs or gossip).** Nobody should see how others voted until everyone has, and nobody should be able to change their vote after seeing the tally. Pure P2P has no neutral referee, so you use the same **commit-reveal** pattern as fair dice:

```
Round 1 (commit):  each voter broadcasts  H = hash(vote || nonce)
                   everyone collects all commits. No vote is readable yet.
Round 2 (reveal):  once all commits are in (or a deadline hits),
                   each voter broadcasts (vote, nonce).
                   everyone verifies hash(vote||nonce) == the commit they saw.
Tally:             computed locally, identically, by every peer.
```

This gives you a provably fair secret ballot with no server and no trusted counter: you cannot peek (commits are opaque), you cannot change your vote after seeing others (your commit is locked), and you cannot stuff (one commit per author signature). It is a small amount of code and it is the same primitive you already need for dice and for the fair randomizer, so build it once as a shared "fair-reveal" module and reuse it across games and voting.

- transport: iroh-gossip for the live rounds, optionally iroh-docs if you want the closed result to persist (e.g. a recorded group decision). tempo: two-round, near-real-time. wrinkle: the **abort case**, a voter commits then disappears before revealing. Handle with a deadline: after timeout, reveal proceeds with whoever showed up, and non-revealers are dropped. Decide whether a missing reveal voids the round or just abstains. [UNVERIFIED] re any existing webxdc/iroh commit-reveal lib; I would build it, it is small.

### Why local + real-time specifically shines

- It is **legible**: a poll is the most universally understood interaction there is, so onboarding weight is near zero.

- It is **social by nature**: votes are signals about the group, so seeing them live makes the group feel present, which is exactly the pond's purpose.

- It **doubles as governance**: the secret-ballot mode is the seed of cooperative decision-making for Croft and any commons-governance use, and the open mode is the casual group-decision toy. Same code, two registers.

- It is the **cleanest possible demo of the fair-reveal primitive**, which you need anyway, so it pays for itself twice.

This is a strong candidate to promote from the utility list into a flagship, precisely because it sits exactly on the intersection of fun, useful, and values-demonstrating that the whole project is about.

---

## 4. Keeping complexity off the user: ponds, pinning, deep-links

Your instinct is the correct one and it is the central product risk. A bag of twenty P2P apps is a developer's delight and a normal person's confusion. The job is to make the surface feel like *one simple thing* while the composable machinery hides behind it. Here is a model that does that.

### The mental model: a pond is a room, an app is a thing you do in the room

Do not present "apps." Present **rooms (ponds) that contain activities**. A person is not launching "the iroh-docs expense app," they are "in the trip group, splitting the bill." The pond is the unit of belonging; the app is just what you are doing right now inside it. This reframing alone removes most of the felt complexity, because people already understand grouped spaces (a chat, a channel, a room) and do not want to understand an app launcher.

Three layers, each with exactly one job:

**Layer 1 - Ponds (the only thing most users navigate).** A short list of the groups you are in: "Family," "Roommates," "Climbing crew." This is home. Most people live here and never think about apps as a category at all.

**Layer 2 - Activities within a pond (grouped by utility, surfaced by use).** Inside a pond, the things you can do, organized by purpose not by app identity: *Decide* (polls, scheduling), *Track* (expenses, lists), *Make* (whiteboard, notes), *Play* (the games), *Share* (files, photos). The grouping-by-utility you proposed is right; it turns twenty apps into five legible verbs. Within each verb, the specific tools are a short, curated set, and recently-used ones float up.

**Layer 3 - The pinned top layer (the user's own shortcuts).** The user pins the two or three things they actually use to a persistent top strip, per pond or globally. "We always split bills and we always use the whiteboard," pin those, everything else recedes into the grouped drawer. This is the pressure valve: power users assemble their own simple surface, casual users never open the drawer at all. Pinning is the mechanism that lets the catalog be deep without the default surface being busy.

### The "jump to the one thing" share (this is the unlock)

The most important interaction, and the one that ties back to the games-on-Bluesky hook, is the **deep-link that drops someone directly into a single activity with all context resolved**. No app browsing, no "now find the poll," just tap and you are *in the thing*.

Concretely, a share is a link/ticket that encodes three things:

```
pond+app://  {pond-or-group identity}      # which room (and the invite/capability if joining)
           / {app id + version}            # which activity, pinned version for reproducibility
           / {instance + entry context}    # which specific poll/ledger/board, and where to land
```

Behaviors that make it feel like magic rather than configuration:

- **One tap resolves everything.** Receiving a "vote on dinner" link joins the pond if needed (via the embedded ticket), opens the voting activity, and lands on *that poll*, ready to tap a choice. The composable layers all fire, invisibly.

- **It degrades gracefully.** If the app is not present, the link carries enough to fetch/wrap it (your webxdc-shim catalog) before landing. If the recipient is not in the pond, the same link is the invite. One artifact, many entry states, exactly the pattern you described.

- **It rides the social layer you already have.** A challenge or a poll posted to Bluesky is one of these links. The games hook ("see a challenge on a post, tap, play") and the utility share ("vote on this," "see our album") are *the same mechanism* with different payloads. Build the deep-link resolver once; it serves games, utilities, and the social pond identically.

- **Shareable at any granularity.** Share a whole pond ("join our family room"), an activity ("we use this whiteboard"), or a single instance ("this specific vote"). The same URL grammar, three depths.

### Principles to hold the line on simplicity

- **Default surface shows verbs, not apps.** A person sees "Decide / Track / Make / Play / Share," never a grid of twenty icons. The catalog depth lives one tap down.

- **Pinning is the personalization, not configuration.** No settings screens. You use a thing, you can pin it. That is the entire customization model.

- **Deep-links collapse navigation to zero.** The fastest path to any activity is a link someone sent you, which lands you *in* it. Browsing is the fallback, not the default.

- **One pond, one membership, one invite.** Joining a pond is the only "account-like" act, and it is just accepting a ticket. Apps inside inherit that membership; the user never re-joins or re-authorizes per app.

- **The seam is also the boundary.** Each pond is its own context (security boundary from the earlier audit point); pinning and deep-linking operate within that boundary, so a deep-link into one pond never leaks capability into another.

The net effect: a non-technical user experiences "rooms with a few things I do in them, and links that take me straight to the thing," while the composable, multi-app, multi-protocol machinery you actually built sits entirely underneath. The complexity is real; the *exposed* complexity is three verbs and a pin.

---

## Quick answers to the three asks

**Licenses:** editor MIT, checklist/corkboard/poll Unlicense, plus one MPL-2.0 example. All permissive, no copyleft or art traps. Confirm each LICENSE file at bundle time since they moved hosts. [UNVERIFIED] against today's live files.

**Split-the-check schema:** one iroh-docs document per group, append-only immutable expense/void/settle events each authored and keyed by ULID, member roster as per-member owned keys, balances folded client-side in integer minor units. Avoids the LWW-clobber trap by never storing a shared mutable total.

**Real-time voting:** two modes. Open live-tally on iroh-gossip (the fun, theatrical one). Secret-ballot via commit-reveal (the fair, governance-grade one), reusing the same fair-reveal primitive you need for dice. Promote it toward flagship; it sits dead-center on fun-plus-useful-plus-values.

**Complexity / UX:** ponds as rooms, activities grouped by verb (Decide/Track/Make/Play/Share), a user-pinned top layer for the two or three things they actually use, and a deep-link grammar that drops people straight into a single activity with membership and context auto-resolved, the same mechanism as the Bluesky game hook.

### Verification notes

iroh-docs model (namespace = write-capability keypair, entries keyed by namespace+author+key, value = BLAKE3 hash + timestamp, LWW per author+key, range-based set reconciliation sync, gossip for live updates, blobs for content) confirmed via docs.iroh.computer, docs.rs/iroh-docs, and the n0-computer/iroh-docs README. Webxdc app licenses confirmed via the webxdc GitHub org mirror; final LICENSE-file confirmation pending because the apps now live on Codeberg, which was unreachable for cloning from the sandbox. Commit-reveal is standard cryptographic practice; no specific existing webxdc/iroh implementation was located, so treat it as build-it-yourself (small). Namespace-level write-capability revocation granularity in current iroh-docs is unconfirmed and flagged for your own check.
