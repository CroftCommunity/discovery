// Minimal atproto identity resolution for the confidential helper — handle→DID→PDS.
// Lifted from croft-pwa/src/atproto/read.ts (the resolveHandle/resolvePds pair the
// OAuth resolve chain needs), trimmed to just what resolve.ts imports. Every fetch
// is injectable so the logic is unit-tested hermetically.

export const PUBLIC_APPVIEW = 'https://public.api.bsky.app';
export const PLC_DIRECTORY = 'https://plc.directory';

export interface ReadDeps {
  readonly fetchImpl?: typeof fetch;
  readonly appView?: string;
  readonly plcDirectory?: string;
}

interface DidService {
  readonly id: string;
  readonly type: string;
  readonly serviceEndpoint: string;
}
interface DidDocument {
  readonly id: string;
  readonly service?: readonly DidService[];
}

export class AtprotoReadError extends Error {
  readonly status: number | undefined;
  constructor(message: string, status?: number) {
    super(message);
    this.name = 'AtprotoReadError';
    this.status = status;
  }
}

const fetchOf = (deps: ReadDeps): typeof fetch => deps.fetchImpl ?? globalThis.fetch.bind(globalThis);

async function getJson(res: Response, what: string): Promise<Record<string, unknown>> {
  if (!res.ok) throw new AtprotoReadError(`${what} failed: ${res.status}`, res.status);
  return (await res.json()) as Record<string, unknown>;
}

/** Handle → DID via the public AppView (com.atproto.identity.resolveHandle). */
export async function resolveHandle(handle: string, deps: ReadDeps = {}): Promise<string> {
  const url = new URL('/xrpc/com.atproto.identity.resolveHandle', deps.appView ?? PUBLIC_APPVIEW);
  url.searchParams.set('handle', handle.replace(/^@/, '').trim());
  const data = await getJson(await fetchOf(deps)(url, { headers: { accept: 'application/json' } }), 'resolveHandle');
  if (typeof data.did !== 'string') throw new AtprotoReadError('resolveHandle returned no DID');
  return data.did;
}

function pdsEndpointFromDoc(doc: DidDocument): string | null {
  const svc = (doc.service ?? []).find(
    (s) => s.type === 'AtprotoPersonalDataServer' || s.id === '#atproto_pds' || s.id.endsWith('#atproto_pds'),
  );
  return svc?.serviceEndpoint ?? null;
}

/** DID → PDS endpoint (did:plc via the directory, did:web via .well-known/did.json). */
export async function resolvePds(did: string, deps: ReadDeps = {}): Promise<string> {
  let docUrl: string;
  if (did.startsWith('did:plc:')) {
    docUrl = `${deps.plcDirectory ?? PLC_DIRECTORY}/${did}`;
  } else if (did.startsWith('did:web:')) {
    const rest = did.slice('did:web:'.length);
    const parts = rest.split(':').map(decodeURIComponent);
    const host = parts[0];
    const path = parts.length > 1 ? parts.slice(1).join('/') + '/did.json' : '.well-known/did.json';
    docUrl = `https://${host}/${path}`;
  } else {
    throw new AtprotoReadError(`unsupported DID method: ${did}`);
  }
  const res = await fetchOf(deps)(docUrl, { headers: { accept: 'application/json' } });
  if (!res.ok) throw new AtprotoReadError(`DID resolution failed: ${res.status}`, res.status);
  const doc = (await res.json()) as DidDocument;
  const endpoint = pdsEndpointFromDoc(doc);
  if (!endpoint) throw new AtprotoReadError(`no PDS endpoint in DID document for ${did}`);
  return endpoint.replace(/\/+$/, '');
}
