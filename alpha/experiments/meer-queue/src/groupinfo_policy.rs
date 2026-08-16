//! **The readmission serving policy — applied by every peer, because there is no server.**
//!
//! S19/S20 located the readmission gate: it is **the moment a `GroupInfo` is served**, not the moment
//! of re-entry. Re-entry is *self-admission* — the joiner builds and finalizes its own commit — so
//! there is no request to deny and no permission prompt to show. The only two levers are **who is
//! served** and **what is released**.
//!
//! > **CORRECTED 2026-08-16 (owner), and the correction is architectural, not cosmetic.** This module
//! > was first written as a `GroupInfoServer`, on the reading that §11.6/§11.11's
//! > **history-convergence node** is where the gate lives. **That is wrong under Part 1 §2.4**
//! > (P-Durable-Enablement): *"a Group **MUST NOT** structurally depend on any single persona's
//! > presence to act"*, and the no-helper path **MUST** stay exercised and real. A meer is optional;
//! > everything else is distributed. **There is no chokepoint to gate, by construction.**
//! >
//! > **Every member can produce a `GroupInfo`** — it is an ordinary export from live group state. So
//! > the serving decision is not a *server's* policy but a **policy every peer applies when asked**,
//! > exactly like the merge-time policy, and for the same reason it must be a **group-context rule
//! > agreed in advance** rather than a local judgement.
//! >
//! > **This makes position 1's residual materially worse than the server framing implied**, and the
//! > earlier claim that it becomes "a small enumerable set a community can watch" is **withdrawn**.
//! > The leak surface is the **whole membership**, and it is exactly S20's original framing. S22
//! > measures this rather than asserting it.
//!
//! ## Two dials, not one
//!
//! - **WHO** — [`ServePolicy`] decides whether the requester's lineage is entitled to anything.
//! - **WHAT** — the `want_tree` argument decides whether the released artifact bundles the ratchet
//!   tree. S18/S19 measured the tree to be the actual admission surface: a `GroupInfo` **without**
//!   it proves current group state (§7.4.2's corroboration use) **without** admitting its holder,
//!   which is the one property S19 showed a bare `GroupInfo` otherwise cannot have.
//!
//! ## SPEC-DELTA[groupinfo-serving-standing-stub | stand-in]
//!
//! **The standing chain here is a set of banned lineage ids, and nothing else.** Part 2 §7.3.1
//! specifies the real resolution — a layered operation-type precedence, then causal precedence, then
//! a content-address tiebreak, over a *complete* causal set — and §11.11 item 3 carries
//! gap-completeness as an open beam. **None of that is reimplemented here**, and no test over this
//! module is evidence about it. What is under test is the *shape*: that a serving decision consulting
//! standing at head is sufficient to separate a ban from a dormancy migration, which the key layer
//! provably cannot do (§11.6/§11.8 use the identical removal commit).
//! — Register: `alpha/experiments/SPEC-DIVERGENCE-REGISTER.md`

use std::collections::HashSet;

/// Which position of the readmission dial this server occupies.
///
/// The positions differ **only** here. The cryptography is identical at every one — which is the
/// finding that makes the dial a policy question rather than a protocol question.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ServePolicy {
    /// **Position 0.** Serve anyone who asks. This is what any member does by default before a
    /// group agrees otherwise — and it is where the delivery design sat while describing itself as
    /// stronger.
    Open,
    /// **Position 1.** Serve iff the requester's lineage is not banned at head.
    StandingChecked,
    /// **Position 2.** Serve iff the requester presents a governance-issued token this peer
    /// recognises.
    ///
    /// **Stronger than position 1 in a distributed setting, and the reason is structural.** Position
    /// 1 is a *negative* check: it requires every peer to already know about a ban. Position 2 is a
    /// *positive* check: the requester must present something, and the verifier checks a signature
    /// against governance it already holds. **A negative check fails open on a stale peer; a
    /// positive check fails closed.** With no chokepoint (Part 1 §2.4), that asymmetry decides which
    /// position is actually robust — see S22.
    Vouched(HashSet<Vec<u8>>),
}

/// Why a request was refused. Carries enough to render an honest message to a returner and to
/// distinguish "you are banned" from "you did not present a token".
#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum RefusalReason {
    /// The lineage is banned at the head of the standing chain (§11.8's ceiling comparison).
    #[error("lineage is banned at head (standing chain head {head})")]
    BannedAtHead {
        /// The standing-chain position the decision was resolved at.
        head: u64,
    },
    /// Position 2 only: no recognised governance token accompanied the request.
    #[error("no recognised governance token presented")]
    NoRecognisedToken,
}

/// What the server decided.
#[derive(Debug, Clone)]
pub enum ServeDecision {
    /// Released. `artifact` is a serialized `GroupInfo`, with the ratchet tree bundled iff it was
    /// both requested and released.
    Served {
        /// The serialized `GroupInfo` to hand over.
        artifact: Vec<u8>,
        /// Whether the ratchet tree is bundled in `artifact`.
        with_tree: bool,
    },
    /// Withheld, with a reason.
    Refused(RefusalReason),
}

/// The server's view of the governance chain: which lineages are banned, resolved **at head**.
///
/// **Resolved at head, never over a returner-asserted range** (§11.8) — a returner controls which
/// position it attests to, so scoping the check to that range would let a lineage banned later
/// re-enter by attesting earlier.
#[derive(Debug, Default, Clone)]
pub struct StandingChain {
    banned: HashSet<Vec<u8>>,
    head: u64,
}

impl StandingChain {
    /// An empty chain at position zero.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Record a ban and advance the head.
    pub fn ban(&mut self, lineage: Vec<u8>) {
        self.banned.insert(lineage);
        self.head += 1;
    }

    /// Is `lineage` banned as of head?
    #[must_use]
    pub fn is_banned_at_head(&self, lineage: &[u8]) -> bool {
        self.banned.contains(lineage)
    }

    /// The chain position decisions are resolved at.
    #[must_use]
    pub fn head(&self) -> u64 {
        self.head
    }
}

/// A peer that may be asked for a `GroupInfo`, and the policy it applies when asked.
///
/// **Not a server.** Every member holds live group state and can export a `GroupInfo`, so every
/// member is one of these. The policy is what a group agrees in advance that its members will do;
/// it has no privileged enforcement point and cannot acquire one without violating Part 1 §2.4.
pub struct ServingPeer {
    policy: ServePolicy,
    standing: StandingChain,
    with_tree: Vec<u8>,
    bare: Vec<u8>,
}

impl ServingPeer {
    /// A peer at `policy`, holding the two forms of the current `GroupInfo` it can export.
    ///
    /// Both forms are held because they answer different questions: the bare form proves current
    /// group state for corroboration, the tree-bundled form additionally admits its holder.
    #[must_use]
    pub fn new(policy: ServePolicy, with_tree: Vec<u8>, bare: Vec<u8>) -> Self {
        Self {
            policy,
            standing: StandingChain::new(),
            with_tree,
            bare,
        }
    }

    /// Record a ban in this peer's standing view.
    ///
    /// **A peer's standing view and its group view propagate on different paths** (§11.8: the
    /// governance chain is separate from the epoch chain), so a peer can hold current group state
    /// with a stale standing view. **Every member is such a peer**, so this gap is not a property of
    /// some small serving tier — it is the membership-wide residual S20 named and S22 measures.
    pub fn ban_at_head(&mut self, lineage: Vec<u8>) {
        self.standing.ban(lineage);
    }

    /// Refresh the group state this server holds.
    pub fn refresh(&mut self, with_tree: Vec<u8>, bare: Vec<u8>) {
        self.with_tree = with_tree;
        self.bare = bare;
    }

    /// This peer's standing view.
    #[must_use]
    pub fn standing(&self) -> &StandingChain {
        &self.standing
    }

    /// Decide whether to serve `requester`, and what to release.
    ///
    /// `want_tree` is a *request*, not a grant: a peer may hold a policy of never bundling the
    /// tree, in which case the bare artifact is released and `with_tree` in the decision is `false`.
    /// Callers must read the returned flag rather than assume their request was honoured.
    #[must_use]
    pub fn serve(&self, requester: &[u8], want_tree: bool) -> ServeDecision {
        match &self.policy {
            ServePolicy::Open => {}
            ServePolicy::StandingChecked => {
                if self.standing.is_banned_at_head(requester) {
                    return ServeDecision::Refused(RefusalReason::BannedAtHead {
                        head: self.standing.head(),
                    });
                }
            }
            ServePolicy::Vouched(tokens) => {
                if !tokens.contains(requester) {
                    return ServeDecision::Refused(RefusalReason::NoRecognisedToken);
                }
                if self.standing.is_banned_at_head(requester) {
                    return ServeDecision::Refused(RefusalReason::BannedAtHead {
                        head: self.standing.head(),
                    });
                }
            }
        }

        ServeDecision::Served {
            artifact: if want_tree {
                self.with_tree.clone()
            } else {
                self.bare.clone()
            },
            with_tree: want_tree,
        }
    }
}
