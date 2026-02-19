// <FILE>crates/tui-vfx-types/src/cell.rs</FILE> - <DESC>Cell type: character with styling</DESC>
// <VERS>VERSION: 0.3.0</VERS>
// <WCTX>Remove modifier-alpha feature flag</WCTX>
// <CLOG>Make mod_alpha field always present - no feature flag needed</CLOG>

//! Cell type representing a single styled character.

use crate::{Color, Modifiers};

/// A single cell in a terminal display.
///
/// Each cell contains a character and its styling information (colors and modifiers).
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Cell {
    /// The character to display.
    pub ch: char,
    /// Foreground (text) color.
    pub fg: Color,
    /// Background color.
    pub bg: Color,
    /// Text modifiers (bold, italic, etc.).
    pub mods: Modifiers,
    /// Optional alpha override for modifier application (0-255).
    #[cfg_attr(
        feature = "serde",
        serde(default, skip_serializing_if = "Option::is_none")
    )]
    pub mod_alpha: Option<u8>,
}

impl Default for Cell {
    fn default() -> Self {
        Self {
            ch: ' ',
            fg: Color::TRANSPARENT,
            bg: Color::TRANSPARENT,
            mods: Modifiers::NONE,
            mod_alpha: None,
        }
    }
}

impl Cell {
    /// Create a new cell with the given character and default styling.
    pub const fn new(ch: char) -> Self {
        Self {
            ch,
            fg: Color::TRANSPARENT,
            bg: Color::TRANSPARENT,
            mods: Modifiers::NONE,
            mod_alpha: None,
        }
    }

    /// Create a cell with full styling.
    pub const fn styled(ch: char, fg: Color, bg: Color, mods: Modifiers) -> Self {
        Self {
            ch,
            fg,
            bg,
            mods,
            mod_alpha: None,
        }
    }

    /// Return a new cell with a different character.
    pub const fn with_char(self, ch: char) -> Self {
        Self { ch, ..self }
    }

    /// Return a new cell with a different foreground color.
    pub const fn with_fg(self, fg: Color) -> Self {
        Self { fg, ..self }
    }

    /// Return a new cell with a different background color.
    pub const fn with_bg(self, bg: Color) -> Self {
        Self { bg, ..self }
    }

    /// Return a new cell with different modifiers.
    pub const fn with_mods(self, mods: Modifiers) -> Self {
        Self { mods, ..self }
    }

    /// Return a new cell with a modifier alpha override.
    pub const fn with_mod_alpha(self, mod_alpha: Option<u8>) -> Self {
        Self { mod_alpha, ..self }
    }

    /// Check if this cell is a space with transparent colors.
    pub fn is_empty(&self) -> bool {
        self.ch == ' ' && self.fg.a == 0 && self.bg.a == 0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_cell() {
        let cell = Cell::default();
        assert_eq!(cell.ch, ' ');
        assert_eq!(cell.fg, Color::TRANSPARENT);
        assert_eq!(cell.bg, Color::TRANSPARENT);
        assert!(cell.is_empty());
    }

    #[test]
    fn test_cell_with_char() {
        let cell = Cell::new('X');
        assert_eq!(cell.ch, 'X');
        assert!(!cell.is_empty());
    }

    #[test]
    fn test_cell_builders() {
        let cell = Cell::new('A')
            .with_fg(Color::RED)
            .with_bg(Color::BLUE)
            .with_mods(Modifiers::bold());

        assert_eq!(cell.ch, 'A');
        assert_eq!(cell.fg, Color::RED);
        assert_eq!(cell.bg, Color::BLUE);
        assert!(cell.mods.bold);
    }

    #[test]
    fn test_cell_modifier_alpha_builder() {
        let cell = Cell::new('Z').with_mod_alpha(Some(200));
        assert_eq!(cell.mod_alpha, Some(200));
    }
}

// <FILE>crates/tui-vfx-types/src/cell.rs</FILE> - <DESC>Cell type: character with styling</DESC>
// <VERS>END OF VERSION: 0.3.0</VERS>
