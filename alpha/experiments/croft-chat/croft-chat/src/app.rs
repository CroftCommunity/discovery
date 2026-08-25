//! `App` — the shell's mutable state: the session, the tenant model, and focus.
//!
//! The app owns the ports (here, the `Session`); it syncs the pure
//! `chat-core` model (the croft pond, E117 P5) from session queries
//! (`refresh`) and performs the model's effects against the session. The
//! session's ChannelRef converts to the pond's at this boundary — the pond
//! depends on the substrate only, never on the facade.

use chat_core::{
    project, update, ChatView, Effect, GroupRef, Intent, MessageLine, Model, Snapshot, TreeRow,
};
use social_graph_core::{GroupId, PrincipalId, Session, TimelineWindow, TypedId};

use crate::input::Action;
use std::path::PathBuf;

/// Which pane has keyboard focus.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Focus {
    /// The left-pane group tree.
    Tree,
    /// The right-pane message input.
    Input,
}

/// The shell application state.
pub struct App {
    session: Session,
    my_principal: PrincipalId,
    /// Where the personal mute set persists (hex lines) — local truth,
    /// never folded, never on the wire (E134). `None` in tests without a
    /// store directory.
    muted_path: Option<PathBuf>,
    model: Model,
    focus: Focus,
    tree_cursor: usize,
}

impl App {
    /// Build an app over `session`.
    #[must_use]
    pub fn new(session: Session) -> Self {
        let my_principal = session.my_principal();
        Self::build(session, my_principal)
    }

    fn build(session: Session, my_principal: PrincipalId) -> Self {
        Self {
            session,
            my_principal,
            muted_path: None,
            model: Model::default(),
            focus: Focus::Tree,
            tree_cursor: 0,
        }
    }

    /// Attach the mute-set file: seeds the model from what was persisted
    /// and persists future toggles there.
    #[must_use]
    pub fn with_muted_file(mut self, path: PathBuf) -> Self {
        let seeded: Vec<PrincipalId> = std::fs::read_to_string(&path)
            .ok()
            .map(|text| {
                text.lines()
                    .filter_map(|line| {
                        let mut bytes = [0u8; 32];
                        let line = line.trim();
                        if line.len() != 64 {
                            return None;
                        }
                        for (i, chunk) in line.as_bytes().chunks(2).enumerate() {
                            let hi = (chunk[0] as char).to_digit(16)?;
                            let lo = (chunk[1] as char).to_digit(16)?;
                            bytes[i] = ((hi << 4) | lo) as u8;
                        }
                        Some(PrincipalId::new(bytes))
                    })
                    .collect()
            })
            .unwrap_or_default();
        self.muted_path = Some(path);
        if !seeded.is_empty() {
            let _ = self.apply(Intent::SetMuted(seeded));
        }
        self
    }

    /// Perform the pond's non-Send effects (mute persistence). Send effects
    /// are handled in the async action path.
    fn perform_local(&mut self, effects: &[Effect]) {
        for effect in effects {
            if let Effect::PersistMuted(set) = effect {
                if let Some(path) = &self.muted_path {
                    let text: String = set
                        .iter()
                        .map(|p| {
                            p.as_bytes()
                                .iter()
                                .map(|b| format!("{b:02x}"))
                                .collect::<String>()
                                + "\n"
                        })
                        .collect();
                    if let Err(e) = std::fs::write(path, text) {
                        tracing::error!("persisting mute set failed: {e}");
                    }
                }
            }
        }
    }

    /// Toggle the personal mute for `principal` (E134: an annotation on the
    /// edge, cross-group, local truth).
    pub fn toggle_mute(&mut self, principal: PrincipalId) {
        let effects = self.apply(Intent::ToggleMute(principal));
        self.perform_local(&effects);
    }

    /// Resolve a hex prefix against the visible principals (member rows,
    /// then timeline authors). `None` when nothing or more than one matches.
    fn resolve_principal_prefix(&self, prefix: &str) -> Option<PrincipalId> {
        let prefix = prefix.trim().to_lowercase();
        if prefix.is_empty() {
            return None;
        }
        let hex = |p: &PrincipalId| -> String {
            p.as_bytes().iter().map(|b| format!("{b:02x}")).collect()
        };
        let mut candidates: Vec<PrincipalId> = Vec::new();
        for p in self
            .model
            .members
            .iter()
            .map(|m| m.principal)
            .chain(self.model.timeline.iter().filter_map(|l| l.author_principal))
        {
            if hex(&p).starts_with(&prefix) && !candidates.contains(&p) {
                candidates.push(p);
            }
        }
        match candidates.as_slice() {
            [one] => Some(*one),
            _ => None,
        }
    }

    /// The tree cursor row (which group is highlighted for selection).
    #[must_use]
    pub fn tree_cursor(&self) -> usize {
        self.tree_cursor
    }

    /// Handle one mapped action. Returns `false` when the app should quit.
    ///
    /// Bridges the synchronous core to the async session: `Submit` in the input
    /// performs the `Send` effect with an awaited `send_message`, then refreshes.
    pub async fn perform(&mut self, action: Action) -> bool {
        match action {
            Action::Quit => return false,
            Action::ToggleFocus => self.toggle_focus(),
            Action::SelectPrev => self.move_cursor_back(),
            Action::SelectNext => self.move_cursor_forward(),
            Action::Input(c) => {
                let _ = self.apply(Intent::TypeChar(c));
            }
            Action::Backspace => {
                let _ = self.apply(Intent::Backspace);
            }
            Action::Submit => self.submit().await,
        }
        true
    }

    async fn submit(&mut self) {
        match self.focus {
            Focus::Tree => {
                match self.cursored_row() {
                    Some(TreeRow::Group(g)) => {
                        self.select_group(g.id);
                        self.focus = Focus::Input;
                    }
                    Some(TreeRow::Channel(c)) => {
                        self.select_channel(c.id);
                        self.focus = Focus::Input;
                    }
                    None => {}
                }
            }
            Focus::Input => {
                // The mute register, reachable from the input line (E116's
                // lightest register): "/mute <hex-prefix>" toggles the
                // personal mute instead of sending. Parsed here because
                // commands are shell vocabulary, not pond vocabulary.
                let draft = self.model.draft.trim().to_string();
                if let Some(prefix) = draft.strip_prefix("/mute ") {
                    if let Some(principal) = self.resolve_principal_prefix(prefix) {
                        self.toggle_mute(principal);
                    } else {
                        tracing::warn!("/mute: no unique principal matches '{prefix}'");
                    }
                    // Clear the draft through the pond's own vocabulary.
                    for _ in 0..self.model.draft.chars().count() {
                        let _ = self.apply(Intent::Backspace);
                    }
                    self.refresh();
                    return;
                }
                let effects = self.apply(Intent::SendMessage);
                for effect in effects {
                    if let Effect::Send { group, channel, body } = effect {
                        let result = match channel {
                            Some(ch) => self.session.send_to_channel(&group, ch, &body, None).await,
                            None => self.session.send_message(&group, &body, None).await,
                        };
                        if let Err(e) = result {
                            tracing::error!("send failed: {e}");
                        }
                    }
                }
                // Reload so the confirmed message replaces the optimistic line.
                self.refresh();
            }
        }
    }

    fn move_cursor_back(&mut self) {
        self.tree_cursor = self.tree_cursor.saturating_sub(1);
    }

    fn move_cursor_forward(&mut self) {
        let last = self.view().tree.rows.len().saturating_sub(1);
        self.tree_cursor = (self.tree_cursor + 1).min(last);
    }

    fn cursored_row(&self) -> Option<TreeRow> {
        self.view().tree.rows.get(self.tree_cursor).cloned()
    }

    /// The session handle (so the shell can perform effects / replicate).
    #[must_use]
    pub fn session(&self) -> &Session {
        &self.session
    }

    /// The current focus.
    #[must_use]
    pub fn focus(&self) -> Focus {
        self.focus
    }

    /// Toggle focus between the two panes.
    pub fn toggle_focus(&mut self) {
        self.focus = match self.focus {
            Focus::Tree => Focus::Input,
            Focus::Input => Focus::Tree,
        };
    }

    /// The renderable view.
    #[must_use]
    pub fn view(&self) -> ChatView {
        project(&self.model)
    }

    /// The selected group, if any.
    #[must_use]
    pub fn selected_group(&self) -> Option<GroupId> {
        self.model.selected_group
    }

    /// Apply an intent to the model, returning the emitted effects.
    pub fn apply(&mut self, intent: Intent) -> Vec<Effect> {
        let (model, effects) = update(std::mem::take(&mut self.model), intent);
        self.model = model;
        effects
    }

    /// Select a group and load its (group-level) timeline.
    pub fn select_group(&mut self, group: GroupId) {
        let _ = self.apply(Intent::SelectGroup(group));
        self.refresh();
    }

    /// Select a channel within the current group and load its timeline.
    pub fn select_channel(&mut self, channel: TypedId) {
        let _ = self.apply(Intent::SelectChannel(channel));
        self.refresh();
    }

    /// Pull fresh group/channel/timeline data from the session into the model.
    pub fn refresh(&mut self) {
        let groups: Vec<GroupRef> = self
            .session
            .list_my_groups()
            .unwrap_or_default()
            .into_iter()
            .map(|g| GroupRef {
                id: g.group_id,
                title: short_id(&g.group_id),
                member_count: g.member_count,
            })
            .collect();

        let selected_group = self.model.selected_group;
        let selected_channel = self.model.selected_channel;

        let channels = match selected_group {
            Some(g) => self
                .session
                .list_channels(&g)
                .unwrap_or_default()
                .into_iter()
                .map(|c| chat_core::ChannelRef { id: c.id, name: c.name })
                .collect(),
            None => Vec::new(),
        };

        let timeline = match selected_group {
            Some(g) => self.load_timeline(&g, selected_channel),
            None => Vec::new(),
        };

        // The selected group's summary feeds both the fork banner and the
        // truthful membership panel: seated members, then the subjects of
        // open contradictions ("membership pending resolution"), then the
        // standing ceiling ("admission voided") — the fold's truth, never a
        // cleaner list.
        let (fork, members) = match selected_group.and_then(|g| self.session.get_group_summary(&g).ok()) {
            Some(summary) => {
                let fork = Some(summary.fork_status.clone()).filter(|status| status != "clean");
                let mut rows: Vec<chat_core::MemberRow> = summary
                    .members
                    .iter()
                    .map(|m| chat_core::MemberRow {
                        principal: m.principal,
                        role: format!("{:?}", m.role).to_lowercase(),
                        standing: chat_core::Standing::Seated,
                    })
                    .collect();
                rows.extend(summary.contested.iter().map(|p| chat_core::MemberRow {
                    principal: *p,
                    role: String::new(),
                    standing: chat_core::Standing::PendingResolution,
                }));
                rows.extend(summary.banned.iter().map(|p| chat_core::MemberRow {
                    principal: *p,
                    role: String::new(),
                    standing: chat_core::Standing::Voided,
                }));
                (fork, rows)
            }
            None => (None, Vec::new()),
        };

        let _ = self.apply(Intent::Refresh(Snapshot {
            groups,
            channels,
            group: selected_group,
            channel: selected_channel,
            timeline,
            fork,
            members,
        }));
    }

    fn load_timeline(&self, group: &GroupId, channel: Option<TypedId>) -> Vec<MessageLine> {
        let view = match channel {
            Some(ch) => self
                .session
                .get_channel_timeline(group, &ch, TimelineWindow::LastN(usize::MAX)),
            None => self
                .session
                .get_timeline(group, TimelineWindow::LastN(usize::MAX)),
        };
        let timeline = match view {
            Ok(t) => t,
            Err(_) => return Vec::new(),
        };
        let mut lines: Vec<MessageLine> = timeline
            .entries
            .iter()
            .filter_map(|entry| {
                self.session.get_message(&entry.hash).map(|m| MessageLine {
                    lamport: m.lamport,
                    author: self.author_label(&m.author),
                    author_principal: Some(m.author),
                    body: m.body,
                })
            })
            .collect();
        lines.sort_by_key(|l| l.lamport);
        lines
    }

    fn author_label(&self, principal: &PrincipalId) -> String {
        if *principal == self.my_principal {
            "me".to_string()
        } else {
            short_principal(principal)
        }
    }
}

/// First 4 bytes of a group id as hex (a compact, stable label — groups carry no
/// title in the substrate today).
#[must_use]
pub fn short_id(group: &GroupId) -> String {
    hex_prefix(group.as_bytes(), 4)
}

fn short_principal(principal: &PrincipalId) -> String {
    hex_prefix(principal.as_bytes(), 3)
}

fn hex_prefix(bytes: &[u8; 32], n: usize) -> String {
    let mut s = String::with_capacity(n * 2);
    for b in bytes.iter().take(n) {
        s.push_str(&format!("{b:02x}"));
    }
    s
}

#[cfg(test)]
mod tests {
    use super::{short_id, App};
    use crate::input::Action;
use std::path::PathBuf;
    use crate::ui;
    use ratatui::backend::TestBackend;
    use ratatui::Terminal;
    use social_graph_core::{Identity, Session, TimelineWindow};

    #[tokio::test]
    async fn typing_and_submitting_sends_and_displays_message() {
        // P11 wiring test: scripted keys -> update -> async send -> refresh ->
        // render, and the message is readable back through the session.
        let dir = tempfile::tempdir().expect("tempdir");
        let identity = Identity::from_seed([0x11; 32]);
        let session = Session::open(&dir.path().join("s.redb"), &identity).expect("open");
        session.create_group().await.expect("create_group");

        let mut app = App::new(session);
        app.refresh(); // cursor at the new group

        // Submit in tree focus selects the group and moves focus to the input.
        assert!(app.perform(Action::Submit).await);
        // Type "hi" and send.
        app.perform(Action::Input('h')).await;
        app.perform(Action::Input('i')).await;
        app.perform(Action::Submit).await;

        // The timeline pane shows the message.
        let backend = TestBackend::new(70, 12);
        let mut terminal = Terminal::new(backend).expect("terminal");
        terminal
            .draw(|frame| ui::draw(frame, &app.view(), app.focus()))
            .expect("draw");
        let text = ui::buffer_to_string(terminal.backend().buffer());
        assert!(text.contains("hi"), "sent message shown in timeline: {text}");

        // And it is readable via the session.
        let group = app.selected_group().expect("a group is selected");
        let timeline = app
            .session()
            .get_timeline(&group, TimelineWindow::LastN(10))
            .expect("timeline");
        let last = timeline.entries.last().expect("a message entry");
        let message = app.session().get_message(&last.hash).expect("get_message");
        assert_eq!(message.body, "hi");
    }

    #[tokio::test]
    async fn selecting_a_channel_switches_the_timeline_pane() {
        // P15 wiring test: with two channels, selecting #photos shows only its
        // message in the right pane.
        let dir = tempfile::tempdir().expect("tempdir");
        let identity = Identity::from_seed([0x15; 32]);
        let session = Session::open(&dir.path().join("s.redb"), &identity).expect("open");
        let group = session.create_group().await.expect("create");
        let general = session.create_channel(&group, "general").await.expect("c1");
        let photos = session.create_channel(&group, "photos").await.expect("c2");
        session.send_to_channel(&group, general, "in general", None).await.expect("s1");
        session.send_to_channel(&group, photos, "a photo", None).await.expect("s2");

        let mut app = App::new(session);
        app.select_group(group);
        app.select_channel(photos);

        let backend = TestBackend::new(70, 14);
        let mut terminal = Terminal::new(backend).expect("terminal");
        terminal
            .draw(|frame| ui::draw(frame, &app.view(), app.focus()))
            .expect("draw");
        let text = ui::buffer_to_string(terminal.backend().buffer());

        assert!(text.contains("a photo"), "photos channel message shown: {text}");
        assert!(!text.contains("in general"), "general's message not shown when #photos selected");
        // Both channels appear in the tree.
        assert!(text.contains("#photos") && text.contains("#general"), "channels nested in tree");
    }

    #[tokio::test]
    async fn app_renders_created_group_in_left_pane() {
        // Wiring test: session -> App.refresh -> project -> ui::draw, end to end.
        let dir = tempfile::tempdir().expect("tempdir");
        let identity = Identity::from_seed([0x10; 32]);
        let session = Session::open(&dir.path().join("s.redb"), &identity).expect("open");
        let group = session.create_group().await.expect("create_group");

        let mut app = App::new(session);
        app.refresh();

        let backend = TestBackend::new(70, 12);
        let mut terminal = Terminal::new(backend).expect("terminal");
        terminal
            .draw(|frame| ui::draw(frame, &app.view(), app.focus()))
            .expect("draw");
        let text = ui::buffer_to_string(terminal.backend().buffer());

        assert!(
            text.contains(&short_id(&group)),
            "left pane shows the created group ({}): {text}",
            short_id(&group)
        );
        assert!(text.contains("(1)"), "the owner counts as one member");
    }
}
