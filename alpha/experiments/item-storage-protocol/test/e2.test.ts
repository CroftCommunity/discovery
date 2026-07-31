import { run } from "../experiments/e2-manifest.ts";
import { registerExperiment } from "../src/testkit.ts";

registerExperiment("E2", "The manifest", run, 20);
