# Freeze notice — paste this at the top of the standalone `experiments/` repo README

`This file is a staging convenience: it is the banner to apply to the OLD standalone` experiments
`repo so anyone landing there is redirected here. I cannot push to that repo from this session (it is
not in scope), so apply it there yourself, then this file can be deleted.`

---

```markdown
> ## ⛔ FROZEN — moved into `discovery`
>
> **As of 2026-07-13 this repo is frozen (read-only, archived).** The entire experiment corpus now
> lives in the `discovery` repo at **`discovery/alpha/experiments/`**, so discovery and
> experimentation stay co-located. All new experiment work, backlog updates, and the
> spec-divergence register happen there.
>
> - Import commit in discovery: `d52ed6f` (source here: `c17b8c8`)
> - Bridge to the spec: `discovery/alpha/experiments/SPEC-ALIGNMENT-AND-ACTION-PLAN.md`
> - What's proven / pending / missing: `discovery/alpha/experiments/SPEC2-OVERLAY.md`
>
> Nothing below is maintained. Do not open PRs against this repo.
```

## Recommended archival steps (on the standalone repo)

1. Apply the banner above to the repo's root `README.md`.
2. Set the GitHub repo to **Archived** (Settings → Danger Zone → Archive) so it is read-only and
   visibly frozen in the org listing.
3. (Optional) Add a final commit `chore: freeze — folded into discovery/alpha/experiments` so the
   git log records the handoff.

If you'd rather I do the archival commit, add the standalone `experiments` repo to this session and I
can prepare the banner change on its default branch.
