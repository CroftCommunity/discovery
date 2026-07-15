# Live Bluesky Validation (ready, pending egress + creds)

Validates against the **real atproto network** (two Bluesky test accounts) the
claims the local phases proved on a stand-in PDS. This is the one piece the rest
of the suite couldn't run, because the sandbox's **egress allowlist** blocks the
live hosts. The code is complete and compiles; it runs as soon as the
environment is configured.

```
cargo run -p live-bsky-validate
```

Run with no creds and it prints exactly what to configure.

## What it validates (against the live network)

1. **Both test accounts authenticate** (`createSession`, app-password).
2. **The live PDS accepts a custom-NSID lexicon record** (`org.croftc.experiment.feed.post`) via `createRecord`.
3. **CID parity** — the PDS-assigned CID equals our locally-computed CIDv1 (DAG-CBOR/SHA-256). A mismatch is itself a finding (DAG-CBOR canonicalization differences).
4. **A strongRef reaction** from the second account references the post.
5. **Round-trip readback** (`getRecord`) re-validates against our lexicon.
6. **(best-effort)** the records appear on the **real Jetstream firehose** (filtered to our collections + DIDs, backfilled from a pre-publish cursor).
7. **Cleanup** — `deleteRecord` tidies both repos afterward.

## Prerequisites (your side)

This cannot run until the environment is configured — verified blocked here
(`Host not in allowlist: bsky.social`):

1. **Egress allowlist** (environment network settings): `bsky.social`,
   `plc.directory`, and the Jetstream host (`jetstream2.us-east.bsky.network`).
   If your test accounts live on a non-bsky PDS, allowlist that host and set
   `PDS_HOST`.
2. **Credentials as env vars** (never paste app passwords in chat):
   `ATP_IDENTIFIER`/`ATP_APP_PASSWORD` (author) and
   `ATP_IDENTIFIER_2`/`ATP_APP_PASSWORD_2` (reactor). Use **app passwords**
   (Bluesky Settings → App Passwords), not the main account password.
3. Optional: `PDS_HOST`, `JETSTREAM_HOST` (or `off` to skip the firehose step),
   `KEEP_RECORDS=1` to skip cleanup.

## Scope / honesty

- **Publishing is public.** atproto repos are public; published records hit the
  relay/firehose. Cleanup deletes them from the repos, but cannot un-emit what the
  firehose already carried. Records use a clearly-experimental NSID and label.
- **Identity binding (Phase 8/11) is NOT validated here.** It needs the DID's
  verification key, which bsky-hosted accounts don't expose to an app password;
  it requires a self-hosted PDS or `did:web`. Documented, not attempted.
- **If the PDS rejects custom NSIDs**, that is a real, valuable finding (it tells
  us the public publish path needs a registered/known lexicon or a self-hosted
  PDS) — the validator reports the PDS response faithfully rather than asserting
  success.

## Resolved versions

rustc 1.94.1 · reqwest 0.13.4 (rustls) · tokio-tungstenite 0.29.0 (rustls,
webpki-roots) · cid 0.11.3 · serde_ipld_dagcbor 0.6.4 · serde_json 1.0.150.
