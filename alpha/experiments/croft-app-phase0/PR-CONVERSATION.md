# PR-CONVERSATION — CroftC SecurityPolicy PR #10 (Croft app Phase 0)

> **Verbatim provenance** for the imported code, captured per PLAYBOOK §2 step 6.
> Source: `croftc/SecurityPolicy#10` — *"Add app-design experiment: Phase 0 (design docs +
> functional core / CLI shell)"*. Read with the `cpettet_croftc` gh account.
>
> **Import note (A8 / IP-ownership):** this code was built in a **CroftC repo** and imported to
> `chasemp/CroftCommunity` `experiments/croft-app-phase0/` on 2026-06-22 at the user's direction.
> The invention-assignment / CoI consideration vs. the Head-of-Product-Security role is the user's
> decision (ROADMAP_TODO A8), exercised by directing this import. Paper trail preserved here.
>
> - **PR:** https://github.com/croftc/SecurityPolicy/pull/10
> - **State:** OPEN (not merged) · **created** 2026-06-21 · **author** cpettet_croftc (Chase Pettet)
> - **Head branch:** `claude/experiments-pcl2ym` · **base:** `main` · **+7106 / −0, 87 files**
> - **Placed at:** `experiments/croft-app-phase0/` — verified **byte-identical** (`diff -rq`, empty).
>   Only the PR's `experiments/` subtree was taken; the SecurityPolicy repo's own root `.github/`,
>   `Publish/`, `README.md` were **excluded** (upstream plumbing, per §2 step 3).

---

## PR description (verbatim)

An app-design experiment under `experiments/` — design docs plus the Phase 0 implementation built to
spec.

### Docs
- `design-philosophy.md` — the *why* (functional core / imperative shell, honest-seam "ponds", token
  discipline, test-first).
- `BUILD-SPEC.md` — the *what* and *in what order* for Phase 0, with acceptance tests.

### Code (`experiments/`, Cargo workspace)
- `crates/core` — pure `(state, intent) -> (state, effects)`; no I/O, no async, no clock; WASM-clean.
  **20 acceptance tests pass** (behavior A1–D2, projection P1–P7).
- `crates/bluesky` — native post type (DECISION 2) + async port trait (consumed by the shell, never
  the core — DECISION 1) + fixture-backed fake (normal/empty/error modes).
- `crates/cli` — imperative shell: intent/effect runtime loop, effects-via-port, text renderer for
  every view state. Runs fully offline.

### Milestones
| Milestone | Status |
|---|---|
| M1–M3 (core + projection, test-first) | ✅ done, 20 tests green |
| M4 (port + fake) | ⚠️ code complete; needs the **real recorded fixtures** (now present — see below) |
| M5 (CLI) | ⚠️ loop + all render states verified offline; happy path needs fixtures |
| M6 (real Jacquard adapter) | ⏳ deferred |

### Deliberate gap (as stated at PR creation)
Per BUILD-SPEC §0/§6, Bluesky fixtures must be **real recorded `getTimeline` responses, never
fabricated**. They weren't in the repo at creation, so `crates/bluesky/fixtures/` was empty and
`pond open` failed *honestly* rather than serving invented data.

🤖 Generated with [Claude Code]

> **Update at import time (2026-06-22):** the head branch now contains real recorded fixtures —
> `timeline_page_1.json` (16.9KB), `timeline_page_2.json` (17.8KB), `timeline_empty.json`. These are
> **real recorded public `getTimeline`/`getAuthorFeed` responses** (real DIDs incl.
> `did:plc:z72i7hdynmk6r22z27h6tvur` = bsky.app, real public handles, real CIDs) — satisfying the
> no-fabrication rule. They are public social posts, not private PII. M4/M5 happy path is now lit;
> M6 (live Jacquard adapter) remains deferred.

### Also present beyond the PR body's description
The placed subtree includes more than the body's core/bluesky/cli list — it carries the **Phase-0
shell stack** built to the design system: `crates/design` (tokens + primitives + token-contract
test), `crates/shell` (layout/slots/pinning), `crates/web` (Leptos/WASM app + a snapshot harness and
~20 PNG visual-regression snapshots), and `crates/desktop` (Tauri wrapper). The CodeRabbit summary
confirms the feed core, offline playback, real HTTP adapter, CLI shell, and tests.

---

## Carried-forward findings (do not lose)

### 1. License-compliance flags (cycode-security scanner) — CroftC-policy-scoped

Two non-permissive-license findings on `experiments/Cargo.lock`, both transitive deps of the Leptos
web stack. **These are CroftC-internal-policy findings** (they cite CroftC Confluence/legal review);
on chasemp/CroftCommunity infra the *CroftC policy* no longer applies, but the **underlying license
facts remain relevant** and were resolved/annotated in-PR:

- ⚠️ **`webpki-roots` 1.0.8 — CDLA-Permissive-2.0** (`Cargo.lock:585`/`594`). Flagged
  non-permissive by the scanner. Per the app-design dialogue, **CDLA-Permissive-2.0 *is* permissive**
  (the scanner's category is conservative); the Phase-0 adapter reads the public unauthenticated
  `getAuthorFeed` so the dep is in the real path. No action needed for the experiment; note the fact.
- ⚠️ **`r-efi` 6.0.0 — tri-licensed `MIT OR Apache-2.0 OR LGPL-2.1-or-later`** (resolved by the
  author): *"we use it under the permissive MIT/Apache-2.0 terms (the LGPL is one option in the
  disjunction, not a requirement). UEFI-target-only transitive dependency (via `getrandom`), never
  compiled for any target we build — wasm32 (web), macOS/Linux (desktop/CLI). No LGPL code linked or
  shipped. Documented openly per build-spec §0a."* (`#cycode_ignore_non_permissive_license_use`.)

Relation to the standing **MPL-2.0 license gate (ROADMAP_TODO A1)**: distinct — A1 is about
`hpke-rs` for the lineage-groups crypto; these are web-stack transitive deps. Both belong to the same
*license-discipline* theme. ⚠️ For the experiment on chasemp infra, no license gate blocks; record
the deps so a later move to a project repo re-checks them under the destination's policy.

### 2. CodeRabbit review nitpicks (8 inline, advisory)

- `design-philosophy.md:216` — *"written-down shortcut"* is undefined jargon; clarify which Phase-0
  deviation is permitted (where the adapter is wired) vs. the hard rule (core never calls the port).
- `design-philosophy.md:524` (DECISION 5) — places high burden on the CLI fake as the sole
  error-injection mechanism; quick-win clarification suggested.
- `design-philosophy.md:451` — nitpick.
- Code nitpicks: `crates/bluesky/src/adapter.rs:63`, `crates/bluesky/src/fake.rs:90`,
  `crates/bluesky/tests/parsing.rs:57`, `crates/cli/src/executor.rs:34`, `crates/cli/src/main.rs:95`.

All advisory (CodeRabbit "Nitpick"/"Minor"); none blocking. Left as-is in the imported snapshot
(byte-identical to the PR); fix if/when the code graduates to a project repo.

### 3. Security scanner

`cycode-security` ran and surfaced **only the two license findings above** — no secret/credential or
vulnerability findings. Consistent with the in-session scan at import (no `.env`, no live secrets, no
committed build artifacts/`.so`/`target`).

### 4. Status carried forward
- **Done:** M1–M3 (pure core + projection, 20 acceptance tests green — A1–D2, P1–P7); the shell/
  design/web/desktop stack; real recorded fixtures (M4/M5 happy path lit).
- **Deferred:** M6 — the real Jacquard (atproto Rust client) adapter wired to live Bluesky.
- The spine held to spec: DECISION 1 (async port in `bluesky`, consumed by the shell, never the
  core), DECISION 2 (native post type in the model), DECISION 4 (cursor-bearing states carry the
  cursor), the no-fabricated-fixtures rule. (See the design-imperative + app dialogues for the
  derivation; this code is their executable Phase 0.)

---

## Coding-transcript linkage (PLAYBOOK §2 step 7)

This code was generated by Claude Code (branch `claude/experiments-pcl2ym`). No separate verbatim
coding transcript was provided; the **design reasoning** behind it is the body of dialogues filed in
`discovery/seeds/transcripts/raw/` on 2026-06-20→22 — principally:
- `croft-app-portdecision-review-2026-06-21.md` (the DECISION 1–5 derivation this code implements),
- `croft-app-design-dialogue-2026-06-20-to-22.md` (the app/client-layer body),
- `croft-architecture-design-dialogue-2026-06-20.md` (the local-first/provenance foundation).

The as-built `BUILD-SPEC.md` / `design-philosophy.md` in this directory are the spec the code was
written against (kept verbatim from the PR). These may differ from the further-developed copies in
`discovery/thinking/app/` — see the import README and COHESION for the drift note.
