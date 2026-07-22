// KC0 single-origin probe. Exposes window.kc0 with IndexedDB, OPFS, and SharedWorker helpers.
// Two same-origin pages both load this; one writes, the other reads. Same-origin storage is
// expected to be shared on every engine (unlike the cross-subdomain case K1 refuted on WebKit).

const DB = "kc0-db", ST = "kv";
function idbOpen() {
  return new Promise((resolve, reject) => {
    const r = indexedDB.open(DB, 1);
    r.onupgradeneeded = () => r.result.createObjectStore(ST);
    r.onsuccess = () => resolve(r.result);
    r.onerror = () => reject(r.error);
  });
}
async function idbPut(k, v) {
  const db = await idbOpen();
  await new Promise((res, rej) => { const tx = db.transaction(ST, "readwrite"); tx.objectStore(ST).put(v, k); tx.oncomplete = res; tx.onerror = () => rej(tx.error); });
  db.close();
}
async function idbGet(k) {
  const db = await idbOpen();
  const v = await new Promise((res, rej) => { const tx = db.transaction(ST, "readonly"); const q = tx.objectStore(ST).get(k); q.onsuccess = () => res(q.result ?? null); q.onerror = () => rej(q.error); });
  db.close();
  return v;
}
async function opfsWrite(k, v) {
  const root = await navigator.storage.getDirectory();
  const fh = await root.getFileHandle(`kc0-${k}.txt`, { create: true });
  const w = await fh.createWritable(); await w.write(String(v)); await w.close();
}
async function opfsRead(k) {
  try {
    const root = await navigator.storage.getDirectory();
    const fh = await root.getFileHandle(`kc0-${k}.txt`, { create: false });
    return await (await fh.getFile()).text();
  } catch (e) { if (e && e.name === "NotFoundError") return null; throw e; }
}

let port = null;
function sw() {
  if (!("SharedWorker" in self)) return null;
  if (!port) { const s = new SharedWorker("kc0-worker.js"); port = s.port; port.start(); }
  return port;
}
function swRpc(op, key, val) {
  return new Promise((resolve, reject) => {
    const p = sw();
    if (!p) return resolve({ unsupported: true });
    const id = Math.random();
    const h = (e) => { if (e.data && e.data.id === id) { p.removeEventListener("message", h); resolve(e.data); } };
    p.addEventListener("message", h);
    p.postMessage({ id, op, key, val });
    setTimeout(() => { p.removeEventListener("message", h); reject(new Error("sw timeout")); }, 3000);
  });
}

window.kc0 = {
  whoami: async () => ({ origin: location.origin, secure: isSecureContext, sharedWorker: "SharedWorker" in self }),
  idbPut, idbGet, opfsWrite, opfsRead,
  swSet: (k, v) => swRpc("set", k, v),
  swGet: (k) => swRpc("get", k),
};
