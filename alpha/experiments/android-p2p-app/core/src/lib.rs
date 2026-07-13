//! `group-core`: a self-contained Rust core wiring **iroh** (p2p transport) and
//! **Automerge** (CRDT) into a *truly basic* private-group read/post experience,
//! exposed to Android through a single JSON-in / JSON-out call (UniFFI).
//!
//! See `experiments/android-p2p-app/README.md` for the full build brief, the
//! capability-tier report, and the honest friction log.
//!
//! ## Shape
//! * [`group`]    — the Automerge document (the `messages` list).
//! * [`net`]      — the iroh endpoint + snapshot-exchange channel (lazily started).
//! * [`protocol`] — the JSON `Command`/`Response` surface and the group `Invite`.
//! * [`GroupClient`] — the UniFFI object Kotlin holds; its `handle(json)` method is
//!   the whole FFI boundary.

use std::sync::{Arc, Mutex};
use std::time::{SystemTime, UNIX_EPOCH};

use anyhow::{Context, Result};
use iroh::SecretKey;

pub mod group;
pub mod net;
pub mod protocol;

use group::GroupDoc;
use net::P2pNode;
use protocol::{Command, Invite, Response};

uniffi::setup_scaffolding!();

/// One group session: local identity, the group document, and (once a group is
/// created or joined) the lazily-started iroh node.
pub struct Session {
    /// Display name used as the author of locally posted messages.
    author: String,
    /// Stable identity material (kept as raw bytes so we can rebuild a `SecretKey`
    /// without requiring it to be `Clone`).
    secret_bytes: [u8; 32],
    /// Tokio runtime used to drive iroh's async APIs from the synchronous,
    /// Delta-Chat-style blocking FFI surface.
    rt: tokio::runtime::Runtime,
    /// The group CRDT.
    doc: GroupDoc,
    /// The iroh node — `None` until a group is created/joined (lazy start).
    node: Option<P2pNode>,
    /// 32-byte group topic id.
    topic: Option<[u8; 32]>,
    /// A known peer to sync with (for a joiner: the inviter).
    peer: Option<iroh::EndpointAddr>,
    /// Whether to use n0 relay/discovery (`true`, real devices) or bind locally with
    /// no relay (`false`, hermetic tests / same-host).
    relay: bool,
}

impl Session {
    fn new(relay: bool) -> Self {
        Session {
            author: "anon".to_string(),
            secret_bytes: SecretKey::generate().to_bytes(),
            rt: tokio::runtime::Builder::new_multi_thread()
                .enable_all()
                .build()
                .expect("build tokio runtime"),
            doc: GroupDoc::new_empty(),
            node: None,
            topic: None,
            peer: None,
            relay,
        }
    }

    fn secret_key(&self) -> SecretKey {
        SecretKey::from_bytes(&self.secret_bytes)
    }

    /// Lazily bind the iroh endpoint (Delta-Chat lesson: don't start p2p machinery
    /// until the group is actually opened).
    fn ensure_node(&mut self) -> Result<()> {
        if self.node.is_none() {
            let secret = self.secret_key();
            let relay = self.relay;
            let node = self
                .rt
                .block_on(P2pNode::bind(Some(secret), relay))
                .context("bind iroh endpoint")?;
            if relay {
                // Wait until online, but don't block forever if connectivity is poor.
                self.rt.block_on(async {
                    let _ = tokio::time::timeout(
                        std::time::Duration::from_secs(15),
                        node.online(),
                    )
                    .await;
                });
            }
            self.node = Some(node);
        }
        Ok(())
    }

    fn handle(&mut self, cmd: Command) -> Result<Response> {
        match cmd {
            Command::Init { author } => {
                self.author = author;
                Ok(Response::Ok)
            }
            Command::CreateGroup => {
                self.ensure_node()?;
                self.topic = Some(rand::random::<[u8; 32]>());
                self.doc = GroupDoc::new_group()?;
                // Drop any peer carried over from a previously joined group, so a
                // later best-effort sync can't merge this fresh group's doc into a
                // stale remote group's state.
                self.peer = None;
                Ok(Response::Ok)
            }
            Command::GetInvite => {
                let node = self.node.as_ref().context("create or join a group first")?;
                let topic = self.topic.context("no group topic yet")?;
                let invite = Invite::new(&node.addr(), &topic).encode()?;
                Ok(Response::Invite { invite })
            }
            Command::JoinGroup { invite } => {
                let inv = Invite::decode(&invite)?;
                self.topic = Some(inv.topic_bytes()?);
                let peer = inv.endpoint_addr()?;
                self.ensure_node()?;
                self.doc = GroupDoc::new_empty();
                // First sync: pull the creator's shared-genesis document.
                let node = self.node.as_ref().expect("node bound above");
                let local = self.doc.snapshot();
                let peer_bytes = self
                    .rt
                    .block_on(node.connect_and_exchange(peer.clone(), local))
                    .context("initial sync with inviter")?;
                self.doc.merge_snapshot(&peer_bytes)?;
                self.peer = Some(peer);
                Ok(Response::Ok)
            }
            Command::PostMessage { text } => {
                self.doc.post(&self.author, &text, now_millis())?;
                // Best-effort propagation to a known peer (joiner -> creator).
                self.sync_with_peer().ok();
                Ok(Response::Ok)
            }
            Command::GetMessages => Ok(Response::Messages {
                messages: self.doc.messages(),
            }),
            Command::Sync => {
                self.sync_with_peer()?;
                Ok(Response::Ok)
            }
        }
    }

    /// Exchange snapshots with the known peer and merge the result.
    fn sync_with_peer(&mut self) -> Result<()> {
        let peer = self.peer.clone().context("no peer to sync with")?;
        let node = self.node.as_ref().context("node not started")?;
        let local = self.doc.snapshot();
        let peer_bytes = self.rt.block_on(node.connect_and_exchange(peer, local))?;
        self.doc.merge_snapshot(&peer_bytes)?;
        Ok(())
    }
}

fn now_millis() -> i64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_millis() as i64)
        .unwrap_or(0)
}

/// The object Kotlin holds across the FFI boundary. Its single [`GroupClient::handle`]
/// method is the entire surface: a JSON `Command` in, a JSON `Response` out.
#[derive(uniffi::Object)]
pub struct GroupClient {
    inner: Mutex<Session>,
}

#[uniffi::export]
impl GroupClient {
    /// Production client: uses n0 relay + discovery, so peers can be reached across
    /// NATs using only the relay url + public key carried in an invite.
    #[uniffi::constructor]
    pub fn new() -> Arc<Self> {
        Arc::new(Self {
            inner: Mutex::new(Session::new(true)),
        })
    }

    /// Local client: binds with relay/discovery disabled. Intended for tests and
    /// same-host/LAN experimentation.
    #[uniffi::constructor]
    pub fn new_local() -> Arc<Self> {
        Arc::new(Self {
            inner: Mutex::new(Session::new(false)),
        })
    }

    /// The whole Kotlin <-> Rust boundary: send a JSON `Command`, get a JSON
    /// `Response`. Call this on a background thread from Kotlin (the call blocks
    /// while iroh work runs on the embedded Tokio runtime).
    pub fn handle(&self, command_json: String) -> String {
        // Never panic across the FFI boundary: a poisoned mutex becomes a JSON error.
        let mut session = match self.inner.lock() {
            Ok(guard) => guard,
            Err(_) => {
                return r#"{"status":"error","message":"session mutex poisoned"}"#.to_string()
            }
        };
        let response = match serde_json::from_str::<Command>(&command_json) {
            Ok(cmd) => session
                .handle(cmd)
                .unwrap_or_else(|e| Response::error(e.to_string())),
            Err(e) => Response::error(format!("bad command json: {e}")),
        };
        serde_json::to_string(&response).unwrap_or_else(|e| {
            format!("{{\"status\":\"error\",\"message\":\"serialize response: {e}\"}}")
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// The command surface works offline: init -> create -> post -> read, plus an
    /// invite is produced. (Live multi-device sync is exercised by the
    /// `sync_over_iroh` integration test over real iroh.)
    #[test]
    fn command_surface_offline() {
        let client = GroupClient::new_local();

        let r = client.handle(r#"{"cmd":"init","author":"alice"}"#.to_string());
        assert!(r.contains("\"ok\""), "init: {r}");

        let r = client.handle(r#"{"cmd":"create_group"}"#.to_string());
        assert!(r.contains("\"ok\""), "create_group: {r}");

        let r = client.handle(r#"{"cmd":"post_message","text":"hello group"}"#.to_string());
        assert!(r.contains("\"ok\""), "post_message: {r}");

        let r = client.handle(r#"{"cmd":"get_messages"}"#.to_string());
        assert!(r.contains("hello group"), "get_messages: {r}");
        assert!(r.contains("alice"), "author should be alice: {r}");

        let r = client.handle(r#"{"cmd":"get_invite"}"#.to_string());
        assert!(r.contains("croftcgrp1:"), "get_invite: {r}");

        // Unknown command is reported, not panicked.
        let r = client.handle(r#"{"cmd":"nonsense"}"#.to_string());
        assert!(r.contains("\"error\""), "unknown command: {r}");
    }
}
