// <FILE>crates/tui-vfx-content/src/cell_motion/fnc_collect_cell_actors.rs</FILE> - <DESC>Collect selected cell-motion actors</DESC>
// <VERS>VERSION: 0.1.0</VERS>
// <WCTX>V3 Packet 1: row-major actor extraction and selected-cell baseline clearing.</WCTX>
// <CLOG>0.1.0: add deterministic actor extraction using Cell::is_empty and authored scopes.</CLOG>

use super::{CellActor, CellMotionAffect, CellMotionPhaseSpec};
use tui_vfx_types::{Cell, Grid, OwnedGrid, RoleTag, SemanticScene};

/// Collect selected actors and return an output baseline with selected authored cells cleared.
pub fn collect_cell_actors(
    scene: &SemanticScene,
    phase_spec: &CellMotionPhaseSpec,
) -> (Vec<CellActor>, SemanticScene) {
    let area = scene.area();
    let mut actors = Vec::new();
    let mut baseline_grid = scene.grid().clone();
    let mut baseline_roles = scene.roles().clone();
    let mut selected_ordinal = 0_u32;
    for y in 0..area.height {
        for x in 0..area.width {
            let cell = *scene.cell((x, y)).unwrap_or(&Cell::default());
            let role = scene.role((x, y)).unwrap_or(RoleTag::Background);
            let affect_ok = matches!(phase_spec.affect, CellMotionAffect::All) || !cell.is_empty();
            let scope_ok = phase_spec
                .scope
                .as_ref()
                .is_none_or(|scope| scope.contains(x, y));
            if affect_ok && scope_ok {
                let authored_index = y as u32 * area.width as u32 + x as u32;
                actors.push(CellActor {
                    authored_index,
                    selected_ordinal,
                    authored_x: x,
                    authored_y: y,
                    cell,
                    role,
                });
                selected_ordinal += 1;
                baseline_grid.set(x as usize, y as usize, Cell::default());
                baseline_roles.set((x, y), RoleTag::Background);
            }
        }
    }
    (
        actors,
        SemanticScene::new(
            OwnedGrid::from_cells(
                area.width as usize,
                area.height as usize,
                baseline_grid.cells().to_vec(),
            ),
            baseline_roles,
        )
        .with_metadata(scene.metadata().clone()),
    )
}

// <FILE>crates/tui-vfx-content/src/cell_motion/fnc_collect_cell_actors.rs</FILE>
// <VERS>END OF VERSION: 0.1.0</VERS>
