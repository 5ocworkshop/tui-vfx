// <FILE>recyclebin/crates/tui-vfx-compositor/src/overlays/enum_loopback_badge_style.rs</FILE> - <DESC>Glyph-set selector for the abandoned hardcoded-cell-painter L3 badge; recycled in favor of recipe-driven badge per Intention 39</DESC>
// <VERS>VERSION: 0.1.0 (recycled)</VERS>
// <WCTX>Loopback Phase L3 first attempt (recycled 2026-04-26). Hardcoded LoopbackBadgeStyle { Auto, Ascii, NerdFont } that the abandoned `apply_loopback_badge` consumed. The badge style is now expressed by which `recipes/internal/loopback_badge*.json` recipe the host invokes; this enum is no longer needed.</WCTX>
// <CLOG>0.1.0 (recycled): preserved as the artefact surrounding Intention 39's emergence.</CLOG>

//! Glyph-set selector for the L3 loopback visibility badge.
//!
//! ABANDONED: superseded by recipe-based architecture per Intention 39.

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum LoopbackBadgeStyle {
    #[default]
    Auto,
    Ascii,
    NerdFont,
}

impl LoopbackBadgeStyle {
    pub fn resolve(self) -> Self {
        match self {
            Self::Auto => Self::NerdFont,
            other => other,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_is_auto() {
        assert_eq!(LoopbackBadgeStyle::default(), LoopbackBadgeStyle::Auto);
    }

    #[test]
    fn auto_resolves_to_nerd_font_in_v1() {
        assert_eq!(
            LoopbackBadgeStyle::Auto.resolve(),
            LoopbackBadgeStyle::NerdFont
        );
    }

    #[test]
    fn explicit_ascii_passes_through_resolution() {
        assert_eq!(
            LoopbackBadgeStyle::Ascii.resolve(),
            LoopbackBadgeStyle::Ascii
        );
    }

    #[test]
    fn explicit_nerd_font_passes_through_resolution() {
        assert_eq!(
            LoopbackBadgeStyle::NerdFont.resolve(),
            LoopbackBadgeStyle::NerdFont
        );
    }
}

// <FILE>recyclebin/crates/tui-vfx-compositor/src/overlays/enum_loopback_badge_style.rs</FILE>
// <VERS>END OF VERSION: 0.1.0 (recycled)</VERS>
