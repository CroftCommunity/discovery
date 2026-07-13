//! Key → [`Action`] mapping.
//!
//! Pure and terminal-independent: `map_key` turns a crossterm `KeyEvent` plus the
//! current [`Focus`] into an optional [`Action`], so the event loop and tests
//! share one source of truth for the keybindings.

use ratatui::crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

use crate::app::Focus;

/// A high-level action derived from a keypress.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Action {
    /// Quit the app.
    Quit,
    /// Move focus between the tree and the input.
    ToggleFocus,
    /// Move the tree cursor up.
    SelectPrev,
    /// Move the tree cursor down.
    SelectNext,
    /// Confirm: select the cursored group (tree focus) or send (input focus).
    Submit,
    /// Type a character into the draft (input focus).
    Input(char),
    /// Delete the last draft character (input focus).
    Backspace,
}

/// Map a key event (given the current focus) to an action, if any.
#[must_use]
pub fn map_key(key: KeyEvent, focus: Focus) -> Option<Action> {
    // Ctrl-C always quits.
    if key.modifiers.contains(KeyModifiers::CONTROL) && key.code == KeyCode::Char('c') {
        return Some(Action::Quit);
    }
    match key.code {
        KeyCode::Tab => Some(Action::ToggleFocus),
        KeyCode::Esc => Some(Action::Quit),
        KeyCode::Enter => Some(Action::Submit),
        KeyCode::Up => Some(Action::SelectPrev),
        KeyCode::Down => Some(Action::SelectNext),
        KeyCode::Backspace if focus == Focus::Input => Some(Action::Backspace),
        KeyCode::Char(c) => match focus {
            // In the tree, 'q' quits; other chars are ignored.
            Focus::Tree if c == 'q' => Some(Action::Quit),
            Focus::Tree => None,
            // In the input, every character is typed (no 'q' shortcut).
            Focus::Input => Some(Action::Input(c)),
        },
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn ev(code: KeyCode) -> KeyEvent {
        KeyEvent::new(code, KeyModifiers::empty())
    }

    #[test]
    fn typing_in_input_yields_input_actions() {
        assert_eq!(map_key(ev(KeyCode::Char('a')), Focus::Input), Some(Action::Input('a')));
        assert_eq!(map_key(ev(KeyCode::Backspace), Focus::Input), Some(Action::Backspace));
    }

    #[test]
    fn q_quits_only_in_tree_focus() {
        assert_eq!(map_key(ev(KeyCode::Char('q')), Focus::Tree), Some(Action::Quit));
        // In the input, 'q' is a literal character, not a quit.
        assert_eq!(map_key(ev(KeyCode::Char('q')), Focus::Input), Some(Action::Input('q')));
    }

    #[test]
    fn ctrl_c_always_quits() {
        let key = KeyEvent::new(KeyCode::Char('c'), KeyModifiers::CONTROL);
        assert_eq!(map_key(key, Focus::Input), Some(Action::Quit));
        assert_eq!(map_key(key, Focus::Tree), Some(Action::Quit));
    }

    #[test]
    fn enter_is_submit_tab_toggles() {
        assert_eq!(map_key(ev(KeyCode::Enter), Focus::Input), Some(Action::Submit));
        assert_eq!(map_key(ev(KeyCode::Tab), Focus::Tree), Some(Action::ToggleFocus));
    }
}
