// Origin-A page logic. Exposes two probes the Playwright spike calls:
//   window.__directFetch(url)  — the PWA's own fetch (expected: CORS-blocked)
//   window.__viaExtension(url) — routed through the extension bridge (expected: ok)
(() => {
  let extReady = false;
  window.addEventListener("message", (ev) => {
    if (ev.source === window && ev.data && ev.data.__croftExtReady) {
      extReady = true;
    }
  });

  window.__extReady = () => extReady;

  window.__directFetch = async (url) => {
    try {
      const res = await fetch(url);
      const body = await res.text();
      return { ok: true, status: res.status, body };
    } catch (e) {
      return { ok: false, error: String(e) };
    }
  };

  window.__viaExtension = (url) =>
    new Promise((resolve) => {
      const id = String(performance.now()) + ":" + url;
      const timer = setTimeout(
        () => resolve({ ok: false, error: "extension timeout (no content script?)" }),
        5000,
      );
      function handler(ev) {
        if (ev.source !== window) return;
        const d = ev.data;
        if (d && d.__croftFetchResponse && d.id === id) {
          clearTimeout(timer);
          window.removeEventListener("message", handler);
          resolve(d.result);
        }
      }
      window.addEventListener("message", handler);
      window.postMessage({ __croftFetchRequest: true, id, url }, "*");
    });
})();
