// Baseline hashes captured 2026-07-20 (RUN-AP-01 red commit). Regenerate
// only when a LATER run legitimately touches the referenced files and
// declares the update in its summary — never silently.
//
// DRYSTONE_HEX rebase-refresh (2026-07-20): the rebase onto main picked
// up RUN-HIST-01's `beta/drystone-spec/EVIDENCE-MAP.md` update (the
// exact §5-rule-5 case — a different run legitimately touched the
// file). RUN-AP-01 itself still did not modify anything under
// `beta/drystone-spec/`, cross-checked by
// `git diff --stat origin/main..HEAD -- beta/drystone-spec/` returning
// empty. Baseline advanced to the post-merge value.
const BASELINE_ATTEST_FOLD_HEX: &str =
    "c76d56fff92c6aa0fc22448bfa36d8c5f9e0814742f4c6b7a71087b4f22d7db0";
const BASELINE_ATTEST_TYPES_HEX: &str =
    "6622e10917b4cb5330638812b5e2837dfd233748e4ff09a28ea54605a81c996c";
const BASELINE_DRYSTONE_HEX: &str =
    "33b48d7cf356811c85a734281dbd8bda8f371e483e17d04a0aab7b242beb0bc4";
