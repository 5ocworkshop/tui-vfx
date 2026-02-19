// <FILE>tui-vfx-geometry/src/transitions/mod.rs</FILE> - <DESC>Transition helpers (slide + expand/collapse)</DESC>
// <VERS>VERSION: 0.3.0</VERS>
// <WCTX>Physics motion path implementation</WCTX>
// <CLOG>Export interpolate_position for physics path support in tui_vfx_recipes</CLOG>

pub mod col_arc_bezier_point;
pub mod col_interpolate_position;
pub mod col_lerp;
pub mod fnc_expand_collapse_calculate_rect;
pub mod fnc_resolve_slide_direction;
pub mod fnc_slide_calculate_rect;
pub mod fnc_slide_calculate_rect_path;
pub mod fnc_slide_calculate_rect_path_with_path_type;
pub mod fnc_slide_offscreen_position;
pub mod fnc_slide_path_offscreen;
pub mod fnc_slide_path_offscreen_start_end;
pub mod types;

pub use col_interpolate_position::interpolate_position;
pub use fnc_expand_collapse_calculate_rect::expand_collapse_calculate_rect;
pub use fnc_resolve_slide_direction::resolve_slide_direction;
pub use fnc_slide_calculate_rect::slide_calculate_rect;
pub use fnc_slide_calculate_rect_path::slide_calculate_rect_path;
pub use fnc_slide_calculate_rect_path_with_path_type::{
    slide_calculate_rect_path_with_path_type, slide_calculate_signed_rect,
};
pub use fnc_slide_offscreen_position::slide_offscreen_position;
pub use fnc_slide_path_offscreen::slide_path_offscreen;
pub use fnc_slide_path_offscreen_start_end::slide_path_offscreen_start_end;
pub use types::{ExpandPhase, SlidePath, SlidePhase};

// <FILE>tui-vfx-geometry/src/transitions/mod.rs</FILE> - <DESC>Transition helpers (slide + expand/collapse)</DESC>
// <VERS>END OF VERSION: 0.3.0</VERS>
