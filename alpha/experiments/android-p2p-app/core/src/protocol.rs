//! The JSON command surface (Option A — Delta-Chat-style JSON-RPC-over-FFI) and the
//! group invite format.
//!
//! The whole Kotlin <-> Rust boundary is a single string-in / string-out call:
//! Kotlin sends a JSON `Command`, Rust returns a JSON `Response`. Adding a feature is
//! "add a `Command` variant + a match arm" — no new binding plumbing per method,
//! which is exactly why Delta Chat moved from per-method C-FFI/JNI to JSON-RPC.

use std::str::FromStr;

use anyhow::{anyhow, Context, Result};
use data_encoding::BASE64URL_NOPAD;
use iroh::{EndpointAddr, EndpointId, RelayUrl, TransportAddr};
use serde::{Deserialize, Serialize};

/// Commands accepted by [`crate::Session::handle`]. Tagged JSON, e.g.
/// `{"cmd":"post_message","text":"hi"}`.
#[derive(Debug, Clone, Deserialize)]
#[serde(tag = "cmd", rename_all = "snake_case")]
pub enum Command {
    /// Set the local display name used as the author of posted messages.
    Init { author: String },
    /// Create a brand-new group (shared genesis) and start the iroh endpoint lazily.
    CreateGroup,
    /// Join an existing group from an invite string and perform the first sync.
    JoinGroup { invite: String },
    /// Produce an invite string others can use to join (creator only).
    GetInvite,
    /// Append a message authored locally.
    PostMessage { text: String },
    /// Return the current message list.
    GetMessages,
    /// Pull/push the latest document with the known peer (manual sync trigger).
    Sync,
}

/// Responses returned by [`crate::Session::handle`], serialized as tagged JSON.
#[derive(Debug, Clone, Serialize)]
#[serde(tag = "status", rename_all = "snake_case")]
pub enum Response {
    Ok,
    Invite { invite: String },
    Messages { messages: Vec<crate::group::Message> },
    Error { message: String },
}

impl Response {
    pub fn error(message: impl Into<String>) -> Self {
        Response::Error {
            message: message.into(),
        }
    }
}

/// A group invitation, mirroring Delta Chat's introduction message.
///
/// Carries the inviter's relay url + public key and a random 32-byte topic id. It
/// **deliberately excludes the direct IP address** — that information should not be
/// persisted by whoever relays the invite; holepunching finds the direct path later.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Invite {
    /// Inviter's public key (iroh `EndpointId`), as a string.
    pub node_id: String,
    /// Inviter's relay url, if it has one yet.
    pub relay_url: Option<String>,
    /// Random 32-byte topic id, hex-encoded. Identifies the group channel.
    pub topic: String,
}

const INVITE_PREFIX: &str = "croftcgrp1:";

/// Upper bound on the base64 body of an invite. A well-formed invite is a few
/// hundred bytes; this guards [`Invite::decode`] against hostile input forcing
/// large allocations during base64/JSON decode.
const MAX_INVITE_BODY_LEN: usize = 8 * 1024;

impl Invite {
    /// Build an invite from the inviter's endpoint address and the group topic.
    /// Any IP transport addresses in `addr` are dropped on purpose.
    pub fn new(addr: &EndpointAddr, topic: &[u8; 32]) -> Self {
        let relay_url = addr.relay_urls().next().map(|u| u.to_string());
        Invite {
            node_id: addr.id.to_string(),
            relay_url,
            topic: data_encoding::HEXLOWER.encode(topic),
        }
    }

    /// Encode as a single opaque, copy-pasteable string.
    pub fn encode(&self) -> Result<String> {
        let json = serde_json::to_vec(self).context("serialize invite")?;
        Ok(format!("{INVITE_PREFIX}{}", BASE64URL_NOPAD.encode(&json)))
    }

    /// Decode an invite string produced by [`Invite::encode`].
    pub fn decode(s: &str) -> Result<Self> {
        let body = s
            .strip_prefix(INVITE_PREFIX)
            .ok_or_else(|| anyhow!("not a group invite (bad prefix)"))?;
        if body.len() > MAX_INVITE_BODY_LEN {
            return Err(anyhow!(
                "invite too large ({} > {MAX_INVITE_BODY_LEN} bytes)",
                body.len()
            ));
        }
        let json = BASE64URL_NOPAD
            .decode(body.as_bytes())
            .context("base64-decode invite")?;
        serde_json::from_slice(&json).context("parse invite json")
    }

    /// The 32-byte topic id this invite refers to.
    pub fn topic_bytes(&self) -> Result<[u8; 32]> {
        let bytes = data_encoding::HEXLOWER
            .decode(self.topic.as_bytes())
            .context("decode topic hex")?;
        bytes
            .as_slice()
            .try_into()
            .map_err(|_| anyhow!("topic id must be 32 bytes"))
    }

    /// Reconstruct a dialable [`EndpointAddr`] (relay + public key, no IP) from the
    /// invite — exactly the information that crosses the wire to join a group.
    pub fn endpoint_addr(&self) -> Result<EndpointAddr> {
        let id = EndpointId::from_str(&self.node_id).context("parse node id")?;
        let mut addr = EndpointAddr::new(id);
        if let Some(url) = &self.relay_url {
            let relay = RelayUrl::from_str(url).context("parse relay url")?;
            addr = addr.with_addrs(std::iter::once(TransportAddr::Relay(relay)));
        }
        Ok(addr)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn invite_roundtrip_excludes_ip() {
        let topic = [7u8; 32];
        // An address that includes an IP — the invite must drop it.
        let id = iroh::SecretKey::generate().public();
        let addr = EndpointAddr::from_parts(
            id,
            [TransportAddr::Ip("203.0.113.5:4242".parse().unwrap())],
        );

        let invite = Invite::new(&addr, &topic);
        assert_eq!(invite.node_id, id.to_string());
        assert!(invite.relay_url.is_none());

        let encoded = invite.encode().unwrap();
        assert!(encoded.starts_with(INVITE_PREFIX));
        // No IP leaked into the serialized invite.
        assert!(!encoded.contains("203.0.113"));

        let decoded = Invite::decode(&encoded).unwrap();
        assert_eq!(decoded.topic_bytes().unwrap(), topic);
        let reconstructed = decoded.endpoint_addr().unwrap();
        assert_eq!(reconstructed.id, id);
        assert_eq!(reconstructed.ip_addrs().count(), 0, "invite must carry no IP");
    }
}
