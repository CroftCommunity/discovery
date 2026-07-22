// KC0 SharedWorker (same-origin coordinator probe). Holds an in-memory map shared across
// all same-origin pages that connect. Tests whether a SharedWorker can coordinate two
// same-origin tabs on WebKit (the single-origin kernel's coordination mechanism).
const store = new Map();
onconnect = (e) => {
  const port = e.ports[0];
  port.addEventListener("message", (ev) => {
    const { id, op, key, val } = ev.data || {};
    if (op === "set") { store.set(key, val); port.postMessage({ id, ok: true }); }
    else if (op === "get") { port.postMessage({ id, val: store.has(key) ? store.get(key) : null }); }
  });
  port.start();
};
