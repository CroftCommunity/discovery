//! Render a `ViewModel` to text for the terminal. Pure (view in, string out),
//! so it is easy to assert against; `main` prints the result. The platform-
//! agnostic `ViewModel` lives in the core; the CLI formatting lives here.

use group_core::ViewModel;

/// Render a view model to a plain string.
#[must_use]
pub fn render(view: &ViewModel) -> String {
    if !view.joined {
        return "(not joined)".to_string();
    }
    if view.lines.is_empty() {
        return "(joined — no messages yet)".to_string();
    }
    view.lines
        .iter()
        .map(|line| format!("{}: {}", line.sender, line.text))
        .collect::<Vec<_>>()
        .join("\n")
}

#[cfg(test)]
mod tests {
    use super::*;
    use group_core::MessageLine;

    #[test]
    fn renders_each_message_as_sender_colon_text_in_order() {
        let view = ViewModel {
            joined: true,
            lines: vec![
                MessageLine {
                    sender: "alice".to_string(),
                    text: "one".to_string(),
                },
                MessageLine {
                    sender: "bob".to_string(),
                    text: "two".to_string(),
                },
            ],
        };
        let out = render(&view);
        let alice_at = out.find("alice: one").expect("alice's line is rendered");
        let bob_at = out.find("bob: two").expect("bob's line is rendered");
        assert!(alice_at < bob_at, "lines render in arrival order");
    }

    #[test]
    fn renders_an_empty_joined_view_without_panicking() {
        let out = render(&ViewModel {
            joined: true,
            lines: vec![],
        });
        assert!(
            out.contains("no messages"),
            "an empty joined view shows a placeholder, got {out:?}"
        );
    }
}
