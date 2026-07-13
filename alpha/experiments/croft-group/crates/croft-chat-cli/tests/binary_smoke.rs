//! Binary smoke test: run the actual `croft-chat` entry point and confirm it
//! drives the two-peer round-trip — the sent message must appear in both peers'
//! rendered views (so the receiver genuinely got it through the bus).

use std::process::Command;

#[test]
fn croft_chat_demo_prints_the_sent_message_in_both_views() {
    let output = Command::new(env!("CARGO_BIN_EXE_croft-chat"))
        .args(["demo", "--message", "wiring works"])
        .output()
        .expect("the croft-chat binary runs");

    assert!(
        output.status.success(),
        "croft-chat demo exits successfully (status {:?})",
        output.status
    );
    let stdout = String::from_utf8(output.stdout).expect("stdout is utf-8");

    // The message appears twice: once in the sender's view, once in the
    // receiver's view after the pump — proof the round-trip reached the receiver.
    let occurrences = stdout.matches("wiring works").count();
    assert!(
        occurrences >= 2,
        "the sent message should appear in both peers' views (found {occurrences}), got:\n{stdout}"
    );
    assert!(
        stdout.contains("bob"),
        "the receiver's view is labelled, got:\n{stdout}"
    );
}
