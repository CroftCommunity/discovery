//! Design tokens: the single source of every spacing, type, color, radius, and
//! motion decision. Named constants only — nothing in the UI ever writes a raw
//! value (no hardcoded pixel, hex, or duration). Everything reaches for a token.
//!
//! # How this is wired
//!
//! Each token constant resolves to a CSS custom-property *reference*
//! (`var(--…)`). The actual values live in exactly one place: [`root_css`],
//! which emits the `:root { … }` block the app injects once. So components only
//! ever name tokens; the raw values exist once. The M1.6 token-contract check
//! enforces that primitives contain no raw `#`/`px`/`ms` literals.
//!
//! # Rationale (tying choices to the project's values, not to defaults)
//!
//! The brief is a calm, honest, non-extractive reader across several social
//! "ponds". The frontend-design skill warns that "warm cream + high-contrast
//! serif + terracotta" is the AI-default warm look — exactly where a "warm"
//! social app drifts. So warmth is spent differently:
//!
//! - **Surface** is a warm *linen*, slightly green-grey rather than cream; cards
//!   lift onto a lighter warm white. Warmth without the cliché.
//! - **Text** is a warm near-black with a green-brown undertone, never pure
//!   black — softer contrast reads calmer, which serves "low mental load".
//! - **Accent** is a single botanical eucalyptus green (the "pond"), not
//!   terracotta. One accent, used with restraint.
//! - **Clay** is reserved *only* for error/retry, so alarm has a dedicated,
//!   rarely-seen color and the everyday surface stays serene. ("Not extractive"
//!   means nothing screams for attention without cause.)
//! - **Type** is one warm humanist grotesk for UI and body at a generous
//!   reading line-height (warmth = air, not ornament), plus a quiet monospace
//!   used only for `@handles` and timestamps. That mono treatment is the
//!   signature: identifiers are *data*, and showing them as data is quietly
//!   honest and unlike clients that flatten everything into one sans.
//! - **Motion** is calm and decelerating, and yields entirely to
//!   `prefers-reduced-motion`. Engagement-bait motion is a non-goal.

/// Emit the single `:root` block holding every raw value. Injected once by the
/// shell; every token constant below points back into it via `var(--…)`.
pub fn root_css() -> &'static str {
    "
:root {
  /* color — 'pond at morning' */
  --color-surface: #F2F1E9;          /* warm linen app background */
  --color-surface-raised: #FBFAF5;   /* cards lift onto a lighter warm white */
  --color-surface-sunken: #EAE8DE;   /* wells, insets */
  --color-text-primary: #23271F;     /* warm near-black, green-brown undertone */
  --color-text-secondary: #5C6356;   /* sage-grey: handles, timestamps, captions */
  --color-text-faint: #8B9183;       /* the quietest readable text */
  --color-accent: #3E6B5E;           /* eucalyptus — the pond */
  --color-accent-strong: #2C5044;    /* pressed/active accent */
  --color-accent-wash: #DEE8E2;      /* hover/selected backgrounds */
  --color-border: #E0DED3;           /* hairlines */
  --color-error: #9C5B3F;            /* muted clay — error/retry ONLY */
  --color-error-wash: #F1E3DB;       /* error surfaces */
  --color-focus: #3E6B5E;            /* deliberate app focus ring color */

  /* spacing — strict 4px scale */
  --space-0: 0;
  --space-1: 0.25rem;  /* 4 */
  --space-2: 0.5rem;   /* 8 */
  --space-3: 0.75rem;  /* 12 */
  --space-4: 1rem;     /* 16 */
  --space-5: 1.5rem;   /* 24 */
  --space-6: 2rem;     /* 32 */
  --space-7: 3rem;     /* 48 */
  --space-8: 4rem;     /* 64 */

  /* type — families */
  --font-sans: 'Hanken Grotesk', ui-sans-serif, system-ui, -apple-system, 'Segoe UI', Roboto, sans-serif;
  --font-mono: 'IBM Plex Mono', ui-monospace, 'SF Mono', Menlo, Consolas, monospace;

  /* type — sizes, each with its line-height bound alongside */
  --text-caption-size: 0.8125rem;   /* 13 */
  --text-caption-line: 1.125rem;    /* 18 */
  --text-body-size: 1rem;           /* 16 */
  --text-body-line: 1.625rem;       /* 26 — generous, warm reading */
  --text-callout-size: 0.9375rem;   /* 15 */
  --text-callout-line: 1.375rem;    /* 22 */
  --text-title-size: 1.25rem;       /* 20 */
  --text-title-line: 1.75rem;       /* 28 */
  --text-heading-size: 1.625rem;    /* 26 */
  --text-heading-line: 2rem;        /* 32 */

  /* type — weights (no black/heavy; calm) */
  --weight-regular: 400;
  --weight-medium: 500;
  --weight-semibold: 600;

  /* radius — soft, not bubbly */
  --radius-sm: 0.375rem;  /* 6 */
  --radius-md: 0.625rem;  /* 10 */
  --radius-lg: 1rem;      /* 16 */
  --radius-full: 9999px;

  /* motion — calm, decelerating */
  --motion-instant: 0ms;
  --motion-fast: 120ms;
  --motion-base: 200ms;
  --motion-slow: 320ms;
  --ease-standard: cubic-bezier(0.2, 0, 0, 1);  /* decelerate */
  --ease-gentle: cubic-bezier(0.4, 0, 0.2, 1);

  /* layout */
  --measure-column: 38rem;  /* single-column reading width */
  --measure-pin: 15rem;     /* a pinned item's width in the strip */
  --hairline: 1px;
}

@media (prefers-reduced-motion: reduce) {
  :root {
    --motion-fast: 0ms;
    --motion-base: 0ms;
    --motion-slow: 0ms;
  }
}
"
}

// --- token accessors: the only names the UI uses ---

// color
pub const COLOR_SURFACE: &str = "var(--color-surface)";
pub const COLOR_SURFACE_RAISED: &str = "var(--color-surface-raised)";
pub const COLOR_SURFACE_SUNKEN: &str = "var(--color-surface-sunken)";
pub const COLOR_TEXT_PRIMARY: &str = "var(--color-text-primary)";
pub const COLOR_TEXT_SECONDARY: &str = "var(--color-text-secondary)";
pub const COLOR_TEXT_FAINT: &str = "var(--color-text-faint)";
pub const COLOR_ACCENT: &str = "var(--color-accent)";
pub const COLOR_ACCENT_STRONG: &str = "var(--color-accent-strong)";
pub const COLOR_ACCENT_WASH: &str = "var(--color-accent-wash)";
pub const COLOR_BORDER: &str = "var(--color-border)";
pub const COLOR_ERROR: &str = "var(--color-error)";
pub const COLOR_ERROR_WASH: &str = "var(--color-error-wash)";
pub const COLOR_FOCUS: &str = "var(--color-focus)";

// spacing
pub const SPACE_0: &str = "var(--space-0)";
pub const SPACE_1: &str = "var(--space-1)";
pub const SPACE_2: &str = "var(--space-2)";
pub const SPACE_3: &str = "var(--space-3)";
pub const SPACE_4: &str = "var(--space-4)";
pub const SPACE_5: &str = "var(--space-5)";
pub const SPACE_6: &str = "var(--space-6)";
pub const SPACE_7: &str = "var(--space-7)";
pub const SPACE_8: &str = "var(--space-8)";

// type families
pub const FONT_SANS: &str = "var(--font-sans)";
pub const FONT_MONO: &str = "var(--font-mono)";

// type sizes / line-heights
pub const TEXT_CAPTION_SIZE: &str = "var(--text-caption-size)";
pub const TEXT_CAPTION_LINE: &str = "var(--text-caption-line)";
pub const TEXT_BODY_SIZE: &str = "var(--text-body-size)";
pub const TEXT_BODY_LINE: &str = "var(--text-body-line)";
pub const TEXT_CALLOUT_SIZE: &str = "var(--text-callout-size)";
pub const TEXT_CALLOUT_LINE: &str = "var(--text-callout-line)";
pub const TEXT_TITLE_SIZE: &str = "var(--text-title-size)";
pub const TEXT_TITLE_LINE: &str = "var(--text-title-line)";
pub const TEXT_HEADING_SIZE: &str = "var(--text-heading-size)";
pub const TEXT_HEADING_LINE: &str = "var(--text-heading-line)";

// weights
pub const WEIGHT_REGULAR: &str = "var(--weight-regular)";
pub const WEIGHT_MEDIUM: &str = "var(--weight-medium)";
pub const WEIGHT_SEMIBOLD: &str = "var(--weight-semibold)";

// radius
pub const RADIUS_SM: &str = "var(--radius-sm)";
pub const RADIUS_MD: &str = "var(--radius-md)";
pub const RADIUS_LG: &str = "var(--radius-lg)";
pub const RADIUS_FULL: &str = "var(--radius-full)";

// motion
pub const MOTION_INSTANT: &str = "var(--motion-instant)";
pub const MOTION_FAST: &str = "var(--motion-fast)";
pub const MOTION_BASE: &str = "var(--motion-base)";
pub const MOTION_SLOW: &str = "var(--motion-slow)";
pub const EASE_STANDARD: &str = "var(--ease-standard)";
pub const EASE_GENTLE: &str = "var(--ease-gentle)";

// layout
pub const MEASURE_COLUMN: &str = "var(--measure-column)";
pub const MEASURE_PIN: &str = "var(--measure-pin)";
pub const HAIRLINE: &str = "var(--hairline)";

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn every_token_constant_resolves_to_a_var_reference() {
        // Tokens must be indirections, never raw values.
        for t in [
            COLOR_SURFACE, COLOR_ACCENT, COLOR_ERROR, SPACE_4, FONT_SANS,
            TEXT_BODY_SIZE, TEXT_BODY_LINE, RADIUS_MD, MOTION_BASE, EASE_STANDARD,
            MEASURE_COLUMN,
        ] {
            assert!(t.starts_with("var(--"), "token {t} is not a var() reference");
            assert!(t.ends_with(')'));
        }
    }

    #[test]
    fn root_css_defines_every_referenced_variable() {
        // Each token's underlying `--name` must be defined in root_css(), so no
        // token can dangle. Extract `--name` from `var(--name)` and check.
        let css = root_css();
        let tokens = [
            COLOR_SURFACE, COLOR_SURFACE_RAISED, COLOR_SURFACE_SUNKEN,
            COLOR_TEXT_PRIMARY, COLOR_TEXT_SECONDARY, COLOR_TEXT_FAINT,
            COLOR_ACCENT, COLOR_ACCENT_STRONG, COLOR_ACCENT_WASH, COLOR_BORDER,
            COLOR_ERROR, COLOR_ERROR_WASH, COLOR_FOCUS,
            SPACE_0, SPACE_1, SPACE_2, SPACE_3, SPACE_4, SPACE_5, SPACE_6, SPACE_7, SPACE_8,
            FONT_SANS, FONT_MONO,
            TEXT_CAPTION_SIZE, TEXT_CAPTION_LINE, TEXT_BODY_SIZE, TEXT_BODY_LINE,
            TEXT_CALLOUT_SIZE, TEXT_CALLOUT_LINE, TEXT_TITLE_SIZE, TEXT_TITLE_LINE,
            TEXT_HEADING_SIZE, TEXT_HEADING_LINE,
            WEIGHT_REGULAR, WEIGHT_MEDIUM, WEIGHT_SEMIBOLD,
            RADIUS_SM, RADIUS_MD, RADIUS_LG, RADIUS_FULL,
            MOTION_INSTANT, MOTION_FAST, MOTION_BASE, MOTION_SLOW,
            EASE_STANDARD, EASE_GENTLE, MEASURE_COLUMN, MEASURE_PIN, HAIRLINE,
        ];
        for t in tokens {
            let name = t.trim_start_matches("var(").trim_end_matches(')');
            assert!(
                css.contains(&format!("{name}:")),
                "token {t} -> {name} is not defined in root_css()"
            );
        }
    }
}
