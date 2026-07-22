# Raw: Croft card-maker webxdc packaging + push notifications — design dialogue (2026-07-22)

**Preservation status: preserved-condensed (cleaned-paste, content-faithful — NOT byte-pristine) —
PLAYBOOK §4.** Source: a claude.ai design dialogue pasted 2026-07-22. UI render chrome stripped
(duplicated thinking-summary headers "Reconciled maker hosting with card portability across formats" /
"Weighed Pushover's limitations against Web Push viability"; "Added to memory" confirmations). No
secrets. Part of the 2026-07-22 batch (plan: `alpha/plans/2026-07-22-transcript-intake-ponds-kernel-ens.md`).
This is the card-maker packaging + push-notification thread; it weaves into the `account.croft.ing`
kernel thread (filed separately as `croft-encrypted-prefs-repo-mirror-and-account-kernel-2026-07-22.md`)
and the greetings.croft.ing MVP (ROADMAP_TODO E43). Web-Push / iOS facts are protocol-vs-product; where
they touch atproto/iroh/iOS the FACTCHECK is the source of truth.

---

## Thread 1 — the card, not the maker; and only when the card is collaborative (assistant)

**The maker is a page you arrive at.** Nobody hands you a card maker. You go to `greetings.croft.ing`
to make one. It is single-user, so the shared update log is dead weight, and it is asset-heavy in
exactly the ways the format handles badly: template libraries, fonts, photo uploads, file input and
output. By the send-versus-arrive line, it is a hosted PWA. That mirrors the games line neatly: the
level maker is an internal hosted tool, the game is the portable artifact. The tool is a page, the
output is a file.

**The card only needs to be an app if people write on it together.** A solo card is a static artifact —
image, text, maybe an animation. Wrapping that in a container gives you a runtime for something with no
state, and it makes the file harder to open for no gain. The case that earns webxdc is the group card:
the office birthday card passed around for signatures, or the family card where six relatives each add a
message before it reaches your grandmother. Send the `.xdc` into a family group chat, everyone hits
Start, everyone adds their bit, the card fills up. That is precisely what the update log is for, and it
is a genuinely warm thing the incumbents charge for and host forever.

**The output ladder is three rungs — pick the dumbest one that works:**
1. An **image file** for maximum compatibility. Opens anywhere, in any messenger, on any device, forever.
2. A **single self-contained HTML file** when it should animate or respond. Still no app runtime, still
   emailable, still opens in any browser.
3. A **`.xdc`** only when it needs shared state that accumulates across people.

**The limit that bites hardest here.** A `.xdc` is inert without a host. Games are opt-in by someone who
went looking; a greeting card goes to your aunt, who has whatever her phone came with. Greeting cards
have close to the worst possible recipient profile for a novel file format. Which means the collaborative
card cannot be file-only. It needs a hosted URL that opens the same card with the same accumulated
signatures, so the `.xdc` is the enhancement for people already inside a host and the link is the floor
for everyone else. That is the same-build-two-targets pattern arriving where it matters most: write the
collaborative card against the capability interface, render it through the kernel host at a URL, package
it as `.xdc` for messenger contexts, and the signatures land in the same log either way.

## Thread 2 — Croft Chat as host + an open card endpoint (user)

Seems like we could easily take greeting cards for example and offer online or offline packaging and then
our own Croft Chat can handle both nicely — but we have an endpoint to open and share webxdc cards easily
ourselves, so here's a card, opens in Croft. Then it's just a static asset format we can reliably have an
OS render on our viewer and it can easily convert to static-whatever too for storing offline or sharing
other ways.

## Thread 2 — response: the endpoint is load-bearing (assistant)

That endpoint is the load-bearing piece, and it fixes the exact objection raised. A `.xdc` is inert
without a host, and a greeting card has the worst recipient profile of anything you will ship, because it
goes to people who did not opt into anything. A link that opens the card in Croft makes the file an
enhancement rather than a requirement. Croft Chat gets the native experience, everyone else gets a URL,
and the same card is behind both.

Two specifics worth pinning while it is cheap:

**The untrusted renderer cannot share a registrable domain with your session cookie.** A public "open any
card someone sent you" endpoint is, structurally, a URL that runs a stranger's code. That is fine and it
is what sandboxes are for, but it collides directly with the shared-auth design, where a
`Domain=croft.ing` cookie is readable across the estate. A renderer at `cards.croft.ing` would sit inside
that cookie's scope. Either the untrusted renderer lives on a separate registrable domain entirely, or
the estate session is host-only and the kernel brokers access rather than a domain-wide cookie doing it.
Both are workable. Choosing accidentally is not, and this is the kind of thing that is a one-line config
decision today and a migration once three apps depend on it. The sandbox itself then needs to be real
rather than nominal: opaque origin, strict CSP, no network, no reach into the kernel frame — which is
what webxdc already assumes, so you are inheriting the posture rather than inventing it.

**Carry the static rendering, do not derive it.** "Converts to static whatever" is much more reliable if
the card ships with its static representation as a declared asset rather than being produced by running
the app and capturing it. Rendering-to-capture needs a runtime, a headless browser, and a moment in time.
Extracting a bundled PNG or single-file HTML needs a zip reader. So the manifest declares a static
fallback, and everything downstream gets easy: the link preview uses it, email uses it, the archive uses
it, and in ten years when nothing runs the app, the card still opens — the survivability position applied
to an artifact a person actually cares about keeping. The one honest wrinkle: for a collaborative card the
static form is a snapshot as of a moment, so it needs regenerating as signatures accumulate, and the
exported copy should say when it was taken.

**Croft Chat is the natural host, and the endpoint keeps it from being a dependency.** webxdc came out of
a messenger because a messenger is where the update log has an obvious transport and an obvious group.
Croft Chat inherits that fit directly. The discipline is just that the richest path stays optional: if
the only way to open a card well is to install your chat app, you have rebuilt the thing you are trying
to avoid.

User: Agreed.

## Thread 3 — Pushover vs Web Push (user)

Is https://pushover.net/ a viable multiplatform push notifications option rather than using per-ecosystem
special impl?

## Thread 3 — response: not for recipients; Web Push is the answer (assistant)

Not for anything a recipient touches. Three structural problems, and the first one alone settles it.

1. **The recipient has to buy it.** Pushover is a $4.99 one-time purchase on each platform where you want
   to receive notifications (iPhone/iPad, Android, Desktop separately), after a 30-day trial. You cannot
   ship a greeting card whose notification path requires your aunt to purchase a third-party app on each
   of her devices. Their licensing API does let you buy credits and assign licenses to users, but paying
   per user per platform for a free consumer product inverts your whole economics.
2. **It requires a server holding a secret.** Their docs are explicit that API tokens should be protected
   and not disclosed to third parties, so the token cannot live in a client. You would need a server
   holding that token plus a durable mapping of Croft user to Pushover user key — precisely the durable
   authoritative state your own contract minimizes, and a credential to defend.
3. **A third party reads the payload.** Pushover's API is deliberately simple, with no OAuth-style
   complexity and no payload encryption, so message contents pass through their servers in the clear.
   That contradicts the no-third-party-requests line drafted an hour ago.

There is also a volume ceiling: 10,000 messages per month free per account as of the May 2026 change,
with paid capacity above that. `[UNVERIFIED — product-level, volatile]`

**Where it is genuinely right: your own operations.** Build failed, PDS unreachable, relay down,
certificate expiring. That is Pushover's actual sweet spot and it is very good at it. Worth adopting there
and nowhere near the product surface.

**Web Push is the multiplatform answer you're looking for.** It is one W3C standard across all of them.
WebKit's own announcement puts it plainly: it is the same standards-based Web Push added for macOS, and if
you implemented it with feature detection rather than browser detection it automatically works on iPhone
and iPad. It runs over APNs underneath, but you never touch APNs, and you do not need Apple Developer
Program membership. So the per-ecosystem implementation you are trying to avoid is already done for you, by
the browser vendors, behind one API. And unlike Pushover, payloads are encrypted to the subscription keys
so the push service cannot read them — the property that lets you put the subscription in a sealed record
and have peers wake each other with no Croft server in the path.

**The honest catch.** On iOS, Web Push requires iOS 16.4 or later and the web app must be installed to the
Home Screen. A Safari tab cannot receive push notifications; the Safari team documented this as
intentional, and since every iOS browser uses WebKit, Chrome does not route around it. There is also no
automatic install prompt on iOS, so the user adds it manually through Share → Add to Home Screen. That
means the residual per-ecosystem work is not implementation, it is onboarding copy for iPhone users. And
it stacks neatly with the storage decision from earlier: Home Screen install already earned its place for
persistent storage; now it also gates notifications. Two independent reasons to make that flow good.
`[atproto/iOS facts — cite the FACTCHECK SoT; iOS 16.4 Web-Push + Home-Screen requirement is
product-level, treat as UNVERIFIED against a primary source.]`

---

### Orphaned opening line (preserved for fidelity)

The paste opened with a line that does not connect to the card/push substance and appears to be a stray
prompt fragment: *"Could I build a PWA/SPA with PDS backend, candy-crush alternative? Local LLM usage
needed?"* Preserved here rather than in-thread; likely belongs to the games-pond thread
(`croft-games-pond-roadmap-browser-p2p-phased-build-2026-07-22.md`).
