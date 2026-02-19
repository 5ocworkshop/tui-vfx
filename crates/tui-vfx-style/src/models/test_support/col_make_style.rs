// <FILE>tui-vfx-style/src/models/test_support/col_make_style.rs</FILE> - <DESC>Create default Style for shader tests</DESC>
// <VERS>VERSION: 1.0.0</VERS>
// <WCTX>Consolidate style test helpers</WCTX>
// <CLOG>Add shared Style helper</CLOG>

use tui_vfx_types::{Color, Modifiers, Style};

pub(crate) fn make_style() -> Style {
    Style {
        fg: Color::rgb(100, 100, 100),
        bg: Color::rgb(50, 50, 50),
        mods: Modifiers::NONE,
    }
}

// <FILE>tui-vfx-style/src/models/test_support/col_make_style.rs</FILE> - <DESC>Create default Style for shader tests</DESC>
// <VERS>END OF VERSION: 1.0.0</VERS>
