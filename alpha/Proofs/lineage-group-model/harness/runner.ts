import { InvariantResult } from '../core/invariants';

export interface ExperimentResult {
  name: string;
  group: string;
  invariants: InvariantResult[];
  provenanceTrace: string;
  socialInputsUsed: string[];
  passed: boolean;
  durationMs: number;
  metrics?: Record<string, number | string>;
}

export class ScenarioRunner {
  run(
    name: string,
    group: string,
    fn: () => Omit<ExperimentResult, 'name' | 'group' | 'passed' | 'durationMs'>,
  ): ExperimentResult {
    const start = Date.now();
    try {
      const result = fn();
      const passed = result.invariants.every(inv => inv.passed);
      return { name, group, ...result, passed, durationMs: Date.now() - start };
    } catch (err) {
      return {
        name,
        group,
        invariants: [{
          invariant: 'EXECUTION',
          passed: false,
          details: `Experiment threw: ${err instanceof Error ? err.message : String(err)}`,
        }],
        provenanceTrace: '',
        socialInputsUsed: [],
        passed: false,
        durationMs: Date.now() - start,
      };
    }
  }
}
