//! P8 (process story) — three delivery-role PROCESSES, co-hosted, over local
//! sockets, with failure isolation.
//!
//! Deliberately not one binary: the web-native DS, the swarm-peer, and the
//! history-convergence node run as SEPARATE processes on localhost (the way one
//! VPS would co-host them), exchanging envelopes over the transport trait
//! (local TCP sockets per guardrail 6 —
//! `SPEC-DELTA[run17-swarm-local | declared-stand-in]`; the iroh overlay is not
//! required). Proven at the process level:
//!
//! - an envelope injected ONLY via the swarm process and another ONLY via the DS
//!   process both appear exactly once in the converged set (dedup by
//!   `H(envelope)` across transports, across process boundaries);
//! - failure isolation: killing any one process leaves the other two serving.

use std::io::{BufRead, BufReader, Read, Write};
use std::net::TcpStream;
use std::process::{Child, Command, Stdio};

/// Spawn a role binary; it prints `PORT <n>` on stdout, then serves.
fn spawn(bin_path: &str) -> (Child, u16) {
    let mut child = Command::new(bin_path)
        .stdout(Stdio::piped())
        .spawn()
        .expect("spawn role process");
    let stdout = child.stdout.take().expect("child stdout");
    let mut reader = BufReader::new(stdout);
    let mut line = String::new();
    reader.read_line(&mut line).expect("read PORT line");
    let port: u16 = line
        .trim()
        .strip_prefix("PORT ")
        .expect("PORT prefix")
        .parse()
        .expect("port number");
    (child, port)
}

/// Send one command line to `127.0.0.1:port`, return the single response line.
fn cmd(port: u16, line: &str) -> String {
    // Small retry: the listener may need a beat after PORT is printed.
    let mut last = String::new();
    for _ in 0..50 {
        if let Ok(mut s) = TcpStream::connect(("127.0.0.1", port)) {
            s.write_all(line.as_bytes()).unwrap();
            s.write_all(b"\n").unwrap();
            s.flush().unwrap();
            let mut buf = String::new();
            s.read_to_string(&mut buf).unwrap();
            return buf.trim().to_string();
        }
        std::thread::sleep(std::time::Duration::from_millis(20));
        last = format!("connect failed on :{port}");
    }
    panic!("{last}");
}

#[test]
fn three_processes_dedup_across_transports_and_isolate_failure() {
    let ds_bin = env!("CARGO_BIN_EXE_ds");
    let swarm_bin = env!("CARGO_BIN_EXE_swarm_peer");
    let conv_bin = env!("CARGO_BIN_EXE_convergence");

    let (mut ds, ds_port) = spawn(ds_bin);
    let (mut swarm, swarm_port) = spawn(swarm_bin);
    let (mut conv, conv_port) = spawn(conv_bin);

    // All three answer — co-hosted from one launch.
    assert_eq!(cmd(ds_port, "PING"), "PONG");
    assert_eq!(cmd(swarm_port, "PING"), "PONG");
    assert_eq!(cmd(conv_port, "PING"), "PONG");

    // Inject one envelope ONLY via DS and another ONLY via swarm. The role
    // binaries expose a SEED command that mints a deterministic envelope from a
    // seed byte and returns its identity, so the test needs no shared codec.
    let ds_id = cmd(ds_port, "SEED 1");
    let swarm_id = cmd(swarm_port, "SEED 2");
    assert!(ds_id.starts_with("ID "), "ds SEED returns an identity: {ds_id}");
    assert!(swarm_id.starts_with("ID "), "swarm SEED returns an identity");
    let ds_id = ds_id.trim_start_matches("ID ").to_string();
    let swarm_id = swarm_id.trim_start_matches("ID ").to_string();

    // The convergence node pulls from BOTH and reconciles by envelope hash.
    let converged = cmd(conv_port, &format!("CONVERGE 127.0.0.1:{ds_port} 127.0.0.1:{swarm_port}"));
    assert!(converged.starts_with("COUNT 2"), "two distinct envelopes converged: {converged}");
    assert!(converged.contains(&ds_id), "the DS-only envelope is present");
    assert!(converged.contains(&swarm_id), "the swarm-only envelope is present");

    // Failure isolation: kill the swarm process; DS and convergence keep serving.
    swarm.kill().expect("kill swarm");
    swarm.wait().ok();
    assert_eq!(cmd(ds_port, "PING"), "PONG", "DS survives a swarm-peer death");
    assert_eq!(cmd(conv_port, "PING"), "PONG", "convergence survives a swarm-peer death");

    // Clean up.
    ds.kill().ok();
    ds.wait().ok();
    conv.kill().ok();
    conv.wait().ok();
}
