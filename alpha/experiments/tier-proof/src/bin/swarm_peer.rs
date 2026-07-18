//! The swarm-peer role as a standalone PROCESS (RUN-16 §A.7, P8).
//!
//! Exchanges envelopes over the transport trait. Here that transport is a local
//! TCP socket (`SPEC-DELTA[run17-swarm-local | declared-stand-in]`); a real
//! two-peer iroh exchange would upgrade the tag but is not required. Same store
//! protocol as the DS, a different process.
use tier_proof::roles;

fn main() -> std::io::Result<()> {
    let (listener, port) = roles::bind_ephemeral()?;
    println!("PORT {port}");
    use std::io::Write;
    std::io::stdout().flush()?;
    roles::serve_store(&listener);
    Ok(())
}
