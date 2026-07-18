// RUN-19 P5 — the full loop: the sealed tier in the browser's shape, one motion.
//
//   wasm member (Node-hosted module) seals
//     → WebTransport/QUIC (native ds-cli, the browser-parity client)
//       → content-blind DS (separate process, no unseal capability)
//         → offer-gated fetch (QUIC again)
//           → a DIFFERENT wasm member unseals, in a different host process.
//
// EVERY sealed artifact — key packages, Welcomes, commits, application
// messages — travels ONLY via the DS store over real QUIC; the wasm hosts
// never talk to each other directly. A commit (add, then removal) rides the
// same path and rolls every member's epoch. The removed member REMAINS on
// the DS offer roster — the strongest form of offering-vs-reading: the DS
// still serves it ciphertext, which its own wasm module provably cannot
// decrypt (MLS forward secrecy), while surviving members read on.
//
// Host stand-ins (the only substitutions): SPEC-DELTA[run19-node-runner]
// (Node hosts the wasm module, not a browser page) and the module state
// living in host memory per SPEC-DELTA[run19-storage-shim] discipline.

import { execFile, spawn } from 'node:child_process';
import { createInterface } from 'node:readline';
import path from 'node:path';
import { fileURLToPath } from 'node:url';
import { promisify } from 'node:util';

const here = path.dirname(fileURLToPath(import.meta.url));
const target = (bin) => path.join(here, '..', 'target', 'release', bin);
const execFileP = promisify(execFile);

// --- the DS (separate blind process) ----------------------------------------
const ds = spawn(target('blind-ds'), ['--port', '0'], { stdio: ['ignore', 'pipe', 'inherit'] });
const announce = await new Promise((resolve) => {
  createInterface({ input: ds.stdout }).once('line', (l) => resolve(JSON.parse(l)));
});
const URL_ = `https://127.0.0.1:${announce.port}`;
const HASH = announce.cert_hash;
const GROUP = 'run19';

// --- the QUIC legs (native browser-parity client, one process per request) --
async function quic(args, stdinHex) {
  const child = execFileP(target('ds-cli'), [URL_, HASH, ...args]);
  if (stdinHex !== undefined) {
    child.child.stdin.write(stdinHex);
    child.child.stdin.end();
  }
  const { stdout } = await child;
  return stdout.trim();
}
const put = (seq, member, hexBlob) => quic(['put', GROUP, String(seq), member], hexBlob);
const fetchSeq = async (seq, member) => {
  const rows = JSON.parse(await quic(['fetch', GROUP, String(seq), member]));
  const row = rows.find((r) => r.seq === seq);
  if (!row) throw new Error(`seq ${seq} not offered to ${member}`);
  return row.data;
};

// --- the wasm member hosts (one Node process per member) --------------------
function wasmHost() {
  const child = spawn('node', [path.join(here, 'wasm-peer.mjs')], { stdio: ['pipe', 'pipe', 'inherit'] });
  const pending = [];
  createInterface({ input: child.stdout }).on('line', (l) => pending.shift()?.(JSON.parse(l)));
  const call = (req) => new Promise((resolve, reject) => {
    pending.push((res) => (res.ok ? resolve(res) : reject(new Error(res.error))));
    child.stdin.write(`${JSON.stringify(req)}\n`);
  });
  return { child, call };
}
const hostA = wasmHost();
const hostB = wasmHost();
const hostC = wasmHost();

const failures = [];
function check(name, cond, detail = '') {
  console.log(cond ? `ok   ${name}` : `FAIL ${name} ${detail}`);
  if (!cond) failures.push(name);
}

try {
  // DS offer roster (roster-admin auth = the RUN-14 EXP-A seam, non-goal).
  for (const m of ['A', 'B', 'C']) await quic(['roster-add', GROUP, m]);

  // --- founding: every artifact through the DS ---------------------------
  await hostA.call({ op: 'found', name: 'A', did: 'did:example:alice' });
  await hostB.call({ op: 'enroll', name: 'B', did: 'did:example:bob' });

  const kpB = (await hostB.call({ op: 'key_package', name: 'B' })).data;
  await put(1, 'B', kpB);
  const invB = await hostA.call({ op: 'invite', name: 'A', kp: await fetchSeq(1, 'A') });
  await put(2, 'A', invB.welcome);
  await hostB.call({ op: 'accept_welcome', name: 'B', welcome: await fetchSeq(2, 'B') });

  const eA0 = (await hostA.call({ op: 'epoch_secret', name: 'A' })).data;
  const eB0 = (await hostB.call({ op: 'epoch_secret', name: 'B' })).data;
  check('P5 wasm B joined via a Welcome that rode QUIC', eA0 === eB0);

  // --- sealed messages, wasm → QUIC → blind DS → QUIC → wasm -------------
  const m1 = (await hostA.call({ op: 'seal', name: 'A', sender: 'alice', text: 'sealed in wasm, carried by QUIC' })).data;
  await put(3, 'A', m1);
  const o1 = await hostB.call({ op: 'open', name: 'B', data: await fetchSeq(3, 'B') });
  check('P5 A→DS→B message unseals in wasm', o1.text === 'sealed in wasm, carried by QUIC');

  const m2 = (await hostB.call({ op: 'seal', name: 'B', sender: 'bob', text: 'and back the other way' })).data;
  await put(4, 'B', m2);
  const o2 = await hostA.call({ op: 'open', name: 'A', data: await fetchSeq(4, 'A') });
  check('P5 B→DS→A message unseals in wasm', o2.text === 'and back the other way');

  // --- an ADD commit travels the same path; epochs roll everywhere -------
  const epochBefore = (await hostA.call({ op: 'epoch', name: 'A' })).epoch;
  await hostC.call({ op: 'enroll', name: 'C', did: 'did:example:carol' });
  const kpC = (await hostC.call({ op: 'key_package', name: 'C' })).data;
  await put(5, 'C', kpC);
  const invC = await hostA.call({ op: 'invite', name: 'A', kp: await fetchSeq(5, 'A') });
  await put(6, 'A', invC.commit);
  await put(7, 'A', invC.welcome);
  await hostB.call({ op: 'apply_control', name: 'B', data: await fetchSeq(6, 'B') });
  await hostC.call({ op: 'accept_welcome', name: 'C', welcome: await fetchSeq(7, 'C') });

  const epochs = [];
  for (const [host, name] of [[hostA, 'A'], [hostB, 'B'], [hostC, 'C']]) {
    epochs.push((await host.call({ op: 'epoch', name })).epoch);
  }
  check('P5 add-commit rode QUIC and rolled every member', epochs.every((e) => e === epochBefore + 1), JSON.stringify(epochs));

  const m3 = (await hostA.call({ op: 'seal', name: 'A', sender: 'alice', text: 'three of us now' })).data;
  await put(8, 'A', m3);
  const o3b = await hostB.call({ op: 'open', name: 'B', data: await fetchSeq(8, 'B') });
  const o3c = await hostC.call({ op: 'open', name: 'C', data: await fetchSeq(8, 'C') });
  check('P5 new-epoch message reads for B and C', o3b.text === 'three of us now' && o3c.text === 'three of us now');

  // --- the REMOVAL commit travels the wire; C stays on the DS roster -----
  const rm = (await hostA.call({ op: 'remove_member', name: 'A', did: 'did:example:carol' })).commit;
  await put(9, 'A', rm);
  await hostB.call({ op: 'apply_control', name: 'B', data: await fetchSeq(9, 'B') });
  await hostC.call({ op: 'apply_control', name: 'C', data: await fetchSeq(9, 'C') }).catch(() => {});

  const post = (await hostA.call({ op: 'seal', name: 'A', sender: 'alice', text: 'post-roll: not for carol' })).data;
  await put(10, 'A', post);

  const oB = await hostB.call({ op: 'open', name: 'B', data: await fetchSeq(10, 'B') });
  check('P5 surviving member reads post-roll traffic', oB.text === 'post-roll: not for carol');

  // Offering vs reading, across the wire: the DS still OFFERS to C…
  const offeredToC = await fetchSeq(10, 'C');
  check('P5 removed member is still OFFERED the ciphertext (DS is authority-free)', offeredToC === post);
  // …but C's own wasm module cannot decrypt it.
  const cReads = await hostC.call({ op: 'open', name: 'C', data: offeredToC })
    .then(() => true).catch(() => false);
  check('P5 removed member provably CANNOT READ what it is offered', cReads === false);

  const sA = (await hostA.call({ op: 'epoch_secret', name: 'A' })).data;
  const sB = (await hostB.call({ op: 'epoch_secret', name: 'B' })).data;
  check('P5 survivors agree on the post-removal epoch secret', sA === sB);
} catch (e) {
  console.log(`FAIL (exception) ${e.message}`);
  failures.push('exception');
}

for (const h of [hostA, hostB, hostC]) h.child.kill('SIGKILL');
ds.kill('SIGKILL');
console.log(failures.length === 0
  ? 'P5 VERDICT: PASS — the sealed tier, browser-shaped, end to end'
  : `P5 VERDICT: FAIL — ${failures.join(', ')}`);
process.exit(failures.length === 0 ? 0 : 1);
