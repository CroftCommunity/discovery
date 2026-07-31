import { run } from "../experiments/e1-items.ts";
import { registerExperiment } from "../src/testkit.ts";

registerExperiment("E1", "Items and fingerprints", run, 10);
