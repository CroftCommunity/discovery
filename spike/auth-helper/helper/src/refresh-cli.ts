import { readFileSync, writeFileSync, existsSync, appendFileSync } from 'node:fs';
import { join } from 'node:path';
import { confidentialRefresh, type OAuthSession } from './confidential.ts';
import { assertionSigner } from './assertion.ts';
import { loadOrCreateAssertionKey, loadOrCreateStoreKey } from './keystore.ts';
import { encryptJson, decryptJson } from './store.ts';

// Server-side refresh with NO browser (FLOW-SPEC §6 step 9 — the thing the spike
// proves). Loads a stored session by DID, refreshes it, records the measured
// TTLs + whether the refresh token rotated, and re-stores the rotated session.
// Usage: node src/refresh-cli.ts <did>

const DATA_DIR = process.env.AUTH_HELPER_DATA_DIR ?? join(process.cwd(), 'data');
const ORIGIN = process.env.AUTH_HELPER_ORIGIN ?? 'https://account.croft.ing';
const CLIENT_ID = `${ORIGIN}/client-metadata.json`;

const did = process.argv[2];
if (!did) {
  console.error('usage: refresh-cli <did>');
  process.exit(2);
}

const safe = (s: string): string => s.replace(/[^A-Za-z0-9._-]/g, '_');
const file = join(DATA_DIR, 'sessions', `${safe(did)}.enc`);
if (!existsSync(file)) {
  console.error(`no stored session for ${did} at ${file}`);
  process.exit(1);
}

const { key: assertionKey } = await loadOrCreateAssertionKey(join(DATA_DIR, 'assertion-key.jwk'));
const storeKey = await loadOrCreateStoreKey(join(DATA_DIR, 'store-key.bin'));
const sign = assertionSigner(CLIENT_ID, assertionKey);

const before = (await decryptJson(storeKey, new Uint8Array(readFileSync(file)))) as OAuthSession;
const oldRefresh = before.refreshToken;

const after = await confidentialRefresh(before, sign);
writeFileSync(file, await encryptJson(storeKey, after), { mode: 0o600 });

const ttl = after.expiresAt ? Math.round((after.expiresAt - Date.now()) / 1000) : undefined;
const rotated = after.refreshToken !== oldRefresh;
const line = `refresh did=${did} access_expires_in=${ttl}s refresh_rotated=${rotated}`;
appendFileSync(join(DATA_DIR, 'measurements.log'), `${new Date().toISOString()} ${line}\n`);
console.log(`[refresh] ${line}`);
console.log(`[refresh] new access token acquired server-side, no browser. refresh token ${rotated ? 'ROTATED (single-use, as spec)' : 'unchanged'}.`);
