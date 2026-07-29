// @live — the real-origin check the hermetic spike cannot do in-sandbox.
// Proves the extension SW reads a REAL remote reader host (public->public, so
// PNA does not apply) that the PWA's own fetch cannot. Run in a NETWORKED env:
//
//   CROFT_LIVE_READER_URL="https://example.com/feed.xml" node run-live.mjs
//
// The reader URL is operator-supplied (no guessed endpoints). Pick any real feed
// whose server does NOT send Access-Control-Allow-Origin (most RSS endpoints), so
// the "PWA can't, extension can" contrast is visible. A committed hermetic ext/
// only allows localhost, so this generates a LIVE extension variant in a temp dir
// with host_permissions + allowlist scoped to exactly the reader's origin.
import http from "node:http";
import https from "node:https";
import os from "node:os";
import { readFile, writeFile, copyFile, mkdir, access } from "node:fs/promises";
import { execFileSync } from "node:child_process";
import { fileURLToPath } from "node:url";
import path from "node:path";
import { createRequire } from "node:module";

const READER_URL = process.env.CROFT_LIVE_READER_URL;
if (!READER_URL) {
  console.error('Set CROFT_LIVE_READER_URL, e.g. CROFT_LIVE_READER_URL="https://<host>/feed.xml" node run-live.mjs');
  process.exit(2);
}
const READER_ORIGIN = new URL(READER_URL).origin;

const HERE = path.dirname(fileURLToPath(import.meta.url));
const require = createRequire("/Users/cpettet/git/chasemp/CroftC/croft-pwa/package.json");
const { chromium } = require("playwright-core");
const PORT_A = 5601;

async function ensureCert() {
  const dir = path.join(os.tmpdir(), "croft-spike-cert");
  const key = path.join(dir, "key.pem"), cert = path.join(dir, "cert.pem");
  try { await access(key); await access(cert); } catch {
    await mkdir(dir, { recursive: true });
    execFileSync("openssl", ["req", "-x509", "-newkey", "rsa:2048", "-nodes", "-keyout", key,
      "-out", cert, "-days", "1", "-subj", "/CN=localhost", "-addext", "subjectAltName=DNS:localhost"], { stdio: "ignore" });
  }
  return { key: await readFile(key), cert: await readFile(cert) };
}

// Build a live extension variant scoped to the operator's reader origin.
async function buildLiveExt() {
  const dir = path.join(os.tmpdir(), "croft-live-ext");
  await mkdir(dir, { recursive: true });
  const manifest = {
    manifest_version: 3,
    name: "Croft content-fetch bridge (LIVE)",
    version: "0.0.1",
    background: { service_worker: "background.js" },
    content_scripts: [{ matches: [`https://localhost:${PORT_A}/*`], js: ["content.js"], run_at: "document_start" }],
    host_permissions: [`${READER_ORIGIN}/*`],
  };
  await writeFile(path.join(dir, "manifest.json"), JSON.stringify(manifest, null, 2));
  const bg = (await readFile(path.join(HERE, "ext", "background.js"), "utf8"))
    .replace(/const ALLOWED_ORIGINS = new Set\(\[[^\]]*\]\);/, `const ALLOWED_ORIGINS = new Set(["${READER_ORIGIN}"]);`);
  await writeFile(path.join(dir, "background.js"), bg);
  await copyFile(path.join(HERE, "ext", "content.js"), path.join(dir, "content.js"));
  return dir;
}

const CTYPE = { ".html": "text/html", ".js": "text/javascript" };
const listen = (s, p) => new Promise((r) => s.listen(p, () => r(s)));

let server, context;
try {
  const { key, cert } = await ensureCert();
  const ext = await buildLiveExt();
  server = await listen(https.createServer({ key, cert }, async (req, res) => {
    const rel = (req.url === "/" ? "/index.html" : req.url).split("?")[0];
    try {
      const body = await readFile(path.join(HERE, "pwa", rel));
      res.setHeader("Content-Type", CTYPE[path.extname(rel)] || "application/octet-stream");
      res.end(body);
    } catch { res.statusCode = 404; res.end("not found"); }
  }), PORT_A);

  context = await chromium.launchPersistentContext("", {
    channel: "chromium", ignoreHTTPSErrors: true,
    args: [`--disable-extensions-except=${ext}`, `--load-extension=${ext}`],
  });
  const page = await context.newPage();
  await page.goto(`https://localhost:${PORT_A}/`);
  await page.waitForFunction(() => window.__extReady && window.__extReady(), { timeout: 10000 }).catch(() => {});

  console.log(`reader: ${READER_URL}  (origin ${READER_ORIGIN})`);
  const direct = await page.evaluate((u) => window.__directFetch(u), READER_URL);
  console.log(`direct PWA fetch: ${direct.ok ? `ok (status ${direct.status}) — this feed sends CORS headers` : `blocked — ${direct.error}`}`);

  const viaExt = await page.evaluate((u) => window.__viaExtension(u), READER_URL);
  const ok = viaExt.ok && typeof viaExt.body === "string" && viaExt.body.length > 0;
  if (ok) {
    console.log(`extension read: ok (status ${viaExt.status}, ${viaExt.body.length} bytes)`);
    console.log(`  first 120 chars: ${JSON.stringify(viaExt.body.slice(0, 120))}`);
  } else {
    console.log(`extension read: FAILED — ${viaExt.error}`);
  }
  console.log(`\n${ok ? "LIVE GREEN — extension read a real remote reader" : "LIVE RED — see error above (egress blocked? host_permission? host down?)"}`);
  process.exitCode = ok ? 0 : 1;
} finally {
  if (context) await context.close();
  if (server) server.close();
}
