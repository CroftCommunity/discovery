import { run } from "../experiments/e5-spotcheck.ts";
import { registerExperiment } from "../src/testkit.ts";

registerExperiment("E5", "Spot checks and the detection math", run, 50);
