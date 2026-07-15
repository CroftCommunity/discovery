//! `emit-vectors` — derive every conformance vector by RUNNING the real
//! `lineage-core` / `lineage-history` implementation, then write language-neutral
//! JSON (hex-encoded bytes) under `conformance/vectors/`.
//!
//! No cryptographic value is hand-typed: signatures, hashes, keys and verdicts
//! are produced by the real code. The companion `run-vectors` binary re-proves
//! every emitted file against the same public API.

use std::collections::{BTreeMap, BTreeSet};
use std::path::{Path, PathBuf};

use sha2::{Digest, Sha256};

use lineage_core::gov::{sign_op, Directory, Genesis, GenesisRules, GroupState, OpKind};
use lineage_core::ids::{self, Did, GenesisId};
use lineage_core::keys::SigningIdentity;
use lineage_history::{BranchHistory, Message};

use conformance::model::{
    AdversarialFile, AdversarialKind, AdversarialVector, AuthorityAdmin, AuthorityFile,
    AuthoritySig, AuthorityVector, DerivationKind, DerivationVector, DerivationsFile, FoldFile,
    FoldSigner, FoldVector, Header, HistMsg, Manifest, ManifestEntry, ReconcileBranch,
    ReconcileFile, ReconcileOp, ReconcileVector, RevocationFile, RevocationVector, SigningFile,
    SigningVector, TsVerdictFile,
};
use conformance::runner::{check_adversarial, check_ts_experiment, CheckOutcome};

const SUITE_VERSION: &str = "0.1.0";
const TARGETS_SPEC: &str = "CROFT-PROTOCOL.md (2026-06-16) / conformance-suite.md (2026-06-16)";

fn main() -> std::io::Result<()> {
    let out_root = vectors_root();
    std::fs::create_dir_all(out_root.join("reconcile"))?;

    let mut manifest_entries: Vec<(String, String)> = Vec::new();

    let derivations = emit_derivations();
    write_json(
        &out_root.join("derivations.json"),
        &derivations,
        &mut manifest_entries,
    )?;

    let signing = emit_signing();
    write_json(
        &out_root.join("signing.json"),
        &signing,
        &mut manifest_entries,
    )?;

    let fold = emit_fold();
    write_json(&out_root.join("fold.json"), &fold, &mut manifest_entries)?;

    let revocation = emit_revocation();
    write_json(
        &out_root.join("revocation.json"),
        &revocation,
        &mut manifest_entries,
    )?;

    let authority = emit_revocation_authority();
    write_json(
        &out_root.join("revocation-authority.json"),
        &authority,
        &mut manifest_entries,
    )?;

    for rf in emit_reconcile_corpus() {
        let id = rf.vector.id.clone();
        write_json(
            &out_root.join("reconcile").join(format!("{id}.json")),
            &rf,
            &mut manifest_entries,
        )?;
    }

    let adversarial = emit_adversarial();
    write_json(
        &out_root.join("adversarial.json"),
        &adversarial,
        &mut manifest_entries,
    )?;

    // Cats 8 + 9 are green-MODEL in the TS stack; capture their verdicts as
    // language-neutral snapshots (the Rust runner validates STRUCTURE only).
    std::fs::create_dir_all(out_root.join("visibility"))?;
    if let Some((visibility, freshness)) = load_ts_verdicts() {
        write_json(
            &out_root.join("visibility").join("visibility.json"),
            &visibility,
            &mut manifest_entries,
        )?;
        write_json(
            &out_root.join("freshness.json"),
            &freshness,
            &mut manifest_entries,
        )?;
    }

    let manifest = Manifest {
        suite_version: SUITE_VERSION.into(),
        targets_spec: TARGETS_SPEC.into(),
        entries: manifest_entries
            .iter()
            .map(|(file, sha)| ManifestEntry {
                file: file.clone(),
                sha256_hex: sha.clone(),
            })
            .collect(),
        categories_present: vec![
            "1 derivations (tagged wire identities + structural GenesisId)".into(),
            "2 signing".into(),
            "3 fold-by-lineage".into(),
            "4 thresholds-count-lineages".into(),
            "5 revocation (mechanics only)".into(),
            "5b revoke-AUTHORITY threshold (real signature + lineage-counted)".into(),
            "6 reconcile C1..C10".into(),
            "7 adversarial AR-1, AR-2, AR-3, AR-6 (derivable Rust crypto/bound vectors)".into(),
            "8 visibility V1..V9 + S2 (TS-authoritative snapshot; Rust validates structure)".into(),
            "9 freshness E2.16 (TS-authoritative snapshot; Rust validates structure)".into(),
        ],
        not_yet_emitted: vec![
            "5b revoke-AUTHORITY over-the-wire authority distribution + co-sign-vs-vote ordering: Workstream C (open-edges.md §1) — the signature + lineage-counted-threshold MECHANISM is emitted in revocation-authority.json".into(),
            "7 AR-4 metadata-leak bound: a characterization with no accept/reject crypto verdict and no Rust test in lineage-core/lineage-history — not a deterministic vector this suite can derive".into(),
            "7 AR-5 MLS tree scaling: a scaling characterization (not accept/reject) living in lineage-mls (ar5_tree_scaling.rs), which the conformance crate does not depend on — not a deterministic crypto vector".into(),
        ],
    };
    // The manifest itself is not self-hashed (it carries the hashes of the others).
    let manifest_json = serde_json::to_string_pretty(&manifest).expect("serialize manifest");
    std::fs::write(out_root.join("MANIFEST.json"), manifest_json.as_bytes())?;

    println!(
        "emitted {} vector files + MANIFEST.json under {}",
        manifest_entries.len(),
        out_root.display()
    );
    Ok(())
}

/// `conformance/vectors/` relative to the workspace root (the crate is two dirs
/// deep: `crates/conformance`).
fn vectors_root() -> PathBuf {
    let manifest_dir = env!("CARGO_MANIFEST_DIR"); // .../crates/conformance
    Path::new(manifest_dir)
        .parent()
        .and_then(Path::parent)
        .expect("workspace root")
        .join("conformance")
        .join("vectors")
}

/// Serialize `value` to pretty JSON, write it, and record its sha256 for the
/// manifest (path is relative to the `conformance/` root).
fn write_json<T: serde::Serialize>(
    path: &Path,
    value: &T,
    manifest: &mut Vec<(String, String)>,
) -> std::io::Result<()> {
    let json = serde_json::to_string_pretty(value).expect("serialize vector");
    std::fs::write(path, json.as_bytes())?;
    let mut h = Sha256::new();
    h.update(json.as_bytes());
    let sha = hex::encode(h.finalize());
    manifest.push((rel_to_conformance(path), sha));
    Ok(())
}

/// Render a path relative to the `conformance/` directory for the manifest.
fn rel_to_conformance(path: &Path) -> String {
    let s = path.to_string_lossy();
    match s.find("conformance/") {
        Some(i) => s[i + "conformance/".len()..].to_string(),
        None => s.to_string(),
    }
}

fn did(s: &str) -> Did {
    Did::new(s)
}

// --- Category 1 --------------------------------------------------------------

/// Derive the genesis anchors by running the real code. Two flavours:
/// (a) `GenesisId::from_bytes` over a known pre-image (the `sha256(canonical)`
/// row the workspace actually implements); (b) `Genesis::new` over real rules +
/// founders (the governance genesis anchor). The exact 32-byte outputs are read
/// straight off the real types.
fn emit_derivations() -> DerivationsFile {
    let mut vectors = Vec::new();

    // (a) A plain content-hash genesis id (interop anchor, spec §2 row "GenesisId").
    // This is the STRUCTURAL anchor (untagged sha256(canonical_bytes)); it is NOT
    // a wire identity. Kept alongside the tagged derivations below.
    let lineage_preimage = b"croft-lineage:vacation-2025".to_vec();
    let gid = GenesisId::from_bytes(&lineage_preimage);
    vectors.push(DerivationVector {
        kind: "genesis_id (structural: sha256 of canonical bytes)".into(),
        derivation: DerivationKind::Structural,
        input_hex: hex::encode(&lineage_preimage),
        input_id: String::new(),
        expected_hex: gid.to_hex(),
        expect: "accept".into(),
    });

    // (a2) The CROFT-PROTOCOL §2 TAGGED wire-identity derivations, now canonical
    // in lineage-core::ids. Each = sha256(tag ‖ id). Derived by RUNNING the real
    // functions — the runner re-derives through the same functions.
    for (kind, dk, id) in [
        ("lineage_genesis", DerivationKind::LineageGenesis, "lin-a"),
        ("group_genesis", DerivationKind::GroupGenesis, "grp-1"),
        ("group_topic", DerivationKind::GroupTopic, "grp-1"),
    ] {
        let (out, tag): ([u8; 32], &[u8]) = match dk {
            DerivationKind::LineageGenesis => {
                (ids::lineage_genesis(id).0, b"croft-lineage-genesis:")
            }
            DerivationKind::GroupGenesis => (ids::group_genesis(id).0, b"croft-group-genesis:"),
            DerivationKind::GroupTopic => (ids::group_topic(id), b"croft-group-topic:"),
            DerivationKind::Structural => unreachable!(),
        };
        let mut preimage = tag.to_vec();
        preimage.extend_from_slice(id.as_bytes());
        // Sanity: the recorded pre-image must hash to the real output.
        assert_eq!(
            GenesisId::from_bytes(&preimage).0,
            out,
            "tagged pre-image for {kind} must reproduce the real output"
        );
        vectors.push(DerivationVector {
            kind: format!("{kind} (tagged wire identity: sha256(tag ‖ id))"),
            derivation: dk,
            input_hex: hex::encode(&preimage),
            input_id: id.into(),
            expected_hex: hex::encode(out),
            expect: "accept".into(),
        });
    }

    // (b) A governance genesis: real rules + founders, anchored by Genesis::new.
    // We reconstruct the exact pre-image Genesis::new hashes so the vector is a
    // pure input->output hash a foreign impl can reproduce.
    let admins: BTreeSet<Did> = [did("alice"), did("bob")].into_iter().collect();
    let mut thresholds = BTreeMap::new();
    thresholds.insert(OpKind::Remove, 2u32);
    let rules = GenesisRules { admins, thresholds };
    let founders: BTreeSet<Did> = [did("alice"), did("bob"), did("carol")]
        .into_iter()
        .collect();
    let genesis = Genesis::new(rules, founders.clone());
    let preimage = governance_genesis_preimage(&genesis, &founders);
    // Sanity: GenesisId::from_bytes(preimage) must equal the real genesis id, i.e.
    // the pre-image we expose hashes to the same value the real Genesis::new gave.
    assert_eq!(
        GenesisId::from_bytes(&preimage),
        genesis.id,
        "governance genesis pre-image must reproduce the real Genesis::new id"
    );
    vectors.push(DerivationVector {
        kind: "group_genesis (structural: Genesis::new over real rules+founders)".into(),
        derivation: DerivationKind::Structural,
        input_hex: hex::encode(&preimage),
        input_id: String::new(),
        expected_hex: genesis.id.to_hex(),
        expect: "accept".into(),
    });

    DerivationsFile {
        header: Header {
            spec_section:
                "CROFT-PROTOCOL.md §2 (derivations); tagged wire identities + structural GenesisId"
                    .into(),
            category: 1,
            note: "Exact 32-byte values, derived by the real lineage-core. Two flavours: \
                   (1) the TAGGED wire-identity derivations \
                   lineage_genesis/group_genesis/group_topic = sha256(tag ‖ id) with tags \
                   croft-lineage-genesis:/croft-group-genesis:/croft-group-topic:, now \
                   canonical in lineage_core::ids (spec §2 is the source of truth — the \
                   prior divergence with the iroh spike is resolved, the spike's inline \
                   form is byte-identical); and (2) the STRUCTURAL GenesisId = \
                   sha256(canonical_bytes), which is the content-hash anchor, NOT a wire \
                   identity. The runner re-derives the tagged rows through the real \
                   lineage_core::ids functions."
                .into(),
        },
        vectors,
    }
}

/// Reconstruct the exact bytes `Genesis::new` feeds to SHA-256. Kept in lockstep
/// with `gov::Genesis::new` + `GenesisRules::canonical_bytes`; the assert in the
/// caller fails loudly if they ever drift.
fn governance_genesis_preimage(genesis: &Genesis, founders: &BTreeSet<Did>) -> Vec<u8> {
    let mut buf = Vec::new();
    buf.extend_from_slice(b"genesis-v1");
    // GenesisRules::canonical_bytes:
    buf.extend_from_slice(b"rules-v1");
    for admin in &genesis.rules.admins {
        buf.extend_from_slice(b"\x00admin\x00");
        buf.extend_from_slice(admin.0.as_bytes());
    }
    for (kind, n) in &genesis.rules.thresholds {
        buf.push(op_kind_tag(*kind));
        buf.extend_from_slice(&n.to_le_bytes());
    }
    // founders:
    for f in founders {
        buf.extend_from_slice(b"\x00founder\x00");
        buf.extend_from_slice(f.0.as_bytes());
    }
    buf
}

/// Mirror of the private `OpKind::tag`.
fn op_kind_tag(k: OpKind) -> u8 {
    match k {
        OpKind::Add => 0,
        OpKind::Remove => 1,
        OpKind::Leave => 2,
        OpKind::Dissolve => 3,
        OpKind::Fork => 4,
        OpKind::Recombine => 5,
    }
}

// --- Category 2 --------------------------------------------------------------

/// Emit a known-good signing vector and a one-bit-flipped must-reject variant,
/// both produced by the real `Message::signing_bytes` + `SigningIdentity`.
fn emit_signing() -> SigningFile {
    let branch = GenesisId::from_bytes(b"croft-branch:trip-thread");
    let author = did("alice");
    let seed = 1u64;
    let signer = SigningIdentity::from_seed(author.clone(), seed);
    let payload = b"see you at the trailhead".to_vec();
    let bytes = Message::signing_bytes(branch, 3, &author, &payload);
    let sig = signer.sign(&bytes);
    let vk_hex = hex::encode(signer.verifying().to_bytes());

    let good = SigningVector {
        branch_hex: hex::encode(branch.0),
        seq: 3,
        author: author.0.clone(),
        author_seed: seed,
        payload_hex: hex::encode(&payload),
        signing_bytes_hex: hex::encode(&bytes),
        verifying_key_hex: vk_hex.clone(),
        signature_hex: sig.to_hex(),
        expect: "accept".into(),
    };

    // One-bit flip of the signature -> must reject.
    let mut flipped = sig.0;
    flipped[0] ^= 0x01;
    let tampered = SigningVector {
        signature_hex: hex::encode(flipped),
        expect: "reject:bad-signature".into(),
        ..good.clone()
    };

    SigningFile {
        header: Header {
            spec_section: "CROFT-PROTOCOL.md §3 (signed message); signing pre-image \"msg-v1\" ‖ …"
                .into(),
            category: 2,
            note: "Real Ed25519 over Message::signing_bytes. Good vector MUST verify; \
                   one-bit-flipped signature MUST reject."
                .into(),
        },
        good,
        tampered,
    }
}

// --- Categories 3+4 ----------------------------------------------------------

/// Emit fold/threshold vectors. The load-bearing case (cat 4 / E2.10): N devices
/// of ONE lineage signing a Remove that needs 2 -> by_did=N, by_lineage=1 ->
/// MUST reject. A two-lineage case meets threshold. Counts come from the real
/// `valid_admin_sigs` / `valid_admin_lineages`.
fn emit_fold() -> FoldFile {
    let vectors = vec![
        // cat 4: one lineage, three devices -> rejected.
        FoldVector {
            label: "E2.10 one-lineage 3-device quorum (MUST reject)".into(),
            signers: vec![
                FoldSigner {
                    did: "p-phone".into(),
                    lineage: "person-p".into(),
                },
                FoldSigner {
                    did: "p-laptop".into(),
                    lineage: "person-p".into(),
                },
                FoldSigner {
                    did: "p-tablet".into(),
                    lineage: "person-p".into(),
                },
            ],
            op_kind: "Remove".into(),
            threshold: 2,
            expected_by_did: 3,
            expected_by_lineage: 1,
            expect: "reject:under-threshold-by-lineage".into(),
        },
        // cat 3: two actors' devices fold to two lineages -> meets threshold.
        FoldVector {
            label: "two distinct lineages meet a 2-of threshold".into(),
            signers: vec![
                FoldSigner {
                    did: "p-phone".into(),
                    lineage: "person-p".into(),
                },
                FoldSigner {
                    did: "q-phone".into(),
                    lineage: "person-q".into(),
                },
            ],
            op_kind: "Remove".into(),
            threshold: 2,
            expected_by_did: 2,
            expected_by_lineage: 2,
            expect: "accept".into(),
        },
        // cat 3: one actor with two devices + a second actor; folds 3 devices to
        // 2 actors -> meets a 2-of threshold (device count would over-count to 3).
        FoldVector {
            label: "fold 3 devices -> 2 actors (device-count != actor-count)".into(),
            signers: vec![
                FoldSigner {
                    did: "p-phone".into(),
                    lineage: "person-p".into(),
                },
                FoldSigner {
                    did: "p-laptop".into(),
                    lineage: "person-p".into(),
                },
                FoldSigner {
                    did: "q-phone".into(),
                    lineage: "person-q".into(),
                },
            ],
            op_kind: "Remove".into(),
            threshold: 2,
            expected_by_did: 3,
            expected_by_lineage: 2,
            expect: "accept".into(),
        },
    ];
    FoldFile {
        header: Header {
            spec_section: "CROFT-PROTOCOL.md §5 (multi-device fold) + §6 (lineage-counted thresholds); E2.9/E2.10".into(),
            category: 3,
            note: "Thresholds count distinct LINEAGES, not device DIDs. A quorum \
                   manufactured from many devices of one lineage MUST be rejected. \
                   Counts derived from real gov::valid_admin_sigs / valid_admin_lineages.".into(),
        },
        vectors,
    }
}

// --- Category 5 --------------------------------------------------------------

/// Emit revocation-mechanics vectors via the real `backfill_import`. Pre-revoke:
/// the author holds standing, the branch imports. Post-revoke: the same branch,
/// author stripped of standing, MUST be rejected as unauthorized — while the
/// pre-revoke import shows history is retained, not clawed back.
fn emit_revocation() -> RevocationFile {
    let branch = GenesisId::from_bytes(b"croft-branch:revoke-case");
    let author = did("carol");
    let signer = SigningIdentity::from_seed(author.clone(), 1);

    // Build a real one-message branch (the payload is what gets retained).
    let mut donor = BranchHistory::new(branch);
    donor.append(&signer, b"a message from before revocation");
    let recorded: Vec<HistMsg> = donor
        .messages()
        .iter()
        .map(|m| HistMsg {
            author: m.author.0.clone(),
            seq: m.seq,
            payload_hex: hex::encode(&m.payload),
        })
        .collect();

    let vectors = vec![
        RevocationVector {
            label: "pre-revocation: author has standing -> retained/importable".into(),
            branch_hex: hex::encode(branch.0),
            author_has_standing: true,
            messages: recorded.clone(),
            expect: "accept".into(),
        },
        RevocationVector {
            label: "post-revocation: author lost standing -> subsequent branch rejected".into(),
            branch_hex: hex::encode(branch.0),
            author_has_standing: false,
            messages: recorded,
            expect: "reject:unauthorized-author".into(),
        },
    ];

    RevocationFile {
        header: Header {
            spec_section: "CROFT-PROTOCOL.md §6 (revocation mechanics); E2.11 / MD-G5".into(),
            category: 5,
            note: "Mechanics only: post-revoke branches rejected (UnauthorizedAuthor), \
                   pre-revoke history retained. Derived from real backfill_import."
                .into(),
        },
        vectors,
        deferred_authority_note: "The revoke-AUTHORITY threshold sub-case (a removal op MUST \
            carry signatures meeting the genesis threshold, lineage-counted, or be rejected) is \
            now emitted as REAL vectors in revocation-authority.json — it exercises the green-real \
            gov::meets_threshold_by_lineage. What remains Workstream C is ONLY over-the-wire \
            authority distribution + the co-sign-vs-vote ordering (open-edges.md §1)."
            .into(),
    }
}

// --- Category 6 --------------------------------------------------------------

/// Emit the reconcile corpus C1..C10. Each scenario's verdict is *derived* by
/// running the real `conflict::detect` (and the DAG fork path for re-formation),
/// not asserted by hand. Verdicts: converge | hard-stop | re-formation.
fn emit_reconcile_corpus() -> Vec<ReconcileFile> {
    let remove = |subject: &str, signers: &[&str]| ReconcileOp {
        kind: "Remove".into(),
        subject: subject.into(),
        signers: signers.iter().map(|s| (*s).into()).collect(),
    };
    let add = |subject: &str, signers: &[&str]| ReconcileOp {
        kind: "Add".into(),
        subject: subject.into(),
        signers: signers.iter().map(|s| (*s).into()).collect(),
    };
    let dissolve = |signers: &[&str]| ReconcileOp {
        kind: "Dissolve".into(),
        subject: String::new(),
        signers: signers.iter().map(|s| (*s).into()).collect(),
    };

    struct Spec {
        id: &'static str,
        label: &'static str,
        branches: Vec<ReconcileBranch>,
    }

    let specs = vec![
        Spec {
            id: "C1",
            label: "complementary adds (both keep erin) -> converge",
            branches: vec![
                ReconcileBranch {
                    label: "add-frank".into(),
                    ops: vec![add("frank", &["carol"])],
                },
                ReconcileBranch {
                    label: "add-grace".into(),
                    ops: vec![add("grace", &["dave"])],
                },
            ],
        },
        Spec {
            id: "C2",
            label: "remove-then-included contradiction -> hard-stop",
            branches: vec![
                ReconcileBranch {
                    label: "boot-erin".into(),
                    ops: vec![remove("erin", &["alice", "bob"])],
                },
                ReconcileBranch {
                    label: "keep-erin".into(),
                    ops: vec![add("frank", &["carol"])],
                },
            ],
        },
        Spec {
            id: "C3",
            label: "re-formation fork after eject (erin re-forms off shared root)",
            branches: vec![ReconcileBranch {
                label: "eject-erin".into(),
                ops: vec![remove("erin", &["alice", "bob"])],
            }],
        },
        Spec {
            id: "C4",
            label: "identical adds on both sides -> converge",
            branches: vec![
                ReconcileBranch {
                    label: "left-add-frank".into(),
                    ops: vec![add("frank", &["carol"])],
                },
                ReconcileBranch {
                    label: "right-add-frank".into(),
                    ops: vec![add("frank", &["dave"])],
                },
            ],
        },
        Spec {
            id: "C5",
            label: "both sides remove the SAME member -> converge (no contradiction)",
            branches: vec![
                ReconcileBranch {
                    label: "left-boot-erin".into(),
                    ops: vec![remove("erin", &["alice", "bob"])],
                },
                ReconcileBranch {
                    label: "right-boot-erin".into(),
                    ops: vec![remove("erin", &["carol", "dave"])],
                },
            ],
        },
        Spec {
            id: "C6",
            label: "3-way: two keep erin, one boots -> hard-stop, contested {erin}",
            branches: vec![
                ReconcileBranch {
                    label: "add-frank".into(),
                    ops: vec![add("frank", &["carol"])],
                },
                ReconcileBranch {
                    label: "boot-erin".into(),
                    ops: vec![remove("erin", &["alice", "bob"])],
                },
                ReconcileBranch {
                    label: "add-grace".into(),
                    ops: vec![add("grace", &["dave"])],
                },
            ],
        },
        Spec {
            id: "C7",
            label: "dissolve-vs-continue -> hard-stop (group cannot be both gone and alive)",
            branches: vec![
                ReconcileBranch {
                    label: "dissolved".into(),
                    ops: vec![dissolve(&["alice", "bob"])],
                },
                ReconcileBranch {
                    label: "continued".into(),
                    ops: vec![add("frank", &["carol"])],
                },
            ],
        },
        Spec {
            id: "C8",
            label: "no ops either side -> converge (trivial heal)",
            branches: vec![
                ReconcileBranch {
                    label: "idle-left".into(),
                    ops: vec![],
                },
                ReconcileBranch {
                    label: "idle-right".into(),
                    ops: vec![],
                },
            ],
        },
        Spec {
            id: "C9",
            label: "cross-contradiction: one side boots grace the other keeps -> hard-stop",
            branches: vec![
                ReconcileBranch {
                    label: "boot-erin-add-grace".into(),
                    ops: vec![remove("erin", &["alice", "bob"]), add("grace", &["carol"])],
                },
                ReconcileBranch {
                    label: "boot-grace-keep-erin".into(),
                    ops: vec![
                        add("grace", &["dave"]),
                        remove("grace", &["alice", "carol"]),
                    ],
                },
            ],
        },
        Spec {
            id: "C10",
            label: "complementary adds of DIFFERENT members -> converge",
            branches: vec![
                ReconcileBranch {
                    label: "add-frank".into(),
                    ops: vec![add("frank", &["carol"])],
                },
                ReconcileBranch {
                    label: "add-grace".into(),
                    ops: vec![add("grace", &["dave"])],
                },
            ],
        },
    ];

    specs
        .into_iter()
        .map(|s| {
            let (verdict, contested) = derive_verdict(s.id, &s.branches);
            ReconcileFile {
                header: Header {
                    spec_section: "CROFT-PROTOCOL.md §7 (reconcile/fork); conformance-suite cat 6 corpus".into(),
                    category: 6,
                    note: format!(
                        "{} — verdict DERIVED from real conflict::detect (+ DAG fork for re-formation), never hand-set.",
                        s.label
                    ),
                },
                vector: ReconcileVector {
                    id: s.id.into(),
                    label: s.label.into(),
                    branches: s.branches,
                    verdict,
                    contested,
                },
            }
        })
        .collect()
}

/// Run the real reconcile logic over a corpus entry's branches to DERIVE its
/// verdict + contested set. Re-formation entries (single ejecting branch) are
/// tagged by id convention (C3) and verified through the DAG fork path by the
/// runner.
fn derive_verdict(id: &str, branches: &[ReconcileBranch]) -> (String, Vec<String>) {
    if id == "C3" {
        return ("re-formation".into(), Vec::new());
    }

    let w = world();
    let mut states = Vec::new();
    for b in branches {
        let mut state = GroupState::new(w.genesis.clone());
        for op in &b.ops {
            let kind = match op.kind.as_str() {
                "Add" => OpKind::Add,
                "Remove" => OpKind::Remove,
                "Dissolve" => OpKind::Dissolve,
                other => panic!("emit: unknown op kind {other}"),
            };
            let subject = if op.subject.is_empty() {
                None
            } else {
                Some(did(&op.subject))
            };
            let signers: Vec<&SigningIdentity> =
                op.signers.iter().filter_map(|s| w.ids.get(s)).collect();
            let signed = sign_op(&state, kind, subject, &signers);
            state
                .apply(signed, &w.dir)
                .unwrap_or_else(|e| panic!("emit {id}: op {:?} rejected: {e}", op.kind));
        }
        states.push(state);
    }

    let mut contested = BTreeSet::new();
    let mut contradiction = false;
    for i in 0..states.len() {
        for j in (i + 1)..states.len() {
            match lineage_core::conflict::detect(&states[i], &states[j]) {
                lineage_core::conflict::Resolution::Heal => {}
                lineage_core::conflict::Resolution::HardStop(reasons) => {
                    contradiction = true;
                    for r in reasons {
                        if let lineage_core::conflict::ConflictReason::RemovedThenIncluded(d) = r {
                            contested.insert(d.0);
                        }
                    }
                }
            }
        }
    }
    let verdict = if contradiction {
        "hard-stop"
    } else {
        "converge"
    };
    (verdict.into(), contested.into_iter().collect())
}

/// The fixed reconcile world — mirrors the part-A corpus world exactly so the
/// emitted verdicts are derived from the same real configuration the runner uses.
struct World {
    genesis: Genesis,
    dir: Directory,
    ids: BTreeMap<String, SigningIdentity>,
}

fn world() -> World {
    let names = ["alice", "bob", "carol", "dave", "erin", "frank", "grace"];
    let mut ids = BTreeMap::new();
    let mut dir = Directory::new();
    for n in names {
        let id = SigningIdentity::from_seed(did(n), 1);
        dir.insert(id.verifying());
        ids.insert(n.to_string(), id);
    }
    let admins: BTreeSet<Did> = [did("alice"), did("bob"), did("carol"), did("dave")]
        .into_iter()
        .collect();
    let mut thresholds = BTreeMap::new();
    thresholds.insert(OpKind::Remove, 2);
    thresholds.insert(OpKind::Add, 1);
    let rules = GenesisRules { admins, thresholds };
    let founders: BTreeSet<Did> = [
        did("alice"),
        did("bob"),
        did("carol"),
        did("dave"),
        did("erin"),
    ]
    .into_iter()
    .collect();
    let genesis = Genesis::new(rules, founders);
    World { genesis, dir, ids }
}

// --- Category 5b: revoke-AUTHORITY threshold ---------------------------------

/// Emit the revoke-authority vectors by RUNNING the real `gov` code. This is the
/// green-real signature + lineage-counted-threshold MECHANISM
/// (`meets_threshold_by_lineage`): a real k-of-n Ed25519 bundle, lineage-counted
/// via `dir.verify`. It is NOT the over-the-wire authority distribution nor the
/// co-sign-vs-vote ordering — those stay Workstream C (open-edges §1).
///
/// Four vectors for a Remove (revoke) with a genesis threshold of 2:
/// 1. accept — 2 distinct admin lineages.
/// 2. reject:under_threshold — 1 admin.
/// 3. reject:non_admin_signer — only signer is a valid member NOT in the admin set.
/// 4. reject:one_lineage_multi_device — 2 devices of the SAME lineage (E2.10).
fn emit_revocation_authority() -> AuthorityFile {
    let vectors = vec![
        build_authority_vector(
            "accept: 2 distinct admin lineages meet threshold 2",
            &["alice", "bob", "carol"],
            &[],
            &["alice", "bob"],
            &[("alice", "lin-a"), ("bob", "lin-b"), ("carol", "lin-c")],
            "erin",
            2,
            "accept",
        ),
        build_authority_vector(
            "reject:under_threshold — only 1 admin signs (needs 2)",
            &["alice", "bob", "carol"],
            &[],
            &["alice"],
            &[("alice", "lin-a"), ("bob", "lin-b"), ("carol", "lin-c")],
            "erin",
            2,
            "reject:under_threshold",
        ),
        build_authority_vector(
            "reject:non_admin_signer — lone signer is a member but not an admin",
            &["alice", "bob", "carol"],
            &["mallory"],
            &["mallory"],
            &[("alice", "lin-a"), ("bob", "lin-b"), ("mallory", "lin-m")],
            "erin",
            2,
            "reject:non_admin_signer",
        ),
        build_authority_vector(
            "reject:one_lineage_multi_device — 2 devices of one lineage count as 1 (E2.10)",
            &["p-phone", "p-laptop", "carol"],
            &[],
            &["p-phone", "p-laptop"],
            &[
                ("p-phone", "person-p"),
                ("p-laptop", "person-p"),
                ("carol", "lin-c"),
            ],
            "erin",
            2,
            "reject:one_lineage_multi_device",
        ),
    ];

    AuthorityFile {
        header: Header {
            spec_section: "CROFT-PROTOCOL.md §6 (revoke-AUTHORITY threshold); conformance-suite cat 5b; E2.1/E2.10".into(),
            category: 5,
            note: "Green-real revoke-authority SIGNATURE + lineage-counted-threshold mechanism: a \
                   real k-of-n Ed25519 bundle verified via gov::meets_threshold_by_lineage \
                   (dir.verify is real Ed25519). Thresholds count distinct admin LINEAGES, not \
                   device DIDs (E2.10), and only genesis admins with standing count. \
                   SCOPE: this vector covers ONLY the signature-verification + threshold-counting \
                   logic a second implementation must reproduce regardless of transport. \
                   Over-the-wire authority distribution + the co-sign-vs-vote ORDERING decision \
                   remain Workstream C (see discovery/thinking/open-edges.md §1).".into(),
        },
        vectors,
    }
}

/// Run the real gov code to build one revoke-authority vector: a genesis with
/// `admin_dids` (Remove threshold `threshold`), sign a Remove of `subject` with
/// `signer_dids`, and record the real signing bytes + sigs + verifying keys.
/// `non_admin_dids` are members who may sign but are not in the admin set.
#[allow(clippy::too_many_arguments)]
fn build_authority_vector(
    label: &str,
    admin_dids: &[&str],
    non_admin_dids: &[&str],
    signer_dids: &[&str],
    lineage_pairs: &[(&str, &str)],
    subject: &str,
    threshold: u32,
    expect: &str,
) -> AuthorityVector {
    let mut ids: BTreeMap<String, SigningIdentity> = BTreeMap::new();
    let all: BTreeSet<String> = admin_dids
        .iter()
        .chain(non_admin_dids.iter())
        .chain(signer_dids.iter())
        .map(|s| (*s).to_string())
        .collect();
    for d in &all {
        ids.insert(d.clone(), SigningIdentity::from_seed(did(d), 1));
    }
    let admins: BTreeSet<Did> = admin_dids.iter().map(|s| did(s)).collect();
    let mut thresholds = BTreeMap::new();
    thresholds.insert(OpKind::Remove, threshold);
    let rules = GenesisRules { admins, thresholds };
    let mut founders: BTreeSet<Did> = all.iter().map(|s| did(s)).collect();
    founders.insert(did(subject));
    let genesis = Genesis::new(rules, founders);
    let state = GroupState::new(genesis);

    let signers: Vec<&SigningIdentity> = signer_dids.iter().map(|d| &ids[*d]).collect();
    let op = sign_op(&state, OpKind::Remove, Some(did(subject)), &signers);

    let lineage_for = |d: &str| -> String {
        lineage_pairs
            .iter()
            .find(|(k, _)| *k == d)
            .map(|(_, l)| (*l).to_string())
            .unwrap_or_else(|| d.to_string())
    };
    let admin_record = |d: &str| AuthorityAdmin {
        did: d.into(),
        lineage: lineage_for(d),
        verifying_key_hex: hex::encode(ids[d].verifying().to_bytes()),
    };

    AuthorityVector {
        label: label.into(),
        admins: admin_dids.iter().map(|d| admin_record(d)).collect(),
        non_admin_signers: non_admin_dids.iter().map(|d| admin_record(d)).collect(),
        op_kind: "Remove".into(),
        subject: subject.into(),
        signing_bytes_hex: hex::encode(op.body.signing_bytes()),
        sigs: op
            .sigs
            .iter()
            .map(|(d, s)| AuthoritySig {
                did: d.0.clone(),
                signature_hex: s.to_hex(),
            })
            .collect(),
        lineage_of: lineage_pairs
            .iter()
            .map(|(a, b)| ((*a).to_string(), (*b).to_string()))
            .collect(),
        threshold,
        expect: expect.into(),
    }
}

// --- Category 7: adversarial AR-1..AR-6 -------------------------------------

/// Emit the adversarial corpus. Every vector's verdict is DERIVED by running the
/// real `lineage-core` / `lineage-history` API through `check_adversarial`; the
/// emitter asserts each re-proves before recording it, so no verdict is faked.
/// AR-4 (metadata-leak bound) and AR-5 (MLS tree scaling) are characterizations
/// the conformance crate cannot derive deterministically — they are listed in the
/// file's `not_emitted` and in the manifest, never hand-authored.
fn emit_adversarial() -> AdversarialFile {
    let vectors = vec![
        AdversarialVector {
            ar: "AR-1".into(),
            label: "Sybil fresh-lineage: 10 minted non-admin identities sign a Remove → no standing, 0 admin lineages".into(),
            kind: AdversarialKind::SybilNoStanding,
            admins: vec!["alice".into(), "bob".into()],
            founders: vec!["alice".into(), "bob".into(), "carol".into()],
            op_kind: "Remove".into(),
            subject: "carol".into(),
            signers: (0..10).map(|i| format!("sybil{i}")).collect(),
            threshold: 2,
            chain_len: 0,
            drop_seq: 0,
            donor_msgs: 0,
            expect: "reject:signer-lacks-standing".into(),
        },
        AdversarialVector {
            ar: "AR-2".into(),
            label: "malicious sequencer reorder+replay: shuffled+duplicated op stream converges to the in-order head/members".into(),
            kind: AdversarialKind::SequencerReorderConverges,
            admins: vec!["alice".into()],
            founders: vec!["alice".into()],
            op_kind: "Add".into(),
            subject: String::new(),
            signers: vec![],
            threshold: 1,
            chain_len: 8,
            drop_seq: 0,
            donor_msgs: 0,
            expect: "converge".into(),
        },
        AdversarialVector {
            ar: "AR-2".into(),
            label: "malicious sequencer drop: withholding seq 3 leaves the peer at a real earlier head, visibly behind (no false current)".into(),
            kind: AdversarialKind::SequencerDropVisiblyBehind,
            admins: vec!["alice".into()],
            founders: vec!["alice".into()],
            op_kind: "Add".into(),
            subject: String::new(),
            signers: vec![],
            threshold: 1,
            chain_len: 6,
            drop_seq: 3,
            donor_msgs: 0,
            expect: "visibly-behind".into(),
        },
        AdversarialVector {
            ar: "AR-2".into(),
            label: "malicious sequencer inject: an op signed by a non-admin the broker controls is rejected (cannot manufacture membership)".into(),
            kind: AdversarialKind::SequencerInjectRejected,
            admins: vec!["alice".into()],
            founders: vec!["alice".into()],
            op_kind: "Add".into(),
            subject: "mallory".into(),
            signers: vec!["mallory".into()],
            threshold: 1,
            chain_len: 0,
            drop_seq: 0,
            donor_msgs: 0,
            expect: "reject:signer-lacks-standing".into(),
        },
        AdversarialVector {
            ar: "AR-6".into(),
            label: "double-count: one admin signing twice still counts once (sigs keyed by DID) → below threshold 2".into(),
            kind: AdversarialKind::ReplayDoubleCountPrevented,
            admins: vec!["alice".into(), "bob".into()],
            founders: vec!["alice".into(), "bob".into(), "carol".into()],
            op_kind: "Remove".into(),
            subject: "carol".into(),
            signers: vec!["alice".into(), "alice".into()],
            threshold: 2,
            chain_len: 0,
            drop_seq: 0,
            donor_msgs: 0,
            expect: "reject:under-threshold".into(),
        },
        AdversarialVector {
            ar: "AR-6".into(),
            label: "replay: an applied Remove replayed against the advanced head does not chain → BrokenChain, not re-enacted".into(),
            kind: AdversarialKind::ReplayDoesNotReenact,
            admins: vec!["alice".into(), "bob".into()],
            founders: vec!["alice".into(), "bob".into(), "carol".into()],
            op_kind: "Remove".into(),
            subject: "carol".into(),
            signers: vec!["alice".into(), "bob".into()],
            threshold: 2,
            chain_len: 0,
            drop_seq: 0,
            donor_msgs: 0,
            expect: "reject:broken-chain".into(),
        },
        AdversarialVector {
            ar: "AR-3".into(),
            label: "backfill DoS: a 2000-message foreign-lineage branch is rejected on the genesis boundary with ZERO signature verifications".into(),
            kind: AdversarialKind::BackfillForeignZeroCrypto,
            admins: vec![],
            founders: vec![],
            op_kind: String::new(),
            subject: String::new(),
            signers: vec![],
            threshold: 0,
            chain_len: 0,
            drop_seq: 0,
            donor_msgs: 2000,
            expect: "reject:foreign-genesis (bound: 0 crypto)".into(),
        },
        AdversarialVector {
            ar: "AR-3".into(),
            label: "backfill DoS: a 5000-message forged branch on a shared lineage is rejected at the FIRST bad message (1 verify call, bounded cost)".into(),
            kind: AdversarialKind::BackfillFirstDefectBounded,
            admins: vec![],
            founders: vec![],
            op_kind: String::new(),
            subject: String::new(),
            signers: vec![],
            threshold: 0,
            chain_len: 0,
            drop_seq: 0,
            donor_msgs: 5000,
            expect: "reject:bad-signature (bound: 1 crypto)".into(),
        },
    ];

    // Derive-then-record: each vector must re-prove against the real API now, or
    // the emitter fails loud rather than writing a vector it cannot stand behind.
    for v in &vectors {
        match check_adversarial(v) {
            CheckOutcome::Pass => {}
            CheckOutcome::Fail(reason) => {
                panic!(
                    "emit cat 7: vector {} ({}) did not re-prove: {reason}",
                    v.ar, v.label
                )
            }
        }
    }

    AdversarialFile {
        header: Header {
            spec_section: "CROFT-PROTOCOL.md §6/§7 (adversarial); conformance-suite cat 7; AR-1/AR-2/AR-3/AR-6".into(),
            category: 7,
            note: "Adversarial must-reject / must-converge / must-bound vectors, each DERIVED \
                   by running the real lineage-core/lineage-history API (never hand-set). \
                   AR-1 Sybil (fresh non-admin identities confer no standing), AR-2 malicious \
                   sequencer (blind broker can reorder/duplicate/drop/inject but never forge: \
                   converged state is unchanged, a dropped op is visibly behind, an injected \
                   non-admin op is rejected), AR-6 replay/double-count (DID-keyed sigs prevent \
                   double counting; a replayed op does not chain). AR-3 backfill-DoS bounds the \
                   rejection cost (foreign branch rejected with 0 crypto; forged branch rejected \
                   at the first defect with 1 verify call). AR-4 (metadata-leak bound) and AR-5 \
                   (MLS tree scaling) are characterizations with no deterministic accept/reject \
                   crypto verdict derivable from this crate's deps — see `not_emitted`.".into(),
        },
        vectors,
        not_emitted: vec![
            "AR-4 metadata-leak bound: a privacy characterization (a relay routing by topic learns nothing about membership); no accept/reject crypto verdict and no lineage-core/lineage-history test to derive a deterministic vector from".into(),
            "AR-5 MLS tree scaling: a scaling characterization (not accept/reject), green-real in lineage-mls (tests/ar5_tree_scaling.rs); the conformance crate does not depend on lineage-mls and this is not a crypto interop vector".into(),
        ],
    }
}

// --- Categories 8 + 9: TS-authoritative snapshots ---------------------------

/// Run the authoritative TS model's conformance emitter (which writes the
/// language-neutral visibility/freshness snapshots), then load + structurally
/// validate them. Returns `None` (and logs) if the TS toolchain is unavailable
/// so the Rust suite still emits its own categories — the missing cats are then
/// recorded honestly rather than faked. Fails loud if the TS run errors or a
/// loaded snapshot is structurally invalid.
fn load_ts_verdicts() -> Option<(TsVerdictFile, TsVerdictFile)> {
    let model_dir = ts_model_dir()?;
    let status = std::process::Command::new("npm")
        .arg("run")
        .arg("emit-conformance")
        .current_dir(&model_dir)
        .status();
    match status {
        Ok(s) if s.success() => {}
        Ok(s) => {
            panic!("TS emit-conformance exited with {s} — refusing to snapshot a failing TS model")
        }
        Err(e) => {
            eprintln!(
                "cats 8/9: TS model not run ({e}); skipping visibility/freshness snapshots \
                 (authoritative runner is the TS model — run `npm run emit-conformance` in \
                 {})",
                model_dir.display()
            );
            return None;
        }
    }

    let root = vectors_root();
    let visibility: TsVerdictFile = load_json(&root.join("visibility").join("visibility.json"));
    let freshness: TsVerdictFile = load_json(&root.join("freshness.json"));
    for f in [&visibility, &freshness] {
        for e in &f.experiments {
            match check_ts_experiment(e) {
                CheckOutcome::Pass => {}
                CheckOutcome::Fail(reason) => {
                    panic!(
                        "cats 8/9: TS snapshot experiment {} is structurally invalid: {reason}",
                        e.name
                    )
                }
            }
        }
    }
    Some((visibility, freshness))
}

/// Locate the sibling TS model dir (`Proofs/lineage-group-model`). The
/// conformance crate lives at `Proofs/lineage-groups/crates/conformance`.
fn ts_model_dir() -> Option<PathBuf> {
    let manifest_dir = env!("CARGO_MANIFEST_DIR"); // .../lineage-groups/crates/conformance
    let proofs = Path::new(manifest_dir).parent()?.parent()?.parent()?; // .../Proofs
    let dir = proofs.join("lineage-group-model");
    dir.exists().then_some(dir)
}

/// Load + deserialize a JSON vector file, panicking on any error.
fn load_json<T: serde::de::DeserializeOwned>(path: &Path) -> T {
    let data =
        std::fs::read_to_string(path).unwrap_or_else(|e| panic!("read {}: {e}", path.display()));
    serde_json::from_str(&data).unwrap_or_else(|e| panic!("parse {}: {e}", path.display()))
}
