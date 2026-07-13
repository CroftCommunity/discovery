//! THE load-bearing test: two iroh peers sync an Automerge group document over
//! **real iroh**, in both directions.
//!
//! It is hermetic — both endpoints bind to loopback with relay/discovery disabled
//! and dial each other via explicit direct addresses — so it exercises real iroh
//! QUIC connectivity and the real Automerge merge without needing the public n0
//! relay/DNS infrastructure to be reachable from the build sandbox.
//!
//! Scenario (post -> sync -> read, both ways):
//!   1. `host` creates the group (shared genesis) and posts "from host".
//!   2. `joiner` starts empty, connects to `host`, and they exchange snapshots.
//!      The joiner now has the shared list + the host's message.
//!   3. `joiner` posts "from joiner" and they exchange again.
//!   4. Both documents must converge to the same two messages.

use group_core::group::GroupDoc;
use group_core::net::P2pNode;

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn two_peers_sync_automerge_over_real_iroh() {
    // 1. Host builds the shared genesis and posts.
    let mut host_doc = GroupDoc::new_group().expect("genesis");
    host_doc.post("host", "from host", 1).expect("post host");

    let host = P2pNode::bind(None, false).await.expect("bind host");
    let joiner = P2pNode::bind(None, false).await.expect("bind joiner");

    // The joiner dials the host's direct (loopback) address. Real iroh, no relay.
    let host_addr = host.direct_addr();
    assert!(
        host_addr.ip_addrs().count() > 0,
        "host must expose a loopback ip addr for the hermetic test"
    );

    // 2. First exchange: host accepts, joiner connects. Run both concurrently.
    let mut joiner_doc = GroupDoc::new_empty();

    let host_snapshot = host_doc.snapshot();
    let host_task =
        tokio::spawn(async move { host.accept_and_exchange(host_snapshot).await.map(|b| (host, b)) });

    let joiner_local = joiner_doc.snapshot();
    let from_host = joiner
        .connect_and_exchange(host_addr.clone(), joiner_local)
        .await
        .expect("joiner connect/exchange #1");
    joiner_doc.merge_snapshot(&from_host).expect("merge host->joiner");

    let (host, from_joiner) = host_task.await.expect("host task #1").expect("host exchange #1");
    host_doc.merge_snapshot(&from_joiner).expect("merge joiner->host");

    // Joiner obtained the shared list and the host's message.
    assert_eq!(joiner_doc.messages().len(), 1, "joiner synced host's message");

    // 3. Joiner posts, then a second exchange propagates it back to the host.
    joiner_doc.post("joiner", "from joiner", 2).expect("post joiner");

    let host_snapshot = host_doc.snapshot();
    let host_task = tokio::spawn(async move { host.accept_and_exchange(host_snapshot).await });

    let joiner_local = joiner_doc.snapshot();
    let from_host = joiner
        .connect_and_exchange(host_addr, joiner_local)
        .await
        .expect("joiner connect/exchange #2");
    joiner_doc.merge_snapshot(&from_host).expect("merge host->joiner #2");

    let from_joiner = host_task.await.expect("host task #2").expect("host exchange #2");
    host_doc.merge_snapshot(&from_joiner).expect("merge joiner->host #2");

    // 4. Both converge to the same two messages.
    let host_msgs = host_doc.messages();
    let joiner_msgs = joiner_doc.messages();

    println!("host sees:   {:?}", host_msgs);
    println!("joiner sees: {:?}", joiner_msgs);

    assert_eq!(host_msgs, joiner_msgs, "documents must converge");
    assert_eq!(host_msgs.len(), 2, "two messages after bidirectional sync");

    let texts: Vec<&str> = host_msgs.iter().map(|m| m.text.as_str()).collect();
    assert!(texts.contains(&"from host"), "host message present: {texts:?}");
    assert!(texts.contains(&"from joiner"), "joiner message present: {texts:?}");
}
