// KC0 runner: on ONE origin, two same-origin pages — does page B see page A's writes to
// IndexedDB, OPFS, and a shared SharedWorker? Env: PW_BROWSER (chromium|webkit), PW_EXEC, KC0_URL.
const pw = require("playwright");
const BROWSER = process.env.PW_BROWSER || "chromium";
const URL = process.env.KC0_URL || "http://localhost:8080/kc0/";

(async () => {
  const bt = pw[BROWSER];
  const launchOpts = BROWSER === "chromium" && process.env.PW_EXEC ? { executablePath: process.env.PW_EXEC } : {};
  const browser = await bt.launch(launchOpts);
  const out = { engine: BROWSER, browser: browser.version(), url: URL };
  try {
    const ctx = await browser.newContext();
    const nonce = `SO@${Date.now()}-${Math.floor(Math.random() * 1e9)}`;

    const a = await ctx.newPage();
    a.on("console", (m) => { if (/ERR|error|Exception/i.test(m.text())) console.log("  [A]", m.text()); });
    await a.goto(URL, { waitUntil: "networkidle", timeout: 30000 });
    out.who = await a.evaluate(() => window.kc0.whoami());
    out.wrote = await a.evaluate(async (n) => {
      const r = {};
      try { await window.kc0.idbPut("k", n); r.idb = "ok"; } catch (e) { r.idb = "ERR " + e; }
      try { await window.kc0.opfsWrite("k", n); r.opfs = "ok"; } catch (e) { r.opfs = "ERR " + e; }
      try { r.sw = await window.kc0.swSet("k", n); } catch (e) { r.sw = "ERR " + e; }
      return r;
    }, nonce);

    const b = await ctx.newPage();
    b.on("console", (m) => { if (/ERR|error|Exception/i.test(m.text())) console.log("  [B]", m.text()); });
    await b.goto(URL, { waitUntil: "networkidle", timeout: 30000 });
    const read = await b.evaluate(async () => {
      const r = {};
      try { r.idb = await window.kc0.idbGet("k"); } catch (e) { r.idb = "ERR " + e; }
      try { r.opfs = await window.kc0.opfsRead("k"); } catch (e) { r.opfs = "ERR " + e; }
      try { r.sw = await window.kc0.swGet("k"); } catch (e) { r.sw = "ERR " + e; }
      return r;
    });
    out.read = read;
    out.checks = {
      idbShared: read.idb === nonce,
      opfsShared: read.opfs === nonce,
      swSupported: !!out.who.sharedWorker,
      swShared: !!(read.sw && read.sw.val === nonce),
    };
    out.nonce = nonce;
    await ctx.close();
  } catch (e) { out.error = String(e && e.stack ? e.stack : e); }
  finally { await browser.close(); }
  console.log(JSON.stringify(out, null, 2));
})();
