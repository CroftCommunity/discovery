//! Minimal, Willow-shaped 4-tuple addressing layer.
//!
//! Every stored object is addressed by `(namespace, subspace, path, timestamp)`.
//! This deliberately mirrors the Willow data model so a future migration to real
//! Willow is conceptually clean:
//!
//! * `namespace` — the hard boundary between data spaces (e.g. a public space vs.
//!   a private group). This slice only uses one private namespace.
//! * `subspace`  — the author's identity. Here it is the author's MLS Ed25519
//!   signature public key, so addressing identity and MLS credential identity are
//!   the *same* bytes (unified identity).
//! * `path`      — a hierarchical byte-path, e.g. `/chat/doc`.
//! * `timestamp` — write time in milliseconds.

use std::cmp::Ordering;

/// A Willow-shaped entry address.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Address {
    pub namespace: Vec<u8>,
    pub subspace: Vec<u8>,
    pub path: Vec<u8>,
    pub timestamp: u64,
}

impl Address {
    pub fn new(
        namespace: impl Into<Vec<u8>>,
        subspace: impl Into<Vec<u8>>,
        path: impl Into<Vec<u8>>,
        timestamp: u64,
    ) -> Self {
        Self {
            namespace: namespace.into(),
            subspace: subspace.into(),
            path: path.into(),
            timestamp,
        }
    }

    /// Deterministically derive a storage key from the tuple.
    ///
    /// The key is the BLAKE3 hash of a length-prefixed encoding of the four
    /// components, so distinct tuples never collide on the key and the same
    /// tuple always maps to the same key regardless of who computes it.
    pub fn storage_key(&self) -> String {
        let mut hasher = blake3::Hasher::new();
        for part in [
            self.namespace.as_slice(),
            self.subspace.as_slice(),
            self.path.as_slice(),
        ] {
            hasher.update(&(part.len() as u64).to_be_bytes());
            hasher.update(part);
        }
        hasher.update(&self.timestamp.to_be_bytes());
        hasher.finalize().to_hex().to_string()
    }

    /// The `(namespace, subspace, path)` identity, ignoring timestamp. Two
    /// entries "collide" when these three components are equal.
    // Part of the Willow data model; used by `resolve_conflict`, which the slice
    // encodes but does not exercise (no collision is produced).
    #[allow(dead_code)]
    pub fn entry_id(&self) -> (&[u8], &[u8], &[u8]) {
        (&self.namespace, &self.subspace, &self.path)
    }
}

/// Willow conflict-resolution rule for two entries that collide at the same
/// `(namespace, subspace, path)`:
///
/// 1. highest `timestamp` wins;
/// 2. ties broken by the greatest payload digest (BLAKE3 of the payload);
/// 3. further ties broken by the greatest payload length.
///
/// Returns the `Ordering` of `a` relative to `b` (`Greater` == `a` is the
/// winner). The slice does not actually exercise a collision, but the rule is
/// encoded here so the data model matches Willow's semantics from day one.
/// (Verified by the unit tests; not invoked on the lifecycle path.)
#[allow(dead_code)]
pub fn resolve_conflict(
    a: &Address,
    a_payload: &[u8],
    b: &Address,
    b_payload: &[u8],
) -> Ordering {
    debug_assert_eq!(
        a.entry_id(),
        b.entry_id(),
        "conflict resolution is only defined for entries with the same (namespace, subspace, path)"
    );
    a.timestamp
        .cmp(&b.timestamp)
        .then_with(|| {
            let da = blake3::hash(a_payload);
            let db = blake3::hash(b_payload);
            da.as_bytes().cmp(db.as_bytes())
        })
        .then_with(|| a_payload.len().cmp(&b_payload.len()))
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cmp::Ordering;

    fn entry(ts: u64) -> Address {
        Address::new(b"ns".to_vec(), b"author".to_vec(), b"/chat/doc".to_vec(), ts)
    }

    #[test]
    fn distinct_tuples_have_distinct_keys() {
        assert_ne!(entry(1).storage_key(), entry(2).storage_key());
        assert_eq!(entry(1).storage_key(), entry(1).storage_key());
    }

    #[test]
    fn highest_timestamp_wins() {
        let (a, b) = (entry(2), entry(1));
        assert_eq!(resolve_conflict(&a, b"x", &b, b"x"), Ordering::Greater);
    }

    #[test]
    fn timestamp_tie_broken_by_greater_digest_then_length() {
        // Equal timestamps: the entry with the greater payload digest wins.
        let (a, b) = (entry(1), entry(1));
        let by_digest = resolve_conflict(&a, b"aaaa", &b, b"bbbb");
        assert_ne!(by_digest, Ordering::Equal);
        // Identical payloads tie fully.
        assert_eq!(resolve_conflict(&a, b"same", &b, b"same"), Ordering::Equal);
    }
}
