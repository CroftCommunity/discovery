// v2 — walk out the honest edges the hermetic v1 spike left open, as far as is
// possible without network egress or a second browser engine:
//
//   Edge 1  Secure-context (HTTPS page) mixed-content sidestep — the finding that
//           the extension model avoids the mixed-content surface the proxy has.
//   Edge 2  Consent/abuse: the extension enforces a per-origin allowlist itself.
//   Edge 3  Install-flow core: the PWA detects extension presence AND absence.
//
// Not walked here (documented in README, not faked):
//   - real remote reader host @live (sandbox blocks browser egress)
//   - Firefox MV3 parity (Playwright cannot --load-extension in Firefox)
//
// A throwaway self-signed cert is generated into the OS temp dir (never committed).
import http from "node:http";
import https from "node:https";
import os from "node:os";
import { readFile, mkdir, access } from "node:fs/promises";
import { execFileSync } from "node:child_process";
import { fileURLToPath } from "node:url";
import path from "node:path";
import { createRequire } from "node:module";

const HERE = path.dirname(fileURLToPath(import.meta.url));
const EXT = path.join(HERE, "ext");
const require = createRequire("/Users/cpettet/git/chasemp/CroftC/croft-pwa/package.json");
const { chromium } = require("playwright-core");

const PORT_A = 5601; // HTTPS PWA (matches manifest https://localhost:5601/*)
const PORT_B = 5602; // reader, ALLOWLISTED
const PORT_C = 5603; // reader, NOT allowlisted
const MARKER = "CROFT-SPIKE-FEED-MARKER-42";
const CTYPE = { ".html": "text/html", ".js": "text/javascript", ".xml": "application/rss+xml" };

const results = [];
function record(name, pass, detail) {
  results.push({ name, pass });
  console.log(`${pass ? "PASS" : "FAIL"}  ${name}${detail ? "  — " + detail : ""}`);
}

async function ensureCert() {
  const dir = path.join(os.tmpdir(), "croft-spike-cert");
  const key = path.join(dir, "key.pem");
  const cert = path.join(dir, "cert.pem");
  try {
    await access(key);
    await access(cert);
  } catch {
    await mkdir(dir, { recursive: true });
    execFileSync("openssl", [
      "req", "-x509", "-newkey", "rsa:2048", "-nodes",
      "-keyout", key, "-out", cert, "-days", "1",
      "-subj", "/CN=localhost", "-addext", "subjectAltName=DNS:localhost",
    ], { stdio: "ignore" });
  }
  return { key: await readFile(key), cert: await readFile(cert) };
}

function handler(dir) {
  return async (req, res) => {
    try {
      const rel = (req.url === "/" ? "/index.html" : req.url).split("?")[0];
      const body = await readFile(path.join(dir, rel));
      // Readers deliberately send NO Access-Control-Allow-Origin header.
      res.setHeader("Content-Type", CTYPE[path.extname(rel)] || "application/octet-stream");
      res.end(body);
    } catch {
      res.statusCode = 404;
      res.end("not found");
    }
  };
}
const listen = (server, port) => new Promise((r) => server.listen(port, () => r(server)));

const servers = [];
let ctxWith, ctxWithout;
try {
  const { key, cert } = await ensureCert();
  servers.push(await listen(https.createServer({ key, cert }, handler(path.join(HERE, "pwa"))), PORT_A));
  servers.push(await listen(http.createServer(handler(path.join(HERE, "reader"))), PORT_B));
  servers.push(await listen(http.createServer(handler(path.join(HERE, "reader"))), PORT_C));

  const launch = (withExt) =>
    chromium.launchPersistentContext("", {
      channel: "chromium",
      ignoreHTTPSErrors: true,
      args: withExt ? [`--disable-extensions-except=${EXT}`, `--load-extension=${EXT}`] : [],
    });

  // ---- Edges 1 & 2: extension present, HTTPS secure-context page ----
  ctxWith = await launch(true);
  const p = await ctxWith.newPage();
  await p.goto(`https://localhost:${PORT_A}/`);
  await p.waitForFunction(() => window.__extReady && window.__extReady(), { timeout: 10000 }).catch(() => {});

  const secure = await p.evaluate(() => window.isSecureContext);
  record("page is a genuine secure context (HTTPS)", secure === true, `isSecureContext=${secure}`);

  const directHttp = await p.evaluate((u) => window.__directFetch(u), `http://localhost:${PORT_B}/feed.xml`);
  record("HTTPS page cannot fetch HTTP reader directly (mixed-content/CORS)", directHttp.ok === false, directHttp.error);

  const viaExt = await p.evaluate((u) => window.__viaExtension(u), `http://localhost:${PORT_B}/feed.xml`);
  const sidestep = viaExt.ok && typeof viaExt.body === "string" && viaExt.body.includes(MARKER);
  record("extension delivers HTTP content to the HTTPS page (mixed-content sidestep)", sidestep,
    viaExt.ok ? `status ${viaExt.status}` : viaExt.error);

  const refused = await p.evaluate((u) => window.__viaExtension(u), `http://localhost:${PORT_C}/feed.xml`);
  record("extension REFUSES a non-allowlisted origin (consent gate)", refused.ok === false && refused.refused === true, refused.error);

  // ---- Edge 3: extension absent, PWA detects it and degrades gracefully ----
  ctxWithout = await launch(false);
  const p2 = await ctxWithout.newPage();
  await p2.goto(`https://localhost:${PORT_A}/`);
  await p2.waitForTimeout(1000);
  const readyNoExt = await p2.evaluate(() => window.__extReady());
  record("PWA detects extension ABSENCE (install-flow signal)", readyNoExt === false, `__extReady=${readyNoExt}`);

  const viaNoExt = await p2.evaluate((u) => window.__viaExtension(u), `http://localhost:${PORT_B}/feed.xml`);
  record("without the extension the bridge fails gracefully (no hang)", viaNoExt.ok === false, viaNoExt.error);
} finally {
  if (ctxWith) await ctxWith.close();
  if (ctxWithout) await ctxWithout.close();
  for (const s of servers) s.close();
}

const passed = results.every((r) => r.pass) && results.length === 6;
console.log(`\n${passed ? "EDGES GREEN" : "EDGES RED"} — ${results.filter((r) => r.pass).length}/${results.length} checks`);
process.exit(passed ? 0 : 1);
