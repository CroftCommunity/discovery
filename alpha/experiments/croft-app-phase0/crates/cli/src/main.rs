//! The Phase 0 command-line shell. Minimal, obvious argument parsing; the
//! runtime loop and the renderer do the real work.
//!
//! Usage:
//!   pond [--fixtures <dir>] [--real] [--actor <handle>] <command>
//!
//! Commands:
//!   open    Open the feed and print it.
//!   more    Open the feed, request load-more, and print the appended result.
//!   empty   Open against the empty-timeline fixture (fake only).
//!   error   Drive the fake into error mode to exercise the error rendering
//!           (DECISION 5: the error path drives the fake, not a separate
//!           failure-injection mechanism). Fake only.

use app_core::{project, Intent, Model};
use bluesky::adapter::RealBluesky;
use bluesky::fake::{default_fixtures_dir, FakeBluesky, FakeMode};
use bluesky::port::BlueskyPort;
use pond_cli::{render, runtime};
use std::path::{Component, Path, PathBuf};
use std::process::ExitCode;

fn main() -> ExitCode {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let mut fixtures: PathBuf = default_fixtures_dir().to_path_buf();
    let mut fixtures_overridden = false;
    let mut real = false;
    let mut actor: Option<String> = None;
    let mut command: Option<String> = None;

    let mut it = args.into_iter();
    while let Some(arg) = it.next() {
        match arg.as_str() {
            "--fixtures" => match it.next() {
                Some(dir) => {
                    fixtures = PathBuf::from(dir);
                    fixtures_overridden = true;
                }
                None => return fail("--fixtures needs a directory"),
            },
            "--actor" => match it.next() {
                Some(handle) => actor = Some(handle),
                None => return fail("--actor needs a handle"),
            },
            "--real" => real = true,
            "-h" | "--help" => {
                print_help();
                return ExitCode::SUCCESS;
            }
            other if command.is_none() => command = Some(other.to_string()),
            other => return fail(&format!("unexpected argument: {other}")),
        }
    }

    // Guard the one piece of operator-supplied path input: reject `..` segments
    // so a stray `--fixtures` value can't walk out of the intended tree. The
    // built-in default is trusted and skips this.
    if fixtures_overridden {
        if let Err(e) = validate_fixtures(&fixtures) {
            return fail(&e);
        }
    }

    // `--actor` only means something for the live adapter; reject it on fake
    // runs rather than silently ignoring it.
    if actor.is_some() && !real {
        return fail("--actor requires --real");
    }

    let Some(command) = command else {
        print_help();
        return ExitCode::SUCCESS;
    };

    let run_load_more = match command.as_str() {
        "open" => false,
        "more" => true,
        "empty" | "error" if real => {
            return fail(&format!("`{command}` is a fake-only command; drop --real"));
        }
        "empty" | "error" => {
            // Handled in the fake branch below.
            return run_fake(&fixtures, &command);
        }
        other => return fail(&format!("unknown command: {other}")),
    };

    if real {
        // M6: the live adapter, behind the same port. Phase 0 reads a public
        // author feed (same shape as getTimeline); --actor picks whose.
        let mut port = RealBluesky::new();
        if let Some(actor) = actor {
            port = port.with_actor(actor);
        }
        run(&port, run_load_more);
    } else {
        let port = FakeBluesky::new(&fixtures).with_mode(FakeMode::Normal);
        run(&port, run_load_more);
    }

    ExitCode::SUCCESS
}

/// Drive the core through the shell loop and print each settled view. Generic
/// over the port, so fake and real go through exactly the same path.
fn run<P: BlueskyPort>(port: &P, run_load_more: bool) {
    let mut model = runtime::drive(Model::new(), Intent::OpenFeed, port);
    println!("{}", render::render(&project(&model)));

    if run_load_more {
        println!("\n— (load more) —\n");
        model = runtime::drive(model, Intent::FeedReachedEnd, port);
        println!("{}", render::render(&project(&model)));
    }
}

/// The fake-only `empty` / `error` commands.
fn run_fake(fixtures: &Path, command: &str) -> ExitCode {
    let mode = match command {
        "empty" => FakeMode::Empty,
        "error" => FakeMode::Error,
        _ => unreachable!("only empty/error reach here"),
    };
    let port = FakeBluesky::new(fixtures).with_mode(mode);
    run(&port, false);
    ExitCode::SUCCESS
}

/// Reject operator-supplied fixture paths that try to traverse upward.
fn validate_fixtures(path: &Path) -> Result<(), String> {
    if path.components().any(|c| matches!(c, Component::ParentDir)) {
        return Err("--fixtures must not contain '..' path segments".to_string());
    }
    Ok(())
}

fn print_help() {
    println!(
        "pond — Phase 0 CLI shell\n\n\
         usage: pond [--fixtures <dir>] [--real] [--actor <handle>] <open|more|empty|error>\n\n\
         commands:\n\
         \topen    open the feed and print it\n\
         \tmore    open, then load more, and print the appended result\n\
         \tempty   open against the empty-timeline fixture (fake only)\n\
         \terror   drive the fake into error mode and render the error view (fake only)\n\n\
         flags:\n\
         \t--real            fetch a live feed instead of the fixtures\n\
         \t--actor <handle>  whose feed to read with --real (default: bsky.app)\n\
         \t--fixtures <dir>  directory of recorded fixtures (fake)"
    );
}

fn fail(msg: &str) -> ExitCode {
    eprintln!("error: {msg}");
    ExitCode::FAILURE
}
