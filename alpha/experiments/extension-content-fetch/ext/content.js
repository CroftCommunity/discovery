// Injected into the origin-A PWA page (isolated world). Bridges page <-> background:
// the page cannot call chrome.* and cannot fetch origin B; the content script can
// reach the background, which holds the cross-origin host permission.
window.addEventListener("message", (ev) => {
  if (ev.source !== window) return;
  const d = ev.data;
  if (!d || !d.__croftFetchRequest) return;
  chrome.runtime.sendMessage({ kind: "croft-fetch", url: d.url }, (result) => {
    window.postMessage({ __croftFetchResponse: true, id: d.id, result }, "*");
  });
});

// Announce presence so the page (and the test) can tell the extension is live.
window.postMessage({ __croftExtReady: true }, "*");
