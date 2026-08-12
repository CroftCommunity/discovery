# croft-relay: iroh-relay extensions for atproto-gated calling

Implementation plan for Claude Code. Phased, TDD-first, each phase sized as one
reviewable, upstreamable unit with its own "story" (the one why).

---

## 1. Context and product story

We are building a calling system where:

- Identity lives in atproto. A custom lexicon record (`ing.croft.iroh.endpoint`,
  rkey `self`) in a user's PDS repo binds their DID to an iroh EndpointId and a
  home relay URL (`https://relay.croft.ing`).

- Transport is iroh. Callers resolve handle -> DID -> PDS record -> EndpointId,
  then dial. The relay carries first contact and coordination; holepunching
  upgrades to direct QUIC when possible.

- The relay is the enforcement point for admission, and the app layer is the
  enforcement point for pairwise call policy (mutuals-only etc.). The relay
  never learns call content and never holds social-graph state.

Product modes the relay must support, as discrete dials:

1. **Registered-only reception**: only endpoints holding a valid credential
   issued by our enrollment service may attach (camp or dial in).

2. **Coordination-only tier**: admitted, but rate-limited so hard that
   holepunch coordination works while sustained relayed media is starved.
   (Content-based splitting is impossible by design: the relay cannot
   distinguish disco frames from app data. Volume is the only honest proxy.)

3. **Full-broker tier**: generous or absent limits. Tier is a claim in the
   credential, chosen at admission, enforced by a per-connection rate bucket.

Explicitly out of scope for v1 (designed, deferred): pairwise (src, dst)
policy in the forwarding path; priority scheduling between tiers; firehose-
driven cache invalidation. Do not build these. Leave seams.

---

## 2. Grounding: current upstream state (verify at Phase 0, do not trust this doc)

As of iroh ~1.0.x (verify exact versions in-repo):

- `iroh-relay` supports access control modes in TOML `access` field: open,
  shared bearer tokens (`Authorization: Bearer` header or `?token=` query
  param, no revocation without restart), and an HTTP hook that POSTs per
  incoming connection with header `X-Iroh-Endpoint-Id` (hex endpoint id),
  granting access only on `200` + body `true`.

- Per-client receive rate limits exist in config:
  `[limits.client.rx] bytes_per_second`, `max_burst_bytes`.

- 1.0.2 added `RelayService::set_client_rate_limit` for live updates, but the
  limit appears service-wide (one value picked up by all connections), not
  per-connection.

- The `Bucket` rate-limit primitive was made public for embedders that mount
  the relay protocol under their own HTTP server (axum etc.).

- The relay authenticates the connecting endpoint's key during attach, so the
  endpoint id in the access check is cryptographically bound, not asserted.

- n0's managed relays use short-lived signed access tokens scoped to the
  endpoint's identity. That pattern is not in the self-host OSS path. It is
  our template for Phase 2.

Primary sources to re-verify (fetch fresh, pin exact commit in ADR-0001):

- https://github.com/n0-computer/iroh/tree/main/iroh-relay (README, config docs)

- https://github.com/n0-computer/iroh/blob/main/iroh-relay/src/main.rs

- https://github.com/n0-computer/iroh/releases (search: access control, rate limit)

- https://docs.iroh.computer/concepts/relays and /concepts/holepunching

---

## 3. Architectural decision to make first: embed vs fork

Two viable shapes. Decide in Phase 0 and record as ADR-0001.

**Option A: embed (preferred if feasible).** New binary crate `croft-relay`
that depends on `iroh-relay` as a library, mounts the relay protocol under our
own axum server, and implements admission + per-connection buckets in our
code using the public `Bucket` primitive. Upstream diff: zero or near-zero.
Upstreams become "please expose X" PRs, which are the easiest kind to land.

**Option B: patch fork.** Fork `n0-computer/iroh`, carry a short patch series
on `iroh-relay`. Each phase is one series entry. Higher maintenance cost,
cleaner if the library seams turn out not to exist.

Decision rule: attempt Option A first. Fall back to B only for the specific
capability the library cannot express, and keep that patch minimal and
upstream-shaped. Hybrid is acceptable (embed + one small patch).

---

## 4. What Claude Code needs

Environment:

- Rust stable toolchain matching the repo's MSRV (check `Cargo.toml` /
  `rust-toolchain.toml`); `cargo`, `clippy`, `rustfmt`.

- `cargo-mutants` for mutation testing (`cargo install cargo-mutants`).

- Network access to clone `n0-computer/iroh` and fetch docs.

- Ability to run multi-process integration tests locally (relay on localhost
  ports; upstream has integration tests for embedding the relay to crib from).

Inputs from the human (answer before or during Phase 0; defaults provided so
this remains single-shot runnable):

- **Token format** for Phase 2. Default: JWT with EdDSA (ed25519), claims:
  `sub` = hex EndpointId, `tier` = `coordination` | `broker`, `exp` short
  (minutes to an hour), `iss` = enrollment service. Rationale: boring,
  library-supported, auditable. Alternative if dependency-averse: raw ed25519
  signature over a canonical CBOR struct. Pick JWT unless told otherwise.

- **Repo home**: default new repo `croft-relay` with iroh as a git dependency
  pinned to a release tag.

- **CI**: default GitHub Actions, fmt + clippy + test + (nightly job) mutants.

Conventions to detect in Phase 0 and then follow (do not assume):

- Upstream test style and error-handling crate (they use `n0_error` and
  friends; match it in any patch destined upstream).

- Commit message and changelog conventions (they appear to use conventional
  commits, e.g. `feat(iroh-relay): ...`; verify against recent history).

---

## 5. Phases

Every phase: write failing tests first, implement to green, refactor, then run
mutation testing on the new module and either kill surviving mutants with
tests or document why they are equivalent/acceptable in the phase's ADR.
Every phase ends with: ADR written, README section updated, branch pushed,
self-review pass for "would n0 merge this shape."

### Phase 0: Baseline and reconnaissance

Story: we change nothing until we can build, run, and test what exists, and
until this plan's claims about upstream are re-verified against the actual
pinned commit.

- Clone iroh, pin latest release tag. Build `iroh-relay` with `--features
  server`. Run its test suite; record pass state and runtime as baseline.

- Stand up a local relay with each existing access mode (open, tokens, HTTP
  hook) and connect two iroh endpoints through it. Script this; it becomes
  the skeleton of our integration harness.

- Verify each bullet in section 2 against source. Correct this plan where
  reality disagrees. Reality wins.

- Write ADR-0001 (embed vs fork decision, pinned versions, discovered seams).

- Scaffold `croft-relay` repo: workspace, CI, `docs/adr/`, integration test
  harness crate.

Acceptance: relay runs locally in all three existing modes; harness test
proves endpoint A reaches endpoint B via our relay; ADR-0001 merged.

### Phase 1: Admission service (app-side, no relay changes)

Story: before touching the relay, make its existing HTTP hook do our bidding.
This ships value with zero fork risk and becomes the enrollment authority
Phase 2 needs.

- New service `croft-admit` (axum): implements the relay's HTTP access-check
  contract. Reads `X-Iroh-Endpoint-Id`, checks membership in a registry,
  returns `200`/`true` or deny.

- Registry v1: a table of enrolled EndpointIds with bound DID. Populate via
  an enrollment endpoint that (a) verifies control of the DID (fetch the PDS
  record `ing.croft.iroh.endpoint` for that DID and check it names this
  EndpointId), and (b) records the binding.

- TDD: contract tests for the access-check endpoint (allow, deny, malformed
  header, upstream-timeout behavior = deny closed); enrollment tests with a
  mocked PDS.

- Integration: relay configured with `access.http.url` pointing at
  `croft-admit`; harness proves unenrolled endpoints are refused attach and
  enrolled ones connect.

Acceptance: mode 1 (registered-only) works end to end using stock upstream
relay binary. ADR-0002: registry semantics, deny-closed rationale.

### Phase 2: Signed per-endpoint tokens (the cryptographic door)

Story: replace "relay asks a database per connection" with "relay verifies a
signature and holds no state." Token embeds the EndpointId; the relay checks
(1) signature by our enrollment key, (2) not expired, (3) embedded id equals
the cryptographically authenticated connecting id. Stolen token without the
key is useless; key without token gets nothing.

- Extend `croft-admit` to mint tokens (per section 4 default format) after
  enrollment verification. Claims include `tier`.

- Relay side, Option A shape: our embedded server validates the token during
  attach before handing the connection to the relay service. Option B shape:
  a new access mode `access.signed_token` with a configured verification
  public key, implemented as a small patch mirroring the existing shared-token
  mode's structure.

- TDD matrix: valid token/matching id -> admit; valid token/mismatched id ->
  deny; expired -> deny; wrong issuer key -> deny; malformed -> deny; clock
  skew tolerance bounds; replay of a valid token by the legitimate key within
  expiry -> admit (documented as fine, tokens are capabilities not nonces).

- Client side: thin wrapper for our app that fetches a token from
  `croft-admit` and attaches it (upstream client already supports sending a
  token via `RelayConfig::with_auth_token`; verify the header/query mechanism
  carries arbitrary token strings of our length).

Acceptance: relay admits solely on signature verification, no network call in
the attach path; Phase 1 HTTP-hook mode remains available as a fallback
toggle. ADR-0003: token format, claim schema, expiry policy, revocation =
short expiry + refusal to re-issue.

Upstream candidate: the `signed_token` access mode (policy-free: "verify
bearer tokens against a configured public key, require sub == endpoint id").
Keep the patch free of any atproto or tier concepts.

### Phase 3: Tiered per-connection rate buckets

Story: the tier claim must become enforcement. Coordination tier gets a bucket
sized for disco chatter; broker tier gets a generous or absent bucket. One
credential, one primitive, three product dials.

- At admission, map `tier` claim -> `ClientRateLimit` (or `Bucket`) for that
  connection. Option A: assign the bucket in our embedding layer per
  connection. Option B: patch to allow the access-control result to carry an
  optional per-connection rate limit override, defaulting to the global
  config value (upstream-shaped: generic, useful beyond us).

- Calibration step (measured, not guessed): instrument the harness to measure
  bytes used by a successful holepunch coordination exchange, then set the
  coordination bucket with comfortable headroom above that and far below
  usable media bitrate. Record the measurement method and numbers in the ADR.

- TDD: unit tests for claim->bucket mapping; integration tests proving (a)
  coordination-tier pair can still achieve a direct connection via the relay,
  (b) coordination-tier relayed throughput is capped at the configured rate,
  (c) broker-tier relayed throughput is not, (d) live limit changes apply if
  we rely on the dynamic mechanism.

- App-facing behavior note (not relay code): failed-holepunch on
  coordination tier must surface as a tier limitation, not a bug.

Acceptance: three modes demonstrable in the harness with one config toggle
each. ADR-0004: bucket sizes, calibration data, why content-based splitting
was rejected (encrypted frames are indistinguishable by design and must stay
that way).

Upstream candidate: per-connection rate-limit override at admission time.

### Phase 4: Hardening and operations

Story: a relay that gates strangers will meet strangers.

- Deny-path behavior: cheap rejects, no logging amplification, per-source
  attach-attempt rate limiting ahead of token verification cost.

- Metrics: admissions by outcome and tier, active connections by tier,
  bytes relayed by tier, bucket saturation events. Use upstream's metrics
  feature if present; otherwise prometheus in the embedding layer.

- Config story: one TOML for croft-relay covering mode toggles; document
  every knob with its "why" inline.

- Fuzz the token parser (cargo-fuzz, time-boxed) since it faces the network.

Acceptance: load-ish test (harness spawning N clients) shows stable memory
and correct per-tier behavior; dashboards render; ADR-0005 threat notes.

### Phase 5: Contribution packaging

Story: each capability leaves home as one PR with one why.

- Rebase patch series (if any) against upstream main; one feature per PR:
  (1) signed-token access mode, (2) per-connection rate-limit override,
  plus any "expose seam X" PRs Option A required.

- Match upstream conventions detected in Phase 0 (error crate, test style,
  commit format, changelog). Strip croft-specific naming, mutation-testing
  configs, and anything opinionated; those stay in our repo.

- Each PR description: problem, design, alternatives considered (link the
  reasoning, not the ADR file), test evidence.

- File issues first for the two features, referencing existing related
  issues/discussions if found, and offer the PR. Ask before opening PRs.

---

## 6. Testing and mutation-testing policy

- TDD throughout: red, green, refactor. Integration harness from Phase 0 is
  the backbone; every phase adds scenarios to it rather than bespoke rigs.

- Mutation testing with `cargo-mutants`, scoped per phase to the new/changed
  modules (`cargo mutants -p <crate> --in-diff` against the phase branch
  where supported, otherwise path filters). Full-crate runs are a nightly CI
  job, not a per-commit gate.

- Policy: no known-surviving mutant in admission or token-verification code
  paths without a written justification. Elsewhere, record survivors in the
  phase ADR with disposition (kill / equivalent / accepted risk).

- Upstream-destined patches carry upstream-conventional tests only. Mutation
  configs and our harness stay in croft-relay.

---

## 7. Open questions for the human

Defaults are chosen so the run does not block, but flag these at Phase 0:

1. Confirm token default (JWT/EdDSA) or specify alternative.

2. Confirm repo shape (new `croft-relay` repo, iroh pinned by tag).

3. Coordination-tier product stance: hard-fail relayed media (recommended,
   honest) vs allow a trickle. Plan assumes hard cap.

4. Any interest in shipping Phase 1 to relay.croft.ing before Phase 2 lands,
   or hold deployment until tokens exist.

5. Whether metrics may include EndpointId-level labels (cardinality and
   privacy trade) or tier-level aggregates only. Plan assumes tier-level.
