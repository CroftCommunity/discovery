# 08 — Croft the product: a composable garden rooted in the social graph

date: 2026-06-24 · reframed 2026-06-26 (social-graph-as-substrate)

status: synthesis — **DECISION-GATED** (see the banner below). The client architecture is green-real
(proven in Phase 0); the brand and the IP/ownership status are not settled.

> ⚠️ **DECISION-GATED.** Unresolved calls that are the user's: the CroftC Phase-0 IP/ownership status (the
> code was built in a CroftC repo, PR #10, imported byte-identical to `experiments/croft-app-phase0/`, and
> the CroftC-side PR remains open); the product/brand naming (brand-and-voice-notes vs `NAMING.md`); and the
> cooperative funding mechanism (shared with `07`). Surfaced, not resolved.

verification: design-synthesis with proof status carried where it exists (Phase 0 green-real). For iroh
facts, cite FACTCHECK SoT.

---

## Theme narrative (overview)

Everything in the corpus until this body was the *protocol / substrate* — the Drystone social graph,
lineage-groups, the iroh transport (`04`, and the Drystone protocol spec). This theme is the distinct
**application layer**: "Croft" the product that rides on that substrate without re-opening it.

**The foundational move — and the correction this theme is built on — is that the social graph is the bottom
of the product, and chat is one tenant on it, not the floor.** Most messaging apps put the chat thread at
the bottom and bolt games, calls, and shared media onto it; the thread becomes both the conversation *and*
the index of everything a group ever did, and it is bad at the second — a game pollutes the thread, can't be
pinned, can't be managed long-term. Croft inverts the pyramid: the durable thing is the **group** — a stable
social object with its own identity and history — and chat, games, calls, and shared media are **co-equal
siblings attached to it**, none nested inside another. A chat can end while the group persists; you can start
a fresh chat with the same group and the games and photos are still there, because the functional group is
distinct from any one chat but determinate. The group is the durable index; the thread goes back to being
only a thread. This is the **"equal in rights, not capabilities" social graph** of the substrate, surfaced
as a product.

On that substrate sits a user-respecting **composable garden** — and the garden grows *from the graph*. The
app does not decide how useful any pond is to a given user; the user shapes their own garden, and a pond that
matters to one person is absent and unmissed for another. Composability *is* the user-respecting value: a
stance, not a feature. The garden has two kinds of unit. A **pond** is a connection to an existing social
ecosystem, kept native end-to-end with **honest seams** (no fused timeline, no normalized cross-pond model);
a **pad** is a small self-contained app (a game, a tool) that attaches to a group as one of its siblings,
sandboxed and permission-scoped. The owned social experience — private group chat, and later P2P social
games — is the **first cultivation of the group substrate** (the lineage-groups work, surfaced) [working
name *Croft Group*, pending brand reconciliation — see the decision-gated banner]. That it slots in with no
protocol change is itself evidence the substrate decisions hold.

The graph must be **load-bearing and invisible at the same time** — foundational as architecture, almost
absent as experience. The user lives "me and these people, and the things we do together," never
"administering a graph." The group's identity is **not its member set** (it has a stable identity that
survives membership change, with a presentation name that is shareable but locally overridable); groups form
*implicitly* (start a chat and a group quietly exists) or *explicitly* (make a group, then attach a chat and
a game) and the two are indistinguishable once they exist; and a group becomes durable-and-rediscoverable by
an affirmative act (a "sticky" gesture), with casual ones living and fading without the user ever curating.

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

- The application/client layer: **the social-graph-as-substrate product shape** (the durable group; chat/
  games/calls/media as co-equal siblings attached to it; group identity ≠ member set; the implicit/sticky/
  pruned group lifecycle; the load-bearing-but-invisible-graph UX), the garden thesis, the ponds + pads
  taxonomy, the shared-core/thin-shell client architecture, interaction tiers, the design-criteria quality
  bar, the ponds/games catalog + build pathways, the pad security model, the visual system intent.
- **The owned social experience** (private group chat + later P2P social games) = lineage-groups *surfaced*
  as the first cultivation of the group substrate. The product consumes the proven protocol and the
  Drystone social graph; it does not re-specify them.

**Out of scope (and where it lives).**

- The proven protocol it consumes (iroh transport tiers, MLS epoch state, the meer, the scoped appview) →
  `04` (the Croft Group pond *is* lineage-groups surfaced; this theme does not re-open it).
- The safety / membership-vs-access *semantics* and the moderation-vs-kid-friendly tension → `06`.
- The cooperative / sustainability mechanism → `07` (this theme states the product-layer dependency only).
- Cross-platform identity provenance and the ephemeral-vs-durable identity split → `05`.

**The access floor (a charter line, not a feature).** Participation must be possible on a bare phone —
CPU, memory, storage, and an internet connection — with **no purchased infrastructure and no always-on
node required**. This is a refusal, and it constrains the product rather than decorating it: the
optional-broker and degraded-no-broker tiers exist partly to honor it, so that a user who buys nothing
still has standing, only a more visibly degraded experience — graceful degradation, never loss of
standing. Every pond and pad in the garden inherits this floor; anything that *requires* paid infra to
have standing fails it.

**Boundary calls.**

- **Awareness vs interactivity** is the line that keeps the architecture clean and is *inside* this theme:
  cross-pond awareness (read-only surfacing of one pond's content in another, resolved in the shell via an
  `at://` reference) is expected-now and cheap; cross-pond interactivity (acting in pond A from pond B) needs
  an idiom-translating broker between cores, deferred by default per honest-seams. Pad trust mechanics are a
  product-layer pad criterion here; the underlying attestation/crypto is treated as solved per the user's call.

## 1. The social graph is the substrate; the garden grows from it

> "The durable thing is the group, not the chat. Chat is one tenant; games, calls, and shared media are
> co-equal siblings attached to the same group. A chat can end while the group persists."

*Verification:* **settled design thesis** (the substrate it rides — the Drystone social graph and the
Phase-0 client substrate, one module behind a port — is green-real). *Grounds:* this inverts the pyramid
most apps build. When the chat thread is the bottom, it is forced to be both the communication medium and
the index of everything the group ever did, and it is bad at the second: a game launched into a thread
pollutes it and can't be pinned, history can't be managed, and ending a thread destroys the group — so
zombie threads live forever and membership and governance stay welded to a conversation. Putting the
**group** at the bottom and hanging chat/games/calls/media off it as siblings dissolves all of that at once:
each activity gets its own surface, the thread is only a thread again, and a group's durability never
depended on any one chat. This is the product expression of the substrate's relationship graph — equal in
rights, not capabilities — surfaced.

### 1.1 The durable group, made invisible

The group is a **first-class, durable object with a stable identity that is not its member set** — the same
group survives members joining and leaving, and its presentation name is shareable but **locally
overridable** (one identity, many faces). Two paths bring a group into being and are indistinguishable once
it exists: **implicit** (start a chat and a group quietly forms behind it) and **explicit** (make a group,
then attach a chat and a game). A group becomes durable-and-rediscoverable by an affirmative **"sticky"**
act; casual groups live and fade without the user curating, and a group can be deliberately **ended** (and
not silently resurrected). Reusing the same people *suggests* an existing group but never forces it — the
human stays in control of whether old history comes into a new context, which matters acutely in a
family-safety product where "the system grouped me back with someone" is the surprise to avoid.

The discipline that keeps this from becoming a chore is that **the graph is load-bearing and invisible at
the same time.** The user experiences "me and these people, and the things we do together," never
"administering a graph." The one surface the group genuinely needs is its **home / face** — a place you and
these people share, navigable into its attached chats/games/media — and the hardest UX problem in the whole
product is keeping that face from becoming a settings page for a graph node. Entry points are **plural but
convergent**: you can enter through a person, an activity, a chat, or the group's face, all routing to the
same group (many doors, one room).

One honest seam the product layer must respect, inherited from the substrate: **presentation and
association are local, but access and cross-participant group identity need a shared anchor.** Naming,
stickiness, and a user's own view are local projections requiring no agreement; *membership* is a shared
construct, and a member removed from a group stops receiving new content but **cannot be made to un-see what
they already hold** (`06`). The UX must communicate that boundary truthfully — "removed, won't see anything
new" is sayable; "can no longer see what was already shared" generally is not.

### 1.2 The garden — ponds and pads grow from the graph

On the group substrate sits the composable garden:

> "A pond is a connection to an external social ecosystem (native data, honest seams, attribution); a pad is
> a small self-contained app that runs in the shell (a game, a tool; sandboxed, permission-scoped, vetted).
> Ponds are ecosystems; pads are the lily pads that float in the garden."

> "Composability is itself the user-respecting value. Not a feature, a stance."

A **pad is one of a group's siblings** — a game or tool attached to the group alongside its chat, not nested
inside the thread — which is exactly what makes it pinnable, durable, and manageable rather than lost in a
message river. *Grounds:* naming the thesis converts an emergent architectural accident (modules behind
ports → the user assembles their own garden) into the explicit organizing principle. The garden *test*
gates existence:

> Does it "add optional leverage a user can adopt or ignore, without the surface feeling broken either way?"

*Grounds:* abandoning a pond that fails to earn its place costs nothing structural (it was always one module
behind a port), which is what makes speculative ponds safe to try. The garden test gates *existence*; the
design-criteria bar (§5) gates *behavior*.

## 2. Honest seams

> "We do not fuse ecosystems into a shared data model. Each pond keeps its own native model end to end."
> — and — "What we share is the chrome, not the content." — and — "The seam is about experience shape, not
> protocol lineage."

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
`experiments/croft-app-phase0/`.

## 4. Per-pond domain cores unified by the shared shell (decomposition: option C)

Not one god-core (which would couple a Bluesky read-model with an MLS group engine), not two disconnected
cores (which would re-fatten the shells). The **`group-core` is the substrate core** — it carries the
durable group and the social graph (§1) that a group's sibling attachments hang off — and a pond is symmetric
to it: add a domain core + its port; the existing `shell` composes both view models. *Caveat carried:* the chat core is
**richer** than the feed read-model (bidirectional / real-time, MLS epoch state, fork/merge with
reconvergence-policy-per-plane, governance/delegate planes) — the *pattern* transfers cleanly, the *core
content* is substantially more complex. **Awareness vs interactivity** is the clean line: awareness is shell
composition (the message carries an `at://` reference, the shell resolves it via the feed-core's port to a
renderable card, nothing flows back); interactivity is a broker that translates idioms, sitting *between*
cores, never inside a pond core — deferred until a concrete cross-pond action is committed.

## 5. Interaction tiers: "three products, one send button"

> "Group size is not one axis with a power knob — it is **three different products that share a send
> button**, and the architecture changes character at each break."

*Verification:* **settled design thinking** (extends lineage-groups). *Grounds:* **Interactive** (~2 to a few
dozen; live iroh, opt-in presence/typing over gossip); **Quiet-but-large** (presence/typing auto-off above a
threshold, eventual consistency); **Broadcast** (>~1000 or any one-to-many; drop the real-time apparatus,
append-only rolling-forward log — Scuttlebutt-shaped without SSB's immutable-infinite-history flaw). Type is
chosen at creation, not a switchable mode; guarantees slide automatically with behavior. The
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

The genuinely-ours security finding:

> "CSP alone does NOT contain a webview" — "platforms that serve web code must trust their entire JS supply
> chain, because CSP does not prevent leakage." — and — "Your transport is iroh QUIC, not browser WebRTC. So
> … disabling WebRTC in the webview is pure upside."

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
cooperative-governance infrastructure." *(License verification flagged for bundle time.)*

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
**falls back to relays** (n0's by default) — connection-bootstrap leans on relays
(iroh `1.0.0` / DERP→relays rename / Tauri native WebView: cite FACTCHECK SoT). Record
it, do not over-claim; this metered infra cost feeds the `07` sustainability question. And the **deep-link
resolver / ten-second door (E11)** is the single most strategically important component, because it is
simultaneously the core navigation UX and the *entire* acquisition model (there is no public discovery by
design). Its URL grammar is `{pond identity + capability} / {app id + version} / {instance + entry context}`,
shareable at three depths; but cold-install deep-linking is **not privately achievable** (Instant Apps /
Firebase Dynamic Links dead; MMPs need fingerprinting) → it resolves to a claim-code "one-more-tap," framed as
a feature.

## 10. The on-device assistant is strictly optional — and that is what protects the navigation spine

A natural-language assistant ("any travel games?" → ranked internal deep-links plus a one-line orientation)
is an attractive front-end onto the same resolver §9 already builds — intent text in, real catalog links
out, one resolver and one manifest consumed by two front-ends (links others send, language you type). But it
rides on an on-device model, and **there is no single on-device model that can be assumed present** — coverage
is a patchwork (strong on capable Apple devices, narrow on Android flagships, desktop-only and flag-gated in
Chrome, absent in Firefox and mobile web). That coverage reality forces the invariant, and the invariant is
exactly the rule that keeps §9 honest:

- **Detect availability first, on every platform** — and when the model is absent, the assistant simply
  isn't offered. No dead ends, no degraded-but-broken state.
- **Strictly optional and gracefully absent.** It is an accelerant, trivially disabled, never load-bearing.
- **Accelerate but never gate.** **Every path the assistant offers must also be reachable without it** — so
  the menus, verbs, pins, and deep-links of §9 must be a *complete* navigation system on their own. The
  assistant only ranks intent onto links that already exist; it adds no navigation plumbing.

Held this way, uneven model coverage stops being a problem and becomes a bonus on devices that have it. The
deep-link grammar is the spine; the assistant is one optional language adapter onto it, never a second,
privileged way in.

---

## What this theme establishes (and does not)

**Establishes:** the product is built on the **social graph as its substrate** — the durable group is the
bottom of the pyramid and chat/games/calls/media are co-equal siblings attached to it (a chat can end while
the group persists; group identity ≠ member set; implicit/sticky/pruned lifecycle), with the graph
load-bearing but invisible. On that substrate sits a composable garden of ponds + pads on one proven spine —
the shared-core/thin-shell client architecture is green-real (Phase 0, 20/20); the owned social experience is
lineage-groups surfaced with no protocol change (evidence the substrate holds); interaction tiers, the quality
bar, and the build pathways are settled design; the webxdc security finding turns iroh-as-transport into a
free containment win; the free-tier access floor (a bare phone, no purchased infra, no always-on node) is a
charter-level refusal the tiers exist to honor; and the on-device assistant is strictly optional — detect-
first, accelerate-never-gate, every path reachable without it — which is precisely what keeps the deep-link
navigation spine complete on its own.

**Does not establish (decision-gated):** the CroftC Phase-0 IP/ownership status (the user's call; the
CroftC-side PR remains open); the product/brand names (brand DRIFT vs `NAMING.md` must be reconciled before
any brand chapter — do not propagate unsettled names into structure); the cooperative funding mechanism
(shared with `07`, "the most important unthought thing"); and the E11 protocol expression (open engineering).
"Serverless" is true only as "no application server," always with the relay asterisk.
