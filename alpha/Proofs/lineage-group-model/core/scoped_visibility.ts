// S2 — Scoped visibility, not opaque structure.
//
// The safety invariant (discovery/thinking/social-layer.md §S2): you CANNOT offer
// "visible structure with hidden identities," because graph topology deanonymizes —
// the shape of someone's connections is close to a fingerprint. "Six people, two
// connecting to the Henderson family group, in a town of 4,000" re-identifies fast.
// So a structure-only share (topology revealed, identities withheld) must be
// UNREPRESENTABLE, not merely discouraged. The only safe share is visibility scoped
// to what people consented to be seen by someone at a given distance/resolution.
//
// This module models both halves: (a) the re-identification — a topology fingerprint
// collapses the anonymity set in a real population; (b) the only constructible share
// is a consented-distance/resolution-scoped one, and a structure-only share throws.

import { VisibilityViolation } from './visibility';

/// A person's connection shape: degree plus the named "anchor" groups they touch.
/// This is exactly the structure a "hidden-identity" share would reveal.
export interface ConnectionFingerprint {
  degree: number;
  anchorGroups: string[]; // notable, nameable groups (e.g. a family group, a school-parents group)
}

export interface Person {
  id: string;
  fingerprint: ConnectionFingerprint;
}

function fingerprintKey(f: ConnectionFingerprint): string {
  return `${f.degree}|${[...f.anchorGroups].sort().join(',')}`;
}

/// The anonymity set for a fingerprint in a population: how many members share the
/// exact connection shape. An anonymity set of 1 means the topology alone pins the
/// identity — the share re-identifies even with every name withheld.
export function anonymitySet(population: Person[], fingerprint: ConnectionFingerprint): string[] {
  const key = fingerprintKey(fingerprint);
  return population.filter(p => fingerprintKey(p.fingerprint) === key).map(p => p.id);
}

/// (a) The unrepresentable share. Offering topology with identities withheld is not a
/// share type the system can construct — it always throws, exactly like a regime
/// change (V1). Modelling it as a `never`-returning function is the point: there is
/// no code path that produces a structure-only share for a viewer to receive.
export function attemptStructureOnlyShare(
  _population: Person[],
  _revealedFingerprints: ConnectionFingerprint[],
): never {
  throw new VisibilityViolation(
    'INV-STRUCTURE-LEAKS-IDENTITY',
    'Cannot construct a structure-only share: revealing topology with identities withheld is ' +
      're-identifying (a connection shape is a near-fingerprint). Only consented-distance/resolution ' +
      'scoped sharing is representable.',
  );
}

/// (b) The only safe, constructible share: content the owner consented to be seen at
/// each distance/resolution. It carries NO topology — a viewer learns only the
/// consented content for their distance, nothing about the graph beyond it.
export interface ScopedShare {
  // distance -> the exact content the owner consented to expose at that distance.
  consentByDistance: Map<number, string[]>;
  maxConsentedDistance: number;
}

export function createScopedShare(consentByDistance: Map<number, string[]>): ScopedShare {
  const distances = [...consentByDistance.keys()];
  return {
    consentByDistance,
    maxConsentedDistance: distances.length === 0 ? -1 : Math.max(...distances),
  };
}

/// A viewer at `viewerDistance` sees EXACTLY the content consented for that distance —
/// and never any topology, and never anything consented only for a nearer distance.
/// Beyond the consented horizon the share yields nothing (no leakage by over-reach).
export function viewScopedShare(share: ScopedShare, viewerDistance: number): string[] {
  if (viewerDistance > share.maxConsentedDistance) {
    return []; // beyond the consented horizon: nothing, not "structure minus names"
  }
  return [...(share.consentByDistance.get(viewerDistance) ?? [])];
}
