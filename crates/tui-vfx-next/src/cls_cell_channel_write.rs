// <FILE>crates/tui-vfx-next/src/cls_cell_channel_write.rs</FILE> - <DESC>Proof-only payload for one written cell channel</DESC>
// <VERS>VERSION: 0.1.0</VERS>
// <WCTX>New kernel Phase G3: capture channel-aware branch deltas for parallel merge.</WCTX>
// <CLOG>0.1.0: INIT — add proof delta payload variants for cell and role channels.</CLOG>

use tui_vfx_types::{Color, Modifiers, RoleTag};

/// Proof-only payload for one channel write in a surface delta.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum CellChannelWrite {
    /// Glyph character payload.
    Glyph(char),
    /// Foreground color payload.
    Foreground(Color),
    /// Background color payload.
    Background(Color),
    /// Terminal modifiers payload.
    Modifiers(Modifiers),
    /// Optional modifier alpha payload.
    ModifierAlpha(Option<u8>),
    /// Semantic role payload.
    Role(RoleTag),
}

// <FILE>crates/tui-vfx-next/src/cls_cell_channel_write.rs</FILE> - <DESC>Proof-only payload for one written cell channel</DESC>
// <VERS>END OF VERSION: 0.1.0</VERS>
