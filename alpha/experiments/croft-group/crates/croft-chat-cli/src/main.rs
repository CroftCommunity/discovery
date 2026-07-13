//! The `croft-chat` command-line shell. Minimal, obvious argument parsing; the
//! runtime (`apply`/`pump`) and the renderer do the real work.
//!
//! Usage:
//!   croft-chat demo [--message <text>]
//!
//! Commands:
//!   demo   Run a deterministic two-peer scenario over the in-proc bus: alice
//!          joins and sends a message, bob joins and pumps, and both rendered
//!          views are printed. No network — the regression/demo bed.

use std::process::ExitCode;

use croft_chat_cli::render::render;
use croft_chat_cli::runtime::{apply, pump};
use croft_chat_cli::transport::{InProcBus, Topic};
use croft_chat_cli::GROUP_TOPIC;
use group_core::{project, Intent, Model};

const DEFAULT_MESSAGE: &str = "hello from the in-proc bus";

fn main() -> ExitCode {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let mut command: Option<String> = None;
    let mut message = DEFAULT_MESSAGE.to_string();

    let mut it = args.into_iter();
    while let Some(arg) = it.next() {
        match arg.as_str() {
            "--message" => match it.next() {
                Some(text) => message = text,
                None => return fail("--message needs a value"),
            },
            "-h" | "--help" => {
                print_help();
                return ExitCode::SUCCESS;
            }
            other if command.is_none() => command = Some(other.to_string()),
            other => return fail(&format!("unexpected argument: {other}")),
        }
    }

    match command.as_deref() {
        Some("demo") => {
            print!("{}", run_demo(&message));
            ExitCode::SUCCESS
        }
        Some(other) => fail(&format!("unknown command: {other}")),
        None => {
            print_help();
            ExitCode::SUCCESS
        }
    }
}

/// Run the deterministic two-peer scenario and return the rendered transcript
/// (both peers' views). Pure of args/IO so the binary smoke test asserts on it
/// via stdout and the logic stays easy to follow.
fn run_demo(message: &str) -> String {
    let bus = InProcBus::new();
    let mut alice = bus.attach();
    let mut bob = bus.attach();
    let topic = Topic::new(GROUP_TOPIC);

    // Both peers join; alice sends; bob pumps to receive.
    let mut alice_model = apply(Model::new(), Intent::JoinGroup, &mut alice, &topic);
    let bob_model = apply(Model::new(), Intent::JoinGroup, &mut bob, &topic);
    alice_model = apply(
        alice_model,
        Intent::SendMessage {
            text: message.to_string(),
        },
        &mut alice,
        &topic,
    );
    let bob_model = pump(bob_model, &mut bob, &topic);

    format!(
        "alice's view:\n{}\n\nbob's view:\n{}\n",
        render(&project(&alice_model)),
        render(&project(&bob_model)),
    )
}

fn print_help() {
    println!("usage: croft-chat demo [--message <text>]");
}

fn fail(message: &str) -> ExitCode {
    eprintln!("error: {message}");
    ExitCode::FAILURE
}
