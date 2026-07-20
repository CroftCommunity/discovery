//! The role boundary (AP-V4 / P5) — the permanent-red pair.

use crate::records::ReceiptRecord;
use crate::types::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GovernanceBoundaryError {
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

pub fn reject_governance_use(_receipt: &ReceiptRecord) -> Result<(), GovernanceBoundaryError> {
    unimplemented!("P5 GREEN: always Err(ReceiptIsGatewayAttested) — permanent red")
}

pub fn reject_governance_use_id(_id: &ReceiptId) -> Result<(), GovernanceBoundaryError> {
    unimplemented!("P5 GREEN: always Err(ReceiptIsGatewayAttested) — permanent red")
}
