# Workspace triage queue — the 2026-09-14 canvass

**Status: EXECUTE PASS DONE 2026-09-14 — seven PRs open (fun #94, CISS #42, croft #16, discovery #55, croft-stack #22, forage #69, CroftC #47); CISS/croft/croft-stack also carry the rustls RUSTSEC-2026-0285 fix that surfaced mid-pass; #3 (R1) is built and open as croft #18; DECIDE items await the owner; WAIT items belong to peer sessions.**

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
 2. DECIDE  croft v0.6.0 cut — retargeted 2026-09-25: the pile under [Unreleased] is D3 (the Android app onto the shared core, croft #22–#24) plus R4's macOS shell and the three §16 fixes, a minor not a patch; the candidate would be signed with a laptop debug keystore where v0.5.0 was CI-signed, so existing installs take a fresh install (key wiped → `self` re-published) — that is the owner's call and the release-keystore question ops/RELEASING.md names; then EXECUTE per RELEASING.md (rc → two-device test on the PUBLISHED APK → promote)
 3. LANDED — R1 (croft #18), R2 (croft #19, the calling transport port; the §15 defect is a `:live` cargo test), R3 (croft #20, `croft-arc`, the headless arc RUN against production with no phone), all 2026-09-14/15; R4 (croft #21, the macOS shell, a production call from the window) and D3.1–D3.4 (croft #22, the Android app onto the core; #23 phone-to-phone over our port; #24 the relayed call with the Pixel on LTE) 2026-09-21/23 — the child plan is COMPLETE; the parent's R5–R8 are unstarted and unqueued (child plan accepted 2026-09-10)
 4. PLAN    §16 unclaimed rungs: REBIND_FAILED guard, NAT/cellular/lifecycle — device-queue rows
 5. DECIDE  E135(b) dead OAuth refresh reads "Signed in" — wording; then EXECUTE
 6. FILED   E113 scheduled OAuth refresh — the croft TODO row exists as of 2026-09-25 ("Schedule the OAuth refresh so an idle phone keeps its session", `[device: android]`), carrying the three contradictory idle measurements (dead at ~10 and 6 days, alive at 7) and the rule to measure the lifetime before choosing a period; now proposable as work
 7. DONE (2026-09-25) ENFORCEMENT-SCENARIOS: the camp row carried DEVICE-VERIFIED since §15; the rebind row since §16; the v1 tokenless dial row now carries it from §16/§17 (every phone dial in the arc was a no-grant card, both phones, both directions, LAN and LTE). The token-bearing dial rows stay unmarked on purpose — no grant-bearing dial has been placed from a phone
 8. DONE (croft #16) croft plan Status lines: phase11 + m4 read ACTIVE, m3 has none — mark shipped
 9. PLAN    openmls 0.9.0 adoption (carries §12/§13 device re-validation) — defer behind R1–R4

## Tier 2 — backbone pins and releases
10. DONE (croft-stack #22; **converged 2026-09-15 03:49Z**, owner-authorized — `ok=101 changed=12`, then `changed=0`; croft-stack `sessions/2026-09-14-vps-ops.md`) ciss-admit pin 0.8.0 → 0.10.0 with the git rev, together (h2 fix reaches the box); converge
11. DONE (croft-stack #34, 2026-09-24) the pile became the two tag sections `[croft-admit 0.2.1]` + `[croft-relay 0.2.1]` in the v0.2.1 release commit; the ansible / drill / services / broker rows rode under the relay tag, which is the tag that closed their window (CHANGELOG.md header). Both binaries converged 2026-09-24 (croft-stack #35).
12. DONE (CISS #42) CISS TODO item 1 (h2 bump) is stale — delete
13. DONE — already shipped 2026-08-29, closed on the record (CISS #42) CISS: rsa RUSTSEC-2023-0071 dated exception + wire SCA gate (audit check 31)
14. DISMISS CISS object-lifecycle plan — park until a consumer needs it
15. PLAN    meer custodian queue (DRAFT, mechanism UNVERIFIED) — gate on R5; owner Qs: grant minter, meter retention
16. DECIDE  E156 R2 backup for the ciss tenant
17. DECIDE  E149 OVH volume encryption — console check first
18. DISMISS staging on-device enforce rehearsal — production supplied both states; keep the wrong-key row in the device queue only
19. INSPECTED → DECIDE stray pre-rule branches: croft-stack 4, CISS 4, bluebird 7 — inspect, PR or delete each
20. DISMISS optional did:web:admit.croft.ing did.json
21. DONE (**rebooted 2026-09-15 04:15Z**, owner-listed: ssh back in 12 s on kernel 6.12.107; static v6 address + route, relayns + 6 DNAT rules, all units, both listeners' admission verdicts verified after boot — croft-stack `sessions/2026-09-14-vps-ops.md`) IPv6 reboot survival unproven — a maintenance-window reboot, or accept

## Tier 3 — contract and ecosystem
22. DECIDE  publish ing.croft.* lexicons: croft.ing bsky account + 2 Porkbun TXT (same task blocks fyi.forage.*)
23. PLAN    contract v3 group-derived grants (E120/E137, P7 S3) — authored in croft, lands in connect
24. DECIDE  connect/android tree still on disk though retired — delete (kills the Gradle-lock TODO) or lock
25. DECIDE  E114 assetlinks.json on connect.croft.ing — the file exists with a placeholder; the release signing cert's SHA-256 is the owner's to supply
26. PLAN    E128 native iroh logging silent on Android — roadmap row, needs a croft TODO/plan entry first (same rule as #6)
27. DISMISS E131 browser calling thin tier — behind the web shell, which is a .gitkeep

## Tier 4 — workspace hygiene (audit FLAGs and NOTEs)
28. DONE (CroftC #47) register 3 ADRs in DECISIONS.md (forage 0005, fun 0002, fun 0003)
29. DECIDE  4 sites without a11y/mobile gates: connect (execute), crofting_site / stellin / treatise (gate or recorded exemption)
30. DONE (fun #94 + memory pointers) promote 3 memory files to committed homes (fun auto-merge, fun webkit flake, starter-pack probe)
31. WAIT    worktrees/design-defaults (croft-pwa, UNCOMMITTED work, no FEATURE.md) + mocks-handoff — find owners
32. DONE (local; p7-phase0 left for the owner) tear down: p7-plan + spec-decisions (0 commits, pre-Rule-2 names), empty dirs croft/discovery/music-intake; p7-phase0 = owner confirm
33. DONE (croft-stack #22, discovery #55) name the CI gate command in croft-stack + discovery docs (CI-PATTERN rule 6)
34. DISMISS two landings missing Claude-Session trailer — historical
35. DISMISS experiments "5 ahead of upstream" — frozen, verified duplicate 2026-08-26
36. DECIDE  axe 4.13.0 resolved vs 4.12.1 canonical (forage, regift); playwright 1.62.1 in regift — bump canonical or pin down

## Tier 5 — satellites
37. DECIDE  fun: `claude/tier2-wraps` 13 ahead unmerged; mahjong + play-surface PRs open — land or close
38. WAIT    forage `claude/feature-manifest` (2 ahead, paired with a CroftC branch) and `ring-nav-comment` (1) — owners
39. WAIT    arecipe `claude/docs-ingredients` worktree — in flight
40. DONE (forage #69) forage surf skin --muted on --card-2 = 4.47 (< 4.5 AA)
41. PLAN    regift §2 large-mux measurement + §2a Photos credit — device-queue rows
42. PLAN    SHARED-CODE debt: croft-pwa flips 8 "ported from skylite" files canonical; fun (6 files) + arecipe (providers.ts) consume the package
43. DISMISS forage DPoP handshake untested (W17 uses app password) — record as accepted gap, or PLAN if OAuth breaks again

## Tier 0 — surfaced by the pass itself
44. DONE    RUSTSEC-2026-0285 (rustls 0.23.43, published 2026-09-14 12:00Z) blocked the dependency gate on CISS, croft, and croft-stack — croft-stack's scheduled main scan was red the same morning. Production for every shipped binary, so upgraded (0.23.45, lockfile-only) on the three triage branches, each repo's gate green
46. WAIT    the same three deps scans now block rustls 0.23.45 on licence UNKNOWN — deps.dev has no record yet of a crate published this morning (404 at 21:00Z; 0.23.43 resolves to Apache-2.0 OR ISC OR MIT). Not a widening case: re-run the `security` job on CISS #42, croft-stack #22, croft #16 once deps.dev indexes it (`gh run rerun <id> --failed`)
47. PLAN    the gate's blind spot #46 exposed: a crate too new for deps.dev blocks as UNKNOWN even when the lockfile's own registry manifest states an allowlisted licence — a rung the gate could resolve from `cargo metadata` rather than a network lag it cannot see (croft-pwa `dep_gate.py`; SUPPLY-CHAIN rule 7 wording)
48. DECIDE  forage main CI is RED on `mixes.workflow.mjs` ("Top: 10 likes × 2 = 20 beats harvest's 13 (got undefined)") since a2af930 on 2026-09-09, through #66, #67, #68 — every landing since has merged on a red main (fun's auto-merge lesson, forage edition); the forage owner session should take it
45. DISMISS discovery's 17 experiment/spike lockfiles also hold affected rustls (0.23.40–0.23.43) — frozen spikes under `advisory-paths`; their scan passed and nothing ships from them

## Findings from the execute pass (2026-09-14)

- **#19 stray branches, inspected.** All predate the worktree rules and are 127–214 commits
  behind main. croft-stack (4, all 2026-08-05, 1–2 commits each): `ciss-fixed-uid`,
  `docs-box-constraints`, `docs-ciss-atproto`, `todo-ovh-encryption` — docs/identity notes
  whose subjects (fixed uid, box constraints, OVH encryption = E149) were since recorded
  elsewhere. CISS (4, 2026-08-04/05): `atproto-identity` (11 commits, a v0.3.0 bump — long
  superseded), `docs-consistency-pass`, `gated-reads-design` (spec since EXECUTED),
  `resolver-observability` (v0.3.1 bump). bluebird (7, July): two are the heads of open
  upstream PRs #33 (`pwa-hardening-ios`) and #34 (`fix-sponsor-a11y-contrast`); the other
  five are July hardening branches. Recommendation: delete the croft-stack 4 and CISS 4
  (content superseded; a diff against main confirms nothing unique survives), keep
  bluebird's until #33/#34 resolve. Deletion is the owner's call — nothing was deleted.
- **Two pre-existing local reds met, neither caused by this pass.** forage
  `mixes.workflow.mjs` fails identically on a clean checkout of `origin/main` (26b2cef);
  croft-stack `make check-local-drill` is E150 (RED on main since 2026-08-27; the
  ciss-admit/ciss/croft-admit tenants PASS in the same drill).
- **The Homebrew cargo trap, met again.** `cargo --version` on bare PATH resolved to
  Homebrew's 1.98 while `rustup which cargo` said 1.94.1; the gate was re-run with
  `~/.cargo/bin` first. CI-PATTERN's rule, observed once more.
- **Re-dispositioned:** #6, #26 EXECUTE → PLAN (roadmap rows are to-be-planned, never
  work — TRACKING § Two piles); #25 EXECUTE → DECIDE (the assetlinks file exists with a
  placeholder; the release cert fingerprint is the owner's).
- **After croft-stack #22 converges:** regenerate `CroftC/.claude/DEPLOYED.md` — *converged 2026-09-15; nothing to regenerate: the register reads the declaration (it already said 0.10.0), the box was what lagged it*
  (ciss-admit 0.8.0 → 0.10.0).
