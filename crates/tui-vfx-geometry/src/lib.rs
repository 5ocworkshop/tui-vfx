// <FILE>tui-vfx-geometry/src/lib.rs</FILE> - <DESC>Library root</DESC>
// <VERS>VERSION: 1.5.0</VERS>
// <WCTX>Audit recommendation 1.2 + 1.3 — surface the canonical WipeDirection and shared wipe geometry helpers at the crate root so downstream crates (tui-vfx-style for the RevealWipe shader, tui-vfx-compositor for the Wipe mask) consume one source of truth.</WCTX>
// <CLOG>1.5.0: add wipe module + re-export WipeDirection at crate root.
// 1.4.0: Mixed-signals migration Phase 5 - WP3 (removed internal module, bezier math migrated to mixed_signals)</CLOG>

pub mod anchors;
pub mod borders;
pub mod easing;
pub mod layout;
pub mod paths;
pub mod traits;
pub mod transitions;
pub mod types;
pub mod widgets;
pub mod wipe;
// Re-exports
pub use traits::MotionPath;
pub use types::{
    Anchor, Origin, PathType, Position, PositionSpec, RectScaleSpec, SignedRect, SlideDirection,
    SnappingStrategy, TransitionSpec, WipeDirection,
};
pub use wipe::{wipe_progress, wipe_visible_at};

// <FILE>tui-vfx-geometry/src/lib.rs</FILE> - <DESC>Library root</DESC>
// <VERS>END OF VERSION: 1.5.0</VERS>
