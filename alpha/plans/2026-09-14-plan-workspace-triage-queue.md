# Workspace triage queue — the 2026-09-14 canvass

**Status: WORKING — EXECUTE items in progress on `claude/triage-2026-09` (one PR per repo); DECIDE items await the owner; WAIT items belong to peer sessions.**

## Problem Statement

A full canvass of every repo under `CroftC/` (versions, changelogs, TODOs, plan Status lines,
worktrees, branches, and the workspace audit) produced 43 findings with no single place to
work them from. Each finding already has — or must get — a home in its repo's TODO, plan, or
roadmap row (`CroftC/.claude/TRACKING.md`); what was missing is the pass that decides, for
each, whether it is executed, planned, dismissed, or handed to the owner.

## Approach

One dated queue, five tiers by leverage (the calling arc first, then backbone pins, contract,
hygiene, satellites). Each row carries a recommended disposition. EXECUTE rows are worked in
the `triage-2026-09` feature worktrees and land as ordinary PRs; DECIDE rows are put to the
owner in one sitting per cluster; DISMISS rows are recorded here so the next canvass does not
re-raise them; WAIT rows name the peer session that owns them.

## Reasoning

The queue is a working surface, not a second backlog: no row is documented here beyond a
one-line gist and a pointer, and a row that outlives this pass moves to its home. The
canvass itself (what each repo is at, and why) is in the session that wrote this; the
durable facts it surfaced went into the repos they describe in the same feature.

## The queue

Dispositions: EXECUTE | PLAN | DECIDE (owner) | WAIT (peer session) | DISMISS
Working surface only — every item's home is its repo TODO / plan / roadmap row.

## Tier 1 — the calling arc (unblocks the roadmap)
 1. WAIT    croft `claude/dial-device-run` §16 landing — croftc-b4 owns it and both phones
 2. EXECUTE croft v0.5.1 cut after #1 lands (3 fixes under [Unreleased]; ops/RELEASING.md)
 3. EXECUTE start R1 call-core decision rules (child plan accepted 2026-09-10; expiry clause on R3)
 4. PLAN    §16 unclaimed rungs: REBIND_FAILED guard, NAT/cellular/lifecycle — device-queue rows
 5. DECIDE  E135(b) dead OAuth refresh reads "Signed in" — wording; then EXECUTE
 6. EXECUTE E113 scheduled OAuth refresh — elevated by §16's "OAuth dies at ~6 days idle"
 7. EXECUTE ENFORCEMENT-SCENARIOS: put DEVICE-VERIFIED on the camp + dial rows (after #1)
 8. EXECUTE croft plan Status lines: phase11 + m4 read ACTIVE, m3 has none — mark shipped
 9. PLAN    openmls 0.9.0 adoption (carries §12/§13 device re-validation) — defer behind R1–R4

## Tier 2 — backbone pins and releases
10. EXECUTE ciss-admit pin 0.8.0 → 0.10.0 with the git rev, together (h2 fix reaches the box); converge
11. DECIDE  croft-stack [Unreleased] pile — relay 0.2.1, or an infra section not tied to a relay tag
12. EXECUTE CISS TODO item 1 (h2 bump) is stale — delete
13. EXECUTE CISS: rsa RUSTSEC-2023-0071 dated exception + wire SCA gate (audit check 31)
14. DISMISS CISS object-lifecycle plan — park until a consumer needs it
15. PLAN    meer custodian queue (DRAFT, mechanism UNVERIFIED) — gate on R5; owner Qs: grant minter, meter retention
16. DECIDE  E156 R2 backup for the ciss tenant
17. DECIDE  E149 OVH volume encryption — console check first
18. DISMISS staging on-device enforce rehearsal — production supplied both states; keep the wrong-key row in the device queue only
19. EXECUTE stray pre-rule branches: croft-stack 4, CISS 4, bluebird 7 — inspect, PR or delete each
20. DISMISS optional did:web:admit.croft.ing did.json
21. DECIDE  IPv6 reboot survival unproven — a maintenance-window reboot, or accept

## Tier 3 — contract and ecosystem
22. DECIDE  publish ing.croft.* lexicons: croft.ing bsky account + 2 Porkbun TXT (same task blocks fyi.forage.*)
23. PLAN    contract v3 group-derived grants (E120/E137, P7 S3) — authored in croft, lands in connect
24. DECIDE  connect/android tree still on disk though retired — delete (kills the Gradle-lock TODO) or lock
25. EXECUTE E114 assetlinks.json on connect.croft.ing
26. EXECUTE E128 native iroh logging silent on Android
27. DISMISS E131 browser calling thin tier — behind the web shell, which is a .gitkeep

## Tier 4 — workspace hygiene (audit FLAGs and NOTEs)
28. EXECUTE register 3 ADRs in DECISIONS.md (forage 0005, fun 0002, fun 0003)
29. DECIDE  4 sites without a11y/mobile gates: connect (execute), crofting_site / stellin / treatise (gate or recorded exemption)
30. EXECUTE promote 3 memory files to committed homes (fun auto-merge, fun webkit flake, starter-pack probe)
31. WAIT    worktrees/design-defaults (croft-pwa, UNCOMMITTED work, no FEATURE.md) + mocks-handoff — find owners
32. EXECUTE tear down: p7-plan + spec-decisions (0 commits, pre-Rule-2 names), empty dirs croft/discovery/music-intake; p7-phase0 = owner confirm
33. EXECUTE name the CI gate command in croft-stack + discovery docs (CI-PATTERN rule 6)
34. DISMISS two landings missing Claude-Session trailer — historical
35. DISMISS experiments "5 ahead of upstream" — frozen, verified duplicate 2026-08-26
36. DECIDE  axe 4.13.0 resolved vs 4.12.1 canonical (forage, regift); playwright 1.62.1 in regift — bump canonical or pin down

## Tier 5 — satellites
37. DECIDE  fun: `claude/tier2-wraps` 13 ahead unmerged; mahjong + play-surface PRs open — land or close
38. WAIT    forage `claude/feature-manifest` (2 ahead, paired with a CroftC branch) and `ring-nav-comment` (1) — owners
39. WAIT    arecipe `claude/docs-ingredients` worktree — in flight
40. EXECUTE forage surf skin --muted on --card-2 = 4.47 (< 4.5 AA)
41. PLAN    regift §2 large-mux measurement + §2a Photos credit — device-queue rows
42. PLAN    SHARED-CODE debt: croft-pwa flips 8 "ported from skylite" files canonical; fun (6 files) + arecipe (providers.ts) consume the package
43. DISMISS forage DPoP handshake untested (W17 uses app password) — record as accepted gap, or PLAN if OAuth breaks again
