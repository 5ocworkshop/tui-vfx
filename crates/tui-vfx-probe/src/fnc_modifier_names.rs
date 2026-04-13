// <FILE>crates/tui-vfx-probe/src/fnc_modifier_names.rs</FILE> - <DESC>Convert Modifiers into probe output names</DESC>
// <VERS>VERSION: 0.1.0</VERS>
// <WCTX>Phase-1.5 probe causation and diff support</WCTX>
// <CLOG>NEW: Extract modifier-name formatting so frame dumps, traces, and diffs share one canonical representation</CLOG>

use tui_vfx_types::Modifiers;

pub fn modifier_names(modifiers: Modifiers) -> Vec<String> {
    let mut names = Vec::new();
    if modifiers.bold {
        names.push("bold".to_string());
    }
    if modifiers.italic {
        names.push("italic".to_string());
    }
    if modifiers.underline {
        names.push("underline".to_string());
    }
    if modifiers.dim {
        names.push("dim".to_string());
    }
    if modifiers.reverse {
        names.push("reverse".to_string());
    }
    if modifiers.strikethrough {
        names.push("strikethrough".to_string());
    }
    if modifiers.slow_blink {
        names.push("slow_blink".to_string());
    }
    if modifiers.rapid_blink {
        names.push("rapid_blink".to_string());
    }
    if modifiers.hidden {
        names.push("hidden".to_string());
    }
    names
}

// <FILE>crates/tui-vfx-probe/src/fnc_modifier_names.rs</FILE> - <DESC>Convert Modifiers into probe output names</DESC>
// <VERS>END OF VERSION: 0.1.0</VERS>
