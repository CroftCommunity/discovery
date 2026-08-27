# History rewrites: why old SHAs are invalid, and what a future rewrite must do

**This repo's git history has been rewritten and force-pushed twice.** That has two
consequences every session needs, which is why this is committed here rather than living
in one machine's agent memory (where it sat until 2026-08-26, unversioned and invisible
to peers — `CroftC/.claude/COORDINATION.md` § Knowledge placement).

## Do not reference old commit SHAs

| Date | Scope | Effect |
|---|---|---|
| **2026-07-27** | `discovery` and `experiments` | Entire histories rewritten (`git-filter-repo`) and force-pushed. **Every SHA from before this date is invalid.** `discovery` took two force-pushes that day. |
| **2026-08-06** | `discovery` only | Second rewrite over 630 commits / 45 branches. 10 branch tips changed; the other 34 were untouched. |

So: a SHA cited in an older plan, doc, PR description or handoff prompt **may not
resolve, and may resolve to something unrelated**. Verify before relying on one, and
prefer naming a branch, a tag, or a file-and-date over a bare SHA in anything durable.

Old commits may remain fetchable by SHA on GitHub through cached PR refs until GitHub
garbage-collects them; that is a leftover, not a source of truth. Pre-rewrite backup
bundles were deleted at the owner's request, so there is no local archive to fall back
on.

*What was rewritten and the exact substitution list are deliberately NOT recorded here:
the whole purpose of the rewrite was to purge a set of strings from this history, and
writing them down would put them straight back. That detail is the standing exception to
the knowledge-placement rule and stays in operator-local memory.*

## Lessons for any future rewrite

Each of these cost a re-run when it was learned. They are general `filter-repo`
discipline and apply to any repo, not just this one.

- **`filter-repo` needs its own fresh clone.** A git worktree **shares the object
  store** and is therefore not isolation — this looks like a safe sandbox and is not.
- **Resync every other checkout afterwards** (`git reset --hard origin/<branch>`), or
  one of them will happily re-push the old history and undo the rewrite. Check
  `git worktree list`, not just the primary checkout.
- **Local-only branches are missed** by a remote-driven rewrite. Enumerate `refs/heads`
  for branches with no upstream and rebase or drop them explicitly.
- **Blanket token replacement leaves clumsy prose** ("a an unrelated work repo"). Let
  the blanket rule cover historical blobs only, and hand-write the replacement on the
  branch that actually matters.
- **Verify against a fresh clone of the published remote**, across every blob, commit
  message, and author/committer field — not against your working copy, which may be
  stale or may be the thing that is wrong.
- **Expect benign residuals and record them**, so the next audit does not re-litigate
  them. Two here that are *not* what they look like: an SVG path coordinate in
  `beta/drystone-spec/superseded/drystone-catchup-flow.svg` that matches a search
  pattern by coincidence, and a generic industry phrase in
  `alpha/thinking/open-considerations.md`.

A rewrite is not done when the force-push succeeds. It is done when a fresh clone
verifies clean and every other checkout has been resynced.
