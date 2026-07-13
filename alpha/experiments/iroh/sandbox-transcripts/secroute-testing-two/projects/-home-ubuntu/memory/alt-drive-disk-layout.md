---
name: alt-drive-disk-layout
description: "Default 8G AWS AMI root is too small for an iroh build (~300 transitive deps). On both alt.drive nodes, ~/.cargo and alt.drive/target are symlinked to a mounted EBS volume at /mnt/data; cargo build hits 6-10G during release LTO."
metadata: 
  node_type: memory
  type: project
  originSessionId: c6c6d6d6-dd3d-4f80-8013-ac8831bc9d05
---

iroh + iroh-blobs pulls ~300 transitive crates (hickory, hyper, rustls, redb, quinn, h2, etc.). A debug build of `altdrive-spike-iroh` writes ~2.2G into `target/`; release with `lto = "thin", codegen-units = 1` (set in workspace `Cargo.toml`) goes higher during compile. Neither fits on the default 8G AWS AMI root volume after the OS and rustup are installed.

**Why:** the workspace's release profile is tuned for crypto-code audit safety, not compile-time disk footprint. Don't soften `lto`/`codegen-units` to fit a tiny disk — solve it with storage.

**How to apply:**
- On each node, attach an EBS volume (≥ 30G is comfortable; 100G is what we used here) **in the same AZ as the instance** (EBS is AZ-locked — a volume in `us-east-1a` cannot attach to an instance in `us-east-1b`).
- Format ext4, mount at `/mnt/data`, fstab-persist with `nofail,x-systemd.device-timeout=5` so a missing volume never blocks boot. Validate with `sudo findmnt --verify`.
- Layout on this node (`172.31.19.13`):
  - `/mnt/data/alt.drive/` — the repo lives here (canonical location)
  - `/mnt/data/cargo/` — CARGO_HOME (registry + git deps cache)
  - `/home/ubuntu/alt.drive` → `/mnt/data/alt.drive` (compat symlink so tooling/docs that hardcode `/home/ubuntu/alt.drive` still resolve)
  - `/home/ubuntu/.cargo` → `/mnt/data/cargo` (same idea, keeps `~/.cargo/env` working unchanged)
- The peer (`172.31.43.122`, us-east-1c) uses the same pattern with its own EBS volume in us-east-1c. **The AZ topology is three different AZs across the test setup — this box us-east-1b, peer us-east-1c, an unattached 150G volume in us-east-1a.**

**Faster alternative when the peer is just a fetcher**: build release on one node, scp the stripped binary to the peer. Peer skips iroh compile entirely; only needs glibc (iroh uses rustls, not OpenSSL).

Related: see [[alt-drive-project]] for the EC2/VPC topology this lives on.
