//! The model: the messages received so far plus whether we have joined.

/// One chat message in its native domain shape (the pond's own type — the
/// analog of the feed pond's `Post`, DECISION 2). The wire format
/// (`wire.rs`) serializes this; the transport never inspects it.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ChatMessage {
    /// Who sent the message (a hardcoded handle placeholder in this slice;
    /// real identity arrives in L1).
    pub sender: String,
    /// The message body.
    pub text: String,
}

/// The whole core state: the messages seen so far and whether we have joined
/// the group. A fresh model is empty and unjoined.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Model {
    /// Messages in arrival order (own optimistic appends and decoded inbound).
    pub messages: Vec<ChatMessage>,
    /// Whether this peer has joined the group (subscribed to its topic).
    pub joined: bool,
}

impl Model {
    /// A fresh, never-joined model with no messages.
    #[must_use]
    pub fn new() -> Self {
        Model {
            messages: Vec::new(),
            joined: false,
        }
    }
}

impl Default for Model {
    fn default() -> Self {
        Model::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fresh_model_has_no_messages_and_is_unjoined() {
        let model = Model::new();
        assert!(
            model.messages.is_empty(),
            "a fresh model carries no messages"
        );
        assert!(!model.joined, "a fresh model has not joined a group yet");
    }
}
