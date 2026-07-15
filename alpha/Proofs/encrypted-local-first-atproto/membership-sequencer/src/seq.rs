//! The delivery-service sequencer — the explicit role Phase 7 showed is
//! load-bearing. It imposes a TOTAL ORDER on membership commits so the group
//! never forks: it accepts at most one commit per epoch (the first to arrive
//! targeting the current epoch) and rejects the rest, which must catch up and
//! re-submit. It sees only commit messages (membership metadata), never content.

pub enum Submit {
    Accepted,
    /// The epoch already advanced; re-stage against `current_epoch` and resubmit.
    Rejected { current_epoch: u64 },
}

#[derive(Default)]
pub struct Sequencer {
    pub epoch: u64,
    /// The totally-ordered log of accepted commit messages.
    pub accepted: Vec<Vec<u8>>,
}

impl Sequencer {
    pub fn at(epoch: u64) -> Self {
        Self { epoch, accepted: Vec::new() }
    }

    /// Submit a commit generated against `target_epoch`. Accepted iff it targets
    /// the sequencer's current epoch (then the epoch advances); else rejected.
    pub fn submit(&mut self, target_epoch: u64, commit: Vec<u8>) -> Submit {
        if target_epoch == self.epoch {
            self.accepted.push(commit);
            self.epoch += 1;
            Submit::Accepted
        } else {
            Submit::Rejected { current_epoch: self.epoch }
        }
    }
}
