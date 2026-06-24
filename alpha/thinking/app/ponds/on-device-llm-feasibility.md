# On-Device LLM for Navigation: Feasibility Across Platforms

author: research session

scope: what is *actually* available, now (mid-2026), for an on-device model to power semantic ecosystem navigation, across iOS, macOS, Android, Chrome, and Firefox; plus the verified deferred-deep-link state and how the assistant ties to the deep-link resolver

date: 2026-06-21

confidence: claims below are grounded in vendor docs (Apple Developer, Android Developers, Chrome for Developers) and current reporting, cited inline in the chat. Version numbers and device lists move monthly; treat specifics as "true as of now," re-check at build time.

---

## The headline

On-device LLM navigation is **genuinely feasible today on the platforms where your users most plausibly are (Apple), workable-but-gated on Android, real-but-desktop-only-and-flag-gated in Chrome, and absent in Firefox**. The capability is real and shipping, not speculative. But coverage is uneven enough that the only safe architecture is the one you already wanted: **the assistant is an optional accelerant over a navigation system that is complete without it.** That is not a hedge forced by taste; it is forced by the hardware-and-OS coverage reality below.

And there is a clean strategic fit worth stating up front: your task (intent to a small set of known internal links) is *exactly* the task these small models are good at, and *exactly* the task their vendors explicitly scope them to. You are not fighting the grain. Every vendor says the same thing in the same words: these models are for classification, extraction, short rewriting, and structured output over text you already hold, and they are bad at open-ended reasoning and world knowledge. Semantic navigation over a known catalog is the former, not the latter.

---

## Platform-by-platform, verified

### Apple (iOS, iPadOS, macOS): strongest fit, lowest friction

This is the best case and it is very good.

- **The Foundation Models framework** ships in iOS 26 / iPadOS 26 / macOS 26, exposing the ~3B-parameter on-device model that powers Apple Intelligence, directly from Swift, free, offline, no API key, no cloud. Three lines of Swift to a response.

- **It is purpose-built for your exact pattern.** It specializes in "language understanding, structured output, and tool calling." The framing in Apple's own docs and the developer community is "a fast, private, free intern who is excellent at shaping text and terrible at knowing facts." That is precisely the right tool for intent-to-link routing.

- **Guided generation is the killer feature for you.** The `@Generable` macro plus `@Guide` gives **constrained decoding at the token level**: you define a Swift type, and the model is *forced* to emit valid output matching it, including `.anyOf([...])` enums. This directly solves the hallucination risk: you constrain the model to emit a choice from your real catalog of links, and it *cannot* invent an app that doesn't exist. This is exactly the "pick from a list, don't free-form" discipline I flagged earlier, and Apple built it into the API.

- **Tool calling is native**, so the model can call back into your code (e.g. "search the catalog for travel games") and fold the result into its answer. iOS 26.4 added context-size and token-counting APIs so you can adapt to the device.

- **As of WWDC 2026 (iOS 27 / macOS 27)** the framework is opening to any LLM provider via a Swift-package protocol, the core is going open source, and it runs "everywhere Swift runs including Linux." There is now also a **Spotlight-backed local RAG search tool** built in, which is directly relevant if you ever want the assistant to retrieve over local content. Anthropic and Google partner integrations are coming. None of this is required for you, but it means the substrate is deepening fast.

- **The hard floor:** requires an Apple-Intelligence-capable device with Apple Intelligence enabled. That means roughly iPhone 15 Pro / 16-and-later, M-series iPads, M-series Macs, Vision Pro. Below that line, "the framework simply is not there," and the availability check tells you so on every run. So even on Apple you must handle absence, but the *API to do so cleanly exists* and the capable-device base is large and growing.

Net: on Apple, the assistant-as-navigator is a few-lines-of-Swift, free, private, offline feature with built-in anti-hallucination via guided generation. This is as good as it gets.

### Android: workable, but the coverage cliff is steep

Real, but materially more constrained than Apple, and the device gate is the story.

- **Gemini Nano runs in AICore** (the system service since Android 14), accessed via **ML Kit GenAI APIs**. No model bundling, AICore downloads and updates Nano, routes to NPU/TPU, isolates requests, and has *no direct internet access* (downloads go through a separate Private Compute Services APK). The privacy architecture is genuinely strong and aligns with your values.

- **The high-level GenAI APIs** cover summarization, proofreading, rewriting, image description, speech, and a general **Prompt API**. The Prompt API is the one you'd use for intent routing.

- **Structured output is weaker than Apple's.** As of early 2026 the ML Kit GenAI Prompt API is **alpha** and does *not* offer Apple's fluent schema-to-type binding; you coerce structure by putting the format in an `<INSTRUCTIONS>` block in the prompt. Workable, less robust, more prompt-craft. So the anti-hallucination "pick from this list" discipline is enforced by prompt convention, not by the runtime, which is a real difference you'd test carefully.

- **The coverage cliff is the headline problem.** Device support is narrow and *gets narrower with each Nano generation*. The base model needs a flagship SoC and significant RAM; the newest Nano 4 needs ~12GB RAM and a supported accelerator, which as of mid-2026 is roughly **only Pixel 10, Galaxy S26, and a few high-end Oppo/OnePlus/Xiaomi**, with even a Pixel 9 Pro or Galaxy S25 Ultra *not* qualifying for Nano 4. One source estimates Nano-4-compatible devices at ~1-3% of active Android. Older Nano generations cover more (Pixel 8+, Galaxy S24+, etc.), but the point stands: **a large share of Android users will have no usable on-device model at all.**

- **Operational constraints to know:** inference is **foreground-only** (background use returns `BACKGROUND_USE_BLOCKED`), there are per-app and daily battery quotas (`BUSY`, `PER_APP_BATTERY_USE_QUOTA_EXCEEDED`), and you must check `isDeviceSupported()` and degrade. None of these break a foreground "ask the assistant" navigation feature, but they rule out anything background.

Net: on Android, the assistant works on supported flagships with more prompt-engineering effort and weaker structured-output guarantees, and you must assume a big fraction of devices can't run it. The fallback isn't an edge case here, it's the majority path on older/midrange hardware.

### Chrome: real, but desktop-only and flag-gated (not yet a reliable base)

Promising for your PWA/web-served path, but not dependable yet.

- **The Prompt API** (`LanguageModel.create()`, with `availability()` returning available / downloadable / downloading / unavailable) runs **Gemini Nano inside the browser** via WebAssembly/WebGPU, no server, no key, on-device. The API surface is clean and exactly suited to intent routing, with streaming and a documented availability/degradation pattern.

- **But the constraints are serious as of now:**

  - **Desktop only.** Works on Chrome desktop (Windows 10/11, macOS 13+, Linux, ChromeOS on Chromebook Plus). **Explicitly not supported on Chrome for Android, Chrome for iOS, or regular ChromeOS.** So in a mobile webview or mobile-PWA context, the Chrome built-in model is *not available* today.

  - **Flag-gated / trial, not Chrome Stable yet.** The Prompt API still requires `chrome://flags` toggles or an origin-trial token (or distribution as an extension for stable access). Reporting estimates **Stable around Chrome 145-150, late 2026 / early 2027.** So you can't assume a normal user has it on by default right now.

  - **Heavy hardware/storage gate:** ~22GB free disk to host, GPU with >4GB VRAM or 16GB RAM and 4+ cores, unmetered connection for the ~4GB download. The model is purged if free space drops below 10GB.

  - **Honest scoping from people shipping against it:** "Nano is the autocomplete of LLMs, use it where you'd use smart-suggest." And a real caveat for your privacy claim: there is **no browser indicator** that a page is using the on-device model vs exfiltrating prompts, so if you make the "stays on device" claim in a web context, you have to back it with network-panel evidence, not just assert it.

- **Direction of travel is good:** it's a W3C WICG proposal aiming at cross-browser standardization, Apple is expected to expose a Safari-compatible equivalent eventually, and on-device web coverage is projected to climb. But that's future, not now.

Net: Chrome built-in AI is a real future pillar for your web/PWA front-end and a *bonus* on desktop today, but it is desktop-only, flag-gated, and storage-heavy, so it cannot be a load-bearing assumption in 2026. For the web build, the dependable on-device path right now is a bundled WASM model (below), not the browser's.

### Firefox: effectively absent for your purpose

- Firefox has done **on-device translation** (local models) and has experimented with optional local inference, but there is **no Firefox equivalent of the Prompt API / window-level on-device LLM** you could rely on for navigation. Mozilla has publicly worried about Chrome's Prompt API becoming a de-facto standard rather than shipping their own.

- So for Firefox users, the only on-device option is a **model you bring yourself** (WASM), same as the universal fallback.

Net: assume no platform-provided model in Firefox. Bundled-WASM-or-nothing.

### The universal fallback: a bundled WASM model

For everywhere the platform gives you nothing (Firefox, mobile web, older Android, non-Apple-Intelligence iPhones, flag-off Chrome):

- Small models in the **0.5B-3B class (Qwen-2.5-0.5B/1.5B, Llama-3.2-1B, Gemma-2B, Phi-class)** run in-browser via **WebAssembly + WebGPU** through libraries like `wllama`, MLC's WebLLM, or Transformers.js, and natively in a Tauri/Rust core via `llama.cpp` or MLC.

- The cost is a **model download (hundreds of MB to ~1-2GB)** and a WebGPU-capable device, which is a real onboarding weight and not viable on low-end hardware. So the bundled model is itself gated by device capability, it's a fallback that *extends* coverage, not one that reaches everyone.

- [UNVERIFIED] exact current sizes/perf of specific WASM model builds; verify against the library's current benchmarks at build time.

---

## What this means for the design

### The coverage reality, summarized

| Platform | On-device model | Structured output | Mobile? | Reliable now? |
|---|---|---|---|---|
| Apple (iOS/macOS 26+) | Foundation Models, ~3B | Strong (guided generation, token-level) | Yes (capable devices) | **Yes** |
| Android (flagship) | Gemini Nano via ML Kit | Weak (prompt-coerced, alpha) | Yes (narrow device list) | Partial |
| Android (mid/old) | none | n/a | n/a | No |
| Chrome desktop | Prompt API, Gemini Nano | Moderate | No (desktop only) | Flag-gated, ~2027 stable |
| Chrome mobile / Firefox | none | n/a | n/a | No |
| Anywhere (WASM fallback) | bundled 0.5-3B | depends on model | yes if WebGPU | Heavyish, device-gated |

The unavoidable conclusion: **there is no single on-device model you can assume is present.** Coverage is a patchwork. Therefore:

### The architecture this forces (and that you already wanted)

1. **The assistant is strictly optional and gracefully absent.** On every platform, detect availability first (Apple's availability check, Android's `isDeviceSupported`, Chrome's `LanguageModel.availability()`), and when it returns unavailable, the assistant simply isn't offered. No dead ends, no degraded-but-broken state. This is non-negotiable given the coverage table, and it's exactly the "out of the way, trivially disabled" stance you described. The hardware reality and the right product decision coincide.

2. **Every path the assistant offers must also be reachable without it.** Because the model is absent for a large fraction of users (all Firefox, most Android, all mobile Chrome, older devices), the menus/verbs/pins/deep-links must be a *complete* navigation system on their own. The assistant accelerates; it never gates. If you hold this one line, uneven coverage stops being a problem and becomes a "nice on devices that have it" bonus.

3. **Prefer the platform model, fall back to WASM, then fall back to no-assistant.** Three tiers: use Apple Foundation Models / Android ML Kit / Chrome Prompt API where present (free, fast, no download); offer the bundled WASM model where the device is capable but the platform gives nothing (costs a download); show the plain navigation UI everywhere else. Same assistant *interface*, three engines behind it, and absence handled at the bottom.

4. **Constrain output to real catalog links, hard.** On Apple this is free via `@Generable` + `.anyOf`. On Android and WASM you enforce it by prompt convention plus post-validation: the model proposes, your code checks every proposed target against the real catalog manifest and drops anything that doesn't resolve. The model is never trusted to emit a valid link; it's trusted only to *rank intent*, and your resolver maps that to real targets.

### The assistant is a front-end to the deep-link resolver (the unifying insight holds, and verifies)

This is the part that makes the whole thing cheap and coherent, and the platform research supports it:

- The assistant's job is **intent text in, ranked internal deep-links out**, plus a one-line orientation. "Any travel games?" becomes a guided-generation call that returns, say, an ordered list of catalog entry IDs, which your existing deep-link resolver turns into the *same* `pond/app/instance` links a shared URL would produce.

- So you build **one resolver and one catalog manifest**, consumed by two front-ends: links others send you, and language you type yourself. The assistant adds no new navigation plumbing; it's a natural-language adapter onto the router you're building anyway. Apple's tool-calling and Android's prompt APIs both support exactly this "model calls into my catalog, I return structured candidates" shape.

- This also gives you the **cold-arrival moment** cleanly: someone lands in Croft from a deep link into one activity, and the assistant ("you're in a vote, here's the rest of this pond, ask me if you want something else") converts the precision-landing into orientation. The deep-link gets them to the spot; the assistant helps them look up. Two halves of one answer to "how does a newcomer not feel lost," and the assistant half degrades to plain orientation chrome where no model exists.

### One privacy nuance to honor

In a **web context specifically**, there's no browser-level proof to the user that prompts stay on-device (Chrome's own docs flag this). Since "private by construction" is your whole brand, in any web/PWA build that uses an in-browser or WASM model, back the claim with verifiable behavior (no network calls during inference, documented, inspectable) rather than just asserting it. In the native Tauri build this is less fraught because you control the binary and the model is local by construction, but the web build deserves the explicit proof.

---

## Bottom line

**Feasible, and well-matched to your task, with a coverage caveat that mandates optionality.** Apple is excellent and nearly turnkey with built-in anti-hallucination. Android works on flagships with more effort and a steep device cliff. Chrome is a real desktop-only, flag-gated bonus heading to stable around 2027. Firefox and mobile-web need a bundled WASM model or nothing. There is no universal on-device model, so the assistant must be an optional accelerant over a navigation system that is already complete, detect-and-degrade everywhere, and constrained to emit only real catalog links. Built that way, it is one of the better-justified on-device-assistant uses around: narrow, private by necessity, optional by design, and a language front-end to the deep-link resolver you're already building rather than a bolted-on chatbot.

The single highest-value, lowest-risk first build: **the Apple Foundation Models version, using guided generation to emit catalog-constrained deep-links**, because it's free, private, offline, anti-hallucinating by construction, and proves the whole intent-to-link pattern on the platform with the cleanest API and a large capable-device base. Everything else is the same interface with a different engine and a graceful-absence path.

### Verification notes

Apple Foundation Models (iOS/iPadOS/macOS 26, ~3B on-device, @Generable guided generation, tool calling, WWDC26 opening to any provider + open-sourcing + Spotlight RAG) confirmed via Apple Developer docs and WWDC session pages. Android Gemini Nano / AICore / ML Kit GenAI (foreground-only, battery quotas, alpha Prompt API without fluent schema binding, Nano 4 ~12GB-RAM device gate, ~1-3% install-base estimate) confirmed via Android Developers docs and current reporting. Chrome Prompt API (desktop-only, flag/origin-trial gated, ~22GB storage gate, projected stable Chrome 145-150, no on-device indicator) confirmed via Chrome for Developers docs and shipping-developer writeups. Firefox: no comparable on-device Prompt API located; translation-only local models. WASM fallback (wllama/WebLLM/Transformers.js, llama.cpp/MLC, 0.5-3B class) is real but specific sizes/perf are [UNVERIFIED] and device-gated by WebGPU. Deferred deep linking (Android Instant Apps ended Dec 2025, Firebase Dynamic Links shut down Aug 2025, iOS App Clips survive, iOS treats install-context handoff as a privacy boundary, seamless deferred requires fingerprinting via Branch/AppsFlyer/Adjust which conflicts with the no-tracking stance) confirmed via vendor and industry sources; the privacy-respecting answer is Universal/App Links for warm opens, App Clips for proximity, and a claim-code/one-more-tap flow for cold installs.
