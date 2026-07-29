// Hermetic spike: prove a browser extension grants a static PWA a cross-origin
// read that the PWA's own fetch() cannot do (same-origin policy / CORS).
//
//   origin A (5601) = the static PWA        origin B (5602) = a reader server
//   that serves feed.xml with NO Access-Control-Allow-Origin header.
//
// Test 1 (RED baseline): A.fetch(B) is blocked -> {ok:false}.
// Test 2 (GREEN):        A -> extension -> B succeeds and A receives the marker.
//
// No network egress: both origins are localhost. Playwright is borrowed from
// croft-pwa/node_modules (see CroftC/.claude/CLAUDE.md). MV3 extensions need the
// full Chromium in new-headless mode -> channel:'chromium'.
import http from "node:http";
import { readFile } from "node:fs/promises";
import { fileURLToPath } from "node:url";
import path from "node:path";
import { createRequire } from "node:module";

const HERE = path.dirname(fileURLToPath(import.meta.url));
const EXT = path.join(HERE, "ext");
const require = createRequire("/Users/cpettet/git/chasemp/CroftC/croft-pwa/package.json");
const { chromium } = require("playwright-core");

const PORT_A = 5601; // PWA
const PORT_B = 5602; // reader (no CORS)
const MARKER = "CROFT-SPIKE-FEED-MARKER-42";
const FEED_URL = `http://localhost:${PORT_B}/feed.xml`;

const CTYPE = { ".html": "text/html", ".js": "text/javascript", ".xml": "application/rss+xml" };

function serveDir(dir, port, { cors }) {
  const server = http.createServer(async (req, res) => {
    try {
      const rel = (req.url === "/" ? "/index.html" : req.url).split("?")[0];
      const body = await readFile(path.join(dir, rel));
      // Deliberately: origin B sends NO Access-Control-Allow-Origin header.
      if (cors) res.setHeader("Access-Control-Allow-Origin", "*");
      res.setHeader("Content-Type", CTYPE[path.extname(rel)] || "application/octet-stream");
      res.end(body);
    } catch {
      res.statusCode = 404;
      res.end("not found");
    }
  });
  return new Promise((resolve) => server.listen(port, () => resolve(server)));
}

let servers = [];
let context;
const results = [];
function record(name, pass, detail) {
  results.push({ name, pass, detail });
  console.log(`${pass ? "PASS" : "FAIL"}  ${name}${detail ? "  — " + detail : ""}`);
}

try {
  servers.push(await serveDir(path.join(HERE, "pwa"), PORT_A, { cors: false }));
  servers.push(await serveDir(path.join(HERE, "reader"), PORT_B, { cors: false })); // no CORS on purpose

  context = await chromium.launchPersistentContext("", {
    channel: "chromium",
    args: [`--disable-extensions-except=${EXT}`, `--load-extension=${EXT}`],
  });

  // Wait for the MV3 background service worker to register.
  let sw = context.serviceWorkers()[0];
  if (!sw) sw = await context.waitForEvent("serviceworker", { timeout: 10000 }).catch(() => null);
  record("extension service worker registered", !!sw, sw ? sw.url() : "none");

  const page = await context.newPage();
  await page.goto(`http://localhost:${PORT_A}/`);
  await page.waitForFunction(() => window.__extReady && window.__extReady(), { timeout: 10000 })
    .catch(() => {});

  // Test 1: the PWA's own fetch is blocked by CORS (the problem we are solving).
  const direct = await page.evaluate((u) => window.__directFetch(u), FEED_URL);
  record(
    "direct PWA fetch is CORS-blocked (baseline)",
    direct.ok === false,
    direct.ok ? `unexpectedly ok (status ${direct.status})` : direct.error,
  );

  // Test 2: routed through the extension, the PWA receives origin B's content.
  const viaExt = await page.evaluate((u) => window.__viaExtension(u), FEED_URL);
  const got = viaExt.ok && typeof viaExt.body === "string" && viaExt.body.includes(MARKER);
  record(
    "extension bridge delivers cross-origin content",
    got,
    viaExt.ok ? `status ${viaExt.status}, marker ${viaExt.body?.includes(MARKER) ? "found" : "MISSING"}` : viaExt.error,
  );
} finally {
  if (context) await context.close();
  for (const s of servers) s.close();
}

const passed = results.every((r) => r.pass) && results.length === 3;
console.log(`\n${passed ? "SPIKE GREEN" : "SPIKE RED"} — ${results.filter((r) => r.pass).length}/${results.length} checks`);
process.exit(passed ? 0 : 1);
