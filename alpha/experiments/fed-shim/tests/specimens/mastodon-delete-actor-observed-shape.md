# Specimen: Mastodon Delete(Actor) activity JSON

**Date:** 2026-07-20
**Source:** Mastodon `app/serializers/activitypub/delete_actor_serializer.rb`
(the account-delete emit path). Cross-checked against AP-1.0 §5.7 (Delete)
and the AP guidance "receiver SHOULD remove, MAY Tombstone" (AP-1.0 §5.7
+ ActivityStreams-2.0 Tombstone).

## The shape (as bytes, single-line compact JSON)

```
{"@context":"https://www.w3.org/ns/activitystreams","id":"https://alice.example/users/alice#delete","type":"Delete","actor":"https://alice.example/users/alice","object":"https://alice.example/users/alice","to":["https://www.w3.org/ns/activitystreams#Public"]}
```

## Structure

An account-delete: `actor` and `object` are the SAME actor URL. Mastodon
emits `to: ["as:Public"]` on the Delete so it fans out to every follower
inbox (via sharedInbox where available). The shim emits the same shape
for wire fidelity.

## Key-order (Mastodon's emit)

`@context`, `id`, `type`, `actor`, `object`, `to`.

## Byte-fidelity notes

- Single-line compact JSON, no BOM, no trailing newline.
- The `to` array is exactly `["https://www.w3.org/ns/activitystreams#Public"]`
  (the compact-IRI `as:Public` expanded).
- Mastodon's Delete does NOT include `published`; the delete event has
  no useful timestamp on the wire (the receiver's clock governs when
  the redaction takes effect).

## Fields intentionally not modeled

- Delete of a Note (deleting a single post) — a distinct emit path
  (`delete_serializer.rb`) with a different `object` shape (either
  a nested Tombstone or a bare URL). The shim's Delete this run is
  **actor-only** (matches AP-V3's actor-delete semantics used by the
  ambassador).
- `bcc` / `bto` — omitted.
- LD-signatures — omitted.
