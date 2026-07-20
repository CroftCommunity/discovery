# Skylite package addendum A1 — TDD convention, landing funnel, seam rulings

`Status: amends run package v2 (2026-07-15-2) and RUN-DISCOVER (2026-07-15-3).
Apply to all runs not yet executed. Seam items are [confirm]: each carries a
default so runs can proceed; a stop rule halts only the affected item.`

---

## 1. Test-first convention (standing, all runs)

Every phase begins by encoding its acceptance bullets as FAILING tests before
implementation:

- Core logic (merge, filtering, config parsing, switch plumbing): unit tests
  first, against checked-in fixtures. Fixtures are built before features.

- Behavior (pages, flows): Playwright specs written from the acceptance
  bullets, running hermetically against mocked responses, red before the UI
  work starts.

- Named invariant tests, written before any code they guard:
  `capabilities-key-on-localOnly-never-skin` and
  `label-floor-excludes` (a known labeled fixture provably never renders).

- Run summaries must show the red-to-green order per phase (test commit
  precedes or accompanies the first implementation commit).

- Honest carve-out: visual/skin polish is not TDD-able; it is covered by the
  behavior specs plus sponsor review. Do not write pixel-assertion theater.

The hermetic gate and @live tier discipline are unchanged.

## 2. RUN-STRUCT S1 expanded — landing content and role funnel

S1 is a content phase, not just a page. House rule applies: copy is approved
by the owner first and carried VERBATIM in the instruction file; Claude Code
lays it out but does not rewrite it.

Funnel map to implement:

- `/` hero + one-switch explainer + two doors.
- Door A "I look after someone" → `/sponsor` (sign-in → dashboard → first-
  garden wizard → provisioning QR).
- Door B "I was given a link or code" → explorer provisioning (paste/scan →
  garden). A device arriving via a provisioning link skips the landing
  entirely and lands provisioned.
- Footer: "about the project" (the demoted idea-capture docs), license.

Draft copy v1 `[confirm before publish — every line]`:

> **Skylite**
> A window to the stars.
>
> A calm, read-first window into Bluesky, grown for you by someone who cares
> about you. No algorithm, no ads, no counts, no strangers.
>
> **How it works.** A sponsor tends a garden: the set of voices an explorer
> sees. Explorers read, save, and share what they find. One switch matters:
> "on this device only." While it is on, nothing about the explorer ever
> leaves the device. Turning it off, together, when the time is right, adds
> hearts that friends can see.
>
> [ I look after someone ]   [ I was given a link or code ]
>
> **Honesty, up front.** Gardens are public records, like everything on this
> network. Saves and notes never leave the device, ever. Hearts, when enabled,
> are public records shown among friends.

Accept: funnel walkable end to end from a cold visit for both roles; copy
byte-verbatim against the approved text; a stranger can state the two roles
and the one switch after one screen.

## 3. Seam rulings (defaults set; [confirm] each)

**A1-1 Public config hygiene (RUN-STRUCT).** Config records are public.
Therefore: sponsor UI enforces nickname-only display names with inline
explanation; record keys are random (never child names); schema carries no
age, birthday, school, or location fields, ever; dashboard states plainly
that this record is public. Default: as written.

**A1-2 Quote/repost principle (RUN-STRUCT, garden rendering) — REVISED.**
Uniform principle: outside content is eligible only via a garden author's act
(quote or repost) and is label-floored identically to everything else (labels
attach to the embedded quoted post's view; exact embed view shape = verify
in-run). Quotes always render inline under this rule. The wall moves to
navigation: a quote/repost never opens casual browsing of the outside
author's feed; the deliberate path in is follow-to-My-Sky. One per-explorer
sponsor switch `showReposts` (default true) exists because reposts inject
whole outside posts at volume with no garden words attached; the switch UI
explains this reasoning in plain words. Honest residual, stated in sponsor
copy and counted in the S7 audit (label-excluded embeds get their own count):
for outside authors, labels are the only safety layer; sponsor trust does not
cover them. Test-first: labeled-quote fixture provably never renders;
navigation-wall fixture provably blocks feed browsing from an embed.

**A1-3 Explorer record sovereignty (doc stance, RUN-SOCIAL).** The sponsor
cannot delete the explorer's likes/follows; remedies are conversation and
pause. Documented in sponsor-facing copy as a stance (her repo is hers, that
is the point), not discovered as a gap. Explorer UI makes deleting her own
records one tap. Default: as written.

**A1-4 Garden-change transparency (RUN-STRUCT).** The explorer device shows
plain notices when the garden changes ("3 accounts were added to your
garden"), derived by diffing config polls locally. Doubles as sponsor-account-
compromise tripwire and as honesty toward the explorer. Default: on, not
switchable off (transparency is not optional). 

**A1-5 Explorer account custody (BLOCKS RUN-SOCIAL B1).** Lazy creation says
when, not who. Ruling needed on: who holds the account email and password
(default: sponsor, in their password manager); recovery/rotation key ceremony
(default: generated at creation, stored offline by the sponsor, per the
IDEAS.md graduation story, handed over at graduation); what the explorer
device holds (default: only the scoped OAuth session, never the password).
B1 halts on this item until confirmed; the rest of RUN-SOCIAL proceeds.

**A1-6 Sponsor account hardening (RUN-STRUCT, sponsor onboarding).** The
sponsor account is the remote control for every garden; its compromise is the
worst credible failure. Skylite cannot enforce upstream MFA, so onboarding
carries a required checklist step with grounded, plain guidance: enable
bsky.social email 2FA (email-code 2FA is what bsky.social offers today; TOTP
and passkeys are requested upstream but not shipped), harden the email
account behind it (that inbox is the real second factor), and know the PDS
OAuth-session revocation page. The A1-4 garden-change notices are the in-app
tripwire if upstream protection fails. Default: as written.

## 4. Verify-in-run additions

Video/media playback path from the Bluesky CDN inside the PWA (CSP, HLS
support in target browsers); getAuthorFeed filter parameters for excluding
replies/reposts server-side vs filtering client-side.
