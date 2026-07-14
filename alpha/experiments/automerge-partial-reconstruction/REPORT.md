# Verification Report: Automerge (Rust crate) partial-reconstruction and snapshot behavior

## 0.7 confirmation — 2026-07-14 (RUN-01 EXP-2)

**CLOSED. All four invariants hold identically on the 0.7 ship target.** The version caveat that
dominated the original report (below) is now retired: `automerge = "0.7"` resolves to **0.7.4** and
builds and runs cleanly on **Rust 1.94.1** (this environment), with no dependency pins and no vendoring.

Method: bumped `Cargo.toml` from `=0.6.1` to `0.7`, deleted the stale `Cargo.lock` so cargo
re-resolved the tree, applied the **two API deltas the README already documented** — `get_changes`
now returns owned `Vec<Change>` (dropped the `.cloned()`), `get_missing_deps` now takes `&self` (the
`load()`ed doc no longer needs `mut`) — and re-ran the same four scenarios in `src/main.rs`.

Result (captured in `run_output.txt`):

| Scenario | Invariant | 0.6.1 | 0.7.4 |
|---|---|---|---|
| **A** (critical) | later-epoch changes with deps withheld held **inert** — `apply` Ok, `messages` absent, 0 heads, 1 missing dep | PASS | **PASS** |
| **B** | withheld deps supplied → full list materializes in causal order | PASS | **PASS** |
| **C** | `save()` → self-contained snapshot; `load()` restores fully, 0 missing deps | PASS | **PASS** |
| **D** | `get_changes` since epoch-1 heads = exactly 3; applying brings the doc current | PASS | **PASS** |

The only observable difference is cosmetic: change hashes differ between versions (`cea08274…` on
0.6.1 vs `e8524485…` on 0.7.4), as expected from serialization changes across the 0.x line — the
*behavioral* invariants are identical. **FALSIFY condition not met.** The late-joiner
partial-reconstruction inertness is now proven on the ship target, not a proxy. Register row
`automerge-0.6.1` moves from "Already-declared caveats" to **Reconciled**.

Everything below this line is the original 0.6.1 report, kept verbatim as provenance for the
toolchain narrative.

---

## Bottom line

All four scenarios pass on the Rust `automerge` crate and match the JavaScript (3.2.6) findings exactly.

The critical one, Scenario A, is unambiguous: when you apply only the epoch-2 changes and withhold their epoch-1 dependencies, the Rust crate holds the changes inert.

`messages` is absent, the document has zero heads, and nothing errors. There is no partial reconstruction. The late-joiner design is validated on the Rust core.

One important caveat on version: this ran on `automerge 0.6.1`, not `0.7`, for a toolchain reason documented in full below. The four behaviors under test are core CRDT invariants of the shared Rust core, so the result is expected to hold identically on 0.7, but 0.7 itself was not compiled here and should be confirmed on a machine with Rust 1.80 or newer before you treat the 0.7-specific result as closed. **[Update 2026-07-14: this confirmation is now done — see the 0.7 confirmation section at the top.]**

---

## Resolved versions

- `automerge` = **0.6.1** (the version actually built and run)

- Target was `automerge = "0.7"`, which resolves to **0.7.4** (or 0.7.0 when pinned). Not built here. See the toolchain section.

- Toolchain: **rustc / cargo 1.75.0** (Ubuntu archive build, `1.75.0+dfsg0ubuntu1-0ubuntu7.4`)

- Supporting crates pinned to keep the tree buildable on 1.75: `sha2 0.10.9`, `uuid 1.4.1`, `getrandom 0.2.17`, `smol_str 0.2.2`, `unicode-segmentation 1.12.0`. These are dependency-resolution pins only and do not touch automerge's logic.

---

## Why 0.6.1 and not 0.7 (toolchain constraint)

This is the single most important caveat in the report, so it is stated plainly rather than glossed.

The build environment had no Rust toolchain preinstalled, and the standard install routes are blocked by the network egress policy:

- `sh.rustup.rs` returns `403 host_not_allowed`.

- `static.rust-lang.org` (where rustup fetches toolchains) returns `403 host_not_allowed`.

- The rustup binary on PyPI installs, but still fetches toolchains from the blocked `static.rust-lang.org`, so it does not help.

- GitHub does not host prebuilt Rust toolchains as release assets (only source tarballs, off-GitHub).

The only Rust obtainable was from the Ubuntu archive (which is allowlisted): **rustc/cargo 1.75.0**.

`automerge 0.7.x` cannot compile on 1.75. Three independent walls, in the order I hit them:

1. **`sha2 = "0.11.0-pre.5"` prerelease dependency.** This resolves to sha2 0.11.0 final, which pulls the RustCrypto edition-2024 chain (`hybrid-array 0.4`, `crypto-common 0.2`). Cargo 1.75 cannot parse `edition2024` manifests. I worked around this by vendoring automerge 0.7.0 and relaxing its sha2 requirement to the stable `0.10` line. The sha2 call site in automerge is trivial and API-identical across 0.10 and 0.11 (`Sha256::new()` / `update` / `finalize` / `.into::<[u8;32]>()`), so this changed the manifest only, not behavior.

2. **`smol_str 0.3.6`** (also edition2024) and **`hexane` / `rustc-hash`** MSRV requirements. Pinnable, and I pinned them.

3. **automerge 0.7.0 itself declares `rust-version = "1.80.0"`.** This is fundamental and cannot be patched away. 0.7 requires Rust 1.80+, full stop.

`automerge 0.6.1` has MSRV 1.70 and depends on stable `sha2 ^0.10`, so it builds on 1.75 after pinning a handful of latest-version transitive deps down to 1.75-compatible releases (the pins listed above). Its `AutoCommit` API is the same shape as 0.7 for everything these scenarios exercise, with two minor signature differences noted below.

**Recommendation:** treat the 0.6.1 result as strong cross-version evidence (the behaviors are CRDT-core invariants, and the JS 3.2.6 package is a WASM wrapper over a 0.x-line Rust core that shows the same results). Confirm on the real `0.7` target by running the exact same `src/main.rs` on a machine with Rust 1.80+, where it should build against `automerge = "0.7"` unmodified apart from the two API notes below. If you want, add `static.rust-lang.org` and `sh.rustup.rs` to the egress allowlist and I can finish the 0.7 run directly.

---

## API surface used (Rust, AutoCommit ergonomic API)

These are the real function names and signatures, which differ from the JS package:

- Capture discrete changes (JS `getAllChanges`): `AutoCommit::get_changes(&mut self, have_deps: &[ChangeHash]) -> Vec<&Change>`. Call with an empty slice for the full history. In 0.6.x this returns borrowed `&Change`, so the program clones into owned `Change`. In 0.7 this returns `Vec<Change>` (owned) — that is API difference #1.

- Apply changes (JS `applyChanges`): `AutoCommit::apply_changes(impl IntoIterator<Item = Change>) -> Result<(), AutomergeError>`.

- Detect buffered/withheld dependencies (no direct JS analog used in the test): `AutoCommit::get_missing_deps(&mut self, heads: &[ChangeHash]) -> Vec<ChangeHash>`. Takes `&mut self` in 0.6.x — that is API difference #2; 0.7 takes `&self`.

- Snapshot (JS `save` / `load`): `AutoCommit::save(&mut self) -> Vec<u8>` and `AutoCommit::load(&[u8]) -> Result<AutoCommit, AutomergeError>`.

- Heads (JS `getHeads`): `AutoCommit::get_heads(&mut self) -> Vec<ChangeHash>`.

- Incremental since a point (JS `getChanges(old, new)`): `AutoCommit::get_changes(&mut self, have_deps: &[ChangeHash])` passing the captured heads. There is also `get_changes_added(&mut self, other: &mut Self)` for the two-document form.

- Reads: the `ReadDoc` trait. `doc.get(ROOT, "messages")` returns `Result<Option<(Value, ObjId)>, _>`; `Ok(None)` is how an absent field surfaces. `doc.length(&list_id)` and `doc.get(&list_id, i)` read list elements. Writes use the `Transactable` trait: `put_object(ROOT, "messages", ObjType::List)` then `insert(&list, idx, "...")`, with `commit()` sealing each change.

---

## Scenario-by-scenario results

### Scenario A — partial application with MISSING dependencies (the critical one)

Applied only changes 3 and 4 (epoch 2) to a fresh document, withholding 1 and 2 (epoch 1).

Observed:

- `apply_changes([c3, c4])` returned `Ok(())`. It does not error.

- `get_missing_deps(&[])` returned **1 hash** (`9a9f896b…`), i.e. the document knows it is waiting on a dependency.

- Reading `messages` returned `Ok(None)` — the field is **absent**, not empty-list, not partial.

- The document had **0 heads**.

Result: **MATCH.** The Rust crate buffers changes whose causal dependencies are missing and renders nothing visible. This is exactly the JS behavior and is the result the whole late-joiner design hinges on. No partial state, no error, no surprise.

### Scenario B — dependencies arrive afterward

Continuing the Scenario A document, applied the withheld changes 1 and 2.

Observed:

- `apply_changes([c1, c2])` returned `Ok(())`.

- `get_missing_deps` dropped to **0**.

- `messages` materialized as `["e1-m0", "e1-m1", "e2-m2", "e2-m3"]`, correct causal order.

Result: **MATCH.** Buffered changes apply automatically once dependencies are satisfied. Behavior is causal, not lossy.

### Scenario C — snapshot via save/load

Saved the complete four-change document, loaded the bytes into a fresh document.

Observed:

- `save()` produced **191 bytes**.

- The loaded document read `["e1-m0", "e1-m1", "e2-m2", "e2-m3"]`.

- `get_missing_deps` on the loaded document was **0** — self-contained, no dependency on any prior history.

Result: **MATCH.** `save` produces a compacted, history-independent snapshot. This validates bootstrapping a new member from a state snapshot.

### Scenario D — incremental changes since a known point

Captured the heads of an epoch-1-only document (changes 1, 2). On a separate copy holding all four changes plus a fifth (`e2-m4`), extracted the changes since those heads and applied them to the epoch-1 document.

Observed:

- epoch-1 heads: **1 hash** (`9a9f896b…`).

- `get_changes(since heads)` yielded exactly **3 changes** (c3, c4, c5).

- After applying, the epoch-1 document read `["e1-m0", "e1-m1", "e2-m2", "e2-m3", "e2-m4"]`.

Result: **MATCH.** Clean incremental sync, exactly the count and contents expected. This validates the steady-state sync mechanism for existing members.

---

## Things worth flagging

- **API ergonomics vs JS.** The two differences a reader should expect when moving this code to 0.7: `get_changes` returns owned `Vec<Change>` in 0.7 (borrowed `Vec<&Change>` in 0.6.x, hence the `.cloned()` in the program), and `get_missing_deps` takes `&self` in 0.7 (`&mut self` in 0.6.x). Neither affects the four results. When you run on 0.7 you can drop the `.cloned()` calls if you want, but leaving them in still compiles.

- **Absent vs empty.** Scenario A's "no partial state" surfaces specifically as `get(ROOT, "messages") == Ok(None)`, an absent field, with zero document heads. It is not an empty list and not an error. Worth knowing for how your code should probe a possibly-incomplete document: check `get_missing_deps` or `get_heads`, do not infer readiness from a list being empty.

- **The 0.7 confirmation is still open.** Restated because it matters: 0.7 was not compiled here. The result is expected to be identical on CRDT-invariance grounds, but if for any reason 0.7 ever showed partial application under missing deps, that would contradict everything above and would be the thing to stop on. The cheapest way to close this is to run `src/main.rs` against `automerge = "0.7"` on Rust 1.80+.

---

## How to reproduce on a 1.80+ machine against real 0.7

1. Set `Cargo.toml` dependency to `automerge = "0.7"` (remove the `=0.6.1` pin and the version-pin lines for sha2/uuid/getrandom/smol_str/unicode-segmentation; they were only needed for the 1.75 build).

2. `cargo run`. Optionally set `AM_VERSION_NOTE` to label the output, or replace the `env!` with a literal.

3. The two API notes above mean the program compiles as-is; the `.cloned()` on `get_changes` results is harmless on 0.7.
