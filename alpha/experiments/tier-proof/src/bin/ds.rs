//! The web-native DS role as a standalone PROCESS (RUN-16 §A.7, P8).
//!
//! HTTP/socket serve of backplane scopes. Here it speaks the shared
//! newline-delimited socket protocol (guardrail 6: plain sockets stand in for
//! WebTransport, which is a Phase-2/product concern). It is deliberately its own
//! binary so `make roles-up` co-hosts three distinct processes.
use tier_proof::roles;

fn main() -> std::io::Result<()> {
    let (listener, port) = roles::bind_ephemeral()?;
    println!("PORT {port}");
    // Ensure the port line reaches the parent before we block on accept.
    use std::io::Write;
    std::io::stdout().flush()?;
    roles::serve_store(&listener);
    Ok(())
}
