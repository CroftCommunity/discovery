// Reference implementation of the Drystone governance fold, faithful to R1-R4
// and the A12 layered-fold type precedence.
// No production fold was found in this repository (`croftc/upstream-repo`).
// A green run proves the R1-R4+A12 semantics are order-independent; it does NOT
// prove a production implementation. See RESULTS.md.

use std::collections::{BTreeMap, BTreeSet, HashMap, HashSet};
use crate::types::{AuthorityState, Fact, FactId, FactPayload, MemberId, Role};

/// A referenced predecessor was absent from the fold set (R3 gap detection).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GapError {
    /// FactIds that were referenced but not present in the input set.
    pub missing: Vec<FactId>,
}

/// Type-precedence tier for A12 layered fold.
///
/// When concurrent facts (neither causally before the other) compete in the
/// same slot, the fact with the lower tier number wins. Within the same tier,
/// the greatest FactId breaks the tie (R1). Causally-later always beats
/// concurrent regardless of tier.
///
/// Tier 1 (highest priority) .. Tier 5 (lowest priority):
///   1 = SetThreshold, 2 = RemoveMember, 3 = RevokeRole, 4 = GrantRole, 5 = AddMember
fn tier(payload: &FactPayload) -> u8 {
    match payload {
        FactPayload::SetThreshold(..) => 1,
        FactPayload::RemoveMember(..) => 2,
        FactPayload::RevokeRole(..)   => 3,
        FactPayload::GrantRole(..)    => 4,
        FactPayload::AddMember(..)    => 5,
    }
}

/// Fold a set of governance facts into an authority state (R1–R4, A12).
///
/// Returns `Err(GapError)` if any fact references a predecessor absent from
/// the set (R3). Stage 1 sets are complete; call `.unwrap()`. Stage 2
/// checks the error to verify gap detection.
///
/// # R1 — causal order is authoritative
/// For each slot, the causally-maximal facts are found; if exactly one, it
/// wins; if several (concurrent), the greatest FactId breaks the tie.
/// The tiebreak MUST NOT override causal precedence.
///
/// # A12 — type precedence for concurrent cross-type conflicts
/// Among causally-maximal concurrent facts in the same slot, the fact with
/// the lower tier number wins (Tier 1 = SetThreshold beats Tier 5 = AddMember).
/// Within the same tier, R1 FactId tiebreak applies.
/// Causally-later always wins regardless of tier.
///
/// # R2 — cross-slot effects are projections on final sets
/// `RemoveMember(m)` acts as a revoke on every role:m:* slot at tier 2.
/// `effective_roles` is then the projection: (m, r) is effective iff
/// role:m:r resolved to granted AND member:m resolved to member.
/// Computed once on final slots, never incrementally.
///
/// # R3 — no fold-time semantic-validity rejection
/// Operations on absent targets are idempotent no-ops. An absent predecessor
/// is a detected gap, not a tolerated ambiguity.
///
/// # R4 — threshold changes are a special case of R1
pub fn fold(facts: &[Fact]) -> Result<AuthorityState, GapError> {
    let fact_map: HashMap<FactId, &Fact> = facts.iter().map(|f| (f.id, f)).collect();

    // R3: referenced-predecessor gap detection.
    let mut missing: Vec<FactId> = facts
        .iter()
        .flat_map(|f| f.predecessors.iter().copied())
        .filter(|p| !fact_map.contains_key(p))
        .collect();
    if !missing.is_empty() {
        missing.sort_unstable();
        missing.dedup();
        return Err(GapError { missing });
    }

    // Gather all (member, role) pairs mentioned by any GrantRole or RevokeRole
    // so we can cascade RemoveMember into the right role slots (R2).
    let all_role_pairs: BTreeSet<(MemberId, Role)> = facts
        .iter()
        .filter_map(|f| match &f.payload {
            FactPayload::GrantRole(m, r) | FactPayload::RevokeRole(m, r) =>
                Some((m.clone(), r.clone())),
            _ => None,
        })
        .collect();

    // Build slot → [(fact_id, tier)] index (A12: track tier alongside id).
    let mut member_ops: BTreeMap<MemberId, Vec<(FactId, u8)>>         = BTreeMap::new();
    let mut role_ops:   BTreeMap<(MemberId, Role), Vec<(FactId, u8)>> = BTreeMap::new();
    let mut thresh_ops: BTreeMap<Role, Vec<(FactId, u8)>>             = BTreeMap::new();

    for fact in facts {
        let t = tier(&fact.payload);
        match &fact.payload {
            FactPayload::AddMember(m) | FactPayload::RemoveMember(m) => {
                member_ops.entry(m.clone()).or_default().push((fact.id, t));
            }
            FactPayload::GrantRole(m, r) | FactPayload::RevokeRole(m, r) => {
                role_ops.entry((m.clone(), r.clone())).or_default().push((fact.id, t));
            }
            FactPayload::SetThreshold(r, _, _) => {
                thresh_ops.entry(r.clone()).or_default().push((fact.id, t));
            }
        }
    }

    // R2 + A12: each RemoveMember(m) also acts as a revoke on every role:m:* slot
    // at tier 2 (same precedence tier as RemoveMember in the member slot).
    for fact in facts {
        if let FactPayload::RemoveMember(m) = &fact.payload {
            for (pm, pr) in &all_role_pairs {
                if pm == m {
                    role_ops.entry((m.clone(), pr.clone())).or_default().push((fact.id, 2));
                }
            }
        }
    }

    // ── Resolve member slots (R1 + A12) ──────────────────────────────────────
    let mut resolved_members: BTreeSet<MemberId> = BTreeSet::new();
    for (member, ops) in &member_ops {
        let winner = resolve_slot(ops, &fact_map);
        if matches!(fact_map[&winner].payload, FactPayload::AddMember(_)) {
            resolved_members.insert(member.clone());
        }
    }

    // ── Resolve role slots (R1 + A12 + R2 cascade) ───────────────────────────
    // A slot resolves to "granted" only if the winner is a GrantRole fact.
    // RemoveMember and RevokeRole winners both mean "not granted".
    let mut role_granted: BTreeSet<(MemberId, Role)> = BTreeSet::new();
    for ((m, r), ops) in &role_ops {
        let winner = resolve_slot(ops, &fact_map);
        if matches!(fact_map[&winner].payload, FactPayload::GrantRole(_, _)) {
            role_granted.insert((m.clone(), r.clone()));
        }
    }

    // ── R2: effective-roles projection ───────────────────────────────────────
    // (m, r) is effective iff role:m:r = granted AND member:m = member.
    // Computed once on final slots, never incrementally.
    let effective_roles: BTreeSet<(MemberId, Role)> = role_granted
        .into_iter()
        .filter(|(m, _)| resolved_members.contains(m))
        .collect();

    // ── Resolve threshold slots (R1 + A12 = R4) ──────────────────────────────
    let mut thresholds: BTreeMap<Role, (u32, u32)> = BTreeMap::new();
    for (role, ops) in &thresh_ops {
        let winner = resolve_slot(ops, &fact_map);
        if let FactPayload::SetThreshold(_, k, n) = fact_map[&winner].payload {
            thresholds.insert(role.clone(), (k, n));
        }
    }

    Ok(AuthorityState { members: resolved_members, effective_roles, thresholds })
}

/// Resolve a slot under R1 + A12:
///
/// 1. Find causally-maximal facts (causal precedence always wins, R1).
/// 2. Among causally-maximal, prefer lowest tier number (A12 type precedence).
/// 3. Within the same tier, greatest FactId wins (R1 tiebreak for concurrents).
fn resolve_slot(ops: &[(FactId, u8)], fact_map: &HashMap<FactId, &Fact>) -> FactId {
    let ids: Vec<FactId> = ops.iter().map(|(id, _)| *id).collect();
    let maximal = causal_maximal(&ids, fact_map);

    // Find the best (lowest) tier among causally-maximal ops.
    let best_tier = maximal
        .iter()
        .map(|id| {
            ops.iter()
                .find(|(oid, _)| oid == id)
                .map(|(_, t)| *t)
                .unwrap_or(u8::MAX)
        })
        .min()
        .expect("maximal must be non-empty");

    // Within best tier, greatest FactId wins (R1 tiebreak).
    maximal
        .into_iter()
        .filter(|id| {
            ops.iter()
                .find(|(oid, _)| oid == id)
                .map(|(_, t)| *t)
                .unwrap_or(u8::MAX)
                == best_tier
        })
        .max()
        .expect("filtered maximal must be non-empty")
}

/// Return the subset of `ops` not causally dominated by any other op in `ops`.
fn causal_maximal(ops: &[FactId], fact_map: &HashMap<FactId, &Fact>) -> Vec<FactId> {
    ops.iter()
        .copied()
        .filter(|&f| {
            !ops.iter().any(|&g| g != f && is_causally_before(f, g, fact_map))
        })
        .collect()
}

/// Returns true iff fact `a` is in the transitive predecessor closure of `b`.
fn is_causally_before(a: FactId, b: FactId, fact_map: &HashMap<FactId, &Fact>) -> bool {
    let Some(b_fact) = fact_map.get(&b) else { return false; };
    let mut visited: HashSet<FactId> = HashSet::new();
    let mut stack: Vec<FactId> = b_fact.predecessors.clone();
    while let Some(cur) = stack.pop() {
        if cur == a { return true; }
        if visited.insert(cur) {
            if let Some(f) = fact_map.get(&cur) {
                stack.extend_from_slice(&f.predecessors);
            }
        }
    }
    false
}
