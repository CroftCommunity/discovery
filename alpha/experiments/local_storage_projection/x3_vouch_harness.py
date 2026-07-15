#!/usr/bin/env python3
"""X3 cross-package mutation harness (RUN-07, Phase B).

For each substrate-surviving mutant in `local_storage_projection`'s auth/governance
files, apply the mutation to the real source tree, run the **croft-chat** integration
suite (which links `local_storage_projection` as a path dependency and drives it through
`surface::LocalStore` -> `fold_derived::DerivedFold`), and record whether the mutant is
killed (a croft-chat test fails) or still survives.

This is the automated cross-package sweep the X3 report left as the residual: the two
crates are separate workspaces, so a single `cargo mutants` invocation cannot mutate one
while testing the other. The harness closes that gap in place, with no production-code or
manifest change: it patches the substrate file, runs the consumer suite, reverts.

Usage:
  python3 x3_cross_package_harness.py <survivors.txt> <out.json>

survivors.txt: one mutant header per line, e.g.
  src/fold_derived.rs:517:5: replace rule_change_approval_subject -> [u8; 32] with [0; 32]
"""
import json
import re
import subprocess
import sys
import time
from pathlib import Path

LSP = Path("/home/user/discovery/alpha/experiments/local_storage_projection")
CROFT = Path("/home/user/discovery/alpha/experiments/croft-chat")
SCOPE_FILES = ["src/fold_derived.rs"]
SCOPE_RE = (
    "required_threshold_for_rule_change|count_personae_by_lineage|threshold_met|"
    "is_under_determined|check_authorization|role_ge_admin|role_ge_owner|role_ge_member|"
    "fn author_role|rule_change_approval_subject|act_subject|decode_rule_key|tiebreak|"
    "detect_fork"
)


def build_diff_catalog():
    """Map each mutant header -> its unified diff (normalized for `git apply -p0`).

    Built with `--file` only (no `--re`), so every survivor's diff is present regardless of
    which function it lives in — a few survivors (the NodeCard `upsert_node_*` field deletions)
    fall outside the auth/threshold `--re` scope but are still real substrate survivors.
    """
    args = ["cargo", "mutants", "--list", "--diff"]
    for f in SCOPE_FILES:
        args += ["--file", f]
    out = subprocess.run(args, cwd=LSP, capture_output=True, text=True).stdout
    blocks = re.split(r"(?m)^(?=src/[\w/]+\.rs:\d+:\d+:)", out)
    catalog = {}
    for b in blocks:
        b = b.rstrip()
        if not b.strip():
            continue
        header = b.splitlines()[0].strip()
        idx = b.find("--- ")
        if idx < 0:
            continue
        diff_lines = b[idx:].splitlines()
        # header line "--- src/foo.rs"; force the +++ line to the same path so -p0 applies
        srcpath = diff_lines[0][4:].strip()
        diff_lines[1] = "+++ " + srcpath
        catalog[header] = ("\n".join(diff_lines) + "\n", srcpath)
    return catalog


def run_croft():
    """Return (killed: bool, reason: str). killed == a croft-chat test failed to compile/pass.

    PROPTEST_CASES=8 mirrors the substrate sweep config (`.cargo/mutants.toml`): the
    deterministic governance behavior tests (mutual_expulsion, rulechange_threshold_enforced,
    contradiction, competing_quorums, role_thrash, removed_then_included, ...) do the killing;
    the property tests only need enough cases to trip, not their full default budget.
    """
    import os
    env = dict(os.environ, PROPTEST_CASES="8")
    r = subprocess.run(
        ["cargo", "test", "-p", "croft-chat", "-q"],
        cwd=CROFT, capture_output=True, text=True, env=env,
    )
    killed = r.returncode != 0
    tail = (r.stdout + r.stderr)
    reason = ""
    if killed:
        # A test failure is the kill we want. Detect it FIRST — cargo prints
        # "error: test failed" on a failing test, which must not be mistaken for a
        # compile error. Only a genuine rustc diagnostic (error[EXXXX]) is a build break.
        fails = re.findall(r"test (\S+) \.\.\. FAILED", tail)
        if fails:
            reason = f"{len(fails)} test(s) FAILED: " + ",".join(sorted(set(fails))[:6])
        elif re.search(r"error\[E\d+\]", tail):
            reason = "consumer build failed (rustc diagnostic)"
        else:
            reason = "nonzero exit"
    return killed, reason


def main():
    survivors_path, out_path = sys.argv[1], sys.argv[2]
    survivors = [ln.strip() for ln in Path(survivors_path).read_text().splitlines() if ln.strip()]
    catalog = build_diff_catalog()
    results = []
    t0 = time.time()
    for i, header in enumerate(survivors, 1):
        entry = {"mutant": header}
        if header not in catalog:
            entry["disposition"] = "ERROR: no diff in catalog (scope mismatch)"
            results.append(entry)
            print(f"[{i}/{len(survivors)}] MISSING {header}", flush=True)
            continue
        diff, srcpath = catalog[header]
        patch = LSP / "_mutant.patch"
        patch.write_text(diff)
        ap = subprocess.run(["git", "apply", "-p0", str(patch)], cwd=LSP,
                            capture_output=True, text=True)
        if ap.returncode != 0:
            entry["disposition"] = "ERROR: patch did not apply"
            entry["detail"] = ap.stderr.strip()
            results.append(entry)
            subprocess.run(["git", "checkout", "--", srcpath], cwd=LSP)
            print(f"[{i}/{len(survivors)}] APPLYFAIL {header}", flush=True)
            continue
        s = time.time()
        killed, reason = run_croft()
        dt = time.time() - s
        subprocess.run(["git", "checkout", "--", srcpath], cwd=LSP)
        entry["croft_killed"] = killed
        entry["reason"] = reason
        entry["secs"] = round(dt, 1)
        entry["disposition"] = "KILLED cross-package" if killed else "SURVIVES croft-chat"
        results.append(entry)
        print(f"[{i}/{len(survivors)}] {'KILL' if killed else 'SURV'} ({dt:.0f}s) {header}", flush=True)
    patch = LSP / "_mutant.patch"
    if patch.exists():
        patch.unlink()
    summary = {
        "total": len(survivors),
        "killed": sum(1 for r in results if r.get("croft_killed")),
        "survived": sum(1 for r in results if r.get("croft_killed") is False),
        "errors": sum(1 for r in results if "ERROR" in r.get("disposition", "")),
        "wall_secs": round(time.time() - t0, 1),
        "results": results,
    }
    Path(out_path).write_text(json.dumps(summary, indent=2))
    print(f"\nDONE killed={summary['killed']} survived={summary['survived']} "
          f"errors={summary['errors']} wall={summary['wall_secs']}s")


if __name__ == "__main__":
    main()
