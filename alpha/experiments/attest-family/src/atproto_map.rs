//! atproto_map — the RUN-ATTEST-03 Part C mapping spike. Pure, no network.
//!
//! Lossless round-trip between crate payloads and the DRAFT lexicon record
//! shapes in `lexicons/` (`ing.croft.attest.*` — non-normative mirrors;
//! `src/canonical.rs` stays the source of truth). The mapping is PAYLOAD
//! level: the crate envelope's `{lamport, antecedents, signature}` realize at
//! the ATProto tier as repo commits and strong refs, not as record fields —
//! see [`fields_without_lexicon_home`], the mechanical statement of the
//! two-tier boundary (T-A3.8).
//!
//! Declared stand-ins (this is a spike, not a wire format):
//! - a persona id renders as `did:croft-fixture:<64-hex>` — fixture keypairs
//!   are RUN-ATTEST-01 §3's declared stand-in for real DIDs;
//! - an object id renders as a strongRef-shaped map whose `cid` is the
//!   64-hex of the 32-byte content address and whose `uri` is a fixture
//!   `at://` path — the same hash under a stand-in wrapper, not a real CID.
//!
//! Grade note: the only `Grade` consumer here is serialization-shaped
//! mapping, none — grades are fold outputs and never appear in payloads, so
//! they never appear in records either.

use std::collections::BTreeMap;

use ipld_core::ipld::Ipld;

use crate::issuer::EpochRecord;
use crate::types::*;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MapError(pub String);

impl std::fmt::Display for MapError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "atproto map: {}", self.0)
    }
}
impl std::error::Error for MapError {}

/// One deliberately-unmapped surface: what it is, which tier holds it, why.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct NoLexiconHome {
    pub surface: &'static str,
    pub tier: &'static str,
    pub why: &'static str,
}

/// The test-enforced list (T-A3.8) of surfaces that deliberately have NO
/// lexicon representation — the two-tier boundary made mechanical. Every
/// payload kind for which [`to_record`] returns `None` is named here.
pub fn fields_without_lexicon_home() -> &'static [NoLexiconHome] {
    &[
        NoLexiconHome {
            surface: "resolvability_policy (whole payload kind)",
            tier: "drystone",
            why: "repos are public today (matchup row 8); per-viewer disclosure is served \
                  by the authed AppView from Drystone-held policy state, and \
                  policy-restricted attestations never reach the public repo",
        },
        NoLexiconHome {
            surface: "ceremony_fact (whole payload kind, incl. the session id)",
            tier: "drystone/private",
            why: "ceremony session privates never publish; edge cores cite ceremony \
                  facts by content address only, so the public tier sees hashes, \
                  never sessions",
        },
        NoLexiconHome {
            surface: "issuer retained state (seam ledger, used salts, assertion heads)",
            tier: "issuer seam (drystone)",
            why: "T-PA6.1: the one named linkage point; not serializable by \
                  construction, so it cannot have a record shape",
        },
        NoLexiconHome {
            surface: "vouch_withdraw / edge_dissolve / credential_supersede (payload kinds)",
            tier: "atproto record ops + drystone lineage",
            why: "V2: withdrawal/claw-back realizes as the author deleting or \
                  same-rkey-replacing their own record (authoritative layer), while \
                  the Drystone fold keeps supersede lineage; these are operations, \
                  not record kinds",
        },
        NoLexiconHome {
            surface: "transaction_fact / thing_decl / vetting_fact (payload kinds)",
            tier: "deferred (stand-in facts)",
            why: "declared stand-ins with no real rail behind them yet; no lexicon is \
                  drafted in the B.2 six-kind scope — the credential's `vetting` \
                  strongRef gains a target when the vetting-fact lexicon is drafted \
                  (named residual)",
        },
        NoLexiconHome {
            surface: "predicate (RUN-ATTEST-01 payload kind)",
            tier: "frozen record",
            why: "the credential is the anchor-persona unit that graduated to a \
                  lexicon; the older predicate kind stays crate-internal frozen \
                  record (RUN-ATTEST-02 deviation 4)",
        },
        NoLexiconHome {
            surface: "envelope {lamport, antecedents, signature}",
            tier: "drystone fold inputs / atproto commit machinery",
            why: "repo commits sign records and strong refs cite them at the ATProto \
                  tier; the crate envelope's logical clock and antecedent list are \
                  Drystone fold inputs, not record fields",
        },
        NoLexiconHome {
            surface: "epoch_record.signature (detached issuer signature)",
            tier: "atproto commit signature",
            why: "the commitmentEpoch record is covered by the repo commit signature; \
                  the detached signature exists only in the crate's standalone form",
        },
    ]
}

// ---------------------------------------------------------------------------
// Stand-in leaf encodings
// ---------------------------------------------------------------------------

fn map(pairs: Vec<(&str, Ipld)>) -> Ipld {
    let mut m = BTreeMap::new();
    for (k, v) in pairs {
        m.insert(k.to_string(), v);
    }
    Ipld::Map(m)
}

fn hex(bytes: &[u8]) -> String {
    bytes.iter().map(|b| format!("{b:02x}")).collect()
}

fn unhex(s: &str) -> Result<Vec<u8>, MapError> {
    if !s.len().is_multiple_of(2) || !s.chars().all(|c| c.is_ascii_hexdigit()) {
        return Err(MapError(format!("bad hex: {s}")));
    }
    Ok((0..s.len())
        .step_by(2)
        .map(|i| u8::from_str_radix(&s[i..i + 2], 16).expect("checked hex"))
        .collect())
}

fn did_of(p: &PersonaId) -> Ipld {
    Ipld::String(format!("did:croft-fixture:{}", hex(&p.0)))
}

fn persona_from(v: &Ipld) -> Result<PersonaId, MapError> {
    let Ipld::String(s) = v else { return Err(MapError("did: expected string".into())) };
    let h = s
        .strip_prefix("did:croft-fixture:")
        .ok_or_else(|| MapError(format!("not a fixture did: {s}")))?;
    let b: [u8; 32] =
        unhex(h)?.try_into().map_err(|_| MapError("did: expected 32 bytes".into()))?;
    Ok(PersonaId(b))
}

/// strongRef stand-in: the 32-byte content address as hex `cid` + fixture uri.
fn strong_ref(id: &ObjectId) -> Ipld {
    map(vec![
        ("cid", Ipld::String(hex(&id.0))),
        ("uri", Ipld::String(format!("at://croft.fixture/{}", hex(&id.0)))),
    ])
}

fn object_from(v: &Ipld) -> Result<ObjectId, MapError> {
    let Ipld::Map(m) = v else { return Err(MapError("strongRef: expected map".into())) };
    let Some(Ipld::String(cid)) = m.get("cid") else {
        return Err(MapError("strongRef: missing cid".into()));
    };
    let b: [u8; 32] =
        unhex(cid)?.try_into().map_err(|_| MapError("strongRef: expected 32 bytes".into()))?;
    Ok(ObjectId(b))
}

fn date_of(d: &DateClaim) -> Ipld {
    map(vec![
        ("day", Ipld::Integer(d.day as i128)),
        ("month", Ipld::Integer(d.month as i128)),
        ("year", Ipld::Integer(d.year as i128)),
    ])
}

fn get<'a>(m: &'a BTreeMap<String, Ipld>, k: &str) -> Result<&'a Ipld, MapError> {
    m.get(k).ok_or_else(|| MapError(format!("missing key {k}")))
}

fn get_str<'a>(m: &'a BTreeMap<String, Ipld>, k: &str) -> Result<&'a str, MapError> {
    match get(m, k)? {
        Ipld::String(s) => Ok(s),
        _ => Err(MapError(format!("key {k}: expected string"))),
    }
}

fn get_int(m: &BTreeMap<String, Ipld>, k: &str) -> Result<u64, MapError> {
    match get(m, k)? {
        Ipld::Integer(i) if *i >= 0 && *i <= u64::MAX as i128 => Ok(*i as u64),
        _ => Err(MapError(format!("key {k}: expected non-negative integer"))),
    }
}

fn get_bytes(m: &BTreeMap<String, Ipld>, k: &str) -> Result<Vec<u8>, MapError> {
    match get(m, k)? {
        Ipld::Bytes(b) => Ok(b.clone()),
        _ => Err(MapError(format!("key {k}: expected bytes"))),
    }
}

fn date_from(v: &Ipld) -> Result<DateClaim, MapError> {
    let Ipld::Map(m) = v else { return Err(MapError("dateClaim: expected map".into())) };
    Ok(DateClaim {
        year: get_int(m, "year")? as u16,
        month: get_int(m, "month")? as u8,
        day: get_int(m, "day")? as u8,
    })
}

fn as_map(v: &Ipld) -> Result<&BTreeMap<String, Ipld>, MapError> {
    match v {
        Ipld::Map(m) => Ok(m),
        _ => Err(MapError("expected map".into())),
    }
}

fn subject_of(sr: &SubjectRef) -> Ipld {
    match sr {
        SubjectRef::Persona(p) => map(vec![("did", did_of(p))]),
        SubjectRef::Thing(t) => map(vec![("id", Ipld::Bytes(t.0.to_vec()))]),
    }
}

fn subject_from(v: &Ipld) -> Result<SubjectRef, MapError> {
    let m = as_map(v)?;
    if m.contains_key("did") {
        return Ok(SubjectRef::Persona(persona_from(get(m, "did")?)?));
    }
    let b: [u8; 32] = get_bytes(m, "id")?
        .try_into()
        .map_err(|_| MapError("thing id: expected 32 bytes".into()))?;
    Ok(SubjectRef::Thing(ThingId(b)))
}

// ---------------------------------------------------------------------------
// Payload ↔ record
// ---------------------------------------------------------------------------

/// The record shape (`$type`-tagged map mirroring the draft lexicon) for a
/// payload kind that HAS a lexicon home; `None` for the deliberately
/// unmapped kinds — every `None` is named in [`fields_without_lexicon_home`].
pub fn to_record(p: &Payload) -> Option<Ipld> {
    match p {
        Payload::EdgeHalf(h) => {
            let core = map(vec![
                ("ceremony", Ipld::List(h.core.ceremony.iter().map(strong_ref).collect())),
                ("consent", Ipld::String(h.core.consent.as_str().to_string())),
                ("edgeNonce", Ipld::Bytes(h.core.edge_nonce.to_vec())),
                ("personaA", did_of(&h.core.persona_a)),
                ("personaB", did_of(&h.core.persona_b)),
            ]);
            Some(map(vec![
                ("$type", Ipld::String("ing.croft.attest.edgeHalf".into())),
                ("core", core),
                ("label", Ipld::String(h.label.clone())),
            ]))
        }
        Payload::Vouch(v) => {
            let mut pairs = vec![
                ("$type", Ipld::String("ing.croft.attest.vouch".into())),
                ("madeOn", date_of(&v.made_on)),
                ("scope", Ipld::String(v.scope.0.clone())),
                ("statement", Ipld::String(v.statement.clone())),
                ("subject", did_of(&v.subject)),
            ];
            if let Some(e) = &v.base_edge {
                pairs.push(("baseEdge", Ipld::Bytes(e.to_vec())));
            }
            if let Some(u) = &v.supersedes {
                pairs.push(("supersedes", strong_ref(u)));
            }
            Some(map(pairs))
        }
        Payload::Review(r) => {
            let mut pairs = vec![
                ("$type", Ipld::String("ing.croft.attest.review".into())),
                ("consent", Ipld::String(r.consent.as_str().to_string())),
                ("madeOn", date_of(&r.made_on)),
                ("scope", Ipld::String(r.scope.0.clone())),
                ("statement", Ipld::String(r.statement.clone())),
                ("subject", subject_of(&r.subject)),
            ];
            if let Some(u) = &r.supersedes {
                pairs.push(("supersedes", strong_ref(u)));
            }
            Some(map(pairs))
        }
        Payload::Reply(rp) => Some(map(vec![
            ("$type", Ipld::String("ing.croft.attest.reviewReply".into())),
            ("madeOn", date_of(&rp.made_on)),
            ("review", strong_ref(&rp.review)),
            ("statement", Ipld::String(rp.statement.clone())),
        ])),
        Payload::Credential(c) => {
            let process = map(vec![
                ("method", Ipld::String(c.process.method.as_str().to_string())),
                ("performedOn", date_of(&c.process.performed_on)),
                ("role", Ipld::String(c.process.role.as_str().to_string())),
            ]);
            let mut pairs = vec![
                ("$type", Ipld::String("ing.croft.attest.credential".into())),
                ("mintNonce", Ipld::Bytes(c.mint_nonce.to_vec())),
                ("predicate", Ipld::String(c.predicate.as_str().to_string())),
                ("process", process),
                ("subject", did_of(&c.subject)),
            ];
            if let Some(u) = &c.supersedes {
                pairs.push(("supersedes", strong_ref(u)));
            }
            Some(map(pairs))
        }
        // Every `None` below is a named row in `fields_without_lexicon_home`.
        Payload::EdgeDissolve(_)
        | Payload::CeremonyFact(_)
        | Payload::TransactionFact(_)
        | Payload::ThingDecl(_)
        | Payload::VouchWithdraw(_)
        | Payload::Predicate(_)
        | Payload::ResolvabilityPolicy(_)
        | Payload::VettingFact(_)
        | Payload::CredentialSupersede(_) => None,
    }
}

/// Record shape → payload. Total over exactly the shapes [`to_record`] emits.
pub fn from_record(v: &Ipld) -> Result<Payload, MapError> {
    let m = as_map(v)?;
    match get_str(m, "$type")? {
        "ing.croft.attest.edgeHalf" => {
            let core_m = as_map(get(m, "core")?)?;
            let consent = ConsentMode::from_str(get_str(core_m, "consent")?)
                .ok_or_else(|| MapError("unknown consent mode".into()))?;
            let mut ceremony = Vec::new();
            let Ipld::List(refs) = get(core_m, "ceremony")? else {
                return Err(MapError("ceremony: expected list".into()));
            };
            for r in refs {
                ceremony.push(object_from(r)?);
            }
            Ok(Payload::EdgeHalf(EdgeHalf {
                core: EdgeCore {
                    persona_a: persona_from(get(core_m, "personaA")?)?,
                    persona_b: persona_from(get(core_m, "personaB")?)?,
                    edge_nonce: get_bytes(core_m, "edgeNonce")?
                        .try_into()
                        .map_err(|_| MapError("edgeNonce: expected 16 bytes".into()))?,
                    consent,
                    ceremony,
                },
                label: get_str(m, "label")?.to_string(),
            }))
        }
        "ing.croft.attest.vouch" => {
            let base_edge = match m.get("baseEdge") {
                None => None,
                Some(Ipld::Bytes(b)) => Some(
                    b.clone()
                        .try_into()
                        .map_err(|_| MapError("baseEdge: expected 32 bytes".into()))?,
                ),
                Some(_) => return Err(MapError("baseEdge: expected bytes".into())),
            };
            let supersedes = match m.get("supersedes") {
                None => None,
                Some(r) => Some(object_from(r)?),
            };
            Ok(Payload::Vouch(Vouch {
                subject: persona_from(get(m, "subject")?)?,
                scope: Scope(get_str(m, "scope")?.to_string()),
                statement: get_str(m, "statement")?.to_string(),
                base_edge,
                made_on: date_from(get(m, "madeOn")?)?,
                supersedes,
            }))
        }
        "ing.croft.attest.review" => {
            let supersedes = match m.get("supersedes") {
                None => None,
                Some(r) => Some(object_from(r)?),
            };
            Ok(Payload::Review(Review {
                subject: subject_from(get(m, "subject")?)?,
                scope: Scope(get_str(m, "scope")?.to_string()),
                statement: get_str(m, "statement")?.to_string(),
                consent: ConsentMode::from_str(get_str(m, "consent")?)
                    .ok_or_else(|| MapError("unknown consent mode".into()))?,
                made_on: date_from(get(m, "madeOn")?)?,
                supersedes,
            }))
        }
        "ing.croft.attest.reviewReply" => Ok(Payload::Reply(Reply {
            review: object_from(get(m, "review")?)?,
            statement: get_str(m, "statement")?.to_string(),
            made_on: date_from(get(m, "madeOn")?)?,
        })),
        "ing.croft.attest.credential" => {
            let proc_m = as_map(get(m, "process")?)?;
            let supersedes = match m.get("supersedes") {
                None => None,
                Some(r) => Some(object_from(r)?),
            };
            Ok(Payload::Credential(Credential {
                predicate: PredicateKind::from_str(get_str(m, "predicate")?)
                    .ok_or_else(|| MapError("unknown predicate".into()))?,
                subject: persona_from(get(m, "subject")?)?,
                process: ProcessProvenance {
                    method: MethodKind::from_str(get_str(proc_m, "method")?)
                        .ok_or_else(|| MapError("unknown method".into()))?,
                    performed_on: date_from(get(proc_m, "performedOn")?)?,
                    role: IssuerRole::from_str(get_str(proc_m, "role")?)
                        .ok_or_else(|| MapError("unknown role".into()))?,
                },
                mint_nonce: get_bytes(m, "mintNonce")?
                    .try_into()
                    .map_err(|_| MapError("mintNonce: expected 16 bytes".into()))?,
                supersedes,
            }))
        }
        other => Err(MapError(format!("unknown record type {other}"))),
    }
}

// ---------------------------------------------------------------------------
// EpochRecord ↔ ing.croft.attest.commitmentEpoch
// ---------------------------------------------------------------------------

/// The commitmentEpoch record shape for an issuer epoch. The detached epoch
/// signature is deliberately NOT mapped (repo commit signature replaces it —
/// a `fields_without_lexicon_home` row); the round-trip is lossless over the
/// signed content (epoch number, commitment set, declared total).
pub fn epoch_to_record(r: &EpochRecord) -> Ipld {
    map(vec![
        ("$type", Ipld::String("ing.croft.attest.commitmentEpoch".into())),
        (
            "commitments",
            Ipld::List(r.commitments.iter().map(|c| Ipld::Bytes(c.to_vec())).collect()),
        ),
        ("declaredTotal", Ipld::Integer(r.declared_total as i128)),
        ("epochNo", Ipld::Integer(r.epoch_no as i128)),
    ])
}

/// Record shape → epoch content. The returned record carries an EMPTY
/// signature (the unmapped surface); compare signed content, not signatures.
pub fn epoch_from_record(v: &Ipld) -> Result<EpochRecord, MapError> {
    let m = as_map(v)?;
    if get_str(m, "$type")? != "ing.croft.attest.commitmentEpoch" {
        return Err(MapError("expected commitmentEpoch".into()));
    }
    let Ipld::List(cs) = get(m, "commitments")? else {
        return Err(MapError("commitments: expected list".into()));
    };
    let mut commitments = std::collections::BTreeSet::new();
    for c in cs {
        let Ipld::Bytes(b) = c else { return Err(MapError("commitment: expected bytes".into())) };
        let arr: [u8; 32] = b
            .clone()
            .try_into()
            .map_err(|_| MapError("commitment: expected 32 bytes".into()))?;
        commitments.insert(arr);
    }
    Ok(EpochRecord {
        epoch_no: get_int(m, "epochNo")?,
        commitments,
        declared_total: get_int(m, "declaredTotal")?,
        signature: Vec::new(),
    })
}
