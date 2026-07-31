import { run } from "../experiments/e13-covenants.ts";
import { registerExperiment } from "../src/testkit.ts";

registerExperiment("E13", "Covenants as code, and the underwriting packet", run, 130);
