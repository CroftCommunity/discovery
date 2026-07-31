import { run } from "../experiments/e11-royalty.ts";
import { registerExperiment } from "../src/testkit.ts";

registerExperiment("E11", "The financing ledger: extinguishing royalty", run, 110);
