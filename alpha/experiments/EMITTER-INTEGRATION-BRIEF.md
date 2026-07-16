# Emitter-integration brief — wiring the over-the-wire key source into the frozen conformance emitter

`Serves: the RUN-08 §1B residual — the mls-welcome-over-iroh spike PROVED welcome-over-iroh
key-distribution sourcing green-real at loopback grade, but it is NOT wired into the frozen Proofs
conformance emitter (whose registry is in-process). This brief lays out three ways to close that gap
and recommends one. A decision brief; the decision is the owner's.`

`Class: read-and-report decision brief (RUN-10 Part 3). No spec, register, crate, or frozen record is
edited by this file. It proposes; it decides nothing (I9 firewall: a read decides nothing).`

`Reads: alpha/experiments/RUN-08-SUMMARY.md §1B; alpha/Proofs/FROZEN-NOTICE.md;
alpha/Proofs/lineage-groups/crates/conformance/{Cargo.toml,src/lib.rs,src/bin/emit_vectors.rs};
alpha/experiments/iroh/crates/mls-welcome-over-iroh/src/main.rs;
alpha/experiments/iroh/relay-lab-runs/C-mls-welcome-2026-07-15-run08/{manifest,verdict,conformance-suite-reprove};
beta/drystone-spec/part-2-certifiable-design.md §9 + §10.5; beta/drystone-spec/EVIDENCE-MAP.md rows
9/10.5/(a)/(b); alpha/experiments/EXPERIMENT-BACKLOG.md §6d + §6d-i.`

> **Decided (owner, 2026-07-15; annotation added RUN-11): Option C — defer to the `[gates-release]`
> pass.** Option B (the thin adapter crate outside Proofs) remains the fallback if early closure is
> wanted; Option A is not adopted. The §10.5 residual line stays as-is (the emitter-integration remains
> the named residual, mechanism `Verified` at loopback, vectors 66/0). The emitter integration is thus
> **formally deferred by decision** and stays on the parked list. The recommendation reasoning in §4
> below is unchanged; this banner records the owner's ruling.

---

## 1. The gap, precisely

RUN-08 §1B landed the **key-distribution half** of the conformance-suite honesty boundary and left
one thing open, in its own words:

> "**no iroh was wired into the frozen conformance emitter** (beyond emission plumbing); the
> emitter-integration is recorded as the residual." (`RUN-08-SUMMARY.md` §1B)

What was proven, and where it lives:

- **The mechanism is green-real (loopback).** `alpha/experiments/iroh/crates/mls-welcome-over-iroh`
  builds a **real** openmls group and sends a **real 1466-byte Welcome** across a **real iroh
  connection** homed on a real (loopback) relay; the joiner runs `join_from_welcome` and derives the
  **identical** MLS exporter secret (`epoch_secret_match: true`) **and** the identical lineage fold
  (`alice`'s two devices → one actor; `bob` → one; `fold_matches: true`). The spike's own verdict:
  the "verifying-key/standing registry the conformance vectors rest on is now demonstrably sourceable
  from a real over-the-wire MLS group rather than a modeled fixture"
  (`relay-lab-runs/C-mls-welcome-2026-07-15-run08/verdict.json`; source `main.rs`).

- **The frozen emitter sources that same registry in-process.** In
  `conformance/src/bin/emit_vectors.rs`, the standing/verifying-key world is built by hand in-process
  — e.g. `SigningIdentity::from_seed(...)` seeded into a `Directory`, and the revoke-authority world
  in `emit_revocation_authority()` — never from a wire-delivered Welcome. The emitter's discipline is
  the load-bearing constraint on any change here:

  > "The vector *values* are never hand-written… no cryptographic constant is typed by hand — the
  > emitter computes signatures, hashes, and keys from the real code" (`conformance/src/lib.rs`)

  > "the emitter fails loud rather than writing a vector it cannot stand behind."
  > (`emit_vectors.rs`, the cat-7 derive-then-record loop)

- **The gap is a *provenance nicety*, not a correctness hole.** The full suite is emitted and
  re-proven **66/0 across cats 1–9** in-environment (`part-2-certifiable-design.md` §10.5, RUN-08),
  and the over-the-wire evidence is captured **alongside** the suite in the same relay-lab-run bundle
  (`conformance-suite-reprove.txt` co-located with `verdict.json`). What is *not* done is having the
  emitter itself *derive its registry from the wire* rather than in-process. The registry it uses is
  provably sourceable over the wire; the emitter simply does not do the sourcing.

What is explicitly **out of scope / firewalled** (do not conflate it with this residual): the
**threshold-revoke-over-wire + co-sign-vs-vote authority ordering** half — piece (b) — stays gated on
the revocation-authority trust model (I9), today an MD-G5 sha-256 MAC stand-in
(`part-2-certifiable-design.md` §10.5 (b); `EVIDENCE-MAP.md` row `10.5 (b)`; `EXPERIMENT-BACKLOG.md`
§6d-i). This brief concerns piece (a) only.

## 2. What the freeze protects (the thing every option is weighed against)

`alpha/Proofs/FROZEN-NOTICE.md` makes the Proofs corpus **"frozen, canonical"**, folded in at commit
`b3ecf8f` (owner-authorized, RUN-08):

> "**Frozen / read-only.** New proof work is not expected to land here as a matter of course; this is
> a canonical snapshot folded in to make the experiment spikes buildable in-repo and to give the
> conformance-core a home discovery can build against."

The freeze buys three things:

1. **Provenance.** The emitter is a byte-identical export of the reference conformance-core; its
   vectors carry a `MANIFEST.json` of per-file sha256 and re-prove against the real API. "Canonical
   snapshot" means an implementer can trust the emitted vectors *because* the tree that produced them
   is unchanged from the origin of record.
2. **The reproducibility guarantee.** RUN-08's welcome run is a "**byte-identical** reproduction" of
   the 2026-06-17 archive precisely because nothing in the frozen tree moved. Editing the emitter
   forfeits that byte-identity claim for the emitter's own outputs.
3. **The firewall statement.** The notice records that `lineage-groups` + `conformance` "do not touch
   the I9 identity/key-recovery trust tier." An edit that reached toward the authority half would
   erode that clean-of-firewall property.

Breaking the freeze is therefore not free even when the diff is small: it converts a *canonical
record* into a *maintained artifact*, which is a different and lower-value thing.

## 3. The three options

### (A) Fork-and-extend the frozen emitter inside `alpha/Proofs/`

Add, inside `conformance/src/bin/emit_vectors.rs` (or a sibling emitter), a path that sources the
verifying-key/standing registry from a real over-the-wire Welcome (reusing the spike's
`Device`/`join_from_welcome` interface) instead of the in-process `SigningIdentity::from_seed` world,
then emit the standing-dependent vectors against that wire-sourced registry. **One suite; the
provenance lives in one place.**

- **Cost.** A real code change to a frozen crate: new iroh + `lineage-mls` deps on the `conformance`
  crate (today it depends only on `lineage-core`, `lineage-history`, serde, sha2, hex — see
  `Cargo.toml`), an async emitter path, and a re-freeze/re-snapshot afterward.
- **Risk (the load-bearing one).** It **breaks the frozen notice.** The moment the canonical tree is
  edited, the "byte-identical snapshot" guarantee (§2.1–§2.2) is gone: the emitter is no longer the
  export of record, and the 2026-06-17 byte-identity reproduction can no longer be asserted against a
  moving tree. For a **loopback-grade** improvement (real-NAT remains X1) that buys *provenance
  polish on a registry already provably wire-sourceable*, that is a large, mostly-irreversible cost
  for a small, cosmetic gain. It also invites scope-creep toward the firewalled authority half, since
  once the emitter speaks iroh the (b) piece looks "one more function away" — exactly the seam the
  firewall exists to hold.

### (B) A thin adapter crate OUTSIDE Proofs that drives the frozen emitter and adds the sourcing evidence alongside

A new crate under `alpha/experiments/` (the natural home is beside the spike, e.g.
`alpha/experiments/iroh/crates/`) that: (i) runs the spike to produce the over-the-wire sourcing
verdict, (ii) drives the frozen `run-vectors` / `emit-vectors` binaries unchanged, and (iii) asserts
the two agree — emitting a combined "registry sourced over the wire **and** the suite re-proves 66/0
against it" artifact. **Frozen preserved; two artifacts to keep in sync.**

- **Cost.** A second artifact and the discipline to keep it aligned with the frozen emitter: if the
  emitter's registry shape ever changes, the adapter's wire-sourcing must track it. Drift risk is
  real but *bounded* — the adapter drives the frozen binaries rather than re-implementing them, so
  the only sync surface is the registry interface, not the vector logic.
- **Risk.** Low correctness risk (it cannot corrupt the canonical vectors — it only reads them), but
  it is **two things that can disagree**, and a green adapter over a stale emitter would be a
  false comfort. Someone must own the "re-run both, assert equal" gate.
- **Note — this is ~80% already done.** RUN-08 already realized (i)+(ii) informally: the relay-lab-run
  bundle `C-mls-welcome-2026-07-15-run08/` co-locates the welcome `verdict.json` with the 66/0
  `conformance-suite-reprove.txt`. Option (B) is the act of *hardening that co-location into a crate
  that asserts the agreement*, rather than leaving it as two files a human reads side by side.

### (C) Defer to the `[gates-release]` pass, where the governance-resolution vectors join the suite anyway

Do nothing now; let the residual stand as recorded. The spec already states that a **future,
freeze-breaking suite extension is coming regardless**:

> "the suite covers the §4, §5, and §6 proven layer, while the **§7.3 through §7.5
> governance-resolution vectors depend on the `[gates-release]` encodings in Appendix B and are not
> yet in the suite.**" (`part-2-certifiable-design.md` §9)

That pass necessarily edits the emitter (it adds whole new vector categories once the Appendix B
byte encodings are pinned), and it necessarily does so under a *deliberate, release-grade*
freeze-break rather than an ad-hoc one. Wiring the over-the-wire registry source can ride that same
pass, when the emitter is already open on the bench and the byte encodings the wire form would need
are finally pinned.

- **Cost.** **Zero work now.** The residual remains open until `[gates-release]`.
- **Risk / "is anything blocked?"** Nothing is blocked meanwhile. The mechanism is `Verified` at
  loopback (`EVIDENCE-MAP.md` row `10.5 (a)`), all vectors are green **66/0** (`EVIDENCE-MAP.md`
  row `9 / 10.5`), and the §10.5 footnote is already reconciled to say exactly this — that the
  emitter-integration is the residual and the mechanism is proven. The only thing "open" is the
  cosmetic provenance point of §1. The residual is tracked in `EXPERIMENT-BACKLOG.md` §6d and sits at
  execution-order item 5 ("MLS-welcome-over-iroh wired into conformance emission… the spike exists;
  emission is the remaining integration").

## 4. Recommendation — **(C), defer to `[gates-release]`**, with **(B)** as the fallback if the owner wants it formalized sooner

The decision is the owner's (I9 firewall: this brief is a read; it decides nothing). With that said,
the reasoning points one way:

- **(A) breaks a canonical guarantee for a cosmetic, loopback-grade gain.** The registry is already
  *provably* wire-sourceable (the spike proves it); having the emitter *do* the sourcing changes no
  vector value and no verdict. Trading the "frozen, canonical snapshot" property — and its
  byte-identical reproducibility, and its clean-of-firewall statement — for that is a bad trade, and
  it puts the emitter one function away from the firewalled authority half.
- **(C) costs nothing and blocks nothing, and the work it defers is work a freeze-break is already
  scheduled to do.** The §7.3–§7.5 governance-resolution vectors *must* re-open the emitter at
  `[gates-release]`; folding the wire-sourcing into that already-planned, deliberate freeze-break is
  strictly cheaper and cleaner than forcing an unplanned one now. Deferring keeps the freeze intact
  through the interval where it is most valuable (while implementers are trusting the canonical
  snapshot) and pays the integration cost once, at the point where an over-the-wire *vector shape*
  could actually be encoded (its byte form is itself a `[gates-release]` item).
- **(B) is the right move only if the owner wants the sourcing formalized before `[gates-release]`**
  — e.g. to retire the residual from the backlog sooner, or to convert the informal RUN-08
  side-by-side (§3B note) into an asserting crate. It preserves the freeze, and because RUN-08
  already produced the two co-located artifacts, the marginal cost is low. It is the *safe* way to
  close the residual early; (C) is the *cheapest* way, and nothing is lost by waiting.

**Recommended: (C) now; (B) if early closure is wanted; (A) not advised.** The residual is genuinely
low-stakes — mechanism `Verified` (loopback), vectors 66/0, footnote reconciled — so the default of
"leave the canonical record frozen and let the scheduled freeze-break absorb the wiring" is the
disciplined choice.

## 5. Explicit non-actions (firewall)

This brief authors no code, edits no spec/register/frozen record, and moves no status tag. It does
**not** touch piece (b) (threshold-revoke-over-wire / co-sign-vs-vote), which stays gated on the I9
revocation-authority trust model. The proposed shared-file edits that would *follow* an owner
decision (e.g. an `EXPERIMENT-BACKLOG.md` §6d status change, or an `EVIDENCE-MAP.md` row-`10.5 (a)`
note) are reported to the caller for the owner to make, not made here.
