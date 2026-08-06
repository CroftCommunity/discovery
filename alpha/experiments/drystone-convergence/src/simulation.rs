// Stage 2: seeded deterministic multi-node simulation for gap-detection tests.
//
// Each Node holds a local fact store. Delivery and reconciliation are
// controlled by the test; the simulation is single-threaded and deterministic.

use std::collections::HashMap;
use crate::types::{AuthorityState, Fact, FactId};
use crate::fold::{self, GapError};

pub struct Node {
    pub id:    String,
    facts: HashMap<FactId, Fact>,
}

impl Node {
    pub fn new(id: impl Into<String>) -> Self {
        Node { id: id.into(), facts: HashMap::new() }
    }

    pub fn deliver(&mut self, fact: Fact) {
        self.facts.insert(fact.id, fact);
    }

    pub fn has(&self, id: FactId) -> bool {
        self.facts.contains_key(&id)
    }

    pub fn fact_count(&self) -> usize {
        self.facts.len()
    }

    /// Reconcile: accept every fact from `other` not already held.
    pub fn reconcile_from(&mut self, other: &Node) {
        for (id, fact) in &other.facts {
            self.facts.entry(*id).or_insert_with(|| fact.clone());
        }
    }

    /// Fold the local fact store. Returns GapError if any predecessor is absent.
    pub fn fold(&self) -> Result<AuthorityState, GapError> {
        let facts: Vec<Fact> = self.facts.values().cloned().collect();
        fold::fold(&facts)
    }
}
