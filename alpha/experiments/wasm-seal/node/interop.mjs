// RUN-19 P2 — cross-build interop goldens (the correctness claim that matters).
//
// Drives the SAME seal stack in its two builds side by side:
//   - wasm32 build: the seal-wasm module (wasm-pack --target nodejs pkg),
//     hosted in this Node process (SPEC-DELTA[run19-node-runner]).
//   - native build: the seal-peer ndjson subprocess.
//
// Goldens:
//   G1  ciphertext SEALED IN WASM is unsealed by the NATIVE build;
//   G2  ciphertext SEALED NATIVE is unsealed by the WASM build;
//   G3  a full transcript (adds, messages both ways, a commit, a removal)
//       generated half-in-wasm half-native folds to the identical group
//       state on both sides, byte-compared via the canonical encodings
//       (the exported epoch secret — the lineage-groups I4 comparator —
//       plus the epoch counter) at every step;
//   G4  the removed (native) member is forward-blind to post-roll wasm
//       ciphertext — asymmetric failure would be a stop-the-line finding.
//
// FALSIFY: any asymmetry between the builds. Exit 0 with `P2 VERDICT: PASS`
// only if every golden holds.

import { spawn } from 'node:child_process';
import { createInterface } from 'node:readline';
import { createRequire } from 'node:module';
import path from 'node:path';
import { fileURLToPath } from 'node:url';

const here = path.dirname(fileURLToPath(import.meta.url));
const require = createRequire(import.meta.url);

// --- the wasm build ---------------------------------------------------------
const pkgPath = path.join(here, '..', 'crates', 'seal-wasm', 'pkg', 'seal_wasm.js');
const wasm = require(pkgPath); // throws if the JS API / pkg is unbuilt (RED)

// --- the native build -------------------------------------------------------
const peerBin = path.join(
  here, '..', 'target', 'release', 'seal-peer',
);
const peer = spawn(peerBin, [], { stdio: ['pipe', 'pipe', 'inherit'] });
const peerLines = createInterface({ input: peer.stdout });
const pending = [];
peerLines.on('line', (l) => {
  const next = pending.shift();
  if (next) next(JSON.parse(l));
});
function native(req) {
  return new Promise((resolve, reject) => {
    pending.push((res) => (res.ok ? resolve(res) : reject(new Error(`native: ${res.error}`))));
    peer.stdin.write(`${JSON.stringify(req)}\n`);
  });
}

const failures = [];
function check(name, cond, detail = '') {
  if (cond) {
    console.log(`ok   ${name}`);
  } else {
    console.log(`FAIL ${name} ${detail}`);
    failures.push(name);
  }
}

// State-agreement helper: every live member (wasm objects + native names)
// must sit in the same epoch with byte-identical exported epoch secrets.
async function assertStateAgreement(label, wasmMembers, nativeNames) {
  const secrets = [];
  const epochs = [];
  for (const [name, s] of wasmMembers) {
    secrets.push([`wasm:${name}`, s.epoch_secret_hex()]);
    epochs.push([`wasm:${name}`, Number(s.epoch())]);
  }
  for (const name of nativeNames) {
    const sec = await native({ op: 'epoch_secret', name });
    const ep = await native({ op: 'epoch', name });
    secrets.push([`native:${name}`, sec.data]);
    epochs.push([`native:${name}`, ep.epoch]);
  }
  const s0 = secrets[0][1];
  const e0 = epochs[0][1];
  check(
    `${label}: epoch secrets byte-identical across builds`,
    secrets.every(([, s]) => s === s0),
    JSON.stringify(secrets),
  );
  check(
    `${label}: epochs identical across builds`,
    epochs.every(([, e]) => e === e0),
    JSON.stringify(epochs),
  );
}

try {
  // --- setup: a group founded IN WASM with one native + one wasm member ----
  const alice = wasm.JsSealer.found('did:example:alice'); // wasm founder
  await native({ op: 'enroll', name: 'bob', did: 'did:example:bob' }); // native
  const carol = wasm.JsSealer.enroll('did:example:carol'); // wasm member

  // native bob's key package crosses the build boundary into wasm
  const kpBob = (await native({ op: 'key_package', name: 'bob' })).data;
  const invBob = alice.invite(Buffer.from(kpBob, 'hex'));
  await native({ op: 'accept_welcome', name: 'bob', welcome: Buffer.from(invBob.welcome).toString('hex') });
  await assertStateAgreement('after wasm-invites-native', [['alice', alice]], ['bob']);

  // --- G1: sealed in wasm, unsealed native --------------------------------
  const g1 = alice.seal('alice', 'sealed in wasm, opened natively');
  const g1open = await native({ op: 'open', name: 'bob', data: Buffer.from(g1).toString('hex') });
  check('G1 wasm→native unseal', g1open.sender === 'alice' && g1open.text === 'sealed in wasm, opened natively', JSON.stringify(g1open));

  // --- G2: sealed native, unsealed wasm ------------------------------------
  const g2 = (await native({ op: 'seal', name: 'bob', sender: 'bob', text: 'sealed natively, opened in wasm' })).data;
  const g2open = alice.open(Buffer.from(g2, 'hex'));
  check('G2 native→wasm unseal', g2open.sender === 'bob' && g2open.text === 'sealed natively, opened in wasm', JSON.stringify(g2open));

  // --- G3: half-wasm half-native transcript --------------------------------
  // add wasm carol via a NATIVE commit (native produces, wasm folds)
  const kpCarol = carol.key_package();
  const invCarol = await native({ op: 'invite', name: 'bob', kp: Buffer.from(kpCarol).toString('hex') });
  alice.apply_control(Buffer.from(invCarol.commit, 'hex'));
  carol.accept_welcome(Buffer.from(invCarol.welcome, 'hex'));
  await assertStateAgreement('after native-invites-wasm commit', [['alice', alice], ['carol', carol]], ['bob']);

  // messages in the new epoch, all directions across builds
  const m1 = carol.seal('carol', 'wasm carol to all');
  const m1native = await native({ op: 'open', name: 'bob', data: Buffer.from(m1).toString('hex') });
  const m1wasm = alice.open(m1);
  check('G3 wasm ciphertext read by both builds', m1native.text === 'wasm carol to all' && m1wasm.text === 'wasm carol to all');
  const m2 = (await native({ op: 'seal', name: 'bob', sender: 'bob', text: 'native bob to all' })).data;
  check(
    'G3 native ciphertext read by both wasm members',
    alice.open(Buffer.from(m2, 'hex')).text === 'native bob to all'
      && carol.open(Buffer.from(m2, 'hex')).text === 'native bob to all',
  );

  // --- G4: a WASM removal commit evicts the NATIVE member ------------------
  const rm = alice.remove_member('did:example:bob');
  carol.apply_control(rm);
  // bob applies the commit that removes him (or errors) — either way blind after.
  await native({ op: 'apply_control', name: 'bob', data: Buffer.from(rm).toString('hex') }).catch(() => {});
  await assertStateAgreement('after wasm-removes-native roll', [['alice', alice], ['carol', carol]], []);
  const post = alice.seal('alice', 'post-roll: bob must not read');
  check('G4 remaining wasm member reads post-roll', carol.open(post).text === 'post-roll: bob must not read');
  const bobPost = await native({ op: 'open', name: 'bob', data: Buffer.from(post).toString('hex') })
    .then(() => true).catch(() => false);
  check('G4 removed native member is forward-blind', bobPost === false);
} catch (e) {
  console.log(`FAIL (exception) ${e.message}`);
  failures.push('exception');
}

peer.stdin.end();
peer.kill();

if (failures.length === 0) {
  console.log('P2 VERDICT: PASS — no cross-build asymmetry');
  process.exit(0);
} else {
  console.log(`P2 VERDICT: FAIL — ${failures.length} failing: ${failures.join(', ')}`);
  process.exit(1);
}
