//! A fixture-backed fake implementing `BlueskyPort`. Deterministic, no network.
//! This is what every Phase 0 behavior/projection-adjacent test and the offline
//! CLI run against (BUILD-SPEC §6).
//!
//! It serves `timeline_page_1.json` for a `None` cursor and
//! `timeline_page_2.json` for the cursor page 1 returns. It can be configured
//! into an empty mode (`timeline_empty.json`) or an error mode. The error mode
//! is what the CLI's error-path command drives (DECISION 5).
//!
//! The fixtures themselves are real recorded `getTimeline` responses supplied
//! with the repo; this code never fabricates them (BUILD-SPEC §0, §6).

use crate::port::{BlueskyPort, TimelinePage};
use crate::wire::parse_timeline;
use std::path::{Path, PathBuf};

/// How the fake behaves for this run.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FakeMode {
    /// Serve page 1 then page 2 from the fixtures (the happy path).
    Normal,
    /// Serve the empty-timeline fixture for the cold load.
    Empty,
    /// Fail every fetch (drives the error rendering paths).
    Error,
}

/// The fake. Reads fixtures from a directory at call time, so it compiles
/// whether or not the fixtures are present and fails honestly (not silently) if
/// they are missing.
pub struct FakeBluesky {
    fixtures_dir: PathBuf,
    mode: FakeMode,
}

impl FakeBluesky {
    pub fn new(fixtures_dir: impl Into<PathBuf>) -> Self {
        FakeBluesky {
            fixtures_dir: fixtures_dir.into(),
            mode: FakeMode::Normal,
        }
    }

    pub fn with_mode(mut self, mode: FakeMode) -> Self {
        self.mode = mode;
        self
    }

    fn load(&self, file: &str) -> Result<TimelinePage, String> {
        let path: PathBuf = self.fixtures_dir.join(file);
        // Report the fixture file name only, not the full resolved path, so the
        // error doesn't leak the directory layout.
        let raw = std::fs::read_to_string(&path).map_err(|_| {
            format!(
                "fixture {file} unreadable. The committed real recordings must be \
                 present (BUILD-SPEC §0, §6); fixtures are never fabricated."
            )
        })?;
        parse_timeline(&raw)
    }
}

impl BlueskyPort for FakeBluesky {
    async fn fetch_timeline(&self, cursor: Option<String>) -> Result<TimelinePage, String> {
        match self.mode {
            FakeMode::Error => Err("simulated fetch failure (fake error mode)".to_string()),
            FakeMode::Empty => self.load("timeline_empty.json"),
            FakeMode::Normal => match cursor {
                // Cold load -> page 1.
                None => self.load("timeline_page_1.json"),
                // A cursor must be the one page 1 actually emitted; anything
                // else means broken cursor plumbing, so fail rather than
                // silently serve page 2 and let a bug pass.
                Some(got) => {
                    let expected = self
                        .load("timeline_page_1.json")?
                        .next_cursor
                        .ok_or_else(|| "page 1 fixture has no next cursor".to_string())?;
                    if got == expected {
                        self.load("timeline_page_2.json")
                    } else {
                        // Don't echo the cursor itself — it is an opaque token;
                        // a length fingerprint is enough to debug bad plumbing.
                        Err(format!(
                            "unexpected cursor (not page 1's): got a {}-char token",
                            got.len()
                        ))
                    }
                }
            },
        }
    }
}

// --- fixtures ---

/// The conventional location of the committed fixtures, for shells that want a
/// default. `CARGO_MANIFEST_DIR` resolves to this crate at build time.
pub fn default_fixtures_dir() -> &'static Path {
    Path::new(concat!(env!("CARGO_MANIFEST_DIR"), "/fixtures"))
}
