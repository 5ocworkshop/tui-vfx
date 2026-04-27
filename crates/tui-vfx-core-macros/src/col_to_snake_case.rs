// <FILE>tui-vfx-core-macros/src/col_to_snake_case.rs</FILE> - <DESC>Pure leaf helper: convert a Rust identifier (e.g. "VfxBindable") to snake_case ("vfx_bindable") for serde rename_all support.</DESC>
// <VERS>VERSION: 1.0.0 - 2026-04-28</VERS>
// <WCTX>Macro crate hygiene cleanup US-012 — relocate to_snake_case out of inline lib.rs.</WCTX>
// <CLOG>1.0.0: initial — body lifted from lib.rs:219-232 verbatim.</CLOG>

/// Convert Rust identifier to snake_case for JSON.
pub(crate) fn to_snake_case(s: &str) -> String {
    let mut result = String::new();
    for (i, c) in s.chars().enumerate() {
        if c.is_uppercase() {
            if i > 0 {
                result.push('_');
            }
            result.push(c.to_lowercase().next().unwrap());
        } else {
            result.push(c);
        }
    }
    result
}

// <FILE>tui-vfx-core-macros/src/col_to_snake_case.rs</FILE> - <DESC>Pure leaf helper: convert identifier to snake_case</DESC>
// <VERS>END OF VERSION: 1.0.0 - 2026-04-28</VERS>
