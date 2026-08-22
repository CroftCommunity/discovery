# Raw: treesocial — atproto social graph as a literal tree at tunable radii (design dialogue, 2026-08-21)

**Preservation status: preserved-condensed (cleaned-paste, content-faithful — NOT byte-pristine) —
PLAYBOOK §4.** Source: a claude.ai product-design dialogue pasted 2026-08-21. UI render chrome
stripped ("Thought for Ns" markers — which appeared duplicated in the paste, "Created a file, read
a file" lines, and the "Code · HTML" artifact chips). No secrets.

**PARTIAL PASTE — gap acknowledged by the owner.** The paste opens with the owner's initial idea
prompt and then jumps directly to the assistant's v3 message. The intervening exchanges — the
first tree mock, the "ripple" mock, and whatever prioritization discussion produced "the
prioritized set" — are **not in the paste**. Owner (2026-08-21, in-session): the early mocks
"don't matter, they were not a fit," and this paste is all there is. The gap is bracketed inline
below. The **artifacts** from the missing exchanges survive anyway: all four HTML mocks
(`tree-view-mock.html`, `tree-ripple-mock.html`, `tree-v3-mock.html`, `tree-rings-v4.html`) are
preserved **verbatim, byte-verified** at `../../treesocial-unpacked/` (source `treesocial.zip`,
retired with owner authorization after `diff -rq` verification).

**Fact posture.** This is a model-generated design dialogue; visual/design claims are the
dialogue's own. The one load-bearing external claim — that a Jetstream-style firehose
subscription can be filtered to a few hundred DIDs per connection — is flagged `[UNVERIFIED]`
**by the dialogue itself** and stays that way; do not build on it without a check. atproto
baseline facts defer to the FACTCHECK source of truth
(`atproto-atmospheric-web-iroh-mobile-FACTCHECK.md`); Jetstream as a project is already
registered in `ECOSYSTEM.md`. Account names in the mocks (Dara, Iona, Halcyon, n8n) are sample
data, not real registry entries.

**What this is.** The founding design dialogue for **treesocial**: a standalone atproto site
where you enter a handle and see your social graph rendered as a **literal tree** — follows as
branches, followers as roots, mutuality as symmetry across the soil line, radius tunable outward
(your follows → their follows, with counts), and PDS/provider indicated. Two mock generations
survive in the paste (v3 tree, v4 tree + trunk-cross-section rings) plus a closing ideas round
(seasons, firehose weather, grafts, forest, dendrochronology, feed-lens, sound). Owner intent
(stated at filing time): likely to become its own project → ROADMAP_TODO **E118**.

---

## Dialogue

**Owner:**

> An atproto site that shows the social tree of a user at various radii to demonstrate the
> personal Internet and it's shape
>
> So like you put in your atproto handle and you can show your follows and followers with
> mutuals indicates and a count and you can turn it up to say mutuals follows plus their follows
> kind of thing with a count
>
> And an indication of the pds and provider at play would be cool
>
> What else?

*[GAP — the paste omits the exchanges that produced the first two mocks
(`tree-view-mock.html`, `tree-ripple-mock.html`) and a prioritization discussion. Owner: the
early mocks were not a fit. Artifacts preserved at `../../treesocial-unpacked/`.]*

**Assistant** *(v3; produced `tree-v3-mock.html`)*:

> Mocked the prioritized set into v3: roots, prune preview, timeline growth, vines, PDS
> spotlight, and silhouette export mode, all on the same tree.
>
> What's in it:
>
> - **Roots.** Ground line with followers fanning downward. Mutual follows get a paired stub
>   directly beneath their branch attachment, so reciprocity reads as symmetry across the soil
>   line. Follower-only roots go deeper the bigger their own reach (deep roots carry your posts
>   further), and there's leaf litter at the base for the deactivated accounts still sitting in
>   your follow records.
> - **Prune preview.** Toggle ✂, tap a branch: it ghosts, its unique leaves (no other route)
>   drop to the ground as litter, and the detail card states the loss: "unfollowing loses unique
>   access to N endorsed accounts." Leaves with other routes stay lit, which is the point — the
>   tree distinguishes replaceable reach from irreplaceable.
> - **Growth timeline.** Drag the growth slider from 2023 Q3 to now and the tree replays itself:
>   Dara and n8n first, Halcyon only appearing in 2025. Selecting anything shows its follow date.
> - **Vines.** Tap a multi-route leaf and dashed tendrils arc from it to its other route
>   branches.
> - **Spotlight.** Tap a legend chip and everything off-host desaturates.
> - **Silhouette.** The toggle flattens the same geometry to single-ink shapes with a stats
>   caption, ready to screenshot and compare.
>
> Two honest notes from building it. First, prune preview and timeline interact awkwardly:
> pruning a branch and then scrubbing time resets the prune, which I chose deliberately because a
> preview against a historical graph is a confusing counterfactual, but a real build should
> decide this on purpose. Second, root depth currently encodes the follower's own reach, which is
> a claim about amplification rather than relationship; if the roots should instead mirror the
> canopy's semantics (what following-you-back opens for them), that's a different and more
> symmetric design worth considering before this hardens.

**Owner:**

> I still want the rings idea tho, and I want it as rings of a tree outward but pretty much what
> you showed before
>
> And the tree will need some life put into it for sure, it's very charlie brown Christmas right
> now

**Assistant** *(v4; produced `tree-rings-v4.html`)*:

> Both fixes in v4: the ripple is now a trunk cross-section (same semantics as before, wood
> aesthetics), and the tree got foliage, taper, twigs, and a little wind.
>
> Rings view is now wood. Bark rim, sapwood, heartwood disc, and the same semantics as the old
> ripple wearing cross-section clothes: you are the pith, your follows are knots on the ring-1
> growth ring (dark whorl, PDS-colored core), each knot's sector gets a stained arc on the outer
> band for its ring-2 count, and endorsed accounts precipitate out of the stain as dots. The nice
> fusion that fell out of the metaphor: the fine grain lines are age (one per quarter since your
> first follow, so scrubbing growth literally ages the wood) while the two bold rings are social
> distance. Time and distance coexist in the same disc the way they do in real timber. All rings
> have deterministic wobble so it reads as organic rather than compass-drawn.
>
> De-Charlie-Browning the tree turned out to be a semantics upgrade, not just decoration. The
> foliage mass on each branch is the below-threshold count, rendered as canopy: raise the slider
> and foliage swells as berries dissolve back into it; drop it and berries ripen out of the
> leaves. Branches are now tapered filled shapes instead of stick strokes, with twigs, a leaning
> trunk, root hairs below ground, and a slow staggered sway (disabled under reduced-motion).
> Whales read as bare branches, which is honestly better than the "whale" badge ever was: big
> limb, no foliage you can inspect.
>
> Prune preview now thins the foliage too, so a pruned branch goes visibly winter.
>
> Seam to watch: foliage blobs claim area, and area lies about counts worse than length does. I
> scaled blob radius by √count so it's not egregious, but a viewer will still read Dara's canopy
> as "maybe 3× Iona's" when the real ratio is closer to 2×. The count label under each branch is
> the honest number; the foliage is atmosphere with a rough truth, and that division of labor is
> worth keeping deliberate if this ships.

**Owner:**

> Interesting, other ideas?

**Assistant** *(closing ideas round — no artifact)*:

> Building on where v4 landed, grouped by direction.
>
> Deeper into the wood metaphor:
>
> - **Seasons as activity.** Posting/interaction frequency per branch mapped to foliage color: an
>   account you follow that's gone quiet turns autumn, dormant ones drop leaves. Your tree's
>   overall color becomes a health readout of how alive your graph actually is, which no follower
>   count tells you.
> - **Weather as the firehose.** Ambient drift of tiny particles through the canopy representing
>   live activity from your graph (via a Jetstream-style firehose subscription filtered to your
>   DIDs). Posts land as brief glints on their branch. The tree stops being a snapshot and
>   becomes a barometer.
> - **Grafts.** When you follow someone discovered through the tree (the ring-2 berries), the new
>   branch renders as a visible graft joint onto the parent that introduced you. The tree starts
>   recording its own influence on your graph, which closes the loop between visualization and
>   discovery.
> - **A forest.** Your mutuals' trees rendered small around yours at their graph-distance,
>   canopies overlapping where your follows overlap. Shared canopy is literal shared context.
>   This is the side-by-side comparison idea grown into an environment.
> - **Dendrochronology on the rings view.** Real tree rings record events: wide rings for good
>   years, scars for fires. Annotate the grain: a dense cluster of grain lines where you followed
>   40 people in one quarter (the Twitter exodus wave), a scar where a chunk of your graph
>   deactivated. Your rings become a diary you didn't write.
>
> Off the metaphor entirely, worth keeping in mind:
>
> - **The tree as a feed lens.** Tap any branch or foliage mass and get that subgraph's recent
>   posts as a temporary feed. Combined with grafts, the whole app becomes: see your shape,
>   browse a region of it, grow it, watch the graft take.
> - **Sound.** Each PDS host a timbre, follows plucked in chronological order as the growth
>   slider plays. Thirty seconds of your graph as audio is another shareable artifact, and it's
>   the one format where the bsky.social monoculture would be audible as one note repeating.
>
> The strongest candidates against your original goals: grafts (reinforces "content discovered
> through the social tree" better than anything so far, because discovery leaves a permanent mark
> on the shape) and dendrochronology (makes the rings view tell a story rather than display a
> structure). The forest is the most ambitious and the most expensive, since it needs the ring-1
> fanout crawl for every displayed neighbor.
>
> One feasibility note flagged rather than asserted: live "weather" needs a firehose subscription
> filtered to a few hundred DIDs, which Jetstream-style services support in principle, but I
> haven't verified current filter limits per connection this session, so that one needs a check
> before you design around it. `[UNVERIFIED]`

*(End of paste.)*
