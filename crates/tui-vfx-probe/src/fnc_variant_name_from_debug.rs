// <FILE>crates/tui-vfx-probe/src/fnc_variant_name_from_debug.rs</FILE> - <DESC>Extract a stable variant-style name from Debug output</DESC>
// <VERS>VERSION: 0.1.0</VERS>
// <WCTX>Direct probe operational analysis and pipeline inventory naming</WCTX>
// <CLOG>NEW: Add a tiny helper that trims Debug output down to a stable leading variant name so configured pipeline elements can be compared against observed trace effect names</CLOG>

use std::fmt::Debug;

pub fn variant_name_from_debug<T: Debug>(value: &T) -> String {
    let debug = format!("{value:?}");
    debug
        .split(['{', '(', ' '])
        .next()
        .unwrap_or_default()
        .to_string()
}

// <FILE>crates/tui-vfx-probe/src/fnc_variant_name_from_debug.rs</FILE> - <DESC>Extract a stable variant-style name from Debug output</DESC>
// <VERS>END OF VERSION: 0.1.0</VERS>
