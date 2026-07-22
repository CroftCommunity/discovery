# Raw: Croft games-pond roadmap + browser-native P2P + phased build plan — design dialogue (2026-07-22)

**Preservation status: preserved-condensed (cleaned-paste, content-faithful — NOT byte-pristine) —
PLAYBOOK §4.** Source: a claude.ai design dialogue pasted 2026-07-22. UI render chrome stripped
(duplicated thinking-summary header "Prioritized blocking decisions and recommended solitaire approach";
"Added to memory" confirmations). No secrets. Part of the 2026-07-22 batch (plan:
`alpha/plans/2026-07-22-transcript-intake-ponds-kernel-ens.md`).

**What this is (the candy-crush-clone narrative).** The game discussed here is a **match-3 (candy-crush-style)
game** plus **solitaire** — the "candy crush alternative" named in the orphaned opening line of the companion
card-maker raw. Its P1–P10 build narrative is the source of the **per-pond build discipline** now in the
roadmap: `../../../beta/croft/build-order-and-ponds-roadmap.md` (see "Per-pond build discipline" + the pond
catalog). Read this raw and that roadmap section together.

**Completeness note (§4, honest).** This paste is the **tail** of the P1–P10 plan: it opens at "Nine was the
whole plan…" and contains P10 Sustainment, the quantified-thresholds pass + the user's cut to a follow-chain
number, the P1 blocking decisions + feel-spike, and the solitaire-first sequencing. The **P1–P9 detail
existed earlier in the source conversation but is NOT in the paste** — not reconstructed here (would be
lossy). If the fuller P1–P9 write-up is provided, fold it in and this note is cleared.

**Truncation note.** The user's opening question about browser-native P2P for games is preserved, but the
paste does **not** contain the assistant's answer to it — the text jumps to the game-phases discussion
(the "P10 Sustainment" material answers a *different* prior question about build phases). The known answer
in-corpus is WebRTC data channels via a code-exchange/matchbox signaling pattern (`GGRS + matchbox`,
already in `beta/cairn/iroh-app-pond-building-blocks.md`). Flagged as answered-elsewhere / OPEN-THREADS.

---

## Opener 1 — the ask (user)

I would like to start building up a potential roadmap(s) in Croft for ponds like `fun.croft.ing` that lays
out what to build and how it stacks.

## Opener 2 — browser-native P2P for games (user)

Could I use direct connections between browsers to allow users to play games, especially WASM-accelerated
games? Like one inputs a code from the other — iroh, but browser-native.

`[Assistant answer to Opener 2 not present in the paste — see Truncation note above.]`

## Phase P10 — Sustainment (assistant)

Nine was the whole plan, so there is no tenth *build* phase. There is one thing that genuinely comes
after, and it is the only part with no end date.

**P10. Sustainment.** Not a build phase, a standing drill — the phase where the ten-year registration
either means something or does not.
- **The compatibility matrix is the deliverable.** Old save under new engine, old code under new engine,
  old pack under new engine. P2's version-and-unknown-field policy now governs three document types across
  levels, saves, and codes, and it decays silently unless something exercises it continuously. Keep a
  fixture per document version forever, and add one each time the schema moves.
- **The annual regeneration drill.** From the master seed, rebuild P4's pack byte-identically on a clean
  machine. Same shape as the restore drill you already use for infra, and it is what preserves your ability
  to diagnose level 140 in 2033 rather than guess at it. If the drill stops passing, you have lost that
  capability quietly and will not notice until you need it.
- **Dependency minimalism as a maintenance decision** rather than an aesthetic. Every dependency is a
  decade of somebody else's churn. Few, pinned, boring.
- **Browser drift is the part you do not control.** Storage eviction policy, service-worker behaviour, wasm
  feature availability all move under you, and P6's durability assumptions are the ones most exposed. A
  periodic check on real devices, not just CI.
- **Exit artefacts.** Export works, source is licensed, the static bundle is self-hostable by a stranger,
  and there is a plain statement of what happens if you stop. That is the piece that makes the promise
  checkable rather than rhetorical.

The honest limitation: sustainment is invisible work with no reward signal, and it is exactly what gets
dropped. Scheduling it as a dated drill is the only mitigation I know of.

## Quantified thresholds (assistant)

Thresholds beat a raw ratio, and P3 already contains the material. Its report contract includes moves
remaining on wins across thousands of plays, which is a distribution rather than a single number, so you
can derive per-level bands from its percentiles at build time and ship them in P4's pack. No new machinery.

Three design properties worth locking:
- **Absolute, not relative.** Bands are set against the level, not against other players. You compete with
  the puzzle, and comparison becomes a shared fact rather than a ranking — which is what keeps the bot from
  being able to farm anything meaningful.
- **Two axes, one binary and one graded.** Assistance used or not is binary, legible, and immune to gaming.
  Moves remaining against par is the graded one. "Cleared clean" is the stronger of the two and probably
  the one to lead with, since it is precisely the scarcity you wanted to preserve.
- **Count, not ratio, in aggregate.** "One hundred and forty-two cleared clean" is additive and forgiving.
  A ratio across the pack punishes a single bad night and quietly discourages experimenting on a hard
  level — the wrong pressure for this game.

The count form is also the architecturally sound one: a per-level clean clear is individually verifiable by
replaying its move list against P1's state hash, exactly as P8 does for scores. An aggregate ratio is not
verifiable at all; you would just be trusting the client's arithmetic. So the version that fits your
comparability goal is also the version that keeps the verification property. The caveat is P3's standing
one: bands derived from bot percentiles are calibrated to bot play, and their relationship to human play is
unvalidated. Set them generously and let par be gettable.

## The user cuts it back (user)

It's not that deep. You only care to compare to accounts you follow, and possibly ones who follow them or
whatever — the point is, it's not that deep.

## Response — scope cut (assistant)

Fair, I overbuilt it. The leaderboard problems I was solving only exist at scale among strangers. Twenty
people you actually know is just conversation. So: a number per level, shown next to whoever you follow,
read by follow-chain as already ruled. Par from the bot if it's cheap, skip it if it isn't. That's the
feature.

## What's next (user)

Ok, what's next?

## Response — three decisions, an afternoon, a run brief (assistant)

Three decisions, then an afternoon, then a run brief.

**Decisions blocking P1:**
1. **Language:** Rust to wasm, or TypeScript with integer discipline. Rust gives you the native-plus-wasm
   cross-build determinism test essentially free, and that test protects every phase downstream.
2. **Which specials ship in v1**, everything else into the not-yet set.
3. **One representative blocker in P1 or none.** I'd include one, so the layered cell is exercised by
   something.

Then the **feel spike**: one afternoon, no tests, no architecture, a grid you can swap tiles on to check
the cascade feels good. Delete it after.

Then dispatch P1 as a **run brief**: golden vectors and the rules document first, engine second, red-first
throughout per your standing directive. The deliverable that matters is the fixture corpus and the
tie-break tables, not the code.

**One sequencing note before you commit.** Solitaire's P1 is the same determinism work, and it has no P4 at
all because there are no levels to generate — four phases instead of nine for the thing you ranked higher
on your own list. If you want a real artefact out in the world sooner, dispatch solitaire's P1 first and let
match-3 inherit the engine discipline from it.
