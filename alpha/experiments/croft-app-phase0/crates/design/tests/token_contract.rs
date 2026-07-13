//! Token-contract check: fail if a primitive resolves to a raw value (a
//! hardcoded hex color or a number with a CSS unit) instead of a token. This
//! mechanically enforces "nothing placed by eye" — every space, size, color,
//! radius, and duration must come through a token constant.

/// Scan the primitives source for raw values, ignoring line comments.
#[test]
fn primitives_contain_no_raw_values() {
    let src = include_str!("../src/primitives.rs");

    for (n, raw) in src.lines().enumerate() {
        // Drop any line comment so prose in `//` doesn't trip the scan.
        let line = raw.split("//").next().unwrap_or(raw);
        let bytes = line.as_bytes();

        // Raw hex color: `#` immediately followed by a hex digit. (Rust
        // attributes are `#[`, which this never matches.)
        for (i, &b) in bytes.iter().enumerate() {
            if b == b'#' && bytes.get(i + 1).is_some_and(u8::is_ascii_hexdigit) {
                panic!("raw hex color in primitives.rs line {}: {}", n + 1, line.trim());
            }
        }

        // Raw dimension/duration: a digit immediately followed by a CSS unit.
        // The digit guard avoids matching unit letters inside words like "items".
        for unit in ["px", "rem", "em", "ms", "pt", "vh", "vw"] {
            let mut from = 0;
            while let Some(rel) = line[from..].find(unit) {
                let at = from + rel;
                let prev = line[..at].chars().next_back();
                if prev.is_some_and(|c| c.is_ascii_digit()) {
                    panic!(
                        "raw {unit} value in primitives.rs line {}: {}",
                        n + 1,
                        line.trim()
                    );
                }
                from = at + unit.len();
            }
        }
    }
}
