import { describe, it, expect } from 'vitest';
import { encryptJson, decryptJson, newStoreKey } from '../src/store.ts';

// Sessions (access+refresh tokens, DPoP private JWK) are encrypted at rest
// (FLOW-SPEC §7, task constraint). AES-256-GCM: a tampered ciphertext must fail
// the auth tag rather than decrypt to garbage.

describe('encrypted session store', () => {
  it('round-trips an object through encrypt→decrypt', async () => {
    const key = await newStoreKey();
    const obj = { did: 'did:plc:x', refreshToken: 'rt-secret', n: 42 };
    const blob = await encryptJson(key, obj);
    expect(await decryptJson(key, blob)).toEqual(obj);
  });

  it('rejects a tampered ciphertext (GCM auth tag)', async () => {
    const key = await newStoreKey();
    const blob = await encryptJson(key, { a: 1 });
    const last = blob.length - 1;
    blob[last] = (blob[last] ?? 0) ^ 0xff; // flip a byte in the tag/ciphertext
    await expect(decryptJson(key, blob)).rejects.toThrow();
  });

  it('does not decrypt under a different key', async () => {
    const k1 = await newStoreKey();
    const k2 = await newStoreKey();
    const blob = await encryptJson(k1, { secret: 'no' });
    await expect(decryptJson(k2, blob)).rejects.toThrow();
  });
});
