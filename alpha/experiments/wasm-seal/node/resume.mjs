// RUN-19 P3 — the wasm host-kill drill (the browser-shaped resumption story).
//
//   1. wasm member bob (child Node process hosting the module) joins a group
//      founded by the native peer alice; they converse.
//   2. bob's state goes to rest as an AES-GCM blob on the host filesystem
//      (run19-storage-shim); the parent SIGKILLs bob's host MID-CONVERSATION.
//   3. alice keeps talking AND rolls the epoch (adds carol) while bob is dead.
//   4. a FRESH host process restores bob from the blob: he opens the missed
//      message, folds the roll commit, and decrypts the NEXT EPOCH's traffic.
//   5. EVICTION drill: the blob is destroyed; resume fails (no self-restore —
//      forward secrecy is not overridden); re-entry is a fresh Welcome,
//      provably blind to the gap.

import { spawn } from 'node:child_process';
import { createInterface } from 'node:readline';
import { existsSync, mkdtempSync, rmSync } from 'node:fs';
import { tmpdir } from 'node:os';
import path from 'node:path';
import { fileURLToPath } from 'node:url';

const here = path.dirname(fileURLToPath(import.meta.url));
const stateDir = mkdtempSync(path.join(tmpdir(), 'run19-p3-'));
const stateFile = path.join(stateDir, 'bob.state');
const KEY = Buffer.from('run19-at-rest-k!').toString('hex'); // 16 bytes

function ndjsonClient(cmd, argv) {
  const child = spawn(cmd, argv, { stdio: ['pipe', 'pipe', 'inherit'] });
  const lines = createInterface({ input: child.stdout });
  const pending = [];
  lines.on('line', (l) => pending.shift()?.(JSON.parse(l)));
  const call = (req) => new Promise((resolve, reject) => {
    pending.push((res) => (res.ok ? resolve(res) : reject(new Error(res.error))));
    child.stdin.write(`${JSON.stringify(req)}\n`);
  });
  return { child, call };
}

const spawnNative = () => ndjsonClient(path.join(here, '..', 'target', 'release', 'seal-peer'), []);
const spawnWasmHost = () => ndjsonClient('node', [path.join(here, 'wasm-peer.mjs'), '--state-file', stateFile]);

const failures = [];
function check(name, cond, detail = '') {
  console.log(cond ? `ok   ${name}` : `FAIL ${name} ${detail}`);
  if (!cond) failures.push(name);
}

const native = spawnNative().call;
let host = spawnWasmHost();

try {
  // 1. founded natively, joined in wasm; a live conversation.
  await native({ op: 'found', name: 'alice', did: 'did:example:alice' });
  await host.call({ op: 'enroll', name: 'bob', did: 'did:example:bob' });
  const kp = (await host.call({ op: 'key_package', name: 'bob' })).data;
  const inv = await native({ op: 'invite', name: 'alice', kp });
  await host.call({ op: 'accept_welcome', name: 'bob', welcome: inv.welcome });
  const m1 = await native({ op: 'seal', name: 'alice', sender: 'alice', text: 'hello wasm bob' });
  const o1 = await host.call({ op: 'open', name: 'bob', data: m1.data });
  check('P3 conversation live before the kill', o1.text === 'hello wasm bob');

  // 2. state to rest; SIGKILL the wasm host mid-conversation.
  const persisted = await host.call({ op: 'persist', name: 'bob', key: KEY });
  check('P3 encrypted blob written by the host', persisted.bytes > 0 && existsSync(stateFile));
  host.child.kill('SIGKILL');

  // 3. life goes on without bob: a message, an epoch roll, a post-roll message.
  const missed = await native({ op: 'seal', name: 'alice', sender: 'alice', text: 'while you were down' });
  await native({ op: 'enroll', name: 'carol', did: 'did:example:carol' });
  const kpC = (await native({ op: 'key_package', name: 'carol' })).data;
  const invC = await native({ op: 'invite', name: 'alice', kp: kpC });
  await native({ op: 'accept_welcome', name: 'carol', welcome: invC.welcome });
  const postRoll = await native({ op: 'seal', name: 'alice', sender: 'alice', text: 'next epoch says hi' });

  // 4. a fresh host restores bob from the blob.
  host = spawnWasmHost();
  await host.call({ op: 'resume', name: 'bob', key: KEY });
  const oMissed = await host.call({ op: 'open', name: 'bob', data: missed.data });
  check('P3 restored member opens the missed message', oMissed.text === 'while you were down');
  await host.call({ op: 'apply_control', name: 'bob', data: invC.commit });
  const oPost = await host.call({ op: 'open', name: 'bob', data: postRoll.data });
  check('P3 restored member decrypts the NEXT EPOCH after folding the roll', oPost.text === 'next epoch says hi');
  const sBob = (await host.call({ op: 'epoch_secret', name: 'bob' })).data;
  const sAlice = (await native({ op: 'epoch_secret', name: 'alice' })).data;
  check('P3 restored member agrees on the epoch secret byte-for-byte', sBob === sAlice);

  // 5. EVICTION: destroy the blob; no self-restore; rejoin blind to the gap.
  host.child.kill('SIGKILL');
  rmSync(stateFile);
  host = spawnWasmHost();
  const resumeFailed = await host.call({ op: 'resume', name: 'bob', key: KEY })
    .then(() => false).catch(() => true);
  check('P3 eviction: destroyed blob cannot self-restore', resumeFailed);

  const gapMsg = await native({ op: 'seal', name: 'alice', sender: 'alice', text: 'sealed into the gap' });

  await host.call({ op: 'enroll', name: 'bob', did: 'did:example:bob' });
  const kp2 = (await host.call({ op: 'key_package', name: 'bob' })).data;
  const inv2 = await native({ op: 'invite', name: 'alice', kp: kp2 });
  await native({ op: 'apply_control', name: 'carol', data: inv2.commit });
  await host.call({ op: 'accept_welcome', name: 'bob', welcome: inv2.welcome });
  const fresh = await native({ op: 'seal', name: 'alice', sender: 'alice', text: 'welcome back' });
  const oFresh = await host.call({ op: 'open', name: 'bob', data: fresh.data });
  check('P3 re-entry via fresh Welcome reads new traffic', oFresh.text === 'welcome back');
  const gapBlind = await host.call({ op: 'open', name: 'bob', data: gapMsg.data })
    .then(() => false).catch(() => true);
  check('P3 re-entry is BLIND to the gap (forward secrecy not overridden)', gapBlind);
} catch (e) {
  console.log(`FAIL (exception) ${e.message}`);
  failures.push('exception');
}

host.child.kill('SIGKILL');
process.exitCode = failures.length === 0 ? 0 : 1;
console.log(failures.length === 0
  ? 'P3 VERDICT: PASS — resumption + eviction honesty hold across a real host kill'
  : `P3 VERDICT: FAIL — ${failures.join(', ')}`);
process.exit();
