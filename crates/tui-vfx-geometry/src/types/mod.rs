// <FILE>tui-vfx-geometry/src/types/mod.rs</FILE> - <DESC>Types module root</DESC>
// <VERS>VERSION: 2.1.0</VERS>
// <WCTX>Audit recommendation 1.2 + 1.3 — add WipeDirection as a foundational geometry type so the Wipe mask, RevealWipe shader, and V3 grouped reveal family share one source of truth (instead of three drifting copies).</WCTX>
// <CLOG>2.1.0: add cls_wipe_direction module hosting the canonical WipeDirection enum (cardinal + diagonal + centre/edges + corner-out + corner-in variants).
// 2.0.0: V2.2 schema standardization - added AnchorSpec</CLOG>

pub mod anchor;
pub mod cls_anchor_spec;
pub mod cls_easing_curve;
pub mod cls_keyframe_timeline;
pub mod cls_motion_spec;
pub mod cls_origin;
pub mod cls_placement_spec;
pub mod cls_rect_scale;
pub mod cls_shake;
pub mod cls_time_warp_curve;
pub mod cls_wipe_direction;
pub mod path_type;
pub mod position;
pub mod position_spec;
pub mod slide_direction;
pub mod snapping_strategy;
pub mod timeline;
pub mod transition_spec;

pub use anchor::Anchor;
pub use cls_anchor_spec::AnchorSpec;
pub use cls_easing_curve::EasingCurve;
pub use cls_keyframe_timeline::{Keyframe, KeyframeTimeline};
pub use cls_motion_spec::MotionSpec;
pub use cls_origin::Origin;
pub use cls_placement_spec::PlacementSpec;
pub use cls_rect_scale::RectScaleSpec;
pub use cls_shake::{Shake, ShakeOffset};
pub use cls_time_warp_curve::TimeWarpCurve;
pub use cls_wipe_direction::WipeDirection;
pub use path_type::PathType;
pub use position::{Position, SignedRect};
pub use position_spec::PositionSpec;
pub use slide_direction::SlideDirection;
pub use snapping_strategy::SnappingStrategy;
pub use timeline::Timeline;
pub use transition_spec::TransitionSpec;

// <FILE>tui-vfx-geometry/src/types/mod.rs</FILE> - <DESC>Types module root</DESC>
// <VERS>END OF VERSION: 2.1.0</VERS>
