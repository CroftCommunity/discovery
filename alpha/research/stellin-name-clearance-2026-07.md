# Stellin by Croft: Second-Pass Research on Domain, Name-Collision, Trademark, and ATProto Landscape

author: Research agent (claude.ai, commissioned; ~442 sources, 2026-07)

date: 2026-07

status: draft for review — NOT legal advice; a professional clearance search is still required

verdict: CONTESTED (not blocked) — brand-adjacency + SEO risk from "Stellina / Stellinapp"

---

`Commissioned name-clearance / collision deliverable for **Stellin** — a LinkedIn-alternative
professional-networking app on atproto (professional microblogging, groups, resume profiles),
PWA/SPA on Bluesky identity, primary domain stellin.app, branded "Stellin by Croft". Filed
content-faithful. **Provenance caveat carried from the source:** live RDAP/WHOIS, USPTO/EUIPO
trademark records, and Bluesky handle resolution **could not be verified** in the research
environment, and this limitation was **independently reproduced across two passes** (a third
harness pass was not run for that reason). Every such item is flagged "could not verify" and must
be re-run against live primary sources before any filing or purchase. Name background (Scots
"stell"/"stellin"; the stella/star echo) is established elsewhere — see NAMING.md; not re-derived
here. Feasibility (custom AppView vs Bluesky primitives) is answered by the appview-infra work
(RUN-14 Stellin appview spikes / RUN-15), not this report.`

## TL;DR
- **Verdict: CONTESTED, not blocked — but with a real brand-adjacency risk.** "Stellin" has no dominant software/app incumbent, so "stellin.app" is a viable product brand, but it sits one letter away from "Stellina," the well-documented Vaonis smart-telescope product whose control app was literally branded "Stellinapp," and there is at least one live, long-established company literally named Stellin (Stellin, an Italian labeling-machinery maker). Expect search-ranking competition and phonetic confusion rather than a hard legal wall.
- **Registry/registration specifics could not be verified in this environment.** Live RDAP/WHOIS records for stellin.app and its sibling domains, live USPTO/EUIPO trademark records, and Bluesky handle resolution are all served by JavaScript query tools or direct RDAP JSON endpoints that were not retrievable here (this constraint was independently reproduced by a second research pass). Each such item is flagged "could not verify" and must be re-run against live RDAP and the official trademark databases before any filing or purchase decision.
- **The .app HSTS constraint is confirmed:** every .app domain is HSTS-preloaded at the TLD level, so per The SSL Store, "You are now obligated to use HTTPS with .app domains—they literally will not work without an SSL certificate. You will instead receive a browser error and your site will be inaccessible." There is no "ignore warning" escape hatch. Provision TLS from day one.

## Key Findings

**1. Domain status (stellin.app and siblings).** Could not verify registration status, creation date, or registrar for stellin.app, stellin.com, stellin.io, stellin.net, stellin.dev, stellin.social, getstellin.com, or usestellin.com — the direct RDAP JSON endpoints (rdap.nic.google, rdap.verisign.com, rdap.org) and the who.is/whois.com deep pages were not retrievable in this environment. No prominent live "stellin"-branded software/app site surfaced for any of these candidates. The only closely related live site found was **stellin.it**, the Italian machinery company Stellin. Absence of a visible site does not equal availability; for .app in particular, a registered-but-parked domain may simply not serve content because of the HTTPS requirement.

**2. .app TLD constraint (confirmed).** The .app TLD is on the HSTS preload list at the registry level — Google added the 45 TLDs including .app to the HSTS preload list at the TLD level. Per The SSL Store: "You are now obligated to use HTTPS with .app domains—they literally will not work without an SSL certificate. You will instead receive a browser error and your site will be inaccessible." Comodo, Openprovider, Enom, EuroDNS, and Porkbun all corroborate that browsers refuse HTTP connections to .app with no user override. This is a hard technical requirement, not a recommendation.

**3. Name-collision landscape.**
- **Strongest adjacency — "Stellina" (Vaonis):** Vaonis's first smart telescope was first unveiled at CES in January 2018 and put on sale exclusively at MoMA on May 10, 2018 (T3: "The Vaonis Stellina first launched in spring 2018"). Its companion mobile app was branded "Stellinapp" (com.vaonis.stellina on Google Play / Apple App Store). Stellinapp was then retired on a hard date — per OPT Telescopes/Vaonis, "Stellinapp will no longer be supported from November 1, 2021" — and replaced by Vaonis's "Singularity" app, described as "not just a name change but an entirely new application." Vaonis states: "Although Stellina is discontinued, we continue to offer full support, updates..." This is the single biggest source of search-ranking competition and phonetic confusion for an app called "Stellin," because the query "stellin app" maps directly onto "Stellinapp." The Stellina line remains heavily indexed across retailers and review sites.
- **Exact-name company:** Stellin (Italy), industrial machinery. Confirmed at stellin.it: "Stellin was launched in 1987 thanks to an insight on the part of Giuseppe Stellin"; a second product line (Flexlabeller labelling machines) was added in 2005 by his son Moreno Stellin. A real commercial "Stellin" user, though in an unrelated (packaging-machinery) field.
- **App stores:** No app named exactly "Stellin" was confirmed. Nearby names abound: "Stella" (OwletLabs RSS/Mastodon reader), "Stell App," "Stella – Manifest Anything," "Stellar by ARRI," "STELLA Libraries," and Vaonis's "Stellinapp." The near-namespace is crowded with Stell-/Stella-/Stellar- names.
- **GitHub:** No prominent "stellin" org/repo confirmed; adjacent orgs exist (Stellun, Stellain, Stella-IT, stelligent, stellar). The exact "stellin" name on GitHub was not fully verified.
- **Package registries (npm/PyPI/crates.io):** Not verified in this pass.
- **Fiction/other:** "Stellin Industries" is a fictional corporation in the game Aion — trivia-level, not a commercial conflict.

**4. Trademark exposure.** Could not verify live USPTO or EUIPO records for the exact word "STELLIN" in Nice classes 9 (software), 42 (SaaS/tech), or 38 (telecom) — the USPTO (tmsearch.uspto.gov) and EUIPO (eSearch plus / TMview) systems are JavaScript query interfaces that were not retrievable here. Could not verify whether Vaonis holds a registered "STELLINA" mark, or its owner-of-record, serial/registration number, status, or Nice classes, although Vaonis's commercial use of "STELLINA" as a product brand is well documented. These are the highest-priority items to check directly before filing.

**5. ATProto/Bluesky checks.** Could not verify whether stellin.bsky.social is registered, whether anyone uses stellin.app as a custom-domain handle, or whether any public app.stellin.* lexicon NSID exists. The Bluesky resolveHandle API and profile pages were not retrievable in this environment (a cached, unrelated DID was returned by the tooling and correctly discarded). No app.stellin.* namespace surfaced in searches, but this is absence of evidence, not confirmed absence. Ecosystem norm: an AppView typically defines its own lexicon namespace (e.g., app.bsky.*), so "app.stellin.*" would be the natural NSID root and should be checked/claimed early.

**6. "by Croft" maker-suffix and croft.ing.** "Croft" is heavily used across software and other sectors: the "Croft" Laravel MCP server by Ashley Hindle (usecroft.com); Croft MSP (UK managed service provider); Croft Technology, Inc. (Purdue/High Alpha B2B SaaS); Croft Computer Systems (golf-club accounting software); Croft Production Systems (oil/gas, holds registered CROFT-family trademarks in its own field); plus Croft the sherry/port producer in drinks. None appears to own a dominant, generic "Croft" software brand that would obviously block "Stellin by Croft," but "Croft" is common enough that the parent brand is not distinctive on its own and should not be relied on for differentiation.

## Details

**Why "contested" and not "clear":** The core problem is not legal blockage; it is discoverability collision. A user typing "stellin app" today is most likely to hit Vaonis's "Stellinapp" content, deeply indexed across retailers (OPT, Astroshop) and review sites (Astronomy Now, Galactic Hunter, Space Explored) and app stores. The Croft product would launch into a search-results environment already saturated by a phonetically identical query ("stellin app" ≈ "Stellinapp"). That is a marketing/SEO cost, and — depending on classes — a possible trademark-similarity consideration, since "Stellina" and "Stellin" differ by one character and both would live in software/app classes.

**.app operational requirement:** Because .app is HSTS-preloaded at the TLD level, wildcard/subdomain TLS must be provisioned before any tenant subdomain (e.g., tenant.stellin.app) is usable. This aligns well with ATProto's HTTPS-everywhere posture, but it is a launch-blocking prerequisite, not an afterthought.

**ATProto fit:** ATProto/Bluesky handles are domain-based, and custom domains are set via a DNS TXT record (`_atproto`) or a `/.well-known/atproto-did` file. Owning stellin.app would let the product both host its AppView and offer handles like name.stellin.app to tenants — a clean fit with the multi-tenant plan. The lexicon namespace app.stellin.* would be the conventional NSID root; claiming/publishing it early avoids a later collision. All of these should be verified against live Bluesky/ATProto infrastructure.

**Croft brand adjacency detail:** The most software-relevant "Croft" collision is the Laravel "Croft" MCP tool, but that project is explicitly deprecated — GitHub (github.com/ashleyhindle/croft): "Croft is no longer maintained - check out boost instead!" Croft "was built by Ashley Hindle in collaboration with Springloaded, and would eventually become Laravel Boost" (Boost was previewed at Laracon US 2025 and released as public beta August 2025). That reduces live conflict risk. Croft Production Systems holds registered CROFT trademarks but only in oil/gas equipment classes; the drinks-industry "Croft" is in an unrelated class. Net: "by Croft" carries low hard-conflict risk in software classes but low distinctiveness.

## Recommendations

1. **Immediately re-run the primary-source checks that failed here, against live tools:**
   - RDAP for stellin.app via Google Registry (rdap.nic.google) and for the .com/.io/.net via their registries; capture creation date, registrar, and EPP status codes. Threshold: if stellin.app is unregistered, register it before any public mention of the brand.
   - USPTO (tmsearch.uspto.gov) and EUIPO (eSearch plus / TMview) for exact "STELLIN" and for "STELLINA," filtered to classes 9, 42, 38. Threshold: a live "STELLIN" mark in class 9/42 owned by an unrelated party = escalate to counsel before filing; a live "STELLINA" mark in class 9/42 (likely Vaonis) = get a clearance opinion on confusing similarity.
   - Bluesky: resolve stellin.bsky.social and stellin.app via com.atproto.identity.resolveHandle; check bsky.app/profile for both. Threshold: if stellin.app already resolves to someone else's DID, the custom-domain handle plan needs rework.

2. **Secure the defensive set now if RDAP shows availability:** stellin.app (primary) plus at least stellin.com (redirect/brand protection) and the getstellin/usestellin variants, given how cheap they are relative to a later rebrand. Provision wildcard TLS before launch to satisfy the .app HSTS requirement.

3. **Claim the ATProto surface early:** register the stellin.bsky.social handle, set stellin.app as the org handle once DNS is live, and publish the app.stellin.* lexicon namespace to stake the NSID root.

4. **Mitigate the Stellina/Stellinapp SEO collision deliberately:** brand consistently as "Stellin by Croft" (never "Stellin app," which collides head-on with "Stellinapp"), and invest in disambiguating content so the product ranks for its own name. Benchmark: if after launch "stellin by croft" does not rank on page one within a reasonable indexing window, lean harder on the "by Croft" lockup or reconsider the name.

5. **Decision trigger to reconsider the name entirely:** a live, senior "STELLIN" (exact word) trademark in class 9 or 42 owned by an active software company, OR a Vaonis "STELLINA" registration that counsel judges confusingly similar in software classes. Absent those, proceed.

## Caveats
- Registration status of every stellin.* domain, all live trademark records, and all Bluesky/ATProto handle resolutions **could not be verified** in this environment because the relevant sources (live RDAP JSON endpoints, USPTO/EUIPO JavaScript search tools, and the Bluesky API) were not retrievable — a limitation independently reproduced on a second pass. Every such item is flagged above as "could not verify" and must be confirmed against live systems. Do not treat any domain as available or any name as trademark-clear on the basis of this report alone.
- The .app HSTS/HTTPS requirement is sourced from registrars and certificate authorities (The SSL Store, Comodo, Enom, EuroDNS, Openprovider, Porkbun) that restate Google Registry policy; treat these as corroborating sources for a policy that originates with Google Registry.
- Name-collision findings reflect what is publicly indexed; package-registry (npm/PyPI/crates.io) and exhaustive app-store checks were not completed and should be finished before launch.
- "Stellina" (Vaonis) is discontinued (Stellinapp support ended November 1, 2021; replaced by Singularity) but still supported and heavily indexed; its search footprint is the primary practical risk to "Stellin" discoverability.
