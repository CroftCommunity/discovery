# FINDINGS — anonymity sets under anchor-persona credentials (RUN-ATTEST-02, EXP-PA4)

`Grade: measurement, Modeled fixtures. The harness is proven red-first on a
hand-computed 6-persona fixture (T-PA4.1) before touching these populations;
the M-PA4.2/M-PA4.3 tables below are recomputed by `tests/t_pa4_anonymity.rs
::measure_anonymity_sets` on every run and asserted equal to this document, so
the numbers cannot drift from the code.`

## What is being measured

A credential does not identify a persona, but it does **partition** personas:
showing `over_18` from issuer C places the presenter inside the pool of
personas holding a standing `over_18` from C. That pool IS the presenter's
anonymity set for that showing. Two forces shrink it:

- **small co-ops** — fewer members, thinner cover for everyone;
- **bundle composition** — each additional predicate shown together
  intersects the pools.

## Fixture populations

Same code paths, different populations (§3): **COOP-S** = 12 member-holders,
15 personas (one member holds 3 anchors, one holds 2, ten hold 1).
**COOP-L** = 400 member-holders, 441 personas (one 3-anchor, thirty-nine
2-anchor, 360 single-anchor). Predicates assigned by fixed congruences
(`fixtures::generated_kinds`): every anchor persona carries `vetted_holder`;
~95% `over_18`; ~60% `phone_verified`; ~35% `payment_verified`.

## M-PA4.2 — anonymity-set size per (issuer, predicate)

| predicate | COOP-S (15 personas) | COOP-L (441 personas) |
|---|---|---|
| `vetted_holder` | 15 | 441 |
| `over_18` | 14 | 419 |
| `phone_verified` | 10 | 265 |
| `payment_verified` | 7 | 155 |

## M-PA4.3 — the shrink from presentation-side bundle composition

| bundle shown together | COOP-S | COOP-L |
|---|---|---|
| `over_18` alone | 14 | 419 |
| `over_18` + `phone_verified` | 9 | 245 |
| `over_18` + `phone_verified` + `payment_verified` | 5 | 71 |

The same persona presenting the full bundle instead of `over_18` alone cuts
its cover ~3× in COOP-S (14 → 5) and ~6× in COOP-L (419 → 71). In the small
co-op the full bundle leaves an anonymity set of FIVE — thin enough that any
side information (scope of interaction, timing, geography) plausibly
completes the identification.

## Plain-language guidance

- **Coarse predicates by default.** Present the least you can: `over_18`
  alone keeps you in the biggest pool. Every extra predicate shown in the
  same interaction is an intersection, not an addition. The protocol makes
  the subset choice free (single-predicate credentials are the unit,
  T-PA4.4); clients should make the coarse choice the default one.
- **Small co-ops should say so to their members.** A 12-member co-op cannot
  offer big-pool cover for ANY predicate, and members deserve to know that a
  rare bundle there is close to identifying. Pool sizes are computable by the
  issuer from public counts alone — surfacing them is a client/issuer UX
  duty, not a protocol change.
- **Federation across issuers is the eventual widener** (direction only, not
  built): if several co-ops' predicates were acceptable interchangeably to
  verifiers, the pool for "some trusted issuer says over_18" is the union
  across issuers rather than one co-op's membership. That composes with, and
  does not replace, the deferred unlinkable-presentation cryptography
  recorded in FINDINGS.md (F-AT-2, F-PA-2).

(evidence: `tests/t_pa4_anonymity.rs`, RUN-ATTEST-02, measurement on Modeled
fixtures)
