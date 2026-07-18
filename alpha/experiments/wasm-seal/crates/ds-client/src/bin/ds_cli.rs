//! `ds-cli` — the CLI over [`ds_client::DsClient`] that the P5 Node loop
//! driver shells out to for its QUIC legs (Node has no WebTransport API;
//! this native client speaks the browser-identical protocol — guardrail 4).
//!
//! Usage:
//!   ds-cli <url> <cert_hash_hex> roster-add    <group> <member>
//!   ds-cli <url> <cert_hash_hex> roster-remove <group> <member>
//!   ds-cli <url> <cert_hash_hex> put   <group> <seq> <member>   (blob: hex on stdin)
//!   ds-cli <url> <cert_hash_hex> fetch <group> <from> <member>  (stdout: JSON [{seq,data}])
//!
//! Exit codes: 0 ok · 2 refused (the DS's flat refusal) · 1 anything else.

use ds_client::{ClientError, DsClient};

fn die(msg: &str, code: i32) -> ! {
    eprintln!("ds-cli: {msg}");
    std::process::exit(code);
}

fn exit_for(e: &ClientError) -> ! {
    match e {
        ClientError::Refused(_) => die(&e.to_string(), 2),
        _ => die(&e.to_string(), 1),
    }
}

#[tokio::main]
async fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    if args.len() < 5 {
        die("usage: ds-cli <url> <hash> <op> <group> <args…>", 1);
    }
    let (url, hash, op, group) = (&args[0], &args[1], args[2].as_str(), &args[3]);
    let client = match DsClient::connect(url, hash).await {
        Ok(c) => c,
        Err(e) => exit_for(&e),
    };
    let result = match op {
        "roster-add" => client.roster_add(group, &args[4]).await,
        "roster-remove" => client.roster_remove(group, &args[4]).await,
        "put" => {
            let seq: u64 = args[4].parse().unwrap_or_else(|_| die("bad seq", 1));
            let member = args.get(5).unwrap_or_else(|| die("missing member", 1));
            let mut hex_in = String::new();
            use std::io::Read as _;
            std::io::stdin()
                .read_to_string(&mut hex_in)
                .unwrap_or_else(|e| die(&e.to_string(), 1));
            let blob =
                hex::decode(hex_in.trim()).unwrap_or_else(|e| die(&e.to_string(), 1));
            client.put(group, seq, member, &blob).await
        }
        "fetch" => {
            let from: u64 = args[4].parse().unwrap_or_else(|_| die("bad from_seq", 1));
            let member = args.get(5).unwrap_or_else(|| die("missing member", 1));
            match client.fetch(group, from, member).await {
                Ok(blobs) => {
                    let rows: Vec<serde_json::Value> = blobs
                        .iter()
                        .map(|(seq, data)| {
                            serde_json::json!({ "seq": seq, "data": hex::encode(data) })
                        })
                        .collect();
                    println!(
                        "{}",
                        serde_json::to_string(&rows).unwrap_or_else(|e| die(&e.to_string(), 1))
                    );
                    return;
                }
                Err(e) => exit_for(&e),
            }
        }
        other => die(&format!("unknown op {other}"), 1),
    };
    if let Err(e) = result {
        exit_for(&e);
    }
}
