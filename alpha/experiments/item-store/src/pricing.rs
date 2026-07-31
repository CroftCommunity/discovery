//! The price list. Every figure is in integer cents so anything that must
//! balance balances to the cent, with no floating-point drift. Rent is priced
//! per byte-day, postage per byte. "At cost" is the point: the assurance dial
//! (Phase 5) has a true, linear price.
//!
//! Ports `item-storage-protocol-standalone/src/pricing.ts`. These are mock rates
//! chosen for legible arithmetic, not real tariffs. Audit pricing arrives with
//! the dial in Phase 5.

/// Rent numerator (cents).
pub const RENT_NUMERATOR: u64 = 1;
/// Rent denominator: 1 cent per this many byte-days.
pub const RENT_DENOMINATOR: u64 = 10_000;
/// Postage: 1 cent per this many bytes transferred.
pub const POSTAGE_BYTES_PER_CENT: u64 = 1_000;

/// Rent in cents for a byte-day total: `floor(byte_days / RENT_DENOMINATOR)`.
#[must_use]
pub fn rent_cents(byte_days: u64) -> u64 {
    byte_days * RENT_NUMERATOR / RENT_DENOMINATOR
}

/// Postage in cents for a byte total: `floor(bytes / POSTAGE_BYTES_PER_CENT)`.
#[must_use]
pub fn postage_cents(bytes: u64) -> u64 {
    bytes / POSTAGE_BYTES_PER_CENT
}

#[cfg(test)]
mod tests {
    use super::{postage_cents, rent_cents};

    #[test]
    fn rent_floors_byte_days() {
        assert_eq!(rent_cents(0), 0);
        assert_eq!(rent_cents(9_999), 0); // below one cent
        assert_eq!(rent_cents(10_000), 1);
        assert_eq!(rent_cents(25_000), 2); // floor(2.5)
    }

    #[test]
    fn postage_floors_bytes() {
        assert_eq!(postage_cents(0), 0);
        assert_eq!(postage_cents(999), 0);
        assert_eq!(postage_cents(1_000), 1);
        assert_eq!(postage_cents(8_192), 8); // floor(8.192)
    }
}
