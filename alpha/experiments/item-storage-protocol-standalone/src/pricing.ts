// The price list. Every figure is in integer cents so that everything that must
// balance balances to the cent, with no floating-point drift. Rent is priced per
// byte-day, postage per byte, audits at cost (a per-byte read cost plus a fixed
// per-audit overhead). "At cost" is the whole point: the assurance dial has a
// true, linear price, no margin encoded into paranoia.
//
// These are mock rates chosen for legible arithmetic, not real tariffs.

/** Rent: 1 cent per this many byte-days. Rent in cents = floor(byteDays / D). */
export const RENT_NUMERATOR = 1; // cents
export const RENT_DENOMINATOR = 10_000; // per this many byte-days

/** Postage: 1 cent per this many bytes transferred. */
export const POSTAGE_BYTES_PER_CENT = 1_000;

/** Audit read cost: 1 cent per this many bytes retrieved (same physics as postage). */
export const AUDIT_BYTES_PER_CENT = 1_000;

/** Fixed overhead booked per audit, on top of the bytes-read cost. */
export const AUDIT_OVERHEAD_CENTS = 2;

export function rentCents(byteDays: number): number {
  return Math.floor((byteDays * RENT_NUMERATOR) / RENT_DENOMINATOR);
}

export function postageCents(bytes: number): number {
  return Math.floor(bytes / POSTAGE_BYTES_PER_CENT);
}

export function auditCents(bytesRead: number, auditCount: number): number {
  return Math.floor(bytesRead / AUDIT_BYTES_PER_CENT) + auditCount * AUDIT_OVERHEAD_CENTS;
}
