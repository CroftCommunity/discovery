//! Render a `FeedView` to the terminal as text. Renders every view state — this
//! is the text-mode equivalent of the Phase 1 per-state snapshot rule
//! (philosophy §8): an undesigned state is a visible gap here.

use app_core::{Avatar, FeedView, Footer, PostCard};

/// Render a view to a plain string. Pure (takes a view, returns text), so it is
/// easy to assert against; `main` prints the result.
pub fn render(view: &FeedView) -> String {
    match view {
        FeedView::Loading => "⟳ Loading…".to_string(),

        FeedView::Empty { message } => format!("(empty) {message}"),

        FeedView::Feed { posts, footer } => {
            let mut out = String::new();
            for (i, card) in posts.iter().enumerate() {
                if i > 0 {
                    out.push('\n');
                }
                out.push_str(&render_card(card));
            }
            if !posts.is_empty() {
                out.push('\n');
            }
            out.push_str(&render_footer(footer));
            out
        }

        FeedView::Error { reason, .. } => {
            format!("✗ Couldn't load the feed: {reason}\n  [r] retry")
        }
    }
}

fn render_card(card: &PostCard) -> String {
    let avatar = match &card.avatar {
        Avatar::Url(url) => url.as_str(),
        Avatar::NoAvatar => "(no avatar)",
    };
    format!(
        "┌─ {name} @{handle} · {ts}\n│  {body}\n└─ avatar: {avatar}",
        name = card.author_display_name,
        handle = card.handle,
        ts = card.timestamp,
        body = card.body,
    )
}

fn render_footer(footer: &Footer) -> String {
    match footer {
        Footer::MoreAvailable => "— more below, scroll to load —".to_string(),
        Footer::LoadingMore => "— loading more… —".to_string(),
        Footer::EndReached => "— end of feed —".to_string(),
        Footer::InlineError { reason, .. } => {
            format!("— couldn't load more: {reason}  [r] retry —")
        }
    }
}
