import { readFileSync, writeFileSync, existsSync, mkdirSync } from 'node:fs';
import { dirname } from 'node:path';
import { jwkThumbprint, type PublicJwk } from './oauth/dpop.ts';
import { type AssertionKey } from './assertion.ts';
import { importStoreKey, exportStoreKey, newStoreKey } from './store.ts';

// Load-or-create the server-held secret material (FLOW-SPEC §7, D5). Files are
// written mode-0600 outside the repo. The ES256 private key signs client
// assertions; the AES key encrypts the session store at rest. Neither is ever
// logged or serialized in the clear beyond these 0600 files.

function ensureDir(file: string): void {
  mkdirSync(dirname(file), { recursive: true });
}

/** Load the confidential client's ES256 assertion key, generating+persisting it if absent. */
export async function loadOrCreateAssertionKey(
  keyFile: string,
): Promise<{ key: AssertionKey; publicJwk: PublicJwk }> {
  let privJwk: JsonWebKey;
  if (existsSync(keyFile)) {
    privJwk = JSON.parse(readFileSync(keyFile, 'utf8')) as JsonWebKey;
  } else {
    const pair = await crypto.subtle.generateKey({ name: 'ECDSA', namedCurve: 'P-256' }, true, ['sign', 'verify']);
    privJwk = (await crypto.subtle.exportKey('jwk', pair.privateKey)) as JsonWebKey;
    ensureDir(keyFile);
    writeFileSync(keyFile, JSON.stringify(privJwk), { mode: 0o600 });
  }
  const privateKey = await crypto.subtle.importKey(
    'jwk',
    privJwk,
    { name: 'ECDSA', namedCurve: 'P-256' },
    false,
    ['sign'],
  );
  if (!privJwk.x || !privJwk.y) throw new Error('assertion key JWK missing EC coordinates');
  const publicJwk: PublicJwk = { kty: 'EC', crv: 'P-256', x: privJwk.x, y: privJwk.y };
  const kid = await jwkThumbprint(publicJwk);
  return { key: { privateKey, kid }, publicJwk };
}

/** Load the AES store key, generating+persisting (mode-0600) it if absent. */
export async function loadOrCreateStoreKey(keyFile: string): Promise<CryptoKey> {
  if (existsSync(keyFile)) {
    return importStoreKey(new Uint8Array(readFileSync(keyFile)));
  }
  const key = await newStoreKey();
  const raw = await exportStoreKey(key);
  ensureDir(keyFile);
  writeFileSync(keyFile, raw, { mode: 0o600 });
  return importStoreKey(raw);
}
