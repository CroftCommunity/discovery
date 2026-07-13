# Croft — execution runbook (media round + meer build + faithful follow-ons + conformance)

date: 2026-06-17 · self-contained operational runbook. Paste the one-line kickoff (bottom of this
header) into a fresh session; everything needed to act without rediscovery is below.

> **Kickoff line:** *"Read `experiments/iroh/RESUME-NEXT-SESSION.md` and execute it. Verify the fabric
> (§3) is up, do the box-dependent work first (Workstream A, then meer P0→P1) while it's live, run
> conformance vectors (D) locally in parallel. Commit per repo as chasemp and push `main`; production
> code (B/C/D) is TDD-gated, lab spikes (A/E) are not."*

This is a **runbook, not a discovery exercise** — the state, paths, node quirks, proven command
recipes, and the input→output mapping for every workstream are all pinned below. Trust them, but
verify the fabric is alive (§3) before box work, and re-confirm a path with `ls` only if a command
fails.

---

## 1. Where we are (done — do not redo)

Green-real messaging spine: crypto gate, E2.9–E2.15, AR-1…AR-6, C1–C10, T1/T3/T9/T11, MD-G1…MD-G5 over
live iroh incl. NAT Mac, and the **faithful path** (real Ed25519-signed `lineage-history::Message`
verified for signature + standing over the wire). Model suite `lineage-group-model` = 42/42 incl. S2 +
E2.16. Wire spec `CROFT-PROTOCOL.md` is the spine; `conformance-suite.md` + `open-edges.md` beside it.
Real-time media / meer / geer / governance are **designed** (doc map §6) — this round **builds** them.

## 2. Repos, identity, push, current heads

Under `/Users/cpettet/git/chasemp/CroftC/`. Identity **chasemp / chase@owasp.org**, SSH host
`github-personal`, commit on the Mac, **push `main` each repo** (we push now). Commit-message trailer:
`Co-Authored-By: Claude Opus 4.8 (1M context) <noreply@anthropic.com>`.
Heads at hand-off (pushed, clean): **discovery `52af1dc` · Proofs `5571bd9` · experiments `74ed6ed`**.

## 3. The fabric — the nodes are NOT uniform (verified 2026-06-17)

SSH key `~/Downloads/chase-sandbox-one.pem`, user `ubuntu`. **All x86_64 / Ubuntu 26.04 LTS → binaries
ARE portable between boxes** (build once, push to a sibling). Ephemeral — verify alive on resume; if a
box is gone, the build/sync steps re-create state from the repo (source of truth).

| node | host / SSH IP | internal IP | iface | cores/mem | `/mnt/data` | role notes |
|---|---|---|---|---|---|---|
| **node-1** | `secroute-testing-one` · `54.172.175.109` | `172.31.43.122` | **ens5** | **4c / 15G (fat)** | **yes** | primary **build host** + relay/meer; everything warm here |
| **node-2** | `secroute-testing-two` · `34.207.146.151` | `172.31.19.13` | enX0 | 2c / 7G | yes | second build host; relay-B / peer; warm |
| **node-3** | `ip-172-31-88-18` · `3.84.55.217` | `172.31.88.18` | enX0 | 2c / **3G (smallest)** | **NO** | **not a build host** (no `/mnt/data`); use as a **generator/peer by pushing a prebuilt x86_64 binary to `~`** |
| **node-4** | this Mac | — | — | 14c | (contained `.node4-*`) | **off-VPC, real NAT** — the NAT-path peer |

Same on all: `systemd-run`, `tc`, cgroup **v2**, `bc`, `git`. **NOT installed anywhere:** `ffmpeg`,
`gstreamer`, `cargo`-on-default-PATH (use `bash -lc`). Network: same-VPC direct works; NAT Mac reaches
in **via relay** (`presets::N0`); hole-punch ports 3343/3478 **closed** (E0-NAT, gated). Clocks: assume
NTP; latency-sensitive runs already tolerate it.

## 4. What's already on the boxes (warm — don't rebuild unless changed)

On **node-1 and node-2** (both `/mnt/data/croft-iroh/`):
- `iroh/` — the experiments workspace (iroh `1.0.0-rc.1` / iroh-gossip `0.100`); warm `target/`.
  - `iroh/target/debug/altdrive-spike-lineage-sync` — the MD-G carrier (built).
  - `iroh/crates/altdrive-spike-faithful-sync/ff-target/debug/altdrive-spike-faithful-sync` — faithful path (built); path-deps `/mnt/data/Proofs/lineage-groups`.
- `relay-loadtest/` — standalone (iroh `=1.0.0`), `CARGO_TARGET_DIR=rl-target`; `rl-target/debug/relay-loadtest` (built). Subcommands: `relay`, `responder` (`--secret <64hex>` for stable id), `generate` (`--mode passthrough|matchmaking`), `e2-connect`, `sync-hub`, `sync-member`; `--metrics-port` exposes OpenMetrics on 9090.
- `/mnt/data/Proofs/lineage-groups` — synced (so the faithful crate's path-dep resolves; relative path `../../../../Proofs/lineage-groups` from the crate = `/mnt/data/Proofs/lineage-groups`).
- Leftover `g4-*/g5-*/ff-*` `.addr`/`.log` run artifacts — harmless; `rm` them when tidying.

On **node-3:** nothing (no `/mnt/data`, no checkout). To use it: `scp` a prebuilt x86_64 binary
(relay-loadtest / a media generator) to `~` and run it — it can be a third generator/peer, not a build host.

The Mac builds via `CARGO_TARGET_DIR=.node4-target CARGO_HOME=.node4-cargo` (rc.1 workspace) or
`.ff-target/.ff-cargo` (faithful crate, under `crates/altdrive-spike-faithful-sync`).

## 5. Run mechanics + proven recipes (use these; they work)

- **cargo:** `ssh box 'bash -lc "cd /mnt/data/croft-iroh/<crate> && CARGO_TARGET_DIR=<t> cargo build 2>&1 | tail -3"'`.
- **Sync code:** `tar -cf - <paths> | ssh box 'mkdir -p <dest> && tar -xf - -C <dest>'` (repo is source of truth; rebuild on the box after).
- **Stop a process:** `sudo fuser -k <port>/udp` — **NEVER `pkill -f`** (it self-matches the ssh argv and kills your session). systemd unit: `sudo systemctl stop <unit>`.
- **Gossip/carrier orchestration (proven):** start the **origin detached** on node-1 (`nohup BIN args >log 2>&1 &`), it writes its `self_out` addr to a FILE ~3s in; `scp` that addr to the Mac and joiners; launch joiners fast so windows overlap; the carrier runs `ROUNDS=18` (~38s) then exits; `grep SUMMARY` each log. The **origin sometimes shows empty verdicts** (it exits before peers' broadcasts mesh back; own broadcasts aren't echoed) — read the verdict off the **joiners**.
- **Relay/responder/generate (proven):** relay `--advertise-ip <internal>`; responder `--relay-url https://<internal>:3343 --quic-port 3478`; generate `--mode passthrough` forces relay-only; it prints `ESTABLISHED=…` (stderr) + a JSON summary (stdout, with `relay.rtt_ms_*`).
- **cgroup per-tenant (E5 template `relay-lab-runs/e5-run.sh`):** `sudo systemd-run --unit=<u> -p CPUAccounting=1 -p MemoryAccounting=1 -p CPUQuota=N% <BIN …>`; read `/sys/fs/cgroup/system.slice/<u>.service/cpu.stat` (`usage_usec`) + `memory.current`. `systemctl set-property --runtime <u>.service CPUQuota=N%` to retune live.
- **tc netem, SSH-safe (E6 template `relay-lab-runs/e6-orch.sh`):** shape **only** node-1→node-2 via a prio qdisc + u32 dst filter on node-2's IP (so the Mac's SSH path is unshaped), plus a watchdog that `tc qdisc del` after N s no matter what. Conditions baseline/+delay/+loss/+bw-cap.
- **placement churn (E7 template `relay-lab-runs/e7-orch.sh`):** two relays + a pinned-secret responder re-homed A→B; `e2-connect --assign-relay` shows convergence vs the stale-assignment partition window.
- **The three `relay-lab-runs/e{5,6,7}-*.sh` scripts are your orchestration templates** — copy/adapt for the media round rather than starting from scratch.

## 6. Doc map — WHERE TO LOOK FOR WORK (inputs)

- Wire spec (normative spine): `discovery/crystallized/CROFT-PROTOCOL.md` — §6 governance/geer, §8 transport+media, §9 freshness.
- **Media round design:** `discovery/thinking/realtime-media-over-iroh.md` — RoQ vs MoQ, str0m lines L1/L2/L3, challenges **C1–C4**, **Next experiments**, the feasibility map.
- **Meer build:** `discovery/thinking/meer-superpeer-design.md` — three blind roles, **phases P0–P6**, anti-entrenchment guards.
- Lab spec (E-series, incl. **E10–E12 in §4a**): `experiments/iroh/RELAY-PLACEMENT-LAB-SPEC.md`.
- Conformance: `discovery/crystallized/conformance-suite.md`. Governance: `revocation-authority.md`, `crystallized/principles.md`. Abuse posture: `abuse-resistance-and-the-rave-trap.md`. Geer (gated): `geer-gating-peer.md`. Refs: `research/iroh-realtime-media-references.md`, `str0m-production-readiness.md`. Triage backlog: `open-edges.md`.

## 7. Recording targets — WHAT TO PRODUCE (outputs, every item)

1. **Run/build → verdict** (SUMMARY / measured numbers / manifest / passing tests + exit code; for production code, the proving command output).
2. **Lab/transport** → `experiments/iroh/relay-lab-runs/E{n}-<facet>-2026-06-17/manifest.json` + a `MEASURED` annotation in `RELAY-PLACEMENT-LAB-SPEC.md` + a section in `experiments/iroh/TEST-LOG.md`. **Model proofs** → `Proofs/lineage-group-model` (new `experiments/*.ts` + `report/runner.ts` registration + findings doc). **Code** → the relevant crate under `experiments/iroh/crates/` (TDD).
3. **`discovery/crystallized/proof-ledger.md`** — add/adjust the status row (`green-real` | `green-model` | `characterized` | `design`).
4. **`discovery/crystallized/test-narrative.md`** — the **Why / Tells-us / Means / Open-edges** entry (a proof without a narrative entry is not filed).
5. **Tidy boxes** (`fuser -k`, `systemctl stop`, `rm` stale `.addr/.log`).
6. **Commit per repo (chasemp + trailer) and `git push origin main`.**

---

## 8. Workstreams — each: input doc → produce → exit → nodes

### A — Real-time media round (E10–E12) [boxes; FIRST while fabric is live]
**Look:** `RELAY-PLACEMENT-LAB-SPEC.md §4a` + `realtime-media-over-iroh.md`. **Prereq:** `git clone`
n0's **`callme`** and **`iroh-live`** onto node-1/node-2; `sudo apt-get install -y ffmpeg
libopus-dev libgstreamer1.0-dev gstreamer1.0-plugins-{base,good}` (media deps absent — §3); headless
→ **synthetic Opus source**, no `cpal` capture. **Produce:** per-E manifest + SPEC annotation +
TEST-LOG + ledger/narrative.
- **E10 — RoQ under netem (the C1 unknown — HIGHEST PRIORITY, cheapest).** callme/iroh-roq over node-1↔node-2(+Mac), through the **E6 `tc netem` rig** (reuse `e6-orch.sh`). **Exit:** audio holds to 30% loss with *visible* degradation; estimator converges to the netem cap; quantify whether iroh QUIC CC fights the media estimator. **Nodes:** node-1 (relay/peer) + node-2 (peer) + Mac; node-3 optional extra generator (push binary).
- **E11 — MoQ broadcast lazy fan-out.** `iroh-live` (moq-rs): 1 publisher → N subscribers via a MoQ relay. **Exit:** lazy holds (zero publisher egress until a subscriber attaches); fan-out cost vs N; relay blind; measure the metadata-only scale/admission policy a blind relay can enforce (abuse lever). **Nodes:** node-1 (relay) + node-2 + node-3 (subscribers) + Mac.
- **E12 — blind media-meer SFrame-over-MLS.** Reuse `/mnt/data/Proofs/lineage-groups` (openmls) for per-sender keys. **Exit:** loss-tolerant decrypt + replay-reject; non-member can't derive a key (media `UnauthorizedAuthor`); revoked sender's later frames undecryptable (media MD-G5); meer recovers zero plaintext. **Nodes:** node-1 (meer) + node-2/Mac (peers).

### B — The meer build (P0–P6) [PRODUCTION → TDD; the deep builder work]
**Look:** `meer-superpeer-design.md`. **Produce:** a new crate (likely `experiments/iroh/crates/meer`
or in `Proofs/`), TDD-driven (tdd-guardian + rust-enforcer; `Zeroize` secrets; no prod `unwrap()`); the
anti-entrenchment guards (encrypted-state portability, **stand-up-and-elect-a-replacement**) as
testable requirements; `meer_*` metrics. Phases:
- **P0** skeleton: always-on iroh endpoint + admission (`on_connect` identity) + `meer_*` metrics (reuse relay-lab harness + faithful MLS machinery).
- **P1** **Tier-0 blind message mirror** over the E3 sync workload — asserts no payload key, logs only §3b metadata. **= E9 Tier 0, real. THE first milestone.**
- **P2** bridge mode (straddle 2 namespaces/relays) **= E8**. **P3** Tier-1 + no-mirror + reliability-vs-overlap curve **= rest of E9**. **P4** RoQ SFU role **= E12**. **P5** MoQ relay role **= E11**. **P6** Tier-2 (policy-gated).
**Exit/phase:** named experiment passes + `meer_*` emitted + the relevant anti-entrenchment guard demonstrated. **Nodes:** node-1 (meer) + node-2/node-3/Mac (peers/clients).

### C — Faithful follow-ons [PRODUCTION → TDD]
**Look:** `open-edges.md §1` + `revocation-authority.md` + the faithful crate. **Produce/Exit:**
(1) MLS **key-distribution over the wire** (faithful crate currently models the key registry as agreed
state — make it real over iroh); (2) **threshold revoke-authority as a real k-of-n signature** over the
wire (MD-G5 uses a MAC). Ledger/narrative updated; honesty boundaries in `CROFT-PROTOCOL.md §12` retired
as closed.

### D — Conformance vectors [local; PRODUCTION → TDD]
**Look:** `conformance-suite.md`. **Produce:** the language-neutral JSON vector files for the
green-real/green-model rows (derivations, signing pre-images, fold/thresholds, revocation, C1–C10,
AR-1…AR-6, V1–V9/S2, freshness) under a `conformance/` dir + a runner that diffs a Croft impl against
them. `design`-status rows wait on C. **Exit:** vectors + runner; a fresh build of the model/Rust crates passes them.

### E — Smaller open-edges [spike-class; as time permits]
**Look:** `open-edges.md` "doable now" + `failed-op-response.md`. Items: **metadata-leak-under-failed-ops
spike**; **E6 steady-state goodput + bandwidth-cap**; **E7 churn storm**; **fold cost under churn**.
Spike-class (no TDD gate), recorded like the E-series.

## 9. TDD — which code is which (do not blur)
- **Spike-class (no TDD gate, throwaway, measure-a-decision):** A (E10–E12), E. Same posture as E0–E9 / `altdrive-spike-*`.
- **Production (TDD-gated RED→GREEN→REFACTOR; tdd-guardian + rust-enforcer; `Zeroize`; no prod `unwrap()`):** B (meer), C (faithful follow-ons), D (conformance runner). "It's just plumbing" does not skip the failing test first.

## 10. Gated — do NOT start without the resource/decision
- **geer implementation** — designed (`geer-gating-peer.md`); **needs legal review** (compellability). Design only.
- **S3 / S4** — need design gate **G5**. **T8** — UX decision. **T10** (bsky), **T13** (iOS host), **E0-NAT hole-punch** (public ingress), **E4** (`ipvsadm`) — external resources. Surface; don't attempt.

## 11. Priority / sequencing
1. **E10** — the one live technical unknown, cheap, most-proven via callme; **do first while the boxes are up.**
2. **Meer P0 → P1** — the deep-build foundation (Tier-0 blind mirror = headline).
3. **E11/E12 ↔ meer P5/P4** (media-meer roles; intertwined) — boxes.
4. **Conformance vectors (D)** — local, parallel with box work.
5. **Faithful follow-ons (C)** + **Workstream E** — as time permits.
Box-dependent work first (the fabric is ephemeral); pull every result back to the repo as you finish.

## 12. Done criteria
- **A:** E10/E11/E12 each → verdict + manifest + SPEC annotation + ledger/narrative.
- **B:** meer P0→P1 real (Tier-0 blind mirror passing E9-Tier-0; `meer_*` metrics; blindness + one anti-entrenchment guard asserted); later phases as reached.
- **C/D:** as reached, TDD-driven, with the closed honesty-boundaries retired in the wire spec.
- All: ledger + narrative + TEST-LOG/findings per item; three repos committed clean **and pushed**.
