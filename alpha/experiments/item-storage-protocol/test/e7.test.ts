import { run } from "../experiments/e7-seal.ts";
import { registerExperiment } from "../src/testkit.ts";

registerExperiment("E7", "The seal, revocable tier", run, 70);
