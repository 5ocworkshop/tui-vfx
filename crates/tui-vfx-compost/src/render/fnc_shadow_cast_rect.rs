// <FILE>crates/tui-vfx-compost/src/render/fnc_shadow_cast_rect.rs</FILE> - <DESC>Resolve scene-element shadow casting rectangle</DESC>
// <VERS>VERSION: 0.2.0</VERS>
// <WCTX>Shadow edge-crossing policy selects clipped-visible versus preserved source casting bounds.</WCTX>
// <CLOG>0.2.0: MINOR — evaluate sourceRegion scopes before resolving shadow cast rectangles.
// 0.1.0: INIT — map v3.1 edgeCrossingPolicy into tui-vfx-shadow Rect geometry.</CLOG>

use tui_vfx_contract::{
    CoordinateSpace, RecipeSceneElement, RoleSpace, ScopeEvalInput, ShadowEdgeCrossingPolicy,
    ShadowOutset, ShadowSpec,
};
use tui_vfx_types::{Grid, Rect, RoleTag, SemanticScene};

use crate::render::ElementClipBounds;

pub(crate) fn shadow_cast_rect(
    element: &RecipeSceneElement,
    source: &SemanticScene,
    bounds: ElementClipBounds,
    shadow: &ShadowSpec,
) -> Rect {
    let cast_rect = if let Some(region_rect) =
        source_region_cast_rect(element, source, bounds, shadow)
    {
        region_rect
    } else {
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
    };

    shadow
        .paint_outset
        .map_or(cast_rect, |outset| apply_paint_outset(cast_rect, outset))
}

fn apply_paint_outset(rect: Rect, outset: ShadowOutset) -> Rect {
    let x = rect.x.saturating_sub(outset.left);
    let y = rect.y.saturating_sub(outset.top);
    let right = rect
        .x
        .saturating_add(rect.width)
        .saturating_add(outset.right);
    let bottom = rect
        .y
        .saturating_add(rect.height)
        .saturating_add(outset.bottom);
    Rect::new(x, y, right.saturating_sub(x), bottom.saturating_sub(y))
}

fn source_region_cast_rect(
    element: &RecipeSceneElement,
    source: &SemanticScene,
    bounds: ElementClipBounds,
    shadow: &ShadowSpec,
) -> Option<Rect> {
    let scope = shadow.source_region.as_ref()?;
    let mut min_x = usize::MAX;
    let mut min_y = usize::MAX;
    let mut max_x = 0usize;
    let mut max_y = 0usize;
    let mut matched = false;

    for y in bounds.local_y_start..bounds.local_y_start + bounds.height {
        for x in bounds.local_x_start..bounds.local_x_start + bounds.width {
            let input = ScopeEvalInput {
                destination_x: x.saturating_sub(bounds.local_x_start),
                destination_y: y.saturating_sub(bounds.local_y_start),
                sampled_source_x: x,
                sampled_source_y: y,
                sampled_source_role: source
                    .role((x as u16, y as u16))
                    .unwrap_or(RoleTag::Background),
                destination_role: RoleTag::Background,
                destination_width: Some(bounds.width),
                destination_height: Some(bounds.height),
                sampled_source_width: Some(source.grid().width()),
                sampled_source_height: Some(source.grid().height()),
                destination_glyph: None,
                sampled_source_glyph: source.grid().get(x, y).map(|cell| cell.ch.to_string()),
            };
            if scope.matches(
                &input,
                CoordinateSpace::SampledSource,
                RoleSpace::SampledSource,
            ) {
                matched = true;
                min_x = min_x.min(x);
                min_y = min_y.min(y);
                max_x = max_x.max(x);
                max_y = max_y.max(y);
            }
        }
    }

    matched.then(|| {
        Rect::new(
            clamp_i32_to_u16(element.placement.x + min_x as i32),
            clamp_i32_to_u16(element.placement.y + min_y as i32),
            clamp_usize_to_u16(max_x - min_x + 1),
            clamp_usize_to_u16(max_y - min_y + 1),
        )
    })
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

fn clamp_i32_to_u16(value: i32) -> u16 {
    value.clamp(0, i32::from(u16::MAX)) as u16
}

// <FILE>crates/tui-vfx-compost/src/render/fnc_shadow_cast_rect.rs</FILE> - <DESC>Resolve scene-element shadow casting rectangle</DESC>
// <VERS>END OF VERSION: 0.2.0</VERS>
