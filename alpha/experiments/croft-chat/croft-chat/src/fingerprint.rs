//! A total, deterministic fingerprint of a group's derived state.
//!
//! Two nodes that have applied the same set of assertions must produce the same
//! fingerprint regardless of arrival order (invariant I5). The fingerprint is a
//! canonical string over the public projection — members and the message
//! timeline — so a mismatch is not just detectable but *diffable*: the failing
//! convergence test prints both fingerprints and the per-line difference.

use social_graph_core::{GroupId, Session, TimelineWindow};

/// Build the canonical fingerprint lines for `group` as seen by `session`.
///
/// Returns one sorted `Vec` of lines so a caller can both hash it (join) and
/// diff it line-by-line on mismatch. Empty if the group is unknown.
#[must_use]
pub fn fingerprint_lines(session: &Session, group: &GroupId) -> Vec<String> {
    let mut lines = Vec::new();

    if let Ok(summary) = session.get_group_summary(group) {
        let mut members: Vec<String> = summary
            .members
            .iter()
            .map(|m| format!("member {} role={:?}", hex32(m.principal.as_bytes()), m.role))
            .collect();
        members.sort();
        lines.extend(members);
    }

    if let Ok(timeline) = session.get_timeline(group, TimelineWindow::LastN(usize::MAX)) {
        let mut messages: Vec<String> = timeline
            .entries
            .iter()
            .filter_map(|entry| {
                session.get_message(&entry.hash).map(|m| {
                    format!(
                        "msg lamport={} hash={} reply={} body={:?}",
                        m.lamport,
                        hex32(m.hash.as_bytes()),
                        m.reply_to
                            .map_or_else(|| "none".to_string(), |h| hex32(h.as_bytes())),
                        m.body,
                    )
                })
            })
            .collect();
        // Total order independent of arrival: by lamport then hash (already in
        // the line text, so a plain sort is canonical).
        messages.sort();
        lines.extend(messages);
    }

    lines
}

/// The fingerprint as a single canonical string.
#[must_use]
pub fn fingerprint(session: &Session, group: &GroupId) -> String {
    fingerprint_lines(session, group).join("\n")
}

fn hex32(bytes: &[u8; 32]) -> String {
    let mut s = String::with_capacity(64);
    for b in bytes {
        s.push_str(&format!("{b:02x}"));
    }
    s
}

#[cfg(test)]
mod tests {
    use super::hex32;

    #[test]
    fn hex32_is_64_chars() {
        assert_eq!(hex32(&[0xab; 32]).len(), 64);
        assert_eq!(&hex32(&[0x0f; 32])[..2], "0f");
    }
}
