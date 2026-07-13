//! ratatui rendering: a two-pane layout (left graph tree, right chat).
//!
//! Pure rendering from a [`ChatView`] — no session/model access — so it renders
//! identically under a real terminal and a `TestBackend`. P10 fills the left
//! pane; P11 adds the right pane timeline + input.

use group_chat_core::ChatView;
use ratatui::layout::{Constraint, Layout};
use ratatui::style::{Modifier, Style};
use ratatui::widgets::{Block, List, ListItem, Paragraph};
use ratatui::Frame;

use crate::app::Focus;

/// Draw the whole UI for `view` with `focus`.
pub fn draw(frame: &mut Frame, view: &ChatView, focus: Focus) {
    // When the selected group hits a §7.6.1 hard-stop, a blocking banner takes the
    // top row and the two panes render below it — the escalation made visible. The
    // set has two shapes: a fork (too many valid claims) and under-determination
    // (a required role vacant), and the banner names which.
    let area = frame.area();
    let body = if let Some(status) = &view.fork {
        let rows = Layout::vertical([Constraint::Length(1), Constraint::Min(1)]).split(area);
        let headline = if status == "under_determined" {
            "⚠ UNDER-DETERMINED — required role vacant, convergence halted"
        } else if status.starts_with("contradiction") {
            "⚠ CONTRADICTION — concurrent conflicting claims, convergence halted"
        } else {
            "⚠ FORK DETECTED — convergence halted, no silent winner"
        };
        let banner = Paragraph::new(format!("{headline} ({status})"))
            .style(Style::default().add_modifier(Modifier::REVERSED | Modifier::BOLD));
        frame.render_widget(banner, rows[0]);
        rows[1]
    } else {
        area
    };

    let columns =
        Layout::horizontal([Constraint::Percentage(30), Constraint::Percentage(70)]).split(body);
    draw_tree(frame, columns[0], view, focus);
    draw_chat(frame, columns[1], view, focus);
}

fn draw_tree(frame: &mut Frame, area: ratatui::layout::Rect, view: &ChatView, focus: Focus) {
    use group_chat_core::TreeRow;
    let items: Vec<ListItem> = view
        .tree
        .rows
        .iter()
        .map(|row| match row {
            TreeRow::Group(g) => {
                let marker = if g.selected { "▸ " } else { "  " };
                ListItem::new(format!("{marker}{} ({})", g.title, g.member_count))
            }
            TreeRow::Channel(c) => {
                let marker = if c.selected { "▸ " } else { "  " };
                ListItem::new(format!("    {marker}#{}", c.name))
            }
        })
        .collect();
    let title = if focus == Focus::Tree { "Groups*" } else { "Groups" };
    let list = List::new(items)
        .block(Block::bordered().title(title))
        .highlight_style(Style::default().add_modifier(Modifier::REVERSED));
    frame.render_widget(list, area);
}

fn draw_chat(frame: &mut Frame, area: ratatui::layout::Rect, view: &ChatView, focus: Focus) {
    // P11 expands this into a timeline + input split; P10 renders the timeline
    // lines and a draft line so the right pane is real, not a stub.
    let rows = Layout::vertical([Constraint::Min(1), Constraint::Length(3)]).split(area);

    let lines: Vec<ListItem> = view
        .timeline
        .lines
        .iter()
        .map(|l| {
            let pending = if l.pending { " (sending…)" } else { "" };
            ListItem::new(format!("{}: {}{pending}", l.author, l.body))
        })
        .collect();
    let timeline = List::new(lines).block(Block::bordered().title("Timeline"));
    frame.render_widget(timeline, rows[0]);

    let input_title = if focus == Focus::Input { "Message*" } else { "Message" };
    let input = Paragraph::new(view.draft.as_str()).block(Block::bordered().title(input_title));
    frame.render_widget(input, rows[1]);
}

/// Flatten a buffer to a string (used by render assertions in tests).
pub fn buffer_to_string(buffer: &ratatui::buffer::Buffer) -> String {
    buffer
        .content()
        .iter()
        .map(ratatui::buffer::Cell::symbol)
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use group_chat_core::{ChannelNode, GraphTreeView, GroupNode, TimelineView, TreeRow};
    use ratatui::backend::TestBackend;
    use ratatui::Terminal;
    use social_graph_core::{GroupId, Hash, KindTag, TypedId};

    fn render(view: &ChatView, focus: Focus) -> String {
        let backend = TestBackend::new(70, 12);
        let mut terminal = Terminal::new(backend).expect("terminal");
        terminal
            .draw(|frame| draw(frame, view, focus))
            .expect("draw");
        buffer_to_string(terminal.backend().buffer())
    }

    #[test]
    fn left_pane_lists_groups_with_member_counts() {
        let view = ChatView {
            tree: GraphTreeView {
                rows: vec![TreeRow::Group(GroupNode {
                    id: GroupId::new([0xab; 32]),
                    title: "abcdef01".to_string(),
                    member_count: 3,
                    selected: true,
                })],
            },
            timeline: TimelineView::default(),
            draft: String::new(),
            fork: None,
        };
        let text = render(&view, Focus::Tree);
        assert!(text.contains("abcdef01"), "group label shown: {text}");
        assert!(text.contains("(3)"), "member count shown");
        assert!(text.contains('▸'), "selected marker shown");
    }

    #[test]
    fn left_pane_nests_channel_rows_under_the_group() {
        let view = ChatView {
            tree: GraphTreeView {
                rows: vec![
                    TreeRow::Group(GroupNode {
                        id: GroupId::new([0xab; 32]),
                        title: "abcdef01".to_string(),
                        member_count: 2,
                        selected: true,
                    }),
                    TreeRow::Channel(ChannelNode {
                        id: TypedId::new(KindTag::ArtifactChat, Hash::new([1; 32])),
                        name: "photos".to_string(),
                        selected: true,
                    }),
                ],
            },
            timeline: TimelineView::default(),
            draft: String::new(),
            fork: None,
        };
        let text = render(&view, Focus::Tree);
        assert!(text.contains("#photos"), "channel row shown: {text}");
    }

    #[test]
    fn fork_renders_a_blocking_banner() {
        let view = ChatView {
            tree: GraphTreeView::default(),
            timeline: TimelineView::default(),
            draft: String::new(),
            fork: Some("forked_from:abcd".to_string()),
        };
        let text = render(&view, Focus::Tree);
        assert!(text.contains("FORK DETECTED"), "banner shown: {text}");
        assert!(text.contains("convergence halted"), "hard-stop wording shown");
    }
}
