# 08 — Croft the product: the composable garden

date: 2026-06-24

status: synthesis — **DECISION-GATED** (see the banner below). The client architecture is green-real
(proven in Phase 0); the brand and the IP/ownership status are not settled.

> ⚠️ **DECISION-GATED.** Unresolved calls that are the user's: the CroftC Phase-0 IP/ownership status (the
> code was built in a CroftC repo, PR #10, imported byte-identical to `experiments/croft-app-phase0/`, and
> the CroftC-side PR remains open); the product/brand naming (brand-and-voice-notes vs `NAMING.md`); and the
> cooperative funding mechanism (shared with `07`). Surfaced, not resolved.

verification: design-synthesis with proof status carried where it exists (Phase 0 green-real). For iroh
facts, cite the source of truth (`../alpha/seeds/transcripts/raw/atproto-atmospheric-web-iroh-mobile-FACTCHECK.md`).

---

## Theme narrative (overview)

Everything in the corpus until this body was the *protocol / substrate* — lineage-groups, the Drystone P2P
protocol, the iroh transport (`04`). This theme is the distinct **application layer**: "Croft" the product
that rides on that substrate without re-opening it. The organizing image is a user-respecting **utility
garden**. The app does not decide how useful any pond is to a given user — the user shapes their own garden,
and a pond that matters to one person is absent and unmissed for another. Composability *is* the
user-respecting value: a stance, not a feature.

The garden has two kinds of unit. A **pond** is a connection to an existing social ecosystem, kept native
end-to-end with **honest seams** (no fused timeline, no normalized cross-pond model); a **pad** is a small
self-contained app (a game, a tool) that floats in the garden, sandboxed and permission-scoped. The one pond
Croft fully owns is **Croft Group** — private group chat (and later P2P social games) on iroh — which is the
lineage-groups work surfaced. That it slots in with no protocol change is itself evidence the substrate
decisions hold.

The spine is settled and proven in code: a pure Rust functional core (`(state, intent) -> (state,
effects)`, no I/O, WASM-clean) behind a thin imperative shell per platform (CLI, web, desktop, mobile), with
effects performed by the host as data, never as calls. This is the Crux *pattern* adopted slim, captured as
the client-architecture ADR (Accepted 2026-06-22) and demonstrated green-real in Phase 0 (20/20 acceptance
tests). Maintenance minimization is the stated driver: business logic lives once, a new platform is a new
thin `effects.rs`, and behavior cannot drift between surfaces because they share the core verbatim.

Three structural ideas keep the garden coherent as it grows: per-pond domain cores unified by one shared
shell composition layer (not a god-core, not disconnected cores); interaction tiers that treat group size as
three products that share one send button; and a design-criteria quality bar every pond and pad must clear.
The theme carries its open edges honestly — the "serverless" claim wears an asterisk (browser iroh peers are
permanently relayed), the deep-link resolver / ten-second door is tier-zero but cold-install deep-linking is
not privately achievable, and the CroftC IP question, the brand drift, and the cooperative funding mechanism
are all surfaced-but-unresolved.

## Charter — what this theme covers

**In scope.**

- The application/client layer: the garden thesis, the ponds + pads taxonomy, the shared-core/thin-shell
  client architecture, interaction tiers, the design-criteria quality bar, the ponds/games catalog + build
  pathways, the pad security model, the visual system intent.
- **Croft Group as a pond** = lineage-groups *surfaced*. The product consumes the proven protocol; it does
  not re-specify it.

**Out of scope (and where it lives).**

- The proven protocol it consumes (iroh transport tiers, MLS epoch state, the meer, the scoped appview) →
  `04` (the Croft Group pond *is* lineage-groups surfaced; this theme does not re-open it).
- The safety / membership-vs-access *semantics* and the moderation-vs-kid-friendly tension → `06`.
- The cooperative / sustainability mechanism → `07` (this theme states the product-layer dependency only).
- Cross-platform identity provenance and the ephemeral-vs-durable identity split → `05`.

**Boundary calls.**

- **Awareness vs interactivity** is the line that keeps the architecture clean and is *inside* this theme:
  cross-pond awareness (read-only surfacing of one pond's content in another, resolved in the shell via an
  `at://` reference) is expected-now and cheap; cross-pond interactivity (acting in pond A from pond B) needs
  an idiom-translating broker between cores, deferred by default per honest-seams. Pad trust mechanics are a
  product-layer pad criterion here; the underlying attestation/crypto is treated as solved per the user's call.

## 1. The garden thesis (ponds + pads)

> "A pond is a connection to an external social ecosystem (native data, honest seams, attribution); a pad is
> a small self-contained app that runs in the shell (a game, a tool; sandboxed, permission-scoped, vetted).
> Ponds are ecosystems; pads are the lily pads that float in the garden."
> — `thinking/app/design-philosophy.md` §1a

> "Composability is itself the user-respecting value. Not a feature, a stance."
> — `thinking/app/design-philosophy.md` §1a

*Verification:* **settled design thesis** (the Phase-0 substrate — one module behind a port — is green-real).
*Grounds:* naming the thesis converts an emergent architectural accident (modules behind ports → the user
assembles their own garden) into the explicit organizing principle. The garden *test* gates existence:

> Does it "add optional leverage a user can adopt or ignore, without the surface feeling broken either way?"
> — `thinking/app/design-philosophy.md` §1a

*Grounds:* abandoning a pond that fails to earn its place costs nothing structural (it was always one module
behind a port), which is what makes speculative ponds safe to try. The garden test gates *existence*; the
design-criteria bar (§5) gates *behavior*. *(COHESION §18.)*

## 2. Honest seams

> "We do not fuse ecosystems into a shared data model. Each pond keeps its own native model end to end."
> — and — "What we share is the chrome, not the content." — and — "The seam is about experience shape, not
> protocol lineage."
> — `thinking/app/design-philosophy.md` §4

*Verification:* **settled.** *Grounds:* Lemmy and Mastodon both ride ActivityPub but stay separate ponds —
the seam is about experience shape, not plumbing. Honest seams keep ponds native and become, at the pad
layer, a security boundary (§6).

## 3. One shared functional core + thin per-platform shells (the ADR)

A pure `(state, intent) -> (state, effects)` core + a projection (model → view model); the core knows no
platform, the design system knows no protocol; **effects-as-data** (the core emits effect values, never
holds / calls / awaits the port — DECISION 1). Two orthogonal callout axes: **platform** (each shell supplies
its own `effects.rs`) and **implementation** (swappable adapters behind a port — a fixture-fake + real HTTP
for Bluesky; an in-proc fake + later real-iroh for the chat Transport). *Verification:* **Phase 0 green-real**
— 20/20 acceptance tests (A1–D2, P1–P7), built in CroftC PR #10 and imported byte-identical to
`experiments/croft-app-phase0/`. *(COHESION §23 DECIDED: the two-CLIs question settled; prior client work
— `croft-chat-cli` — adapts to it as greenfield growth on an existing port, not a refactor.)*

## 4. Per-pond domain cores unified by the shared shell (decomposition: option C)

Not one god-core (which would couple a Bluesky read-model with an MLS group engine), not two disconnected
cores (which would re-fatten the shells). The group pond is symmetric to the feed pond: add a `group-core` +
a Transport port; the existing `shell` composes both ponds' view models. *Caveat carried:* the chat core is
**richer** than the feed read-model (bidirectional / real-time, MLS epoch state, fork/merge with
reconvergence-policy-per-plane, governance/delegate planes) — the *pattern* transfers cleanly, the *core
content* is substantially more complex. **Awareness vs interactivity** is the clean line: awareness is shell
composition (the message carries an `at://` reference, the shell resolves it via the feed-core's port to a
renderable card, nothing flows back); interactivity is a broker that translates idioms, sitting *between*
cores, never inside a pond core — deferred until a concrete cross-pond action is committed.

## 5. Interaction tiers: "three products, one send button"

> "Group size is not one axis with a power knob — it is **three different products that share a send
> button**, and the architecture changes character at each break."
> — `thinking/interaction-tiers.md`

*Verification:* **settled design thinking** (extends lineage-groups). *Grounds:* **Interactive** (~2 to a few
dozen; live iroh, opt-in presence/typing over gossip); **Quiet-but-large** (presence/typing auto-off above a
threshold, eventual consistency); **Broadcast** (>~1000 or any one-to-many; drop the real-time apparatus,
append-only rolling-forward log — Scuttlebutt-shaped without SSB's immutable-infinite-history flaw, COHESION
§11). Type is chosen at creation, not a switchable mode; guarantees slide automatically with behavior. The
privacy-preserving default is the free one; the convenience (presence/typing) costs something *visible*
(battery), not *hidden* (metadata): "Presence and typing are off by default because they require staying
connected, which uses battery, and we never send them through our servers."

## 6. The quality bar, and the pad security finding

The design-criteria bar adapts WeChat's four-principle skeleton with the gatekeeper mechanics stripped
(friendliness, clarity, convenience, consistency-by-opting-into-the-shared-design-system, accessibility), plus
pond-extra (honest seams, attribution, broker-as-little-as-possible, graceful degradation) and pad-extra
(sandbox, user-authorized permission scopes, vetted-by-default, instant-start, social-graph discovery). The
craft rule:

> "**Nothing ships placed by eye.** Every space, size, color, radius, and duration comes from a token."
> — `thinking/app/design-philosophy.md` §7

The genuinely-ours security finding:

> "CSP alone does NOT contain a webview" — "platforms that serve web code must trust their entire JS supply
> chain, because CSP does not prevent leakage." — and — "Your transport is iroh QUIC, not browser WebRTC. So
> … disabling WebRTC in the webview is pure upside."
> — `thinking/app/ponds/webxdc-security-and-competitive-games.md`

*Verification:* **research finding, action-item flagged** (the Cure53 webxdc audit; Tauri uses the native OS
WebView per FACTCHECK SoT, so per-platform mitigation differs). *Grounds:* since iroh QUIC is the transport,
disabling WebRTC closes the exfiltration hole at zero cost; plus a hard-separate webview context between ponds
(a compromised game cannot reach the social session's tokens/DID keys — honest seams *as* a security
boundary) and an ephemeral per-match pseudonym by default (a game must not read a stable DID).

## 7. The ponds/pads catalog, build pathways, and the fair-reveal primitive

Three inclusion pathways determine *effort* (not fun): **build-fresh** (write it in the Rust core; clean
license; native outcome-record loop); **wrap** (a webxdc-compatible shim — `sendUpdate` / `setUpdateListener`
/ `joinRealtimeChannel` backed by iroh transport — makes the ArcaneCircle catalog wrappable for one layer's
cost); **port** (WebRTC → iroh QUIC transport swap; you are a *better* host than webxdc because you have direct
point-to-point channels, not broadcast-only). One reusable **fair-reveal (commit-reveal) primitive** powers
governance-grade voting + dice fairness + hidden-info games at once — built once, early, ranked above any
single app by leverage; its bridge value is that local real-time voting is "a fun group toy that is also
cooperative-governance infrastructure." *(COHESION §19; license verification flagged for bundle time.)*

## 8. Membership ≠ access, at the product layer

Two orthogonal axes deliberately decoupled: **membership** gates the infrastructure/governance layer (who
owns and votes — members + sponsees), **access** gates the room layer (who can enter/post a given pad). A
stakeholder can hold open a public, even anonymous, door into a specific pad while the pond stays
member-governed — a guest in a room, not a member of the co-op. This keeps the ten-second door without
diluting ownership and softens Sybil exactly where it matters (a guest carries no governance weight). The
*membership semantics* are settled (`06`); the *protocol expression* — an anonymous guest's first iroh-pad
entry hitting the ten-second bar — is the open E11 engineering question.

## 9. The honest "serverless" asterisk, and the tier-zero door

"No application server" holds, but browser iroh peers are **permanently relayed** and direct hole-punch
**falls back to relays** (n0's by default) — connection-bootstrap leans on relays (cite FACTCHECK SoT). Record
it, do not over-claim; this metered infra cost feeds the `07` sustainability question. And the **deep-link
resolver / ten-second door (E11)** is the single most strategically important component, because it is
simultaneously the core navigation UX and the *entire* acquisition model (there is no public discovery by
design). Its URL grammar is `{pond identity + capability} / {app id + version} / {instance + entry context}`,
shareable at three depths; but cold-install deep-linking is **not privately achievable** (Instant Apps /
Firebase Dynamic Links dead; MMPs need fingerprinting) → it resolves to a claim-code "one-more-tap," framed as
a feature.

---

## What this theme establishes (and does not)

**Establishes:** the product is a composable garden of ponds + pads on one proven spine — the
shared-core/thin-shell client architecture is green-real (Phase 0, 20/20); the Croft Group pond is
lineage-groups surfaced with no protocol change (evidence the substrate holds); interaction tiers, the quality
bar, and the build pathways are settled design; and the webxdc security finding turns iroh-as-transport into a
free containment win.

**Does not establish (decision-gated):** the CroftC Phase-0 IP/ownership status (the user's call; the
CroftC-side PR remains open); the product/brand names (brand DRIFT vs `NAMING.md` must be reconciled before
any brand chapter — do not propagate unsettled names into structure); the cooperative funding mechanism
(shared with `07`, "the most important unthought thing"); and the E11 protocol expression (open engineering).
"Serverless" is true only as "no application server," always with the relay asterisk.

---

## Provenance trace

The detailed source-by-source rollup lives at the prior level, in `../alpha/BETA-ROLLUP.md` (theme 08
section): treatments, the excluded unsettled brand/product names and over-claimed "serverless," and the
surfaced decision gates (CroftC IP, brand drift, E11, cold-start, cooperative mechanism).

## Sources (alpha)

- `../alpha/thinking/app/README.md` **[S/INDEX]** · `../alpha/thinking/app/design-philosophy.md` **[S]** ·
  `../alpha/thinking/app/client-architecture-adr.md` **[S]** · `../alpha/thinking/app/design-criteria.md` **[S]**
- `../alpha/thinking/app/build-specs/BUILD-SPEC.md` **[S/U]** · `../alpha/thinking/app/ponds/build-order.md` **[U]** ·
  `../alpha/thinking/app/ponds/games-pond-authoritative-list.md` **[U]** ·
  `../alpha/thinking/app/ponds/webxdc-security-and-competitive-games.md` **[U]**
- `../alpha/thinking/interaction-tiers.md` **[S]** ·
  `../alpha/thinking/membership-vs-access-the-public-door.md` **[S]**
- `../alpha/seeds/transcripts/raw/atproto-atmospheric-web-iroh-mobile-FACTCHECK.md` **[INDEX — cite for iroh]**
- `../alpha/COHESION.md` §18, §19, §23
