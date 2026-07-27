import { beginAuthorization, completeAuthorization, type PendingAuth, type OAuthConfig } from './oauth/client.ts';

// stellin.app demo pad — proves a NON-croft.ing pad integrates with the shared
// auth-helper (account.croft.ing), preferring it when reachable and falling back
// to its OWN browser-only public client when the helper is down (FLOW-SPEC §6,
// Stage E). Two independent sign-in paths:
//   - HELPER (brokered): redirect to the helper; it holds the confidential session
//     server-side and we get an opaque ticket (first-party stored, no cross-site
//     cookie — the K1 lesson). Authed calls go through the helper; token never here.
//   - BROWSER-ONLY (fallback): this pad is itself a public OAuth client; if the
//     helper is unreachable the pad signs in directly against the user's PDS.

const HELPER = 'https://account.croft.ing';
const PAD_ORIGIN = location.origin;
const HANDLE = 'ngvalidation2112.bsky.social';
const TICKET_KEY = 'helper_ticket';
const PENDING_KEY = 'pub_pending';

const pubCfg: OAuthConfig = {
  clientId: `${PAD_ORIGIN}/public-client-metadata.json`,
  redirectUri: `${PAD_ORIGIN}/`,
  scope: 'atproto transition:generic',
};

const app = document.getElementById('app')!;
const logEl = document.createElement('pre');
logEl.id = 'log';
function log(msg: string): void {
  logEl.textContent += `${msg}\n`;
}
function el(tag: string, text: string, onclick?: () => void): HTMLElement {
  const e = document.createElement(tag);
  e.textContent = text;
  if (onclick) (e as HTMLButtonElement).onclick = onclick;
  return e;
}

async function helperReachable(): Promise<boolean> {
  try {
    const r = await fetch(`${HELPER}/healthz`, { signal: AbortSignal.timeout(4000) });
    return r.ok;
  } catch {
    return false;
  }
}

async function whoamiBrokered(): Promise<void> {
  const ticket = localStorage.getItem(TICKET_KEY);
  if (!ticket) return log('no helper ticket — sign in via the helper first');
  try {
    const r = await fetch(`${HELPER}/api/whoami`, { headers: { authorization: `Bearer ${ticket}` } });
    const data = await r.json();
    log(`brokered whoami [${r.status}]: ${JSON.stringify(data)}`);
  } catch (e) {
    log(`brokered whoami failed: ${(e as Error).message}`);
  }
}

async function browserOnlySignIn(): Promise<void> {
  log('browser-only: beginning public-client OAuth (no helper)…');
  const { authorizeUrl, pending } = await beginAuthorization(HANDLE, pubCfg);
  sessionStorage.setItem(PENDING_KEY, JSON.stringify(pending));
  location.assign(authorizeUrl);
}

async function handleCallback(params: URLSearchParams): Promise<void> {
  const ticket = params.get('ticket');
  const code = params.get('code');
  const state = params.get('state');
  const error = params.get('error');
  if (error) return log(`authorization error: ${error} ${params.get('error_description') ?? ''}`);
  if (ticket) {
    localStorage.setItem(TICKET_KEY, ticket);
    history.replaceState(null, '', PAD_ORIGIN + '/');
    log('signed in VIA HELPER (brokered). Ticket stored first-party; token stays on the helper.');
    return;
  }
  if (code && state) {
    const raw = sessionStorage.getItem(PENDING_KEY);
    if (!raw) return log('browser-only callback but no pending state');
    const pending = JSON.parse(raw) as PendingAuth;
    const session = await completeAuthorization(pending, { code, state }, pubCfg);
    sessionStorage.removeItem(PENDING_KEY);
    history.replaceState(null, '', PAD_ORIGIN + '/');
    log(`signed in BROWSER-ONLY (public client). DID: ${session.did} · access TTL ~${session.expiresAt ? Math.round((session.expiresAt - Date.now()) / 1000) : '?'}s`);
    return;
  }
}

async function main(): Promise<void> {
  const h1 = el('h1', 'stellin.app — auth-helper integration demo');
  app.append(h1, logEl);

  const btnHelper = el('button', '1. Sign in via helper (brokered)', () => {
    location.assign(`${HELPER}/login?handle=${encodeURIComponent(HANDLE)}&return=${encodeURIComponent(PAD_ORIGIN + '/')}`);
  });
  const btnWho = el('button', '2. Who am I? (brokered call)', () => void whoamiBrokered());
  const btnFallback = el('button', '3. Sign in browser-only (fallback)', () => void browserOnlySignIn());
  app.append(btnHelper, document.createTextNode(' '), btnWho, document.createTextNode(' '), btnFallback);

  await handleCallback(new URLSearchParams(location.search));

  const up = await helperReachable();
  log(`helper /healthz reachable: ${up ? 'YES — preferring the brokered helper session' : 'NO — falling back to the browser-only public client'}`);
  if (!up) log('(the pad still works: use button 3 to sign in browser-only, exactly as it would with no helper at all.)');
}

void main();
