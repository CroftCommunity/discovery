// A small systematic Reed-Solomon erasure code over GF(256): split data into k
// shares, expand to n, and recover the original from ANY k of the n. This is the
// upgrade path from probabilistic spot checks (E5) to a retrievability guarantee:
// with k-of-n coding a collection survives the loss of up to n-k shares outright,
// rather than merely detecting loss after the fact.
//
// SEAM: a production system would use a hardened, wide-word RS/LDPC library
// (e.g. leopard, ISA-L). This is a compact, readable Cauchy-matrix implementation
// sufficient to demonstrate the threshold behavior and the loss math.

// --- GF(256) arithmetic (primitive polynomial 0x11d) ---
const EXP = new Uint8Array(512);
const LOG = new Uint8Array(256);
(function initTables() {
  let x = 1;
  for (let i = 0; i < 255; i++) {
    EXP[i] = x;
    LOG[x] = i;
    x <<= 1;
    if (x & 0x100) x ^= 0x11d;
  }
  for (let i = 255; i < 512; i++) EXP[i] = EXP[i - 255];
})();

function gmul(a: number, b: number): number {
  if (a === 0 || b === 0) return 0;
  return EXP[LOG[a] + LOG[b]];
}
function gdiv(a: number, b: number): number {
  if (b === 0) throw new Error("GF division by zero");
  if (a === 0) return 0;
  return EXP[(LOG[a] + 255 - LOG[b]) % 255];
}

/** The n x k systematic encoding matrix: identity on top, Cauchy parity below. */
function encodingMatrix(n: number, k: number): number[][] {
  const rows: number[][] = [];
  for (let i = 0; i < k; i++) {
    const row = new Array(k).fill(0);
    row[i] = 1;
    rows.push(row);
  }
  // Cauchy rows: x_i in {k..n-1}, y_j in {0..k-1}, all distinct => invertible.
  for (let i = 0; i < n - k; i++) {
    const row: number[] = [];
    const xi = k + i;
    for (let j = 0; j < k; j++) row.push(gdiv(1, xi ^ j));
    rows.push(row);
  }
  return rows;
}

/** Invert a k x k GF(256) matrix via Gauss-Jordan. Throws if singular. */
function invert(matrix: number[][]): number[][] {
  const k = matrix.length;
  const m = matrix.map((r) => r.slice());
  const inv: number[][] = Array.from({ length: k }, (_, i) =>
    Array.from({ length: k }, (_, j) => (i === j ? 1 : 0)),
  );
  for (let col = 0; col < k; col++) {
    let pivot = col;
    while (pivot < k && m[pivot][col] === 0) pivot++;
    if (pivot === k) throw new Error("singular matrix: cannot recover");
    if (pivot !== col) {
      [m[col], m[pivot]] = [m[pivot], m[col]];
      [inv[col], inv[pivot]] = [inv[pivot], inv[col]];
    }
    const p = m[col][col];
    for (let j = 0; j < k; j++) {
      m[col][j] = gdiv(m[col][j], p);
      inv[col][j] = gdiv(inv[col][j], p);
    }
    for (let r = 0; r < k; r++) {
      if (r === col || m[r][col] === 0) continue;
      const f = m[r][col];
      for (let j = 0; j < k; j++) {
        m[r][j] ^= gmul(f, m[col][j]);
        inv[r][j] ^= gmul(f, inv[col][j]);
      }
    }
  }
  return inv;
}

export type Share = { index: number; bytes: Buffer };

/** Encode data into n shares (k-of-n). Data is padded to a multiple of k. */
export function encode(data: Buffer, n: number, k: number): { shares: Share[]; originalLength: number } {
  if (k < 1 || n < k) throw new Error("require 1 <= k <= n");
  const shardLen = Math.ceil(data.length / k);
  const padded = Buffer.alloc(shardLen * k);
  data.copy(padded);
  const dataShards: Buffer[] = [];
  for (let j = 0; j < k; j++) dataShards.push(padded.subarray(j * shardLen, (j + 1) * shardLen));
  const G = encodingMatrix(n, k);
  const shares: Share[] = [];
  for (let i = 0; i < n; i++) {
    const out = Buffer.alloc(shardLen);
    for (let b = 0; b < shardLen; b++) {
      let acc = 0;
      for (let j = 0; j < k; j++) acc ^= gmul(G[i][j], dataShards[j][b]);
      out[b] = acc;
    }
    shares.push({ index: i, bytes: out });
  }
  return { shares, originalLength: data.length };
}

/** Recover the original data from any k shares. Throws if fewer than k. */
export function decode(available: Share[], n: number, k: number, originalLength: number): Buffer {
  if (available.length < k) throw new Error(`need ${k} shares, have ${available.length}`);
  const use = available.slice(0, k);
  const G = encodingMatrix(n, k);
  const sub = use.map((s) => G[s.index]);
  const inv = invert(sub);
  const shardLen = use[0].bytes.length;
  const recovered = Buffer.alloc(shardLen * k);
  for (let j = 0; j < k; j++) {
    for (let b = 0; b < shardLen; b++) {
      let acc = 0;
      for (let i = 0; i < k; i++) acc ^= gmul(inv[j][i], use[i].bytes[b]);
      recovered[j * shardLen + b] = acc;
    }
  }
  return recovered.subarray(0, originalLength);
}

/** Probability a collection is LOST under k-of-n coding when each share is lost
 * independently with probability p: P(> n-k shares lost). */
export function codedLossProbability(p: number, n: number, k: number): number {
  const tolerable = n - k;
  let lost = 0;
  for (let lostCount = tolerable + 1; lostCount <= n; lostCount++) {
    lost += binom(n, lostCount) * Math.pow(p, lostCount) * Math.pow(1 - p, n - lostCount);
  }
  return lost;
}

function binom(n: number, r: number): number {
  let result = 1;
  for (let i = 0; i < r; i++) result = (result * (n - i)) / (i + 1);
  return result;
}
