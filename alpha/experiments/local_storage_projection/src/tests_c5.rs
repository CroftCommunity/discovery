//! **C5 — ack cost honesty (E112, informative, not gating).**
//!
//! Per-op normative acks are O(N) responses per event: every one of N members answers every op
//! with a HeadAck. C5 measures that volume against N, and against the **scoped alternative** the
//! §7.3.3 solicitation dial permits: explicit acks only for **finality-needing** ops
//! (membership/governance — §7.4's own scope), and **lazy piggyback** otherwise (your next
//! authored fact carries your head, so a non-finality op costs zero extra ack messages).
//!
//! This is a counting model — informative, not a gate. Fidelity: **Modeled / loopback grade**
//! (message *counts*, not a live gossip run; the FANOUT-M1 harness measured real gossip volume
//! shape separately).

#[cfg(test)]
mod c5 {
    /// Explicit-ack volume under the **per-op normative** posture: every op solicits an ack from
    /// each of the N members.
    fn per_op_normative(n: u64, ops: u64) -> u64 {
        n * ops
    }

    /// Explicit-ack volume under the **scoped** posture: only the finality-needing ops solicit acks
    /// (N each); the rest ride the next authored fact (lazy piggyback → zero extra ack messages).
    fn scoped(n: u64, finality_ops: u64) -> u64 {
        n * finality_ops
    }

    #[test]
    fn per_op_acks_are_linear_in_n_and_scoped_saves_the_non_finality_tail() {
        // Equal when every op needs finality (no tail to save).
        assert_eq!(scoped(10, 100), per_op_normative(10, 100), "all-finality: scoped == normative");

        // A realistic mix: of 100 ops, 5 are governance/membership (finality-needing), 95 are
        // ordinary messages (piggyback). Scoped pays acks only for the 5.
        let n = 10;
        let ops = 100;
        let finality = 5;
        let normative = per_op_normative(n, ops); // 1000
        let scoped_vol = scoped(n, finality); // 50
        assert!(scoped_vol < normative, "scoped is cheaper whenever a non-finality tail exists");
        assert_eq!(normative - scoped_vol, n * (ops - finality), "the saving is exactly the tail");

        println!(
            "C5 MEASURED (Modeled): per-op normative acks = N·ops = {normative} for N={n}, ops={ops}; \
             scoped (finality-only + lazy piggyback) = N·finality = {scoped_vol} for finality={finality}. \
             Saving = N·(ops-finality) = {}. Acks are O(N) per solicited op; scoping to finality \
             ops removes the ordinary-message tail entirely (it rides authored facts).",
            normative - scoped_vol
        );
    }

    #[test]
    fn cost_curve_over_n() {
        // The shape, tabulated: per-op normative grows linearly per op with N; scoped tracks only
        // the finality fraction. Informative — no gate.
        let ops = 100;
        let finality = 5;
        println!("C5 curve (Modeled), ops={ops}, finality-needing={finality}:");
        for n in [2u64, 4, 8, 16, 32] {
            let normative = per_op_normative(n, ops);
            let scoped_vol = scoped(n, finality);
            let ratio = normative as f64 / scoped_vol as f64;
            println!(
                "  N={n:>2}: normative={normative:>5} acks, scoped={scoped_vol:>4} acks, \
                 reduction {ratio:>4.1}x",
            );
            assert!(scoped_vol * (ops / finality) == normative,
                "scoped × (ops/finality) reconstructs normative — the tail is the whole difference");
        }
        println!("C5 CONCLUSION (informative): scoping acks to finality-needing ops is a constant \
                  reduction of ops/finality (here 20x) independent of N; both remain O(N) per \
                  solicited op. The dial is a volume lever, not a safety one — safety is C2/C3's \
                  fail-closed freshness gate.");
    }
}
