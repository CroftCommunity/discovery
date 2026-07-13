---
name: alt-drive-coding-agents
description: "The chasemp/coding-agents discipline files referenced by alt.drive's CLAUDE.md live at /home/ubuntu/coding-agents on this machine (Linux); CLAUDE.md previously hardcoded a Mac path that doesn't resolve here."
metadata: 
  node_type: memory
  type: reference
  originSessionId: c6c6d6d6-dd3d-4f80-8013-ac8831bc9d05
---

`/home/ubuntu/alt.drive/CLAUDE.md` `@`-includes four discipline files from `coding-agents`:

- `CLAUDE.md` — top-level discipline
- `agents.md` — agent roster + escalation
- `tdd-guardian.md` — RED → GREEN → MUTATE → REFACTOR cycle (load-bearing for altdrive-core)
- `rust-enforcer.md` — Rust idioms, Zeroize discipline, no-`unwrap`-outside-tests rule

On this Linux box those files live at `/home/ubuntu/coding-agents/<file>`. On the user's Mac they live at `/Users/cpettet/git/chasemp/coding-agents/<file>` — CLAUDE.md is currently pinned to the Linux absolute path. If the repo is opened on the Mac the includes won't resolve and the user will need to swap to a Mac-local path (or switch to a relative path like `../coding-agents/<file>` if both checkouts are siblings).

Related: see [[alt-drive-project]] for repo context.
