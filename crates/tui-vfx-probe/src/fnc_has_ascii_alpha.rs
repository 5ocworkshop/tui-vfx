// <FILE>crates/tui-vfx-probe/src/fnc_has_ascii_alpha.rs</FILE> - <DESC>Detect ASCII alphabetic characters in a probe text row</DESC>
// <VERS>VERSION: 0.1.0</VERS>
// <WCTX>Initial probe-side diagnostics for border/text integrity issues</WCTX>
// <CLOG>NEW: Add a reusable helper for detecting semantic text leakage into rows that should behave like decorative chrome</CLOG>

pub fn has_ascii_alpha(text: &str) -> bool {
    text.chars().any(|character| character.is_ascii_uppercase())
}

// <FILE>crates/tui-vfx-probe/src/fnc_has_ascii_alpha.rs</FILE> - <DESC>Detect ASCII alphabetic characters in a probe text row</DESC>
// <VERS>END OF VERSION: 0.1.0</VERS>
