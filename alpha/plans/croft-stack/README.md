# croft-stack — optional server-side accelerators (roadmap)

date: 2026-07-24
status: DRAFT for review — not yet executed. Plan-first. No box mutated, no purchase made.

This is the **roadmap/index** for the croft-stack effort. Cross-cutting concepts (ethos, end state,
the cache/index engine, the auth helper, the deployment map, governance, the service inventory, open
decisions) live here. **Each phase has its own detailed plan file** — see "Steps" below.

The design ethos: **every pad is fully usable with no server; the server is an optional
accelerator.** This effort builds an estate of such accelerators on an adopted OVH box, riding the
pre-existing host kit, contract, and validation crate (pre-existing, not assumed-validated — the
kit's idempotency in particular is re-established here, with caution).

Owner decisions still open are collected in "Open decisions" at the foot; they are tracked, not
pre-resolved. Recommendations are marked *(rec)*.

---

## Steps (the roadmap — each links to its own robust plan)

| Phase | Plan file | Purpose | Gate-out |
|---|---|---|---|
| 0 ✅ | [00-model-and-manifests.md](00-model-and-manifests.md) | agree the model + the concrete `services/*.toml` on paper | **MET** — manifests agreed (Q1–Q6 resolved) |
| 1 ✅ | [01-extract-croft-stack.md](01-extract-croft-stack.md) | extract the kit → `CroftCommunity/croft-stack` | **DONE** (pushed `fcf49a7`) — seeded + renamed; `make check` **12/13 green**; the 2 red are toolchain-gated (terraform→fold→2, local-drill→backups-paused), not the rename |
| 2 ✅ | [02-adopt-box-declaratively.md](02-adopt-box-declaratively.md) | OpenTofu VPS read + reproduce recipe; box reimaged clean | **DONE** — `tofu plan` read `vps-e9655dff.vps.ovh.us` (no order); recipe `vps-2027-model3`/US/`us-west-or-2`; box reimaged Debian 13 |
| 3 ✅ | [03-governance-telemetry.md](03-governance-telemetry.md) | limits+accounting defaults + local telemetry client | **DONE** — governance (`2f596a9`) + telemetry client (4 phases TDD, thru `73c25e5`; 32 pytest + 6 bats; validated on real box cgroups) |
| 4 ✅ | [04-stub-bringup.md](04-stub-bringup.md) | **Ansible** converge on a clean box (idempotent) | **DONE** (`0550fb7`) — box converged; run-3 `changed=0`; no lockout (key-only); canary `/healthz` ok + governed; telemetry sampling |
| 5 ✅ | [05-dns-tls.md](05-dns-tls.md) | A/AAAA + Caddy auto-TLS | **DONE** (`58dec5a`) — `canary.croft.ing` A/AAAA→box; Caddy upgraded (Debian 2.6.2→official v2.11.4); **`https://canary.croft.ing/healthz` → `ok`, trusted prod LE cert** |
| 6 ✅ | [06-iroh-relay.md](06-iroh-relay.md) | iroh-relay **dev/test relay** (prebuilt, behind Caddy) | **DONE** (`89b8b4b`) — iroh-relay 1.0.0 up, governed, telemetry-sampled; mode A (plain HTTP behind Caddy, no UDP); idempotent; public pending `relay.croft.ing` DNS |
| 7 | [07-auth-helper.md](07-auth-helper.md) | confidential-client spike → shared broker | session past browser-only TTL; clean fallback |
| 8 | [08-cache-server.md](08-cache-server.md) | `StateSource` seam; bluebird → arecipe cache | pad reads via cache; unaffected when cache off |
| 9 | [09-stellin-index.md](09-stellin-index.md) | index mode; backups designed but paused | serves a query no upstream can |
| 10 | [10-drystone-layer.md](10-drystone-layer.md) | croft-groups (factoring open) + MLS convergence (gated) | each an independent governed mini-stack |

**Status:** Phases 0–6 **DONE** — plus the **iroh-relay dev/test relay** is live (mode A: plain HTTP
behind Caddy, governed, telemetry-sampled; public pending `relay.croft.ing` DNS). Box **converged and
live over HTTPS**: `https://canary.croft.ing/healthz`
→ `ok` (trusted prod LE cert), firewall default-drop, SSH key-only, `canary` governed, telemetry
sampling; converge idempotent. Phase 7 auth-helper **spike done/GO** (production broker remains).
**Next (serves live pads):** Phase 7 — the production auth-helper broker (Rust; mechanism proven), or
Phase 8 — the cache server for bluebird/arecipe. Detailed = Phases 0–6 + 07 + telemetry-client-plan;
scaffolded = Phases 8–10. (Pad `skylite` renamed **`bluebird`**.)

**Execution logs (procedures, not just plans):** every working session is logged in the `croft-stack`
repo under `sessions/` (grouped by target LOCAL / OVH-API / BOX / GIT, secrets redacted). This roadmap
is the intent; `sessions/` is the actuals.

---

## Followups queue

Discovered mid-phase; each is tagged **[hold]** (finish the current phase first) or **[fold→N]** (fold
into phase N). Cleared items move to the owning phase's plan or are struck.

- **[fold→2]** `tests/terraform.bats` + `scripts/terraform-check.sh` hardcode the `terraform` CLI, but
  we chose **OpenTofu** (`tofu`). Switch them to `tofu` (or a `${TF:-tofu}` hook). Content already
  verified clean under `tofu fmt -check`; `check-terraform` (and its transitive `check-extraction`) are
  the only logic-clean-but-red sub-checks. Owned by Phase 2 (IaC).
- **[Phase 2, done — review + commit]** The kit terraform had **provider drift** vs the current
  `ovh/ovh`: `ovh_order_cart_item*` (order path) are removed resource types, and `ovh_vps.ips` is now a
  set. Fixed in `terraform/main.tf` (stripped the order path — we adopt + reinstall in the panel, never
  order via terraform) and `terraform/outputs.tf` (`tolist(ips)[0]`). **Uncommitted, pending owner
  review.** Related [fold→later]: re-author an OVH order path against the current provider only if
  ordering-via-terraform is ever wanted. And [fold→2] still stands: point `terraform.bats` at `tofu`.
- **[fold→3]** `check-local-drill` (the destroy→restore fire-drill) needs **litestream** (absent
  locally); rclone is present. This is the drill our plan already **defers with backups paused**, so it
  is red until the backup toolchain lands. Owned by the backups track (Phase 3+). Not a rename issue —
  the rename touched no drill/backup logic.
- **[fold→1] (dev-setup, ready to write)** Local `make check` on macOS needs a toolchain the kit assumes
  from Debian: **bash 5** (`brew install bash` — system 3.2 lacks `mapfile`), **bats-core**,
  **shellcheck**, **opentofu**; litestream+terraform still absent (the two red checks). Document this
  prereq list in the croft-stack repo. All installed this run except litestream/terraform.
- **[RESOLVED → Ansible]** In-box mechanism decided (Open decision 10): **Ansible** (Python-ecosystem,
  idempotent) owns the in-box layer, replacing the dropped `bootstrap.sh`; OpenTofu keeps the resource
  layer; the `render.py` generator produces the unit/vhost artifacts Ansible converges. **Phase 4 is
  reframed around Ansible.** `bootstrap.sh --plan` is retained only as the *spec/checklist* for authoring
  the playbook (what the bring-up must do: packages, SSH hardening, nftables, Caddy, users, deploy user,
  unit install), never run. Authoring the playbook + its idempotence test is Phase 4 work.
- **[hold→2]** Box is **Debian 13 (trixie)**, not Debian 12 (`BOX-CHANGELOG` baseline). The kit
  terraform (`os_image` default "Debian 12") and bootstrap assume 12. Decide target OS for the
  reproduce-next-box recipe (accept 13 vs pin 12). Also `terraform/variables.tf` `service_display_name`
  default is still the stray `croft-appview`. Resolve in Phase 2.

---

## Problem statement

Croft has four live client-side pads (`arecipe`, `bluebird`, `pdsview`, `croft-pwa`). They ride the
user's own PDS and Bluesky's public infrastructure and **that is the intended design floor, not a
gap**: each pad is meant to be fully usable with no server of ours. We have deliberately seeded
discovery with high-value starterpacks so friends-scoped, backendless operation is genuinely useful.

Two facts from the code are frequently mis-read as deficiencies. They are not:

- `arecipe` states in-source (`src/social/comments.ts`, `src/social/interactions.ts`):
  *"backendless: no global index of `app.arecipe.*`"* — discovery is friends-scoped, walking known
  repos one at a time. This is **working as intended**. It is not a defect to fix; it is the model.
- `bluebird` reads `app.bsky.feed.getAuthorFeed` from Bluesky's **public** AppView
  (`public.api.bsky.app`, `src/atproto/client.ts:9`). That is an **unauthenticated public read by
  design** ("Does not require auth"); bluebird runs as a pure PWA with no account and no server of
  ours. Its client already takes an injectable `baseUrl` that **defaults to the public AppView** —
  so an optional cache is a one-line origin swap, and if the cache is down the pad falls straight
  back to the public AppView with zero code change.

What is genuinely worth having is not a global index but **optional acceleration**: server-side
components that lighten PDS load, improve latency, and lengthen session life — while holding
**minimal state** and **degrading cleanly to serverless** when absent. We now have an OVH VPS
(manually provisioned) to host them.

This effort takes us from "designed and rehearsed in discovery, never deployed" to a **live,
step-by-step-deployable estate of optional accelerators** on that box.

---

## Design ethos (load-bearing — every later decision defers to these)

1. **Serverless is the floor.** Every pad works with no server. No accelerator may become a hard
   dependency of any pad.
2. **Accelerators are independently deployable and independently removable.** Remove any one and the
   pad it fronts degrades to its serverless path (public AppView / PDS-direct / browser-only OAuth),
   never breaks.
3. **Minimal held state.** Prefer state that is ephemeral and reconstructible. Where state is
   canonical (the auth helper's keys/sessions; an index cursor), it is the smallest possible surface
   and is explicitly flagged.
4. **Mini-stack per thing.** Each product/purpose is its own deployable mini-stack of one or more
   processes. No mega-binary that fuses everything. (Exception by design: the **auth helper is
   shared** — one deployment for the whole estate.)
5. **Governed by default.** Every process runs under a per-process cgroup with resource limits and
   accounting from day one, with local telemetry we can read.

---

## End state (the target)

One OVH Debian-13 box, no orchestrator — **systemd is the control plane** (containers permitted per
stack; not required) — hosting an estate of optional accelerators:

```
CONSUMERS (browser pads — already live, stay fully usable with the box OFF)
   arecipe · bluebird · pdsview · croft-pwa
        │                    │                         │
        │ prefers helper     │ optional cache origin   │ serverless fallback always present
        │ when reachable     │ (baseUrl swap)          │ (public AppView / PDS / browser OAuth)
        ▼                    ▼                         ▼
┌───────────────────────────┐   ┌───────────────────────────────────────────────────────┐
│ AUTH HELPER  (SHARED, 1×)  │   │ CACHE/INDEX SERVER  (per product, one program 2 modes) │
│ confidential OAuth client  │   │  --mode cache : demand-driven, TTL'd, disposable,      │
│ · session broker           │   │                 NO firehose, miss ⇒ proxy PDS/AppView  │
│ · write-outbox             │   │  --mode index : firehose-fed, persistent index,        │
│ preferred-when-reachable;  │   │                 answers queries no upstream can         │
│ absent ⇒ browser-only      │   │  one SERVE surface over a StateSource seam             │
│ public-client OAuth        │   │  arecipe→cache · bluebird→cache · stellin→index         │
└───────────────────────────┘   └───────────────────────────────────────────────────────┘
        └───────────────────────────────┬───────────────────────────────────────────────┘
┌───────────────────────────────────────▼───────────────────────────────────────────────┐
│ HOST KIT (croft-stack) — atproto-agnostic substrate                                     │
│  systemd supervision · Caddy TLS (:443) · per-tenant users + hardening ·                │
│  per-process cgroup limits + accounting (EVERY unit) + local telemetry client ·         │
│  Ansible converge (in-box) · forced-command deploy channel · backups (designed, PAUSED)  │
└───────────────────────────────────────┬───────────────────────────────────────────────┘
                                         ▼
        THE BOX — one OVH Debian 13 VPS (adopted; reimaged clean; managed by OpenTofu (resource) + Ansible (in-box))
```

Plus, off the pad critical path, the **iroh relay** (first real service, connectivity-only) and the
later drystone-layer components (croft-groups; MLS convergence server, gated). See the deployment map.

---

## The cache/index server: one program, two modes

The read/serve surface (XRPC, hydrated views, `/healthz`) is identical in both modes. Only the
**state source** behind it differs — a single seam (a `StateSource` port with `DemandCache` and
`FirehoseIndex` adapters). We build and test **both**; we **run** whichever fits per tenant.

```
PASSTHROUGH CACHE  (--mode cache, demand/pull)     INDEX  (--mode index, firehose/push)
─────────────────────────────────────────         ─────────────────────────────────────
client asks ─▶ hit? serve ; miss ─▶ proxy          Jetstream firehose ─▶ ingest every event
   PDS / public.api.bsky.app, store w/ TTL             ─▶ persistent index.db (keyed by AT-URI)
held state: ephemeral, TTL'd, 100% rebuildable     held state: index (rebuildable via backfill) +
   by re-fetch; wipe ⇒ re-fetch. NO firehose.         a cursor (the one precious bit). IS firehose.
answers: only what upstream already answers,       answers: queries NO upstream offers — network-wide
   just faster + cheaper on the origin PDS            discovery of custom NSIDs, server-side search
backups: N/A (nothing canonical)                   backups: designed, PAUSED (non-canonical first)
```

- **Cache mode is the default accelerator.** It only ever makes an already-answerable query faster
  and lighter on the PDS. A cache **cannot** answer a query no upstream can answer.
- **Index mode is opt-in per tenant**, justified only by a query that has no upstream — currently
  just Stellin discovery/search.
- **Backups are relevant only in index mode** (cache holds nothing canonical), and are **paused in
  both modes for now** — everything is experimental and runs on non-canonical data. The approach we
  are confident in (Litestream→R2 for the index cursor/state) is sketched, not enabled; enabling is a
  switch, not a redesign. Gate to turn on: before any tenant carries real/canonical user data.

## The auth helper: shared, confidential OAuth broker

**Mechanism PROVEN live (spike, 2026-07-24 — GO).** The confidential-client login + server-side refresh
+ cross-domain brokered pad (via an opaque ticket, not a cookie) + clean fallback are all proven against
the live network. Remaining work is the **production broker** (Rust, hardened, multi-account). Full
record + plan: [07-auth-helper.md](07-auth-helper.md) and `discovery/spike/auth-helper/FINDINGS.md`.

- **Mechanism.** A browser-only PWA is a *public* OAuth client, so its DPoP-bound session is
  short-lived (the observed ~2-week TTL — Open decision 9, still needs a FACTCHECK citation). A
  *confidential* client — a backend holding the client's private key — can refresh server-side and
  broker a longer-lived session. The helper is that confidential client plus a session broker and a
  write-outbox (the surviving pieces of the account-kernel spike after K1 killed the shared-origin
  form).
- **Shared by design.** One helper deployment for the whole estate — a single confidential client
  identity and one broker — not one per product.
- **Runs from `account.croft.ing`.** The confidential `client_id` is just an HTTPS URL, so one
  subdomain serving `client-metadata.json` + `/callback` + `jwks` is sufficient. The `client_id` is
  **independent** of the contested Stellin service DID (Open decision 6) — the auth helper is not
  blocked by it.
- **Preferred-when-reachable.** Pads prefer the helper's brokered session whenever it is up; fall
  back to browser-only public-client OAuth on failure. Absent helper ⇒ pads behave exactly as today.
- **Validated (spike).** `authserve.rs` (RUN-14 EXP-A) had proved only the *service-auth JWT verifier*
  and named the OAuth login leg a non-goal; the 2026-07-24 spike then ran exactly that leg live and
  proved the whole confidential-client mechanism (login, background refresh, cross-domain broker,
  fallback). The **cross-domain pattern is an opaque ticket in first-party storage** (not a cross-site
  cookie — immune to WebKit/Safari purging, the account-kernel K1 lesson).

### Security hardening — production broker (post-POC TODOs)

The spike concentrates risk on the box to prove the mechanism; **none of it is safe to run at scale as
built.** The confidential helper is a high-value target (one client key + many users' sessions), and its
security backstop is that it stays **optional** — every pad keeps a browser-only floor. Full analysis,
asset ranking, and case-by-case threat model: `discovery/spike/auth-helper/SECURITY.md`. The production
broker must land these (priority order):

> **Authoritative home:** these decisions now live with the component, in the croft-stack repo at
> `croft-stack/docs/auth-helper.md`. The list below is the roadmap mirror — update the repo doc, not
> just this.

- [ ] **H1 — client key off disk & out of memory.** Sign client assertions via a **KMS/HSM signing
  oracle** (key never in process memory in the clear); add **JWKS key rotation** (multiple `kid`s, roll
  on schedule + on suspicion; revoke a leaked key by dropping its JWK — a `jwks_uri` document edit, not a
  `client_id` change). **Concrete cheap choice: OVHcloud KMS (CMK)** — it does **EC P-256 ECDSA
  sign/verify** natively via REST/KMIP (same vendor as the box, ~$/mo, rotation + revocation built in);
  Shared/Managed HSM is the higher-assurance upgrade. A box-local **vTPM** is the $0 option but is
  box-bound with no central rotation, so it is a weaker fit for a shared broker.
- [ ] **H2 — harden the host.** SSH keys only (no password), nftables **default-drop** (22/80/443),
  fail2ban, unattended-upgrades, **off-box append-only audit log**. (The host kit already generates the
  firewall + non-root units; the spike box did not apply them.)
- [ ] **H3 — envelope-encrypt sessions.** Off-box master key wraps per-record/per-user data keys;
  unwrapped key lives only transiently; **Rust `Zeroize`** in memory. (Today the store key is co-located
  plaintext, so at-rest encryption only survives a partial leak, not box compromise.) **The same
  OVHcloud KMS from H1 is the wrapping master — one dependency covers H1 (sign) + H3 (wrap).**
- [ ] **H4 — make tickets real credentials.** Short TTL + rotation, **DPoP-bound proof-of-possession**
  (stolen ticket without the key is useless), origin-bound + server-checked, minimal scope, per-ticket
  revocation, audit every brokered call. (Today: opaque bearer, no expiry/rotation/scope.)
- [ ] **H5 — decide tenant isolation explicitly** (see below) and record the blast-radius choice.
- [ ] **H6 — revocation + detection.** AS token-revocation path; monitor the refresh-rotation race
  (single-use refresh tokens make theft partly self-detecting); anomaly alerts; recover via the
  browser-only floor.
- [ ] **H7 — pad + supply-chain hardening.** CSP/SRI/Trusted Types on pads (croft-pwa already does this;
  the demo pad does not); dependency-audit gate; signed releases through the forced-command deploy channel.
- [ ] **H8 — keep the floor sacred.** No pad may ever *require* the helper; optionality is the backstop
  for all of the above.

**Tenant isolation (H5), the shape of the choice.** Today there is **one shared client identity** for the
whole estate (`account.croft.ing`, one key) — minimal state, maximal blast radius (that one key's
compromise is estate-wide). The isolation options, to decide deliberately rather than by default:
- **Per-product client identity** — each offering (arecipe, stellin, …) gets its own `client_id` +
  its own key. A breach of one is contained to that product. Cost: more keys/registrations ("key sprawl").
- **One identity, per-tenant keys/data-key domains** — keep one `client_id` but partition signing keys
  and session-encryption domains per tenant, so a breach is containable without multiplying registrations.
Isolating **by domain/offering** is exactly right; the tradeoff is isolation (smaller blast radius) vs
simplicity (fewer creds). Recorded as **Open decision 14** below.

---

## Deployment map (accelerators & components)

**atproto / pad-layer accelerators** (fronting the four live pads):

| Pad | Accelerator | Mode | Auth helper | Serverless fallback if accelerator down |
|---|---|---|---|---|
| `arecipe` | cache/index server | **cache** | uses it for authed paths | friends-scoped backendless walk (today) |
| `bluebird` | cache/index server | **cache** | not required (public reads) | `baseUrl` → `public.api.bsky.app` (default) |
| `stellin` | cache/index server | **index** from start | yes (viewer-aware serving) | friends-feed / Bluesky-direct (less good) |
| `pdsview` | none near-term | — | optional | PDS-direct (today) |

Cache fqdns: `bluebird-cache.croft.ing` (bluebird is `bluebird.croft.ing`) and `cache.arecipe.app`
(arecipe is `arecipe.app` — cache under its own domain, same-site). `arecipe` may later move to
**index** mode for its own lexicons only, if a query emerges that the cache cannot serve. Not now.

**drystone / iroh-layer accelerators** (same optional / serverless-floor ethos; deeper design lives
in their own docs — pointers only):

| Component | What it is / accelerates | Serverless floor (works without it) | Pointer |
|---|---|---|---|
| **iroh relay** | deployed QUIC relay for **connectivity only** — NAT traversal / "making the ends meet" (peer discovery is via atproto, not pkarr) | direct P2P / hole-punch between peers | `alpha/experiments/iroh/` (relay lab E0–E9) |
| **MLS history-convergence server** | content-blind "meer"/blind-mirror that helps MLS-group peers converge their append-only history | P2P convergence over iroh | drystone convergence briefs `beta/impl/experiments/`; relay lab E8/E9 |
| **croft-groups** | an **AppView variant**: roster-gated large-group serving = the cache/index engine + a different membership/write policy (atproto-family; listed here for group affinity) | open tier is a zero-decision on-ramp (RUN-16) | `alpha/experiments/appview-infra/GROUPS.md` A.10; factoring = Open decision 13 |

Order (as built): the **iroh relay** landed as a **dev/test relay** (Phase 6, DONE) in **mode A** —
plain HTTP behind Caddy, **no UDP/QUIC** (so **no nftables exception** was needed). QUIC address-
discovery (which would need the relay's own TLS + UDP 7842 + an nftables exception) is a later mode-B
upgrade if we want it. **croft-groups is an AppView variant** — the same cache/index
engine with a different membership/write policy — deployed as its own isolated mini-stack instance on
product need; its code factoring (mode / shared-lib sibling / separate build) is open (Open decision
13). The **MLS convergence server is gated** on drystone's fold/MLS becoming real — last by dependency,
not choice.

**Where this sits.** This Croft server stack is the **next layer above drystone** — the center-free,
peer-symmetric MLS + iroh protocol for group messaging and governance (`beta/drystone-spec/`, Parts
1–2). The iroh relay and MLS convergence server are drystone-transport accelerators; the auth helper
and cache/index server are atproto/pad accelerators. One ethos across both layers. The drystone
protocol itself is specified elsewhere and is not re-argued here.

**Discovery is via atproto, not pkarr — which collapses two layers into one accelerator.** A person's
crypto identity (their iroh `NodeId`) is published as a **record in their own PDS**, resolved by their
Bluesky id: handle → DID (a DNS/PLC lookup) → PDS → the record. So peer discovery is just *a DNS lookup
plus a PDS read* — exactly the reads the **cache/index server already accelerates and caches**. We do
**not** depend on pkarr / the mainline DHT for discovery. That leaves the **iroh relay with a single
job: connectivity — "making the ends meet" when two peers cannot reach each other directly** (NAT
traversal), never discovery. Cohesion win: the atproto cache we build for the pads doubles as the
crypto-id discovery cache for the drystone transport.

---

## Reasoning

- **Why accelerator, not index.** The pads are designed to work with no server; treating backendless
  as a defect contradicts the product. An optional cache lightens the PDS and improves latency
  without making the server load-bearing, and without acquiring canonical state we would then have to
  protect.
- **Why one program, two modes.** Caching and indexing share the entire serve surface and differ only
  in what populates state. A `StateSource` seam lets us build/test both and pick per tenant, rather
  than maintaining two servers or committing globally to the heavier one.
- **Why the auth helper is shared.** Session brokering wants one confidential client identity for the
  estate; per-product confidential clients would multiply key material and fragment sessions.
- **Why the iroh relay goes first.** The first real service after the stub should isolate *infra*
  faults from *application* faults. An off-the-shelf iroh relay is the ideal shakedown: known behavior,
  long-running, resource-hungry — so it exercises the cgroup/telemetry/noisy-neighbor story (the relay
  lab's E5/E6 bets) on real hardware before we take on any invention. It is not on the pads' critical
  path, and it tests whether the kit contract generalizes to a *third-party* binary or needs a thin
  wrapper. The cost it exposes (a bandwidth-heavy relay co-located on one small VPS) is exactly the
  noisy-neighbor case governance exists to bound.
- **Why the auth helper is a deliberate risk, taken second.** Highest-value application win (session
  life across all pads) but least-proven — `authserve` could not exercise the OAuth login leg. Taking
  it as the first *net-new* mini-stack (after the relay proved the infra) means a spike failure
  implicates the mechanism, not the box. *(Fallback if the spike stalls: cache mode first — nearly pure
  assembly — and let the auth helper follow.)*
- **Why Stellin indexes from the start.** It is the one product whose value is world-view discovery
  and (eventually) server-side search — the queries a cache cannot answer. It still works without the
  index (friends-feed / Bluesky-direct), so the serverless floor holds.
- **Why declarative-from-the-start — three idempotent layers, each at its seam.** The box is up, but
  re-clicking drifts. **OpenTofu** owns the **resource** layer (the `ovh` VPS only — kit terraform is
  VPS-only; DNS stays manual in Porkbun by owner preference; R2/Cloudflare out until backups exist).
  **Ansible** owns the **in-box** layer (packages, users, nftables, Caddy, systemd units, cgroup
  limits) — Python-ecosystem, genuinely idempotent, replacing the dropped `bootstrap.sh` (bash
  idempotency is fragile). The **`render.py` generator** produces the unit/vhost artifacts Ansible
  converges. Each layer is idempotent where it is strong; nothing shells into the box imperatively.
- **Why backups are paused now but designed.** Everything is experimental on non-canonical data, so a
  missing recovery point costs nothing yet, and iterating without a backup plane is faster. The
  approach (Litestream→R2 for the index cursor/state) is fixed so enabling is a switch. Gate: before
  any tenant carries real/canonical data.
- **Why governance+telemetry from day one.** Limits/accounting after the fact are how a runaway ingest
  starves a broker's cursor unnoticed. cgroups are free with systemd; the only build is a small local
  reader. (The relay lab's E5 — "cgroup group accounting: isolation + per-group bill for free" — is
  the same bet.)
- **Why single-box.** RUN-07 measured the full firehose at ~357 ev/s with ~64× single-node headroom;
  distribution is premature. Recovery for index tenants is restore-from-R2, not a hot standby.
  Deliberate non-goals: no HA pair, no load balancer, no orchestrator, no metrics stack beyond the
  local telemetry client + `/healthz` + journald + an external pinger.

---

## How it layers (reference)

### The host contract (generalized)

`CONTRACT.md` is the whole interface, and it generalizes beyond data tenants — the auth helper is a
tenant too. Every mini-stack process honors the core contract:

- Flags `--data-dir <path>` and `--listen <host:port>`; nothing else required to start.
- `GET /healthz` → `200 ok` once ready; fast, side-effect-free.
- **All state under the data dir** (no `$HOME`, `/tmp`, or cwd writes) — enables `ProtectSystem=strict`
  with one `ReadWritePaths=` and one-directory destroy/restore.
- Non-root, `NoNewPrivileges=yes`; ports ≥ 1024 (Caddy owns :443).
- A data profile: `--disposable` (cache index, NEVER backed up), `--canonical` (index cursor + the
  auth helper's sessions/keys, backed up), `--blobs` (opaque bytes, rclone-mirrored).

Addenda by role: the cache/index server carries the own-data API addendum (read-only, self-scoping by
verified DID, bounded pagination) and, in index mode, the roster/gate addendum; the auth helper carries
a secrets addendum — the confidential client key is `Zeroize` material, never logged or serialized in
the clear.

### Mini-stacks, not a monolith

A product is a **mini-stack of one or more processes** that honor the contract, deployed and supervised
independently. The auth helper is a shared mini-stack; each cache/index tenant is its own. Containers
are permitted where a stack wants isolation beyond the systemd unit; binary deploy is the default and
neither is forced.

### Languages

Application/service logic is **Rust or Python**; the pads are **web/JS** (exempt). The rule:

- **Rust** — anything operational / with real-time expectations: the cache/index server, the production
  auth broker, the iroh relay (off-the-shelf, already Rust). Shares the `appview-validation` crate
  lineage and crypto/atproto libraries.
- **Python** — general / utility: the `render.py` generator, the contract stub / canary (`stub.py`).
  The default for new utilities **unless** Rust is better for shared-library leverage with the
  server-side components. The telemetry client (Phase 3) lands on Python by this rule (utility, no
  real-time need, no shared-lib win).
- **Web/JS** — the pads only.
- **Bounded, accepted variances** (config + orchestration substrate, not competing app languages):
  **HCL** for OpenTofu (resource layer); **Ansible YAML** for the in-box layer (Python-ecosystem,
  idempotent — the chosen replacement for bash bring-up, Open decision 10); **bash** only for the tiny
  `deploy-receive.sh` forced command + `extract-to-repo.sh`, with **bats** testing it. `bootstrap.sh` is
  **dropped**. Application-logic tests stay in-language (`cargo test` / `pytest`).
- Note: the 2026-07-24 auth-helper **spike** was TypeScript/Node (throwaway); its **production** broker
  is Rust.

---

## Resource governance & telemetry (first-class, from the first unit)

Every generated unit gets, by default:

- **Limits:** `MemoryHigh=` / `MemoryMax=`, `CPUQuota=`, `TasksMax=`, `IOWeight=` — sized per role in
  the manifest (a firehose ingest and a broker have different envelopes).
- **Accounting:** `MemoryAccounting=yes`, `CPUAccounting=yes`, `IOAccounting=yes`, `TasksAccounting=yes`.
- **Isolation:** empty capability set, `ProtectSystem=strict`, `PrivateTmp=yes`; API/read paths add
  `ReadOnlyPaths=` on the data dir so an export cannot write.

**Local telemetry client.** systemd places each unit in its own cgroup v2, so per-process usage is
readable from `/sys/fs/cgroup/system.slice/<unit>/` (`memory.current`, `cpu.stat`, `pids.current`,
`io.stat`) and `systemctl show <unit>`. *(rec:* a small local reader that polls those cgroup files per
unit into a local time-series (SQLite or append-only) with a tiny CLI — cleaner and lower-state than
scraping `/proc`, no new daemon; Open decision 8.*)* Observation also via `systemd-cgtop` and journald.
Full plan: [03-governance-telemetry.md](03-governance-telemetry.md).

---

## Service inventory (illustrative — final shape set in Phase 0)

```
                          Internet  :80 / :443
                                   ▼
                            ┌───────────────┐
                            │     Caddy     │  auto-HTTPS, one vhost per fqdn
                            └───────┬───────┘
        ┌──────────────────────────┼───────────────────────────────┐
        ▼                          ▼                                ▼
  :8001 auth-helper        :8101 <pad>-cache            :8201 stellin-index
  (account.croft.ing)      :8102 <pad>-cache-api        :8202 stellin-index-api
     + relay.croft.ing (UDP/QUIC, off-the-shelf, first real service)
```

| Unit | Kind | Port | Role | Restart |
|---|---|---|---|---|
| `caddy` | long-running | 80/443 | TLS + reverse proxy, one vhost per fqdn | auto; on boot |
| `nftables` | ruleset | — | default-drop; allow 22/80/443 (relay is mode-A behind Caddy — no UDP port) | on boot |
| `iroh-relay` | long-running | **UDP/QUIC** (+ TLS) | first real service; NAT-traversal relay, infra shakedown (off-the-shelf) | `always`, `RestartSec=2` |
| `auth-helper` | long-running | **8001** | shared confidential OAuth broker + session + outbox (canonical) | `always`, `RestartSec=2` |
| `<pad>-cache` | long-running | **810x** | cache-mode serve; miss ⇒ proxy PDS/public AppView (disposable) | `always`, `RestartSec=2` |
| `<pad>-cache-api` | long-running | **810x+1** | own-data API, read-only, self-scoping | `always`, `RestartSec=2` |
| `stellin-index` | long-running | **8201** | index-mode: Jetstream ingest → SQLite index → serve (canonical cursor) | `always`, `RestartSec=2` |
| `stellin-index-api` | long-running | **8202** | own-data API, read-only | `always`, `RestartSec=2` |
| `stellin-index-blob-0` | timer→oneshot | — | `rclone sync blobs/ → R2` — **PAUSED** | timer ~5 min |
| `litestream` | long-running | — | streams **canonical** `*.db` → R2 — index tenants only, **PAUSED** | auto; on boot |

Cache tenants add no backup units. Only index tenants define `litestream` and blob timers, **paused for
now** (guarded no-ops). You author the manifest; the generator emits units, vhosts, and timers with
governance stanzas baked in. The **iroh relay** slots into the same generator but is deployed **first**.

**Restart paths:** crash → `Restart=always`/`RestartSec=2`; reboot → `WantedBy=multi-user.target` /
`timers.target`; new release → `deploy-receive.sh` atomic symlink swap + single-unit restart; config
change → edit `services/<name>.toml` → `make generate` → **Ansible converge** (in-box).

---

## Open decisions (tracked)

1. **Manifest shape / naming** — *scheme DECIDED: role-based subdomains.* Croft-infra and croft-domain
   pads under `croft.ing` (`account.croft.ing` = auth helper, `relay.croft.ing` = iroh relay,
   `bluebird-cache.croft.ing` = bluebird cache); a pad on its own domain gets its services **under that
   domain** (arecipe → `cache.arecipe.app`; Stellin → `index.stellin.app`, same-site). This also keeps
   the contested "Stellin" name off `croft.ing` — the name-clearance itself is decision 6.
2. **First cache pad** — *DECIDED:* cut both in Phase 8; **bluebird validated first**
   (`bluebird-cache.croft.ing`), **arecipe immediately after** (`cache.arecipe.app`).
3. **Auth-helper vs cache first (among net-new mini-stacks)** — owner chose auth helper first (after
   the relay); fallback if its spike stalls is cache-first. Pre-authorized.
4. **arecipe index mode later** — if/when a query emerges that the cache cannot serve. Not now.
5. **Stellin write-path / group forks** (D11, GROUPS.md A.10) — serving is fork-agnostic behind a store
   trait; deferred until Stellin is a real tenant.
6. **Stellin name clearance + service DID** — contested; the name is **already in active use** on
   `stellin.app` (owner-registered; the auth-helper spike pad runs there) and the index tenant will be
   `index.stellin.app`, so the .app domain is settled — but trademark/RDAP **clearance remains the
   owner's legal call**. `run14-A4` self-issued `aud` stands in until a real `did:web:` service DID
   exists. Gates any `app.stellin.*` publication.
7. **Production repo name** — *DECIDED:* `croft-stack` (discovery source stays `appview-infra/kit`).
8. **Telemetry client shape** — small self-rolled cgroup-v2 reader *(rec)* vs off-the-shelf exporter.
   Language = **Python** (per the Languages policy). Decide the shape at Phase 3.
9. **Session TTL fact** — *RESOLVED (spike):* now spec-cited — access ~1h (measured 3599s); refresh
   public ≤ ~2 weeks vs confidential ≤ ~180 days, session possibly unlimited via refresh. Long-run
   survival is being measured over calendar time (daily refresh on the box); mechanism proven.
   One **spec divergence registered**: `token_endpoint_auth_signing_alg` is spec-optional but
   bsky.social rejects the confidential client without it.
10. **Declarative box management** — *DECIDED: two complementary idempotent layers, not either/or.*
    **OpenTofu** owns the **resource** layer — the `ovh` VPS only (the kit terraform is VPS-only; no DNS,
    no R2/Cloudflare — corrected). **Ansible** owns the **in-box** layer (packages, users, nftables,
    Caddy, systemd units, cgroup limits) — Python-ecosystem, genuinely idempotent. The **`render.py`
    generator** still produces the unit/vhost artifacts Ansible converges. **`bootstrap.sh` (bash) is
    dropped** — not a fit (bash idempotency is fragile). **DNS stays MANUAL** (Porkbun by hand).
    **R2/Cloudflare out** until backups are implemented. For the *adopted* box there is nothing to
    `import`: terraform models order-or-read, so we `data.ovh_vps` READ it (`vps_service_name`,
    `place_order=false`) and capture the plan_code/region to reproduce the *next* box.
11. **Backups enable trigger** — paused both modes now; approach fixed (Litestream→R2). Enable gate:
    before any tenant carries real/canonical data.
12. **Drystone-layer order** — *DECIDED + relay DONE:* the iroh relay shipped (Phase 6) as a dev/test
    relay. Contract-fit sub-item *resolved:* the off-the-shelf binary needs **no wrapper** — it runs as
    a plain governed systemd unit fronted by Caddy (mode A), rather than honoring `CONTRACT.md`'s
    healthz/data-dir surface. MLS convergence still gated on drystone's fold.
13. **croft-groups factoring** *(open — scope concretely later)* — it **is an AppView** (cache/index
    engine + different membership/write policy). Deploys as its own isolated mini-stack instance; code
    factoring (mode / shared-lib sibling / separate build) undecided. *(lean: mode-or-shared-libs.)*
14. **Auth-helper tenant isolation** *(open — decide before real users behind the broker)* — one shared
    client identity (today, max blast radius) vs **per-product client identity** (isolate by
    domain/offering, smaller blast each, more key sprawl) vs **one identity with per-tenant
    keys/data-key domains** (containable without multiplying registrations). Drives H1/H3/H5 in the
    "Security hardening" TODOs above. *(lean: isolate high-value offerings; keep low-value on the shared
    client.)* Full reasoning: `discovery/spike/auth-helper/SECURITY.md`.

---

## Provenance & references

- Host kit + contract: `discovery/alpha/experiments/appview-infra/kit/` (`CONTRACT.md`,
  `bootstrap/bootstrap.sh`, `scripts/render.py`, `scripts/deploy-receive.sh`,
  `scripts/extract-to-repo.sh`, `docs/RUNBOOK.md`, `services/*.toml`, `generated/`, `stub/`).
- Kit build + validation: `discovery/alpha/experiments/RUN-15-SUMMARY.md` (fire-drill green);
  tier/publication/sealed models: RUN-16/17/18/19.
- AppView mechanisms proven live: `discovery/alpha/experiments/appview-validation/`
  (`src/bin/{firehose,feed,labeler,sealed,publish,authserve,bootstrap,local}.rs`); caller-identity
  proof and the OAuth-login non-goal: `RUN-14-SUMMARY.md` (EXP-A service-auth JWT).
- Auth helper lineage (session broker / outbox after the shared-origin pivot):
  `discovery/spike/account-kernel/FINDINGS-AND-PIVOT.md`, OPEN-THREADS T55;
  `discovery/alpha/plans/2026-07-22-account-kernel-spike.md`.
- Pads' integration surface: `arecipe/src/social/{comments,interactions}.ts`,
  `bluebird/src/atproto/client.ts`, `croft-pwa/src/atproto/oauth/` + `croft-pwa/client-metadata.json`.
- Name clearance (contested): `discovery/alpha/research/stellin-name-clearance-2026-07.md`.
- Drystone (the layer below): `discovery/beta/drystone-spec/` (Parts 1–2); overview
  `discovery/beta/socialization/drystone-elevator-pitch.md`; convergence design
  `discovery/beta/impl/drystone-design/`, `.../experiments/drystone-convergence-experiment-brief-v3.md`.
- iroh relay lab: `discovery/alpha/experiments/iroh/` (`RELAY-LAB-CONCLUSIONS.md`,
  `RELAY-PLACEMENT-LAB-SPEC.md`; E5 cgroup accounting; E8/E9 relay-vs-meer). Discovery-via-atproto
  vs pkarr: `alpha/research/messaging-solutions-landscape.md`, `public-social-protocols.md`; pkarr
  appears only in `alpha/TEST-PLAN.md` (`PkarrPublisher`), superseded for discovery.
- croft-groups: `discovery/alpha/experiments/appview-infra/GROUPS.md` A.10.
- Backlog anchors: `discovery/alpha/ROADMAP_TODO.md` (appview-infra / OVH items).

atproto/iroh facts cite the FACTCHECK source of truth; the OAuth session-TTL fact (decision 9) is not
yet in a FACTCHECK and must be captured (Phase 7) before it is load-bearing.
