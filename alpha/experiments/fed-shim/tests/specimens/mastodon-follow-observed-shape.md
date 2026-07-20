# Specimen: Mastodon Follow activity JSON

**Date:** 2026-07-20
**Source:** Mastodon `app/serializers/activitypub/follow_serializer.rb`
(v4.x). Cross-checked against the ActivityStreams 2.0 vocab (AP-1.0 §5.1)
and the raw Follow bodies observed in the wild.

## The shape (as bytes, single-line compact JSON — how Mastodon emits it)

```
{"@context":"https://www.w3.org/ns/activitystreams","id":"https://alice.example/users/alice#follows/42","type":"Follow","actor":"https://alice.example/users/alice","object":"https://bob.example/users/bob"}
```

## Required fields (Mastodon's serializer emits exactly these)

- `@context` — string, always `"https://www.w3.org/ns/activitystreams"` for
  a bare Follow (Mastodon uses an array `["https://www.w3.org/ns/activitystreams", "https://w3id.org/security/v1"]` only when key material is inlined).
- `id` — the activity URL. Mastodon shape: `<actor-url>#follows/<local-id>`.
- `type` — `"Follow"` (case-sensitive).
- `actor` — the follower's actor URL (string, not object).
- `object` — the followee's actor URL (string, not object). NB: for a
  Follow, `object` is an actor URL. For an Undo of a Follow, `object` is
  a nested Follow object.

## Fields Mastodon does NOT emit on a bare Follow

- `to`, `cc`, `audience` — omitted (Follow is a direct interaction).
- `published` — omitted on the Follow itself (Mastodon serializes
  `created_at` only when the receiver requires it, e.g. as part of a
  Create; a Follow doesn't carry it).
- `bcc`, `bto` — omitted.

## Byte-fidelity notes

- Mastodon emits JSON without newlines and without redundant whitespace
  (`multi_json`-based; equivalent to `JSON.generate` on the serializer's
  hash).
- Key order in Mastodon's output is the serializer's insertion order:
  `@context`, `id`, `type`, `actor`, `object`. The shim MUST match this
  order verbatim.
- No BOM, no trailing newline.

## Fields the shim intentionally does not model

- `@context` array with security context — Mastodon only emits the
  array form when key material is inlined into the activity (rare for
  Follow). The shim omits.
- `signature` field (LD-signatures, deprecated in favor of HTTP
  signatures). Not emitted by the shim; Mastodon's own emit also
  degrades from LD-signatures at increasing rates in current
  versions.
