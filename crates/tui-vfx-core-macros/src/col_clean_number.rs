// <FILE>tui-vfx-core-macros/src/col_clean_number.rs</FILE> - <DESC>Pure leaf helper: strip underscore digit separators from a numeric literal's base-10 digits string.</DESC>
// <VERS>VERSION: 0.2.0 - 2026-04-28</VERS>
// <WCTX>Macro crate hygiene cleanup US-012 — promote from abandoned-refactor stub to live status; body matches lib.rs:130-132 verbatim (already-correct stub kept; this is metadata only).</WCTX>
// <CLOG>0.2.0: MAJOR — promote from abandoned-refactor stub to live module. Body unchanged (already matched the live lib.rs version). Will be reachable from lib.rs via `mod col_clean_number;` in US-013.</CLOG>

pub(crate) fn clean_number(digits: &str) -> String {
    digits.replace('_', "")
}

// <FILE>tui-vfx-core-macros/src/col_clean_number.rs</FILE> - <DESC>Pure leaf helper: strip underscore digit separators</DESC>
// <VERS>END OF VERSION: 0.2.0 - 2026-04-28</VERS>

