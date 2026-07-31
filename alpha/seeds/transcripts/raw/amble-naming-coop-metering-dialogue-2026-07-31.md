# Amble naming + cooperative PDS-hosting metered-billing dialogue

date: 2026-07-31 (pasted)

source: claude.ai design dialogue (single conversation)

status: **preserved-condensed (cleaned-paste, content-faithful — PLAYBOOK §4).** This is a chat the
user pasted into the session; no canonical export exists. Reproduced content-faithfully — UI render
chrome stripped (the "Show more", the italic action lines like "Evaluated naming variants…",
"Recalled extensive prior discussion…", "Resolved to offer candid perspective…", and per-message
tool badges) — **not a byte-pristine export.** User turns are reproduced close to verbatim (the
bare-link voice memo and the trucks-vs-load formulation are the user's own words and are load-bearing);
assistant turns are reproduced faithfully to substance.

redaction: none required (no credentials in this dialogue).

provenance notes:
- The **AMBLE name-clearance memo** referenced here was produced as a commissioned research
  deliverable and is filed at `research/amble-name-clearance-2026-07.md` (Stellin-mold), not
  duplicated into this raw.
- The **item-storage protocol experiment suite** the metering half points at was **built and landed
  on `main`** (PRs #33/#34/#36): `experiments/item-storage-protocol/` (E0–E14 + `SPEC.md` + funder
  verifier + tests + `RUN_REPORT.md`) and the dependency-free `experiments/item-storage-protocol-standalone/`.
  `SPEC.md` is the "Item Storage Protocol — experiment hierarchy and narrative" design brief; it is
  the canonical form of the spec the user pasted, so it is cited here, not re-copied.

covers two threads: **(a)** the forum pad's bare-link-sharing idea + the naming move to **Amble by
Croft** (working name, contested clearance) + app-store naming mechanics + the go/no-go; **(b)** the
**cooperative PDS-hosting metered-billing** design (the "second half of the whole conversation").

---

## Thread A — the forum pad, the bare-link view, and the Amble name

### User (voice memo — the bare-link "wish someone would recommend to me" feature)

> Okay. Here's what I want to build. So I'm making this sort of Reddit style site or forum, and it
> runs on a t protocol [atproto]. And one thing I've realized is I like just sharing links, you know,
> and I have nowhere really to… I wanna share, um, no context links. It just feels weird for me. I'm
> like a micro… to not add commentary to reshare a link bare. I don't know. I just feel strange about
> it for some reason. But yet, I wish there was, like — just by virtue of sharing it — like, this is
> interesting. Not I agree with everything in it, not I adopt the opinions or everything in it, but
> just like, this was worth reading. Like, I got something out of this. I learned something from this.
> I discovered something. And I'm not sorry that I did. Doesn't mean that… yeah. I just think that the
> bar should be, like, oh, wow, this is something more people should know about. Or this is something
> that I wish someone would recommend to me. That's it. That's the tagline. **This is something that I
> wish someone else would recommend to me.**
>
> So on this Reddit style site, having a feature where you just share links to it, and it does nothing
> but rank those. No commentary. Just this link is trending, this whatever — and that would be an
> interesting way to discover content. It could be top links… its own little view of what's kind of
> viral or topical, but also perpetual. I don't know if that exists. I don't think I've seen it in the
> way that I'm envisioning it, where maybe you could have a representation of the content of the link:
> you could have the bare link, literal; then the short version that browsers often substitute; then
> maybe, even in a single page application / progressive web app, you could have a little AI summary
> using just the onboard browser AI; or — I was thinking — could part of the service be that things
> would get a generated image, and just see what happens. But that would be fraught. Different ways to
> represent the link, different aspects or elements to it, depending on what you're looking for.
>
> And I personally would blast out links that I'm like, yep, this resonated. I wonder — that may be the
> thing where it would actually make sense to build an app view side rather than try to do it all
> between the browser and the [PDS], but I'm not sure, I'd have to think about it. But definitely
> relational [PDS] followers/following. Followers are followers. That **social tree is definitely my
> preference.**

### Assistant (app-store naming mechanics — the "by Croft" lockup)

The store problem is smaller than it looks, and you don't need "Ambler"/"Ambling" to solve it.
Neither store blocks similar names; the only hard rule is Apple's exact-string uniqueness. The App
Store *listing* name is unique store-wide (like a username); the *display* name under the icon is set
separately and need not be globally unique. So the standard pattern: listing name is
**"Amble by Croft"** (unique string, within Apple's 30-char 2.3.7 limit), display name under the icon
is simply **Amble**. If the NZ app's listing string is bare "amble," your descriptor version registers
fine beside it. Google is looser (package name is the unique id; enforcement only against misleading
brand use, which a distinct icon/palette/"by Croft" lockup avoids).

On "Ambler"/"Ambling": stores would accept either, but they're the wrong move — a shelf showing
"amble" and "Ambler" side by side reads as imitation in a way "Amble by Croft" next to "amble —
social media without influence" does not. Descriptor differentiation is honest and conventional;
near-miss spelling is neither. Two hygiene notes: Apple's name-claim process cuts both ways (it's the
channel the NZ folks could use, and the one the coexistence letter disarms in advance); and Apple's
metadata rules prohibit keyword-packing other apps' names, so never bid on their phrasing.

### User

Oh interesting — and the "by Croft" inclusion is the stronger form anyway.

### Assistant

Right, and the same string does three jobs at once: it clears Apple's uniqueness rule as a side
effect; it's the exact move the clearance memo recommended for trademark strength (a suggestive
dictionary word gains enforceable distinctiveness from its lockup); and it compounds the house brand
(every install screen teaches the Croft mark, so Drystone and the forum inherit recognition Amble paid
for). Precedent for the pattern reading as craft not corporate: "Monument Valley by ustwo" [from
memory, **UNVERIFIED**]. Mechanically: listing name "Amble by Croft," display name stays plain
"Amble," and the by-line propagates for free — About page, templated Bluesky posts ("shared via Amble
by Croft"), the Atom feed generator tag, talk-slide footers.

### User

Ok so real talk, how do you feel about all that for amble usage?

### Assistant (the 80/20)

Real talk: about 80/20, both numbers honestly.

**The 80:** the right call, and not because you wore me down. Best fit this conversation produced; the
register match is exact, the demonym works without strain, and "Amble by Croft" is the rare case where
the defensive move and the beautiful move are identical. The legal theory is sound, not rationalized: a
suggestive dictionary word in a crowded field gives everyone narrow rights; your own claim is scoped to
presentation not the word; and the mitigation stack is real (the letter, the lockup, distinct trade
dress, staying off their domain neighborhood, filing your own applications). The memo's findings support
proceeding more than the cautionary tone suggested: no live blocking registration, both dead marks in
the near classes, an unexamined pending application from a possibly-winding-down company, and a
hundred-download senior user. Plenty of competent counsel would say proceed with the letter and the
filings. You're doing a reasonable thing properly.

**The 20** is one specific scenario, not general unease: the Auckland folks never answer the letter,
stay alive at tiny scale, and you succeed → reverse confusion ripens exactly there. Not likely (tiny
apps mostly die, sell, or answer friendly emails), but it's the branch where the cost lands late and
large. So: the decision is right **conditional on the letter actually being sent and pursued**, not
filed under good intentions. The letter is the load-bearing member of the 80. Warm reply or purchase →
95. Silence → get the intent-to-use filings in fast, publish the homage anyway, and set a
reassess-at-first-traction reminder. Send the letter before the palette gets any more finished;
sentiment compounds faster than clearance does.

### User (the decision)

Agreed, we'll see how amble fits us as a working register and go from there. If it lands, we reach out
before launch.

---

## Thread B — cooperative PDS-hosting metered billing (the "second half")

### User

Did we discuss pds hosting a metered billing in this convo?

### Assistant (recall — the metering design)

Yes, at length; it became the second half of the whole conversation. The arc: it started when we
walked **Frontpage**'s architecture and the **PDS config seam** (the reference PDS takes any
S3-compatible blobstore via environment variables), which became the wedge idea: **an object store
built for PDS-shaped needs, offered by a co-op, working name Drystone.**

The metering design built on top: **meter the boundary, not the machine**, with two units —
**bytes transferred** (postage, bilaterally signed receipts per increment) and **byte-days at rest**
(rent, computed from the member's own signed repo, since a repo plus its referenced blob CIDs is the
storage manifest). **Monthly balance-forward statements** co-signed by both sides, so disputes bound to
one period. The **audit dial**: spot-check frequency as a member-chosen, cost-priced tier, with the
detection math (1−(1−f)^k), plus push-proofs seeded by public randomness so nobody arbitrary can make
you burn cycles. Above that: the **sealed archive and tombstone tiers** (pinned root, key ceremony,
plan for no movement and verify it), the **grace ledger** (waivers and the deceased-member hold as
on-book signed events), the **two-plane split** (public repo and private household-effects storage on
separate hosts, fail-closed by topology), and the **LTS "we won't move the doorknobs" promise** with
remodels by member vote.

All of it is encoded in the experiment file, `item-storage-protocol-experiments.md`: E0 through E8
cover identity, fingerprints, manifest, receipts, statements, audits, dial, seal, and tombstone; E9 is
grace; E10 erasure coding; E11 the extinguishing-royalty financing ledger; E12 to E14 the funder-side
diligence machine.

### User

And the trucks and trips vs load analogy was here?

### Assistant

Yes, same conversation, and it was your formulation. It came up when I flagged that cryptographically
proving compute and bandwidth consumption is mostly unsolved in practice, and you cut through it: all
you need is a consistent, boundary-observable unit, so frame it as the size of the package delivered,
regardless of how it moved. Your words, close to verbatim:

> bytes transferred, "the moving sand," where the size of the trucks and the number of trips don't
> matter, sent and received, signed on each end, and "that's the deal agreed to right there."

I then compressed it into the pile as **"Meter the boundary, not the machine; postage by weight, not by
trips, signed on each end,"** and the analogy did structural work from then on: it's why the provider's
internal costs (hops, retries, hardware) become their problem to optimize under a price rather than
something to prove to you; it's the design principle behind experiment E3's transfer receipts; and it
resurfaced in E14 as the scope condition, since the whole ledger standard only holds where the unit is
countable at the boundary by both sides.

### Assistant (a related fragment — the delivery-cadence "slow lane")

Three threads here, and the first is the keeper: in the tree-out architecture you already chose, the
delivery dial already exists for free. Pull-based reading means your refresh cadence is just how often
you poll your follows' PDSes. The novel move isn't building the slow lane, it's **declaring** it: a
small record in your repo, "delivery cadence: daily," visible to your tree. That flips latency from
something people infer about you (and misread as distance) into a stated fact of the network, like a
mailbox that says mail runs once a day. Nobody apologizes for their mail carrier's schedule. And it's
honest about performance too: **slow-lane users are nearly free to serve, so the economics and the
humanity point the same direction.**

---

## Pointers (where this was distilled)

- Naming → `NAMING.md` "Forum-layer naming" (Graze → Amble by Croft, working name, contested gate);
  clearance report `research/amble-name-clearance-2026-07.md`.
- Bare-link forum feature + subjective-consensus mechanics → `thinking/app/` (bears on the E80↔E62
  architecture fork, COHESION §63).
- Coop metering design → `thinking/cooperative-social-union-model.md`; the "meter the boundary"
  principle → `crystallized/principles.md`; the proof it points at → `experiments/item-storage-protocol/`
  (proof-ledger + test-narrative). Advances **D5**. Naming collision "Drystone (storage)" vs the
  settled Drystone protocol = a tracked reconciliation gate.
