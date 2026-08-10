//! **S8 — object sizes against the 2 MiB cap.** The measurement most likely to change the design.
//!
//! CISS refuses any object over `MAX_OBJECT_BYTES = 2 MiB`, on both put and get, and the HTTP
//! boundary independently caps bodies at the same figure. **The cap is load-bearing** — it came
//! from a real memory-exhaustion finding in the 2026-08-03 security review — so the question is
//! never "raise it", it is "where do real MLS objects cross it".
//!
//! Fidelity: **Rung A (real-lib)**. Run release: `cargo test --release --test s8_object_sizes -- --nocapture`
//!
//! # What was known going in, and what was not
//!
//! - `mls-replant` measured a **sparse self-update commit at O(N)**, ~80–130 B/member — so the
//!   spike spec's "commit ~log N" row was already suspect.
//! - Every prior Welcome figure in the corpus (~152–155 B/member) is the **tree-extension-OFF**
//!   case, because `mls_replant::stamp` hardcodes `MlsGroupCreateConfig::default()`. The O(N)
//!   object the question is actually about had **never been measured** before Phase 0's D7.
//!
//! So the with-extension rows here are a **first measurement**, not a confirmation.

use meer_queue::ciss_harness::MAX_OBJECT_BYTES;
use meer_queue::mls::{self, measure_group, GroupObjects};

/// Report a row and say whether anything on it crosses the cap.
fn row(o: &GroupObjects) -> bool {
    let gi = o.group_info.map_or("-".to_string(), |b| b.to_string());
    let per = |b: usize| {
        if o.n > 1 {
            b as f64 / (o.n - 1) as f64
        } else {
            0.0
        }
    };
    println!(
        "{:>6} {:>4} {:>11} {:>11} {:>11} {:>11} {:>11} {:>10} {:>8.1}",
        o.n,
        if o.tree_ext { "ON" } else { "off" },
        o.app_message,
        o.commit_add,
        o.commit_update,
        o.commit_remove,
        o.welcome,
        gi,
        per(o.welcome),
    );
    let biggest = o
        .welcome
        .max(o.commit_add)
        .max(o.group_info.unwrap_or(0));
    biggest > MAX_OBJECT_BYTES
}

/// **Release-only.** Ignored by default: the N = 8000 rungs are ~50 s in release and minutes in
/// debug, which would make the ordinary suite unusable. Run it deliberately:
///
/// ```text
/// cargo test --release --test s8_object_sizes -- --ignored --nocapture --test-threads=1
/// ```
#[test]
#[ignore = "release-only sweep; see the doc comment for the command"]
fn object_sizes_against_the_two_mib_cap() {
    meer_queue::init_tracing();
    println!(
        "S8 — object sizes vs MAX_OBJECT_BYTES = {} bytes (2 MiB). [{}]",
        MAX_OBJECT_BYTES,
        mls::resolved_versions()
    );
    println!(
        "{:>6} {:>4} {:>11} {:>11} {:>11} {:>11} {:>11} {:>10} {:>8}",
        "N", "ext", "app_msg", "commit_add", "cmt_update", "cmt_remove", "welcome", "grp_info", "wel B/mbr"
    );

    let mut crossed_at: Option<(usize, bool)> = None;
    // Grows until something crosses the cap or the harness gives out. Open Question 3
    // pre-authorised the full sweep with no time ceiling, so nothing here is truncated for
    // convenience — if it stops, the reason is reported.
    for n in [2usize, 10, 50, 200, 500, 1000, 2000, 4000, 8000] {
        for ext in [false, true] {
            let o = measure_group(n, ext);

            // The wiring assertions, checked at every N rather than once.
            assert!(
                o.app_message < MAX_OBJECT_BYTES,
                "an application message must never approach the cap (N={n}): {} bytes",
                o.app_message
            );
            if ext {
                assert!(
                    o.group_info.is_some(),
                    "the tree extension must actually be exercised (N={n})"
                );
            } else {
                assert!(
                    o.group_info.is_none(),
                    "without the extension there is no GroupInfo to carry (N={n})"
                );
            }

            if row(&o) && crossed_at.is_none() {
                crossed_at = Some((n, ext));
            }
        }
        if crossed_at.is_some() {
            println!("  (stopping: the cap has been crossed)");
            break;
        }
    }

    match crossed_at {
        Some((n, ext)) => println!(
            "\nS8 MEASURED (real-lib): the 2 MiB cap is first crossed at N = {n} with the ratchet-tree \
             extension {}.",
            if ext { "ON" } else { "OFF" }
        ),
        None => println!(
            "\nS8 MEASURED (real-lib): nothing crossed 2 MiB within the tested range (max N = 8000)."
        ),
    }
}

/// The `Welcome`-with-k-joiners row: `O(N) + k`, measured separately because it varies in two
/// dimensions and the table above holds k at N-1.
/// Release-only for the same reason; see the sweep above.
#[test]
#[ignore = "release-only sweep; see the doc comment for the command"]
fn welcome_grows_with_both_group_size_and_joiner_count() {
    println!("S8 — Welcome vs joiner count (tree extension ON)");
    println!("{:>6} {:>8} {:>12} {:>12}", "N", "joiners", "welcome B", "B/joiner");
    for (n, _label) in [(200usize, "k=N-1")] {
        let o = measure_group(n, true);
        let k = n - 1;
        println!(
            "{:>6} {:>8} {:>12} {:>12.1}",
            n,
            k,
            o.welcome,
            o.welcome as f64 / k as f64
        );
    }
    println!(
        "S8 NOTE: in this harness every non-planter is a joiner, so k = N-1 and the two dimensions \
         are not independently varied. Separating them needs incremental adds and is left to the \
         substrate work, not the spike."
    );
}
