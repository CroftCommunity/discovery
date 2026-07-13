//! Leptos primitives. They consume only tokens (never raw values) and know no
//! protocol — every component takes plain display data as props, so the design
//! system renders whatever view-model data the shell hands it. The shell maps
//! core view models onto these props.

use crate::tokens::*;
use leptos::prelude::*;

// --- Text ---

/// Which step on the type scale a piece of text sits on.
#[derive(Clone, Copy, Default, PartialEq)]
pub enum TextVariant {
    Caption,
    #[default]
    Body,
    Callout,
    Title,
    Heading,
}

impl TextVariant {
    /// (size token, line-height token) for this step.
    fn size_line(self) -> (&'static str, &'static str) {
        match self {
            TextVariant::Caption => (TEXT_CAPTION_SIZE, TEXT_CAPTION_LINE),
            TextVariant::Body => (TEXT_BODY_SIZE, TEXT_BODY_LINE),
            TextVariant::Callout => (TEXT_CALLOUT_SIZE, TEXT_CALLOUT_LINE),
            TextVariant::Title => (TEXT_TITLE_SIZE, TEXT_TITLE_LINE),
            TextVariant::Heading => (TEXT_HEADING_SIZE, TEXT_HEADING_LINE),
        }
    }
}

/// Ink tone, by semantic role.
#[derive(Clone, Copy, Default, PartialEq)]
pub enum Tone {
    #[default]
    Primary,
    Secondary,
    Faint,
    Accent,
    Error,
}

impl Tone {
    fn color(self) -> &'static str {
        match self {
            Tone::Primary => COLOR_TEXT_PRIMARY,
            Tone::Secondary => COLOR_TEXT_SECONDARY,
            Tone::Faint => COLOR_TEXT_FAINT,
            Tone::Accent => COLOR_ACCENT,
            Tone::Error => COLOR_ERROR,
        }
    }
}

/// A run of text on the type scale. `mono` switches to the identifier face
/// (handles, timestamps — the signature treatment).
#[component]
pub fn Text(
    #[prop(into)] text: String,
    #[prop(optional)] variant: TextVariant,
    #[prop(optional)] tone: Tone,
    #[prop(optional)] mono: bool,
    #[prop(optional)] weight: Option<&'static str>,
) -> impl IntoView {
    let (size, line) = variant.size_line();
    let family = if mono { FONT_MONO } else { FONT_SANS };
    let weight = weight.unwrap_or(WEIGHT_REGULAR);
    let style = format!(
        "margin:0;font-family:{family};font-size:{size};line-height:{line};\
         color:{color};font-weight:{weight};",
        color = tone.color(),
    );
    view! { <p style=style>{text}</p> }
}

// --- Avatar ---

/// An avatar: a real image when present, otherwise a designed placeholder
/// (never a broken image). `seed` provides the placeholder's initial.
#[component]
pub fn Avatar(
    #[prop(into)] src: Option<String>,
    #[prop(into)] seed: String,
) -> impl IntoView {
    let size = SPACE_7; // 48px
    let base = format!(
        "width:{size};height:{size};border-radius:{RADIUS_FULL};flex:0 0 auto;\
         overflow:hidden;"
    );
    match src {
        Some(url) => {
            let style = format!("{base}object-fit:cover;display:block;");
            view! { <img src=url alt="" style=style /> }.into_any()
        }
        None => {
            let initial = seed
                .trim_start_matches('@')
                .chars()
                .next()
                .map(|c| c.to_uppercase().to_string())
                .unwrap_or_else(|| "·".to_string());
            let style = format!(
                "{base}display:flex;align-items:center;justify-content:center;\
                 background:{COLOR_ACCENT_WASH};color:{COLOR_ACCENT_STRONG};\
                 font-family:{FONT_SANS};font-weight:{WEIGHT_SEMIBOLD};\
                 font-size:{TEXT_CALLOUT_SIZE};"
            );
            view! { <div style=style aria-hidden="true">{initial}</div> }.into_any()
        }
    }
}

// --- Button ---

/// Button emphasis.
#[derive(Clone, Copy, Default, PartialEq)]
pub enum ButtonKind {
    #[default]
    Primary,
    Ghost,
}

/// A button wired to a click callback. Calm motion on the background.
#[component]
pub fn Button(
    #[prop(into)] label: String,
    #[prop(optional)] kind: ButtonKind,
    #[prop(into)] on_press: Callback<()>,
) -> impl IntoView {
    let (bg, fg, border) = match kind {
        ButtonKind::Primary => (COLOR_ACCENT, COLOR_SURFACE_RAISED, COLOR_ACCENT),
        ButtonKind::Ghost => ("transparent", COLOR_ACCENT, COLOR_BORDER),
    };
    let style = format!(
        "appearance:none;cursor:pointer;user-select:none;\
         font-family:{FONT_SANS};font-size:{TEXT_CALLOUT_SIZE};font-weight:{WEIGHT_MEDIUM};\
         color:{fg};background:{bg};border:{HAIRLINE} solid {border};\
         border-radius:{RADIUS_MD};padding:{SPACE_2} {SPACE_4};\
         transition:background {MOTION_FAST} {EASE_STANDARD};"
    );
    view! {
        <button style=style on:click=move |_| on_press.run(())>
            {label}
        </button>
    }
}

// --- Card ---

/// A raised surface that holds content (a post, a chrome message).
#[component]
pub fn Card(children: Children) -> impl IntoView {
    let style = format!(
        "background:{COLOR_SURFACE_RAISED};border:{HAIRLINE} solid {COLOR_BORDER};\
         border-radius:{RADIUS_LG};padding:{SPACE_4};"
    );
    view! { <div style=style>{children()}</div> }
}

// --- Post card ---

/// A single post, composed from primitives. Plain display props only — no
/// protocol type. The mono handle + timestamp is the design signature.
#[component]
pub fn PostCard(
    #[prop(into)] author_display_name: String,
    #[prop(into)] handle: String,
    #[prop(into)] body: String,
    #[prop(into)] timestamp: String,
    #[prop(into)] avatar: Option<String>,
) -> impl IntoView {
    let row = format!("display:flex;gap:{SPACE_3};align-items:flex-start;");
    let col = format!("display:flex;flex-direction:column;gap:{SPACE_2};min-width:0;flex:1 1 auto;");
    // One line: name and handle truncate; the timestamp is pinned to the end and
    // never pushed off (flex:0 0 auto, margin-left:auto).
    let meta = format!("display:flex;gap:{SPACE_2};align-items:baseline;min-width:0;");
    let name_style = format!(
        "margin:0;font-family:{FONT_SANS};font-size:{TEXT_BODY_SIZE};line-height:{TEXT_BODY_LINE};\
         color:{COLOR_TEXT_PRIMARY};font-weight:{WEIGHT_SEMIBOLD};\
         white-space:nowrap;overflow:hidden;text-overflow:ellipsis;flex:0 1 auto;min-width:0;"
    );
    let handle_style = format!(
        "margin:0;font-family:{FONT_MONO};font-size:{TEXT_CAPTION_SIZE};line-height:{TEXT_CAPTION_LINE};\
         color:{COLOR_TEXT_SECONDARY};white-space:nowrap;overflow:hidden;text-overflow:ellipsis;\
         flex:0 2 auto;min-width:0;"
    );
    let time_style = format!(
        "margin:0;font-family:{FONT_MONO};font-size:{TEXT_CAPTION_SIZE};line-height:{TEXT_CAPTION_LINE};\
         color:{COLOR_TEXT_FAINT};white-space:nowrap;flex:0 0 auto;margin-left:auto;"
    );
    // Body preserves author line breaks, wraps long words cleanly, and is
    // selectable (real content opts back in to text selection / text cursor).
    let body_style = format!(
        "margin:0;font-family:{FONT_SANS};font-size:{TEXT_BODY_SIZE};line-height:{TEXT_BODY_LINE};\
         color:{COLOR_TEXT_PRIMARY};white-space:pre-wrap;overflow-wrap:anywhere;\
         user-select:text;-webkit-user-select:text;cursor:text;"
    );
    view! {
        <Card>
            <div style=row>
                <Avatar src=avatar seed=handle.clone() />
                <div style=col>
                    <div style=meta>
                        <p style=name_style>{author_display_name}</p>
                        <p style=handle_style>{format!("@{handle}")}</p>
                        <p style=time_style>{timestamp}</p>
                    </div>
                    <p style=body_style data-selectable="">{body}</p>
                </div>
            </div>
        </Card>
    }
}

// --- view-state chrome ---

/// Cold loading: a quiet, centered message. (Calm, not a spinner storm.)
#[component]
pub fn LoadingView() -> impl IntoView {
    let style = format!("padding:{SPACE_7} {SPACE_4};text-align:center;");
    view! {
        <div style=style>
            <Text text="Loading…" tone=Tone::Faint />
        </div>
    }
}

/// Loaded but empty: an invitation, not an apology.
#[component]
pub fn EmptyView(#[prop(into)] message: String) -> impl IntoView {
    let style = format!("padding:{SPACE_7} {SPACE_4};text-align:center;");
    view! {
        <div style=style>
            <Text text=message tone=Tone::Secondary />
        </div>
    }
}

/// Cold failure: the full error view with a retry affordance. Says what went
/// wrong and how to fix it, in the interface's voice.
#[component]
pub fn ErrorView(
    #[prop(into)] reason: String,
    #[prop(into)] on_retry: Callback<()>,
) -> impl IntoView {
    let style = format!(
        "display:flex;flex-direction:column;gap:{SPACE_4};align-items:center;\
         text-align:center;padding:{SPACE_7} {SPACE_4};\
         background:{COLOR_ERROR_WASH};border-radius:{RADIUS_LG};"
    );
    view! {
        <div style=style>
            <Text text="Couldn't load the feed" tone=Tone::Error weight=WEIGHT_SEMIBOLD />
            <Text text=reason variant=TextVariant::Callout tone=Tone::Secondary />
            <Button label="Try again" on_press=on_retry />
        </div>
    }
}

/// Footer: more is available below.
#[component]
pub fn FooterMore() -> impl IntoView {
    footer_note("Scroll to load more")
}

/// Footer: the true end of the feed.
#[component]
pub fn FooterEnd() -> impl IntoView {
    footer_note("You're all caught up")
}

/// Footer: a next page is loading.
#[component]
pub fn FooterLoadingMore() -> impl IntoView {
    footer_note("Loading more…")
}

/// Footer: a load-more failed, but the posts above are intact. Inline, never
/// the full error view.
#[component]
pub fn FooterInlineError(
    #[prop(into)] reason: String,
    #[prop(into)] on_retry: Callback<()>,
) -> impl IntoView {
    let style = format!(
        "display:flex;gap:{SPACE_3};align-items:center;justify-content:center;\
         flex-wrap:wrap;padding:{SPACE_4};"
    );
    view! {
        <div style=style>
            <Text text=format!("Couldn't load more: {reason}") variant=TextVariant::Caption tone=Tone::Error />
            <Button label="Retry" kind=ButtonKind::Ghost on_press=on_retry />
        </div>
    }
}

// --- pinned strip (Phase 2) ---

/// The top-level pinned strip: a horizontal row of pinned items above the feed.
#[component]
pub fn PinnedStrip(children: Children) -> impl IntoView {
    let style = format!(
        "display:flex;gap:{SPACE_3};overflow-x:auto;padding:{SPACE_3} {SPACE_4};\
         background:{COLOR_SURFACE_SUNKEN};border-bottom:{HAIRLINE} solid {COLOR_BORDER};"
    );
    view! { <div style=style>{children()}</div> }
}

/// An empty pinned strip: a quiet invitation, not a void.
#[component]
pub fn EmptyPinnedStrip() -> impl IntoView {
    let style = format!(
        "padding:{SPACE_3} {SPACE_4};background:{COLOR_SURFACE_SUNKEN};\
         border-bottom:{HAIRLINE} solid {COLOR_BORDER};"
    );
    view! {
        <div style=style>
            <Text text="Pin a post to keep it up here." variant=TextVariant::Caption tone=Tone::Faint mono=true />
        </div>
    }
}

fn pin_card_style() -> String {
    format!(
        "flex:0 0 auto;width:{MEASURE_PIN};background:{COLOR_SURFACE_RAISED};\
         border:{HAIRLINE} solid {COLOR_BORDER};border-radius:{RADIUS_MD};\
         padding:{SPACE_3};display:flex;flex-direction:column;gap:{SPACE_2};"
    )
}

/// A compact, hydrated pin: author + a one-line snippet, with an unpin control.
#[component]
pub fn PinnedItem(
    #[prop(into)] author: String,
    #[prop(into)] snippet: String,
    #[prop(into)] on_unpin: Callback<()>,
) -> impl IntoView {
    let head = format!("display:flex;align-items:baseline;gap:{SPACE_2};");
    let snip = format!(
        "margin:0;font-family:{FONT_SANS};font-size:{TEXT_CAPTION_SIZE};line-height:{TEXT_CAPTION_LINE};\
         color:{COLOR_TEXT_SECONDARY};display:-webkit-box;-webkit-line-clamp:2;-webkit-box-orient:vertical;\
         overflow:hidden;"
    );
    view! {
        <div style=pin_card_style()>
            <div style=head>
                <Text text=author variant=TextVariant::Caption weight=WEIGHT_SEMIBOLD />
                <span style=format!("margin-left:auto;")>
                    <Button label="✕" kind=ButtonKind::Ghost on_press=on_unpin />
                </span>
            </div>
            <p style=snip>{snippet}</p>
        </div>
    }
}

/// A pin whose target is gone: degrades gracefully rather than vanishing.
#[component]
pub fn DegradedPinItem(#[prop(into)] on_unpin: Callback<()>) -> impl IntoView {
    let head = format!("display:flex;align-items:baseline;gap:{SPACE_2};");
    let style = format!(
        "{};border-style:dashed;",
        pin_card_style()
    );
    view! {
        <div style=style>
            <div style=head>
                <Text text="Unavailable" variant=TextVariant::Caption tone=Tone::Faint weight=WEIGHT_MEDIUM />
                <span style=format!("margin-left:auto;")>
                    <Button label="✕" kind=ButtonKind::Ghost on_press=on_unpin />
                </span>
            </div>
            <Text text="This item is no longer available." variant=TextVariant::Caption tone=Tone::Faint />
        </div>
    }
}

/// A small pin affordance shown on a feed card.
#[component]
pub fn PinButton(#[prop(into)] on_press: Callback<()>) -> impl IntoView {
    let style = format!(
        "appearance:none;cursor:pointer;user-select:none;background:transparent;\
         border:{HAIRLINE} solid {COLOR_BORDER};border-radius:{RADIUS_FULL};\
         color:{COLOR_TEXT_SECONDARY};font-family:{FONT_SANS};font-size:{TEXT_CAPTION_SIZE};\
         padding:{SPACE_1} {SPACE_3};transition:background {MOTION_FAST} {EASE_STANDARD};"
    );
    view! {
        <button style=style on:click=move |_| on_press.run(())>
            "Pin"
        </button>
    }
}

fn footer_note(text: &'static str) -> impl IntoView {
    let style = format!("padding:{SPACE_5} {SPACE_4};text-align:center;");
    view! {
        <div style=style>
            <Text text=text variant=TextVariant::Caption tone=Tone::Faint mono=true />
        </div>
    }
}
