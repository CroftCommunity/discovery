//! hist-atproto-spike — RUN-HIST-01 Part B: the mechanical seam between the
//! history store's §G reconciliation envelope and an ATProto-PDS-shaped
//! backend.
//!
//! `Serves:` `beta/impl/drystone-design/history-durability.md` (§G envelope,
//! §I member convergence, §J store convergence, §L pruning/checkpoints) and
//! `beta/impl/drystone-design/rbsr-construction.md` (req. 3 stateless
//! responder, req. 5 omission resistance), via the Part A brief
//! `alpha/experiments/HIST-ATPROTO-MATCHUP.md`. Everything here is
//! experiment-grade `Modeled`; no wire encoding is pinned (`[gates-release]`
//! untouched — the rkey padding width and every byte layout in this crate are
//! spike-local choices, documented as such at their definition sites).
//!
//! **The ordering rider (GROUPS.md v2 A.7; matchup §1), carried in code.**
//! Repo commit order, firehose sequence, and rkey sort are delivery
//! artifacts; folding from any of them is a MUST-NOT; convergence ordering is
//! the in-payload `predecessor_digest` chain, only ever. [`delivery`] makes
//! the cursor structurally unreadable by the fold; B4 asserts the rest.
//!
//! **Owner-call register (I9 firewall — surfaced, carried, NOT decided):**
//! - `OWNER-CALL: HS OC-1 pending` — repo ownership: service DID vs per-group
//!   DID (per-group pays the F-HIST-1 enumerability cost; see FINDINGS.md).
//! - `OWNER-CALL: HS OC-2 pending` — reconciliation identity: `entry_digest`
//!   ≡ blob CID vs separate digest carried in the record ([`record`]).
//! - `OWNER-CALL: HS OC-3 pending` — scribe key custody and PLC rotation-key
//!   holders (matchup row 8; live legs README).
//! - `OWNER-CALL: HS OC-4 pending` — envelope posture default, opt-in vs
//!   opt-out per scope (matchup §1 posture dial).
//! - `OWNER-CALL: HS OC-5 pending` — sealed-posture backend shape (matchup
//!   row 11; `lexicons/README.md`).

pub mod car;
pub mod checkpoint;
pub mod delivery;
pub mod envelope;
pub mod fold;
pub mod omission;
pub mod pages;
pub mod record;
pub mod rkey;
