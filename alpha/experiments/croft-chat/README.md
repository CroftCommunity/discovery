# croft-chat — integrated Drystone CLI demo

A `ratatui` two-pane CLI chat that demonstrates the **Drystone protocol**,
architected around the protocol/implementation boundary the protocol work
established: *the social graph is the substrate; chat is one tenant attached to
it.*

## Layout

```
croft-chat/                       (this workspace)
├── social-graph-core/   PROTOCOL substrate facade (Drystone) — tenant-agnostic
│                          session/identity + groups · members · channels ·
│                          timeline; thin domain layer over the redb surface.
├── group-chat-core/     TENANT — chat domain only: messages, threads, channels;
│                          Intent / Effect / update / project / view. Depends on
│                          social-graph-core, never reaches around it.
└── croft-chat/          CLI shell / binary — owns the ports (Transport,
                           storage, identity); drives the tenant via apply/pump;
                           two-pane TUI (left graph tree, right chat timeline).
```

The substrate itself is the mutation-vetted `local_storage_projection` crate
(redb storage + append-only governance fold + derived social-graph projection),
path-dep'd from `../local_storage_projection`.

## What it proves

Convergence is proven **locally first** — a shared-directory transport adapter
deliberately shuffles delivery order to demonstrate order-insensitive
convergence (invariant I5) — then over real **iroh-gossip** across the test
nodes by swapping the adapter behind the same payload-blind `Transport` port.

## Status

Under construction. See `plans/2026-06-26-1-plan-integrated-drystone-cli.md` for
the phased build plan (Milestones A–D). Run recipes land in `RUN.md` (P18).

## Running

```sh
RUST_LOG=croft_chat=debug cargo run -p croft-chat
```

(Interactive TUI lands in P11; earlier phases are exercised through tests:
`cargo test`.)
