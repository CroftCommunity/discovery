//! Content-blind ingest + scheduled read-grant for the card/guestbook spike (E43).
//!
//! Two mechanics, both net-new relative to the proven substrate:
//!
//! 1. **Content-blind ingest.** [`Ingest::append`] takes an already-encrypted contribution, checks
//!    the bearer capability (the unguessable link), content-addresses the ciphertext, and stores it
//!    via a [`WriteTarget`] port. The service has no key, no `open`, and no dependency on the
//!    `card-seal` crate, so "the service cannot read what it stores" is a compile-time fact, not a
//!    convention (prove it: `cargo tree -p card-service`, no AEAD crate on the normal edges).
//!
//! 2. **Scheduled read-grant.** [`Reveal::offer`] withholds the (still-encrypted) records from the
//!    recipient until a logical reveal time AND the viewer is the recipient. Both failure conditions
//!    return the *same* opaque [`Withheld`], so the gate leaks neither existence nor which condition
//!    failed (the RUN-14 EXP-B "one flat refusal" property). Time is an INJECTED logical value, never
//!    a wall clock (the Croft no-shared-clock discipline).
//!
//! The [`WriteTarget`] port stands in for the atproto PDS write (`createRecord` over DPoP OAuth with a
//! per-collection scope). The live adapter is out of scope for a hermetic run (blocked on creds and
//! network); only the in-memory fake runs here.

/// Opaque encrypted bytes. The service handles these; it never holds the key to open them.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Ciphertext(pub Vec<u8>);

/// A content address over the ciphertext (blake3 hex). Stable id; tamper-evident.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct RecordRef(pub String);

/// An atproto-style collection NSID, e.g. `ing.croft.card.entry`.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct Collection(pub String);

/// The bearer capability carried by the card link (the write/append authorization).
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Bearer(pub String);

/// A viewer identity (the recipient DID for the reveal path).
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ViewerId(pub String);

/// Injected logical time. Never a wall clock; a monotonic value the caller supplies.
pub type LogicalTime = u64;

/// Content-address a ciphertext: blake3 over the exact stored bytes.
#[must_use]
pub fn content_address(ct: &Ciphertext) -> RecordRef {
    RecordRef(blake3::hash(&ct.0).to_hex().to_string())
}

/// The append target. In production this is a PDS reached with a per-collection OAuth scope; here it
/// is faked in memory. The trait deliberately exposes no `open`/decrypt: a writer cannot read.
pub trait WriteTarget {
    /// Append a ciphertext record under `r` in `collection`. Idempotent on `r` (content address).
    ///
    /// # Errors
    /// Implementation-defined store failures.
    fn append(
        &mut self,
        collection: &Collection,
        r: &RecordRef,
        ct: &Ciphertext,
    ) -> Result<(), StoreError>;

    /// Fetch a single record's ciphertext, if present.
    fn get(&self, collection: &Collection, r: &RecordRef) -> Option<Ciphertext>;

    /// List all records in a collection in append order, as (ref, ciphertext).
    fn list(&self, collection: &Collection) -> Vec<(RecordRef, Ciphertext)>;
}

/// In-memory [`WriteTarget`] for the hermetic run.
#[derive(Default)]
pub struct InMemoryStore {
    order: Vec<(Collection, RecordRef)>,
    map: std::collections::HashMap<(Collection, RecordRef), Ciphertext>,
}

impl InMemoryStore {
    /// A fresh empty store.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }
}

impl WriteTarget for InMemoryStore {
    fn append(
        &mut self,
        collection: &Collection,
        r: &RecordRef,
        ct: &Ciphertext,
    ) -> Result<(), StoreError> {
        let key = (collection.clone(), r.clone());
        if !self.map.contains_key(&key) {
            self.order.push(key.clone());
        }
        self.map.insert(key, ct.clone());
        Ok(())
    }

    fn get(&self, collection: &Collection, r: &RecordRef) -> Option<Ciphertext> {
        self.map.get(&(collection.clone(), r.clone())).cloned()
    }

    fn list(&self, collection: &Collection) -> Vec<(RecordRef, Ciphertext)> {
        self.order
            .iter()
            .filter(|(c, _)| c == collection)
            .filter_map(|(c, r)| self.map.get(&(c.clone(), r.clone())).map(|ct| (r.clone(), ct.clone())))
            .collect()
    }
}

/// The content-blind ingest for one card. Holds only the bearer capability; no key, no decrypt.
pub struct Ingest {
    authorized: Bearer,
}

impl Ingest {
    /// A new ingest gated by the card's bearer capability.
    #[must_use]
    pub fn new(authorized: Bearer) -> Self {
        Self { authorized }
    }

    /// Append an already-encrypted contribution on behalf of a link holder.
    ///
    /// Verifies the presented bearer against the card's capability, content-addresses the ciphertext,
    /// and stores it. The service never sees plaintext.
    ///
    /// # Errors
    /// [`IngestError::Unauthorized`] if the bearer does not match; [`IngestError::Store`] on a store
    /// failure.
    pub fn append<W: WriteTarget>(
        &self,
        store: &mut W,
        presented: &Bearer,
        collection: &Collection,
        ct: &Ciphertext,
    ) -> Result<RecordRef, IngestError> {
        if presented != &self.authorized {
            return Err(IngestError::Unauthorized);
        }
        let r = content_address(ct);
        store.append(collection, &r, ct).map_err(IngestError::Store)?;
        Ok(r)
    }
}

/// The scheduled read-grant for the recipient reveal.
pub struct Reveal {
    reveal_at: LogicalTime,
    recipient: ViewerId,
}

impl Reveal {
    /// A reveal that opens to `recipient` at logical time `reveal_at`.
    #[must_use]
    pub fn new(reveal_at: LogicalTime, recipient: ViewerId) -> Self {
        Self { reveal_at, recipient }
    }

    /// Offer the (still-encrypted) records to `viewer` at logical time `now`.
    ///
    /// Returns the stored ciphertext records only when `now >= reveal_at` AND `viewer` is the
    /// recipient. The returned bytes are ciphertext; the reveal never decrypts (the viewer needs the
    /// key, which the service never holds).
    ///
    /// # Errors
    /// [`Withheld`] if it is too early OR the viewer is not the recipient. The two are indistinguishable
    /// by design: the gate leaks neither existence nor which condition failed.
    pub fn offer<W: WriteTarget>(
        &self,
        store: &W,
        collection: &Collection,
        now: LogicalTime,
        viewer: &ViewerId,
    ) -> Result<Vec<(RecordRef, Ciphertext)>, Withheld> {
        // Both conditions collapse to the same opaque Withheld: the gate leaks neither the
        // existence of records nor which condition (time vs viewer) failed.
        if now < self.reveal_at || viewer != &self.recipient {
            return Err(Withheld);
        }
        Ok(store.list(collection))
    }
}

/// An append was refused.
#[derive(Debug, PartialEq, Eq)]
pub enum IngestError {
    /// The presented bearer did not match the card's capability.
    Unauthorized,
    /// The underlying store failed.
    Store(StoreError),
}

/// A store-layer failure.
#[derive(Debug, PartialEq, Eq)]
pub struct StoreError(pub String);

/// The reveal is not available to this viewer at this time. Opaque by design.
#[derive(Debug, PartialEq, Eq)]
pub struct Withheld;

impl std::fmt::Display for IngestError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            IngestError::Unauthorized => write!(f, "unauthorized: bearer capability does not match"),
            IngestError::Store(e) => write!(f, "store error: {}", e.0),
        }
    }
}
impl std::fmt::Display for StoreError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
impl std::fmt::Display for Withheld {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "withheld")
    }
}
impl std::error::Error for IngestError {}
impl std::error::Error for StoreError {}
impl std::error::Error for Withheld {}

#[cfg(test)]
mod tests {
    use super::*;

    fn ct(bytes: &[u8]) -> Ciphertext {
        Ciphertext(bytes.to_vec())
    }
    fn coll() -> Collection {
        Collection("ing.croft.card.entry".into())
    }

    #[test]
    fn content_address_is_stable_and_tamper_evident() {
        let a = content_address(&ct(b"opaque-1"));
        let b = content_address(&ct(b"opaque-1"));
        let c = content_address(&ct(b"opaque-2"));
        assert_eq!(a, b, "same bytes -> same ref");
        assert_ne!(a, c, "different bytes -> different ref");
    }

    #[test]
    fn append_requires_the_bearer() {
        let ingest = Ingest::new(Bearer("the-unguessable-link".into()));
        let mut store = InMemoryStore::new();
        let wrong = ingest.append(&mut store, &Bearer("guess".into()), &coll(), &ct(b"x"));
        assert_eq!(wrong, Err(IngestError::Unauthorized));
        assert!(store.list(&coll()).is_empty(), "nothing stored on unauthorized append");
    }

    #[test]
    fn append_stores_ciphertext_at_its_content_address() {
        let ingest = Ingest::new(Bearer("link".into()));
        let mut store = InMemoryStore::new();
        let c = ct(b"opaque-bytes");
        let r = ingest
            .append(&mut store, &Bearer("link".into()), &coll(), &c)
            .expect("append");
        assert_eq!(r, content_address(&c));
        assert_eq!(store.get(&coll(), &r), Some(c));
    }

    #[test]
    fn reveal_withholds_before_time_and_for_wrong_viewer_identically() {
        let ingest = Ingest::new(Bearer("link".into()));
        let mut store = InMemoryStore::new();
        ingest
            .append(&mut store, &Bearer("link".into()), &coll(), &ct(b"c1"))
            .expect("append");
        let reveal = Reveal::new(100, ViewerId("did:recipient".into()));

        // too early, correct viewer
        assert_eq!(
            reveal.offer(&store, &coll(), 50, &ViewerId("did:recipient".into())),
            Err(Withheld)
        );
        // on time, wrong viewer — same opaque error, no leak
        assert_eq!(
            reveal.offer(&store, &coll(), 100, &ViewerId("did:someone".into())),
            Err(Withheld)
        );
    }

    #[test]
    fn reveal_offers_ciphertext_on_time_to_recipient() {
        let ingest = Ingest::new(Bearer("link".into()));
        let mut store = InMemoryStore::new();
        ingest
            .append(&mut store, &Bearer("link".into()), &coll(), &ct(b"c1"))
            .expect("append");
        ingest
            .append(&mut store, &Bearer("link".into()), &coll(), &ct(b"c2"))
            .expect("append");
        let reveal = Reveal::new(100, ViewerId("did:recipient".into()));
        let offered = reveal
            .offer(&store, &coll(), 100, &ViewerId("did:recipient".into()))
            .expect("offered at reveal time");
        assert_eq!(offered.len(), 2, "both contributions offered in order");
        // what is offered is ciphertext, never plaintext — the reveal does not decrypt
        assert_eq!(offered[0].1, ct(b"c1"));
        assert_eq!(offered[1].1, ct(b"c2"));
    }
}
