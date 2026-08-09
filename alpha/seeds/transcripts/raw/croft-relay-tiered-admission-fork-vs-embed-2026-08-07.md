# croft-relay — tiered admission, fork-vs-embed, and member-sponsored tunneling (Claude Code, 2026-08-07)

- **Source:** a live Claude Code session in `CroftC/` on 2026-08-07 (working dir
  `/Users/cpettet/git/chasemp/CroftC`), plus material the owner pasted into its opening turn.
- **Preservation status:** **preserved-verbatim (in-session dialogue)** for Parts 1–7 — the owner's
  turns are byte-exact and the assistant's turns are reproduced in full for all design content, with
  only tool-call render chrome ("Ran a command", file-read echoes, tool result blocks) reduced to
  bracketed notes naming what was read. Part 0 is **preserved-condensed (cleaned-paste,
  content-faithful — §4)**: it reproduces material the owner pasted from *other* sessions, for which
  no canonical export exists here.
- **Credentials:** none present; none redacted.
- **Why this file matters beyond the dialogue:** Part 0 §B is the **original croft-relay build plan**,
  which had never been filed. `experiments/croft-relay/README.md:16` points at "the build plan (in the
  run summary and the ADRs)" as though it were captured; it was not. This transcript is now its only
  copy in the corpus.

---

## Part 0 — pasted context (from prior sessions, cleaned-paste)

### A. Prior Claude Code session: RUN-CROFT-RELAY-03, the local live leg

*(Owner pasted this as the opening of the session. Condensed from the paste; tool-call chrome
stripped; technical claims retained verbatim.)*

The run's own narration:

> This is the live two-endpoint leg, localhost version. The right tool is iroh-relay's own server +
> relay clients (which forces traffic through our admission gate and lets me measure relayed bytes
> precisely — more appropriate for the coordination-tier calibration than a localhost holepunch that
> would instantly go direct).

`runtime_auth.rs` in iroh-relay was identified as the crib: `Server::spawn` on `LOCALHOST:0` with our
`AccessControl`, relay `Client`s via `ClientBuilder`, and the client `auth_token()` riding as
`Authorization: Bearer` into our verifier. Endpoint→endpoint is
`ClientToRelayMsg::Datagrams { dst_endpoint_id, datagrams }`.

A `#![cfg(feature = "server")]` at the top of the new test file gated the whole file out (that feature
is iroh-relay's, not ours — we always enable it); removing it fixed the run.

Three live-relay tests pass against a real relay on localhost:

1. `TokenAccess` admits a valid croft-admit JWT, denies a bogus token, and denies a token replayed
   from a different endpoint — **anti-replay proven over the real relay handshake**, not a synthetic
   request.
2. A's datagram reaches B through our gated relay, payload intact, origin id correct.
3. A relayed contact round-trip measured (3 B each way) and validated against the coordination bucket.

Final state of that run: 46 tests green workspace-wide (36 croft-admit + 7 embed-unit + 3 live-relay),
clippy + fmt clean, croft-admit mutation gate still 0 survivors. Pushed as `52c4ef4`.

The honest caveat recorded at the time:

> The live figure is a localhost relay-client contact round-trip, not the full holepunch disco total —
> that needs two iroh magicsock endpoints doing NAT traversal on separate networks. So I kept the
> coordination constant as a SPEC-DELTA placeholder and recorded the harness + datapoint + what remains
> in ADR-0004. That real-NAT calibration is now the only Phase-3 item still open.

Owner then said: **"yes add this to open threads to be tested in production then pr and merge."**

That produced thread **T62** in `beta/OPEN-THREADS.md` (Croft / Layer 6), PR **#40**, and one
unplanned correctness fix on the way: the CI smoke matrix was a hardcoded list of three crates
(`croft-chat`, `local_storage_projection`, `bip39-recovery-roundtrip`), so `croft-relay` would have
merged **un-gated** — exactly the "green in-session, broken on fresh clone" failure the workflow
exists to catch. `croft-relay` was added to `.github/workflows/smoke.yml`.

CI outcome, recorded honestly at merge time:

- `test + clippy (croft-relay)` **passed** on a fresh clone (46 tests incl. the localhost relay test,
  clippy clean). `croft-chat` and `bip39` also green.
- The `build + broken-ref` gate was **red — but pre-existing on main**, failing on `thinking-app-*`
  companion-doc §-references (§63/§66/§62) the PR did not touch; `main` HEAD and merges #35, #37, #39
  were all red on it. Merged past it, but flagged rather than buried.

Merged as commit `07dfb73`.

### B. The original croft-relay build plan (never previously filed — captured here in full)

> # croft-relay: iroh-relay extensions for atproto-gated calling
>
> Implementation plan for Claude Code. Phased, TDD-first, each phase sized as one reviewable,
> upstreamable unit with its own "story" (the one why).
>
> ## 1. Context and product story
>
> We are building a calling system where:
>
> - Identity lives in atproto. A custom lexicon record (`ing.croft.iroh.endpoint`, rkey `self`) in a
>   user's PDS repo binds their DID to an iroh EndpointId and a home relay URL
>   (`https://relay.croft.ing`).
> - Transport is iroh. Callers resolve handle → DID → PDS record → EndpointId, then dial. The relay
>   carries first contact and coordination; holepunching upgrades to direct QUIC when possible.
> - The relay is the enforcement point for admission, and the app layer is the enforcement point for
>   pairwise call policy (mutuals-only etc.). The relay never learns call content and never holds
>   social-graph state.
>
> Product modes the relay must support, as discrete dials:
>
> 1. **Registered-only reception**: only endpoints holding a valid credential issued by our enrollment
>    service may attach (camp or dial in).
> 2. **Coordination-only tier**: admitted, but rate-limited so hard that holepunch coordination works
>    while sustained relayed media is starved. (Content-based splitting is impossible by design: the
>    relay cannot distinguish disco frames from app data. Volume is the only honest proxy.)
> 3. **Full-broker tier**: generous or absent limits. Tier is a claim in the credential, chosen at
>    admission, enforced by a per-connection rate bucket.
>
> Explicitly out of scope for v1 (designed, deferred): pairwise (src, dst) policy in the forwarding
> path; priority scheduling between tiers; firehose-driven cache invalidation. Do not build these.
> Leave seams.
>
> ## 2. Grounding: current upstream state (verify at Phase 0, do not trust this doc)
>
> As of iroh ~1.0.x (verify exact versions in-repo):
>
> - `iroh-relay` supports access control modes in TOML `access` field: open, shared bearer tokens
>   (`Authorization: Bearer` header or `?token=` query param, no revocation without restart), and an
>   HTTP hook that POSTs per incoming connection with header `X-Iroh-Endpoint-Id` (hex endpoint id),
>   granting access only on `200` + body `true`.
> - Per-client receive rate limits exist in config: `[limits.client.rx] bytes_per_second`,
>   `max_burst_bytes`.
> - 1.0.2 added `RelayService::set_client_rate_limit` for live updates, but the limit appears
>   service-wide (one value picked up by all connections), not per-connection.
> - The `Bucket` rate-limit primitive was made public for embedders that mount the relay protocol
>   under their own HTTP server (axum etc.).
> - The relay authenticates the connecting endpoint's key during attach, so the endpoint id in the
>   access check is cryptographically bound, not asserted.
> - n0's managed relays use short-lived signed access tokens scoped to the endpoint's identity. That
>   pattern is not in the self-host OSS path. It is our template for Phase 2.
>
> Primary sources to re-verify (fetch fresh, pin exact commit in ADR-0001):
> `github.com/n0-computer/iroh/tree/main/iroh-relay` (README, config docs);
> `iroh-relay/src/main.rs`; the releases page (search: access control, rate limit);
> `docs.iroh.computer/concepts/relays` and `/concepts/holepunching`.
>
> ## 3. Architectural decision to make first: embed vs fork
>
> Two viable shapes. Decide in Phase 0 and record as ADR-0001.
>
> **Option A: embed (preferred if feasible).** New binary crate `croft-relay` that depends on
> `iroh-relay` as a library, mounts the relay protocol under our own axum server, and implements
> admission + per-connection buckets in our code using the public `Bucket` primitive. Upstream diff:
> zero or near-zero. Upstreams become "please expose X" PRs, which are the easiest kind to land.
>
> **Option B: patch fork.** Fork `n0-computer/iroh`, carry a short patch series on `iroh-relay`. Each
> phase is one series entry. Higher maintenance cost, cleaner if the library seams turn out not to
> exist.
>
> Decision rule: attempt Option A first. Fall back to B only for the specific capability the library
> cannot express, and keep that patch minimal and upstream-shaped. Hybrid is acceptable (embed + one
> small patch).
>
> ## 4. What Claude Code needs
>
> Environment: Rust stable matching the repo's MSRV; `cargo`, `clippy`, `rustfmt`; `cargo-mutants`;
> network access to clone `n0-computer/iroh` and fetch docs; ability to run multi-process integration
> tests locally (relay on localhost ports; upstream has integration tests for embedding the relay to
> crib from).
>
> Inputs from the human (defaults provided so this remains single-shot runnable):
>
> - **Token format** for Phase 2. Default: JWT with EdDSA (ed25519), claims: `sub` = hex EndpointId,
>   `tier` = `coordination` | `broker`, `exp` short (minutes to an hour), `iss` = enrollment service.
>   Rationale: boring, library-supported, auditable. Alternative if dependency-averse: raw ed25519
>   signature over a canonical CBOR struct. Pick JWT unless told otherwise.
> - **Repo home**: default new repo `croft-relay` with iroh as a git dependency pinned to a release tag.
> - **CI**: default GitHub Actions, fmt + clippy + test + (nightly job) mutants.
>
> Conventions to detect in Phase 0 and then follow (do not assume): upstream test style and
> error-handling crate (they use `n0_error` and friends; match it in any patch destined upstream);
> commit message and changelog conventions (they appear to use conventional commits, e.g.
> `feat(iroh-relay): ...`; verify against recent history).
>
> ## 5. Phases
>
> Every phase: write failing tests first, implement to green, refactor, then run mutation testing on
> the new module and either kill surviving mutants with tests or document why they are
> equivalent/acceptable in the phase's ADR. Every phase ends with: ADR written, README section updated,
> branch pushed, self-review pass for "would n0 merge this shape."
>
> ### Phase 0: Baseline and reconnaissance
>
> Story: we change nothing until we can build, run, and test what exists, and until this plan's claims
> about upstream are re-verified against the actual pinned commit.
>
> - Clone iroh, pin latest release tag. Build `iroh-relay` with `--features server`. Run its test
>   suite; record pass state and runtime as baseline.
> - Stand up a local relay with each existing access mode (open, tokens, HTTP hook) and connect two
>   iroh endpoints through it. Script this; it becomes the skeleton of our integration harness.
> - Verify each bullet in section 2 against source. Correct this plan where reality disagrees. Reality
>   wins.
> - Write ADR-0001 (embed vs fork decision, pinned versions, discovered seams).
> - Scaffold `croft-relay` repo: workspace, CI, `docs/adr/`, integration test harness crate.
>
> Acceptance: relay runs locally in all three existing modes; harness test proves endpoint A reaches
> endpoint B via our relay; ADR-0001 merged.
>
> ### Phase 1: Admission service (app-side, no relay changes)
>
> Story: before touching the relay, make its existing HTTP hook do our bidding. This ships value with
> zero fork risk and becomes the enrollment authority Phase 2 needs.
>
> - New service `croft-admit` (axum): implements the relay's HTTP access-check contract. Reads
>   `X-Iroh-Endpoint-Id`, checks membership in a registry, returns `200`/`true` or deny.
> - Registry v1: a table of enrolled EndpointIds with bound DID. Populate via an enrollment endpoint
>   that (a) verifies control of the DID (fetch the PDS record `ing.croft.iroh.endpoint` for that DID
>   and check it names this EndpointId), and (b) records the binding.
> - TDD: contract tests for the access-check endpoint (allow, deny, malformed header, upstream-timeout
>   behavior = deny closed); enrollment tests with a mocked PDS.
> - Integration: relay configured with `access.http.url` pointing at `croft-admit`; harness proves
>   unenrolled endpoints are refused attach and enrolled ones connect.
>
> Acceptance: mode 1 (registered-only) works end to end using stock upstream relay binary. ADR-0002:
> registry semantics, deny-closed rationale.
>
> ### Phase 2: Signed per-endpoint tokens (the cryptographic door)
>
> Story: replace "relay asks a database per connection" with "relay verifies a signature and holds no
> state." Token embeds the EndpointId; the relay checks (1) signature by our enrollment key, (2) not
> expired, (3) embedded id equals the cryptographically authenticated connecting id. Stolen token
> without the key is useless; key without token gets nothing.
>
> - Extend `croft-admit` to mint tokens (per section 4 default format) after enrollment verification.
>   Claims include `tier`.
> - Relay side, Option A shape: our embedded server validates the token during attach before handing
>   the connection to the relay service. Option B shape: a new access mode `access.signed_token` with a
>   configured verification public key, implemented as a small patch mirroring the existing
>   shared-token mode's structure.
> - TDD matrix: valid token/matching id → admit; valid token/mismatched id → deny; expired → deny;
>   wrong issuer key → deny; malformed → deny; clock skew tolerance bounds; replay of a valid token by
>   the legitimate key within expiry → admit (documented as fine, tokens are capabilities not nonces).
> - Client side: thin wrapper for our app that fetches a token from `croft-admit` and attaches it
>   (upstream client already supports sending a token via `RelayConfig::with_auth_token`; verify the
>   header/query mechanism carries arbitrary token strings of our length).
>
> Acceptance: relay admits solely on signature verification, no network call in the attach path; Phase
> 1 HTTP-hook mode remains available as a fallback toggle. ADR-0003: token format, claim schema, expiry
> policy, revocation = short expiry + refusal to re-issue.
>
> Upstream candidate: the `signed_token` access mode (policy-free: "verify bearer tokens against a
> configured public key, require sub == endpoint id"). Keep the patch free of any atproto or tier
> concepts.
>
> ### Phase 3: Tiered per-connection rate buckets
>
> Story: the tier claim must become enforcement. Coordination tier gets a bucket sized for disco
> chatter; broker tier gets a generous or absent bucket. One credential, one primitive, three product
> dials.
>
> - At admission, map `tier` claim → `ClientRateLimit` (or `Bucket`) for that connection. Option A:
>   assign the bucket in our embedding layer per connection. Option B: patch to allow the access-control
>   result to carry an optional per-connection rate limit override, defaulting to the global config
>   value (upstream-shaped: generic, useful beyond us).
> - Calibration step (measured, not guessed): instrument the harness to measure bytes used by a
>   successful holepunch coordination exchange, then set the coordination bucket with comfortable
>   headroom above that and far below usable media bitrate. Record the measurement method and numbers
>   in the ADR.
> - TDD: unit tests for claim→bucket mapping; integration tests proving (a) coordination-tier pair can
>   still achieve a direct connection via the relay, (b) coordination-tier relayed throughput is capped
>   at the configured rate, (c) broker-tier relayed throughput is not, (d) live limit changes apply if
>   we rely on the dynamic mechanism.
> - App-facing behavior note (not relay code): failed-holepunch on coordination tier must surface as a
>   tier limitation, not a bug.
>
> Acceptance: three modes demonstrable in the harness with one config toggle each. ADR-0004: bucket
> sizes, calibration data, why content-based splitting was rejected (encrypted frames are
> indistinguishable by design and must stay that way).
>
> Upstream candidate: per-connection rate-limit override at admission time.
>
> ### Phase 4: Hardening and operations
>
> Story: a relay that gates strangers will meet strangers.
>
> - Deny-path behavior: cheap rejects, no logging amplification, per-source attach-attempt rate
>   limiting ahead of token verification cost.
> - Metrics: admissions by outcome and tier, active connections by tier, bytes relayed by tier, bucket
>   saturation events. Use upstream's metrics feature if present; otherwise prometheus in the embedding
>   layer.
> - Config story: one TOML for croft-relay covering mode toggles; document every knob with its "why"
>   inline.
> - Fuzz the token parser (cargo-fuzz, time-boxed) since it faces the network.
>
> Acceptance: load-ish test (harness spawning N clients) shows stable memory and correct per-tier
> behavior; dashboards render; ADR-0005 threat notes.
>
> ### Phase 5: Contribution packaging
>
> Story: each capability leaves home as one PR with one why.
>
> - Rebase patch series (if any) against upstream main; one feature per PR: (1) signed-token access
>   mode, (2) per-connection rate-limit override, plus any "expose seam X" PRs Option A required.
> - Match upstream conventions detected in Phase 0 (error crate, test style, commit format, changelog).
>   Strip croft-specific naming, mutation-testing configs, and anything opinionated; those stay in our
>   repo.
> - Each PR description: problem, design, alternatives considered (link the reasoning, not the ADR
>   file), test evidence.
> - File issues first for the two features, referencing existing related issues/discussions if found,
>   and offer the PR. Ask before opening PRs.
>
> ## 6. Testing and mutation-testing policy
>
> - TDD throughout: red, green, refactor. Integration harness from Phase 0 is the backbone; every phase
>   adds scenarios to it rather than bespoke rigs.
> - Mutation testing with `cargo-mutants`, scoped per phase to the new/changed modules
>   (`cargo mutants -p <crate> --in-diff` against the phase branch where supported, otherwise path
>   filters). Full-crate runs are a nightly CI job, not a per-commit gate.
> - Policy: no known-surviving mutant in admission or token-verification code paths without a written
>   justification. Elsewhere, record survivors in the phase ADR with disposition (kill / equivalent /
>   accepted risk).
> - Upstream-destined patches carry upstream-conventional tests only. Mutation configs and our harness
>   stay in croft-relay.
>
> ## 7. Open questions for the human
>
> 1. Confirm token default (JWT/EdDSA) or specify alternative.
> 2. Confirm repo shape (new `croft-relay` repo, iroh pinned by tag).
> 3. Coordination-tier product stance: hard-fail relayed media (recommended, honest) vs allow a
>    trickle. Plan assumes hard cap.
> 4. Any interest in shipping Phase 1 to relay.croft.ing before Phase 2 lands, or hold deployment until
>    tokens exist.
> 5. Whether metrics may include EndpointId-level labels (cardinality and privacy trade) or tier-level
>    aggregates only. Plan assumes tier-level.

### C. The upstream ask that generated the plan, and the two follow-on deliverables

The plan above came from an earlier claude.ai exchange opened by the owner with:

> I want to use a PDS lexicon for an Iroh account lookup so I can have a link for users that choose to
> publish their cryptographic association so that one iroh user could call another. I have a relay of
> my own at relay.croft.ing. Can you talk me through this setup in detail and what's possible.

Two later deliverables from that same thread are referenced but **not** reproduced here (they are
separate artifacts, and reproducing them would triplicate):

1. **A lookup page** ("Croft exchange", HTML artifact) — handle → DID via the public AppView XRPC, DID
   → PDS via plc.directory (did:web handled too), then `com.atproto.repo.getRecord` for
   `ing.croft.iroh.endpoint/self`; resolution rendered as a live ledger trace; a Connect button
   emitting `croftcall://call?endpoint=…&relay=…&handle=…&did=…`. Missing record renders a "not
   listed" state rather than an error.
2. **An Android scaffold** ("croftcall android", ZIP) — official Kotlin bindings from Maven Central;
   persistent identity (secret key in `EncryptedSharedPreferences` so the published EndpointId stays
   stable); `singleTask` activity with the `croftcall://` intent filter feeding `onNewIntent`; callee
   card populated from the link; dial-by-EndpointId; a `croft-call/0` hello exchange as the v0 "call".
   **Honesty ledger recorded at the time:** bind/identity/lifecycle calls verified against retrieved
   Kotlin docs, but accept/connect/stream method names marked `VERIFY` in `CallPeer.kt` — to be checked
   against the Dokka reference and n0's `hello-iroh-ffi` Android sample before first compile. Uses
   `presetN0()` relays for day one with the relay.croft.ing + token swap isolated to one function;
   callable only while foregrounded (background reachability needs a foreground service + push-to-wake,
   its own phase).

A third deliverable — build instructions for a monorepo deploying the page to GitHub Pages and the app
via CI artifact, with `docs/contract.md` as the single shared interface — is likewise referenced, not
reproduced.

---

## Part 1 — the opening ask

**Owner:**

> we are building a storage server in ciss that allows us to meter and grant based on cryptographic
> identity, we had also worked on some iroh relay extensions that woudl allow us to gate what
> functionality is available to what users, I'm going to paste teh claude code transcript
>
> […Part 0 above…]
>
> I think what we want ot do is create our own fork of hte iroh-rely and maintain our chagnes on a
> branch so we can pull in the upstream as we need to, but I would greatly, greaetly prefer to maek as
> little and as discrete changes as possible so maintenance is sane
>
> can you dig through all this and look at what the experiments committed under discovery for
> discusion?

**Assistant** *(after reading the committed experiment: crate inventory, git history, ADR-0001,
`croft-relay-embed/src/lib.rs`, ADR-0004, `tier.rs`, `evidence/live-relay.txt`, T62,
`EXPERIMENT-BACKLOG.md` §6j, `croft-stack` relay role, ADR-0005, and then the pinned
`iroh-relay-1.0.3` source in the cargo registry)*:

**What's committed.** `discovery/alpha/experiments/croft-relay/` — three commits (`16a6fc3`,
`0899a73`, `973825d`), ~1,950 lines Rust, two crates:

| Crate | iroh dep | What it holds |
|---|---|---|
| `croft-admit` | **none** | endpoint_id, DID resolution, PDS record fetch, enrollment, registry, `/access` HTTP hook, JWT/EdDSA token mint+verify, `tier → RateBucket` mapping. 36 tests, `cargo mutants` 0 survivors. |
| `croft-relay-embed` | `iroh-relay 1.0.3` | `TokenAccess` / `RegistryAccess` implementing `AccessControl::on_connect`, plus the localhost live-relay harness. 10 tests. |

Plus five ADRs, `OPEN-QUESTIONS.md` (five unanswered owner calls), `evidence/`, thread T62, backlog
§6j, and PR #40 merged. The deliberate split is what matters: **all admission logic is
relay-agnostic**; only ~155 lines touch iroh.

**What the experiments concluded.** ADR-0001 chose embed and was right for Phases 1–2. `AccessControl`
is public, and the endpoint id handed to `on_connect` is challenge/response-authenticated before the
hook runs — that's what makes the anti-replay gate real rather than asserted. Registered-only and
signed-token admission need **zero** upstream diff.

Exactly one capability doesn't fit: **per-connection rate limits**. Re-verified against 1.0.3 source
rather than trusting the ADRs:

```
RelayService::accept(io, ...)          src/server/http_server.rs:845
  │
  ├─ 856  RateLimited::from_watcher(io, self.rate_limit.subscribe())
  │         ▲ ONE service-wide watch::Sender, shared by every connection
  │         ▲ installed on the RAW stream, BEFORE the handshake
  │
  ├─ 868  handshake::serverside(...)     → authenticated client_key
  ├─ 876  authorize_with(&request, &self.access, ...)   ← our on_connect
  │         └─ returns Access::{Allow | Deny{reason}}   ← carries no rate
  └─ 894  clients.register(...)
```

Three consequences: `Access::Allow` is a unit variant, so the proven tier cannot ride back out of the
hook; the rate limiter is bound **before** authentication, so it structurally cannot depend on who
connected; and — **correcting ADR-0004** — its stated fallback ("wrap the admitted connection in a
`Bucket`" in the embedding layer) **does not work**, because `RateLimited` is `pub(crate)` and an
embedder never touches the post-handshake stream. `Bucket` is `pub`, but there is nothing to wrap it
around at the point where the endpoint id is known. Tier enforcement is not achievable by embedding at
any effort level.

**Three shapes, cheapest first.**

1. **Two RelayServices, no fork at all.** `RelayService::new` is public and takes the rate limit at
   construction. Run one instance per tier behind our own axum server. Zero patch forever. Cost (as
   stated at this point in the dialogue, later corrected): tier becomes visible in the published relay
   URL, and it "splits the relay's client map so two endpoints on different tiers can't reach each
   other."
2. **The minimal patch branch (recommended at this point).** Two hunks: `Access::Allow` gains
   `rate_limit: Option<ClientRateLimit>`; `accept()` seeds a per-connection `watch::Sender` from the
   service value and `send_replace()`s after authorize returns. ~20 lines across 2 files. Everything of
   ours stays outside the fork, consumed via `[patch.crates-io]`.
3. **Carry more in the fork.** Don't.

**The deployment consequence nobody has costed.** `croft-stack/ansible/group_vars/all.yml:72` pins
`relay_version: "1.0.0"` and fetches n0's prebuilt musl tarball by checksum. **Any** of these options
ends that — including pure embed, since embedding means shipping our own binary. "Build our own
artifact" is already the price of admission and is not an argument between fork and embed. Also: the
deployed relay is three releases behind (1.0.0 vs 1.0.3).

**Decisions wanted:** Option 2 vs 1; `Allow { rate_limit }` vs a new `AllowLimited` variant; the five
§7 calls (Q4 — ship Phase 1 registered-only now — could deliver value with no fork at all); and T62
gate 1, since the coordination bucket is still a `SPEC-DELTA` placeholder and the patch exists to
enforce it.

---

## Part 2 — cross-relay, and the reverse-proxy idea

**Owner:**

> how does iroh do cross relay calling or notice or does it?  bc splitting the relay services is really
> the best plan so far I think, we could also have a small map reverse proxy that routes to a relay
> baed on destigation and only known endpointids are routed to the fully tunneling proxy

**Assistant** *(after reading `iroh-1.0.3` `src/socket/transports/relay.rs` and `relay/actor.rs`, plus
`iroh-relay-1.0.3` `src/http.rs`, `src/client.rs`, and `src/protos/handshake.rs`)*:

**Relays do not federate.** There is no relay-to-relay path. From `relay/actor.rs:1-25`:

> The `RelayActor` manages all connections to relay servers. It starts a new `ActiveRelayActor` for
> each relay server needed. The `ActiveRelayActor` will exit when unused — unless it is for the home
> relay, this one never exits.

And the send path takes the destination's relay explicitly (`relay.rs:272`):
`poll_send(dest_url: RelayUrl, dest_endpoint: EndpointId, ...)`.

So cross-relay works by **the sender joining the receiver's relay as an additional client**:

```
A (home: relay-1)                          B (home: relay-2)
  │                                          │
  │ ActiveRelayActor(relay-1) ── camps ───────┼──▶ carries B→A
  │ ActiveRelayActor(relay-2) ── transient ───┘    carries A→B
                                                  (exits after 60s idle)
```

Each direction traverses **the receiver's** home relay.

**Correction issued:** the earlier objection — that splitting would split the client map so
cross-tier endpoints couldn't reach each other — was **wrong**. A client holds connections to several
relays at once, so N instances federate implicitly through the clients. Splitting is viable in a way
the previous turn said it wasn't.

**But the split inverts what "tier" means.** `ClientRateLimit` is `client_rx` — bytes the relay accepts
*from* a client, i.e. it limits sending. Combined with home-relay routing: A→B is metered by B's
relay's limit; B→A by A's relay's limit. So tier is not "what I may do" but **"the rate at which others
may send to me."** Bidirectional media therefore requires **both** parties on broker relays (coherent,
arguably the honest product), and the tier of the *sender* is unenforceable through instance-global
limits. Plus a freeloading hole: nothing stops a coordination user publishing the broker relay URL in
their own PDS record.

**The reverse proxy: destination routing is not possible, source routing is.** `dst_endpoint_id` lives
inside `ClientToRelayMsg::Datagrams`, per-datagram, inside the WebSocket after upgrade — and a single
client connection multiplexes datagrams to *every* peer sharing that relay. There is no
connection-level destination to route on. A proxy that routed by destination would have to terminate
the relay protocol and demux frames; that is a second relay, not a proxy.

| Field | Visible pre-upgrade | Trustworthy at proxy |
|---|---|---|
| `Authorization: Bearer <token>` / `?token=` | yes | signature verifiable if the proxy holds the pubkey |
| `x-iroh-relay-client-auth-v1` (source endpoint id) | usually | **no** — bound to the TLS exporter, unverifiable after termination, absent on the challenge fallback |
| path (`/relay`) | yes, but constant | n/a |
| destination endpoint id | **no** | n/a |

So route on **our own token**. And TLS termination at the proxy does not break iroh auth —
`handshake.rs:447`: "Verification not succeeding is part of normal operation: The TLS exporter isn't
required to match. We'll fall back to verification that takes another round trip." One RTT on first
connect, nothing else.

**Proposed shape:** one public hostname, an axum switch reading the Bearer token and forwarding by tier
claim, two `RelayService` instances behind it with different global limits and different
`AccessControl`s. All public API — no fork, no patch, no rebase burden. The Option-2 patch becomes
unnecessary unless per-connection granularity *within* a tier is later needed.

---

## Part 3 — the product rule

**Owner:**

> my desire was basically, members can use full tunnel whether the reciever or transmitter, and
> non-members can only do coordination adn handoff

**Assistant:** That rule is pairwise, and every lever iroh exposes is per-sender. Both obvious designs
give exactly half:

```
                        M→N              N→M           verdict
split by home relay   N's relay        M's relay
 (instance-global)    = coord ✗        = broker ✓      member receiving works,
                                                       member transmitting throttled

per-connection patch  M is member      N is not
 (rate by sender)     = broker ✓       = coord ✗       member transmitting works,
                                                       member receiving throttled
```

Neither expresses "either end is a member," because at the moment either lever fires only the
connecting sender's identity is known. Half a pipe is worth nothing — a call where audio flows one way
is broken, not degraded.

**Closing it inside the relay would cost the property being protected.** Deciding by pair means
evaluating `member(src) || member(dst)` per datagram in the forwarding path — a deep fork (the build
plan already deferred this as out of scope for v1) that also puts social-graph state in the relay,
contradicting §1.

**Move the pairwise decision to the issuer.** `croft-admit` already talks to both parties at call setup
and knows both DIDs:

```
  N (non-member)                croft-admit                M (member)
      │                              │                          │
      │──── "calling M" ────────────▶│◀──────── "calling N" ────│
      │                              │  either party a member?
      │                              │        → yes
      │◀── broker token (sub=N) ─────│──── broker token (sub=M) ▶│
      │                              │      exp: call-length
      ├─ insert_relay(relay.croft.ing, cfg.with_auth_token(t)) ──┤
      └────────▶  switch reads tier=broker  ◀────────────────────┘
                  both land in the broker instance,
                  same client map, full tunnel both ways
```

Three things verified to make this real rather than hand-wave:

- **Tokens can be swapped live.** `Endpoint::insert_relay(url, Arc<RelayConfig>)` is public and async
  (`iroh-1.0.3` `endpoint.rs:984`), and `RelayConfig::with_auth_token` sets the bearer. A client
  re-tiers mid-session without rebinding — the EndpointId stays stable, which matters because it's
  what's published in the PDS record.
- **The published record never changes.** Single hostname behind the switch means which instance a
  client lands in is decided by the token it holds at that moment. This makes the single-hostname
  choice load-bearing rather than a privacy nicety.
- **No fork.** All public API on 1.0.3.

**What shrinks the problem:** rate limits only bite on *relayed* traffic. A successful holepunch is
direct QUIC and unmetered. This machinery governs only calls where holepunch fails — so that rate
should be measured (T62 gate 1, and the E77 relay-load work) before much is built on it.

**Abuse surface, stated plainly:** a sponsored token is a capability, not a nonce (ADR-0003 says so
deliberately). Bound to the endpoint id and short-lived, blast radius is "one non-member at broker
rates until expiry." Mitigations: short `exp`, cap on sponsorships per member per window.

**The one product call left:** does a non-member calling a member get sponsored automatically? Read
literally the rule says yes, but that makes free broker relay obtainable by anyone willing to dial a
member.

---

## Part 4 — allowlists and member discretion

**Owner:**

> I think it's more about a member being part of a tunneling exchange, whether they are transmit or
> receive, and a membrer is most likely goign to allow list those who can call them, tho open would be
> possible, spam issues to your point, it rally has to be there discretion

**Assistant** *(after grepping the corpus for prior art)*: `ROUND-2026-06-17-media-meer-conformance.md:78`
— "a meer with an allow-list refused an unlisted peer at the door" — and the E11 finding below it
states the principle outright: **a blind relay can still moderate by membership and scale, from
metadata alone.** Different component, same shape. This is that result applied to the tunnel bucket,
not a new idea needing its own defense.

The rule collapses to: the allowlist a member already needs for "who may ring me" is the same list that
sponsors the tunnel. One list, owned by the member, two effects.

**The separation that makes "open" safe to offer:**

- **Who** — entirely the member's call. Allowlist, mutuals, open, whatever.
- **How much** — yours. A per-member sponsorship quota per window, independent of their policy.

Without the second, an open member is an unbounded claim on relay bandwidth and the spam surface
becomes your bill rather than their inbox. Meter the quota in **relayed bytes**, not sponsored-call
count, since a call count prices a 20-second call the same as an hour.

**Where the allowlist lives** — three options, with the privacy hole named: in the member's PDS record
(sovereign but publishes the call graph; salted hashes don't rescue it, since DIDs are enumerable off
the firehose so a public salt lets anyone test every DID); in `croft-admit`'s registry (private, but
centralizes the graph); or **no list at all — a call capability**, where the member signs a cap for each
person they admit and the caller presents it at mint time. Nothing enumerable exists, revocation is a
rotation or short expiry, and it reuses the EdDSA machinery already under test. Recommended.

**The tension owed either way:** whichever is picked, `croft-admit` learns the pair at mint time. Graph
knowledge moved off the relay — the point — but into the admission service, not out of existence. It's
a much thinner surface (call setup only; no content, no frame timing), and two things belong in the
ADR: don't persist mint events beyond what the quota needs, and keep `croft-admit` self-hostable.

---

## Part 5 — capability vs registry, and the DID-signing question

**Owner:**

> Okay, so I just want to talk number one first, because I understood some of it. So it seems like the
> options are like I think it's a is it a cryptographic capability, like you have a bearer token, you
> know what I mean, like a pre-signed URL or version or something where you basically come in with a
> token in hand and say, I'm allowed to make this call which doesn't solve for the I'm calling someone
> who cause it's always gated on the caller, right?
>
> Which maybe I really what I'm looking for is like members to be able to exercise their tunneling
> capabilities even, you know, when they're not initiating. And I guess maybe you could do that two
> ways. Maybe you could allow members to give out tokens to call them, you know, and you could allow
> them to have standing authorizations and you could do both you know cryptographic you could just have
> I mean could you have like the the DID assertion could you sign it that way I don't I don't know that
> says yes that's the person who's allowed and yes this is me.
>
> I'm not sure. And then the registry is I think maybe just a like a lookup right. And I'm saying maybe
> we want both because they they're on two sides of the street. And then the PDS record, nah, definitely
> not. Like we could do the the SIS server, our own storage server.
>
> And honestly that's where we'd keep the registry, I expect. or at least canonically likely. Anyway,
> see what you think of that and like where it leaves.

**Assistant** *(after reading `CISS/src/statements.rs`, `src/policy.rs`, `crates/ciss-auth/src/lib.rs`)*:

**Framing correction.** "It's always gated on the caller" is true about who *holds* the token but not
whose *authority* it carries. A cap M signs and hands to N is M's tunneling being spent, by N, while M
is asleep — precisely the non-initiating case working. What a cap can't cover is a caller M never
handed one to. So it's three sponsorship paths:

| Who initiates | What proves the member is a party | Infrastructure needed |
|---|---|---|
| Member calls out | the member's own broker token — they're right there | none |
| Stranger calls in, holding M's cap | M's signature on the cap | verify a signature. No lookup, works offline |
| Stranger calls in, no cap | M's standing policy record | a lookup |

Row 1 had not been called out explicitly before: a member initiating needs no cap and no list.

**Both, and they're different streets.** Cap is push — issued on-device, no server round trip, nothing
enumerable, works when M is offline; weak on revocation. Standing record is pull — the only thing that
can express "open," and it gives clean revocation; but it's a list, so it exists to be leaked.

**The DID-signing question, answered by what CISS already resolved** (`CISS/src/policy.rs:12-23`):

- **Croft-native `id:` identities** — the DID *is* the hash of their ed25519 key, so they sign the
  record directly. Model A, `OwnerSigned`.
- **atproto `did:plc` identities** — they do **not** hold their signing key; the PDS does. They cannot
  self-sign a CISS record. Model C, `ProviderAttested`: the owner authorizes the *action* via a
  service-auth JWT, CISS verifies it, then counter-signs the record with its own attestation key over a
  separate domain.

**And CISS is not just where the registry goes — the record already exists.** `src/policy.rs` is an
800-line signature-verifiable owner-authorized access record: target; class `world | grantees | owner`
(= open | allowlist | closed, exactly the owner's framing); explicit reader DID list; monotonic `seq`
for rollback protection; both signing models. This is a rename and a new domain tag, not a build. Per
that file's own principle (read policy and rent base are separate records with separate signature
domains), the call policy should be a **sibling** record — `ciss/v1/call-policy` — not an extra class on
the read policy: changing who can ring you shouldn't touch who can read your storage.

Open after this: whether the cap reuses CISS record signing or is a `croft-admit` JWT; cap revocation
(short expiry vs a revocation seq, and the latter reintroduces a lookup); and the graph tension moving
to CISS.

---

## Part 6 — the JWT decision, and hashing

**Owner:**

> I think we for now stick with the get the JWT approach where it's like a short lived grant from the
> PDS, you know. I think that that model makes sense for us right now. And I don't really want to
> extend the storage sys identity much outside of sort of internal work at the moment if I can help it.
> and then I think that solves the revocation question, right?
>
> Because they're fairly short-lived by nature. and agreed on the graph tension. I think that's
> something that we should note because we don't need to store identities. we could basically hash
> these things and store them like as references because we're just seeing are you this thing? We're
> not trying to we don't care what your value is, we just care that it's matched.
>
> So that would be like the thought. It would be the assertion, oh, who are you, blah blah blah. Then
> at least at rest, it's not you know discernible, but yeah, it's a definite it's a definite
> accumulation to keep in mind.

**Assistant** *(after reading `CISS/crates/ciss-auth/src/service_jwt.rs` and `replay.rs`)*:

**The JWT decision.** An atproto service-auth JWT (`com.atproto.server.getServiceAuth`) has `iss` = the
caller's DID, `aud` = the target service, `lxm` = the specific method, `exp` ~60s. Those bindings mean
it authenticates **M's instruction to you** and is deliberately not transferable to N. So two tokens
with distinct jobs:

```
M's app ──[service-auth JWT: iss=M, aud=croft-admit, lxm=grantCall]──▶ croft-admit
                                                                          │ verifies
                                                                          │ against M's
                                                                          │ DID doc key
N ◀────────[cap: sub=N, iss=croft-admit, short exp]───────────────────────┘
```

Same split as CISS's Model C — JWT authorizes the action, the service issues the durable artifact —
without adopting CISS's record signing. `ciss-auth/src/service_jwt.rs` already verifies exactly this,
and does it correctly in the way that's easy to get wrong: **the verification algorithm is taken from
the DID-resolved key's curve, never the JWT header**, so a forged `alg: none`/`HS256` can't downgrade
it, and the signature is checked before any claim is trusted. There's a `ReplayGuard` for the `jti`.
That crate states its own boundary as authentication only, so depending on it takes the verifier, not
CISS's `id:` identity space.

**Revocation — yes, with one condition.** Short expiry gives revocation only if **renewal re-evaluates
the policy**; otherwise auto-renew launders a revoked grant forever. That condition is free here,
because a call outlasts any sane cap lifetime so renewal is mandatory anyway. Make each renewal re-check
and you get something better than revocation-at-next-call: **boot someone mid-call**, within one
renewal interval.

**Hashing — the instinct is right, the naive version doesn't work.** "We only need equality, not the
value" is correct and is what makes it cheap. But plain `sha256(did)` buys close to nothing, because the
DID space is public and enumerable: plc.directory and the firehose let anyone download every atproto
DID, and hashing ten million takes seconds. A stolen database of DID hashes is a stolen database of
DIDs. What works:

- **HMAC with a key held outside the database** — not in the same backup, not in the same dump. This is
  the whole win, one line from the naive version.
- **Key the HMAC over the pair** — `HMAC(k, member_did ‖ caller_did)`. Costs nothing (both DIDs are
  always present at lookup, since the query is only ever "is N on M's list") and means the same person
  is a *different* digest in every list. Without it, one digest recurs across every list they're on and
  an attacker can cluster the graph — who's popular, who overlaps — without learning a name.

Ceiling: this protects the artifact at rest against a leak. Not against the operator or a live
compromise, because a running server that can answer "is N allowed" can answer it for every N. And it
**removes the labels, not the graph** — edge counts per member, grant timing, renewal cadence all stay
legible.

One functional consequence: if the list is HMAC'd with a server key, **the member can't be shown their
own allowlist**. Either the device keeps the display copy, or the list is stored encrypted to the
member's key next to the match index. Naive hashing usually ships and then dies on this.

**Clarifying question raised:** "don't extend the storage-sys identity outside internal work" sounded
like the registry-in-CISS decision was walked back.

---

## Part 7 — clarifications, and the shape that settled

**Owner:**

> Okay, yeah, I understand. we're relying on the user getting an assertion that they are who they are
> and then we mint them the the token to do what they need to do. We're handing off the identity portion
> of the verification because the identity is the authorization. Understood on the revo revocation. On
> the hashing idea, I think we should design it in and test it, but I wouldn't futz with it for now just
> in testing. Just cause we're gonna wanna be able to confirm things. this is a definite need in terms of
> like ability and testing and all that but I wouldn't say it's critical to like get the first couple set
> up.
>
> I guess I just don't want to pre because you have to really store the HMAC somewhere different and I'm
> not ready to sort of answer that question in a way that is is robust enough. So like I wanna see the
> mechanism work, but then also not be fooled that we're relying on it because there's sort of more
> outside of this closed system to think through. And then on the don't extend the storage no, what I
> meant was I don't I'm hesitant to extend the use case for the sis specific you know key pair identity
> key pair like that DID versus ID like I'm really hesitant to use that ID for things outside of that
> scope and context, you know.

**Assistant:**

**On the identity handoff.** Agreed, with one distinction that is the reason the second token exists:
**authentication** — who is this, provably — is delegated to atproto and the PDS. **Authorization** is
not. The DID tells you who; membership, tier, and sponsorship are yours and live in the token you mint.
That's why the PDS JWT never reaches the relay: it answers a question the relay didn't ask. Same
boundary `ciss-auth` draws in its own doc comment.

**Hashing: build the seam, don't claim the property.** Put every stored identifier behind one
indirection with two implementations:

```
trait IdIndex { fn key(member: &Did, caller: &Did) -> IndexKey }

  Transparent   stores the DID as-is        ← dev, test, first deployments
  Keyed         HMAC(k, member ‖ caller)    ← built, tested, not depended on
```

The lookup path never knows which is active; the test suite runs the equality-matching behaviour
against **both**. Two things to build now rather than later: **the migration** (Transparent → Keyed is
cheap while the store is small), and **an ADR note that the security property is explicitly not
claimed** in either mode until key custody is answered — this is the thing that rots, where "we hash the
DIDs" gets read a year later as "they're protected" by someone who doesn't know the key sits in the same
backup. Separately, there's a privacy win available today with no key-custody question: **store only
what the quota needs and don't persist mint events beyond it.** Not logging a thing beats hashing it.

**On CISS — a narrower concern than previously read.** Nothing in this design touches the `id:` space.
The calling identity is `did:plc` end to end: the PDS mints the service-auth JWT, `croft-admit` verifies
against the DID-resolved key, the cap names a `did:` subject. Choosing JWT-mint over CISS record signing
already took Model A off the table, and Model A is the only `id:`-only path. So the registry question is
purely "where do the bytes live," with no identity implication. Recommendation: **`croft-admit`'s own
store**, for a reason unrelated to identity — a relay admission decision shouldn't block on a storage
service being up. Revisit if the registry grows into something wanting durability guarantees and
metering, which is CISS's actual job.

**Settled shape for ADR-0006:** split relay instances behind one hostname with a tier-routing switch (no
fork; supersedes the fork half of ADR-0001); sponsorship decided at mint time by the member's policy
across three paths; PDS service-auth JWT authenticates the instruction and `croft-admit` mints the cap,
reusing `ciss-auth`'s verifier; short cap lifetime with renewal re-evaluating policy, so revocation
lands mid-call; registry in `croft-admit`'s store, atproto DIDs only; `IdIndex` seam with both impls
tested and the migration included, property not claimed; graph-shape accumulation recorded honestly;
ADR-0004 fallback correction; new gates onto T62. Noted that the coordination-bucket number and the
sponsorship quota both want the same real-NAT measurement and should be one gate, not two.

---

## Part 8 — the filing instruction

**Owner:**

> Okay, let's take this and do a couple of things with it. Number one, we really need to just persist
> this conversation as a raw transcript, 'cause there's a lot here. Number two, build out this plan and
> kind of get it all together in you know phases and groups of milestones because there's quite a bit
> here and then we'll want to run it through the phase one and two phase plan scale.

*(This file is the response to item one. Item two is
`../../plans/2026-08-07-1-plan-croft-relay-tiered-admission.md`.)*

---

## Part 9 — Pass 2, and the feasibility sweep (2026-08-08)

The plan was written and run through the `phase-plan` skill's Pass 2 (gap analysis), which verifies a
plan's claims **against the code** rather than against its own logic. It found that three preconditions
the plan assumed did not exist: `Registry` is an in-memory `Mutex<HashMap>` with no persistence and only
an EndpointId→Did direction; `croft-admit` has no `[[bin]]`; and `PdsResolver` is a fixture-only trait
whose promised "production adapter" was never built. It also found that `ciss-auth` does **not** resolve
DIDs (`ResolvedKeys` is defined there and must be supplied), that the Milestone B ∥ C parallel grouping
was disqualified by a shared `Cargo.toml`, and that there was no `rust-toolchain.toml` against a
CI-pinned toolchain.

The owner's response set the direction for everything after:

> our startnig plan got enough things flat out wring that I want to review the feasibility and plan in
> it's entirety and surface items for discussion from there not adhoc big chnanges now

A deeper sweep followed, which **confirmed the load-bearing feasibility claim** —
`RelayServiceWithNotify` is `pub` in `pub mod http_server`, implements `Service<Request<Incoming>>`, and
its own docs carry an embedding example using `serve_connection(...).with_upgrades()`; `KeyCache`,
`Handlers`, `Metrics` are all reachable — and produced two corrections to claims made earlier in this
dialogue:

1. **"You can boot someone mid-call" was wrong.** `on_connect` runs once, at attach. An admitted
   connection is never re-checked and token expiry does not disconnect anything. Eviction is possible
   (`Clients::disconnect` is public) but must be actively driven by a control path that did not exist in
   the design.
2. **The byte quota had no data source.** iroh's metrics are server-wide counters
   (`bytes_sent`/`bytes_recv`, `#[non_exhaustive]`), with no per-endpoint dimension. Per-member
   accounting is not obtainable from the relay.

## Part 10 — the objection that changed the mechanism

**Owner:**

> I still don't understand why we want to constrain moving the full tunnel by constraining speed. Is it
> that it's the same connection that gets upgraded and so we're preventing that because it feels to me
> like the the mutual you know handoff when that fails it seems like there would be an escalation to
> tunnel and that's what we would want to stop rather than just trying to slow it down so it fails.
>
> That feels ugly.

**Assistant:** The intuition is right and the mechanism is the reverse of the picture. **The relay
carries the call from the first packet.** Direct connection is the *upgrade*; holepunch success is what
*removes* the relay from the path. There is no "now switch to tunneling" moment to intercept, which is
why the plan reached for a speed limit — there is no event to refuse, only an ongoing flow to shape.

The relay cannot tell a coordination frame from audio (content-blindness, ADR-0004, a property to keep).
But volume is an honest proxy, and there are two ways to use it:

- **Throttle** — degrade until media fails. A soggy "maybe": the user gets a call that sounds broken
  rather than one that says why it can't proceed. Calibration-sensitive.
- **Budget and drop** — give a generous allowance, several times what an introduction needs, and when
  it is spent **disconnect cleanly** so the client can surface "this needs a membership."

Budget-and-drop is also far less sensitive: an introduction is kilobytes, a call is kilobytes *per
second*, so any budget in the two-to-three-order gap separates them. Throttling had to thread a needle;
a budget only has to land in a canyon.

Two mechanics: no fork is needed (`Clients::disconnect` is public), and the wrinkle that iroh clients
auto-reconnect with backoff is solved by **putting the budget in the token** — a spent token is refused
re-admission, and a new call needs a new token whose issuance croft-admit controls.

Later in the same thread this produced the largest structural consequence: **the budget deletes the tier
switch.** With enforcement in our own layer, multiple relay instances with differing rate limits are
unnecessary — one instance, no rate limit, and a supervisor that drops.

The owner's product framing, given a few paragraphs later, is what made the mechanism obviously right:

> the point here is to provide a very cheap service when possible, matchmaking, as a gesture of goodwill
> when as long as we can, but then also have like you know a membership non-extractive sure you can
> bridge the gap and get you know a different quality of service

## Part 11 — the eleven questions, walked

The remaining items were walked one by one in plain English. Owner rulings, condensed:

1. **Lexicon / multi-device.** The owner pointed at `CroftCommunity/connect`, previously unseen by this
   session. Its `docs/contract.md` defines `ing.croft.iroh.endpoint`, rkey `self`, `endpointId`
   required. Nothing published yet. Ruling: extend for multi-device **up front**, "home"/"work" style,
   and test with at least two devices for early personas. Assistant's addition: the atproto-idiomatic
   shape is one record per device with rkey = the label, discovered via `com.atproto.repo.listRecords`
   — which is also the native answer to the owner's "give me all your phone numbers / load this
   contact" question, and keeps the exchange page backendless.
2. **Strangers.** Coordination-only until admitted. "Open" is a choice we would advise against — build
   it and test it as the counter-case so we understand the mechanism, do not advertise it.
3. **One list or two.** Dissolved by the owner's own proposal: only people you have given a token to can
   call you at all. The cap is both gates. Cost named: discovery and reachability come apart — findable,
   not cold-callable.
4. **Mid-call eviction.** Front door only. Both sides can hang up; do-not-disturb; a zero-cost blocklist
   matters more. Reframed during the discussion: eviction is a **bandwidth-abuse** tool, not a safety
   tool, because app-level hangup already closes the connection.
5. **Who may press eviction.** Not worried; moot while 4 is no.
6. **Usage counting.** Owner: "sponsorships and for how long is a shit proxy" (conceded). Proposed a
   fork for structured call accounting, as pluggable and discrete as possible, and to try upstreaming
   it. Assistant flagged the shape distinction: **volume** accounting (`endpoint, bytes, duration`)
   builds no graph; **pair** accounting (`src, dst`) is a call-detail record, the exact artifact the
   design exists to avoid. Owner agreed on the metadata danger.
7. **Upper-bound byte counts.** Fine, as long as it is never called billing.
8. **Storage.** Owner: person is a Bluesky handle, endpoint is a lexicon record, cache it locally — so
   it is a resolve-and-cache, not a table we own. Most of the scoped work dissolved.
9. **Resolution and failure.** Owner has test-account credentials. Fail closed, "and work it out from
   there." Assistant clarified the two directories: **plc.directory** (DID → which PDS) versus the PDS
   itself.
10. **Measure first.** Owner: ~10% is the prevailing figure and that is fine. Recorded as a working
    assumption, recalled rather than measured.
11. **The two admission paths.** Owner asked for an explanation. Path A = n0's **stock prebuilt binary**
    with its `access.http` hook calling croft-admit per connection — no custom binary needed, and
    literally what runs at relay.croft.ing today (v1.0.0, from a tarball). Path B = **our own binary**
    with the verifier compiled in, no network call in the attach path.

## Part 12 — clarifications that changed the conclusion

**Owner:**

> I'm confused when you say the relay can tell what did you say? But I can't tell how much and for how
> long. I mean, aren't you talking about per connection stats there? Because that just never persisted
> anywhere.

**Assistant, correcting:** two things were blurred. The rate limiter *does* count bytes per connection —
that is what a token bucket is — but that counter lives in a `pub(crate)` wrapper, is never reported,
and dies with the connection. The metrics are server-wide totals with no per-connection dimension. So
the correct statement is: **the relay cannot tell us. We can, if we count it ourselves.**

**The doorman, described in detail** (the owner asked whether it meant nftables or eBPF): none of those
— they sit below the layer where identity is known, and the traffic is TLS. It is a decorator inside our
own process. Our binary accepts the TCP connection, terminates TLS, reads the HTTP request (so the token,
therefore the member, is known), then wraps the stream in a struct implementing `AsyncRead`/`AsyncWrite`
that increments a counter, and hands the wrapped stream to iroh's service. On close it emits
`(member, bytes_in, bytes_out, duration)`. Roughly forty lines. It counts framing as well as payload, so
it is an upper bound.

**The conflation that was driving the wrong conclusion.** The owner had said the compiling was not the
worry, keeping a fork pristine across upstream changes was — and then concluded Path A looked right.
Three distinct things were being collapsed:

| | what it is | maintenance |
|---|---|---|
| Stock binary | n0's prebuilt release | none, but no customization |
| **Our binary** | our `main.rs` depending on `iroh-relay` from crates.io, unmodified | `cargo update` |
| Fork | a patch series carried on their tree | rebase every release, forever |

The design lives in the middle: our own program, every line of relay code unpatched from the published
crate. Nothing to keep pristine because nothing is patched. And Path A's ceiling is exactly one
question — yes or no at the door: no budgets (one global speed limit for everybody), no disconnect
(`Clients::disconnect` is in-process), no accounting (no doorman). So Path A is the **fallback**, not the
destination.

**Storage, followed through.** If a cap gates calling, there is no list to store; verification needs only
the issuer's public key, resolvable from their DID. What must persist shrinks to **membership** (a
commercial relationship we necessarily have) and **accounting** (our own costs). Neither is a social
graph. That directly serves the owner's stated goal:

> I'd rather people be able to use this who aren't, you know, not only not members, but like people we
> don't have to manage storage for.

The remaining fork: a `did:plc` user does not hold their signing key, so they cannot sign a cap on their
device — but atproto signs **repo commits**, so a record in their repo is verifiably theirs. Caps as
**opaque-id records in the callee's own repo** (the record names nobody; the caller holds the cap;
revocation is a record delete) versus caps minted and stored by croft-admit.

## Part 13 — the trust ruling, and the rewrite

**Owner:**

> Yeah, I understand about the them not holding their own signing key. I get it. Yeah, I think we just
> count like a valid signed record in their PDS as a valid signed record and like that whether they run
> their own PDS or not to us as a material.
>
> So, like, the risk is on their side.

Recorded as the design's trust boundary:

> A record in a user's repo is that user's statement. Whether they hold the signing key, or their PDS
> operator does, is between them and their operator. We verify the signature and the DID binding; we do
> not model PDS trust.

Blast radius recorded alongside it: a hostile or compromised PDS could mint a grant in a user's name,
buying **relayed bandwidth in their name** — not access to any call, since the call is end-to-end and the
relay is blind. A bounded, financial-shaped harm, absorbable by a quota. Whether a user holds recovery
keys and can undo a hostile PDS is the standard atproto recovery question; the corpus FACTCHECK docs do
not cover it and nothing here depends on the answer.

Final rulings in the same turn:

- **Request-to-token flow: yes, and it is the user's choice.** "The flow needs to exist and the user
  decides — like if strangers can request or, you know, only mutuals or whatever." Not one size fits
  all. Implemented as a small published enum (`anyone | mutuals | nobody`), which leaks nothing about
  who a user knows.
- **Cap revocation: probably both** — expiry on the minted token *and* deletion of the grant record.
  Each alone is insufficient: expiry alone lets a revoked cap keep minting until it lapses; deletion
  alone leaves already-minted tokens live.
- **No upstream accounting patch.** "let's try the our own binary, you know, wrapping theirs and see what
  happens… it functions kind of like a decorator basically. Their stuff doesn't know the difference."
- **A comprehensive rewrite**, with cohesion as the explicit requirement given how much surgery was
  involved.

The rewrite is `../../plans/2026-08-07-1-plan-croft-relay-tiered-admission.md`, retitled *cap-gated
calling with a metered introduction budget*. Its Review Log carries the superseded Pass-1 and Pass-2
records rather than deleting them.

---

## Source-verified claims in this dialogue

Every mechanism claim above was read from pinned source in this session, not recalled. For a future
reader deciding what to re-verify:

| Claim | Source read |
|---|---|
| `Access::Allow` is a unit variant; no rate field | `iroh-relay-1.0.3/src/server.rs:350` |
| Rate limiter installed pre-handshake, from one service-wide watcher | `iroh-relay-1.0.3/src/server/http_server.rs:856` |
| `on_connect` runs after authentication, at `authorize_with` | `.../http_server.rs:876` |
| `RateLimited` is `pub(crate)`; `Bucket` is `pub` | `.../server/streams.rs:333`, `:363` |
| `RelayService::new` takes access control + rate limit; `set_client_rate_limit` is service-wide | `.../http_server.rs:911-939` |
| Relays do not federate; one `ActiveRelayActor` per relay server | `iroh-1.0.3/src/socket/transports/relay/actor.rs:1-25` |
| Send path takes the destination's relay URL | `iroh-1.0.3/src/socket/transports/relay.rs:272` |
| `Endpoint::insert_relay` is public + async (runtime token swap) | `iroh-1.0.3/src/endpoint.rs:984` |
| Client auth header name; token via `Authorization: Bearer` / `?token=` | `iroh-relay-1.0.3/src/http.rs:18,23`; `src/client.rs:157,320` |
| TLS-exporter auth degrades gracefully to a challenge round-trip | `iroh-relay-1.0.3/src/protos/handshake.rs:447` |
| CISS policy record: class `world/grantees/owner`, `seq`, Model A / Model C | `CISS/src/policy.rs:1-27` |
| service-auth JWT verified with curve from resolved key, not header | `CISS/crates/ciss-auth/src/service_jwt.rs:1-12` |
| `ReplayGuard` bounds `jti` reuse inside validity | `CISS/crates/ciss-auth/src/replay.rs:1-8` |
| Deployed relay is 1.0.0 via prebuilt musl tarball, pinned + checksummed | `croft-stack/ansible/group_vars/all.yml:72-73` |
| Prior art: blind relay moderates by membership + scale | `alpha/ROUND-2026-06-17-media-meer-conformance.md:78` and the E11 finding |
| **(Part 9+)** `RelayServiceWithNotify` is `pub`, implements `Service<Request<Incoming>>`, docs carry the `serve_connection(...).with_upgrades()` embedding example — the seam the whole design rests on | `iroh-relay-1.0.3/src/server/http_server.rs:714-745` |
| **(Part 9+)** `RelayService::handle_connection(TcpStream, Option<TlsConfig>, Duration)` is public but does its own TLS/HTTP, so it cannot expose the header to us | `.../http_server.rs:1013` |
| **(Part 9+)** `KeyCache` exported; `Handlers` is `pub` + `Default` + `DerefMut`; `Metrics`/`RelayService` re-exported | `iroh-relay-1.0.3/src/lib.rs:48`; `src/server.rs:73`; `.../http_server.rs:1130` |
| **(Part 9+)** `Clients::disconnect(endpoint_id, connection_id) -> bool` is public — eviction possible but must be actively driven | `iroh-relay-1.0.3/src/server/clients.rs:181` |
| **(Part 9+)** Metrics are **server-wide counters**, `#[non_exhaustive]`, no per-endpoint dimension — per-user accounting unobtainable from iroh | `iroh-relay-1.0.3/src/server/metrics.rs:9-24` |
| **(Part 9+)** `RelayConfig.limits.client_rx`; `Server::spawn(ServerConfig)` as the higher-level path | `iroh-relay-1.0.3/src/server.rs:111-160`, `:691`, `:741` |
| **(Part 9+)** `Registry` is in-memory `Mutex<HashMap<EndpointId, Did>>`, one direction only | `crates/croft-admit/src/registry.rs:17-50` |
| **(Part 9+)** `croft-admit` has no `[[bin]]`; `PdsResolver` is a fixture-only trait; `Did` is deliberately opaque | `crates/croft-admit/{Cargo.toml, src/pds.rs:38-42, src/did.rs:1-11}` |
| **(Part 9+)** `ciss-auth` does not resolve DIDs — `ResolvedKeys` is defined there and must be supplied; no HTTP client | `CISS/crates/ciss-auth/src/lib.rs:127` and its `Cargo.toml` |
| **(Part 9+)** CI pins toolchain `1.94.1`; experiment has no `rust-toolchain.toml`; `croft-relay` is in the smoke matrix | `discovery/.github/workflows/smoke.yml:41,45-48` |
| **(Part 11)** `connect` lexicon: `ing.croft.iroh.endpoint`, rkey `self`, `endpointId` required; deep-link contract; resolution pipeline | `CroftCommunity/connect` `docs/contract.md` §1–§2 |
