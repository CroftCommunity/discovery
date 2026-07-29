// MV3 service worker. Holds the host_permission for the reader origins, so its
// fetch is not subject to the page's same-origin policy. This is the whole
// mechanism the spike proves: the extension can read what the PWA cannot.
//
// CONSENT / ABUSE GATE (edge walked in v2): the bridge is NOT a blanket proxy.
// It only fetches origins the user has approved. Any page matching the content-
// script glob can *ask*, but a non-allowlisted host is refused by the extension
// itself (not by CORS). In a real extension this list is user-managed; here it
// is a static stand-in that proves enforcement.
const ALLOWED_ORIGINS = new Set(["http://localhost:5602"]);

chrome.runtime.onMessage.addListener((msg, _sender, sendResponse) => {
  if (!msg || msg.kind !== "croft-fetch") return false;
  (async () => {
    try {
      const origin = new URL(msg.url).origin;
      if (!ALLOWED_ORIGINS.has(origin)) {
        sendResponse({ ok: false, refused: true, error: `origin not allowlisted: ${origin}` });
        return;
      }
      const res = await fetch(msg.url);
      const body = await res.text();
      sendResponse({ ok: true, status: res.status, body });
    } catch (e) {
      sendResponse({ ok: false, error: String(e) });
    }
  })();
  return true; // keep the message channel open for the async sendResponse
});
