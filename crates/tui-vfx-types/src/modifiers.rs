// <FILE>crates/tui-vfx-types/src/modifiers.rs</FILE> - <DESC>Text modifiers for terminal display</DESC>
// <VERS>VERSION: 0.2.0</VERS>
// <WCTX>Full modifier parity with ratatui</WCTX>
// <CLOG>Add slow_blink, rapid_blink, hidden modifiers for full ratatui parity

//! Text modifiers for terminal styling.

/// Text modifiers for terminal display.
///
/// These map to common terminal text attributes supported by most terminals.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Modifiers {
    pub bold: bool,
    pub italic: bool,
    pub underline: bool,
    pub dim: bool,
    pub reverse: bool,
    pub strikethrough: bool,
    /// Slow blink (SGR 5). Note: Poorly supported by modern terminals.
    pub slow_blink: bool,
    /// Rapid blink (SGR 6). Note: Rarely supported by terminals.
    pub rapid_blink: bool,
    /// Hidden/invisible text (SGR 8). Note: Often disabled for security.
    pub hidden: bool,
}

impl Modifiers {
    /// No modifiers active.
    pub const NONE: Modifiers = Modifiers {
        bold: false,
        italic: false,
        underline: false,
        dim: false,
        reverse: false,
        strikethrough: false,
        slow_blink: false,
        rapid_blink: false,
        hidden: false,
    };

    /// Create modifiers with bold enabled.
    #[inline]
    pub const fn bold() -> Self {
        Self {
            bold: true,
            ..Self::NONE
        }
    }

    /// Create modifiers with italic enabled.
    #[inline]
    pub const fn italic() -> Self {
        Self {
            italic: true,
            ..Self::NONE
        }
    }

    /// Create modifiers with underline enabled.
    #[inline]
    pub const fn underline() -> Self {
        Self {
            underline: true,
            ..Self::NONE
        }
    }

    /// Create modifiers with dim enabled.
    #[inline]
    pub const fn dim() -> Self {
        Self {
            dim: true,
            ..Self::NONE
        }
    }

    /// Create modifiers with reverse enabled.
    #[inline]
    pub const fn reverse() -> Self {
        Self {
            reverse: true,
            ..Self::NONE
        }
    }

    /// Create modifiers with strikethrough enabled.
    #[inline]
    pub const fn strikethrough() -> Self {
        Self {
            strikethrough: true,
            ..Self::NONE
        }
    }

    /// Create modifiers with slow_blink enabled.
    /// Note: Slow blink (SGR 5) is poorly supported by modern terminals.
    #[inline]
    pub const fn slow_blink() -> Self {
        Self {
            slow_blink: true,
            ..Self::NONE
        }
    }

    /// Create modifiers with rapid_blink enabled.
    /// Note: Rapid blink (SGR 6) is rarely supported by terminals.
    #[inline]
    pub const fn rapid_blink() -> Self {
        Self {
            rapid_blink: true,
            ..Self::NONE
        }
    }

    /// Create modifiers with hidden enabled.
    /// Note: Hidden text (SGR 8) is often disabled for security reasons.
    #[inline]
    pub const fn hidden() -> Self {
        Self {
            hidden: true,
            ..Self::NONE
        }
    }

    /// Check if any modifier is active.
    #[inline]
    pub const fn is_empty(&self) -> bool {
        !self.bold
            && !self.italic
            && !self.underline
            && !self.dim
            && !self.reverse
            && !self.strikethrough
            && !self.slow_blink
            && !self.rapid_blink
            && !self.hidden
    }

    /// Combine with another set of modifiers (OR).
    #[inline]
    pub const fn combine(self, other: Self) -> Self {
        Self {
            bold: self.bold || other.bold,
            italic: self.italic || other.italic,
            underline: self.underline || other.underline,
            dim: self.dim || other.dim,
            reverse: self.reverse || other.reverse,
            strikethrough: self.strikethrough || other.strikethrough,
            slow_blink: self.slow_blink || other.slow_blink,
            rapid_blink: self.rapid_blink || other.rapid_blink,
            hidden: self.hidden || other.hidden,
        }
    }

    /// Return new modifiers with bold added.
    #[inline]
    pub const fn with_bold(self) -> Self {
        Self { bold: true, ..self }
    }

    /// Return new modifiers with italic added.
    #[inline]
    pub const fn with_italic(self) -> Self {
        Self {
            italic: true,
            ..self
        }
    }

    /// Return new modifiers with underline added.
    #[inline]
    pub const fn with_underline(self) -> Self {
        Self {
            underline: true,
            ..self
        }
    }

    /// Return new modifiers with dim added.
    #[inline]
    pub const fn with_dim(self) -> Self {
        Self { dim: true, ..self }
    }

    /// Return new modifiers with reverse added.
    #[inline]
    pub const fn with_reverse(self) -> Self {
        Self {
            reverse: true,
            ..self
        }
    }

    /// Return new modifiers with strikethrough added.
    #[inline]
    pub const fn with_strikethrough(self) -> Self {
        Self {
            strikethrough: true,
            ..self
        }
    }

    /// Return new modifiers with slow_blink added.
    #[inline]
    pub const fn with_slow_blink(self) -> Self {
        Self {
            slow_blink: true,
            ..self
        }
    }

    /// Return new modifiers with rapid_blink added.
    #[inline]
    pub const fn with_rapid_blink(self) -> Self {
        Self {
            rapid_blink: true,
            ..self
        }
    }

    /// Return new modifiers with hidden added.
    #[inline]
    pub const fn with_hidden(self) -> Self {
        Self {
            hidden: true,
            ..self
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_modifiers_combine() {
        let bold = Modifiers::bold();
        let italic = Modifiers::italic();
        let combined = bold.combine(italic);
        assert!(combined.bold);
        assert!(combined.italic);
        assert!(!combined.underline);
    }

    #[test]
    fn test_is_empty() {
        assert!(Modifiers::NONE.is_empty());
        assert!(!Modifiers::bold().is_empty());
    }

    #[test]
    fn test_with_builders() {
        let mods = Modifiers::NONE.with_bold().with_italic();
        assert!(mods.bold);
        assert!(mods.italic);
        assert!(!mods.underline);
    }

    #[test]
    fn test_new_modifiers() {
        assert!(Modifiers::slow_blink().slow_blink);
        assert!(Modifiers::rapid_blink().rapid_blink);
        assert!(Modifiers::hidden().hidden);
    }

    #[test]
    fn test_new_with_builders() {
        let mods = Modifiers::NONE
            .with_slow_blink()
            .with_rapid_blink()
            .with_hidden();
        assert!(mods.slow_blink);
        assert!(mods.rapid_blink);
        assert!(mods.hidden);
        assert!(!mods.bold);
    }

    #[test]
    fn test_combine_with_new_modifiers() {
        let a = Modifiers::slow_blink();
        let b = Modifiers::hidden();
        let combined = a.combine(b);
        assert!(combined.slow_blink);
        assert!(combined.hidden);
        assert!(!combined.rapid_blink);
    }

    #[test]
    fn test_is_empty_with_new_modifiers() {
        assert!(!Modifiers::slow_blink().is_empty());
        assert!(!Modifiers::rapid_blink().is_empty());
        assert!(!Modifiers::hidden().is_empty());
    }
}

// <FILE>crates/tui-vfx-types/src/modifiers.rs</FILE> - <DESC>Text modifiers for terminal display</DESC>
// <VERS>END OF VERSION: 0.2.0</VERS>
