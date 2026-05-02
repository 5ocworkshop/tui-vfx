// <FILE>crates/tui-vfx-compost/src/render/fnc_shadow_cast_rect.rs</FILE> - <DESC>Resolve scene-element shadow casting rectangle</DESC>
// <VERS>VERSION: 0.1.0</VERS>
// <WCTX>Shadow edge-crossing policy selects clipped-visible versus preserved source casting bounds.</WCTX>
// <CLOG>0.1.0: INIT — map v3.1 edgeCrossingPolicy into tui-vfx-shadow Rect geometry.</CLOG>

use tui_vfx_contract::{RecipeSceneElement, ShadowEdgeCrossingPolicy, ShadowSpec};
use tui_vfx_types::{Grid, Rect, SemanticScene};

use crate::render::ElementClipBounds;

pub(crate) fn shadow_cast_rect(
    element: &RecipeSceneElement,
    source: &SemanticScene,
    bounds: ElementClipBounds,
    shadow: &ShadowSpec,
) -> Rect {
    match shadow.edge_crossing_policy {
        Some(ShadowEdgeCrossingPolicy::Preserve) => Rect::new(
            clamp_usize_to_u16(bounds.dest_x_start),
            clamp_usize_to_u16(bounds.dest_y_start),
            clamp_usize_to_u16(preserved_extent(element.placement.x, source.grid().width())),
            clamp_usize_to_u16(preserved_extent(
                element.placement.y,
                source.grid().height(),
            )),
        ),
        None | Some(ShadowEdgeCrossingPolicy::Default | ShadowEdgeCrossingPolicy::Fade) => {
            Rect::new(
                clamp_usize_to_u16(bounds.dest_x_start),
                clamp_usize_to_u16(bounds.dest_y_start),
                clamp_usize_to_u16(bounds.width),
                clamp_usize_to_u16(bounds.height),
            )
        }
    }
}

pub(crate) fn shadow_edge_progress(
    source: &SemanticScene,
    bounds: ElementClipBounds,
    shadow: &ShadowSpec,
) -> f64 {
    if shadow.edge_crossing_policy != Some(ShadowEdgeCrossingPolicy::Fade) {
        return 1.0;
    }
    let total_area = source.grid().width().saturating_mul(source.grid().height());
    if total_area == 0 {
        return 0.0;
    }
    let visible_area = bounds.width.saturating_mul(bounds.height);
    (visible_area as f64 / total_area as f64).clamp(0.0, 1.0)
}

fn preserved_extent(origin: i32, length: usize) -> usize {
    if origin < 0 {
        length.saturating_sub(origin.unsigned_abs() as usize)
    } else {
        length
    }
}
fn clamp_usize_to_u16(value: usize) -> u16 {
    value.min(usize::from(u16::MAX)) as u16
}

// <FILE>crates/tui-vfx-compost/src/render/fnc_shadow_cast_rect.rs</FILE> - <DESC>Resolve scene-element shadow casting rectangle</DESC>
// <VERS>END OF VERSION: 0.1.0</VERS>
