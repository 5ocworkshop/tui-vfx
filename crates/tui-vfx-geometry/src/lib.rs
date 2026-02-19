// <FILE>tui-vfx-geometry/src/lib.rs</FILE> - <DESC>Library root</DESC>
// <VERS>VERSION: 1.4.0 - 2025-12-25</VERS>
// <WCTX>Mixed-signals migration Phase 5 - WP3</WCTX>
// <CLOG>Removed internal module (bezier math migrated to mixed_signals)</CLOG>

pub mod anchors;
pub mod borders;
pub mod easing;
pub mod layout;
pub mod paths;
pub mod traits;
pub mod transitions;
pub mod types;
pub mod widgets;
// Re-exports
pub use traits::MotionPath;
pub use types::{
    Anchor, Origin, PathType, Position, PositionSpec, RectScaleSpec, SignedRect, SlideDirection,
    SnappingStrategy, TransitionSpec,
};

// <FILE>tui-vfx-geometry/src/lib.rs</FILE> - <DESC>Library root</DESC>
// <VERS>END OF VERSION: 1.4.0 - 2025-12-25</VERS>
