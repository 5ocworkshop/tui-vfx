// <FILE>crates/tui-vfx-compost/src/primitive/cls_cell_view.rs</FILE> - <DESC>Runtime-checked primitive cell access view</DESC>
// <VERS>VERSION: 0.1.0</VERS>
// <WCTX>Phase 0 chooses runtime debug assertions for cellAccess enforcement so trait shape can stabilize before heavier type-state enforcement.</WCTX>
// <CLOG>0.1.0: INIT — add CellView read/write guards over tui-vfx-types Cell channels.</CLOG>

use std::marker::PhantomData;

use tui_vfx_contract::{CellAccess, CellChannel};
use tui_vfx_types::{Cell, Color, Modifiers};

use super::EffectPrimitive;

/// Mutable cell facade that checks descriptor-declared channel access in debug builds.
pub struct CellView<'a, P: EffectPrimitive> {
    cell: &'a mut Cell,
    primitive_id: String,
    access: CellAccess,
    _primitive: PhantomData<P>,
}

impl<'a, P: EffectPrimitive> CellView<'a, P> {
    /// Wrap a cell for a primitive runtime implementation.
    pub fn new(cell: &'a mut Cell) -> Self {
        let descriptor = P::descriptor();
        Self {
            cell,
            primitive_id: descriptor.id.as_str().to_string(),
            access: descriptor.cell_access,
            _primitive: PhantomData,
        }
    }

    /// Read the glyph channel.
    pub fn glyph(&self) -> char {
        self.assert_read(CellChannel::Glyph);
        self.cell.ch
    }

    /// Write the glyph channel.
    pub fn set_glyph(&mut self, glyph: char) {
        self.assert_write(CellChannel::Glyph);
        self.cell.ch = glyph;
    }

    /// Read the foreground channel.
    pub fn foreground(&self) -> Color {
        self.assert_read(CellChannel::Foreground);
        self.cell.fg
    }

    /// Write the foreground channel.
    pub fn set_foreground(&mut self, foreground: Color) {
        self.assert_write(CellChannel::Foreground);
        self.cell.fg = foreground;
    }

    /// Read the background channel.
    pub fn background(&self) -> Color {
        self.assert_read(CellChannel::Background);
        self.cell.bg
    }

    /// Write the background channel.
    pub fn set_background(&mut self, background: Color) {
        self.assert_write(CellChannel::Background);
        self.cell.bg = background;
    }

    /// Read the modifier channel.
    pub fn modifiers(&self) -> Modifiers {
        self.assert_read(CellChannel::Modifiers);
        self.cell.mods
    }

    /// Write the modifier channel.
    pub fn set_modifiers(&mut self, modifiers: Modifiers) {
        self.assert_write(CellChannel::Modifiers);
        self.cell.mods = modifiers;
    }

    /// Read the modifier-alpha channel.
    pub fn modifier_alpha(&self) -> Option<u8> {
        self.assert_read(CellChannel::ModifierAlpha);
        self.cell.mod_alpha
    }

    /// Write the modifier-alpha channel.
    pub fn set_modifier_alpha(&mut self, modifier_alpha: Option<u8>) {
        self.assert_write(CellChannel::ModifierAlpha);
        self.cell.mod_alpha = modifier_alpha;
    }

    /// Borrow the full cell for tests and adapter code that already performed checks.
    pub fn cell(&self) -> &Cell {
        self.cell
    }

    fn assert_read(&self, channel: CellChannel) {
        debug_assert!(
            self.access.can_read(channel),
            "primitive `{}` does not declare read access to {:?}",
            self.primitive_id,
            channel
        );
    }

    fn assert_write(&self, channel: CellChannel) {
        debug_assert!(
            self.access.can_write(channel),
            "primitive `{}` does not declare write access to {:?}",
            self.primitive_id,
            channel
        );
    }
}

// <FILE>crates/tui-vfx-compost/src/primitive/cls_cell_view.rs</FILE> - <DESC>Runtime-checked primitive cell access view</DESC>
// <VERS>END OF VERSION: 0.1.0</VERS>
