// <FILE>tui-vfx-core-macros/src/col_clean_number.rs</FILE> - <DESC>Leaf helper for numeric literal cleanup</DESC>
// <VERS>VERSION: 0.1.1 - 2025-12-17T00:00:00Z</VERS>
// <WCTX>Leaf-node helper</WCTX>
// <CLOG>Extracted underscore stripping for numeric literals</CLOG>

pub(crate) fn clean_number(digits: &str) -> String {
    digits.replace('_', "")
}

// <FILE>tui-vfx-core-macros/src/col_clean_number.rs</FILE> - <DESC>Leaf helper for numeric literal cleanup</DESC>
// <VERS>END OF VERSION: 0.1.1 - 2025-12-17T00:00:00Z</VERS>

