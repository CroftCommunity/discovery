// A minimal systematic-free Reed-Solomon erasure code over GF(2^8), for E10.
//
// Split an item into k data shares; expand to n shares such that ANY k of the n
// recover the item exactly. This is the road from spot checks (probabilistic
// detection) to retrievability guarantees (deterministic recovery below the loss
// threshold) — the door E10 opens without walking through.
//
// Construction: a Cauchy generator matrix G (n x k). Every square submatrix of a
// Cauchy matrix is invertible, so any k received shares give an invertible k x k
// system that recovers the data. No external library — GF(256) arithmetic and
// Gaussian elimination are implemented here (zero dependencies).
//
// SEAM: a production deployment would use a hardened, SIMD-accelerated RS library
// (e.g. a maintained Reed-Solomon crate/package) rather than this teaching
// implementation.

const PRIM = 0x11d; // primitive polynomial x^8 + x^4 + x^3 + x^2 + 1
const EXP = new Uint8Array(512);
const LOG = new Uint8Array(256);

(function buildTables() {
  let x = 1;
  for (let i = 0; i < 255; i++) {
    EXP[i] = x;
    LOG[x] = i;
    x <<= 1;
    if (x & 0x100) x ^= PRIM;
  }
  for (let i = 255; i < 512; i++) EXP[i] = EXP[i - 255];
  // Sanity: 2 must be a generator of the multiplicative group (full 255-cycle).
  const seen = new Set<number>();
  for (let i = 0; i < 255; i++) seen.add(EXP[i]);
  if (seen.size !== 255) throw new Error("GF(256) tables invalid: 2 is not primitive for this poly");
})();

function gfMul(a: number, b: number): number {
  if (a === 0 || b === 0) return 0;
  return EXP[LOG[a] + LOG[b]];
}

function gfDiv(a: number, b: number): number {
  if (b === 0) throw new Error("division by zero in GF(256)");
  if (a === 0) return 0;
  return EXP[LOG[a] - LOG[b] + 255];
}

/** Cauchy generator matrix: G[i][j] = 1/(x_i XOR y_j), rows disjoint from cols. */
function cauchyMatrix(n: number, k: number): number[][] {
  if (n + k > 256) throw new Error("n + k must be <= 256");
  const xs = Array.from({ length: n }, (_, i) => i); // 0..n-1
  const ys = Array.from({ length: k }, (_, j) => n + j); // n..n+k-1 (disjoint)
  const G: number[][] = [];
  for (let i = 0; i < n; i++) {
    const row: number[] = [];
    for (let j = 0; j < k; j++) {
      row.push(gfDiv(1, xs[i] ^ ys[j]));
    }
    G.push(row);
  }
  return G;
}

/** Invert a square matrix over GF(256) via Gauss–Jordan. Throws if singular. */
function invertMatrix(m: number[][]): number[][] {
  const n = m.length;
  const a = m.map((row, i) => [
    ...row,
    ...Array.from({ length: n }, (_, j) => (i === j ? 1 : 0)),
  ]);
  for (let col = 0; col < n; col++) {
    let pivot = col;
    while (pivot < n && a[pivot][col] === 0) pivot++;
    if (pivot === n) throw new Error("singular matrix (should not happen for Cauchy submatrix)");
    [a[col], a[pivot]] = [a[pivot], a[col]];
    const inv = gfDiv(1, a[col][col]);
    for (let j = 0; j < 2 * n; j++) a[col][j] = gfMul(a[col][j], inv);
    for (let r = 0; r < n; r++) {
      if (r === col) continue;
      const factor = a[r][col];
      if (factor === 0) continue;
      for (let j = 0; j < 2 * n; j++) a[r][j] ^= gfMul(factor, a[col][j]);
    }
  }
  return a.map((row) => row.slice(n));
}

export interface Share {
  index: number; // row index in G, 0..n-1
  bytes: Buffer;
}

export interface Encoded {
  shares: Share[];
  n: number;
  k: number;
  length: number; // original item byte length
}

/** Encode `data` into n shares (any k recover). */
export function rsEncode(data: Buffer, k: number, n: number): Encoded {
  if (k < 1 || n < k) throw new Error("require 1 <= k <= n");
  const chunkLen = Math.ceil(data.length / k);
  // k data chunks, zero-padded to equal length.
  const chunks: Buffer[] = [];
  for (let j = 0; j < k; j++) {
    const buf = Buffer.alloc(chunkLen);
    data.copy(buf, 0, j * chunkLen, Math.min((j + 1) * chunkLen, data.length));
    chunks.push(buf);
  }
  const G = cauchyMatrix(n, k);
  const shares: Share[] = [];
  for (let i = 0; i < n; i++) {
    const out = Buffer.alloc(chunkLen);
    for (let p = 0; p < chunkLen; p++) {
      let acc = 0;
      for (let j = 0; j < k; j++) acc ^= gfMul(G[i][j], chunks[j][p]);
      out[p] = acc;
    }
    shares.push({ index: i, bytes: out });
  }
  return { shares, n, k, length: data.length };
}

/** Recover the original item from any k of the n shares. */
export function rsDecode(received: Share[], k: number, n: number, length: number): Buffer {
  if (received.length < k) {
    throw new Error(`need at least k=${k} shares, got ${received.length}`);
  }
  const use = received.slice(0, k);
  const G = cauchyMatrix(n, k);
  const sub = use.map((s) => G[s.index]);
  const inv = invertMatrix(sub);
  const chunkLen = use[0].bytes.length;
  const chunks: Buffer[] = Array.from({ length: k }, () => Buffer.alloc(chunkLen));
  for (let p = 0; p < chunkLen; p++) {
    for (let j = 0; j < k; j++) {
      let acc = 0;
      for (let r = 0; r < k; r++) acc ^= gfMul(inv[j][r], use[r].bytes[p]);
      chunks[j][p] = acc;
    }
  }
  return Buffer.concat(chunks).subarray(0, length);
}
