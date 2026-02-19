// <FILE>tui-vfx-style/tests/common/col_make_style.rs</FILE> - <DESC>Create default Style for integration tests</DESC>
// <VERS>VERSION: 1.0.2</VERS>
// <WCTX>Clean up test warnings</WCTX>
// <CLOG>Allow unused integration test helper</CLOG>

use tui_vfx_types::{Color, Modifiers, Style};

#[allow(dead_code)]
pub(crate) fn make_style() -> Style {
    Style {
        fg: Color::rgb(100, 100, 100),
        bg: Color::rgb(50, 50, 50),
        mods: Modifiers::NONE,
    }
}

// <FILE>tui-vfx-style/tests/common/col_make_style.rs</FILE> - <DESC>Create default Style for integration tests</DESC>
// <VERS>END OF VERSION: 1.0.2</VERS>
