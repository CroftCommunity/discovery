import pw from '/Users/cpettet/git/chasemp/CroftC/croft-pwa/node_modules/playwright-core/index.js';
const { chromium } = pw;

const HANDLE = process.env.TEST_HANDLE;
const PASSWORD = process.env.TEST_PASSWORD;
const SHOT = process.env.SHOT_DIR ?? "/tmp";
const log = (m) => console.log(`[pw] ${m}`);

const browser = await chromium.launch({ headless: true });
const ctx = await browser.newContext();
const page = await ctx.newPage();
page.on('console', (m) => { if (m.type() === 'error') log(`page-console-error: ${m.text()}`); });

async function logText() {
  return (await page.locator('#log').textContent()) ?? '';
}

try {
  log('open stellin.app (helper should be DOWN)');
  await page.goto('https://stellin.app/', { waitUntil: 'networkidle' });
  await page.waitForTimeout(1500);
  const before = await logText();
  log(`pad log before: ${JSON.stringify(before)}`);
  await page.screenshot({ path: `${SHOT}/pw-1-fallback-detected.png` });

  log('click "3. Sign in browser-only (fallback)"');
  await page.getByRole('button', { name: /browser-only/i }).click();

  // Land on bsky OAuth. Wait for the sign-in UI.
  await page.waitForURL(/bsky\.social/, { timeout: 30000 });
  await page.waitForTimeout(2500);
  await page.screenshot({ path: `${SHOT}/pw-2-bsky-signin.png`, fullPage: true });
  log(`on ${page.url()}`);

  // Fill identifier if a text/username field is present.
  const idField = page.locator('input[autocomplete="username"], input[name="username"], input[name="identifier"], input[type="text"]').first();
  if ((await idField.count()) && (await idField.isEditable().catch(() => false))) {
    await idField.fill(HANDLE);
    log('filled identifier');
  } else {
    log('identifier prefilled/locked (login_hint) — skipping');
  }
  // Password may be on the same screen or after a "Next".
  let pw = page.locator('input[type="password"]').first();
  if (!(await pw.count()) || !(await pw.isVisible().catch(() => false))) {
    const next = page.getByRole('button', { name: /next|continue/i }).first();
    if (await next.count()) { await next.click(); await page.waitForTimeout(1500); }
    pw = page.locator('input[type="password"]').first();
  }
  await pw.waitFor({ state: 'visible', timeout: 15000 });
  await pw.fill(PASSWORD);
  log('filled password');
  await page.screenshot({ path: `${SHOT}/pw-3-filled.png`, fullPage: true });

  await page.getByRole('button', { name: /sign in|log in|next|continue/i }).first().click();
  await page.waitForTimeout(3500);
  await page.screenshot({ path: `${SHOT}/pw-4-after-signin.png`, fullPage: true });
  log(`after sign-in on ${page.url()}`);

  // Consent / authorize screen, if shown.
  const authorize = page.getByRole('button', { name: /authorize|allow|accept|continue|grant/i }).first();
  if (await authorize.count().catch(() => 0)) {
    log('clicking authorize/consent');
    await authorize.click().catch(() => {});
  }

  // Back to the pad.
  await page.waitForURL(/stellin\.app/, { timeout: 30000 });
  await page.waitForTimeout(2000);
  const after = await logText();
  log(`pad log after: ${JSON.stringify(after)}`);
  await page.screenshot({ path: `${SHOT}/pw-5-signed-in-browser-only.png` });

  const ok = /signed in BROWSER-ONLY \(public client\)\. DID: did:/.test(after);
  log(ok ? 'RESULT: PASS — browser-only fallback login completed' : 'RESULT: FAIL — no browser-only success line');
  process.exit(ok ? 0 : 1);
} catch (e) {
  log(`ERROR: ${e.message}`);
  await page.screenshot({ path: `${SHOT}/pw-error.png`, fullPage: true }).catch(() => {});
  process.exit(2);
} finally {
  await browser.close();
}
