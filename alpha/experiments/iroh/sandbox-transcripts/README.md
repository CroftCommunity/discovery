# Sandbox iroh-spike Claude Code transcripts

Collected 2026-06-15 from the two AWS EC2 sandbox boxes that ran the **Alt.Drive iroh spike**
(the prior P2P testing this `iroh/` directory came from). That testing coordinated via the
`AltID/alt.drive` repo, now being deprecated in favor of living here under
`CroftCommunity/experiments`.

## Hosts

| dir | alias | public IP | VPC IP / AZ | role |
|---|---|---|---|---|
| `secroute-testing-one/` | secroute-testing-one | 54.172.175.109 | 172.31.43.122 / us-east-1c | peer / fetcher |
| `secroute-testing-two/` | secroute-testing-two | 34.207.146.151 | 172.31.19.13 / us-east-1b | provider (memory files live here) |

(Hostnames are easy to flip; the memory notes say to verify with `hostname -I` before assuming.)

## What's here, per host

- `history.jsonl` — prompt history.
- `projects/-home-ubuntu/*.jsonl` — raw Claude Code session transcripts (cwd `/home/ubuntu/alt.drive`).
- `secroute-testing-two/projects/-home-ubuntu/memory/` — the agent's project-memory notes, the
  fastest way to read the testing status (see below).

Credentials (`~/.claude/.credentials.json`, MCP auth cache) were deliberately **not** collected. A
secret scan (private keys / AWS keys / GitHub & Slack tokens / pem material) came back clean; the
only grep hits were a code comment ("passwordless sudo") and a Claude thinking-signature (opaque
base64), neither a credential. RFC1918 VPC IPs and the public OWASP committer email appear but are
not sensitive. These are raw session logs — review before any wider distribution.

## Status of the testing (from the box-two memory notes, 2026-06-05)

- **Alt.Drive** = an E2EE personal vault that mounts as a local folder and syncs P2P between
  explicitly-paired devices via iroh; macOS-first, Linux/iOS later.
- **Phase 0** shipped: design docs + `altdrive-core` foundational crypto (SymKey, secretbox
  XSalsa20-Poly1305, Argon2id KEK), 12 tests. Workspace deps: zeroize, dryoc — **no iroh in core yet**.
- **Spike 2 (iroh-blobs hello-world): PASS** — 54-byte payload round-tripped, BLAKE3 verified on
  receive, over the VPC direct-UDP path (UDP 2112 pinned per design). Two EC2 hosts, same VPC.
- **Still pending** (per `docs/phase-0-spikes.md` on the box): bigger-scale transfer (5 GB), resume,
  multi-source. Also Spike 1 (iroh-docs), 3 (macFUSE), 4 (pairing), 5/6 (write-up + DESIGN update).
- Infra notes worth keeping: iroh + iroh-blobs pulls ~300 transitive crates; a debug build writes
  ~2.2G to `target/` and the default 8G AMI root is too small — both nodes symlink `~/.cargo` and
  `target/` onto a `/mnt/data` EBS volume (EBS is AZ-locked).

This complements the local `iroh/` source: the memory notes are the why/how, the `.jsonl`
transcripts are the full build narrative.
