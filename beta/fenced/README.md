# discovery / beta / fenced: the fenced field (the centered commercial platforms)

date: 2026-07-07

**What this layer is.** The survey and map of the *fenced* field: the centered-authority commercial
messaging and community platforms (Telegram, Discord, WhatsApp, Signal, Slack, Teams, Reddit, X,
iMessage, Messenger, LINE, WeChat, and their kin). It records the **extent and shape** of the fenced
territory: how large each platform's rosters, calls, and broadcast objects can grow; what each can and
cannot do (end-to-end encryption stance by surface and layer); how communities behave inside them
(per-group ban / join / live-fraction rates); and how the platforms are monetized. It is descriptive and
quantitative. It makes no argument; it draws the map.

**Why "fenced," not "enclosure."** Enclosure names the *act* of fencing a commons that was once open,
which is the real historical arc the `philosophy/` and `activism/` layers carry (land that was common
grazing, fenced off). These platforms were **never open**: they were built fenced from the start, centered
and proprietary by construction. So the accurate word is the *state* (fenced), not the *act* (enclosing),
and keeping them separate leaves "enclosure" free for its true historical meaning without collision.

**How `fenced` relates to its neighbors (the register triad).** Three layers touch "the field," each in
its own register, and none competes to be the source of truth for another's claim:

```
   cairn     the OPEN field         what exists that we build AMONG (composable, decentralized): the
                                     waymarker stones. CREDIT + reuse.
   fenced    the FENCED field       the centered commercial platforms: extent, shape, scale, capability,
                                     economics. DESCRIPTIVE / QUANTITATIVE. Just the map, no argument.
   activism  harm + response        what the fenced map MEANS in harm terms and anti-society terms, and
                                     what we do about it (up to and including education). NORMATIVE.
```

`fenced` is the descriptive counterpart to `cairn` (the two halves of the existing landscape, open and
fenced) and the descriptive substrate `activism` reads its harm case off of. It also feeds the spec:
`drystone-spec/` §11.9.1 (the encryption posture, why Drystone can be large *and* encrypted) and §11.13
(the empirical basis for "most members dormant, bans rare, cost scales on the live set") read the
E2EE-vs-scale tradeoff and the per-group rates straight off this map.

## Scope

In scope: the centered, provider-operated commercial platforms and their measurable properties, roster /
concurrent-online / call-concurrency / broadcast ceilings, E2EE stance per surface and per layer, per-group
operational rates (bans, joins, live fraction), and monetization / business models. Inclusive of the
messaging apps (Signal, WhatsApp, iMessage, Messenger, LINE, WeChat), the community/social platforms
(Discord, Telegram, Reddit, Bluesky, X), and the enterprise tools (Slack, Microsoft Teams).

Boundary calls:

- **vs `cairn/`.** Cairn is the *open* field, the composable/decentralized tech Drystone builds among
  (MLS, Willow, atproto, iroh, Blacksky, Roomy, p2panda, Nostr). Fenced is the *closed* field, the centered
  platforms Drystone is an alternative *to*. Test: can you build Drystone out of it? → cairn. Is it a
  centered provider whose limits and posture you measure against? → fenced. Bluesky/atproto appears in both
  registers: the *protocol* is a cairn building block; the *hosted Bluesky platform's* posture (DMs not
  E2EE, moderation access) is a fenced data point.

- **vs `activism/`.** Fenced is the map (extent and shape, no valence). Activism is the harm reading and
  the response. A ban-rate number or a roster cap is fenced; "what platform power does to community labor,
  and what we do about it" is activism.

- **vs `drystone-spec/`.** The spec *cites* the tradeoff and the rates it uses (§11.9.1, §11.13). Fenced
  holds the *whole surveyed field* behind those citations, including the platforms and figures the spec
  did not need, so the map is tracked in one place rather than scattered across justifying clauses.

## Contents

| doc | what it is |
|---|---|
| `group-scale-versus-e2ee.md` | The capability map: roster, mutual-call, and broadcast ceilings across 14 platforms; the E2EE stance by surface (1-to-1 / group / broadcast) and layer (text / voice-video); and the two forces that make roster size and group-text E2EE trade off (Force 1, the key-agreement cost curve; Force 2, server-mediated core function). The load-bearing map behind spec §11.9.1. |
| `operational-rates-and-platform-economics.md` | The internal dynamics and economics of the fenced field: the three per-group operational rates (member-ban, member-join, live-fraction) triangulated across source tiers with explicit confidence, and the platform-economics notes (Telegram's monetization, Premium, and boost model as the worked example). The empirical basis behind spec §11.13. |

## Provenance & status

Seeded 2026-07-07 (batch eleven), the layer's first population. Both survey docs were distilled from the
raw transcript
`../../alpha/seeds/transcripts/raw/drystone-large-group-scaling-e2ee-operational-rates-and-telegram-2026-07-07.md`
(the research report the Part 2 §11.14 prompt produced, plus the Telegram field-notes tail), which remains
the provenance. The as-received batch is frozen at `../../alpha/seeds/large-group-scaling-batch11/`. Every
figure carries its source tier (T1 primary / T2 peer-reviewed-or-direct-reporting / T3 secondary) and a
confidence; caps and rates change, so re-verify any single number before it enters an SLA. See
`../../alpha/seeds/transcripts/RAW-ARTIFACTS-MANIFEST.md`.

## What this layer establishes (and does not)

Establishes a home for the fenced-field map, so the centered-platform survey stops being homeless (it is
not a cairn building block, and it is not the activism argument) and the E2EE-vs-scale tradeoff and the
per-group rates are tracked in one descriptive place that the spec can cite. Does **not** argue the harm
case (that is `activism/`), does **not** catalogue composable open tech (that is `cairn/`), and does
**not** duplicate the spec's own citations of the tradeoff it uses.
