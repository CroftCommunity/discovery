import { run } from "../experiments/e3-receipts.ts";
import { registerExperiment } from "../src/testkit.ts";

registerExperiment("E3", "Transfer receipts", run, 30);
