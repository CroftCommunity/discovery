//! Delivery roles (RUN-16 §A.7): reconciliation, interval backfill, and the
//! thin process transport.
//!
//! Three delivery roles co-host the way one VPS would run them:
//! - the **web-native DS** — HTTP/socket serve of backplane scopes;
//! - the **swarm-peer** — envelope exchange over the transport trait (local
//!   sockets here; the iroh overlay is a not-required upgrade);
//! - the **history-convergence node** — reconciles envelope sets from both by
//!   `H(envelope)` (a hash-set diff standing in for the RUN-12 RBSR
//!   construction, `SPEC-DELTA[run17-rbsr | declared-stand-in]`) and serves
//!   interval backfill.
//!
//! This module holds the reconciliation + backfill LOGIC (component-tested) and
//! a minimal newline-delimited TCP protocol the `ds`/`swarm_peer`/`convergence`
//! binaries share (process-tested).

use std::collections::BTreeMap;
use std::io::{BufRead, BufReader, Read, Write};
use std::net::{TcpListener, TcpStream};

use sha2::{Digest, Sha256};

use crate::envelope::Envelope;

/// A plaintext envelope store keyed by identity — the DS and swarm-peer stores.
/// Insertion dedups by `H(envelope)`; positions are carried for interval
/// backfill.
#[derive(Debug, Default, Clone)]
pub struct EnvelopeStore {
    by_id: BTreeMap<String, (Envelope, u64)>,
}

impl EnvelopeStore {
    /// A new, empty store.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Insert an envelope at causal position `pos`. Idempotent by identity.
    pub fn insert(&mut self, env: Envelope, pos: u64) {
        self.by_id.insert(env.identity_hex(), (env, pos));
    }

    /// All stored (envelope, position) pairs.
    #[must_use]
    pub fn entries(&self) -> Vec<(Envelope, u64)> {
        self.by_id.values().cloned().collect()
    }

    /// Number of distinct envelopes.
    #[must_use]
    pub fn len(&self) -> usize {
        self.by_id.len()
    }

    /// Whether the store is empty.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.by_id.is_empty()
    }
}

/// Reconcile several stores into one deduped set (by `H(envelope)`), sorted by
/// causal position then identity. The dedup makes the SAME envelope arriving via
/// different transports collapse to one — the RBSR stand-in.
#[must_use]
pub fn converge(stores: &[&EnvelopeStore]) -> Vec<Envelope> {
    let mut merged: BTreeMap<String, (Envelope, u64)> = BTreeMap::new();
    for s in stores {
        for (env, pos) in s.entries() {
            merged.entry(env.identity_hex()).or_insert((env, pos));
        }
    }
    let mut out: Vec<(Envelope, u64)> = merged.into_values().collect();
    out.sort_by(|a, b| {
        a.1.cmp(&b.1)
            .then_with(|| a.0.identity_hex().cmp(&b.0.identity_hex()))
    });
    out.into_iter().map(|(e, _)| e).collect()
}

/// Why an interval-backfill offer was refused (offering-side).
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum OfferReject {
    /// The requester proved no membership interval — refused outright.
    NotAMember,
    /// The requested window is not fully covered by a proven interval (reaches
    /// before a join or past a revocation cut). Nothing outside is ever offered.
    NotProven,
}

/// Is `[j, r)` fully inside one proven membership interval? An open interval
/// `(start, None)` covers everything from `start`.
fn window_is_proven(intervals: &[(u64, Option<u64>)], j: u64, r: u64) -> bool {
    intervals
        .iter()
        .any(|(start, end)| j >= *start && end.is_none_or(|e| r <= e))
}

/// Offer exactly the envelopes with causal positions in `[j, r)`, PROVIDED the
/// window is inside a proven membership interval. The refusal is offering-side:
/// the node computes what it will offer and never reaches outside the window, so
/// it holds nothing it must later "unsee".
///
/// # Errors
/// [`OfferReject::NotAMember`] for an empty interval set; [`OfferReject::NotProven`]
/// when the window is not covered.
pub fn offer_interval(
    store: &EnvelopeStore,
    member_intervals: &[(u64, Option<u64>)],
    (j, r): (u64, u64),
) -> Result<Vec<(Envelope, u64)>, OfferReject> {
    if member_intervals.is_empty() {
        return Err(OfferReject::NotAMember);
    }
    if !window_is_proven(member_intervals, j, r) {
        return Err(OfferReject::NotProven);
    }
    let mut out: Vec<(Envelope, u64)> = store
        .entries()
        .into_iter()
        .filter(|(_, p)| *p >= j && *p < r)
        .collect();
    out.sort_by_key(|(_, p)| *p);
    Ok(out)
}

// ───────────────────────── sealed scope (ciphertext-only store) ─────────────

/// A ciphertext-only store: the convergence node holds sealed bytes and never
/// the key. Positions are metadata, so interval offering works blind.
#[derive(Debug, Default, Clone)]
pub struct SealedStore {
    items: Vec<(Vec<u8>, u64)>,
}

impl SealedStore {
    /// A new, empty sealed store.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }
    /// Insert ciphertext at causal position `pos`.
    pub fn insert(&mut self, ciphertext: Vec<u8>, pos: u64) {
        self.items.push((ciphertext, pos));
    }
}

/// Offer sealed ciphertexts in `[j, r)` under the same interval rule. The node
/// gates OFFERING by position without reading content.
///
/// # Errors
/// As [`offer_interval`].
pub fn offer_sealed_interval(
    store: &SealedStore,
    member_intervals: &[(u64, Option<u64>)],
    (j, r): (u64, u64),
) -> Result<Vec<(Vec<u8>, u64)>, OfferReject> {
    if member_intervals.is_empty() {
        return Err(OfferReject::NotAMember);
    }
    if !window_is_proven(member_intervals, j, r) {
        return Err(OfferReject::NotProven);
    }
    let mut out: Vec<(Vec<u8>, u64)> = store
        .items
        .iter()
        .filter(|(_, p)| *p >= j && *p < r)
        .cloned()
        .collect();
    out.sort_by_key(|(_, p)| *p);
    Ok(out)
}

/// A harness seal: a SHA-256 keystream XOR. This is a loopback seal for the
/// store-blindness property (the store holds bytes it cannot read); the real
/// seal is the croft-group MLS ciphertext (P6). Symmetric: `open == seal`.
#[must_use]
pub fn seal(key: &[u8], plaintext: &[u8]) -> Vec<u8> {
    let mut out = Vec::with_capacity(plaintext.len());
    for (i, chunk) in plaintext.chunks(32).enumerate() {
        let mut h = Sha256::new();
        h.update(key);
        h.update((i as u64).to_be_bytes());
        let ks = h.finalize();
        for (b, k) in chunk.iter().zip(ks.iter()) {
            out.push(b ^ k);
        }
    }
    out
}

/// Open a harness-sealed ciphertext (the inverse of [`seal`]).
#[must_use]
pub fn open(key: &[u8], ciphertext: &[u8]) -> Vec<u8> {
    seal(key, ciphertext)
}

// ───────────────────────── the thin process transport ──────────────────────

/// Bind a localhost listener on an ephemeral port; return it and the port.
///
/// # Errors
/// Propagates the bind error.
pub fn bind_ephemeral() -> std::io::Result<(TcpListener, u16)> {
    let listener = TcpListener::bind(("127.0.0.1", 0))?;
    let port = listener.local_addr()?.port();
    Ok((listener, port))
}

/// Read the single command line from a connection.
fn read_line(stream: &TcpStream) -> Option<String> {
    let mut reader = BufReader::new(stream.try_clone().ok()?);
    let mut line = String::new();
    reader.read_line(&mut line).ok()?;
    Some(line.trim().to_string())
}

/// Serve a store role (DS or swarm-peer). Commands (newline-delimited):
/// `PING`→`PONG`; `SEED <byte>`→mint a deterministic envelope, store it, reply
/// `ID <hex>`; `DUMP`→`<hex>;<hex>;…` (canonical bytes) or `EMPTY`. Loops until
/// killed.
pub fn serve_store(listener: &TcpListener) {
    let mut store = EnvelopeStore::new();
    let mut next_pos = 0u64;
    for conn in listener.incoming() {
        let Ok(stream) = conn else { continue };
        let Some(line) = read_line(&stream) else {
            continue;
        };
        let reply = handle_store_cmd(&line, &mut store, &mut next_pos);
        let _ = (&stream).write_all(reply.as_bytes());
    }
}

fn handle_store_cmd(line: &str, store: &mut EnvelopeStore, next_pos: &mut u64) -> String {
    let mut parts = line.splitn(2, ' ');
    match parts.next() {
        Some("PING") => "PONG".to_string(),
        Some("SEED") => {
            let seed: u8 = parts
                .next()
                .and_then(|s| s.trim().parse().ok())
                .unwrap_or(0);
            let signer = crate::identity::Signer::from_seed([seed; 32]);
            let env = crate::records::seal(
                &signer,
                vec![],
                &crate::records::Record::Message {
                    scope: "scope:backplane".to_string(),
                    text: format!("seeded-{seed}"),
                },
            );
            let id = env.identity_hex();
            store.insert(env, *next_pos);
            *next_pos += 1;
            format!("ID {id}")
        }
        Some("DUMP") => {
            let entries = store.entries();
            if entries.is_empty() {
                "EMPTY".to_string()
            } else {
                entries
                    .iter()
                    .map(|(e, _)| {
                        hex::encode(crate::canonical::to_canonical(e).unwrap_or_default())
                    })
                    .collect::<Vec<_>>()
                    .join(";")
            }
        }
        _ => "ERR".to_string(),
    }
}

/// Serve the convergence role. `PING`→`PONG`; `CONVERGE <ds_addr> <swarm_addr>`
/// pulls `DUMP` from both, reconciles by `H(envelope)`, replies
/// `COUNT <n> IDS <id,…>`. Loops until killed.
pub fn serve_convergence(listener: &TcpListener) {
    for conn in listener.incoming() {
        let Ok(stream) = conn else { continue };
        let Some(line) = read_line(&stream) else {
            continue;
        };
        let reply = handle_convergence_cmd(&line);
        let _ = (&stream).write_all(reply.as_bytes());
    }
}

fn handle_convergence_cmd(line: &str) -> String {
    let mut parts = line.split_whitespace();
    match parts.next() {
        Some("PING") => "PONG".to_string(),
        Some("CONVERGE") => {
            let addrs: Vec<&str> = parts.collect();
            let mut store = EnvelopeStore::new();
            let mut pos = 0u64;
            for addr in addrs {
                if let Some(dump) = client_cmd(addr, "DUMP") {
                    if dump != "EMPTY" {
                        for hexbytes in dump.split(';') {
                            if let Ok(bytes) = hex::decode(hexbytes.trim()) {
                                if let Ok(env) =
                                    ciborium::from_reader::<Envelope, _>(bytes.as_slice())
                                {
                                    store.insert(env, pos);
                                    pos += 1;
                                }
                            }
                        }
                    }
                }
            }
            let converged = converge(&[&store]);
            let ids: Vec<String> = converged.iter().map(Envelope::identity_hex).collect();
            format!("COUNT {} IDS {}", ids.len(), ids.join(","))
        }
        _ => "ERR".to_string(),
    }
}

/// Send one command to `addr` (host:port) and read the whole single-line reply.
#[must_use]
pub fn client_cmd(addr: &str, line: &str) -> Option<String> {
    let mut stream = TcpStream::connect(addr).ok()?;
    stream.write_all(line.as_bytes()).ok()?;
    stream.write_all(b"\n").ok()?;
    stream.flush().ok()?;
    let mut buf = String::new();
    stream.read_to_string(&mut buf).ok()?;
    Some(buf.trim().to_string())
}
