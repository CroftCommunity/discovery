import { run } from "../experiments/e9-grace.ts";
import { registerExperiment } from "../src/testkit.ts";

registerExperiment("E9", "The grace ledger", run, 90);
