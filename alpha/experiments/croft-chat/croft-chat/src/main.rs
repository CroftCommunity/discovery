//! Binary entry point.
//!
//! Two modes:
//! - **interactive** (default): the crossterm TUI event loop.
//! - **headless** (`exec <op>`): one scripted operation against the store, used
//!   by the binary-smoke test and by the P18 two-node run recipe over SSH.
//!
//! Args: `--store <path>` (redb file), `--seed <u8>` (deterministic identity),
//! optional `exec <op> [args…]`.

use std::io;
use std::path::Path;
use std::process::ExitCode;
use std::time::Duration;

use ratatui::backend::CrosstermBackend;
use ratatui::crossterm::event::{self, Event, KeyEventKind};
use ratatui::crossterm::{execute, terminal};
use ratatui::Terminal;

use croft_chat::app::App;
use croft_chat::input::map_key;
use croft_chat::ui;
use social_graph_core::{GroupId, Identity, Session, TimelineWindow};

/// Parsed command line.
struct Args {
    store: String,
    seed: u8,
    exec: Vec<String>,
}

fn parse_args() -> Args {
    let mut store = std::env::var("CROFT_CHAT_STORE").unwrap_or_else(|_| "croft-chat.redb".to_string());
    let mut seed: u8 = std::env::var("CROFT_CHAT_SEED")
        .ok()
        .and_then(|s| s.parse().ok())
        .unwrap_or(1);
    let mut exec = Vec::new();

    let mut args = std::env::args().skip(1);
    while let Some(arg) = args.next() {
        match arg.as_str() {
            "--store" => {
                if let Some(v) = args.next() {
                    store = v;
                }
            }
            "--seed" => {
                if let Some(v) = args.next().and_then(|s| s.parse().ok()) {
                    seed = v;
                }
            }
            "exec" => {
                exec.extend(args.by_ref());
            }
            _ => {}
        }
    }
    Args { store, seed, exec }
}

/// Cross-host `serve` arguments (parsed regardless of feature; only run under
/// `iroh-it`).
struct ServeArgs {
    topology: String,
    node: String,
    addr_out: String,
    peers: Vec<String>,
    create: bool,
    send: Option<String>,
    run_seconds: u64,
    /// A4/M1 fan-out: the timeline length at which this node counts as converged.
    /// When >0, the serve loop records the elapsed time to first reach it (with
    /// pending == 0) and prints it as `converged_ms`. 0 disables the check.
    expect_msgs: usize,
}

fn parse_serve() -> Option<ServeArgs> {
    let mut args = std::env::args().skip(1);
    // `serve` may appear after the global flags; scan for it.
    let found = args.by_ref().any(|a| a == "serve");
    if !found {
        return None;
    }
    let mut serve = ServeArgs {
        topology: "stone-alpha.toml".to_string(),
        node: String::new(),
        addr_out: "self-addr.json".to_string(),
        peers: Vec::new(),
        create: false,
        send: None,
        run_seconds: 60,
        expect_msgs: 0,
    };
    while let Some(arg) = args.next() {
        match arg.as_str() {
            "--topology" => serve.topology = args.next().unwrap_or(serve.topology),
            "--node" => serve.node = args.next().unwrap_or_default(),
            "--addr-out" => serve.addr_out = args.next().unwrap_or(serve.addr_out),
            "--peer" => {
                if let Some(p) = args.next() {
                    serve.peers.push(p);
                }
            }
            "--create" => serve.create = true,
            "--send" => serve.send = args.next(),
            "--run-seconds" => {
                serve.run_seconds = args.next().and_then(|s| s.parse().ok()).unwrap_or(60);
            }
            "--expect-msgs" => {
                serve.expect_msgs = args.next().and_then(|s| s.parse().ok()).unwrap_or(0);
            }
            _ => {}
        }
    }
    Some(serve)
}

#[tokio::main]
async fn main() -> ExitCode {
    croft_chat::init_tracing();
    let args = parse_args();

    if let Some(serve) = parse_serve() {
        return run_serve_dispatch(&args.store, serve).await;
    }

    let identity = Identity::from_seed([args.seed; 32]);

    let session = match Session::open(Path::new(&args.store), &identity) {
        Ok(s) => s,
        Err(e) => {
            eprintln!("failed to open store {}: {e}", args.store);
            return ExitCode::FAILURE;
        }
    };

    if args.exec.is_empty() {
        match run_tui(session).await {
            Ok(()) => ExitCode::SUCCESS,
            Err(e) => {
                eprintln!("tui error: {e}");
                ExitCode::FAILURE
            }
        }
    } else {
        run_exec(&session, &args.exec).await
    }
}

/// Headless operations. Prints results to stdout for scripting.
async fn run_exec(session: &Session, op: &[String]) -> ExitCode {
    match op.first().map(String::as_str) {
        Some("create-group") => match session.create_group().await {
            Ok(group) => {
                println!("{}", hex(group.as_bytes()));
                ExitCode::SUCCESS
            }
            Err(e) => fail(&format!("create-group: {e}")),
        },
        Some("send") => {
            let (Some(gid_hex), Some(body)) = (op.get(1), op.get(2)) else {
                return fail("usage: exec send <group_hex> <body>");
            };
            let Some(group) = parse_group(gid_hex) else {
                return fail("invalid group hex");
            };
            match session.send_message(&group, body, None).await {
                Ok(hash) => {
                    println!("{}", hex(hash.as_bytes()));
                    ExitCode::SUCCESS
                }
                Err(e) => fail(&format!("send: {e}")),
            }
        }
        Some("list") => match session.list_my_groups() {
            Ok(groups) => {
                for g in groups {
                    println!("{}", hex(g.group_id.as_bytes()));
                }
                ExitCode::SUCCESS
            }
            Err(e) => fail(&format!("list: {e}")),
        },
        Some("timeline") => {
            let Some(group) = op.get(1).and_then(|h| parse_group(h)) else {
                return fail("usage: exec timeline <group_hex>");
            };
            match session.get_timeline(&group, TimelineWindow::LastN(usize::MAX)) {
                Ok(timeline) => {
                    for entry in &timeline.entries {
                        if let Some(m) = session.get_message(&entry.hash) {
                            println!("{}: {}", m.lamport, m.body);
                        }
                    }
                    ExitCode::SUCCESS
                }
                Err(e) => fail(&format!("timeline: {e}")),
            }
        }
        other => fail(&format!("unknown exec op: {other:?}")),
    }
}

fn fail(msg: &str) -> ExitCode {
    eprintln!("{msg}");
    ExitCode::FAILURE
}

fn hex(bytes: &[u8; 32]) -> String {
    let mut s = String::with_capacity(64);
    for b in bytes {
        s.push_str(&format!("{b:02x}"));
    }
    s
}

fn parse_group(hex: &str) -> Option<GroupId> {
    if hex.len() != 64 {
        return None;
    }
    let mut bytes = [0u8; 32];
    for (i, byte) in bytes.iter_mut().enumerate() {
        *byte = u8::from_str_radix(hex.get(i * 2..i * 2 + 2)?, 16).ok()?;
    }
    Some(GroupId::new(bytes))
}

// --- cross-host serve (iroh) ---------------------------------------------

#[cfg(not(feature = "iroh-it"))]
async fn run_serve_dispatch(_store: &str, _serve: ServeArgs) -> ExitCode {
    eprintln!("`serve` requires building with --features iroh-it");
    ExitCode::FAILURE
}

#[cfg(feature = "iroh-it")]
async fn run_serve_dispatch(store: &str, serve: ServeArgs) -> ExitCode {
    use std::time::Duration;

    use croft_chat::config::Topology;
    use croft_chat::fingerprint::fingerprint;
    use croft_chat::iroh_bus::{IrohGossipBus, RelayChoice};
    use croft_chat::sync::Replicator;
    use croft_chat::transport::Topic;
    use social_graph_core::Role;

    let topology = match Topology::load(std::path::Path::new(&serve.topology)) {
        Ok(t) => t,
        Err(e) => return fail(&format!("topology: {e}")),
    };
    let node = match topology.node(&serve.node) {
        Ok(n) => n.clone(),
        Err(e) => return fail(&format!("{e}")),
    };

    // This node's identity comes from the topology seed.
    let identity = node.identity();
    let session = match Session::open(std::path::Path::new(store), &identity) {
        Ok(s) => s,
        Err(e) => return fail(&format!("open store: {e}")),
    };
    // Trust every other node's credential so their assertions verify.
    for other in &topology.nodes {
        if other.name != node.name {
            let id = other.identity();
            session.trust_peer(id.device_id(), id.principal_id());
        }
    }

    // Bootstrap peer addresses from the supplied JSON files.
    let mut bootstrap = Vec::new();
    for path in &serve.peers {
        match std::fs::read_to_string(path) {
            Ok(s) => match serde_json::from_str(&s) {
                Ok(addr) => bootstrap.push(addr),
                Err(e) => return fail(&format!("parse peer {path}: {e}")),
            },
            Err(e) => return fail(&format!("read peer {path}: {e}")),
        }
    }

    let topic = Topic("drystone/stone-alpha".to_string());
    let relay = RelayChoice::from_relay_mode(&topology.relay_mode);
    let mut bus = match IrohGossipBus::connect(&topic, bootstrap, relay).await {
        Ok(b) => b,
        Err(e) => return fail(&format!("iroh connect: {e}")),
    };
    // Publish our address for peers to bootstrap from.
    match serde_json::to_string(&bus.endpoint_addr()) {
        Ok(json) => {
            if let Err(e) = std::fs::write(&serve.addr_out, json) {
                return fail(&format!("write {}: {e}", serve.addr_out));
            }
            println!("addr written to {}", serve.addr_out);
        }
        Err(e) => return fail(&format!("serialize addr: {e}")),
    }

    let mut replicator = Replicator::new();

    // The creator establishes the group and enrolls every other node.
    let mut group = None;
    if serve.create {
        match session.create_group().await {
            Ok(g) => {
                for other in &topology.nodes {
                    if other.name != node.name {
                        if let Err(e) = session.add_member(&g, other.principal(), Role::Member).await
                        {
                            return fail(&format!("add_member {}: {e}", other.name));
                        }
                    }
                }
                println!("group {}", hex(g.as_bytes()));
                group = Some(g);
            }
            Err(e) => return fail(&format!("create_group: {e}")),
        }
    }

    // Run the replication loop for the requested duration.
    let mut sent = false;
    let ticks = serve.run_seconds * 4; // 250ms cadence
    // A4/M1 fan-out: time from loop start to the first tick at which this node has
    // folded the full expected timeline with nothing pending (its convergence latency).
    let start = std::time::Instant::now();
    // Two latencies, honestly distinguished: `head_at` is the first tick the full
    // N-message timeline is folded (the conversation has converged); `converged_at`
    // additionally requires pending == 0 (nothing buffered — fully settled).
    let mut head_at: Option<Duration> = None;
    let mut converged_at: Option<Duration> = None;
    for _ in 0..ticks {
        // Always drain + apply foreign frames first — this is how a joining node
        // learns the group (and its own membership) before it knows the group id.
        replicator.pump(&session, &mut bus);
        if group.is_none() {
            if let Ok(groups) = session.list_my_groups() {
                group = groups.first().map(|g| g.group_id);
            }
        }
        if let Some(g) = group {
            // Publish our state so peers converge, and send once we're a member.
            replicator.publish_group(&session, &mut bus, &topic, &g).ok();
            if !sent {
                if let Some(body) = &serve.send {
                    if session.send_message(&g, body, None).await.is_ok() {
                        sent = true;
                    }
                }
            }
            // Record convergence latencies: head (full timeline folded) and, more
            // strictly, fully-settled (also nothing pending).
            if serve.expect_msgs > 0 && converged_at.is_none() {
                if let Ok(tl) =
                    session.get_timeline(&g, social_graph_core::TimelineWindow::LastN(usize::MAX))
                {
                    if tl.entries.len() >= serve.expect_msgs {
                        if head_at.is_none() {
                            head_at = Some(start.elapsed());
                        }
                        if replicator.pending_len() == 0 {
                            converged_at = Some(start.elapsed());
                        }
                    }
                }
            }
        }
        tokio::time::sleep(Duration::from_millis(250)).await;
    }

    // Final convergence fingerprint for cross-node comparison. `pending` is the
    // honest incompleteness signal: 0 means fully folded, >0 means some received
    // facts are still held (a per-device gap or a governance antecedent not yet
    // here) — the node is catching up, not settled.
    let pending = replicator.pending_len();
    // A4/M1 fan-out measurement line: gossip message counts + convergence latency.
    let stats = bus.stats();
    let head_ms = head_at.map_or("NA".to_string(), |d| d.as_millis().to_string());
    let converged_ms = converged_at.map_or("NA".to_string(), |d| d.as_millis().to_string());
    println!(
        "metrics live_sent={} resync_sent={} received={} head_ms={head_ms} converged_ms={converged_ms}",
        stats.live_sent, stats.resync_sent, stats.received
    );
    if let Some(g) = group {
        println!(
            "fingerprint {} (pending {pending}{})",
            short_hash(&fingerprint(&session, &g)),
            if pending == 0 { ", settled" } else { ", catching up" }
        );
        println!("--- timeline ---");
        if let Ok(tl) =
            session.get_timeline(&g, social_graph_core::TimelineWindow::LastN(usize::MAX))
        {
            for entry in &tl.entries {
                if let Some(m) = session.get_message(&entry.hash) {
                    println!("{}: {}", m.lamport, m.body);
                }
            }
        }
    } else {
        eprintln!("no group converged in {}s", serve.run_seconds);
        return ExitCode::FAILURE;
    }
    ExitCode::SUCCESS
}

/// A short, comparable digest of the fingerprint string (FNV-1a, hex).
#[cfg(feature = "iroh-it")]
fn short_hash(s: &str) -> String {
    let mut h: u64 = 0xcbf2_9ce4_8422_2325;
    for b in s.as_bytes() {
        h ^= u64::from(*b);
        h = h.wrapping_mul(0x0000_0100_0000_01b3);
    }
    format!("{h:016x}")
}

// --- interactive TUI ------------------------------------------------------

async fn run_tui(session: Session) -> io::Result<()> {
    let mut app = App::new(session);
    app.refresh();
    let mut terminal = setup_terminal()?;
    let result = run_loop(&mut terminal, &mut app).await;
    restore_terminal(&mut terminal)?;
    result
}

fn setup_terminal() -> io::Result<Terminal<CrosstermBackend<io::Stdout>>> {
    terminal::enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, terminal::EnterAlternateScreen)?;
    Terminal::new(CrosstermBackend::new(stdout))
}

fn restore_terminal(terminal: &mut Terminal<CrosstermBackend<io::Stdout>>) -> io::Result<()> {
    terminal::disable_raw_mode()?;
    execute!(terminal.backend_mut(), terminal::LeaveAlternateScreen)?;
    Ok(())
}

async fn run_loop(
    terminal: &mut Terminal<CrosstermBackend<io::Stdout>>,
    app: &mut App,
) -> io::Result<()> {
    loop {
        terminal.draw(|frame| ui::draw(frame, &app.view(), app.focus()))?;
        if event::poll(Duration::from_millis(150))? {
            if let Event::Key(key) = event::read()? {
                if key.kind == KeyEventKind::Press {
                    if let Some(action) = map_key(key, app.focus()) {
                        if !app.perform(action).await {
                            break;
                        }
                    }
                }
            }
        }
    }
    Ok(())
}
