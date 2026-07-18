//! The history-convergence node as a standalone PROCESS (RUN-16 §A.7, P8).
//!
//! Receives envelope sets from BOTH the DS's store and the swarm peer and
//! reconciles by envelope hash (a hash-set diff standing in for the RUN-12 RBSR
//! construction, `SPEC-DELTA[run17-rbsr | declared-stand-in]`), then serves
//! backfill. Its own binary, co-hosted with the other two.
use tier_proof::roles;

fn main() -> std::io::Result<()> {
    let (listener, port) = roles::bind_ephemeral()?;
    println!("PORT {port}");
    use std::io::Write;
    std::io::stdout().flush()?;
    roles::serve_convergence(&listener);
    Ok(())
}
