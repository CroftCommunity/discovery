//! `ds-client` — RUN-19 P4: native WebTransport client for the blind DS,
//! speaking the identical protocol a browser page would: real QUIC, HTTP/3
//! CONNECT session establishment, one bidi stream per request, and
//! certificate-hash dev trust (`with_server_certificate_hashes` ≙ the
//! browser's `serverCertificateHashes`).

#![warn(missing_docs)]

use ds_proto::{read_frame, read_json, write_frame, write_json, Request, Response};
use wtransport::tls::Sha256Digest;
use wtransport::{ClientConfig, Connection, Endpoint};

/// Why a DS client operation failed.
#[derive(Debug, thiserror::Error)]
pub enum ClientError {
    /// The QUIC/WebTransport session could not be established (includes the
    /// wrong-cert-hash refusal — the dev-trust negative case).
    #[error("connect: {0}")]
    Connect(String),
    /// The server refused the request — the exact flat refusal string is
    /// carried so tests can compare refusals byte-for-byte.
    #[error("refused: {0}")]
    Refused(String),
    /// Protocol-level failure.
    #[error("proto: {0}")]
    Proto(String),
}

fn connect_err<E: std::fmt::Debug>(e: E) -> ClientError {
    ClientError::Connect(format!("{e:?}"))
}

fn proto_err<E: std::fmt::Debug>(e: E) -> ClientError {
    ClientError::Proto(format!("{e:?}"))
}

/// A WebTransport session to the blind DS.
pub struct DsClient {
    conn: Connection,
}

impl DsClient {
    /// Connect to `url` trusting exactly the pinned SHA-256 cert hash (hex).
    ///
    /// # Errors
    /// [`ClientError::Connect`] if the hash is malformed, the handshake
    /// fails, or the server's certificate does not match the pin.
    pub async fn connect(url: &str, cert_hash_hex: &str) -> Result<Self, ClientError> {
        let digest_bytes: [u8; 32] = hex::decode(cert_hash_hex)
            .map_err(connect_err)?
            .try_into()
            .map_err(|_| ClientError::Connect("hash must be 32 bytes".into()))?;
        // Explicit IPv4: this environment has no IPv6 (`with_bind_default`
        // binds [::] → os error 97, the same family MASTER-INDEX §5 records).
        let config = ClientConfig::builder()
            .with_bind_address(std::net::SocketAddr::from((
                std::net::Ipv4Addr::UNSPECIFIED,
                0,
            )))
            .with_server_certificate_hashes([Sha256Digest::new(digest_bytes)])
            .build();
        let conn = Endpoint::client(config)
            .map_err(connect_err)?
            .connect(url)
            .await
            .map_err(connect_err)?;
        Ok(Self { conn })
    }

    /// One request over one fresh bidi stream (PRED-WT2); returns the
    /// response header and leaves any payload frames to the caller.
    async fn request(
        &self,
        req: &Request,
        payload: Option<&[u8]>,
    ) -> Result<(Response, wtransport::RecvStream), ClientError> {
        let (mut send, mut recv) = self
            .conn
            .open_bi()
            .await
            .map_err(proto_err)?
            .await
            .map_err(proto_err)?;
        write_json(&mut send, req).await.map_err(proto_err)?;
        if let Some(bytes) = payload {
            write_frame(&mut send, bytes).await.map_err(proto_err)?;
        }
        // Best-effort FIN: the length-prefixed framing already delimits the
        // request, and the server may STOP_SENDING as soon as it has read
        // what it needs — a benign race, not a protocol failure.
        let _ = send.finish().await;
        let header: Response = read_json(&mut recv).await.map_err(proto_err)?;
        if !header.ok {
            return Err(ClientError::Refused(
                header.error.unwrap_or_else(|| "unspecified".to_string()),
            ));
        }
        Ok((header, recv))
    }

    /// Admit `member` to the group's offer roster.
    ///
    /// # Errors
    /// [`ClientError`] on refusal or transport failure.
    pub async fn roster_add(&self, group: &str, member: &str) -> Result<(), ClientError> {
        self.request(
            &Request::RosterAdd {
                group: group.to_string(),
                member: member.to_string(),
            },
            None,
        )
        .await
        .map(|_| ())
    }

    /// Remove `member` from the offer roster.
    ///
    /// # Errors
    /// [`ClientError`] on refusal or transport failure.
    pub async fn roster_remove(&self, group: &str, member: &str) -> Result<(), ClientError> {
        self.request(
            &Request::RosterRemove {
                group: group.to_string(),
                member: member.to_string(),
            },
            None,
        )
        .await
        .map(|_| ())
    }

    /// Store one opaque blob at `(group, seq)` as `member`.
    ///
    /// # Errors
    /// [`ClientError`] on refusal or transport failure.
    pub async fn put(
        &self,
        group: &str,
        seq: u64,
        member: &str,
        blob: &[u8],
    ) -> Result<(), ClientError> {
        self.request(
            &Request::Put {
                group: group.to_string(),
                seq,
                member: member.to_string(),
            },
            Some(blob),
        )
        .await
        .map(|_| ())
    }

    /// Fetch every offered blob of `group` from `from_seq`, as `member`.
    ///
    /// # Errors
    /// [`ClientError`] on refusal or transport failure.
    pub async fn fetch(
        &self,
        group: &str,
        from_seq: u64,
        member: &str,
    ) -> Result<Vec<(u64, Vec<u8>)>, ClientError> {
        let (header, mut recv) = self
            .request(
                &Request::Fetch {
                    group: group.to_string(),
                    from_seq,
                    member: member.to_string(),
                },
                None,
            )
            .await?;
        let mut blobs = Vec::with_capacity(header.seqs.len());
        for seq in header.seqs {
            let blob = read_frame(&mut recv).await.map_err(proto_err)?;
            blobs.push((seq, blob));
        }
        Ok(blobs)
    }
}
