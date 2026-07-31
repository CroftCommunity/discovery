// The suite under the Node built-in test runner. One world is built and run in
// order (dependencies matter), then each experiment becomes a subtest asserting
// every one of its assertions passed. Structural tests confirm the statement chain
// verifies, every ledger entry is signature-checkable, and the run is deterministic.

import { test } from "node:test";
import assert from "node:assert/strict";
import { runSuite } from "../src/suite.ts";
import { World } from "../src/world.ts";
import { verifyChain } from "../src/statement.ts";
import { verifyEntries } from "../src/ledger.ts";

const world = runSuite();

for (const result of world.results) {
  test(`${result.id} ${result.title}`, () => {
    for (const a of result.assertions) {
      assert.ok(a.ok, `${result.id} assertion failed: ${a.name}${a.detail ? ` — ${a.detail}` : ""}`);
    }
    assert.ok(result.assertions.length > 0, `${result.id} recorded no assertions`);
    assert.ok(result.plainSentence.length > 0, `${result.id} has no plain sentence`);
  });
}

test("statement chain verifies from genesis", () => {
  assert.ok(verifyChain(world.statements).ok);
  assert.ok(world.statements.length >= 12);
});

test("every ledger entry is signature-checkable against the pinned keyring", () => {
  const actors = [world.customer, world.provider, ...world.investors];
  for (const a of actors) {
    const issues = verifyEntries(a.ledger.entries, world.keyring);
    assert.equal(issues.length, 0, `${a.name} ledger issues: ${JSON.stringify(issues)}`);
  }
});

test("the run is deterministic (two fresh worlds produce identical ledgers)", () => {
  const a = runSuite(new World());
  const b = runSuite(new World());
  assert.equal(a.customer.ledger.toJSONL(), b.customer.ledger.toJSONL());
  assert.equal(a.provider.ledger.toJSONL(), b.provider.ledger.toJSONL());
  assert.deepEqual(a.keyring, b.keyring);
});
