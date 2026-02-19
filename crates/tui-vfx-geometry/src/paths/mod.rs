// <FILE>tui-vfx-geometry/src/paths/mod.rs</FILE> - <DESC>Updated paths module</DESC>
// <VERS>VERSION: 1.5.0 - 2025-12-18T23:30:00Z</VERS>
// <WCTX>V2 Recipe Gap Analysis: Arbitrary waypoint support</WCTX>
// <CLOG>Added cls_bezier_path for explicit control point paths</CLOG>

pub mod cls_arc_path;
pub mod cls_bezier_path;
pub mod cls_hover_path;
pub mod cls_linear_path;
pub mod cls_rectilinear_path;
pub mod cls_spiral_path;
pub mod cls_spring_path;
pub mod cls_squash_path;
pub mod cls_step_path;
pub use cls_arc_path::ArcPath;
pub use cls_bezier_path::BezierPath;
pub use cls_hover_path::HoverPath;
pub use cls_linear_path::LinearPath;
pub use cls_rectilinear_path::RectilinearPath;
pub use cls_spiral_path::SpiralPath;
pub use cls_spring_path::SpringPath;
pub use cls_squash_path::SquashPath;
pub use cls_step_path::StepPath;

// <FILE>tui-vfx-geometry/src/paths/mod.rs</FILE> - <DESC>Updated paths module</DESC>
// <VERS>END OF VERSION: 1.5.0 - 2025-12-18T23:30:00Z</VERS>
