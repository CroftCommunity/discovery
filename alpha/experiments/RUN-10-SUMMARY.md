# RUN-10 — Publish the specs (Pages + followable cross-references) + three briefs

`Status: complete. Part 1 (site + broken-ref gate) built and verified in-environment; Parts 2–4 are
read-and-report briefs. No spec, register, crate, or frozen record was edited this run — only new site
files, three briefs, two backlog rows, findings entries, and the README/MASTER-INDEX pointers the run
sanctions. I9 firewall held: Parts 2–4 are reads and decide no trust tier.`

`Branch: claude/run-10-publish-briefs-i7p43l (off main). Parallel-safe with RUN-09 by design — adds only
new files and edits no spec/register/crate. Rebase on main after RUN-09 merges and re-run the build gate.`

---

## Part 1 — the spec site (GitHub Pages)

### Tooling chosen, and why

**A single Python build script + one unit-tested resolver module + `actions/deploy-pages`.** No
framework. The three load-bearing requirements — stable anchors *from section numbers* (not title text),
reference autolinking that skips code spans, and a broken-reference gate — are custom logic that a
framework would need a hook for anyway, so the boring path (transparent, directly testable) wins. The only
dependency is `markdown==3.7` (pinned); the resolver is pure standard library, so its unit tests are
hermetic. Python is already a repo toolchain language.

- `site/resolver.py` — section-number → stable anchor, reference autolinking, RFC/BCP-vs-Drystone
  discrimination, the gate. Pure, stdlib-only.
- `site/test_resolver.py` — 26 unit tests (the resolver contract).
- `site/build.py` — render (markdown→HTML), inject anchors, autolink, run the gate, emit site + landing index.
- `.github/workflows/pages.yml` — deploy on push to `main`; run the **same build (hence the gate)** on PRs without deploying.
- `site/requirements.txt` (pinned), `site/README.md` (the contract).

Sources stay **canonical**: zero edits to the markdown; every anchor and link is generated at build time.

### Resolver: red → green (TDD)

Tests written first. Initial run (no `resolver.py`): `ModuleNotFoundError` — 1 error, red. After
implementing the resolver: **26 passed**. The suite covers exactly the run's required cases — in-doc ref,
cross-doc ref, a ref to a missing section (fails with the offender named), an appendix ref, a ref inside a
code span (not linkified) — plus R-bullet anchoring, repo-path (published vs unpublished), and RFC/BCP
external-citation discrimination.

```
$ python3 -m unittest test_resolver
Ran 26 tests in 0.003s — OK
```

### Build stats (in-environment)

```
documents built            : 72  (8 spec, 64 exploratory)
headings anchored          : 1057
§-references found          : 3039
  resolved -> links         : 2946
  external (RFC/BCP) literal : 86     (RFC 9420/9750/8446/8126 §-citations — also use §)
  skipped in code spans      : 84     (code examples — never linkified)
  unresolved                 : 7      (hard-gated 0, companion 7)
repo-path citation links   : 8
```

Independent post-build integrity audit of the emitted HTML: **1086 cross-file anchor links, 0 broken;
0 dangling same-file anchors.** Every followable cross-reference lands on a real id.

**The gate passes on day one (0 hard-gated unresolved).** The 7 companion/exploratory unresolved are
benign and non-fatal: references into *unpublished* sibling docs (`COHESION §23/§29/§36`,
`doc-writing-method §12`, `ROADMAP §13`, `social-layer.md §75`) and one RFC ref whose block does not
literally say "RFC" (`part-2-changelog §8.7`). None is a broken Drystone spec reference. The gate hard-fails
only on Part 1 / Part 2 (where § numbering is self-contained and RUN-08 verified all Part 2 refs resolve).

**Key resolver subtlety, recorded:** Part 1/Part 2 use `§N.N` for **both** Drystone sections **and** RFC
sections (`RFC 9420 §16.4`, `RFC 8126 (BCP 26), §5.2`). A naive resolver flags every RFC citation as a
broken Drystone ref (35 false positives on the first build). The resolver discriminates them by (a)
adjacency to an `RFC ####` / `BCP ##` token and (b) a per-doc set of §-numbers cited in RFC-context blocks
that no Drystone heading defines. A genuine renamed-section break (in non-RFC prose) still fails the gate.

### Five sample links (section → target), verified in the emitted HTML

| # | Kind | In source (Part 2) | Rendered link |
|---|------|--------------------|---------------|
| 1 | in-document | `§7.6` | `<a href="#s7-6">` |
| 2 | cross-document | `Part 1 §2.5` | `<a href="part-1.html#s2-5">` |
| 3 | appendix | `Appendix B` | `<a href="#appendix-b">` |
| 4 | R-bullet (list-item anchor) | `§7.2 R7` | `<a href="#s7-2-r7">` → real `<li id="s7-2-r7">` |
| 5 | repo-path (in a code span, published) | `` `alpha/thinking/reconciliation-horizon.md` `` | `<a href="thinking-reconciliation-horizon.html"><code>…</code></a>` |

Status tags render **literally** as content: `Verified`, `[gates-release]`, `Realizes: P-Peer-Equality`
render as monospace (source backticks preserved), never reinterpreted or iconified. An *unpublished*
repo-path (`alpha/experiments/local_storage_projection/X3-AUTOMATED-SWEEP.md`) is correctly left literal,
not linked. Visually verified by screenshot (landing index + Part 2 §7.2).

### Published URL

**https://croftcommunity.github.io/discovery/** — live after the workflow deploys on merge to `main`.
One-time owner setup: repo **Settings → Pages → Source: "GitHub Actions"**.

### Scope published (nothing else)

Specification tier — the eight named `beta/drystone-spec/` docs (Part 1, Part 2, EVIDENCE-MAP,
conventions-and-decisions, open-threads, part-2-changelog, the DAG-CBOR primer, the historical
proposed-changes). Exploratory tier — all markdown under `alpha/thinking/` (64 files), labeled
"Exploratory — design dialogue," published because Part 2 cites them by path.

---

## Part 2 — brief: the §5.2 group-principal seam (Meadowcap/Willow × MLS)

**Verdict:** A Meadowcap **communal namespace can carry the Group principal** as §5.2/§5.10 sketch it, and
it is the natural fit — the Group *is* the namespace (identified by genesis hash `H(tag ‖ group_id)`), each
persona is a self-authorizing subspace, read confidentiality is the fold-gated asset key already resolved
in `asset-keying.md`, and authority is a folded revocable Group Role. §5.10's "how does the communal-namespace
key rotate under churn" question **dissolves**: a communal namespace has no shared secret to rotate (write
authority is per-subspace, read is the asset key), so the seam decomposes into two already-owned problems
plus a near-free identifier assignment. Capability rotation composes cleanly with MLS epoch rotation
*because* the Group-principal lives above MLS: MLS rotates keys on Commits, the governance fold rotates
authority, and Meadowcap capabilities are re-issued (never revoked-in-place — Meadowcap has no native
revocation) downstream of the fold. Recommended stance: **primary** (communal namespace at all times). All
Drystone bindings `Design`; no trust tier decided. Brief:
`beta/impl/drystone-design/group-principal-seam.md`. Backlog: §2e row added
(`EXPERIMENT-BACKLOG.md`). Live specs cited: Meadowcap "Final (as of 21.11.2025)", Willow Data Model
"Final", willowprotocol.org, fetched 2026-07-15.

---

## Part 3 — brief: frozen-emitter integration (decision brief for the owner)

**Verdict:** **Recommend Option C — defer to the `[gates-release]` pass — with Option B (a thin adapter
crate outside Proofs) as the fallback if early closure is wanted; Option A is not advised.** The residual is
low-stakes: the `mls-welcome-over-iroh` mechanism is `Verified` at loopback grade, the suite emits and
re-proves 66/0 across cats 1–9, and the §10.5 footnote already names emitter-integration as the residual.
What is un-wired is *provenance polish* (sourcing the verifying-key/standing registry over the wire rather
than in-process), not any vector value or verdict. Option A breaks the "frozen, canonical snapshot"
guarantee for a cosmetic, loopback-grade gain and edges the emitter toward the firewalled authority half —
a bad trade. Option C costs zero and blocks nothing, and the work it defers is work a freeze-break is
*already scheduled* to do (the §7.3–§7.5 governance-resolution vectors re-open the emitter at
`[gates-release]` once Appendix B byte encodings are pinned). The decision is explicitly the owner's (I9).
Brief: `alpha/experiments/EMITTER-INTEGRATION-BRIEF.md`.

---

## Part 4 — brief: croft-group L2 readiness

**Verdict:** croft-group **Layer-2 (MLS/encryption) is reuse-ready for its mechanism half and firewalled
from its authority/projection half.** The proven crates already satisfy — at `Verified`/`green-real`
loopback grade against real openmls 0.8.1, real iroh, real Ed25519 — the frame-sealing prerequisites:
Welcome-based key distribution (EVIDENCE-MAP §10.5(a), `mls-welcome-over-iroh`), membership re-key /
survivor-epoch PCS (§7.6.2, `mls-replant`+`replant-continuity`), credential/lineage identity and
forged-message reject (§4.1–4.6, conformance-core), and the Zeroize recovery-anchor lock (§7.3.9,
`bip39-recovery-roundtrip`). What genuinely waits: the *authority* half (authorized revocation-over-the-wire
and the recovery trust tier, §10.5(b)/§7.3.9) waits on **I9**, and read-scope-under-fork waits on the
**parked resolution-ACL** design. A single non-trivial slice — **L2a, MLS-sealed happy-path frame** —
is buildable now, covering R1–R7 without touching either gate; shaped as a RED-able backlog row (§3 L2a),
not built. Brief: `alpha/experiments/CROFT-GROUP-L2-READINESS.md`.

---

## Findings (all read-derived; no edit this run)

Recorded to `CONSISTENCY-FINDINGS-2026-07.md` as `FND-R10-1..5`:

- **FND-R10-1** (LOW) — §5.10's "communal-namespace key rotates under churn" framing presupposes a shared
  whole-namespace key a Meadowcap communal namespace does not have (not a contradiction; a reframe recommended).
- **FND-R10-2** (LOW) — Meadowcap's only revocation workaround is owned-namespace-specific, structurally
  unavailable to a communal Group-principal; revocation MUST come from the governance fold (consistent; recorded).
- **FND-R10-3** (LOW) — the emitter residual is already ~80% realized as the brief's Option B, informally
  (two co-located RUN-08 artifacts); strengthens B as the fallback.
- **FND-R10-4** (LOW/MED) — `alpha/Proofs/FROZEN-NOTICE.md` says the conformance-core "emits categories
  **1–6**" but the emitter and §10.5 confirm **1–9**; a doc-lag *inside the frozen record* (not editable
  this run — recorded for the owner to fix at the next freeze-break). Verified against both files this run.
- **FND-R10-5** (LOW) — two layers share the label "Layer 2": croft-group L2 (MLS) vs the parked
  "Layer-2 resolution-ACL" (which is croft-group L3); the correction is that L2=MLS does not depend on the parked design.

The **broken-ref gate passed clean on day one** — the RUN-08 known-good baseline held (no Part 1/Part 2
discrepancy list, so no gate FINDING).

---

## Files added / edited

**Added (site tooling):** `site/build.py`, `site/resolver.py`, `site/test_resolver.py`,
`site/requirements.txt`, `site/README.md`, `site/.gitignore`, `.github/workflows/pages.yml`.

**Added (briefs):** `beta/impl/drystone-design/group-principal-seam.md`,
`alpha/experiments/EMITTER-INTEGRATION-BRIEF.md`, `alpha/experiments/CROFT-GROUP-L2-READINESS.md`, and this
summary.

**Edited (sanctioned by the run):** `EXPERIMENT-BACKLOG.md` (§2e group-principal-seam row; §3 L2a row),
`CONSISTENCY-FINDINGS-2026-07.md` (RUN-10 findings section), `README.md` (published-URL line),
`alpha/experiments/MASTER-INDEX.md` (site + gate note). No spec, register, crate, or frozen record touched.
