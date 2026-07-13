//! The Bluesky module's pinning hooks (BUILD-SPEC Phase 2 M2.4). A module
//! participates in pinning by declaring two things: that its items are pinnable
//! (they have a stable ID), and how to address one. The shell owns the pin list
//! and the strip; the module owns "given this item, what is its address" and
//! (in the shell that performs I/O) "given this address, hydrate it".
//!
//! These are deliberately tiny and protocol-light: the address is the post's
//! native at:// URI, an opaque stable id the shell stores without interpreting.

use crate::types::Post;

/// Bluesky posts are pinnable: they carry a stable id.
pub const ITEMS_ARE_PINNABLE: bool = true;

/// The stable address used to pin an item — its native URI.
pub fn address_of(post: &Post) -> &str {
    &post.uri
}

/// The XRPC path + query to hydrate posts by address. The shell performs the
/// actual fetch (browser fetch on web, an HTTP client on desktop) and parses
/// the result with [`crate::wire::parse_posts`]; keeping the URL shape here
/// means the module owns how its items are addressed, not the shell.
pub fn hydrate_path(addresses: &[&str]) -> String {
    let mut path = String::from("/xrpc/app.bsky.feed.getPosts");
    for (i, addr) in addresses.iter().enumerate() {
        path.push_str(if i == 0 { "?" } else { "&" });
        path.push_str("uris=");
        path.push_str(&percent_encode(addr));
    }
    path
}

fn percent_encode(value: &str) -> String {
    let mut out = String::with_capacity(value.len());
    for &b in value.as_bytes() {
        match b {
            b'A'..=b'Z' | b'a'..=b'z' | b'0'..=b'9' | b'-' | b'_' | b'.' | b'~' => {
                out.push(b as char)
            }
            _ => out.push_str(&format!("%{b:02X}")),
        }
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn hydrate_path_encodes_the_uri() {
        let p = hydrate_path(&["at://did:plc:x/app.bsky.feed.post/1"]);
        assert!(p.starts_with("/xrpc/app.bsky.feed.getPosts?uris="));
        assert!(p.contains("at%3A%2F%2Fdid")); // ':' and '/' encoded
    }
}
