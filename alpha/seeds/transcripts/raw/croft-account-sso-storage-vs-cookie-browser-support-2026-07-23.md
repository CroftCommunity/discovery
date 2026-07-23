# Raw: account.croft.ing SSO — storage-vs-cookie split & browser support (2026-07-23)

**Preservation status: preserved-condensed (cleaned-paste, content-faithful — NOT byte-pristine) —
PLAYBOOK §4.** Source: a pasted Claude chat session, 2026-07-23 (the assistant's own web-research pass is
included). UI render chrome stripped ("Searched the web", "This response didn't load", "Show more"). The
source paste **duplicated the talk-through**; reproduced once. No secrets. Part of the 2026-07-23 batch
(plan: `alpha/plans/2026-07-23-transcript-intake-socialization-arecipe-measurement.md`).

**Status: web-VERIFIED this pass** (the user asked "research this and confirm it's expected"). Unlike the
rest of the 2026-07-23 batch, this transcript's load-bearing claims were checked against primary sources —
the "Independent verification" section below is the settled record and **supersedes the original talk-
through's mechanism framing** (which called it "third-party partitioning"). Belongs to the uncommitted
2026-07-22 `account.croft.ing` account-kernel thread; ties to the `K1` (measured) and `KC1` (untested-but-
sound) browser-behavior results recorded there.

---

## The talk-through (as pasted)

**User:** Can you research this and confirm it's expected? And we think the `account.croft.ing` mock SSO
would work with Chrome, but not the Safari engine?

**Assistant:** Right — and we don't just think it, we measured it (K1). The storage-based `account.croft.ing`
SSO — where `account.croft.ing` is a client-side iframe holding the session and each subdomain reads it out
of shared storage — is exactly cross-subdomain shared storage. So:
- **Chrome/Chromium:** works (shares the session across subdomains).
- **Safari/WebKit:** fails (partitions it — each subdomain's `account.croft.ing` iframe gets its own empty
  store).

A serverless / "mock" SSO (no server, just the shared-storage trick) is Chrome-only — the K1 result applied
to auth specifically. The split depends on what it rides on:

```
SSO via shared STORAGE  (iframe, client-only, "mock")  → Chrome ✓   Safari ✗   (K1)
SSO via COOKIE          (Domain=croft.ing)             → Chrome ✓   Safari ✓   (cookies aren't storage-partitioned)
```

Cookies cross subdomains on Safari fine — but the catch pulls the server back in: a cookie is a bearer
token, and the atproto session is DPoP-bound (every request needs proof-of-possession of a key). A cookie
alone can't carry that; the DPoP key lives in per-origin storage. So to make cookie-SSO usable on Safari, a
small server (the BFF) holds the DPoP key and the cookie authenticates the app to it. That's why "SSO that
also works on Safari" ⇒ needs the BFF. Net:
- **Serverless storage-SSO** → Chrome-only (dead on iOS).
- **Cross-browser SSO** → cookie + BFF (a small server), works on Safari too — untested but sound (KC1).
- **Per-app login** → works everywhere, no server, no SSO (the serverless default).

The BFF isn't needed for function or for per-app login, but it is the thing that makes sign-in-once SSO work
on Safari/iOS. Without a server, cross-subdomain SSO is a Chrome-only trick.

## The assistant's own research corrections (as pasted)

**What's right:** the top-line result — serverless shared-storage SSO is Chromium-only, and cross-browser
sign-in-once needs a small server (the BFF). The reasons just aren't the ones stated.

**What's wrong:**
1. **The Safari failure is not "third-party partitioning" — it's same-party partitioning, which is worse for
   this design.** The `account.croft.ing` iframe embedded under `app.croft.ing` is same-site (shared eTLD+1).
   Under the old third-party model that would be fine. The actual problem: Safari partitions `localStorage`
   keyed to the combination of source and target domains, and applies it by the top-level page's domain, so
   `example.com` and `help.example.com` each get separate partitions for the same embedded iframe —
   inconsistent with browsers that partition by top-level origin. A WebKit quirk treating sibling subdomains
   as separate partition keys, not standard third-party isolation. If the doc explains it as "third-party
   partitioning," a future reader will wrongly assume SameSite / Storage Access API / first-party status
   fixes it. It won't, without a user gesture and prompt.
2. **"A cookie alone can't carry DPoP" is right, but the sharper reason is that atproto builds the DPoP key
   as non-extractable, so it physically cannot ride in a cookie.** The load-bearing fact isn't that the key
   is in partitioned storage; it's that if the private key is non-extractable (as with the Web Crypto API),
   DPoP renders exfiltrated tokens alone unusable, and the atproto browser client stores it exactly this way.
   IndexedDB is used because it is the only storage mechanism that allows storing `CryptoKeyPair` objects
   with non-extractable private keys. A non-extractable `CryptoKey` can't be serialized into a cookie even in
   principle.

**One correction to the BFF rationale:** the BFF path is recommended primarily because when a backend server
is available it can manage OAuth sessions server-side and use a session cookie to map to the front-end —
letting the backend invalidate credentials at scale — and the OAuth server provides longer-lived tokens when
issued to a BFF. The DPoP-key-server-side variant is still an open proposal (a token-mediating backend where
DPoP proofs are constructed client-side and the key lives client-side, yet the server must negotiate DPoP
nonces with the client). So "BFF holds the DPoP key" is one design under discussion, not the standard atproto
BFF.

## Independent verification (this pass, 2026-07-23 — settled record)

Confirmed against primary sources. The transcript's top-line conclusion is **expected and correct**; the two
corrections above are the accurate framing; one nuance was missing.

1. **Serverless shared-storage SSO is Chromium-only — expected.** WebKit partitions web storage by the
   top-level page's domain and treats sibling subdomains as separate partition keys; Chrome/Firefox
   partition by the registrable domain, so the shared store survives across `*.croft.ing`. Apple's developer
   forum states it directly and notes it is inconsistent with WebKit's own definition ("a website is a
   registrable domain including all of its subdomains"). Matches the K1 measurement.
   (Sources: developer.apple.com/forums/thread/725074; cookiestatus.com/safari; webkit.org/tracking-prevention)
2. **Same-party (top-level-subdomain) partitioning, not third-party isolation — confirmed.**
3. **The cookie can't carry the DPoP session because the key is non-extractable — confirmed.** The atproto
   browser client generates a non-extractable WebCrypto `CryptoKey` persisted in IndexedDB (the only web
   storage holding live `CryptoKeyPair` objects); JS can never read its bytes.
   (Sources: atproto.com/specs/oauth; docs.bsky.app OAuth client; @atproto/oauth-client-browser;
   InfoQ "The DPoP Storage Paradox")
4. **Added nuance (missing from the talk-through):** a `Domain=croft.ing` cookie *does* cross subdomains on
   Safari as first-party, but a cookie set via `document.cookie` (JS) is **capped at 7 days** by ITP; only a
   server-`Set-Cookie` (HTTP header, ideally `HttpOnly`) first-party cookie persists (~400 days). This
   *reinforces* the BFF conclusion — the durable SSO cookie must be server-set, which is exactly what a BFF
   does; a pure client-set cookie SSO would silently decay after a week on iOS.

**Net (corrected):**
```
Serverless storage-SSO → Chromium-only, dead on iOS         (K1; confirmed WebKit behavior)
Cross-browser SSO      → server-set cookie + BFF            (KC1; sound — cookie must be HTTP-set)
Per-app login          → works everywhere, serverless       (the serverless default)
```
The architecture recommendation (single-origin core + isolated subdomains; per-app login as the serverless
default; BFF only when sign-in-once must work on Safari/iOS) stands. Whether the DPoP key sits client- or
server-side remains an unsettled atproto design question.
