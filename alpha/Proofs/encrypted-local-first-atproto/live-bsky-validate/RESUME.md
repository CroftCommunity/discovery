# RESUME — Live atproto validation (handoff)

**Purpose:** pick up live validation against real Bluesky test accounts in an
environment where outbound egress can be enabled. Everything is built and
committed; this file is the checklist to run it and interpret results.

## Where we left off

- The full local suite (Phases 1–12 + overview) is committed and green; see
  `experiments/README.md` for the hypothesis/experiments/results accounting.
- The live validator `experiments/live-bsky-validate/` is **complete and
  compiles**, but **could not run** in the originating session.
- **Blocker (verified):** the sandbox's network egress allowlist denies the live
  hosts. Probe result:
  ```
  $ curl https://bsky.social/xrpc/com.atproto.server.describeServer
  Host not in allowlist: bsky.social. Add this host to your network egress settings to allow access.
  ```
  Same for `plc.directory` and `jetstream2.us-east.bsky.network`. Credentials are
  necessary but NOT sufficient — the egress policy is the gate, configured at
  environment creation.
- Branch: `claude/encrypted-local-first-sync-y2hj1m`; PR: croftc/securitypolicy#3.

## Prerequisites to run (do these in the new environment)

1. **Allowlist egress** (environment network settings):
   - `bsky.social` — createSession / createRecord / getRecord / deleteRecord
   - `plc.directory` — DID resolution (used by atproto identity; not strictly
     required by the current validator but allowlist it for completeness)
   - `jetstream2.us-east.bsky.network` — the firehose read (Step 6)
   - If the test accounts are NOT bsky-hosted, allowlist their PDS host and set
     `PDS_HOST` to it.
2. **Set credentials as env vars** (use Bluesky **app passwords** — Settings →
   App Passwords — NOT the main account password; do not paste secrets into chat
   or commit them):
   ```
   export ATP_IDENTIFIER="handle-or-email-1"
   export ATP_APP_PASSWORD="xxxx-xxxx-xxxx-xxxx"      # app password for account 1
   export ATP_IDENTIFIER_2="handle-or-email-2"
   export ATP_APP_PASSWORD_2="yyyy-yyyy-yyyy-yyyy"    # app password for account 2
   # optional:
   # export PDS_HOST="https://bsky.social"
   # export JETSTREAM_HOST="wss://jetstream2.us-east.bsky.network"   # or "off" to skip firehose
   # export KEEP_RECORDS=1                                            # skip cleanup
   ```

## Run

```
cd experiments/live-bsky-validate
cargo run
```
Running with creds unset prints the configuration checklist instead of failing.

## What it validates (and pass criteria)

1. **Auth** — `createSession` for both accounts returns DIDs. PASS if both authenticate.
2. **Custom-lexicon createRecord** — author publishes `org.croftc.experiment.feed.post`. PASS if the live PDS returns 200.
3. **CID parity** — PDS-assigned CID == locally-computed CIDv1 (DAG-CBOR/SHA-256).
4. **strongRef reaction** — account 2 publishes a reaction referencing the post. PASS if 200.
5. **Readback** — `getRecord` round-trips and re-validates against our lexicon.
6. **Firehose (best-effort)** — the records appear on real Jetstream (filtered to our collections + DIDs).
7. **Cleanup** — `deleteRecord` removes both records (unless `KEEP_RECORDS=1`).

## What this does NOT validate (by design)

- **Identity binding (Phases 8/11)** against a real DID — needs the DID's
  verification key, which bsky-hosted accounts don't expose to an app password.
  To validate it live you need a **self-hosted PDS** or a **`did:web`** identity
  whose signing key you control. Track as a separate follow-up.
- Real transport (`iroh`), and the production OAuth flow (we use app passwords).

## Things to watch when it runs live (likely findings, not bugs)

- **Custom NSID acceptance.** Bluesky's PDS may accept arbitrary collection NSIDs
  (stores the record; the bsky app just won't render it) — or it may reject
  unknown lexicons. **Either outcome is a real finding.** The validator reports
  the PDS response faithfully rather than asserting success. If rejected, the
  conclusion is "the public publish path needs a registered/known lexicon or a
  self-hosted PDS," which feeds directly into the architecture decision.
- **CID parity may differ.** If Step 3 fails, it is almost certainly DAG-CBOR
  *canonicalization* differences (map key ordering, integer vs. float encoding)
  between our encoder (`serde_ipld_dagcbor`) and the PDS. That's a tractable
  encoding fix, and the mismatch itself is the useful signal.
- **Jetstream timing.** Step 6 backfills from a cursor ~10s before publish; if it
  observes 0 records, try `JETSTREAM_HOST` pointing at a different instance or
  re-run (the firehose is best-effort and does not gate the overall result).
- **Handle→PDS routing.** If accounts are on a non-bsky PDS, `createSession`
  against `bsky.social` may fail; set `PDS_HOST` to the account's actual PDS
  (resolve via the handle's DID document).

## After it runs — capture results back here

When you have output, record the verdicts (and any deviations) so the
`experiments/README.md` accounting can be updated from "validated locally" to
"validated against the live network" for H2/H3/H5. Specifically note:
- Did the live PDS accept the custom-NSID records? (yes/no + status)
- Did CID parity hold? (if not, paste both CIDs + the record JSON)
- Did the records appear on the firehose?
- Any behavior the local stand-in PDS did not surface.

## Pointers

- Validator: `experiments/live-bsky-validate/{src/main.rs,src/client.rs,src/jetstream.rs}`
- Lexicons published: `experiments/live-bsky-validate/lexicons/*.json`
- Local stand-in equivalent (for comparison): `experiments/local-pds-bridge/`
- Full accounting + open decisions: `experiments/README.md`
