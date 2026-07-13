// Per-state snapshot harness (spec 1.5). Loads the deterministic gallery and
// captures one PNG per required state. Every state in REQUIRED must be present
// and visible, or the harness exits non-zero — so an undesigned/missing state
// is a failing test. Also loads the live feed and asserts it renders real
// public Bluesky data (spec 1.5 DoD).
//
// Usage: node harness.mjs <baseUrl>   (browsers in PLAYWRIGHT_BROWSERS_PATH)

// Resolve playwright from the global install (NODE_PATH is ignored for ESM).
const pw = await import(
  process.env.PLAYWRIGHT_MODULE || "/opt/node22/lib/node_modules/playwright/index.js"
);
const chromium = pw.chromium ?? pw.default?.chromium;
import { mkdirSync } from "node:fs";
import { dirname, join } from "node:path";
import { fileURLToPath } from "node:url";

const base = process.argv[2] || "http://127.0.0.1:8088";
const here = dirname(fileURLToPath(import.meta.url));
const outDir = join(here, "images");
mkdirSync(outDir, { recursive: true });

// The required state set — must match the gallery's section ids.
const REQUIRED = [
  "feed-loading",
  "feed-more",
  "feed-end",
  "feed-loading-more",
  "feed-empty",
  "feed-error-cold",
  "feed-error-appended",
  "card-standard",
  "card-no-avatar",
  "card-long-text",
  "card-long-name",
  // Phase 2 shell surfaces
  "shell-pinned-empty",
  "shell-pinned-items",
  "shell-pinned-degraded",
  "shell-panel-in-slot",
];

const fail = (msg) => {
  console.error(`✗ ${msg}`);
  process.exitCode = 1;
};

const browser = await chromium.launch();
try {
  const page = await browser.newPage({
    viewport: { width: 720, height: 900 },
    deviceScaleFactor: 2,
    // This sandbox terminates TLS with a private CA Chromium doesn't trust.
    // Ignoring it is a harness-only accommodation for the test environment; the
    // app uses the browser's normal fetch (real CAs) everywhere else.
    ignoreHTTPSErrors: true,
  });

  // --- deterministic gallery: every required state present + snapshotted ---
  await page.goto(`${base}/?gallery`, { waitUntil: "networkidle" });
  await page.waitForSelector('[data-snapshot="card-standard"]', { timeout: 15000 });

  let missing = 0;
  for (const id of REQUIRED) {
    const el = page.locator(`[data-snapshot="${id}"]`);
    if ((await el.count()) === 0) {
      fail(`required state missing a snapshot section: ${id}`);
      missing++;
      continue;
    }
    await el.scrollIntoViewIfNeeded();
    await el.screenshot({ path: join(outDir, `${id}.png`) });
    console.log(`✓ ${id}`);
  }
  await page.screenshot({ path: join(outDir, "_gallery-full.png"), fullPage: true });

  // --- live feed: renders real public Bluesky data ---
  await page.goto(base, { waitUntil: "domcontentloaded" });
  try {
    await page.waitForFunction(
      () => document.body.innerText.includes("@bsky.app"),
      { timeout: 25000 }
    );
    await page.screenshot({ path: join(outDir, "_live-feed.png"), fullPage: true });
    console.log("✓ live feed rendered real public data");

    // Pin a post, reload, and confirm it persisted (layout document survives a
    // restart — M2.3/M2.4). A fresh context starts with an empty strip.
    await page.getByRole("button", { name: "Pin", exact: true }).first().click();
    await page.waitForTimeout(500);
    await page.reload({ waitUntil: "domcontentloaded" });
    await page.waitForFunction(
      () => !document.body.innerText.includes("Pin a post to keep it up here"),
      { timeout: 20000 }
    );
    await page.screenshot({ path: join(outDir, "_live-pinned.png"), fullPage: true });
    console.log("✓ a pinned item persisted across reload");
  } catch (e) {
    fail(`live feed / pin-persistence check failed: ${e.message ?? e}`);
  }

  if (missing === 0 && !process.exitCode) {
    console.log(`\nAll ${REQUIRED.length} required snapshots captured to ${outDir}`);
  }
} finally {
  await browser.close();
}
