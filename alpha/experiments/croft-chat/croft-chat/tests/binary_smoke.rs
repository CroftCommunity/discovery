//! P12 binary smoke (Milestone B gate): the real binary persists state across
//! *separate process invocations*.
//!
//! Each `exec` op opens the store, acts, and exits — so a group created by one
//! process and a message sent by a second must be visible to a third. This
//! proves both persistence and lamport restart-resume end to end through the
//! actual compiled binary, not just the library.

use std::process::Command;

fn bin() -> &'static str {
    // Cargo sets this for integration tests of a binary crate.
    env!("CARGO_BIN_EXE_croft-chat")
}

#[test]
fn group_and_message_persist_across_process_restarts() {
    let dir = tempfile::tempdir().expect("tempdir");
    let store = dir.path().join("smoke.redb");
    let store = store.to_str().expect("utf-8 path");

    // Process 1: create a group, capture its id.
    let created = Command::new(bin())
        .args(["--store", store, "exec", "create-group"])
        .output()
        .expect("spawn create-group");
    assert!(created.status.success(), "create-group exits 0: {created:?}");
    let group_id = String::from_utf8(created.stdout).expect("utf-8").trim().to_string();
    assert_eq!(group_id.len(), 64, "group id is 64 hex chars: {group_id:?}");

    // Process 2: send a message into that group (fresh process — store reopened,
    // lamport resumed).
    let sent = Command::new(bin())
        .args(["--store", store, "exec", "send", &group_id, "hello smoke"])
        .status()
        .expect("spawn send");
    assert!(sent.success(), "send exits 0");

    // Process 3: read the timeline back — the message persisted across restarts.
    let dumped = Command::new(bin())
        .args(["--store", store, "exec", "timeline", &group_id])
        .output()
        .expect("spawn timeline");
    assert!(dumped.status.success(), "timeline exits 0");
    let timeline = String::from_utf8(dumped.stdout).expect("utf-8");
    assert!(
        timeline.contains("hello smoke"),
        "persisted message must be in the timeline, got: {timeline:?}"
    );

    // And the group is listed (membership persisted).
    let listed = Command::new(bin())
        .args(["--store", store, "exec", "list"])
        .output()
        .expect("spawn list");
    let groups = String::from_utf8(listed.stdout).expect("utf-8");
    assert!(groups.contains(&group_id), "created group is listed after restart");
}
