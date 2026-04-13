// <FILE>crates/tui-vfx-probe/src/fnc_normalize_color.rs</FILE> - <DESC>Normalize tui-vfx colors into probe output colors</DESC>
// <VERS>VERSION: 0.2.0</VERS>
// <WCTX>Phase-1 pipeline probe implementation</WCTX>
// <CLOG>MINOR: Emit canonical structured rgba color objects for probe output</CLOG>

use tui_vfx_types::Color;

use crate::cls_probe_color::ProbeColor;

pub fn normalize_color(color: Color) -> ProbeColor {
    ProbeColor {
        space: "rgb".to_string(),
        r: color.r,
        g: color.g,
        b: color.b,
        a: color.a,
    }
}

// <FILE>crates/tui-vfx-probe/src/fnc_normalize_color.rs</FILE> - <DESC>Normalize tui-vfx colors into probe output colors</DESC>
// <VERS>END OF VERSION: 0.2.0</VERS>
