// <FILE>crates/tui-vfx-compost/src/render/fnc_render_element_shadow.rs</FILE> - <DESC>Render scene-element surface shadows into the destination scene</DESC>
// <VERS>VERSION: 0.2.0</VERS>
// <WCTX>Surface shadows reuse tui-vfx-shadow geometry and compositor source-over blending while reporting actual native RoleTag::Shadow writes.</WCTX>
// <CLOG>0.2.0: MINOR — return actual shadow cell write count for observability.
// 0.1.0: INIT — render v3.1 surface shadow attachments around source-backed scene elements.</CLOG>

use tui_vfx_contract::{ShadowBlendMode, ShadowGlyphMaterial, ShadowSpec};
use tui_vfx_shadow::render_shadow;
use tui_vfx_types::{Grid, OwnedGrid, Rect, RoleTag, SemanticScene};

use crate::render::{blend_shadow_cell, blend_underlying_shadow_cell, build_shadow_config};

pub(crate) fn render_element_shadow(
    destination: &mut SemanticScene,
    shadow: &ShadowSpec,
    rect: Rect,
    progress: f64,
) -> u32 {
    let config = build_shadow_config(shadow);
    let mut shadow_grid = OwnedGrid::new(destination.grid().width(), destination.grid().height());
    render_shadow(&mut shadow_grid, rect, &config, progress);
    merge_shadow_grid(
        destination,
        &shadow_grid,
        shadow.glyph_material,
        shadow.blend_mode,
    )
}

fn merge_shadow_grid(
    destination: &mut SemanticScene,
    shadow_grid: &OwnedGrid,
    glyph_material: Option<ShadowGlyphMaterial>,
    blend_mode: ShadowBlendMode,
) -> u32 {
    let mut written_cells = 0;
    for y in 0..shadow_grid.height() {
        for x in 0..shadow_grid.width() {
            let Some(shadow_cell) = shadow_grid.get(x, y) else {
                continue;
            };
            if shadow_cell.ch == ' ' && shadow_cell.bg.a == 0 && shadow_cell.fg.a == 0 {
                continue;
            }
            let dest_cell = destination.grid().get(x, y).copied().unwrap_or_default();
            let blended_cell = match glyph_material {
                Some(ShadowGlyphMaterial::PreserveDestination) => {
                    blend_underlying_shadow_cell(shadow_cell, &dest_cell, blend_mode)
                }
                None | Some(ShadowGlyphMaterial::Solid) => {
                    blend_shadow_cell(shadow_cell, &dest_cell, blend_mode)
                }
            };
            destination.grid_mut().set(x, y, blended_cell);
            destination
                .roles_mut()
                .set((x as u16, y as u16), RoleTag::Shadow);
            written_cells += 1;
        }
    }
    written_cells
}

// <FILE>crates/tui-vfx-compost/src/render/fnc_render_element_shadow.rs</FILE> - <DESC>Render scene-element surface shadows into the destination scene</DESC>
// <VERS>END OF VERSION: 0.2.0</VERS>
