// <FILE>crates/tui-vfx-types/src/style.rs</FILE> - <DESC>Style type: fg + bg + modifiers</DESC>
// <VERS>VERSION: 0.2.0</VERS>
// <WCTX>Full modifier parity with ratatui</WCTX>
// <CLOG>Add 5 missing builder methods: reverse, strikethrough, slow_blink, rapid_blink, hidden

//! Framework-agnostic style type for text rendering.

use crate::{Color, Modifiers};

/// Text styling without character content.
///
/// Represents the visual appearance of text: foreground color, background color,
/// and text modifiers (bold, italic, etc.).
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Style {
    /// Foreground (text) color.
    pub fg: Color,
    /// Background color.
    pub bg: Color,
    /// Text modifiers (bold, italic, underline, etc.).
    pub mods: Modifiers,
}

impl Style {
    /// Create a new style with the given foreground, background, and modifiers.
    pub const fn new(fg: Color, bg: Color, mods: Modifiers) -> Self {
        Self { fg, bg, mods }
    }

    /// Create a style with only foreground color set.
    pub const fn fg(color: Color) -> Self {
        Self {
            fg: color,
            bg: Color::TRANSPARENT,
            mods: Modifiers::NONE,
        }
    }

    /// Create a style with only background color set.
    pub const fn bg(color: Color) -> Self {
        Self {
            fg: Color::TRANSPARENT,
            bg: color,
            mods: Modifiers::NONE,
        }
    }

    /// Return a new style with the foreground color changed.
    pub const fn with_fg(self, fg: Color) -> Self {
        Self {
            fg,
            bg: self.bg,
            mods: self.mods,
        }
    }

    /// Return a new style with the background color changed.
    pub const fn with_bg(self, bg: Color) -> Self {
        Self {
            fg: self.fg,
            bg,
            mods: self.mods,
        }
    }

    /// Return a new style with modifiers changed.
    pub const fn with_mods(self, mods: Modifiers) -> Self {
        Self {
            fg: self.fg,
            bg: self.bg,
            mods,
        }
    }

    /// Return a new style with bold enabled.
    pub const fn bold(self) -> Self {
        Self {
            fg: self.fg,
            bg: self.bg,
            mods: self.mods.with_bold(),
        }
    }

    /// Return a new style with italic enabled.
    pub const fn italic(self) -> Self {
        Self {
            fg: self.fg,
            bg: self.bg,
            mods: self.mods.with_italic(),
        }
    }

    /// Return a new style with underline enabled.
    pub const fn underline(self) -> Self {
        Self {
            fg: self.fg,
            bg: self.bg,
            mods: self.mods.with_underline(),
        }
    }

    /// Return a new style with dim enabled.
    pub const fn dim(self) -> Self {
        Self {
            fg: self.fg,
            bg: self.bg,
            mods: self.mods.with_dim(),
        }
    }

    /// Return a new style with reverse enabled.
    pub const fn reverse(self) -> Self {
        Self {
            fg: self.fg,
            bg: self.bg,
            mods: self.mods.with_reverse(),
        }
    }

    /// Return a new style with strikethrough enabled.
    pub const fn strikethrough(self) -> Self {
        Self {
            fg: self.fg,
            bg: self.bg,
            mods: self.mods.with_strikethrough(),
        }
    }

    /// Return a new style with slow_blink enabled.
    /// Note: Slow blink (SGR 5) is poorly supported by modern terminals.
    pub const fn slow_blink(self) -> Self {
        Self {
            fg: self.fg,
            bg: self.bg,
            mods: self.mods.with_slow_blink(),
        }
    }

    /// Return a new style with rapid_blink enabled.
    /// Note: Rapid blink (SGR 6) is rarely supported by terminals.
    pub const fn rapid_blink(self) -> Self {
        Self {
            fg: self.fg,
            bg: self.bg,
            mods: self.mods.with_rapid_blink(),
        }
    }

    /// Return a new style with hidden enabled.
    /// Note: Hidden text (SGR 8) is often disabled for security reasons.
    pub const fn hidden(self) -> Self {
        Self {
            fg: self.fg,
            bg: self.bg,
            mods: self.mods.with_hidden(),
        }
    }

    /// Check if this style has any non-default values.
    pub fn is_empty(&self) -> bool {
        self.fg == Color::TRANSPARENT
            && self.bg == Color::TRANSPARENT
            && self.mods == Modifiers::NONE
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_style() {
        let style = Style::default();
        assert_eq!(style.fg, Color::default());
        assert_eq!(style.bg, Color::default());
        assert_eq!(style.mods, Modifiers::default());
        assert!(style.is_empty());
    }

    #[test]
    fn test_style_constructors() {
        let red = Color::rgb(255, 0, 0);
        let blue = Color::rgb(0, 0, 255);

        let fg_only = Style::fg(red);
        assert_eq!(fg_only.fg, red);
        assert_eq!(fg_only.bg, Color::TRANSPARENT);

        let bg_only = Style::bg(blue);
        assert_eq!(bg_only.fg, Color::TRANSPARENT);
        assert_eq!(bg_only.bg, blue);
    }

    #[test]
    fn test_builder_methods() {
        let style = Style::default()
            .with_fg(Color::rgb(255, 255, 255))
            .with_bg(Color::rgb(0, 0, 0))
            .bold()
            .italic();

        assert_eq!(style.fg, Color::rgb(255, 255, 255));
        assert_eq!(style.bg, Color::rgb(0, 0, 0));
        assert!(style.mods.bold);
        assert!(style.mods.italic);
    }

    #[test]
    fn test_all_modifier_builders() {
        let style = Style::default()
            .reverse()
            .strikethrough()
            .slow_blink()
            .rapid_blink()
            .hidden();

        assert!(style.mods.reverse);
        assert!(style.mods.strikethrough);
        assert!(style.mods.slow_blink);
        assert!(style.mods.rapid_blink);
        assert!(style.mods.hidden);
    }
}

// <FILE>crates/tui-vfx-types/src/style.rs</FILE> - <DESC>Style type: fg + bg + modifiers</DESC>
// <VERS>END OF VERSION: 0.2.0</VERS>
