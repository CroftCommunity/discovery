//! The view model: a platform-agnostic projection of the model, ready for any
//! shell (CLI/web/desktop) to render. Carries no CLI strings or formatting —
//! that is each shell's `render` concern.

/// One rendered message line, platform-agnostic (no formatting baked in).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MessageLine {
    /// Who sent the message.
    pub sender: String,
    /// The message body.
    pub text: String,
}

/// The whole render-ready view: whether we have joined, and the ordered lines.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ViewModel {
    /// Whether this peer has joined the group.
    pub joined: bool,
    /// Messages in arrival order, ready to render.
    pub lines: Vec<MessageLine>,
}
