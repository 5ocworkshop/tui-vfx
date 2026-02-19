// <FILE>crates/tui-vfx-shadow/src/types/shadow_edge.rs</FILE> - <DESC>Bitflags for shadow edge selection</DESC>
// <VERS>VERSION: 0.2.0</VERS>
// <WCTX>Enable serde support for shadow edge flags</WCTX>
// <CLOG>Add Serialize/Deserialize derives for ShadowEdges</CLOG>

//! Bitflags for selecting which edges to render shadows on.

use bitflags::bitflags;
use serde::{Deserialize, Serialize};

bitflags! {
    /// Flags indicating which edges of an element should have shadows rendered.
    ///
    /// Multiple edges can be combined using bitwise OR operations.
    ///
    /// # Examples
    ///
    /// ```
    /// use tui_vfx_shadow::ShadowEdges;
    ///
    /// // Shadow on right and bottom (typical drop shadow)
    /// let edges = ShadowEdges::RIGHT | ShadowEdges::BOTTOM;
    ///
    /// // Or use the convenience constant
    /// let edges = ShadowEdges::BOTTOM_RIGHT;
    ///
    /// // Shadow on all sides
    /// let edges = ShadowEdges::ALL;
    /// ```
    #[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash, Serialize, Deserialize)]
    pub struct ShadowEdges: u8 {
        /// Shadow on the right edge.
        const RIGHT = 0b0001;
        /// Shadow on the bottom edge.
        const BOTTOM = 0b0010;
        /// Shadow on the left edge.
        const LEFT = 0b0100;
        /// Shadow on the top edge.
        const TOP = 0b1000;

        /// Convenience: bottom-right corner shadow (typical drop shadow).
        const BOTTOM_RIGHT = Self::RIGHT.bits() | Self::BOTTOM.bits();
        /// Convenience: top-left corner shadow (inverted drop shadow).
        const TOP_LEFT = Self::LEFT.bits() | Self::TOP.bits();
        /// Shadow on all four edges.
        const ALL = Self::RIGHT.bits() | Self::BOTTOM.bits() | Self::LEFT.bits() | Self::TOP.bits();
    }
}

impl ShadowEdges {
    /// Check if the right edge should have a shadow.
    #[inline]
    pub fn has_right(self) -> bool {
        self.contains(Self::RIGHT)
    }

    /// Check if the bottom edge should have a shadow.
    #[inline]
    pub fn has_bottom(self) -> bool {
        self.contains(Self::BOTTOM)
    }

    /// Check if the left edge should have a shadow.
    #[inline]
    pub fn has_left(self) -> bool {
        self.contains(Self::LEFT)
    }

    /// Check if the top edge should have a shadow.
    #[inline]
    pub fn has_top(self) -> bool {
        self.contains(Self::TOP)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_is_empty() {
        assert_eq!(ShadowEdges::default(), ShadowEdges::empty());
    }

    #[test]
    fn test_bottom_right_contains_both() {
        let edges = ShadowEdges::BOTTOM_RIGHT;
        assert!(edges.has_right());
        assert!(edges.has_bottom());
        assert!(!edges.has_left());
        assert!(!edges.has_top());
    }

    #[test]
    fn test_all_contains_all_edges() {
        let edges = ShadowEdges::ALL;
        assert!(edges.has_right());
        assert!(edges.has_bottom());
        assert!(edges.has_left());
        assert!(edges.has_top());
    }

    #[test]
    fn test_combining_edges() {
        let edges = ShadowEdges::RIGHT | ShadowEdges::TOP;
        assert!(edges.has_right());
        assert!(edges.has_top());
        assert!(!edges.has_left());
        assert!(!edges.has_bottom());
    }
}

// <FILE>crates/tui-vfx-shadow/src/types/shadow_edge.rs</FILE> - <DESC>Bitflags for shadow edge selection</DESC>
// <VERS>END OF VERSION: 0.2.0</VERS>
