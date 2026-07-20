//! One-command show-and-tell (EXP-LEX-03): an organizer signs an attendance
//! attestation over a REAL public RSVP record, and the clean-room verifier
//! confirms it.
//!
//!   cargo run --example demo_attendance

use lexicon_community::attest::{build_inline, verify_record, Resolver, VerifyOpts};
use lexicon_community::didkey::Curve;
use lexicon_community::sign::SignKey;

struct DemoResolver {
    issuer: String,
    key: String,
}
impl Resolver for DemoResolver {
    fn get_record(&self, uri: &str) -> Result<serde_json::Value, String> {
        Err(format!("no network in the demo (uri {uri})"))
    }
    fn authorized_keys(&self, did: &str) -> Vec<String> {
        if did == self.issuer {
            vec![self.key.clone()]
        } else {
            vec![]
        }
    }
}

fn main() {
    let manifest = env!("CARGO_MANIFEST_DIR");
    let rsvp: serde_json::Value = serde_json::from_str(
        &std::fs::read_to_string(format!("{manifest}/fixtures/recorded/rsvp.getRecord.json")).unwrap(),
    )
    .unwrap();
    let record = rsvp["value"].clone();
    let repo = "did:plc:cbkjy5n7bk3ax2wplmtjofq2"; // the RSVP author's real repo

    println!("Real RSVP record (from pds.cauda.cloud):");
    println!("{}\n", serde_json::to_string_pretty(&record).unwrap());

    let organizer = SignKey::from_seed(Curve::P256, b"RUN-LEX-01/organizer");
    let issuer = "did:plc:organizer";
    let meta = serde_json::json!({
        "$type": "community.lexicon.attest.attendance",
        "issuer": issuer,
        "purpose": "organizer-confirmed-attendance"
    });
    let signed = build_inline(&record, &organizer, &meta, repo).unwrap();

    println!("Organizer signing key: {}", organizer.did_key());
    println!("\nAttested record (CID-first inline attestation appended):");
    println!("{}\n", serde_json::to_string_pretty(&signed["signatures"]).unwrap());

    let resolver = DemoResolver { issuer: issuer.to_string(), key: organizer.did_key() };
    match verify_record(&signed, repo, &resolver, &VerifyOpts::default()) {
        Ok(v) => println!("VERIFIED ({} attestation): {:?}", v.len(), v),
        Err(e) => {
            eprintln!("verification FAILED: {e:?}");
            std::process::exit(1);
        }
    }
    println!("\nStrict posture: signature valid AND the organizer key is authorized by the issuer DID doc.");
    println!("F-AT-6 disclosure: anything published on ATProto inherits PLC op-log / hosting correlators.");
}
