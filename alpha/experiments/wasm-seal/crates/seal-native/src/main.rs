//! `seal-peer` — RUN-19 P2 RED skeleton: refuses every op.
//!
//! GREEN turns this into the ndjson peer holding named native `Sealer`s.

fn main() {
    // RED: the native peer is unbuilt; any driver sees an immediate failure.
    eprintln!("seal-peer: unbuilt (P2 red)");
    std::process::exit(1);
}
