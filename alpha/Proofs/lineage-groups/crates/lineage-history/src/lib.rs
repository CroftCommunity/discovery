//! `lineage-history` — per-branch message history.
//!
//! History is *just data*: it never merges into one canonical transcript.
//! Interleaving two forked branches by timestamp produces noise ("six tapes
//! playing in a room"), so this crate offers **no** such API. Each branch is an
//! append-only, signed log; reconciliation is *consensual backfill* — a branch
//! is gifted to another lineage member and absorbed as a *separate, navigable*
//! branch, never spliced into an existing one.
//!
//! Invariants exercised here:
//! * **I7 — history never corrupts.** No operation mutates or reorders an
//!   existing branch's messages; backfill only *adds* a branch.
//! * **I8 — backfill verifiability.** A branch is absorbed only if every
//!   message verifies against its author's signature *and* the branch shares a
//!   genesis with the recipient's lineage. Unverifiable history is rejected.
//! * **I9 — fold/unfold is lossless and inert.** Folding hides a branch from
//!   the daily view without deleting it; a folded branch yields no visible
//!   messages (no ambient pressure) and unfolds to full context.
//!
//! NOTE (Phase 2 scoping, recorded in PHASE_2_FINDINGS.md): the thesis names
//! Automerge for this layer. Automerge's value is *in-branch* concurrent-edit
//! convergence; the thesis-critical properties under test here (no cross-branch
//! interleave, verifiable consensual backfill, lossless fold) do not require a
//! CRDT — and are arguably *stronger* without a merge mechanism. Automerge
//! integration for concurrent in-branch editing is deferred, not assumed.

use std::collections::BTreeMap;

use lineage_core::dag::Lineage;
use lineage_core::ids::{Did, GenesisId};
use lineage_core::keys::{Sig, SigningIdentity};

/// A single signed message on a branch.
#[derive(Debug, Clone)]
pub struct Message {
    pub author: Did,
    pub seq: u64,
    pub branch: GenesisId,
    pub payload: Vec<u8>,
    pub sig: Sig,
}

impl Message {
    /// Canonical bytes that are signed: binds author, position, branch and
    /// payload so a message cannot be replayed onto another branch/position.
    pub fn signing_bytes(branch: GenesisId, seq: u64, author: &Did, payload: &[u8]) -> Vec<u8> {
        let mut buf = Vec::new();
        buf.extend_from_slice(b"msg-v1");
        buf.extend_from_slice(&branch.0);
        buf.extend_from_slice(&seq.to_le_bytes());
        buf.extend_from_slice(author.0.as_bytes());
        buf.push(0);
        buf.extend_from_slice(payload);
        buf
    }
}

/// An append-only, foldable log for one branch.
#[derive(Debug, Clone)]
pub struct BranchHistory {
    pub branch: GenesisId,
    msgs: Vec<Message>,
    folded: bool,
}

impl BranchHistory {
    pub fn new(branch: GenesisId) -> Self {
        Self {
            branch,
            msgs: Vec::new(),
            folded: false,
        }
    }

    /// Append a message signed by `author`. Returns the new message.
    pub fn append(&mut self, author: &SigningIdentity, payload: &[u8]) -> &Message {
        let seq = self.msgs.len() as u64;
        let bytes = Message::signing_bytes(self.branch, seq, author.did(), payload);
        let msg = Message {
            author: author.did().clone(),
            seq,
            branch: self.branch,
            payload: payload.to_vec(),
            sig: author.sign(&bytes),
        };
        self.msgs.push(msg);
        self.msgs.last().unwrap()
    }

    pub fn len(&self) -> usize {
        self.msgs.len()
    }

    pub fn is_empty(&self) -> bool {
        self.msgs.is_empty()
    }

    pub fn messages(&self) -> &[Message] {
        &self.msgs
    }

    /// Append a pre-built message verbatim, bypassing signing. Intended for
    /// tests/tools that need to construct forged or malformed history to
    /// exercise the backfill rejection paths.
    pub fn push_raw(&mut self, msg: Message) {
        self.msgs.push(msg);
    }

    // --- fold / unfold (I9) -------------------------------------------------

    pub fn fold(&mut self) {
        self.folded = true;
    }

    pub fn unfold(&mut self) {
        self.folded = false;
    }

    pub fn is_folded(&self) -> bool {
        self.folded
    }

    /// Messages visible in the daily view. A folded branch is inert: it shows
    /// nothing and generates no pressure, while remaining fully present.
    pub fn visible(&self) -> &[Message] {
        if self.folded {
            &[]
        } else {
            &self.msgs
        }
    }
}

/// Why a backfill was rejected.
#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum BackfillError {
    #[error("donor branch does not share a genesis with the recipient's lineage")]
    ForeignGenesis,
    #[error("message {seq} from {} failed signature verification", .author.redacted())]
    BadSignature { seq: u64, author: Did },
    #[error("a branch with this genesis is already present")]
    AlreadyPresent,
    #[error("message at index {index} claims seq {seq} (gap or reorder)")]
    NonContiguous { index: u64, seq: u64 },
    #[error("message author {} never held standing on this branch", .author.redacted())]
    UnauthorizedAuthor { author: Did },
}

/// A per-person local store: a set of navigable branches keyed by genesis.
/// There is deliberately no operation that interleaves branches.
#[derive(Debug, Default)]
pub struct HistoryStore {
    branches: BTreeMap<GenesisId, BranchHistory>,
}

impl HistoryStore {
    pub fn new() -> Self {
        Self::default()
    }

    /// Get (creating if absent) the local branch for `genesis`.
    pub fn branch_mut(&mut self, genesis: GenesisId) -> &mut BranchHistory {
        self.branches
            .entry(genesis)
            .or_insert_with(|| BranchHistory::new(genesis))
    }

    pub fn branch(&self, genesis: GenesisId) -> Option<&BranchHistory> {
        self.branches.get(&genesis)
    }

    pub fn branch_count(&self) -> usize {
        self.branches.len()
    }

    /// **Consensual backfill (I7/I8).** Absorb a `donor` branch held by another
    /// member of the same lineage. Verifies (a) the donor branch shares a
    /// genesis with `my_branch` per the `lineage`, and (b) every message
    /// verifies against its author via `verify`. On success the donor is added
    /// as a *separate* branch — existing branches are never touched or
    /// reordered. Unverifiable history is rejected, not taken on faith.
    pub fn backfill_import<F>(
        &mut self,
        donor: &BranchHistory,
        my_branch: GenesisId,
        lineage: &Lineage,
        verify: F,
    ) -> Result<(), BackfillError>
    where
        F: Fn(&Did, &[u8], &Sig) -> bool,
    {
        if !lineage.shares_lineage(donor.branch, my_branch) {
            return Err(BackfillError::ForeignGenesis);
        }
        if self.branches.contains_key(&donor.branch) {
            return Err(BackfillError::AlreadyPresent);
        }
        // Verify every message before absorbing anything. A valid signature is
        // necessary but NOT sufficient: we also require the log to be contiguous
        // (no silent gaps/reorders) and each author to have actually held
        // standing on this branch's lineage — otherwise a well-signed message
        // from a non-member, or a tampered ordering, would slip through.
        for (i, m) in donor.messages().iter().enumerate() {
            if m.branch != donor.branch || m.seq != i as u64 {
                return Err(BackfillError::NonContiguous {
                    index: i as u64,
                    seq: m.seq,
                });
            }
            let bytes = Message::signing_bytes(m.branch, m.seq, &m.author, &m.payload);
            if !verify(&m.author, &bytes, &m.sig) {
                return Err(BackfillError::BadSignature {
                    seq: m.seq,
                    author: m.author.clone(),
                });
            }
            if !lineage.standing(&m.author, donor.branch) {
                return Err(BackfillError::UnauthorizedAuthor {
                    author: m.author.clone(),
                });
            }
        }
        // Absorb as a distinct, navigable branch (no interleave).
        self.branches.insert(donor.branch, donor.clone());
        Ok(())
    }
}
