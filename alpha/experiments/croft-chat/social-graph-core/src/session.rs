//! `Session` — the ergonomic substrate facade.
//!
//! One type that opens a redb-backed store with the P4 ed25519 adapters and
//! exposes tenant-agnostic graph operations + queries, hiding redb and the
//! `traits::*` / `types::*` newtype bridge. Write commands are async (the
//! substrate's are); the TUI bridges sync↔async at P11.
//!
//! Replication: [`Session::export_group_log`] yields a group's assertion frames
//! and [`Session::apply_remote`] applies a peer's frame. The fold enforces
//! per-device lamport order, so a transport must deliver each device's frames in
//! lamport order (the P7 layer sorts drained frames before applying).

use std::path::Path;
use std::sync::Arc;

use local_storage_projection::surface::{
    CommandResult, GroupListItemView, GroupSummaryView, LocalStore, MessageView,
    NotificationReceiver, TimelineView, TimelineWindow,
};
use local_storage_projection::fold_derived::rule_change_approval_subject;
use local_storage_projection::tables::Db;
use local_storage_projection::traits::{
    DeviceId as TraitsDeviceId, PrincipalId as TraitsPrincipalId,
};
use local_storage_projection::types::{
    AssertionType, GroupId, Hash, KindTag, PrincipalId, Role, TypedId,
};
use thiserror::Error;

use crate::crypto::{
    Ed25519Signer, Ed25519Verifier, MonotonicLamport, RegistryCredentialResolver,
};
use crate::identity::Identity;

/// The concrete store type backing a session.
type Store = LocalStore<Ed25519Verifier, RegistryCredentialResolver, MonotonicLamport>;

/// Errors surfaced by [`Session`] operations.
#[derive(Debug, Error)]
pub enum SessionError {
    /// A storage / surface error from the substrate.
    #[error("storage error: {0}")]
    Storage(String),
    /// The command was rejected by the fold (authorization, malformed, etc.).
    #[error("command rejected: {0}")]
    Rejected(String),
    /// The command needs more signatures (threshold not met).
    #[error("command pending signatures: have {have}, need {need}")]
    Pending {
        /// Signatures collected so far.
        have: u32,
        /// Signatures required.
        need: u32,
    },
}

/// A named channel within a group (an `ArtifactChat` attachment).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ChannelRef {
    /// The channel's typed id (used to scope sends and timelines).
    pub id: TypedId,
    /// The channel's display name.
    pub name: String,
}

/// Outcome of applying a remote frame via [`Session::apply_remote`].
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ApplyOutcome {
    /// The assertion was applied for the first time.
    Applied,
    /// The assertion was already present (idempotent no-op).
    Duplicate,
}

/// A session over one identity's view of the social graph.
pub struct Session {
    store: Store,
    signer: Ed25519Signer,
    resolver: RegistryCredentialResolver,
    lamport: MonotonicLamport,
    my_principal: PrincipalId,
}

impl Session {
    /// Open (or create) a redb store at `path` for `identity`.
    ///
    /// Registers the identity's own (device, principal) credential and **resumes
    /// the lamport counter past the device's persisted maximum** — on a fresh
    /// store the high-water mark is 0 so the counter starts at 1; on restart it
    /// continues past prior writes, avoiding a self-`LamportViolation`.
    ///
    /// # Errors
    /// [`SessionError::Storage`] if the database cannot be opened.
    pub fn open(path: &Path, identity: &Identity) -> Result<Self, SessionError> {
        let db = Arc::new(Db::open(path).map_err(|e| SessionError::Storage(e.to_string()))?);
        let verifier = Ed25519Verifier;
        let resolver = RegistryCredentialResolver::new();
        resolver.register(identity.device_id(), identity.principal_id());
        let my_principal = PrincipalId::new(identity.principal_id().0);

        // Resume the lamport past whatever this device already persisted.
        let my_device = local_storage_projection::types::DeviceId::new(identity.device_id().0);
        let resume_from = {
            // A short-lived store just to read the high-water mark.
            let probe = LocalStore::new(
                Arc::clone(&db),
                verifier,
                resolver.clone(),
                MonotonicLamport::new(),
                my_principal,
            );
            probe.max_lamport_for_device(&my_device) + 1
        };
        let lamport = MonotonicLamport::starting_at(resume_from);

        let store = LocalStore::new(
            Arc::clone(&db),
            verifier,
            resolver.clone(),
            lamport.clone(),
            my_principal,
        );
        let signer = Ed25519Signer::new(identity);
        Ok(Self {
            store,
            signer,
            resolver,
            lamport,
            my_principal,
        })
    }

    /// This session's principal id.
    #[must_use]
    pub fn my_principal(&self) -> PrincipalId {
        self.my_principal
    }

    /// Trust a peer's (device, principal) credential so its assertions verify
    /// on ingest. Needed before [`Session::apply_remote`] of that peer's frames.
    pub fn trust_peer(&self, device: TraitsDeviceId, principal: TraitsPrincipalId) {
        self.resolver.register(device, principal);
    }

    /// Subscribe to change notifications (timeline/membership/group-state).
    #[must_use]
    pub fn subscribe(&self) -> NotificationReceiver {
        self.store.subscribe()
    }

    // -- writes (async) -----------------------------------------------------

    /// Create a new group; the caller becomes its owner. Returns the group id.
    ///
    /// Genesis alone makes the founder an owner in the group *state* but writes
    /// no `MemberOf` edge, so the group would not appear in
    /// [`Session::list_my_groups`] (which scans those edges). We therefore follow
    /// genesis with an explicit owner `MembershipAdd` (an upsert — it does not
    /// duplicate the founder) so the founder is a first-class member with an edge.
    ///
    /// # Errors
    /// [`SessionError`] if the create is rejected or the store errors.
    pub async fn create_group(&self) -> Result<GroupId, SessionError> {
        let group =
            require_applied(self.store.create_group(&self.signer).await.map_err(surface_err)?)?;
        require_applied(
            self.store
                .add_member(&group, self.my_principal, Role::Owner, &self.signer)
                .await
                .map_err(surface_err)?,
        )?;
        Ok(group)
    }

    /// Add `principal` to `group` with `role`.
    ///
    /// # Errors
    /// [`SessionError`] if the add is rejected or the store errors.
    pub async fn add_member(
        &self,
        group: &GroupId,
        principal: PrincipalId,
        role: Role,
    ) -> Result<(), SessionError> {
        let result = self
            .store
            .add_member(group, principal, role, &self.signer)
            .await
            .map_err(surface_err)?;
        require_applied(result)
    }

    /// Send a message to `group` (optionally replying to `reply_to`). Returns
    /// the message's content hash.
    ///
    /// # Errors
    /// [`SessionError`] if the send is rejected or the store errors.
    pub async fn send_message(
        &self,
        group: &GroupId,
        body: &str,
        reply_to: Option<Hash>,
    ) -> Result<Hash, SessionError> {
        let result = self
            .store
            .send_message(group, None, body.to_string(), reply_to, &self.signer)
            .await
            .map_err(surface_err)?;
        require_applied(result)
    }

    // -- governance ---------------------------------------------------------

    /// Propose a rule change (amend `rule_key` → `new_value`). `approvals` are the
    /// hashes of `Approval` facts backing it — required when the rule-change threshold
    /// in effect is > 1, empty otherwise. Returns the change's hash.
    ///
    /// # Errors
    /// [`SessionError`] if rejected (e.g. below quorum) or the store errors.
    pub async fn propose_rule_change(
        &self,
        group: &GroupId,
        rule_key: u8,
        new_value: u32,
        approvals: Vec<Hash>,
    ) -> Result<Hash, SessionError> {
        let result = self
            .store
            .rule_change(group, rule_key, new_value, approvals, &self.signer)
            .await
            .map_err(surface_err)?;
        require_applied(result)
    }

    /// Approve a proposed rule change (`rule_key` → `new_value`), naming its content-hash
    /// subject so the fold counts this approver toward the change's quorum. Returns the
    /// approval's hash for the proposer to reference as an antecedent.
    ///
    /// # Errors
    /// [`SessionError`] if rejected (e.g. approver lacks Owner/Admin) or the store errors.
    pub async fn approve_rule_change(
        &self,
        group: &GroupId,
        rule_key: u8,
        new_value: u32,
    ) -> Result<Hash, SessionError> {
        let mut payload = Vec::with_capacity(5);
        payload.push(rule_key);
        payload.extend_from_slice(&new_value.to_be_bytes());
        let subject = PrincipalId::new(rule_change_approval_subject(&payload));
        let result = self
            .store
            .approve(group, AssertionType::RuleChange, subject, &self.signer)
            .await
            .map_err(surface_err)?;
        require_applied(result)
    }

    /// Propose (enact) a `MembershipRemove` of `principal`. `approvals` are the hashes of
    /// `Approval` facts backing it — required when the remove-member threshold in effect
    /// is > 1, empty otherwise. Returns the removal's hash. Mirrors the rule-change shape:
    /// the enacting act references the approvals as its co-signed-op antecedents.
    ///
    /// # Errors
    /// [`SessionError`] if rejected (e.g. below quorum, or the author lacks Owner/Admin)
    /// or the store errors.
    pub async fn propose_remove_member(
        &self,
        group: &GroupId,
        principal: PrincipalId,
        approvals: Vec<Hash>,
    ) -> Result<Hash, SessionError> {
        let result = self
            .store
            .remove_member(group, principal, approvals, &self.signer)
            .await
            .map_err(surface_err)?;
        require_applied(result)
    }

    /// Approve a proposed `MembershipRemove` of `principal`, naming the target principal
    /// as the subject so the fold counts this approver toward the removal's quorum.
    /// Returns the approval's hash for the proposer to reference as an antecedent.
    ///
    /// # Errors
    /// [`SessionError`] if rejected (e.g. approver lacks Owner/Admin) or the store errors.
    pub async fn approve_remove_member(
        &self,
        group: &GroupId,
        principal: PrincipalId,
    ) -> Result<Hash, SessionError> {
        let result = self
            .store
            .approve(group, AssertionType::MembershipRemove, principal, &self.signer)
            .await
            .map_err(surface_err)?;
        require_applied(result)
    }

    /// Propose (enact) a `RoleGrant` setting `principal`'s role to `new_role`. `approvals`
    /// are the hashes of `Approval` facts backing it — required when the role-change
    /// threshold in effect is > 1, empty otherwise. Returns the grant's hash.
    ///
    /// # Errors
    /// [`SessionError`] if rejected (e.g. below quorum, or the author is not Owner) or the
    /// store errors.
    pub async fn propose_role_grant(
        &self,
        group: &GroupId,
        principal: PrincipalId,
        new_role: Role,
        approvals: Vec<Hash>,
    ) -> Result<Hash, SessionError> {
        let result = self
            .store
            .grant_role(group, principal, new_role, approvals, &self.signer)
            .await
            .map_err(surface_err)?;
        require_applied(result)
    }

    /// Approve a proposed `RoleGrant` of `principal`, naming the target principal as the
    /// subject so the fold counts this approver toward the grant's quorum. Returns the
    /// approval's hash for the proposer to reference as an antecedent.
    ///
    /// # Errors
    /// [`SessionError`] if rejected (e.g. approver lacks Owner/Admin) or the store errors.
    pub async fn approve_role_grant(
        &self,
        group: &GroupId,
        principal: PrincipalId,
    ) -> Result<Hash, SessionError> {
        let result = self
            .store
            .approve(group, AssertionType::RoleGrant, principal, &self.signer)
            .await
            .map_err(surface_err)?;
        require_applied(result)
    }

    /// Propose (enact) a `RoleRevoke` withdrawing `principal`'s elevated role (the fold
    /// demotes them to `Member`). `approvals` are the hashes of `Approval` facts backing
    /// it — required when the role-change threshold in effect is > 1, empty otherwise.
    /// Returns the revoke's hash.
    ///
    /// # Errors
    /// [`SessionError`] if rejected (e.g. below quorum, or the author is not Owner) or the
    /// store errors.
    pub async fn propose_role_revoke(
        &self,
        group: &GroupId,
        principal: PrincipalId,
        approvals: Vec<Hash>,
    ) -> Result<Hash, SessionError> {
        let result = self
            .store
            .revoke_role(group, principal, approvals, &self.signer)
            .await
            .map_err(surface_err)?;
        require_applied(result)
    }

    /// Approve a proposed `RoleRevoke` of `principal`, naming the target principal as the
    /// subject so the fold counts this approver toward the revoke's quorum. Returns the
    /// approval's hash for the proposer to reference as an antecedent.
    ///
    /// # Errors
    /// [`SessionError`] if rejected (e.g. approver lacks Owner/Admin) or the store errors.
    pub async fn approve_role_revoke(
        &self,
        group: &GroupId,
        principal: PrincipalId,
    ) -> Result<Hash, SessionError> {
        let result = self
            .store
            .approve(group, AssertionType::RoleRevoke, principal, &self.signer)
            .await
            .map_err(surface_err)?;
        require_applied(result)
    }

    // -- channels -----------------------------------------------------------

    /// Create a named channel in `group`. Returns its typed id.
    ///
    /// # Errors
    /// [`SessionError`] if rejected or the store errors.
    pub async fn create_channel(
        &self,
        group: &GroupId,
        name: &str,
    ) -> Result<TypedId, SessionError> {
        let result = self
            .store
            .attach(group, KindTag::ArtifactChat, name.to_string(), None, &self.signer)
            .await
            .map_err(surface_err)?;
        require_applied(result)
    }

    /// List the channels (`ArtifactChat` attachments) of `group`.
    ///
    /// # Errors
    /// [`SessionError::Storage`] if the store errors.
    pub fn list_channels(&self, group: &GroupId) -> Result<Vec<ChannelRef>, SessionError> {
        let attachments = self.store.list_group_attachments(group).map_err(surface_err)?;
        Ok(attachments
            .into_iter()
            .filter(|a| a.kind == KindTag::ArtifactChat)
            .map(|a| ChannelRef {
                id: a.id,
                name: a.title,
            })
            .collect())
    }

    /// Send a message to a specific `channel` within `group`.
    ///
    /// # Errors
    /// [`SessionError`] if rejected or the store errors.
    pub async fn send_to_channel(
        &self,
        group: &GroupId,
        channel: TypedId,
        body: &str,
        reply_to: Option<Hash>,
    ) -> Result<Hash, SessionError> {
        let result = self
            .store
            .send_message(group, Some(channel), body.to_string(), reply_to, &self.signer)
            .await
            .map_err(surface_err)?;
        require_applied(result)
    }

    /// Timeline scoped to a single `channel` within `group`.
    ///
    /// # Errors
    /// [`SessionError::Storage`] if the store errors.
    pub fn get_channel_timeline(
        &self,
        group: &GroupId,
        channel: &TypedId,
        window: TimelineWindow,
    ) -> Result<TimelineView, SessionError> {
        self.store
            .get_channel_timeline(group, channel, window)
            .map_err(surface_err)
    }

    // -- replication --------------------------------------------------------

    /// Every assertion frame for `group`, in lamport order (for publishing).
    ///
    /// # Errors
    /// [`SessionError::Storage`] if the store errors.
    pub fn export_group_log(&self, group: &GroupId) -> Result<Vec<Vec<u8>>, SessionError> {
        let log = self.store.export_group_log(group).map_err(surface_err)?;
        Ok(log.into_iter().map(|(_, bytes)| bytes).collect())
    }

    /// Apply a peer's assertion frame.
    ///
    /// # Errors
    /// [`SessionError`] if the frame is rejected (e.g. arrived before its
    /// per-device predecessor) or the store errors. Callers retry rejected
    /// frames after their predecessors land.
    pub fn apply_remote(&self, frame: &[u8]) -> Result<ApplyOutcome, SessionError> {
        match self.store.ingest_foreign(frame).map_err(surface_err)? {
            CommandResult::Applied(_) => Ok(ApplyOutcome::Applied),
            CommandResult::Duplicate => Ok(ApplyOutcome::Duplicate),
            CommandResult::Rejected { reason } => Err(SessionError::Rejected(reason)),
            CommandResult::PendingSignatures { have, need } => {
                Err(SessionError::Pending { have, need })
            }
        }
    }

    /// The value the lamport source would next issue (for persistence, P12).
    #[must_use]
    pub fn lamport_peek(&self) -> u64 {
        self.lamport.peek()
    }

    // -- queries (sync) -----------------------------------------------------

    /// List the groups this principal is a member of.
    ///
    /// # Errors
    /// [`SessionError::Storage`] if the store errors.
    pub fn list_my_groups(&self) -> Result<Vec<GroupListItemView>, SessionError> {
        self.store.list_my_groups().map_err(surface_err)
    }

    /// Summary (members, etc.) for `group`.
    ///
    /// # Errors
    /// [`SessionError::Storage`] if the store errors.
    pub fn get_group_summary(&self, group: &GroupId) -> Result<GroupSummaryView, SessionError> {
        self.store.get_group_summary(group).map_err(surface_err)
    }

    /// Timeline for `group` within `window`.
    ///
    /// # Errors
    /// [`SessionError::Storage`] if the store errors.
    pub fn get_timeline(
        &self,
        group: &GroupId,
        window: TimelineWindow,
    ) -> Result<TimelineView, SessionError> {
        self.store.get_timeline(group, window).map_err(surface_err)
    }

    /// Resolve a message hash to its body and reply linkage.
    #[must_use]
    pub fn get_message(&self, hash: &Hash) -> Option<MessageView> {
        self.store.get_message(hash)
    }
}

fn surface_err(e: impl std::fmt::Debug) -> SessionError {
    // SurfaceError is Debug-only; render it that way.
    SessionError::Storage(format!("{e:?}"))
}

fn require_applied<T>(result: CommandResult<T>) -> Result<T, SessionError> {
    match result {
        CommandResult::Applied(value) => Ok(value),
        CommandResult::Rejected { reason } => Err(SessionError::Rejected(reason)),
        CommandResult::PendingSignatures { have, need } => {
            Err(SessionError::Pending { have, need })
        }
        CommandResult::Duplicate => Err(SessionError::Rejected("duplicate".to_string())),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn open_session(seed: u8) -> (tempfile::TempDir, Session) {
        let dir = tempfile::tempdir().expect("tempdir");
        let identity = Identity::from_seed([seed; 32]);
        let session =
            Session::open(&dir.path().join("store.redb"), &identity).expect("open session");
        (dir, session)
    }

    #[tokio::test]
    async fn create_add_send_read_cycle() {
        let (_dir, session) = open_session(0xA1);

        let group = session.create_group().await.expect("create_group");

        // Add a second principal.
        let second = Identity::from_seed([0xA2; 32]).principal_id();
        session
            .add_member(&group, PrincipalId::new(second.0), Role::Member)
            .await
            .expect("add_member");

        // Send and read back via timeline + get_message.
        let hash = session
            .send_message(&group, "hello session", None)
            .await
            .expect("send_message");

        let timeline = session
            .get_timeline(&group, TimelineWindow::LastN(10))
            .expect("timeline");
        assert!(
            timeline.entries.iter().any(|e| e.hash == hash),
            "sent message appears in the timeline"
        );

        let message = session.get_message(&hash).expect("get_message");
        assert_eq!(message.body, "hello session");
        assert_eq!(message.author, session.my_principal());

        let summary = session.get_group_summary(&group).expect("summary");
        assert_eq!(summary.members.len(), 2, "owner + added member");
    }

    #[tokio::test]
    async fn list_my_groups_reflects_creation() {
        let (_dir, session) = open_session(0xB1);
        assert_eq!(session.list_my_groups().expect("list").len(), 0);
        session.create_group().await.expect("create");
        assert_eq!(
            session.list_my_groups().expect("list").len(),
            1,
            "created group is listed"
        );
    }

    #[tokio::test]
    async fn channels_have_isolated_timelines() {
        let (_dir, session) = open_session(0xE1);
        let group = session.create_group().await.expect("create");

        let chan_a = session.create_channel(&group, "general").await.expect("chan a");
        let chan_b = session.create_channel(&group, "photos").await.expect("chan b");

        session.send_to_channel(&group, chan_a, "hi a", None).await.expect("send a");
        session.send_to_channel(&group, chan_b, "hi b", None).await.expect("send b");

        let a_bodies: Vec<String> = session
            .get_channel_timeline(&group, &chan_a, TimelineWindow::LastN(50))
            .expect("a tl")
            .entries
            .iter()
            .filter_map(|e| session.get_message(&e.hash).map(|m| m.body))
            .collect();
        assert_eq!(a_bodies, vec!["hi a".to_string()], "a sees only its message");

        let b_bodies: Vec<String> = session
            .get_channel_timeline(&group, &chan_b, TimelineWindow::LastN(50))
            .expect("b tl")
            .entries
            .iter()
            .filter_map(|e| session.get_message(&e.hash).map(|m| m.body))
            .collect();
        assert_eq!(b_bodies, vec!["hi b".to_string()], "b sees only its message");

        // Both channels are listed.
        let names: Vec<String> = session
            .list_channels(&group)
            .expect("list channels")
            .into_iter()
            .map(|c| c.name)
            .collect();
        assert!(names.contains(&"general".to_string()) && names.contains(&"photos".to_string()));
    }

    #[tokio::test]
    async fn state_persists_and_lamport_resumes_across_reopen() {
        let dir = tempfile::tempdir().expect("tempdir");
        let path = dir.path().join("persist.redb");
        let identity = Identity::from_seed([0xD1; 32]);

        // First session: create a group, send a message, then drop the session.
        let group = {
            let session = Session::open(&path, &identity).expect("open 1");
            let group = session.create_group().await.expect("create");
            session.send_message(&group, "before restart", None).await.expect("send 1");
            group
        };

        // Reopen the same store. Prior state must be there, and a new send must
        // apply (the lamport resumed past the persisted max — no self-violation).
        let session = Session::open(&path, &identity).expect("open 2");
        assert_eq!(
            session.list_my_groups().expect("list").len(),
            1,
            "group persisted across reopen"
        );
        session
            .send_message(&group, "after restart", None)
            .await
            .expect("send after restart must apply");

        let timeline = session
            .get_timeline(&group, TimelineWindow::LastN(10))
            .expect("timeline");
        let bodies: Vec<String> = timeline
            .entries
            .iter()
            .filter_map(|e| session.get_message(&e.hash).map(|m| m.body))
            .collect();
        assert!(bodies.contains(&"before restart".to_string()));
        assert!(bodies.contains(&"after restart".to_string()));
    }

    #[tokio::test]
    async fn rapid_sends_within_one_second_all_apply() {
        // Exercises the P2 lamport fix through the real Session/ed25519 stack.
        let (_dir, session) = open_session(0xC1);
        let group = session.create_group().await.expect("create");
        for i in 0..5 {
            session
                .send_message(&group, &format!("msg {i}"), None)
                .await
                .unwrap_or_else(|e| panic!("send {i} must apply: {e}"));
        }
        let timeline = session
            .get_timeline(&group, TimelineWindow::LastN(10))
            .expect("timeline");
        assert_eq!(timeline.entries.len(), 5, "all five rapid sends applied");
    }
}
