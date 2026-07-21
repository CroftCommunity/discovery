# Build inventory — the Croft ecosystem's actual build state (cross-repo, cross-cut)

date: 2026-07-20

status: living index. A cross-cut snapshot of **what is built, proven, live, designed, or not-yet-built**
across the whole Croft ecosystem, organized by the three legs — the **social-graph baseline**, **Croft
Chat**, and the **ponds / pads** — plus the substrate and proofs beneath them. Re-anchor as the state moves.

**Relationship to the other indexes.** `experiments/MASTER-INDEX.md` covers the *experiments
corpus only* (substrate → integration → re-plant → transport tracks; now folded into `alpha/experiments/`). `ROADMAP_TODO.md` is the open-item
*backlog*. `ECOSYSTEM.md` is the *external* project register. This doc is the **internal build cross-cut**
that spans the sibling apps (`arecipe`, `skylite`) and everything now inside `discovery` (the folded `alpha/experiments/` and `alpha/Proofs/` plus the design docs) — the
thing none of those single indexes shows whole. Product design of record: `../beta/croft/`
(`product-the-garden-of-ponds.md`, `social-graph-as-substrate.md`); build sequencing:
`thinking/app/ponds/build-order.md`.

---

## The three legs (+ what's beneath)

### Substrate / social-graph baseline

| Piece | Where | State |
|---|---|---|
| `local_storage_projection` | `experiments/local_storage_projection` | redb storage + append-only governance fold + derived social-graph projection; mutation-vetted. **Built.** |
| `social-graph-core` (Drystone facade) | `experiments/croft-chat/social-graph-core` | Tenant-agnostic session/identity + groups·members·channels·timeline; thin domain over redb. **Built.** |
| Croft Group engine | `experiments/croft-group` | Lineage-group implementation (crates/). **Built** (the native pond's engine, not yet a user-facing pond). |
| Group crypto proof | `Proofs/lineage-groups`, `lineage-group-model` | Phase-1 crypto gate **GO on real openmls 0.8.1**. **Proven.** |
| Encrypted-local-first proof | `Proofs/encrypted-local-first-atproto` | **Proven.** |
| Design of record | `../beta/croft/social-graph-as-substrate.md`; group tier model → `experiments/appview-infra/{GROUPS,PUBLICATIONS}.md`, RUN-16/17/18 | Spec / settled. |

### Croft Chat

| Piece | Where | State |
|---|---|---|
| `croft-chat` (reference tenant) | `experiments/croft-chat` | ratatui two-pane CLI proving *"the social graph is the substrate; chat is one tenant attached to it."* Layered `social-graph-core` (substrate) → `group-chat-core` (chat tenant) → `croft-chat` (CLI/ports). **Built.** The pattern every pond/tenant should follow. |

### Ponds / pads

| Pond / pad | Where | State |
|---|---|---|
| **arecipe** | `../../arecipe/` → **live at arecipe.app** | Recipe box + meal planner; records in the user's own PDS; shareable public views; the same records render on recipe.exchange. **The first live crop** (atproto/PDS pad, TDD). |
| **skylite** | `../../skylite/` → **live at skylite.croft.ing** | Read-first tended Bluesky window; sponsor/explorer roles; on-device-only mode; installable PWA. **Second live crop** (atproto pad, TDD runs). |
| croft-app-phase0 | `experiments/croft-app-phase0` | App functional core + CLI (from CroftC PR #10). **Built spike** (M1–M6 closed). |
| Garden-of-ponds product | `../beta/croft/product-the-garden-of-ponds.md`, `presence-ritual-and-composed-ponds.md` | Product design of record. |
| Games pond, aggregator ponds, fair-reveal, launch-set | `thinking/app/ponds/` | **Designed / catalogued, not built.** |
| **pdsview** | `CroftCommunity/pdsview` (separate repo) → **live at pdsview.croft.ing** | Standalone SPA/PWA atproto PDS content browser (paste handle/DID → browse repo/collections/records + inline blobs + export .car/.ndjson/.json); zero runtime deps, croft.ing tokens, no live feed. **RUN-01 executed + merged** (phases 0–5, PR #1, built 2026-07-16). A tool-pad sibling. Brief seed: `seeds/pdsview-run-01-instructions.md`. |
| croft.ing landing site | `CroftCommunity/crofting_site` (separate repo) → **live** | Landing + pillar pages; lists arecipe + skylite as the live crops. RUN-01/02/03 executed. |

---

## The two pond lineages (they are at different maturity)

- **atproto / PDS pads** — arecipe and skylite are **live**. This path is proven and shippable and needs
  no deep-link resolver. It rides existing atproto identity + records.
- **iroh-native ponds** (games, the native Croft Group pond, the composable garden) — all gated on the
  **deep-link resolver** by the build-order's own Phase-0 logic (`thinking/app/ponds/build-order.md` §0.2:
  the resolver is *"the single most strategically important component"* — core navigation + the entire
  acquisition model). The resolver is **not built**, and iroh transport is spike-level (relay lab blocked
  on public UDP/NAT ingress).

## Not built yet (the gaps)

- **Deep-link resolver (tier-zero root dependency).** `discovery/site/resolver.py` is the *Drystone
  spec-site* anchor/reference-link tool — **not** the app-side deep-link resolver. No app resolver exists.
- **Aggregator ponds** (Bluesky/Mastodon/Lemmy) — none built.
- **Native Croft Group *pond*** — the *engine* (`croft-group`) and a *chat tenant* (`croft-chat`) exist;
  the user-facing pond product surface does not.
- **iroh integration layer** — spike only (`experiments/alpha/iroh`); relay/NAT path blocked.

---

## Current direction (2026-07-20)

**Organizing, not building.** The chosen next pond to build (when we move from organizing to building) is
an **aggregator pond on the atproto path** — extending the proven arecipe/skylite pad lineage, inheriting a
population rather than facing the empty-room problem, and needing no resolver. Tracked as ROADMAP_TODO E39.
Related surfaced directions: the meetup/community beachhead (D11) and the RSVP+roster primitive reusing the
Smoke Signal strong-reference pattern (dating-fit report rec #2) — an aggregator/events pond is where those
converge. The iroh-native ponds remain gated on the resolver; not this pass.
