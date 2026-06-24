# Croft app — Phase-0 doc review & port-ownership decision dialogue (2026-06-21)

> **Provenance status: preserved-condensed (cleaned-paste, content-faithful — NOT a
> byte-pristine export).** Per PLAYBOOK §2b/§4: this is a chat the user pasted into the
> session; no canonical export exists, so this is the best-available raw. UI render chrome has
> been stripped ("Searched the web", "Edited …", "Created a file", "Presented N files",
> "Document · MD", date dividers); the assistant's tool actions are bracketed inline as
> pointers. Dialogue wording is reproduced as faithfully as the rendered paste allowed.
>
> **What this session is:** a focused review of the then-current Phase-0 `BUILD-SPEC.md` and
> `design-philosophy.md`, in which the assistant flagged gaps, argued out the port-ownership
> ambiguity, and locked five decisions (DECISION 1–5). The session's *output* was an edited pair
> of docs (the `appframework.zip` the user later handed over).
>
> **Supersession note (filed 2026-06-22):** the five decisions and the tightened language this
> session produced are already present — and further developed (with a §3a cursor-invariant proof
> and §1a garden thesis the zip lacks) — in the repo's `thinking/app/build-specs/BUILD-SPEC.md`
> and `thinking/app/design-philosophy.md`, which are byte-identical to the frozen
> `seeds/multiecosystemapp-unpacked/` seed. So `appframework.zip` is a superseded *earlier* snapshot
> of already-verbatim-filed artifacts. This transcript is filed because the *derivation reasoning*
> here (the port-ownership argument; how DECISION 1–5 were reached) was not captured in the larger
> `croft-app-design-dialogue-2026-06-20-to-22.md`. See COHESION §20 and the manifest.

---

## USER

> I am looking to start building out an app framework, can you review these docs and propose
> extensions or updates and give them back so Claude code has the best set of guidance?

[User pasted two documents: the Phase-0 `BUILD-SPEC.md` (a build specification for an AI coding
agent — what to build, in what order, acceptance criteria, paired with `design-philosophy.md`) and
`design-philosophy.md` (the "why": principles, decisions, reasoning behind the project — coherence
across core / shell / design system / ponds).]

## ASSISTANT

I've read both documents closely. Before proposing changes, the most useful thing I can do is flag
where the guidance has gaps or latent contradictions that would actually trip up Claude Code, rather
than just adding polish.

Here's my assessment, organized by severity.

### The gaps that will actually bite

**1. The port ownership decision is left open, and that's a problem for an agent.**
Section 2 of the build spec says the port trait can live in either core or bluesky, "choose one,
document it." But section 3 says `FetchFeed { cursor }` is an effect the core emits, and the runtime
turns port results into intents. An agent reading "choose one" will pick arbitrarily, and the two
choices have different consequences for your WASM constraint. If the trait lives in core and is
async, you've potentially pulled an async surface into the crate you're trying to keep pure and
WASM-clean. The philosophy doc actually resolves this implicitly: the core describes effects as data
and never calls the port. So the trait shouldn't be a core dependency at all in the strict reading.
The shell holds the port. This wants to be stated as a decision, not an option.

**2. "The core knows no clock" plus relative timestamps is underspecified in a way that invites a
leak.** P3 and the P3 note handle this for projection, but the spec only says "surface this choice in
a comment." For an agent, that's an invitation to read the clock anyway and leave an apologetic
comment. The decision should be made: absolute timestamp formatted from post data in Phase 0,
relative time deferred. Otherwise you get `chrono::Utc::now()` smuggled into `project.rs`.

**3. The fixtures are described but their shape isn't pinned.** M4 and section 6 say "real recorded
Bluesky timeline responses, committed." But nothing says how they were recorded, what auth state
they represent, or what happens when the real `app.bsky.feed.getTimeline` shape drifts. An agent will
invent fixture content. You want either a recording procedure or a note that fixtures are provided by
you and the agent must not fabricate them.

**4. The FeedStatus round-trip on retry has an unstated assumption.** D2 says retry from
`ErrorWhileAppended` goes to `LoadingMore` with the preserved cursor. C5 says `ErrorWhileAppended`
preserves the cursor. Good. But there's no test that the cursor survives the projection unchanged,
and no invariant stating that `ErrorWhileAppended` always carries a cursor. It's possible to
construct an `ErrorWhileAppended` with no cursor, which is an illegal state your enum permits. That
contradicts "make illegal states unrepresentable."

### Smaller things worth fixing

The dependency arrows in section 2 say core depends on the bluesky port trait, but the philosophy's
strict reading says the core only emits effect data. These should agree.

There's no mention of error type anywhere. `reason` appears in intents and statuses as an unspecified
type. An agent needs to know: is it a String, an enum, an opaque error? For Phase 0, a string reason
is defensible, but it should be said.

The CLI's "force an error path" command has no specified mechanism. The fake supports error mode
(section 6), so the CLI flag should drive the fake's error mode, not invent its own failure
injection.

### What I'd propose

Rather than rewrite your docs wholesale, I'd add a short DECISIONS.md that resolves the open choices
the build spec deliberately left open, plus targeted patches to a handful of spec sections. This
keeps your two documents intact (they're well-structured) and gives Claude Code an unambiguous
decision layer to check against.

Before I write any of it, two questions that change what I produce:

**Q: How do you want the resolved open decisions delivered?** → *A: Edit the two docs in place*

**Q: Port trait location — which reading do you want locked in?** → *A: Want me to argue both then
pick*

## ASSISTANT

Good. Let me work the port question first, since the answer determines several edits in both docs.
Then I'll make the in-place changes.

### The port location argument

**The case for the trait living in core:** It's the conventional Crux-style shape. The core declares
the capability it needs as a trait, the shell provides an implementation, and dependency inversion
does its job. Tests in core can depend on the fake directly. There's one obvious home for "what the
core requires from a pond," and it sits next to the code that requires it. Most people reading the
codebase would expect the contract to live with the consumer.

**The case for the shell holding the port, core emitting only effect data:** Your own philosophy
already chose this, in three separate places. Section 3 says the core "describes what it wants done
as data" and "the shell performs them and feeds the results back in as new intents." Section 5 says
"modules also do not perform their own I/O in the finished design. They describe requests; the host's
effect handlers perform them." The boundary is explicitly three kinds of traffic: intents in, view
models out, effect requests out as data. A trait the core calls is not effect-as-data. It's
effect-as-function-call. The moment core holds an `async fn fetch_timeline(...)` trait and depends on
it, the core's purity becomes a matter of discipline rather than structure, because the trait is
right there to be called. And an async trait in the core crate is exactly the kind of thing that
complicates your standing WASM constraint and your "no async runtime in the core" rule from section 1
of the spec.

**The deciding factor:** The two readings produce different architectures, not just different file
locations. In the core-holds-trait reading, the core's relationship to I/O is "I call a capability
and await it." In the effect-as-data reading, the core's relationship to I/O is "I return a
`FetchFeed` value and forget about it; something hands me a `FeedPageLoaded` intent later." Your
entire test story (`update` is a pure synchronous function returning `(Model, Vec<Effect>)`) only
holds under the second reading. The first reading can't have that signature, because an awaited call
can't return from a synchronous pure function. So the spec's own `update` signature has already
decided this. `fn update(model, intent) -> (Model, Vec<Effect>)` is incompatible with the core
calling an async port. The "choose one" in section 2 contradicts the signature in section 3.

**Locked decision: the shell holds the port.** The core depends on neither the port trait nor the
bluesky adapter. The core depends only on the bluesky crate's native data types (the post shape it
carries in intents and stores in the model), not on any trait. That last clause matters: core still
has a dependency edge to bluesky (for the native post type that rides inside `FeedPageLoaded` and
lives in the Model), but it's a data-type dependency, not a trait dependency. The port trait moves
entirely to the shell side.

Let me confirm one thing before I edit, because it changes the crate graph:

**Q: The core stores posts in its Model. Should it hold bluesky's native post type directly?** →
*A: Yes — core imports the native post type from bluesky*

## ASSISTANT

Good. That keeps the crate graph simple and is consistent with honest seams: the core holds the
native shape, never a normalized one, and the projection is the only place that turns native into
display-ready. The dependency edge core → bluesky (data types only, no trait, no adapter) stands.

[Assistant attempted to read the source files from disk to edit the actual source; the upload
directory was empty, so the documents had come in as context rather than disk files. Assistant then
wrote both edited documents from the content it had, checked for a markdown skill (none), and made
the edits surgical and marked.]

[Assistant edited `BUILD-SPEC.md`: resolved the open port choice and the other gaps. Edited
`design-philosophy.md`: added a decisions log and tightened the port/effect language to match the
build spec. Verified formatting preferences (blank lines between bullets; removed every em-dash,
including five it had briefly introduced in the decision headings, replacing them with
periods/commas). Presented both files.]

Both documents are edited in place and ready for Claude Code.

### What changed and why

The core move was resolving the port-location ambiguity, and once I worked it through, the answer was
forced rather than chosen. Your `update` signature is `fn update(model, intent) -> (Model,
Vec<Effect>)`, a pure synchronous function. An async port the core awaits cannot return from that
signature. So the spec's section-3 signature and its section-2 "choose one" were already in conflict,
and the signature wins. The shell holds the port; the core depends on the bluesky crate for native
data types only. Your philosophy doc had actually chosen this three times already (sections 3 and 5),
so I tightened those passages to say it outright rather than implying it.

**The five locked decisions** (now in a new section 1a of the build spec and mirrored in a decisions
log, section 13, of the philosophy doc):

1. Shell holds the port, core never touches the trait.
2. Core stores the native post type directly (honest seams, projection is the only conversion point).
3. Phase 0 timestamps are absolute from post data, relative time deferred, so nobody smuggles
   `Utc::now()` into the core.
4. Cursor-bearing states carry the cursor in the variant, which makes "an `ErrorWhileAppended` with
   nothing to retry from" unrepresentable. This changed the `FeedStatus` shape and several A–D test
   assertions, so check those against your intent.
5. `reason` is a plain String for now, and the CLI's error path drives the fake's error mode instead
   of inventing its own failure injection.

I also added a no-fabricated-fixtures rule to section 0, since an agent will happily author plausible
timeline JSON otherwise, which defeats the point of testing against real shapes.

Two things worth your eye before you hand this off. First, the `FeedStatus` redefinition in section 3
and the rewritten C1/C2/C5/D2 assertions are the highest-leverage change. If you had a different
cursor-storage model in mind, that's the place it'll diverge. Second, I kept the source's
hyphen-and-period heading style and removed every em-dash, including ones I'd briefly introduced in
the decision headings. The decisions log is structured so future phases append to it with the same
"reason + bites" format, which gives Claude Code one canonical place to check when a choice feels
underspecified.

[Artifacts presented: **Build spec** (`BUILD-SPEC.md`) and **Design philosophy**
(`design-philosophy.md`) — the pair delivered to the user as `appframework.zip`.]

---

## End of dialogue

Filed 2026-06-22. The decisions derived here live (further-developed) in
`thinking/app/build-specs/BUILD-SPEC.md` (DECISION 1–5 + §3a cursor-invariant proof) and
`thinking/app/design-philosophy.md` (§1a/§4a + decisions log). `appframework.zip` is an earlier
snapshot of those two docs and adds no content the repo lacks.
