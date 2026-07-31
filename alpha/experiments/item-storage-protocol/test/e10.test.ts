import { run } from "../experiments/e10-erasure.ts";
import { registerExperiment } from "../src/testkit.ts";

registerExperiment("E10", "Erasure-coded upgrade path", run, 100);
