import { run } from "../experiments/e12-outside-reader.ts";
import { registerExperiment } from "../src/testkit.ts";

registerExperiment("E12", "The outside reader: diligence from the files alone", run, 120);
