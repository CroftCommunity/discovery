// S2 — Scoped visibility, not opaque structure (social-layer.md §S2).
//
// Two assertions:
//   S2a (INV-STRUCTURE-LEAKS-IDENTITY): a "structure-only" share — topology revealed,
//        identities withheld — is RE-IDENTIFYING. We model the canonical attack ("six
//        people, two connecting to the Henderson family group, in a town of 4,000")
//        and show the anonymity set collapses to 1, then show such a share is
//        UNREPRESENTABLE (the constructor always throws).
//   S2b (INV-SCOPED-CONSENT): the only safe, constructible share is consented-distance/
//        resolution-scoped — a viewer at distance d sees exactly what was consented for
//        distance d, carries NO topology, and over-reach beyond the consented horizon
//        yields nothing.

import { ScenarioRunner, ExperimentResult } from '../harness/runner';
import {
  Person,
  ConnectionFingerprint,
  anonymitySet,
  attemptStructureOnlyShare,
  createScopedShare,
  viewScopedShare,
} from '../core/scoped_visibility';
import { VisibilityViolation } from '../core/visibility';

const runner = new ScenarioRunner();

// Build a town of 4,000. Almost everyone has a generic shape and no Henderson edge.
// A handful touch the Henderson family group; the target additionally touches the
// Oak-St school-parents group — the distinctive intersection that pins identity.
function buildTown(): { population: Person[]; targetFingerprint: ConnectionFingerprint } {
  const population: Person[] = [];
  for (let i = 0; i < 3994; i++) {
    population.push({ id: `resident-${i}`, fingerprint: { degree: 3, anchorGroups: ['town-general'] } });
  }
  // Five people connect to the Henderson family group (the family + close kin).
  for (let i = 0; i < 5; i++) {
    population.push({ id: `henderson-kin-${i}`, fingerprint: { degree: 4, anchorGroups: ['Henderson-family'] } });
  }
  // The target: the one person who touches BOTH Henderson-family AND Oak-St-parents
  // with degree 5 — a unique intersection in this population.
  const targetFingerprint: ConnectionFingerprint = {
    degree: 5,
    anchorGroups: ['Henderson-family', 'Oak-St-parents'],
  };
  population.push({ id: 'TARGET-real-person', fingerprint: targetFingerprint });
  return { population, targetFingerprint };
}

function runS2a(): ExperimentResult {
  return runner.run(
    'S2a: Structure-only share re-identifies (topology is a fingerprint) — unrepresentable',
    'S',
    () => {
      const { population, targetFingerprint } = buildTown();

      // The re-identification: even with every name withheld, the connection shape
      // pins exactly one person out of 4,000.
      const matches = anonymitySet(population, targetFingerprint);
      const reIdentifying = matches.length <= 1;

      // The model refuses to construct a structure-only share at all.
      let unrepresentable = false;
      try {
        attemptStructureOnlyShare(population, [targetFingerprint]);
      } catch (e) {
        unrepresentable = e instanceof VisibilityViolation && e.invariant === 'INV-STRUCTURE-LEAKS-IDENTITY';
      }

      return {
        invariants: [
          {
            invariant: 'INV-STRUCTURE-LEAKS-IDENTITY',
            passed: reIdentifying && unrepresentable,
            details: [
              `Population: ${population.length}; anonymity set for the target's connection shape: ${matches.length} (${matches.join(', ')})`,
              `Re-identifying (anonymity set <= 1): ${reIdentifying} ✓`,
              `Structure-only share is unrepresentable (constructor throws): ${unrepresentable} ✓`,
            ].join('; '),
          },
        ],
        provenanceTrace: [
          `Town of ${population.length}. 3994 generic; 5 touch Henderson-family; 1 touches Henderson-family AND Oak-St-parents (degree 5).`,
          `A "visible structure, hidden identities" share reveals that shape — the shape matches exactly ${matches.length} person.`,
          `=> topology deanonymizes; the share is rejected as unrepresentable, not offered. (social-layer.md §S2)`,
        ].join('\n'),
        socialInputsUsed: [
          'SCRIPTED: Henderson-family / Oak-St-parents intersection as the distinctive connection shape',
          'SCRIPTED: adversary holds the full town population and matches the revealed topology',
        ],
        metrics: {
          population: population.length,
          anonymitySet: matches.length,
        },
      };
    },
  );
}

function runS2b(): ExperimentResult {
  return runner.run(
    'S2b: Only consented-distance/resolution-scoped sharing is representable',
    'S',
    () => {
      // The owner consents to specific content at specific distances — never topology.
      const share = createScopedShare(
        new Map([
          [1, ['close: "here is my phone number"']],
          [2, ['acquaintance: "I am reachable for school carpool"']],
          // distance 3+ : the owner consented to nothing — the horizon.
        ]),
      );

      const d1 = viewScopedShare(share, 1);
      const d2 = viewScopedShare(share, 2);
      const d3 = viewScopedShare(share, 3); // beyond the consented horizon

      // Viewer at distance d sees EXACTLY the consented content for d (not nearer
      // distances' content, never topology), and nothing beyond the horizon.
      const d1Exact = d1.length === 1 && d1[0].startsWith('close:');
      const d2Exact = d2.length === 1 && d2[0].startsWith('acquaintance:');
      const d2NoLeakNearer = !d2.some(c => c.startsWith('close:'));
      const d3Empty = d3.length === 0;
      // The share object carries no topology/fingerprint field at all.
      const carriesNoTopology = !('fingerprint' in share) && !('anchorGroups' in share);

      const passed = d1Exact && d2Exact && d2NoLeakNearer && d3Empty && carriesNoTopology;

      return {
        invariants: [
          {
            invariant: 'INV-SCOPED-CONSENT',
            passed,
            details: [
              `Distance 1 sees exactly the consented close content: ${d1Exact} ✓`,
              `Distance 2 sees exactly the consented acquaintance content: ${d2Exact} ✓`,
              `Distance 2 does NOT see nearer (distance-1) content: ${d2NoLeakNearer} ✓`,
              `Distance 3 (beyond horizon) sees nothing — not "structure minus names": ${d3Empty} ✓`,
              `Scoped share carries no topology/fingerprint: ${carriesNoTopology} ✓`,
            ].join('; '),
          },
        ],
        provenanceTrace: [
          `Owner consents: distance 1 -> phone number; distance 2 -> carpool reachability; distance 3+ -> nothing.`,
          `A viewer receives only the content consented for their distance, never the graph shape.`,
          `Over-reach (distance 3) returns [] — the horizon yields silence, not anonymized structure.`,
        ].join('\n'),
        socialInputsUsed: ['SCRIPTED: owner-defined per-distance consent map'],
        metrics: {
          consentedDistances: share.consentByDistance.size,
          beyondHorizonItems: d3.length,
        },
      };
    },
  );
}

export function run(): ExperimentResult[] {
  return [runS2a(), runS2b()];
}
