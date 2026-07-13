# experiments — Croft code-forward spikes (staged)

Maturity lifecycle `alpha → beta → rc → publish`, aligned stage-for-stage with the `discovery/` and
`Proofs/` repos. Each stage is a self-contained tree with its own linear git history.

- **`alpha/`** — the current spikes: `android-p2p-app`, `appview-validation`,
  `automerge-partial-reconstruction`, `croft-app-phase0`, `croft-chat`, `croft-group`,
  `encrypted-blob-share`, `iroh`, `local_storage_projection`, `mls-replant`, `public-roundtrip`,
  `replant-continuity`. The frozen substrate once `beta/` exists. Per-experiment summaries live in
  `alpha/README.md`.
- **`beta/`** — *(not yet created)* spikes that graduated toward proofs / product; dead-ends stay
  frozen in `alpha/`.
- **`rc/`, `publish/`** — later stages.

Open work across the alpha spikes — every defined-but-unrun experiment, its blockers, and the
recommended order — is catalogued in `alpha/EXPERIMENT-BACKLOG.md`, and sequenced as a dependency
series in `alpha/MASTER-INDEX.md`. Where a green result rests on a stand-in (prototype / scaffold /
weakened assertion) rather than spec-conformant behavior, it is registered in
`alpha/SPEC-DIVERGENCE-REGISTER.md` (grep the code for `SPEC-DELTA`).

Orientation for the whole workspace lives in `discovery/AGENTS.md`. Git identity: chasemp
(`chase@owasp.org`, `github-personal`).
