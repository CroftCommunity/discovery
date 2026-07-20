//! Shared harness for the spike's tests.

#![allow(dead_code)]

use hist_live::budget::{BudgetCaps, Pacer};
use hist_live::live::HttpLeg;
use hist_live::stub::StubLeg;
use std::sync::Arc;

pub fn live_enabled() -> bool {
    std::env::var("HIST_PDS_APP_PASSWORD").is_ok()
        && std::env::var("HIST_PDS_HOST").is_ok()
        && std::env::var("HIST_PDS_LOGIN").is_ok()
}

pub fn stub_gentle() -> Arc<StubLeg> {
    Arc::new(StubLeg::new_with_pacer(Pacer::zero()))
}

pub fn live_gentle() -> Arc<HttpLeg> {
    Arc::new(HttpLeg::from_env(BudgetCaps::GENTLE).expect("live env"))
}
