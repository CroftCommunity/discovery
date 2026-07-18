// RUN-19 P3/P5 — a wasm-hosted ndjson peer: the same line protocol as the
// native seal-peer, but every op executes inside the wasm module hosted by
// THIS Node process (SPEC-DELTA[run19-node-runner]). The persist-capable
// member's encrypted state blob lives on the host filesystem
// (SPEC-DELTA[run19-storage-shim] — the Node-side backing standing in for
// IndexedDB/OPFS; the parent can SIGKILL this host at any moment, which is
// the point of the P3 drill).
//
// Usage: node wasm-peer.mjs --state-file <path>
//   persist ops: {op:"persist", name, key(hex16)} → blob written to state file
//                {op:"resume", name, key(hex16)}  → member rebuilt from it

import { readFileSync, writeFileSync } from 'node:fs';
import { createInterface } from 'node:readline';
import { createRequire } from 'node:module';
import path from 'node:path';
import { fileURLToPath } from 'node:url';

const here = path.dirname(fileURLToPath(import.meta.url));
const require = createRequire(import.meta.url);
const wasm = require(path.join(here, '..', 'crates', 'seal-wasm', 'pkg', 'seal_wasm.js'));

const args = process.argv.slice(2);
const stateFile = args[args.indexOf('--state-file') + 1];

const members = new Map(); // name -> JsPersistSealer

const hex = (buf) => Buffer.from(buf).toString('hex');
const unhex = (s) => Buffer.from(s, 'hex');

function handle(req) {
  const m = (name) => {
    const s = members.get(name);
    if (!s) throw new Error(`no such member: ${name}`);
    return s;
  };
  switch (req.op) {
    case 'found':
      members.set(req.name, wasm.JsPersistSealer.found(req.did));
      return { ok: true };
    case 'enroll':
      members.set(req.name, wasm.JsPersistSealer.enroll(req.did));
      return { ok: true };
    case 'key_package':
      return { ok: true, data: hex(m(req.name).key_package()) };
    case 'invite': {
      const cw = m(req.name).invite(unhex(req.kp));
      return { ok: true, commit: hex(cw.commit), welcome: hex(cw.welcome) };
    }
    case 'accept_welcome':
      m(req.name).accept_welcome(unhex(req.welcome));
      return { ok: true };
    case 'seal':
      return { ok: true, data: hex(m(req.name).seal(req.sender, req.text)) };
    case 'open': {
      const opened = m(req.name).open(unhex(req.data));
      return { ok: true, sender: opened.sender, text: opened.text };
    }
    case 'apply_control':
      m(req.name).apply_control(unhex(req.data));
      return { ok: true };
    case 'remove_member':
      return { ok: true, commit: hex(m(req.name).remove_member(req.did)) };
    case 'epoch':
      return { ok: true, epoch: Number(m(req.name).epoch()) };
    case 'epoch_secret':
      return { ok: true, data: m(req.name).epoch_secret_hex() };
    case 'persist': {
      // The encrypted blob leaves the module; the HOST stores it.
      const blob = m(req.name).snapshot(unhex(req.key));
      writeFileSync(stateFile, Buffer.from(blob));
      return { ok: true, bytes: blob.length };
    }
    case 'resume': {
      const blob = readFileSync(stateFile);
      members.set(req.name, wasm.JsPersistSealer.restore(unhex(req.key), blob));
      return { ok: true };
    }
    default:
      throw new Error(`unknown op: ${req.op}`);
  }
}

const rl = createInterface({ input: process.stdin });
rl.on('line', (line) => {
  if (!line.trim()) return;
  let res;
  try {
    res = handle(JSON.parse(line));
  } catch (e) {
    res = { ok: false, error: String(e.message ?? e) };
  }
  process.stdout.write(`${JSON.stringify(res)}\n`);
});
