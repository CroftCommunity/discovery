# Round 2026-09-14 — the enforcement flip and the first call (archived workspace focus, 2026-08-17 → 2026-09-14)

**What this is.** The `CroftC/.claude/CLAUDE.md` § "Current focus" block, archived
verbatim on 2026-09-14 when the auto-loaded layer was compressed back to an index
(`discovery/alpha/plans/2026-09-14-plan-dimension-streamline.md`, Phase 1; owner's choice
Q2a: strike, never delete). It is dated history: the ladder, the E153 regression, the
enforce flip, the dial defect and its device verification. Each fact's primary record is
the session file or runbook it cites; this file preserves the cross-repo ORDER of events,
which exists nowhere else. It is not maintained — the live focus block in `CLAUDE.md` is.

---

## Current focus — ENFORCEMENT IS LIVE ON PRODUCTION (the flip, 2026-08-30)

**THE ENFORCE FLIP IS DONE — 2026-08-30, owner-authorized, tested the same night**
(`croft-stack/sessions/2026-08-30-enforce-flip.md`): production `relay.croft.ing:8443` runs
`admission = "enforce"`. Pre-flip, the gate's evidence was produced fresh: a
production-minted pass admitted via `attach_probe`, AND a real client (croft 0.5.0-rc.2 on
an emulator, OAuth-signed-in as the second test account) refusing→minting→
`admitted … sponsorship=BudgetBytes(262144)`. Post-flip, all four arcs verified live:
tokenless refused with words, the pass admitted, the signed-in app re-minting under
enforcement, the signed-out app honestly reporting "NOT camped; calls cannot reach this
device". Rollback stays the one-word revert + converge. Consequence by design: v0.4.0
clients (tokenless) no longer reach the relay; v0.5.0 (camp-at-attach) is the client.
Same night, same authorization: **CISS v0.10.0 released and converged** (TODO §§3/4/5/6
closed — did:-plane meter proven live against the VPS, du-lockdown honest refusal, admin-pins
coverage audited, source-tarball asset restored + tap on it), and the paragraphs below are
the history that led here.

**Read this first (2026-08-28).** Widening the relay log filter (E148) made admission
verdicts visible for the first time on production, and they say the relay **REJECTS both
phones' camping passes** (`invalid_token`) — they stay reachable only because open mode
admits them anyway, and **no `admitted sponsorship=` line has ever appeared there**. Flip
today and both phones go dark. Worse for the record: the bake was being read through
attributed `usage` lines, which prove a token was **presented**, never that it verified (the
relay attributes denials too, on purpose). So "first attributed camp/call" claims across the
runbook, changelogs and status blocks proved presentation only — all now corrected in place.
**ROOT-CAUSED the same day, and it was none of the suspects:** production has been running
**croft-relay v0.1.1, not v0.2.0**, since the 2026-08-25 "promotion" — running binary
`d765924f…` vs the v0.2.0 release `8e287cb7…` (which is what staging runs, hence §12 passing
there). v0.1.1 predates the D3 claims, so it refuses every sponsorship token. **Cause: the
ansible relay role's `creates:` guard named the BINARY, not the version**, so the
checksum-verified tarball was fetched and never unpacked. Bisected cleanly — the same token,
key and probe are ADMITTED by a relay built from source and DENIED by production. The role is
FIXED (version-stamped markers, stamp gated on the unpack) and an audit found the same gap in
the broker's build step; rules recorded in `croft-stack/docs/ANSIBLE-HYGIENE.md`.
**CONVERGED 2026-08-30:** the installed binary is now `8e287cb7…` (v0.2.0, marker stamped),
and a production-minted pass earned the first `admitted … sponsorship=BudgetBytes(262144)`
line this relay has ever logged; a second converge reported `changed=0`. Proven with the rust
`attach_probe` (the phones had gone home) — **a phone earning that same line is the bake's
first honest datapoint, and the bake starts from zero: everything before 2026-08-30 was
measured against a relay that refused every pass.** Full trail:
`croft-stack/sessions/2026-08-28-e153-tokens-never-verified.md`.

The ladder validated 2026-08-17 (rungs 0–3, v0.1.0–v0.2.0; M1–M3 = v0.3.0 ticket redemption,
v0.4.0 callability + atproto OAuth identity proof). **M4 (call-time admission) is client-complete
and device-validated**: §11 (2026-08-21, mint-at-dial + revocation live), §12 (2026-08-24, ALL
rungs green against staging enforce — self-minted camping pass admitted with attribution, the
first fully-enforced call, E129 endings verbatim, the refresh-rotation race found and fixed),
and the client is **RELEASED as v0.5.0** (2026-08-28, Latest, versionCode 6 — promoted from
rc.2 after the published APK passed the two-device test on production: both phones honestly
camped, a relayed-then-direct call, the endings verbatim).

**2026-08-25/26 — the operational wall fell in one night** (three sessions dividing by claim,
the tiered-admission plan's Review Log in `discovery/alpha/plans/`; croft-stack RUNBOOK «ACTIVATE-CROFT-ADMIT»):

- **croft-admit + ciss-admit ACTIVE** on the box (ids 646/647; ciss-admit pinned to the
  admit crate's kind-semantics CISS v0.8.0 — released at the pin commit for exactly this;
  the public ciss tenant versions independently, v0.9.0). Mint keypair generated ON-box.
- **Production relay = the v0.2.0 candidate, OPEN mode, REAL admit key** (pubkey
  `d5d33808…`) — verifying and attributing every pass, refusing nothing. Rollback stays
  written. Staging enforce listener stays on 8444.
- **admit.croft.ing publicly live** (A + LE cert 2026-08-26). **IPv6 is live too as of
  2026-08-28** (E147): this network serves neither RAs nor DHCPv6 — both measured — so the
  address and on-link gateway are STATIC from the OVH console, owned by the `base` role.
  Verified where it counts: a phone on CELLULAR v6 reached the relay through the ip6 DNAT
  (udp/7824). Reboot survival is the one unproven claim.
- **§13 steps 2+4 RAN with the published rc.1 on both phones** (runbook §13 results): first
  PRODUCTION camp mint (SILENT success — the relay's attributed `usage` line is the
  instrument; the `admitted sponsorship=` line is debug-level and filtered in production, an
  open TODO decision), the `endpoint_unbound` caller posture live, the **first attributed
  production call**.
- **The enforcement scenario matrix is landed and gated in BOTH repos**
  (`docs/ENFORCEMENT-SCENARIOS.md` in croft-stack [server: every refusal reason pinned,
  bats-gated in `make check`] and croft [client posture: ~30 rows, JUnit-gated in
  `make gate`]): what must admit, refuse, and degrade can no longer drift silently.

**2026-08-28 — E135(a) was fixed for real, and the first fix was wrong.** Pointing a phone at
the staging enforce listener showed the shipped honesty fix was blind: `addr().relayUrl()`
reports the CONFIGURED relay while the relay refuses every attach. `Endpoint.online()` is the
signal (`watchHomeRelay` throws the same "no reactor running" as `conn.watchPaths()`;
`stats()`'s relay_home_change reads 1 in both states) — device-verified both ways, croft
`237c34f`. The caller phone was ALSO unreachable for the whole first bake because its account
published no endpoint record; publishing one fixed it, and the repair arc is now a harness
journey. Three device lessons became workflow journeys the same session (repair,
silence-is-success, possession-is-not-reachability).

**2026-09-08 — BOTH PHONES CAMPED UNDER ENFORCE, and the dial is broken** (croft
`ops/RUNBOOK-two-device-call-test.md` §15). The prediction above is now a result: Samsung
`14af214d8c…` (20:35:32Z) and Pixel `631277dda5…` (20:40:25Z) each walked the full arc on
the enforcing relay — attach → `denied reason="no_token"` → mint →
`admitted … sponsorship=BudgetBytes(262144)` — with "ready, camped on relay" on screen.
Receiving is sound: the callee held that one connection 14 minutes, no further verdicts.
**Calling is not.** A single Connect tap tears down the caller's camped connection (relay:
`actor errored "Stream terminated"` + a `usage` close, 1 s after the tap); the re-attach
recovered in 4 s once and went **tokenless for four minutes** the first time, unreachable
until the app was restarted. The dial itself failed as **`dial failed: null`** — a refusal
with no words — and no call connected. Open mode hid all of this. Three defects filed in
croft `TODO.md`; the ENFORCEMENT-SCENARIOS camp row is now **DEVICE-VERIFIED**. Also found:
a dead OAuth refresh token still renders as `Signed in` while the phone is unreachable —
that is E135(b)'s concrete case, and it refutes the "shows the handle field" step-0 check.

**2026-09-08, same session — the dial defect is FIXED (unit), and the two workstreams are
now one roadmap.** The cause was one line of lifecycle: the relay auth token belongs to the
ENDPOINT, so changing it means `stop()`/`start()`, and the tokenless dial path called
`rebindWithToken(null)` over a live camping pass — discarding it and re-attaching with
nothing, which enforce refuses by definition. `DialAdmission.rebind` now states the rule —
**a dial never lowers admission** — pinned by `RebindPolicyTest` (written RED first) and
three new rows in the Dial posture table; 177 tests green. Two adjacent faults went with it:
a failed rebind returned `null` and the dial proceeded against a dead endpoint, and the
refusal rendered as `dial failed: null` (the P7 S1 uniffi empty-message finding, met again).
**DEVICE-VERIFIED 2026-09-14 (croft runbook §16) — closed.** The condition was written in
advance and met exactly: one Connect tap on a real phone, and the relay journal carried
**one line in total** from the tap through the connected call and the hang-up — no
`Stream terminated`, no `usage` close, no denial, no re-admit, where the identical action a
week earlier produced all four inside a second. **The call connected** (the first of this
arc), which unblocked the **E129 endings**, verbatim on both screens: *"you ended the call —
ready, camped on relay"* and *"call ended: closed by peer: hangup (code 0) — ready, camped on
relay"*, both sides still camped. §15's insistence on the device tier was right and this time
the unit-green fix was also device-true; the caution stands for the next one, not against
this. **So receiving AND calling are now both sound on main** — the released v0.5.0 still
carries the defect.

Three findings came with that run, none of them the fix, and two are rig hazards.
**OAuth sessions did not survive six days idle** (§15.2 measured ~11): both phones came up
`Signed in` over `NOT camped on relay` with `invalid_grant`, so **re-sign-in is step 0 of
every device run**, not a contingency. **`adb install -r` over the released APK cleared app
data**, taking the persisted iroh secret key — the phone returned with a new endpoint id its
published record no longer named, silently unreachable under enforce until `rkey=self` was
re-published. The Pixel now carries a **debug build** and its record names *that* identity;
any reinstall breaks it again (§16's rig-state note). And a piped `gradlew … | tail` reported
exit 0 over a failed build — the pipe shape `VERIFICATION.md` names, caught only because the
APK was missing at install time.

**Calling and chat are now sequenced as ONE stream** (croft
`plans/2026-09-08-plan-one-stream-calling-and-chat.md`, with
`…-plan-call-core-and-apple-shell.md` as its child). The argument is not that they overlap:
this repo's rules already say calling attaches to the rendered-principal seam, P7 S2 just
produced the first real rendered principals, and the seam has never been buildable because
calling is Kotlin over upstream iroh-ffi while social is Kotlin over our Rust — two iroh
integrations in one APK. Cores stay separate (per-pond cores is law); the shell, the FFI and
the three primitives merge. First contact is a **mirrored diagonal**: chat has the local
case (QR/carried code — the intended baseline, not a limitation), calling has the non-local
one (`ing.croft.iroh.endpoint`), and each lacks the other's. R6 builds calling's local card
(one card, either capability omittable) and R8 chat's non-local path. **R7 gates R6's chat
half** — you should not offer to share a capability you cannot revoke, and chat's token
return is unbuilt.

Next, in order: **R1 of the call-core plan** — the child plan is ACCEPTED with Phase 0
CLOSED (croft `plans/2026-09-08-plan-call-core-and-apple-shell.md`), and R1–R4 are all
phone-free; **E135(b)** wording/UX (was "E130" in croft notes, roadmap renumbered when openprices
claimed E130); E125–E128 port-backs. Croft runbook §13 step 3 (staging honesty check) is
**no longer needed as written** — production supplied both screen states on the released
APK (§15.4); scope any staging run to the wrong-key (`SignatureOrMalformed`) refusal only.
E147/IPv6 and E148/journal-visibility are resolved; the flip itself is DONE (top of this
block).

`croft/android` is the one Croft Call app (contract v2). Test devices + accounts:
`.claude/TESTBED.md`. Repo split: connect owns the contract + directory web; croft is the
client; relay = croft-stack (cards: `.claude/ARCHITECTURE.md`).

Content belongs in the repo that owns it — never loose at the `CroftC/` root. Rule, its
exceptions, and its why: `REPO-GRAMMAR.md` § "Where content belongs" (audit check 8).
