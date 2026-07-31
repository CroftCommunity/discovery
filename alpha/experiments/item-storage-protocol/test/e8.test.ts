import { run } from "../experiments/e8-tombstone.ts";
import { registerExperiment } from "../src/testkit.ts";

registerExperiment("E8", "The tombstone, permanent tier", run, 80);
