//! `seal-peer` — the NATIVE build of the same seal stack, as a long-lived
//! ndjson peer. One request per line on stdin, one JSON response per line on
//! stdout. Holds named `group_seal::Sealer`s; all MLS artifacts cross as hex.
//! Driven by `node/interop.mjs` (P2 goldens) beside the wasm build.

use std::collections::HashMap;
use std::io::{self, BufRead as _, Write as _};

use group_seal::Sealer;
use serde_json::{json, Value};

fn main() {
    let stdin = io::stdin();
    let stdout = io::stdout();
    let mut sealers: HashMap<String, Sealer> = HashMap::new();

    for line in stdin.lock().lines() {
        let Ok(line) = line else { break };
        if line.trim().is_empty() {
            continue;
        }
        let response = match handle(&mut sealers, &line) {
            Ok(v) => v,
            Err(e) => json!({ "ok": false, "error": e }),
        };
        let mut out = stdout.lock();
        if writeln!(out, "{response}").and_then(|()| out.flush()).is_err() {
            break;
        }
    }
}

/// Dispatch one request. Errors are strings so every failure is a JSON
/// response, never a peer crash (the driver asserts on some failures —
/// e.g. the removed member's open MUST fail).
fn handle(sealers: &mut HashMap<String, Sealer>, line: &str) -> Result<Value, String> {
    let req: Value = serde_json::from_str(line).map_err(|e| e.to_string())?;
    let op = str_field(&req, "op")?;

    match op {
        "found" => {
            let (name, did) = (str_field(&req, "name")?, str_field(&req, "did")?);
            let sealer = Sealer::found(did).map_err(|e| e.to_string())?;
            sealers.insert(name.to_string(), sealer);
            Ok(json!({ "ok": true }))
        }
        "enroll" => {
            let (name, did) = (str_field(&req, "name")?, str_field(&req, "did")?);
            let sealer = Sealer::enroll(did).map_err(|e| e.to_string())?;
            sealers.insert(name.to_string(), sealer);
            Ok(json!({ "ok": true }))
        }
        "key_package" => {
            let kp = sealer(sealers, &req)?.key_package().map_err(|e| e.to_string())?;
            let bytes = seal_wire::key_package_to_bytes(&kp).map_err(|e| e.to_string())?;
            Ok(json!({ "ok": true, "data": hex::encode(bytes) }))
        }
        "invite" => {
            let kp_bytes = hex_field(&req, "kp")?;
            let kp = seal_wire::key_package_from_bytes(&kp_bytes).map_err(|e| e.to_string())?;
            let (commit, welcome) = sealer(sealers, &req)?
                .invite(&[kp])
                .map_err(|e| e.to_string())?;
            Ok(json!({ "ok": true, "commit": hex::encode(commit), "welcome": hex::encode(welcome) }))
        }
        "accept_welcome" => {
            let welcome = hex_field(&req, "welcome")?;
            sealer(sealers, &req)?
                .accept_welcome(&welcome)
                .map_err(|e| e.to_string())?;
            Ok(json!({ "ok": true }))
        }
        "seal" => {
            let m = group_core::ChatMessage {
                sender: str_field(&req, "sender")?.to_string(),
                text: str_field(&req, "text")?.to_string(),
            };
            let sealed = sealer(sealers, &req)?.seal(&m).map_err(|e| e.to_string())?;
            Ok(json!({ "ok": true, "data": hex::encode(sealed) }))
        }
        "open" => {
            let sealed = hex_field(&req, "data")?;
            let m = sealer(sealers, &req)?.open(&sealed).map_err(|e| e.to_string())?;
            Ok(json!({ "ok": true, "sender": m.sender, "text": m.text }))
        }
        "apply_control" => {
            let control = hex_field(&req, "data")?;
            sealer(sealers, &req)?
                .apply_control(&control)
                .map_err(|e| e.to_string())?;
            Ok(json!({ "ok": true }))
        }
        "remove_member" => {
            let did = str_field(&req, "did")?.to_string();
            let commit = sealer(sealers, &req)?
                .remove_member(&did)
                .map_err(|e| e.to_string())?;
            Ok(json!({ "ok": true, "commit": hex::encode(commit) }))
        }
        "epoch" => {
            let epoch = sealer(sealers, &req)?.epoch().map_err(|e| e.to_string())?;
            Ok(json!({ "ok": true, "epoch": epoch }))
        }
        "epoch_secret" => {
            // TEST-ONLY exposure for the cross-build state comparison.
            let secret = sealer(sealers, &req)?
                .epoch_secret()
                .map_err(|e| e.to_string())?;
            Ok(json!({ "ok": true, "data": hex::encode(secret.as_bytes()) }))
        }
        other => Err(format!("unknown op: {other}")),
    }
}

fn str_field<'a>(req: &'a Value, key: &str) -> Result<&'a str, String> {
    req.get(key)
        .and_then(Value::as_str)
        .ok_or_else(|| format!("missing field: {key}"))
}

fn hex_field(req: &Value, key: &str) -> Result<Vec<u8>, String> {
    hex::decode(str_field(req, key)?).map_err(|e| e.to_string())
}

fn sealer<'a>(
    sealers: &'a mut HashMap<String, Sealer>,
    req: &Value,
) -> Result<&'a mut Sealer, String> {
    let name = str_field(req, "name")?;
    sealers
        .get_mut(name)
        .ok_or_else(|| format!("no such sealer: {name}"))
}
