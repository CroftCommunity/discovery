//! Capture resolved dependency versions and the rustc version at build time so
//! the program's version report reflects exactly what was linked, rather than
//! hardcoded literals.

use std::fs;
use std::path::Path;
use std::process::Command;

fn lock_version(lock: &str, crate_name: &str) -> String {
    // Cargo.lock is a sequence of `[[package]]` tables. Find the one whose
    // `name = "<crate_name>"` and return its following `version = "..."`.
    let needle = format!("name = \"{crate_name}\"");
    let mut lines = lock.lines();
    while let Some(line) = lines.next() {
        if line.trim() == needle {
            if let Some(v) = lines.next() {
                let v = v.trim();
                if let Some(rest) = v.strip_prefix("version = \"") {
                    if let Some(end) = rest.find('"') {
                        return rest[..end].to_string();
                    }
                }
            }
        }
    }
    "unknown".to_string()
}

fn main() {
    let manifest_dir = std::env::var("CARGO_MANIFEST_DIR").unwrap();
    let lock_path = Path::new(&manifest_dir).join("Cargo.lock");
    let lock = fs::read_to_string(&lock_path).unwrap_or_default();
    println!("cargo:rerun-if-changed=Cargo.lock");

    for (env_key, crate_name) in [
        ("SLICE_VER_AUTOMERGE", "automerge"),
        ("SLICE_VER_OPENMLS", "openmls"),
        ("SLICE_VER_CHACHA", "chacha20poly1305"),
        ("SLICE_VER_SERDE_JSON", "serde_json"),
        ("SLICE_VER_CID", "cid"),
        ("SLICE_VER_DAGCBOR", "serde_ipld_dagcbor"),
        ("SLICE_VER_MULTIHASH", "multihash-codetable"),
    ] {
        println!("cargo:rustc-env={env_key}={}", lock_version(&lock, crate_name));
    }

    let rustc = std::env::var("RUSTC").unwrap_or_else(|_| "rustc".to_string());
    let rustc_version = Command::new(rustc)
        .arg("--version")
        .output()
        .ok()
        .and_then(|o| String::from_utf8(o.stdout).ok())
        .map(|s| s.trim().to_string())
        .unwrap_or_else(|| "unknown".to_string());
    println!("cargo:rustc-env=SLICE_RUSTC_VERSION={rustc_version}");
}
