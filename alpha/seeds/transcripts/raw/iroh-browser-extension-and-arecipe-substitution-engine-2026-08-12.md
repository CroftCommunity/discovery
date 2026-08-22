# Raw: iroh-in-a-browser-extension (relay-only) + the arecipe substitution-engine plan (dialogue, ~2026-08-12)

**Preservation status: preserved-condensed (cleaned-paste, content-faithful — NOT byte-pristine) —
PLAYBOOK §4.** Source: a claude.ai dialogue pasted 2026-08-21; dated from the plan artifact's own
status line ("Planned (2026-08-12)"). UI render chrome stripped ("Searched the web, loaded tools",
"Ran 2 commands, created a file, read a file", "Thought for Ns" — each appeared duplicated in the
paste, "Document · MD" artifact chip). Inline citation chips in Body B's final answer reproduced as
bracketed source markers as-authored. No secrets.

**Two bodies in one paste** (the BookHive 2026-08-09 pattern):

- **(A) iroh in a browser extension — can it hole-punch? No.** A short Q&A establishing a durable
  negative constraint: an extension is in the same network sandbox as a page (no raw UDP;
  `chrome.sockets.udp` belonged to the deprecated Chrome Apps platform, not the extensions API), so
  an iroh Wasm peer in an extension is **relay-only**, exactly as on a normal page. The only
  browser mechanism that actually punches UDP is **WebRTC** (ICE/STUN/TURN, OS-owned sockets, data
  channel — not raw UDP), which iroh does not currently speak (listed exploratory). WebTransport,
  when it lands, is client-to-server, not peer-to-peer.
- **(B) The arecipe substitution engine.** Owner question: PR #87's direct-regex substitution
  matching is too naive — "is this a good use case for a browser based LLM?" The answer became a
  mostly-deterministic phased plan (vocabulary + pure matcher → search-by-ingredient +
  substitution engine → correction overlay → **evidence-gated** closed-set fuzzy tier), an
  included/excluded walk-through, and an external-food-database round (USDA FDC, FoodOn, annotated
  ingredient-phrase datasets, FoodKG) positioned as build-time, human-review-gated inputs.

**GAP (Body B).** The paste jumps from the owner's opening PR #87 question straight to the plan
delivery, but the plan message repeatedly references "our conversation" decisions (the
dishKey-pattern build tool, the descriptor taxonomy, the owner's "mostly deterministic" call, the
evidence gate) — so a substantial design dialogue between those two turns is **not in the paste**.
Bracketed inline below. The walk-through message partially reconstructs its content.

**Artifacts.** The plan document is preserved **verbatim** (owner-pasted 2026-08-21) at
`arecipe/plans/2026-08-12-1-plan-ingredient-normalization-and-substitutions.md` — referenced, not
re-embedded. Note: that is the **original** plan; the dialogue ends with an *offer* to fold the
external-database edits into an amended plan file, and no amended version exists — if the
external-DB adapters are wanted, they are an edit still to make (tracked in ROADMAP_TODO E119).

**Fact posture.** Model-generated with in-dialogue web citations. Body A's iroh claims are
consistent with the FACTCHECK source of truth
(`atproto-atmospheric-web-iroh-mobile-FACTCHECK.md` — e.g. its crate-hygiene note that an
`iroh-webrtc-transport` crate likely doesn't exist); the browser-sandbox specifics (MV3/Firefox no
raw UDP, Chrome Apps deprecation) and Body B's dataset/licensing claims (CC0/CC-BY assertions,
dataset sizes, FoodKG's Cook's-Thesaurus scraping provenance) are dialogue-cited but
**`[UNVERIFIED]` this session** — verify licenses before shipping any of that data.

---

## Body A — iroh peer in a browser extension?

**Owner:**

> could I run an iroh peer in a browser extensino and use that do the hole punching needed?

**Assistant** *(after web search)*:

> Short answer: no, not the way you're picturing. The blocker isn't iroh, it's the browser
> sandbox, and running inside an extension doesn't lift that constraint.
>
> Here's the mechanism. iroh's hole punching works by having two peers fire UDP packets directly
> at each other's public IP:port, coordinated through a relay that tells each side where to aim (a
> STUN-style dance over QUIC/UDP). That requires the ability to send raw UDP datagrams to an
> arbitrary IP. Browsers deliberately don't allow that. Per iroh's own docs, browsers "don't
> support sending UDP packets to IP addresses from inside the browser sandbox," so "all
> connections from browsers to somewhere else need to flow via a relay server" (iroh WASM/browser
> docs). Their web roadmap says the same: phase 0 is WebSocket relay connections, and direct peer
> connections via WebRTC are only an exploratory later phase (Iroh & the Web). So iroh-in-browser
> today is relay-only, no hole punching, though still end to end encrypted so the relay can't read
> the traffic.
>
> Does an extension change this? No. A background/service-worker extension is still JavaScript in
> the same network sandbox as a page. Raw UDP sockets (chrome.sockets.udp) were part of the Chrome
> Apps platform, which is deprecated and removed, not the extensions API (chrome.sockets.udp is
> under Platform Apps). MV3 extensions and Firefox extensions have no raw UDP. So iroh running as
> a Wasm peer in an extension is in exactly the same relay-only position as on a normal page.
>
> The one thing in a browser that does do real UDP hole punching is WebRTC. Its ICE/STUN/TURN
> machinery negotiates and punches UDP paths for you, but the OS/browser owns the sockets and only
> hands you a data channel, not raw UDP. iroh does not currently speak WebRTC (it's listed as
> future/exploratory, and the WebRTC stack is "heavy"). So the honest state of things: if you want
> an iroh node in an extension right now, you get relay transport. If you specifically want
> browser-side hole punching today, the tool that actually delivers it is WebRTC data channels
> (directly, or via libp2p's WebRTC transport), not iroh.
>
> Two caveats on where this could go. iroh explicitly names WebRTC and
> WebTransport-with-serverCertificateHashes as ways they "may expand" browser support to get
> direct connections, so an iroh peer that hole-punches from a browser is plausibly a future
> capability, just not a shipped one. And WebTransport, when it lands, is client-to-server rather
> than peer-to-peer, so it isn't hole punching in the peer sense either.
>
> Sources:
>
> - iroh WebAssembly and Browsers docs
> - Iroh & the Web (n0 blog)
> - chrome.sockets.udp (Chrome Platform Apps API)

## Body B — the arecipe substitution engine

**Owner:**

> I want to talk about
>
> https://arecipe.app/pr-preview/pr-87/ (PR #87)
>
> I am building a substitution engine for my recipe site but the direct refex type is too naive,
> is this a good use case for a browser based LLM ?

*[GAP — the paste omits the design dialogue that produced the plan's load-bearing decisions
(dishKey-pattern build tool with human review as the single quality gate, the
variety/prep/quality descriptor taxonomy, matching on `parseIngredient` output, the owner's
"mostly deterministic" call, the evidence gate before ML, the closed-set embedding fallback, PDS
community aliases parked as roadmap). The next message references all of them as settled.]*

**Assistant** *(after running commands and creating the plan file)*:

> Here is the plan, written in your plans/ convention (dated filename, Outcome Summary table,
> Problem Statement, load-bearing constraints, Phase 0 with [verify-in-run] items, TDD-first
> phases with milestone exits) so Claude Code can pick it up like your other plan docs.
>
> The structural choices worth flagging before you hand it over:
>
> - Phase 0 explicitly decides PR #87's disposition (reuse the UI shell or start clean) and runs
>   an ingredient census over the corpus, since that census sizes the human-review step and seeds
>   the descriptor word lists.
> - Milestones map to our conversation: M1 vocabulary + pure matcher, M2 search-by-ingredient +
>   substitution engine, M3 the correction overlay, M4 the fuzzy tier. Between M3 and M4 there is
>   an explicit decision gate: M4 only gets built if unmatched-rate evidence shows the alias
>   workflow cannot keep up, so "mostly deterministic" is enforced by the plan rather than by
>   intention.
> - Every new module is a pure core in your existing style (no DOM, defensive reads, reuses
>   parseIngredient, mirrors the exclusions.ts overlay and *-local store shapes), and the ONNX
>   model plus vectors are pinned out of the precache manifest with hermetic CI kept model-free.
>
> Drop it in plans/ and adjust the coverage floor in Phase 2 once the Phase 0 census gives you a
> real number; 90% is a placeholder pending that data.

*[Artifact: "2026 08 12 1 plan ingredient normalization and substitutions" (Document · MD) —
preserved verbatim at `arecipe/plans/2026-08-12-1-plan-ingredient-normalization-and-substitutions.md`.]*

**Owner:**

> Walk me through what this includes and doesn't from our conversation, what does that setup look
> like when it's built?

**Assistant:**

> What made it in. Every load-bearing decision from the conversation: the dishKey-pattern build
> tool with human review as the single quality gate, the variety/prep/quality descriptor taxonomy
> as data, matching on parseIngredient output instead of raw-text regex, search-by-ingredient as
> one MiniSearch field, substitution rules keyed on canonical key with variety overrides seeded
> from your reference chart, compound-line splitting, the correction overlay keyed on raw strings
> with the export-to-baseline promotion loop, the evidence gate before any ML, the closed-set
> embedding fallback with build-time vocabulary vectors, and PDS community aliases parked as a
> re-plan-first roadmap phase.
>
> What it deliberately excludes:
>
> - The browser LLM tier entirely. The free-form "no buttermilk, no lemons, what do I do"
>   conversational layer from early in our conversation is not in any phase. That is consistent
>   with your "mostly deterministic" call and the gate philosophy; if you ever want it, it is its
>   own plan doc, and nothing here blocks it.
> - Substitution ratio math. Rules carry ratio text ("1/2 cup evaporated milk plus 1/2 cup
>   water"), and context flags only "where known." Actually scaling substitutions against the
>   recipe's quantities via your UNIT_FAMILY conversion table was never discussed and is not
>   planned.
> - Overlay sync across devices. We noted per-device divergence as a v1-acceptable caveat; the PDS
>   half that would fix it is folded into roadmap Phase 9, not planned in detail.
> - Semantic search. Embeddings appear only as the matcher fallback; recipe search stays lexical
>   MiniSearch plus the canonical field.
>
> What the built system looks like. At build time you run the census and key-builder, review the
> proposed groups, and commit ingredientkeys.json plus the substitution rules; both ship as small
> JSON in the normal bundle. If the gate ever approves M4, the build also embeds the vocabulary
> and ships a few hundred KB of vectors as a separate lazy asset.
>
> At runtime, a recipe arrives from some cook's PDS and each ingredient line flows: compound split
> → parseIngredient → descriptor split → overlay lookup → alias/exact lookup, first hit wins,
> every resolution carrying its method. In the M3 end state that pipeline is the whole system: no
> model, nothing downloaded, fully deterministic, and unmatched lines simply show a quiet "is this
> X?" chip. One tap writes the overlay, and that string is deterministic on that device forever
> after.
>
> What the user sees: searching "scallion" surfaces recipes written as "green onion"; ingredient
> lines with a known rule get a substitution affordance, where "smoked paprika" hits its variety
> override if one exists and otherwise falls back to the bare paprika rule; lines with no rule
> show nothing rather than an invented ratio. In settings, an export produces the JSON block you
> paste into the vocabulary at the next build review, which is how the local knowledge base flows
> back into everyone's baseline.
>
> If the gate approves M4, the only change is what happens after alias lookup misses: a worker
> lazy-loads the 23MB quantized model on the first such miss, cosine-matches against the shipped
> vectors, and anything above threshold renders with a visible "closest match" label whose
> confirmation feeds the same overlay. The fuzzy tier therefore shrinks with use, which is the
> property that makes the hybrid converge toward deterministic rather than depending on it.

**Owner:**

> I want this to be as robust as it can, can we pull in tecipebi formation or something to build
> the best version?

**Assistant** *(after web search; citation chips reproduced as bracketed markers)*:

> Reading "tecipebi formation" as "recipe DB information", external food databases: yes, and there
> are good ones, but the robustness move is to pull them in at build time through the same
> human-review gate, as seed and test data. Never ship someone else's ontology wholesale into a
> 289-recipe app, and never make one a runtime dependency.
>
> The usable sources, matched to where they plug into the plan:
>
> - **USDA FoodData Central.** Public domain, CC0 1.0, no permission needed, with full CSV/JSON
>   downloads. Its SR Legacy descriptions carry alias information in the names themselves
>   ("Acerola, (west indian cherry), raw"). Caution: FDC naming is lab-style ("Chicken, broilers
>   or fryers, breast, meat only, raw"), not recipe language, so it validates and enriches your
>   alias table rather than replacing it. Plugs into Phase 1 as an import adapter the reviewer
>   accepts or rejects per group. *[cites: USDA FoodData Central ×2]*
> - **FoodOn.** A farm-to-fork food ontology, CC-BY-4.0, with over 9,600 generic food product
>   categories and facets covering organism parts, processing methods, and quality attributes.
>   Its processing/quality facet split independently validates your variety/prep/quality taxonomy.
>   Use it as a reviewer's cross-check in Phase 1, importing only what your census actually needs;
>   CC-BY means an attribution entry if any of it ships. *[cites: GitHub +2]*
> - **Annotated ingredient-phrase datasets.** Probably the highest-value pull: the AllRecipes and
>   Food.com annotated sets (6,612 training phrases) label each ingredient phrase with attributes
>   including Name and State, where State is the processing state like "ground" or "thawed". That
>   State field is literally your prep-descriptor class, pre-labeled on real recipe language.
>   Plugs in twice: seeds the descriptor word lists in Phase 1, and becomes external test fixtures
>   for Phase 2's coverage floor, so the matcher is proven against recipe text you didn't write.
>   *[cites: arxiv]*
> - **FoodKG substitution data.** Their substitution set was collected by scraping The Cook's
>   Thesaurus and parsing Food.com review data. Scraped third-party content has murky
>   redistribution rights, so treat it as a reference the human consults while writing Phase 4
>   rules, not as data you ship. *[cites: Foodkg]*
>
> Concretely this adds one adapter step to the Phase 1 build tool (external aliases proposed
> alongside census-derived ones, same review), one fixture set to Phase 2 (external coverage
> test), and an attribution asset. The strategic point: external breadth mostly buys you alias
> coverage for the open-world feed, which raises the deterministic hit rate before anything falls
> through to overlays or the fuzzy tier. That is exactly where you want robustness to come from in
> a mostly-deterministic design: more reviewed aliases, not more machinery. If you confirm this is
> what you meant, I can produce the amended plan file with these edits folded into Phases 0–2 and
> 4.

*(End of paste. The amended plan file was offered but not produced — the plan in `arecipe/plans/`
is the original.)*
