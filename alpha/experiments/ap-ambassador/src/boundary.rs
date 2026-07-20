//! The role boundary (AP-V4 / P5) — the permanent-red pair.
//!
//! **Structural** (T-AP5.1): the ambassador crate does NOT and MUST NOT
//! be imported by any R7/governance crate. `ap_ambassador::records::ReceiptId`
//! is a distinct newtype from `attest_family::types::ObjectId`, so at the
//! type layer an ambassador receipt cannot flow into an attest-family
//! antecedent slot by accident. And attest-family's `AntecedentKind` enum
//! is closed at the compile boundary — no ambassador variant exists.
//!
//! **Behavioral** (T-AP5.2): if a caller nonetheless tries to weaponize an
//! ambassador receipt id as governance input, `reject_governance_use`
//! returns a distinct typed error. The attest-family fold ALSO refuses to
//! promote a vouch with a foreign-object-id antecedent (no `AntecedentKind`
//! qualifies), so the boundary is doubly enforced from both sides.

use crate::records::ReceiptRecord;
use crate::types::*;

/// The typed error the ambassador raises when a caller attempts to use an
/// ambassador receipt in a governance context — a co-sign antecedent, a
/// vouch antecedent, an R7 quorum count, etc.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GovernanceBoundaryError {
    /// The ambassador receipt cannot participate in R7/governance. Ever.
    ReceiptIsGatewayAttested,
}

impl std::fmt::Display for GovernanceBoundaryError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            GovernanceBoundaryError::ReceiptIsGatewayAttested => write!(
                f,
                "governance boundary: ambassador receipt is gateway-attested and \
                 cannot participate in R7 / co-sign / vouch antecedent — AP-V4"
            ),
        }
    }
}
impl std::error::Error for GovernanceBoundaryError {}

/// Given a receipt (any receipt from this crate), refuse to permit its use
/// as a governance antecedent. This function returns `Err` unconditionally
/// on any ambassador receipt — that is the point. It is the behavioral
/// half of the permanent-red boundary.
///
/// A caller who has the ID and wants to check "is this ID governance-usable?"
/// gets a hard No. There is no code path that returns `Ok` — deleting the
/// function is the only way to change that, and doing so is the red-flip
/// this boundary pins.
pub fn reject_governance_use(_receipt: &ReceiptRecord) -> Result<(), GovernanceBoundaryError> {
    Err(GovernanceBoundaryError::ReceiptIsGatewayAttested)
}

/// Same, given only the ID.
pub fn reject_governance_use_id(_id: &ReceiptId) -> Result<(), GovernanceBoundaryError> {
    Err(GovernanceBoundaryError::ReceiptIsGatewayAttested)
}
