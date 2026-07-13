# Relay & Placement Lab — Run Manifest

The lab-wide manifest: topology mapping, version pins, and the SG/port plan that every per-run
`manifest.json` references. Per-run dirs (`relay-lab-runs/<experiment>-<timestamp>/`) carry the
experiment-specific params + measured results.

## Decisions (Step 0, 2026-06-16)

- **Topology: MULTIPLEX** the 6–8-node spec onto 3 boxes + Mac (user decision 2026-06-16).
  Relays under test are **cgroup-pinned to identical 2 vCPU / 4 GB slices** so host-size asymmetry
  (node-1 is 4c, node-2/3 are 2c) does not contaminate cross-shard comparison — and this gives E5's
  per-process accounting for free. Caveat (accepted): co-located generators understate the relay
  wall; if an E0/E1 number looks generator-bound, move that generator to the Mac/another box and
  re-measure before trusting it.
- **Security Group: ALL-FROM-SELF intra-SG** (user opening it; not self-serve — no AWS creds on Mac).
  Public ingress unchanged (SSH only). Gate: do not run cross-box relay experiments until the user
  confirms the rule is live. UDP 2112 alone is insufficient for the relay/ctrl/LVS ports.
- **Driver modality:** the entire lab is driven from the Mac (node-4) over SSH — build/sync/run/
  collect loop from `KICKOFF-PROMPT.md` §"BUILD / SYNC / RUN / COLLECT". Mac is also the off-VPC
  NAT'd generator.

## Version pins (one SHA per run; record any change)

- `iroh = "=1.0.0"` (released; the version `RELAY-PLACEMENT-LAB-SPEC.md` was verified against —
  removes the rc.1 API drift the spec warns about). `iroh-relay = "=1.0.0"`.
- `iroh-blobs = "0.103.0"`, `iroh-gossip = "0.101.0"` (latest; no 1.0 line yet) — only if an
  experiment needs them; E0 needs only `iroh` + `iroh-relay`.
- Pin enforced via `=` requirement + committed `Cargo.lock` (records the registry checksum = the SHA).
- `cargo 1.96.0` on all boxes; kernel `7.0.0-1004-aws`; cgroup **v2**.

## Node → role mapping

```
node-1  secroute-testing-one  4c/15G  /mnt/data EBS   us-east-1c
        → relay-1 (cgroup slice 2c/4G) + gen-1 (rest of box) + LVS director (E4)
node-2  secroute-testing-two  2c/7.7G /mnt/data EBS   us-east-1b
        → relay-2 (cgroup slice 2c/4G) + gen-2 (rest)
node-3  ip-172-31-88-18       2c/3.8G 128G root        us-east-1a
        → ctrl-1: custom DNS origin + pkarr + admit hook + Prometheus (control plane, light)
node-4  this Mac              NAT'd, off-VPC
        → off-VPC generator (forces real hole-punch / relay paths) + lab driver
```

Rationale: node-3 is smallest (3.8 GB) → control plane, not a relay. Relays live on the two
`/mnt/data` boxes (disk headroom for blob stores). The unattached 150 GB EBS in us-east-1a can
extend node-3 if the controller/Prometheus needs it (AZ-locked to 1a = node-3 only).

## Tooling to install when first needed

- `ipvsadm` on the LVS director (node-1) — **absent today**; install before E4.
- `tc` present on node-1 (E6); confirm on node-2 before E6.
- Prometheus on node-3 (or scrape to a file) — set up before the first instrumented run.

## Per-run dir convention (spec §3)

Each run writes `relay-lab-runs/<exp>-<UTCstamp>/manifest.json` with: experiment id, iroh SHA, node
sizes, kernel, cgroup slice params, generator params, plus the scraped §3 metrics and any charts.
