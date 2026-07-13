//! `SharedDirBus` — a cross-process [`Transport`] over an append-only directory.
//!
//! One file per frame under `<root>/<topic>/`. Writes are atomic (temp file +
//! rename) so a reader never sees a partial frame. `drain` returns frames a peer
//! has not seen and did not author, in a deliberately scrambled order (sorted by
//! a content hash, not by send order) — this is the local stand-in for an
//! unordered network, exercising the convergence layer's own ordering (P7).

use std::collections::HashSet;
use std::fs;
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicU64, Ordering};

use crate::transport::{Frame, Topic, Transport};

/// A transport backed by a shared directory.
pub struct SharedDirBus {
    root: PathBuf,
    node_id: String,
    seq: AtomicU64,
    subscribed: HashSet<String>,
    seen: HashSet<String>,
}

impl SharedDirBus {
    /// Create a bus rooted at `root`, identified by `node_id` (used to skip this
    /// peer's own frames on `drain`). `node_id` must not contain the filename
    /// separator `__`.
    #[must_use]
    pub fn new(root: impl AsRef<Path>, node_id: impl Into<String>) -> Self {
        Self {
            root: root.as_ref().to_path_buf(),
            node_id: node_id.into(),
            seq: AtomicU64::new(0),
            subscribed: HashSet::new(),
            seen: HashSet::new(),
        }
    }

    fn topic_dir(&self, topic: &str) -> PathBuf {
        self.root.join(topic)
    }

    /// A small, fast, dependency-free content hash used both for unique
    /// filenames and for the scramble ordering (FNV-1a, 64-bit).
    fn fnv1a(bytes: &[u8]) -> u64 {
        let mut hash: u64 = 0xcbf2_9ce4_8422_2325;
        for b in bytes {
            hash ^= u64::from(*b);
            hash = hash.wrapping_mul(0x0000_0100_0000_01b3);
        }
        hash
    }
}

impl Transport for SharedDirBus {
    fn subscribe(&mut self, topic: &Topic) {
        self.subscribed.insert(topic.0.clone());
    }

    fn publish(&mut self, topic: &Topic, frame: Frame) {
        let dir = self.topic_dir(&topic.0);
        if fs::create_dir_all(&dir).is_err() {
            tracing::error!(topic = %topic.0, "shared-dir: create_dir_all failed");
            return;
        }
        // Filename: <node_id>__<seq>__<content-hash>.frame — node_id prefix lets
        // drain skip our own frames; the hash keeps it unique per content.
        let seq = self.seq.fetch_add(1, Ordering::SeqCst);
        let content_hash = Self::fnv1a(&frame.0);
        let name = format!("{}__{seq}__{content_hash:016x}.frame", self.node_id);
        let final_path = dir.join(&name);
        let tmp_path = dir.join(format!(".tmp__{name}"));

        if fs::write(&tmp_path, &frame.0).is_err() {
            tracing::error!(topic = %topic.0, "shared-dir: temp write failed");
            return;
        }
        if fs::rename(&tmp_path, &final_path).is_err() {
            tracing::error!(topic = %topic.0, "shared-dir: atomic rename failed");
            let _ = fs::remove_file(&tmp_path);
            return;
        }
        // We authored it — never deliver it back to ourselves.
        self.seen.insert(name);
        tracing::debug!(topic = %topic.0, len = frame.0.len(), "shared-dir: published");
    }

    fn drain(&mut self) -> Vec<Frame> {
        let mut collected: Vec<(u64, Frame)> = Vec::new();
        for topic in &self.subscribed {
            let dir = self.topic_dir(topic);
            let entries = match fs::read_dir(&dir) {
                Ok(entries) => entries,
                Err(_) => continue, // topic dir not created yet
            };
            for entry in entries.flatten() {
                let file_name = entry.file_name().to_string_lossy().into_owned();
                if !file_name.ends_with(".frame") || file_name.starts_with(".tmp__") {
                    continue;
                }
                if self.seen.contains(&file_name) {
                    continue;
                }
                // Skip our own frames (no self-echo).
                if file_name.starts_with(&format!("{}__", self.node_id)) {
                    self.seen.insert(file_name);
                    continue;
                }
                match fs::read(entry.path()) {
                    Ok(bytes) => {
                        let key = Self::fnv1a(&bytes);
                        self.seen.insert(file_name);
                        collected.push((key, Frame(bytes)));
                    }
                    Err(_) => continue, // mid-write race; pick it up next drain
                }
            }
        }
        // Deliberate scramble: order by content hash, independent of send order.
        collected.sort_by_key(|(key, _)| *key);
        let count = collected.len();
        if count > 0 {
            tracing::debug!(count, "shared-dir: drained");
        }
        collected.into_iter().map(|(_, frame)| frame).collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn topic() -> Topic {
        Topic("group/general".to_string())
    }

    #[test]
    fn peer_receives_all_published_frames() {
        let dir = tempfile::tempdir().expect("tempdir");
        let mut a = SharedDirBus::new(dir.path(), "node-a");
        let mut b = SharedDirBus::new(dir.path(), "node-b");
        a.subscribe(&topic());
        b.subscribe(&topic());

        for i in 0..5u8 {
            a.publish(&topic(), Frame(vec![i, i + 1, i + 2]));
        }

        let mut received = b.drain();
        received.sort(); // assert set-equality, not order
        assert_eq!(received.len(), 5, "B receives all 5 frames A published");
        let mut expected: Vec<Frame> =
            (0..5u8).map(|i| Frame(vec![i, i + 1, i + 2])).collect();
        expected.sort();
        assert_eq!(received, expected);
    }

    #[test]
    fn no_self_echo() {
        let dir = tempfile::tempdir().expect("tempdir");
        let mut a = SharedDirBus::new(dir.path(), "node-a");
        a.subscribe(&topic());
        a.publish(&topic(), Frame(vec![1, 2, 3]));
        assert!(a.drain().is_empty(), "a peer never drains its own frames");
    }

    #[test]
    fn drain_is_idempotent_across_calls() {
        let dir = tempfile::tempdir().expect("tempdir");
        let mut a = SharedDirBus::new(dir.path(), "node-a");
        let mut b = SharedDirBus::new(dir.path(), "node-b");
        a.subscribe(&topic());
        b.subscribe(&topic());

        a.publish(&topic(), Frame(vec![9]));
        assert_eq!(b.drain().len(), 1, "first drain delivers the frame");
        assert!(b.drain().is_empty(), "second drain does not redeliver");
    }

    #[test]
    fn unsubscribed_topic_yields_nothing() {
        let dir = tempfile::tempdir().expect("tempdir");
        let mut a = SharedDirBus::new(dir.path(), "node-a");
        let mut b = SharedDirBus::new(dir.path(), "node-b");
        a.subscribe(&topic());
        // b does NOT subscribe.
        a.publish(&topic(), Frame(vec![1]));
        assert!(b.drain().is_empty(), "unsubscribed peer receives nothing");
    }
}
