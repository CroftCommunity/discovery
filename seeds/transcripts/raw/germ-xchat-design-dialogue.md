# Raw transcript — Germ / X Chat feature comparison + design dialogue (verbatim)

source: research + design session, pasted 2026-06-15 · verbatim

> The research deliverable this produced ("Feature & UX Comparison: Germ (Bluesky) and X
> Chat") is filed at research/germ-xchat-features.md. The genuinely new design thinking — the
> interaction-tiers model, type-at-creation, the broadcast tier, and the durable-product
> principles — is distilled into thinking/interaction-tiers.md and crystallized/principles.md
> Tier 3. This file preserves the dialogue turns verbatim. Inline source citations from the
> research phase are in the filed deliverable.

---

## The deliverable (summary; full matrix in research/germ-xchat-features.md)

A feature-and-UX comparison of two adjacent encrypted messengers, lens = the product surface a
user touches. **Germ DM** (Germ Network): MLS-based E2EE, integrates with atproto identity,
launches from Bluesky profiles, iOS beta, currently 1:1 text. **X Chat**: rebuilt X messaging,
Juicebox-backed server-held keys, no forward secrecy (X's own admission), E2EE claims disputed.
Framing note: three distinct things — Germ DM (the real comparable), Bluesky native DMs (not
E2EE), Bluesky native group chats (launched 2026-06-11, ≤50, no media at launch). Each feature
rated against our stack: Natural / Effortful / Hard. The recurring pattern: **the
privacy-preserving behaviour is the free one; the convenience behaviour is the effortful one.**
X chose convenience and paid in encryption integrity; Germ chose privacy and pays in feature
breadth. Multi-device + cross-device history sync is the sharpest tension and the one place we
are clearly weaker than X (whose entire Juicebox design exists to solve it).

## Dialogue (verbatim)

> **My thinking is maybe there would be like a menu of "uses more power" options and we could
> even turn like peer-to-peer typing options and use the aero gossip right? And on smaller
> group platform or smaller groups I guess I wouldn't think that it's too expensive but I feel
> like you could do it. The question is just how much for drain is it...**

Yeah, your instinct resolves the tension better than the binary I posed. I framed it as
peer-to-peer-only vs leak vs omit, but you're describing a fourth, better option: a
user-controlled capability tier, default off, peer-to-peer-only when on, and let battery
reality decide how far people push it. The metadata-leak objection mostly evaporates when it's
iroh gossip between already-connected peers — presence/typing only leak to the broker if they
route through it. The cost is a function of group size and connection count (keepalive + wake
frequency, not payload), which is why your small-groups intuition holds. On "how much drain," I
wouldn't assume — verify the iroh keepalive behaviour; ship a reasonable default, measure.
Read receipts I'd treat differently: presence/typing are ephemeral, but a receipt is durable
state, so it either stays best-effort+opt-in or becomes a CRDT update on the normal path.

> **And then the group constraint is really like maybe in stages... you can just do the group
> size limit by how much power you wanted... at a certain point interactive communication
> becomes hard on a phone with peer-to-peer... maybe it's the consistency or the guarantees
> that go down... how much real meaningful conversation can you have in a chat with more than a
> thousand people — you can't talk at the same time so really it's a broadcast mechanism... it
> would just be like one rolling forward log of announcements... like scuttlebutt 2 which
> didn't hang itself on the forever history plan...**

You've derived a regime boundary, and it's real. Group size isn't one axis with a power knob —
it's three different products that share a send button: interactive (~2 to dozens, live
connections, gossip presence/typing), quiet-but-large (the room's behaviour rescues it — if
nobody talks there's nothing to fan out; lean into eventual consistency), and broadcast (>~1000
or any one-to-many; drop the real-time apparatus; append-only rolling-forward log). The
Scuttlebutt-2 framing is right: keep the good half (rolling forward log, gossip, no real-time
pretense), drop the bad half (immutable infinite history) via a bounded/rolling window. On
"how do you communicate max size" — you don't communicate a number, you communicate a mode, and
mostly infer it; the UI just stops promising the rich signals as the room grows.

> **I really like the mode idea, and I'm wondering if maybe that actually has to be agreed
> on... it does break if you have to switch to a different model mechanism entirely... so maybe
> it's just two columns... [and the settings-tiers side note, and the LTS-interface idea]**

Your hesitation was correct. Don't make mode a switch on one object — make them two different
object types from birth. An interactive room and a broadcast channel are different things you
create differently. A small interactive room can grow and degrade its signals without ever
changing what it is; the thing you don't support is live in-place conversion (that's the part
that swaps mechanisms underneath people). If a room truly needs to "become" broadcast, that's
create-new-and-redirect, not mutation. That keeps MLS membership coherent and kills the
"agreed-on mode that breaks on switch" problem by never switching.

Two broader principles you raised, both good and independent:

**Settings have three audiences, not two.** The standard basic/advanced split conflates "most
people never touch this" with "this is simple." The three masters are better described by
relationship to the system: those who never change anything (defaults are the product); those
who'll tune one or two findable things; those who want the full surface (your early
adopters/fiercest defenders). Name by intent, not depth — depth-names collapse. The middle is a
curated short list, the bottom is the unfiltered surface.

**Shapeability is only valuable paired with stability; constant UI change is quietly
extractive.** When interfaces change constantly, people never build a durable mental model, and
"change it back" friction becomes an engagement lever. LTS-for-interfaces: alpha/beta/stable,
~3-year stable window, opt-in change "trains" every ~6 months, the learned surface left alone
otherwise; security changes the exception, over-communicated. Hold the *learned surface* stable
(layout, names, where things live) while shipping improvements behind it. Honest cost: multiple
live interface generations to support — name it and budget it, or the principle dies in year
two.

(Folded into the deliverable: interaction-tiers section replacing the old leak-vs-omit framing;
multi-device elevated to the primary open problem with the backup-vs-delegation fork; a
design-principles appendix. The battery figure stays [UNVERIFIED] — a measurement question, not
an architecture one.)
