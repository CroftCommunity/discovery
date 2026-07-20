//! The interval fold — P3.
//!
//! Follow opens an interval; Undo Follow closes it; re-Follow opens a
//! second. Ordering by antecedents and the fold, never by received-at
//! metadata.

use crate::records::ReceiptRecord;
use crate::types::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Interval {
    pub open: ReceiptId,
    pub close: Option<ReceiptId>,
}

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct FollowerRoster {
    _inner: (),
}

impl FollowerRoster {
    pub fn intervals(&self, _actor: &ActorId, _target: &str) -> &[Interval] {
        unimplemented!("P3 GREEN")
    }

    pub fn pairs(&self) -> Box<dyn Iterator<Item = (&ActorId, &str)> + '_> {
        unimplemented!("P3 GREEN")
    }

    pub fn is_currently_following(&self, _actor: &ActorId, _target: &str) -> bool {
        unimplemented!("P3 GREEN: serve-gate at causal position")
    }

    pub fn open_count(&self, _actor: &ActorId, _target: &str) -> usize {
        unimplemented!("P3 GREEN")
    }
}

pub fn fold_roster<'a, I>(_records: I) -> FollowerRoster
where
    I: IntoIterator<Item = &'a ReceiptRecord>,
{
    unimplemented!("P3 GREEN: fold Follow/Undo pairs into interval roster")
}
