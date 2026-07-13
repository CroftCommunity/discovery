---
name: alt-drive-project
description: "Alt.Drive — encrypted P2P vault project at /home/ubuntu/alt.drive, currently in Phase 0 (design + foundational crypto primitives shipped). Repo at github.com/AltID/alt.drive."
metadata: 
  node_type: memory
  type: project
  originSessionId: c6c6d6d6-dd3d-4f80-8013-ac8831bc9d05
---

Alt.Drive is an E2EE personal vault that mounts as a local folder, syncs P2P between explicitly-paired devices via iroh. macOS-first; Linux/iOS later.

**Why:** Substrate for personal AI memory (Vivian), federated social, etc. — "the filesystem is the API." Honest scope: Proton Drive parity is a 5-year company; minimum-viable Chase-uses-it-daily version is 3-6 months solo.

**How to apply:**
- Repo canonical location: `/mnt/data/alt.drive/` (EBS volume); `/home/ubuntu/alt.drive` is a compat symlink. Remote: `git@github.com:AltID/alt.drive.git` (SSH; key at `~/.ssh/id_secroute`). See [[alt-drive-disk-layout]] for the why.
- Phase 0 = design docs + foundational primitives only. `altdrive-core` has TDD-driven SymKey, secretbox (XSalsa20-Poly1305), kdf (Argon2id KEK). 12 tests. Workspace deps: zeroize, dryoc — **no iroh yet**.
- Phase 0 spikes (`docs/phase-0-spikes.md`) live in throwaway crates `crates/altdrive-spike-*/` and are **TDD-exempt** per [[alt-drive-coding-agents]] discipline.
- DESIGN.md decisions log is the source of truth for crypto choices; threat model is in `docs/threat-model.md`.
- Two-host iroh testing setup, both AWS EC2 same VPC, UDP 2112 pinned for iroh per design decision:
  - `secroute-testing-two` (172.31.19.13, us-east-1b) — the host these memories live on; usually the provider in spike runs
  - `secroute-testing-one` (172.31.43.122, us-east-1c) — peer; usually the fetcher
  - Hostnames are easy to flip; verify with `hostname -I` before assuming which is which.
- **Spike 2 (iroh-blobs hello-world) PASS** as of 2026-06-05: 54-byte payload round-tripped, BLAKE3 verified on receive, VPC direct UDP path. Bigger-scale (5 GB, resume, multi-source) per `docs/phase-0-spikes.md` still pending.

See also: [[alt-drive-author-identity]], [[alt-drive-coding-agents]].
