import { run } from "../experiments/e6-dial.ts";
import { registerExperiment } from "../src/testkit.ts";

registerExperiment("E6", "The dial", run, 60);
