# Extraction to the standalone repo

This kit is written from day one as the future root of
`CroftCommunity/appview-infra`. Phase 2 begins by extracting it (P2-0).

## What extraction does

`scripts/extract-to-repo.sh <target>` produces the standalone repo content:

- **root = kit contents.** Everything under `kit/` (Makefile, CONTRACT.md,
  scripts/, services/, generated/, terraform/, bootstrap/, config-templates/,
  stub/, drill/, docs/, tests/, .github/, README.md, .gitignore) becomes the new
  repo's root.
- **corpus is excluded.** The design material stays in discovery: `GROUPS.md`,
  the RUN-15 summary, the spec-facing notes, and `corpus-tests/` all live
  OUTSIDE `kit/`, so a copy of `kit/` naturally leaves them behind.
- **`PROVENANCE.md` is generated** at the new root, recording the source commit
  and pointing back to the corpus in discovery.
- build/scratch noise is excluded: `.local/`, `__pycache__/`, `*.tfstate*`,
  `.terraform/`.

The extracted tree is self-contained: `make check` passes inside it standalone
(no dependency on the discovery repo) — the extraction test asserts exactly this.

## History: clean copy vs subtree split

The default is a **clean copy** (no history carried), with provenance recorded
in `PROVENANCE.md`. If commit history matters, use a git subtree split instead:

```
# from the discovery repo, over the kit subtree:
git subtree split -P alpha/experiments/appview-infra/kit -b appview-infra-export
# then push that branch as the new repo's main
```

The subtree split carries the kit's commit history but also the corpus commits
that touched paths under `kit/` — none should exist (corpus lives outside
`kit/`), so the split is clean. The clean copy is simpler and is the default;
choose the split only if history is wanted (an owner call, noted at P2-0).

## Owner pre-step (P2-0)

Create the empty `CroftCommunity/appview-infra`, run
`scripts/extract-to-repo.sh <checkout>`, commit, and push. From there, all
Phase 2 work happens in the new repo; discovery keeps the corpus and the run
summary, and the new repo's `PROVENANCE.md` points back.
