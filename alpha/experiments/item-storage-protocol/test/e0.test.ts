import { run } from "../experiments/e0-identity.ts";
import { registerExperiment } from "../src/testkit.ts";

registerExperiment("E0", "Identity", run, 0);
