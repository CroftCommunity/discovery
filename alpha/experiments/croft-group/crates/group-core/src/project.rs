//! The projection: `&Model -> ViewModel`. The single, pure model->view
//! conversion (honest seams — the only place this mapping happens).

use crate::model::Model;
use crate::view::{MessageLine, ViewModel};

/// Project the model into a platform-agnostic view model: the joined flag plus
/// the messages as ordered lines. Pure; the only model->view conversion.
#[must_use]
pub fn project(model: &Model) -> ViewModel {
    ViewModel {
        joined: model.joined,
        lines: model
            .messages
            .iter()
            .map(|message| MessageLine {
                sender: message.sender.clone(),
                text: message.text.clone(),
            })
            .collect(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::model::{ChatMessage, Model};

    #[test]
    fn projects_messages_in_order_with_the_joined_flag() {
        let model = Model {
            messages: vec![
                ChatMessage {
                    sender: "alice".to_string(),
                    text: "one".to_string(),
                },
                ChatMessage {
                    sender: "bob".to_string(),
                    text: "two".to_string(),
                },
            ],
            joined: true,
        };
        let view = project(&model);
        assert!(view.joined, "a joined model projects joined == true");
        assert_eq!(
            view.lines,
            vec![
                MessageLine {
                    sender: "alice".to_string(),
                    text: "one".to_string(),
                },
                MessageLine {
                    sender: "bob".to_string(),
                    text: "two".to_string(),
                },
            ],
            "lines preserve message order"
        );
    }

    #[test]
    fn an_empty_unjoined_model_projects_no_lines() {
        let view = project(&Model::new());
        assert!(!view.joined, "a fresh model projects joined == false");
        assert!(view.lines.is_empty(), "no messages -> no lines");
    }
}
