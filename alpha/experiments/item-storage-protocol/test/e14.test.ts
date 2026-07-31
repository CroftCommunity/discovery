import { run } from "../experiments/e14-soft-unit.ts";
import { registerExperiment } from "../src/testkit.ts";

registerExperiment("E14", "The soft-unit counterexample: proving the scope condition", run, 140);
