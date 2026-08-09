//! **D1 probe** — does a cross-repo path dependency from `discovery/` to `CISS/` build,
//! and are `App` / `Blobs` / `Db` reachable from outside the crate?
//!
//! Disposition: `promote` — becomes `src/ciss_harness.rs` in Phase 1.
//!
//! Success criteria (plan Phase 0 D1): a binary that constructs an `App` and calls
//! `.router()` compiles. Or a named compile error telling us CISS must be reached
//! another way.

use ciss::server::{App, Blobs, Db, Limits};

fn main() {
    // Build with `with_limits`, NOT `new` — `App::new` calls `Limits::from_env()` and
    // reads CISS_MAX_STORE_BYTES / CISS_MAX_DID_BYTES from the ambient environment
    // (CISS/src/server.rs:242, :176-188). Pass 3 finding.
    // `Limits` has no `Default`, but its fields are `pub` (server.rs:165-170), so the
    // spike pins them explicitly — which is what we wanted anyway: a fixed, stated
    // ceiling rather than whatever the environment happens to say.
    let dir = tempfile::tempdir().expect("tempdir");
    let limits = Limits {
        store_ceiling: 1024 * 1024 * 1024, // 1 GiB, ample for the spike
        did_cap: None,                     // opportunistic; the spike is single-tenant
    };
    let app = App::with_limits(
        "meer-spike-provider",
        Blobs::Fs(dir.path().to_path_buf()),
        Db::Memory,
        limits,
    )
    .expect("build app");

    let _router = app.router();

    println!("D1 OK: ciss path dep builds; App::with_limits + router() reachable.");
    println!("  provider_id = {}", app.provider_id());
    println!("  blobs       = Blobs::Fs({})", dir.path().display());
}
