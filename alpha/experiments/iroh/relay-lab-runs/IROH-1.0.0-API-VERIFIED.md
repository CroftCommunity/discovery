# iroh 1.0.0 API — verified against source (not guessed)

Per the "verify against source, never guess" rule. Every symbol below was read from the pinned
1.0.0 source extracted under `.node4-cargo/registry/src/.../{iroh,iroh-relay,iroh-base}-1.0.0`.
Citations are `crate/path:line`. This is the surface the E0 harness is built on.

## Client endpoint (generator)

- `Endpoint::builder(preset: impl Preset) -> Builder` — `iroh/src/endpoint.rs:940`.
- Presets — `iroh/src/endpoint/presets.rs`: `Empty` (sets nothing), `Minimal` (crypto provider only;
  needs `tls-ring`/`tls-aws-lc-rs` — in default features), `N0` (full n0 DNS+relays). **Lab uses
  `presets::Minimal`** + explicit relay/bind.
- Builder: `.secret_key(SecretKey)` `:524`, `.alpns(Vec<Vec<u8>>)` `:535`,
  `.bind_addr(A)` `:363`, `.relay_mode(RelayMode)` `:557`, `.ca_tls_config(CaTlsConfig)` `:713`,
  `.bind().await -> Endpoint`.
- `RelayMode::Custom(RelayMap)` / `RelayMode::custom(impl IntoIterator<Item=RelayUrl>)` — `:1950`.
- `endpoint.online().await` `:1345`, `endpoint.addr() -> EndpointAddr` `:1186`.
- `endpoint.connect(impl Into<EndpointAddr>, alpn: &[u8]).await -> Result<Connection, ConnectError>`
  — `:1040`.

## Forced relay-only passthrough (E0 worst case)

- `EndpointAddr::from_parts(id: PublicKey, addrs: impl IntoIterator<Item=TransportAddr>)` —
  `iroh-base/src/endpoint_addr.rs:104`.
- `TransportAddr::Relay(RelayUrl)` — `:57`; `.is_relay()` `:67`. A relay-only addr =
  `from_parts(peer_id, [TransportAddr::Relay(url)])` — no direct IPs ⇒ relay path only.

## Path detection (the core E0 metric)

- `conn.paths() -> PathList` — `iroh/src/endpoint/connection.rs:1095` (snapshot at call time).
- `PathList::iter() -> PathListIter` over `Path`.
- `Path` — `iroh/src/socket/remote_map/remote_state/path_watcher.rs:453`:
  `.is_selected() -> bool` `:470` (the live path), `.is_relay() -> bool` `:480`,
  `.is_ip() -> bool` `:475`, `.rtt() -> Duration` `:494`, `.stats() -> PathStats` `:489`,
  `.remote_addr() -> &TransportAddr` `:460`.
- **Live-path classification:** `conn.paths().iter().find(|p| p.is_selected())` then `.is_relay()` +
  `.rtt()`. (`conn.paths().iter().any(|p| p.is_relay())` per spec also works for "is a relay path open".)
- `conn.paths_stream()` `:1108` / `conn.path_events()` `:1127` to watch matchmaking flip relay→direct.

## Relay server (E0 single relay)

- `iroh_relay::server::RelayConfig::new((ip, port))` — `iroh-relay/src/server.rs:150` (server config;
  alias as `RelayServerConfig`). Fields: `.tls: Option<TlsConfig>` (None ⇒ plain HTTP), `.access:
  Arc<dyn DynAccessControl>`, `.key_cache_capacity`.
- Self-signed certs: `iroh_relay::server::testing::self_signed_tls_certs_and_config() ->
  (Vec<CertificateDer>, rustls::ServerConfig)` — `iroh-relay/src/server/testing.rs:9` (SANs:
  localhost, 127.0.0.1, ::1).
- `TlsConfig::new((ip,0), CertConfig::Manual { server_config })`.
- `ServerConfig::default()` then `.relay = Some(relay)`, `.quic = Some(QuicConfig::new((ip,0)))`.
- `Server::spawn(ServerConfig).await -> Result<Server, SpawnError>` — `:691`; `server.https_addr()`,
  `server.quic_addr()`.
- Client relay map: `RelayMap = iroh::RelayConfig::new(url: RelayUrl, Option<RelayQuicConfig>).into()`
  — `iroh-relay/src/relay_map.rs:250`; `RelayQuicConfig::new(port)` `:299`.

## Admit hook (E3; E0 baseline uses AllowAll)

- `trait AccessControl { async fn on_connect(&self, &ClientRequest) -> Access }` —
  `iroh-relay/src/server.rs:285`; `AllowAll` `:340`.
- `enum Access { Allow, Deny { reason: Option<String> } }` — `:350`.

## Lab TLS posture (⚠️ LAB-ONLY)

Self-signed relay cert SANs are localhost/127.0.0.1/::1; cross-VPC relay URLs use box IPs, so client
cert verification would fail. The lab uses `CaTlsConfig::insecure_skip_verify()`
(`iroh-relay/src/tls.rs:94`, gated behind `test-utils`) on the generator endpoints.
⚠️ WARNING: insecure_skip_verify disables relay-server cert verification — acceptable only because
this is a closed sandbox VPC measuring relay capacity over iroh's own encrypted layer; never ship it.
Alternative for a hardened run: `CaTlsConfig::custom_roots([cert_der])` + a relay URL whose host is in
the cert SANs.

## Deferred-to-controller (E2) — not yet verified, flagged for task 6

`DnsAddressLookup::builder(origin)`, the in-memory/static address lookup + `add_endpoint_info`
(rc.1 `MemoryLookup` → 1.0 name TBD), `PkarrPublisher`. Verify against source before building ctrl/.
