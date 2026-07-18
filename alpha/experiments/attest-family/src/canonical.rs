//! Canonical encoding — the existing §4.6 dag-cbor path, reused.
//!
//! All payload and envelope bytes go through `serde_ipld_dagcbor` over sorted
//! `Ipld::Map`s (the proven public-roundtrip path). Canonicalization is NOT
//! re-implemented here: this module only *builds* Ipld values and hands them to
//! the dag-cbor codec, exactly as `public-roundtrip/src/moderation.rs` and
//! `repo_verify.rs` do.
//!
//! One local discipline keeps the bytes stable under both dag-cbor key-order
//! conventions (pure lexicographic vs length-then-lexicographic): every map in
//! this crate uses **single-character keys**, so the two orderings coincide and
//! a `BTreeMap`'s iteration order is canonical either way.
//!
//! Determinism claim under test: T-AT0.1 — every attestation kind round-trips
//! `encode → decode → encode` byte-identically.

use std::collections::BTreeMap;

use ipld_core::ipld::Ipld;

use crate::types::*;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CanonicalError(pub String);

impl std::fmt::Display for CanonicalError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "canonical decode: {}", self.0)
    }
}
impl std::error::Error for CanonicalError {}

type R<T> = Result<T, CanonicalError>;

fn err<T>(msg: &str) -> R<T> {
    Err(CanonicalError(msg.to_string()))
}

// ---------------------------------------------------------------------------
// Ipld build helpers
// ---------------------------------------------------------------------------

fn map(pairs: Vec<(&str, Ipld)>) -> Ipld {
    let mut m = BTreeMap::new();
    for (k, v) in pairs {
        m.insert(k.to_string(), v);
    }
    Ipld::Map(m)
}

fn bytes(b: &[u8]) -> Ipld {
    Ipld::Bytes(b.to_vec())
}

fn int(v: u64) -> Ipld {
    Ipld::Integer(v as i128)
}

fn s(v: &str) -> Ipld {
    Ipld::String(v.to_string())
}

fn date(d: &DateClaim) -> Ipld {
    map(vec![
        ("d", int(d.day as u64)),
        ("m", int(d.month as u64)),
        ("y", int(d.year as u64)),
    ])
}

fn subject(sr: &SubjectRef) -> Ipld {
    match sr {
        SubjectRef::Persona(p) => map(vec![("i", bytes(&p.0)), ("k", s("persona"))]),
        SubjectRef::Thing(t) => map(vec![("i", bytes(&t.0)), ("k", s("thing"))]),
    }
}

fn oid_list(ids: &[ObjectId]) -> Ipld {
    Ipld::List(ids.iter().map(|o| bytes(&o.0)).collect())
}

// ---------------------------------------------------------------------------
// Encoding
// ---------------------------------------------------------------------------

/// Canonical bytes of an [`EdgeCore`] — the input to `core_hash`.
pub fn encode_edge_core(core: &EdgeCore) -> Vec<u8> {
    let v = map(vec![
        ("a", bytes(&core.persona_a.0)),
        ("b", bytes(&core.persona_b.0)),
        ("c", oid_list(&core.ceremony)),
        ("m", s(core.consent.as_str())),
        ("n", bytes(&core.edge_nonce)),
    ]);
    serde_ipld_dagcbor::to_vec(&v).expect("dag-cbor encode of pure map cannot fail")
}

fn payload_ipld(p: &Payload) -> Ipld {
    match p {
        Payload::EdgeHalf(h) => map(vec![
            ("c", edge_core_ipld(&h.core)),
            ("k", s("edge_half")),
            ("l", s(&h.label)),
        ]),
        Payload::EdgeDissolve(d) => map(vec![
            ("h", bytes(&d.core_hash)),
            ("k", s("edge_dissolve")),
            ("s", oid_list(&d.supersedes)),
        ]),
        Payload::CeremonyFact(c) => map(vec![
            ("d", date(&c.sighted_on)),
            ("k", s("ceremony_fact")),
            (
                "p",
                Ipld::List(vec![bytes(&c.participants[0].0), bytes(&c.participants[1].0)]),
            ),
            ("s", bytes(&c.session)),
        ]),
        Payload::TransactionFact(t) => map(vec![
            ("d", date(&t.occurred_on)),
            ("k", s("transaction_fact")),
            ("p", bytes(&t.payer.0)),
            ("y", subject(&t.payee)),
        ]),
        Payload::ThingDecl(t) => map(vec![
            ("c", bytes(&t.controller.0)),
            ("k", s("thing_decl")),
            ("t", bytes(&t.thing.0)),
            ("y", s(t.kind.as_str())),
        ]),
        Payload::Vouch(v) => {
            let mut pairs = vec![
                ("d", date(&v.made_on)),
                ("e", bytes(&v.base_edge)),
                ("k", s("vouch")),
                ("o", s(&v.scope.0)),
                ("s", bytes(&v.subject.0)),
                ("t", s(&v.statement)),
            ];
            if let Some(u) = &v.supersedes {
                pairs.push(("u", bytes(&u.0)));
            }
            map(pairs)
        }
        Payload::VouchWithdraw(w) => map(vec![
            ("k", s("vouch_withdraw")),
            ("u", bytes(&w.supersedes.0)),
        ]),
        Payload::Review(r) => {
            let mut pairs = vec![
                ("d", date(&r.made_on)),
                ("k", s("review")),
                ("m", s(r.consent.as_str())),
                ("o", s(&r.scope.0)),
                ("s", subject(&r.subject)),
                ("t", s(&r.statement)),
            ];
            if let Some(u) = &r.supersedes {
                pairs.push(("u", bytes(&u.0)));
            }
            map(pairs)
        }
        Payload::Reply(r) => map(vec![
            ("d", date(&r.made_on)),
            ("k", s("reply")),
            ("r", bytes(&r.review.0)),
            ("t", s(&r.statement)),
        ]),
        Payload::Predicate(p) => {
            // T-AT6.2: process provenance is INSIDE the predicate payload —
            // there is no encoding of the predicate without it, and the issuer
            // is the (signed) envelope author. No bare "over_18: true" exists.
            let mut pairs = vec![
                (
                    "c",
                    map(vec![
                        ("d", date(&p.process.performed_on)),
                        ("m", s(p.process.method.as_str())),
                        ("r", s(p.process.role.as_str())),
                    ]),
                ),
                ("k", s("predicate")),
                ("p", s(p.predicate.as_str())),
                ("s", bytes(&p.subject.0)),
            ];
            if let Some(u) = &p.supersedes {
                pairs.push(("u", bytes(&u.0)));
            }
            map(pairs)
        }
        Payload::ResolvabilityPolicy(rp) => {
            let rule = match &rp.rule {
                PolicyRule::AllowAll => map(vec![("k", s("allow_all"))]),
                PolicyRule::AllowOnly(list) => map(vec![
                    ("k", s("allow_only")),
                    (
                        "v",
                        Ipld::List(list.iter().map(|p| bytes(&p.0)).collect()),
                    ),
                ]),
            };
            let mut pairs = vec![
                ("k", s("resolvability_policy")),
                ("p", bytes(&rp.persona.0)),
                ("r", rule),
            ];
            if let Some(u) = &rp.supersedes {
                pairs.push(("u", bytes(&u.0)));
            }
            map(pairs)
        }
    }
}

fn edge_core_ipld(core: &EdgeCore) -> Ipld {
    map(vec![
        ("a", bytes(&core.persona_a.0)),
        ("b", bytes(&core.persona_b.0)),
        ("c", oid_list(&core.ceremony)),
        ("m", s(core.consent.as_str())),
        ("n", bytes(&core.edge_nonce)),
    ])
}

/// Canonical dag-cbor bytes of an envelope; `with_sig` selects the stored/wire
/// form (id-forming) vs the signing form.
pub fn encode_envelope(env: &Envelope, with_sig: bool) -> Vec<u8> {
    let mut pairs = vec![
        ("a", bytes(&env.author.0)),
        ("l", int(env.lamport)),
        ("n", oid_list(&env.antecedents)),
        ("p", payload_ipld(&env.payload)),
        ("v", int(env.version as u64)),
    ];
    if with_sig {
        pairs.push(("s", bytes(&env.signature)));
    }
    serde_ipld_dagcbor::to_vec(&map(pairs)).expect("dag-cbor encode of pure map cannot fail")
}

// ---------------------------------------------------------------------------
// Decoding
// ---------------------------------------------------------------------------

fn as_map(v: &Ipld) -> R<&BTreeMap<String, Ipld>> {
    match v {
        Ipld::Map(m) => Ok(m),
        _ => err("expected map"),
    }
}

fn get<'a>(m: &'a BTreeMap<String, Ipld>, k: &str) -> R<&'a Ipld> {
    m.get(k).ok_or_else(|| CanonicalError(format!("missing key {k}")))
}

fn get_bytes(m: &BTreeMap<String, Ipld>, k: &str) -> R<Vec<u8>> {
    match get(m, k)? {
        Ipld::Bytes(b) => Ok(b.clone()),
        _ => err("expected bytes"),
    }
}

fn get_b32(m: &BTreeMap<String, Ipld>, k: &str) -> R<[u8; 32]> {
    let b = get_bytes(m, k)?;
    b.try_into().map_err(|_| CanonicalError(format!("key {k}: expected 32 bytes")))
}

fn get_b16(m: &BTreeMap<String, Ipld>, k: &str) -> R<[u8; 16]> {
    let b = get_bytes(m, k)?;
    b.try_into().map_err(|_| CanonicalError(format!("key {k}: expected 16 bytes")))
}

fn get_int(m: &BTreeMap<String, Ipld>, k: &str) -> R<u64> {
    match get(m, k)? {
        Ipld::Integer(i) if *i >= 0 && *i <= u64::MAX as i128 => Ok(*i as u64),
        _ => err("expected non-negative integer"),
    }
}

fn get_str<'a>(m: &'a BTreeMap<String, Ipld>, k: &str) -> R<&'a str> {
    match get(m, k)? {
        Ipld::String(v) => Ok(v),
        _ => err("expected string"),
    }
}

fn get_list<'a>(m: &'a BTreeMap<String, Ipld>, k: &str) -> R<&'a Vec<Ipld>> {
    match get(m, k)? {
        Ipld::List(l) => Ok(l),
        _ => err("expected list"),
    }
}

fn decode_oid_list(m: &BTreeMap<String, Ipld>, k: &str) -> R<Vec<ObjectId>> {
    let mut out = Vec::new();
    for v in get_list(m, k)? {
        match v {
            Ipld::Bytes(b) => {
                let arr: [u8; 32] =
                    b.clone().try_into().map_err(|_| CanonicalError("oid: 32 bytes".into()))?;
                out.push(ObjectId(arr));
            }
            _ => return err("oid list: expected bytes"),
        }
    }
    Ok(out)
}

fn decode_date(v: &Ipld) -> R<DateClaim> {
    let m = as_map(v)?;
    Ok(DateClaim {
        year: get_int(m, "y")? as u16,
        month: get_int(m, "m")? as u8,
        day: get_int(m, "d")? as u8,
    })
}

fn decode_subject(v: &Ipld) -> R<SubjectRef> {
    let m = as_map(v)?;
    let id = get_b32(m, "i")?;
    match get_str(m, "k")? {
        "persona" => Ok(SubjectRef::Persona(PersonaId(id))),
        "thing" => Ok(SubjectRef::Thing(ThingId(id))),
        other => Err(CanonicalError(format!("unknown subject kind {other}"))),
    }
}

fn decode_edge_core(v: &Ipld) -> R<EdgeCore> {
    let m = as_map(v)?;
    let consent = ConsentMode::from_str(get_str(m, "m")?)
        .ok_or_else(|| CanonicalError("unknown consent mode".into()))?;
    Ok(EdgeCore {
        persona_a: PersonaId(get_b32(m, "a")?),
        persona_b: PersonaId(get_b32(m, "b")?),
        edge_nonce: get_b16(m, "n")?,
        consent,
        ceremony: decode_oid_list(m, "c")?,
    })
}

fn opt_supersedes(m: &BTreeMap<String, Ipld>) -> R<Option<ObjectId>> {
    match m.get("u") {
        None => Ok(None),
        Some(Ipld::Bytes(b)) => {
            let arr: [u8; 32] =
                b.clone().try_into().map_err(|_| CanonicalError("supersedes: 32 bytes".into()))?;
            Ok(Some(ObjectId(arr)))
        }
        Some(_) => err("supersedes: expected bytes"),
    }
}

fn decode_payload(v: &Ipld) -> R<Payload> {
    let m = as_map(v)?;
    match get_str(m, "k")? {
        "edge_half" => Ok(Payload::EdgeHalf(EdgeHalf {
            core: decode_edge_core(get(m, "c")?)?,
            label: get_str(m, "l")?.to_string(),
        })),
        "edge_dissolve" => Ok(Payload::EdgeDissolve(EdgeDissolve {
            core_hash: get_b32(m, "h")?,
            supersedes: decode_oid_list(m, "s")?,
        })),
        "ceremony_fact" => {
            let plist = get_list(m, "p")?;
            if plist.len() != 2 {
                return err("ceremony_fact: exactly two participants");
            }
            let mut parts = [PersonaId([0; 32]); 2];
            for (i, p) in plist.iter().enumerate() {
                match p {
                    Ipld::Bytes(b) => {
                        parts[i] = PersonaId(
                            b.clone()
                                .try_into()
                                .map_err(|_| CanonicalError("participant: 32 bytes".into()))?,
                        )
                    }
                    _ => return err("participant: expected bytes"),
                }
            }
            Ok(Payload::CeremonyFact(CeremonyFact {
                session: get_b16(m, "s")?,
                participants: parts,
                sighted_on: decode_date(get(m, "d")?)?,
            }))
        }
        "transaction_fact" => Ok(Payload::TransactionFact(TransactionFact {
            payer: PersonaId(get_b32(m, "p")?),
            payee: decode_subject(get(m, "y")?)?,
            occurred_on: decode_date(get(m, "d")?)?,
        })),
        "thing_decl" => Ok(Payload::ThingDecl(ThingDecl {
            thing: ThingId(get_b32(m, "t")?),
            kind: ThingKind::from_str(get_str(m, "y")?)
                .ok_or_else(|| CanonicalError("unknown thing kind".into()))?,
            controller: PersonaId(get_b32(m, "c")?),
        })),
        "vouch" => Ok(Payload::Vouch(Vouch {
            subject: PersonaId(get_b32(m, "s")?),
            scope: Scope(get_str(m, "o")?.to_string()),
            statement: get_str(m, "t")?.to_string(),
            base_edge: get_b32(m, "e")?,
            made_on: decode_date(get(m, "d")?)?,
            supersedes: opt_supersedes(m)?,
        })),
        "vouch_withdraw" => Ok(Payload::VouchWithdraw(VouchWithdraw {
            supersedes: opt_supersedes(m)?
                .ok_or_else(|| CanonicalError("vouch_withdraw: supersedes required".into()))?,
        })),
        "review" => Ok(Payload::Review(Review {
            subject: decode_subject(get(m, "s")?)?,
            scope: Scope(get_str(m, "o")?.to_string()),
            statement: get_str(m, "t")?.to_string(),
            consent: ConsentMode::from_str(get_str(m, "m")?)
                .ok_or_else(|| CanonicalError("unknown consent mode".into()))?,
            made_on: decode_date(get(m, "d")?)?,
            supersedes: opt_supersedes(m)?,
        })),
        "reply" => Ok(Payload::Reply(Reply {
            review: ObjectId(get_b32(m, "r")?),
            statement: get_str(m, "t")?.to_string(),
            made_on: decode_date(get(m, "d")?)?,
        })),
        "predicate" => {
            let proc_m = as_map(get(m, "c")?)?;
            Ok(Payload::Predicate(Predicate {
                predicate: PredicateKind::from_str(get_str(m, "p")?)
                    .ok_or_else(|| CanonicalError("unknown predicate".into()))?,
                subject: PersonaId(get_b32(m, "s")?),
                process: ProcessProvenance {
                    method: MethodKind::from_str(get_str(proc_m, "m")?)
                        .ok_or_else(|| CanonicalError("unknown method".into()))?,
                    performed_on: decode_date(get(proc_m, "d")?)?,
                    role: IssuerRole::from_str(get_str(proc_m, "r")?)
                        .ok_or_else(|| CanonicalError("unknown role".into()))?,
                },
                supersedes: opt_supersedes(m)?,
            }))
        }
        "resolvability_policy" => {
            let rule_m = as_map(get(m, "r")?)?;
            let rule = match get_str(rule_m, "k")? {
                "allow_all" => PolicyRule::AllowAll,
                "allow_only" => {
                    let mut list = Vec::new();
                    for v in get_list(rule_m, "v")? {
                        match v {
                            Ipld::Bytes(b) => list.push(PersonaId(
                                b.clone()
                                    .try_into()
                                    .map_err(|_| CanonicalError("viewer: 32 bytes".into()))?,
                            )),
                            _ => return err("viewer: expected bytes"),
                        }
                    }
                    PolicyRule::AllowOnly(list)
                }
                other => return Err(CanonicalError(format!("unknown policy rule {other}"))),
            };
            Ok(Payload::ResolvabilityPolicy(ResolvabilityPolicy {
                persona: PersonaId(get_b32(m, "p")?),
                rule,
                supersedes: opt_supersedes(m)?,
            }))
        }
        other => Err(CanonicalError(format!("unknown payload kind {other}"))),
    }
}

/// Decode the stored/wire form (`canonical_bytes_with_sig`) back to a typed
/// envelope. T-AT0.1 pins `encode(decode(bytes)) == bytes` for every kind.
pub fn decode_envelope(raw: &[u8]) -> R<Envelope> {
    let v: Ipld = serde_ipld_dagcbor::from_slice(raw)
        .map_err(|e| CanonicalError(format!("dag-cbor: {e}")))?;
    let m = as_map(&v)?;
    Ok(Envelope {
        version: get_int(m, "v")? as u8,
        author: PersonaId(get_b32(m, "a")?),
        lamport: get_int(m, "l")?,
        antecedents: decode_oid_list(m, "n")?,
        payload: decode_payload(get(m, "p")?)?,
        signature: get_bytes(m, "s")?,
    })
}
