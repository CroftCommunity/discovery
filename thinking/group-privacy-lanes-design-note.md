# Design Note: Public and Private Group Lanes

author: research agent

date: 2026-06-13

status: draft for discussion

scope: how the stack offers users a choice between private and public groups without weakening the encryption model

---

## Problem

Users will expect to choose whether a group is private or public, the way Discord, Matrix, and now Bluesky all let them. The naive implementation is a `visibility` flag on a single group object. That is the wrong model for an end-to-end encrypted stack, because "private" and "public" are not two settings on one object. They are two different trust models, and collapsing them into one flag reintroduces the exact failure the encryption exists to prevent.

This note specifies a three-lane model instead, where the user's choice routes content down a different path at compose time rather than toggling a property on a shared object.

## Why not a visibility flag

Three reasons, each tied to a specific layer.

The MLS layer makes world-readable and encrypted-to-members mutually exclusive. A group key gates decryption to the current epoch's members. If a non-member can read the content, the content is by definition not gated by that key, so it is not an MLS group at all. A `public: true` flag on an MLS group is a contradiction the crypto cannot honor.

The precedent layer shows the footgun is real. Matrix's `m.room.history_visibility` is effectively a visibility flag, and its `shared` default has caused recurring confusion where prior private conversation becomes visible to newly invited members. The lesson is that a quiet setting governing who can read is a setting people misjudge.

The blind-broker layer means we cannot lean on a server to enforce the distinction at read time. A centralized app can store everything in the clear and gate reads with an access check. We cannot. The distinction has to be structural (different keys, different path), because the broker is not trusted to see content and therefore cannot be trusted to enforce who reads it.

## The three lanes

### Lane 1: Private, closed membership

A standard MLS group, invite-gated.

Membership is the MLS ratchet tree. Epoch keys gate decryption to current members. The broker sees only ciphertext. Forward-only history on join falls out of epoch semantics for free, since a new member cannot derive keys for epochs before they joined. Shared mutable state (messages, reactions, pins, roles) lives in an Automerge document encrypted to the group; a new member bootstraps from a snapshot at their join epoch.

Fit: Natural. This is the case the stack is built for.

### Lane 2: Private, open-join

Still a real MLS group, but the approval gate is removed.

Anyone may request to join and is auto-admitted, becoming a full MLS member with forward-only history. Everything else is identical to Lane 1. The cost is operational, not architectural: MLS performs an epoch change on every membership change, so a high-churn open group rekeys frequently and the ratchet tree grows with the roster. That is a scale consideration to bound (for example, cap size or rate-limit joins), not a new trust model.

Fit: Effortful at scale, fully encrypted throughout.

### Lane 3: Truly public / world-readable

Not an MLS group at all. This is the atproto public lane.

World-readable content is published as atproto records, the same path used for public social content. There is no group key because there is no membership boundary to enforce. This is where the "anything that reaches the relay is public" property is correct and wanted, rather than a leak. Read access is universal by design; there is nothing to gate.

Fit: Natural, but as a different mechanism. It is publishing, not encrypted group messaging.

## The routing decision

User choice governs the lane and the openness within a lane, but it cannot make MLS simultaneously gate content to members and expose it to non-members. So the choice is implemented as routing at compose time, not as a property on a group:

- "Private group" composes to Lane 1 or Lane 2 (closed vs. open-join), an MLS group on the encrypted iroh path.

- "Public" composes to Lane 3, atproto records on the public path.

The two paths share one thing deliberately: identity. A user's AT Protocol DID is the same whether they are a member of a private MLS group or the author of a public atproto record. Identity is shared; the message path is not. This is the clean piggyback on atproto, and it is also where we improve on the leading precedent (see below).

A note on terminology that has caused confusion: MLS is not "encryption in transit." Transit encryption (iroh's QUIC/TLS link layer) protects a hop and terminates at each endpoint. MLS is end-to-end group encryption where the broker never sees plaintext. The private lanes use both, at different layers. Choosing MLS is choosing end-to-end, which is the opposite of transit-only.

## Identity binding: learn from Germ, then harden

Germ (the MLS-over-atproto messenger Bluesky integrated) solves the identity-to-key binding by publishing an "Anchor Key" in the user's atproto profile bio and monitoring it for changes. It works, and Germ is candid that it is a stopgap with a masquerade risk: whoever controls the account or its data server can swap the key, and users are expected to watch for that.

We should not copy the mutable-bio binding. Our DID-based identity can bind device and MLS credentials through the DID document's verification methods or signed key records, which is a tighter binding than editable profile text. Any key rotation should be surfaced to group members as a security-boundary event, not applied silently. This is a concrete place our design can beat the shipped precedent rather than inherit its weak seam.

## Crypto/transport layering (implementation reference)

The private lanes (1 and 2) need an MLS engine driven over iroh, not over a vendor's delivery service. Wire's `core-crypto` is the best available reference that this is viable: it wraps `openmls` and exposes an `MlsTransport` trait of client callbacks with separate endpoints for messages and commit bundles, so the delivery mechanism is pluggable rather than baked in. Its `TransactionContext` model (all mutating operations go through a transaction) is a useful pattern for ordering membership and permission commits and reconciling them with Automerge.

The caveat is that core-crypto is GPL-3.0-only, which is a copyleft obligation to weigh before depending on it. The permissive alternatives are to build directly on `openmls` (which our stack already names) with a thin transport and keystore layer of our own, or to evaluate AWS Labs' `mls-rs` (Apache-2.0). Either way, the `MlsTransport`-style decoupling is the pattern to follow so the crypto layer never assumes a particular transport.

## What we adopt from the precedents

Taxonomy: separate private chats from public communities, exactly as Bluesky now does (group chats as one feature, public communities with their own public/private settings as another). Do not merge them into one object.

Invite UX: Bluesky's invite-link-as-embedded-card is a clean pattern, as is the "who can invite me" control (everyone / only people I follow / no one). These are worth copying directly for Lanes 1 and 2.

What we do not adopt: routing private messaging through Bluesky's native group chats, which are access-controlled but not end-to-end encrypted (operator-visible for moderation), and any data-server-mediated transport that would reintroduce a metadata-bearing hop our blind broker is designed to remove.

## Open questions

- Bound for Lane 2 churn: what join rate and group size make per-join epoch rekeying acceptable, and do we cap or rate-limit.

- Whether Lane 3 public content and a Lane 1/2 private group should ever be linkable under one group identity, or kept deliberately separate to avoid leaking that the same people run both.

- Recovery and multi-device for the private lanes, which is the hardest UX problem and is covered separately in the competitive analysis (Matrix lessons section).

- core-crypto GPL-3.0 vs. building on `openmls` directly: a licensing decision that needs an owner.
