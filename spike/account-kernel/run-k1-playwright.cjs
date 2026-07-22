// K1 automated run via Playwright. Measures H1: does a kernel.<site> iframe embedded under two
// different same-site subdomains share one storage (unpartitioned), or is it partitioned?
//
// Parametrized by env:
//   PW_BROWSER : chromium | webkit | firefox   (default chromium)
//   PW_EXEC    : executablePath for chromium (when the bundled build is missing)
//   APPA_URL   : skin A url (default http://app-a.localhost:8080/app/)
//   APPB_URL   : skin B url (default http://app-b.localhost:8080/app/)
// The kernel origin is whatever each app page targets (derived locally, or baked KERNEL_ORIGIN on deploy).
//
// Checks: (1) round-trip (A writes+reads its own value), (2) H1 (A writes, B reads, same context),
// (3) isolation (a FRESH context reads null).

const pw = require("playwright");
const BROWSER = process.env.PW_BROWSER || "chromium";
const APPA_URL = process.env.APPA_URL || "http://app-a.localhost:8080/app/";
const APPB_URL = process.env.APPB_URL || "http://app-b.localhost:8080/app/";
const KEY = "k1-probe";

async function rpc(page, op, val) {
  return await page.evaluate(
    async ({ op, key, val }) => {
      const iframe = document.getElementById("kernel");
      const kernelOrigin = new URL(iframe.src).origin;
      for (let i = 0; i < 50 && !iframe.contentWindow; i++)
        await new Promise((r) => setTimeout(r, 100));
      const once = () =>
        new Promise((resolve, reject) => {
          const id = Math.random();
          const h = (e) => {
            if (e.origin === kernelOrigin && e.data && e.data.id === id) {
              window.removeEventListener("message", h);
              resolve(e.data.result);
            }
          };
          window.addEventListener("message", h);
          iframe.contentWindow.postMessage({ id, op, key, val }, kernelOrigin);
          setTimeout(() => {
            window.removeEventListener("message", h);
            reject(new Error("rpc timeout"));
          }, 3000);
        });
      let lastErr;
      for (let i = 0; i < 15; i++) {
        try { return await once(); } catch (e) { lastErr = e; await new Promise((r) => setTimeout(r, 300)); }
      }
      throw lastErr;
    },
    { op, key: KEY, val },
  );
}

async function openApp(context, url, label) {
  const page = await context.newPage();
  page.on("console", (m) => { if (/ERR|error|Exception/i.test(m.text())) console.log(`  [${label} console] ${m.text()}`); });
  await page.goto(url, { waitUntil: "networkidle", timeout: 30000 });
  return page;
}

(async () => {
  const bt = pw[BROWSER];
  const launchOpts = BROWSER === "chromium" && process.env.PW_EXEC ? { executablePath: process.env.PW_EXEC } : {};
  const browser = await bt.launch(launchOpts);
  const out = { engine: BROWSER, browser: browser.version(), appAUrl: APPA_URL, appBUrl: APPB_URL, checks: {} };
  try {
    const ctx = await browser.newContext();
    const nonce = `A@${Date.now()}-${Math.floor(Math.random() * 1e9)}`;

    const a = await openApp(ctx, APPA_URL, "app-a");
    out.appAOrigin = (await rpc(a, "whoami")).origin;
    await rpc(a, "write", nonce);
    const aRead = await rpc(a, "read");
    out.checks.roundTrip = { idb: aRead.idb === nonce, opfs: aRead.opfs === nonce, raw: aRead };

    const b = await openApp(ctx, APPB_URL, "app-b");
    out.appBOrigin = (await rpc(b, "whoami")).origin;
    const bRead = await rpc(b, "read");
    out.checks.h1Shared = { idb: bRead.idb === nonce, opfs: bRead.opfs === nonce, raw: bRead };
    out.nonce = nonce;
    await ctx.close();

    const ctx2 = await browser.newContext();
    const c = await openApp(ctx2, APPB_URL, "app-b-fresh");
    const cRead = await rpc(c, "read");
    out.checks.freshContextEmpty = { idb: cRead.idb == null, opfs: cRead.opfs == null, raw: cRead };
    await ctx2.close();

    const shared = out.checks.h1Shared.idb || out.checks.h1Shared.opfs;
    const roundTrip = out.checks.roundTrip.idb || out.checks.roundTrip.opfs;
    const isolated = out.checks.freshContextEmpty.idb && out.checks.freshContextEmpty.opfs;
    out.verdict = !roundTrip
      ? "INVALID (probe round-trip failed)"
      : !isolated
      ? "INVALID (fresh context not empty)"
      : shared
      ? "H1 PASS (same-site subdomain embeds share storage)"
      : "H1 FAIL (partitioned: app-b did not see app-a's write)";
  } catch (e) {
    out.error = String(e && e.stack ? e.stack : e);
  } finally {
    await browser.close();
  }
  console.log(JSON.stringify(out, null, 2));
})();
