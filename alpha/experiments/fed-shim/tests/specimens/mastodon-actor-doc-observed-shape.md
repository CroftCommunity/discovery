# Specimen: Mastodon actor-document JSON-LD

**Date:** 2026-07-20
**Source:** Mastodon `app/serializers/activitypub/actor_serializer.rb`
(v4.x). Cross-checked against AP-1.0 §4.1 and the security-v1 context
for public-key serialization (draft-cavage HTTP-signatures §2.1.1).

## The shape (as bytes, single-line compact JSON — how Mastodon emits it
when queried at the actor URL with `Accept: application/activity+json`)

```
{"@context":["https://www.w3.org/ns/activitystreams","https://w3id.org/security/v1"],"id":"https://alice.example/users/alice","type":"Person","preferredUsername":"alice","inbox":"https://alice.example/users/alice/inbox","outbox":"https://alice.example/users/alice/outbox","publicKey":{"id":"https://alice.example/users/alice#main-key","owner":"https://alice.example/users/alice","publicKeyPem":"-----BEGIN PUBLIC KEY-----\nMIIBIjANBgkq...\n-----END PUBLIC KEY-----\n"}}
```

## Required fields (the shim's minimum viable Person)

- `@context` — array `[activitystreams, security-v1]`. Security-v1 is
  required to name the `publicKey`, `publicKeyPem`, `owner` terms.
- `id` — the actor URL. The URL served here MUST be the URL used as
  keyId in HTTP signatures (up to the `#main-key` fragment).
- `type` — `"Person"`. Mastodon uses `Service` for bots and `Group` for
  a group actor; the shim uses `Person` unconditionally.
- `preferredUsername` — the handle-part of `@alice@example.social`.
- `inbox` — the actor's inbox URL (where POSTs land).
- `outbox` — the actor's outbox URL (Mastodon serves it; the shim's
  outbox is a stub — see FED-SHIM.md §3).
- `publicKey.{id, owner, publicKeyPem}` — RSA public key in SPKI PEM
  format. `id` is `<actor-url>#main-key`; `owner` equals the actor
  URL.

## Fields Mastodon emits that the shim does NOT

- `following`, `followers`, `featured`, `endpoints.sharedInbox` —
  additional collection URLs. The shim serves inbox / outbox
  only; anything else 501s (`FED-SHIM.md §3`).
- `name`, `summary`, `url`, `manuallyApprovesFollowers`, `discoverable`,
  `indexable` — display metadata; not on the shim's fidelity path.
- `icon`, `image`, `attachment` — avatar / header / metadata fields;
  the shim serves none of them.
- `tag`, `alsoKnownAs`, `movedTo` — profile-migration and hashtag
  metadata; not modeled.

## Byte-fidelity notes

- Single-line compact JSON, no BOM, no trailing newline.
- The `publicKeyPem` value contains literal `\n` between the header /
  base64 body / footer — this is a JSON-escaped newline **inside** the
  string. Consumers `JSON.parse` and receive the raw PEM with real
  newlines.
- Key order: `@context`, `id`, `type`, `preferredUsername`, `inbox`,
  `outbox`, `publicKey`. Inside `publicKey`: `id`, `owner`,
  `publicKeyPem`.
