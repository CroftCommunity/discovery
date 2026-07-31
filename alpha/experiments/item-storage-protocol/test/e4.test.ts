import { run } from "../experiments/e4-statements.ts";
import { registerExperiment } from "../src/testkit.ts";

registerExperiment("E4", "Balance-forward statements", run, 40);
