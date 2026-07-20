# Specimen: Mastodon Undo Follow activity JSON

**Date:** 2026-07-20
**Source:** Mastodon `app/serializers/activitypub/undo_follow_serializer.rb`.
Cross-checked against ActivityPub §7.4 (Undo).

## The shape (as bytes, single-line compact JSON)

```
{"@context":"https://www.w3.org/ns/activitystreams","id":"https://alice.example/users/alice#follows/42/undo","type":"Undo","actor":"https://alice.example/users/alice","object":{"id":"https://alice.example/users/alice#follows/42","type":"Follow","actor":"https://alice.example/users/alice","object":"https://bob.example/users/bob"}}
```

## Structure

An Undo activity whose `object` is a **nested Follow activity object**
(not just the Follow's URL). The nested Follow carries its own `id`,
`type: "Follow"`, `actor`, and `object`. This is the shape AP-1.0 §7.4
mandates (the target of an Undo is the activity being undone).

## Key-order (Mastodon's emit)

- Outer: `@context`, `id`, `type`, `actor`, `object`.
- Nested Follow: `id`, `type`, `actor`, `object`. (No `@context` on the
  nested activity — the outer's `@context` covers the whole envelope.)

## Byte-fidelity notes

- Single-line compact JSON, no BOM, no trailing newline.
- The nested Follow's `id` MUST equal the `id` of the Follow being
  undone (so the receiver can pair the Undo with its matching
  Follow).

## Fields intentionally not modeled

- `published`, `to`, `cc`, `bto`, `bcc` — omitted (as for Follow).
- LD-signatures — omitted.
