# automerge-partial-reconstruction

Verifies the partial-reconstruction and snapshot behavior of the Rust `automerge` crate —
the load-bearing invariant for the late-joiner / partial-sync design: a node that receives
only later-epoch changes (whose dependencies are withheld) must **not** materialize a partial
document.

Answers "what is actually true?" against the real crate. Four scenarios, all PASS:

- **A (critical)** — apply only epoch-2 changes with epoch-1 deps withheld: `apply_changes`
  returns `Ok(())`, `get_missing_deps` reports the withheld hash, `messages` is absent, zero
  heads, no error. Changes held inert; no partial reconstruction.
- **B** — supply the withheld deps: the full list materializes in causal order.
- **C** — `save()` produces a self-contained snapshot; `load()` restores fully, zero missing deps.
- **D** — `get_changes` since epoch-1 heads returns exactly the 3 later changes; applying them
  brings the doc current.

Matches the JavaScript (`automerge` 3.2.6) findings exactly.

## Run

```
cargo run    # prints the four scenarios; see run_output.txt for captured output
```

## Version caveat (open gap)

Built and run on **`automerge` 0.6.1**, not the shipping **0.7** target. The original session
had no Rust toolchain and the only obtainable compiler was 1.75 (egress policy blocked
`sh.rustup.rs` / `static.rust-lang.org`); `automerge` 0.7 declares `rust-version = "1.80.0"`,
a hard MSRV wall, on top of a prerelease `sha2 0.11` edition-2024 dependency. 0.6.1 (MSRV 1.70,
stable `sha2 ^0.10`) builds cleanly.

The four behaviors are core CRDT invariants of the shared Rust core, so the result is expected
to hold identically on 0.7 — but **0.7 was not compiled here**. To close the gap, run the same
`src/main.rs` against `automerge = "0.7"` on Rust 1.80+. Two API notes for 0.7: `get_changes`
returns owned `Vec<Change>` (0.6.1 returns `Vec<&Change>`), and `get_missing_deps` takes `&self`
(0.6.1 takes `&mut self`). Full toolchain narrative and reproduction steps are in `REPORT.md`.

## Provenance

Unlike the sibling experiments, this did not originate as a `croftc/SecurityPolicy` PR — it was
produced in a Claude.ai web sandbox verification session and imported here from
`discovery/chainvalidation.zip`. No `PR-CONVERSATION.md` / `CODING-TRANSCRIPT.md`; the session's
deliverable is `REPORT.md`.
