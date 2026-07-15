//! Deterministic survivor selection (invariant I5).
//!
//! When two partitioned branches reconnect, both sides must independently
//! compute the *same* surviving epoch with no negotiation round. The rule is a
//! parameter; the requirement is determinism. The default orders by member
//! count (more members wins), breaking ties by genesis hash — a total,
//! symmetric order, so `select(a, b) == select(b, a)` always.

use crate::ids::GenesisId;

/// A minimal summary of a branch sufficient to choose a survivor.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BranchSummary {
    pub genesis: GenesisId,
    pub member_count: usize,
}

/// The survivor-selection rule. Parameterized so the experiments can vary it;
/// every variant must be a deterministic total order.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SurvivorRule {
    /// More members wins; ties broken by the smaller genesis hash.
    MemberCountThenGenesis,
}

/// Choose the surviving branch. Deterministic and symmetric in its arguments.
pub fn select_survivor(a: &BranchSummary, b: &BranchSummary, rule: SurvivorRule) -> GenesisId {
    match rule {
        SurvivorRule::MemberCountThenGenesis => {
            match a.member_count.cmp(&b.member_count) {
                std::cmp::Ordering::Greater => a.genesis,
                std::cmp::Ordering::Less => b.genesis,
                // Tie: smaller genesis hash wins (lexicographic on bytes).
                std::cmp::Ordering::Equal => {
                    if a.genesis.0 <= b.genesis.0 {
                        a.genesis
                    } else {
                        b.genesis
                    }
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn g(seed: &[u8]) -> GenesisId {
        GenesisId::from_bytes(seed)
    }

    #[test]
    fn more_members_wins_and_is_symmetric() {
        let a = BranchSummary { genesis: g(b"a"), member_count: 5 };
        let b = BranchSummary { genesis: g(b"b"), member_count: 3 };
        let r = SurvivorRule::MemberCountThenGenesis;
        assert_eq!(select_survivor(&a, &b, r), a.genesis);
        assert_eq!(select_survivor(&b, &a, r), a.genesis); // symmetric
    }

    #[test]
    fn tie_breaks_on_genesis_and_is_symmetric() {
        let a = BranchSummary { genesis: g(b"aaaa"), member_count: 4 };
        let b = BranchSummary { genesis: g(b"bbbb"), member_count: 4 };
        let r = SurvivorRule::MemberCountThenGenesis;
        let winner = select_survivor(&a, &b, r);
        assert_eq!(winner, select_survivor(&b, &a, r));
        let expected = if a.genesis.0 <= b.genesis.0 { a.genesis } else { b.genesis };
        assert_eq!(winner, expected);
    }
}
