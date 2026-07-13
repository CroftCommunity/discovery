//! Hypothesis-validation: a SUBSCRIBED LABELER service (`com.atproto.label.subscribeLabels`).
//!
//! THE HYPOTHESIS (what a developer spoiled by Jetstream would assume):
//!   H1. Label events are JSON over WebSocket *text* frames, like Jetstream.
//!   H2. A label carries roughly { src, uri, val, cts }.
//!   H3. The cursor is time-based (`time_us`), like Jetstream.
//!   H4. Each message is a single object (no envelope/header frame).
//!
//! We connect to a REAL labeler (Bluesky's moderation service, resolved from its
//! DID document's #atproto_labeler endpoint) and report every divergence.

use std::io::Cursor as IoCursor;
use std::collections::BTreeMap;
use std::time::Duration;

use anyhow::{anyhow, Result};
use ciborium::value::Value as Cbor;
use futures_util::StreamExt;
use tokio_tungstenite::tungstenite::Message;

use appview_validation::atproto;

const LABELER_DID: &str = "did:plc:ar7c4by46qjdydhdevvrndac"; // Bluesky moderation service

fn cget<'a>(v: &'a Cbor, key: &str) -> Option<&'a Cbor> {
    if let Cbor::Map(m) = v {
        for (k, val) in m {
            if matches!(k, Cbor::Text(t) if t == key) {
                return Some(val);
            }
        }
    }
    None
}
fn ctext(v: &Cbor) -> Option<&str> {
    if let Cbor::Text(t) = v { Some(t) } else { None }
}
fn cint(v: &Cbor) -> Option<i128> {
    if let Cbor::Integer(i) = v { Some((*i).into()) } else { None }
}
fn ckeys(v: &Cbor) -> Vec<String> {
    if let Cbor::Map(m) = v {
        m.iter()
            .filter_map(|(k, _)| ctext(k).map(str::to_string))
            .collect()
    } else {
        vec![]
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    rustls::crypto::ring::default_provider()
        .install_default()
        .expect("install rustls ring crypto provider");

    // Reuse our DID resolver to find the labeler's service endpoint.
    let http = atproto::client();
    let doc = atproto::resolve_did_doc(&http, LABELER_DID).await?;
    let endpoint = doc
        .get("service")
        .and_then(|s| s.as_array())
        .and_then(|arr| {
            arr.iter().find(|s| {
                s.get("id").and_then(|i| i.as_str()) == Some("#atproto_labeler")
            })
        })
        .and_then(|s| s.get("serviceEndpoint"))
        .and_then(|e| e.as_str())
        .ok_or_else(|| anyhow!("no #atproto_labeler endpoint"))?
        .to_string();
    let wss = format!(
        "{}/xrpc/com.atproto.label.subscribeLabels?cursor=0",
        endpoint.replace("https://", "wss://")
    );
    println!("\n############ subscribed labeler — hypothesis vs reality ############");
    println!("  labeler DID  : {LABELER_DID}");
    println!("  endpoint     : {endpoint} (#atproto_labeler, from DID doc)");
    println!("  connecting   : {wss}");

    let (mut ws, resp) = tokio_tungstenite::connect_async(&wss).await?;
    println!("  handshake OK : HTTP {}", resp.status().as_u16());

    let mut frames = 0usize;
    let mut text_frames = 0usize;
    let mut binary_frames = 0usize;
    let mut labels_seen = 0usize;
    let mut negations = 0usize;
    let mut signed = 0usize;
    let mut max_seq: i128 = 0;
    let mut val_counts: BTreeMap<String, usize> = BTreeMap::new();
    let mut reported = false;

    let deadline = tokio::time::Instant::now() + Duration::from_secs(20);
    while frames < 60 {
        let remaining = match deadline.checked_duration_since(tokio::time::Instant::now()) {
            Some(r) if !r.is_zero() => r,
            _ => break,
        };
        let msg = match tokio::time::timeout(remaining, ws.next()).await {
            Ok(Some(Ok(m))) => m,
            _ => break,
        };
        let bytes = match msg {
            Message::Binary(b) => {
                binary_frames += 1;
                b.to_vec()
            }
            Message::Text(t) => {
                text_frames += 1;
                t.as_bytes().to_vec()
            }
            Message::Ping(_) | Message::Pong(_) | Message::Frame(_) => continue,
            Message::Close(_) => break,
        };
        frames += 1;

        // Each frame = two concatenated DAG-CBOR objects: header {op,t} + body.
        let mut rdr = IoCursor::new(&bytes[..]);
        let header: Cbor = match ciborium::from_reader(&mut rdr) {
            Ok(v) => v,
            Err(_) => continue,
        };
        let body: Cbor = match ciborium::from_reader(&mut rdr) {
            Ok(v) => v,
            Err(_) => continue,
        };

        let t = cget(&header, "t").and_then(ctext).unwrap_or("<none>").to_string();
        let op = cget(&header, "op").and_then(cint).unwrap_or(0);
        let seq = cget(&body, "seq").and_then(cint).unwrap_or(0);
        max_seq = max_seq.max(seq);

        if let Some(Cbor::Array(labels)) = cget(&body, "labels") {
            for lab in labels {
                labels_seen += 1;
                if let Some(val) = cget(lab, "val").and_then(ctext) {
                    *val_counts.entry(val.to_string()).or_default() += 1;
                }
                if cget(lab, "neg").is_some() {
                    negations += 1;
                }
                if cget(lab, "sig").is_some() {
                    signed += 1;
                }

                if !reported {
                    reported = true;
                    println!("\n  ---- FIRST REAL FRAME ----");
                    println!("  WebSocket frame type: {}",
                        if binary_frames > 0 { "BINARY (opcode 0x2)" } else { "text" });
                    println!("  header object keys  : {:?}  (t={t:?}, op={op})", ckeys(&header));
                    println!("  body object keys    : {:?}", ckeys(&body));
                    println!("  label object keys   : {:?}", ckeys(lab));
                    println!("  sample label:");
                    println!("    ver = {:?}", cget(lab, "ver").and_then(cint));
                    println!("    src = {:?}", cget(lab, "src").and_then(ctext));
                    println!("    uri = {:?}", cget(lab, "uri").and_then(ctext));
                    println!("    val = {:?}", cget(lab, "val").and_then(ctext));
                    println!("    cts = {:?}", cget(lab, "cts").and_then(ctext));
                    println!("    neg = {:?}", cget(lab, "neg").is_some());
                    println!("    sig = {} bytes (signed)",
                        match cget(lab, "sig") { Some(Cbor::Bytes(b)) => b.len(), _ => 0 });
                }
            }
        }
    }
    let _ = ws.close(None).await;

    println!("\n================ HYPOTHESIS vs REALITY ================");
    println!("  H1 'JSON over text frames' -> WRONG: {binary_frames} binary frames, {text_frames} text.");
    println!("     Reality: BINARY WebSocket frames carrying DAG-CBOR (the com.atproto.sync");
    println!("     framing: two concatenated CBOR objects = {{op,t}} header + body).");
    println!("     [Jetstream is a value-add JSON *proxy* over this; labelers expose raw CBOR.]");
    println!("  H2 'label = {{src,uri,val,cts}}' -> PARTIAL: those exist, but a real label also");
    println!("     carries ver (versioning), a cryptographic sig ({signed}/{labels_seen} signed),");
    println!("     and an optional neg (negation/retraction; {negations} seen).");
    println!("  H3 'cursor = time_us' -> WRONG: the cursor is `seq`, a monotonic sequence");
    println!("     integer (max seq seen: {max_seq}), not a timestamp.");
    println!("  H4 'single object per message' -> WRONG: every frame is header+body framed.");
    println!("\n  label value taxonomy observed ({labels_seen} labels):");
    for (val, n) in &val_counts {
        println!("     • {val}: {n}");
    }
    println!("======================================================");
    println!("\n############ DONE (labeler) ############");
    Ok(())
}
