// <FILE>crates/tui-vfx-player/src/fnc_build_player_render_ir.rs</FILE> - <DESC>Build player-owned render IR reports</DESC>
// <VERS>VERSION: 0.1.0</VERS>
// <WCTX>Player render evidence work: centralize render IR construction from existing frame reports.</WCTX>
// <CLOG>0.1.0: INIT — convert frame reports, graph values, and recipe scene metadata into render IR.</CLOG>

use std::collections::BTreeMap;

use tui_vfx_contract::{CellWritePolicy, GraphValueId, RecipeDocument, Value};

use crate::{
    PlayerFrameReport, PlayerRenderCell, PlayerRenderGraphValueSnapshot, PlayerRenderIrReport,
    PlayerRenderProvenance,
};

/// Build a render IR report from an existing frame report plus graph and recipe metadata.
pub fn build_player_render_ir(
    recipe: &RecipeDocument,
    frame_report: PlayerFrameReport,
    graph_values: BTreeMap<GraphValueId, Value>,
) -> PlayerRenderIrReport {
    let styled_cells = frame_report
        .styled_grid
        .as_ref()
        .map(|grid| {
            grid.cells()
                .iter()
                .filter(|cell| !cell.is_default())
                .map(|cell| PlayerRenderCell {
                    x: cell.x,
                    y: cell.y,
                    glyph: cell.glyph.clone(),
                    foreground: cell.foreground.clone(),
                    background: cell.background.clone(),
                    modifiers: cell.modifiers.clone(),
                    role: cell.role.clone(),
                })
                .collect::<Vec<_>>()
        })
        .unwrap_or_default();
    let provenance = recipe
        .scenes
        .first()
        .map(|scene| {
            scene
                .elements
                .iter()
                .map(|element| {
                    let source = recipe.sources.get(&element.source);
                    PlayerRenderProvenance {
                        scene_id: scene.id.as_str().to_string(),
                        element_id: element.id.as_str().to_string(),
                        layer_id: element.layer.as_ref().map(|id| id.as_str().to_string()),
                        source_id: Some(element.source.as_str().to_string()),
                        source_descriptor_id: source
                            .map(|source| source.source.as_str().to_string()),
                        x: element.placement.x,
                        y: element.placement.y,
                        z_index: element.z_index,
                        cell_write_policy: cell_write_policy_label(element.cell_write_policy)
                            .to_string(),
                    }
                })
                .collect::<Vec<_>>()
        })
        .unwrap_or_default();
    let graph_values = graph_values
        .into_iter()
        .map(|(id, value)| PlayerRenderGraphValueSnapshot {
            id: id.as_str().to_string(),
            value,
        })
        .collect();

    PlayerRenderIrReport {
        schema_version: "v3.1.player.renderIr.1",
        recipe_id: frame_report.recipe_id,
        path: frame_report.path,
        status: frame_report.status,
        phase: frame_report.phase,
        phase_t: frame_report.phase_t,
        loop_t: frame_report.loop_t,
        width: frame_report.width,
        height: frame_report.height,
        render_hash: frame_report.render_hash,
        non_empty_cells: frame_report.non_empty_cells,
        rows: frame_report.rows,
        styled_cells,
        provenance,
        graph_values,
        errors: frame_report.errors,
        warnings: frame_report.warnings,
    }
}

fn cell_write_policy_label(policy: CellWritePolicy) -> &'static str {
    match policy {
        CellWritePolicy::WriteCell => "writeCell",
        CellWritePolicy::SkipTransparentEmpty => "skipTransparentEmpty",
    }
}

// <FILE>crates/tui-vfx-player/src/fnc_build_player_render_ir.rs</FILE> - <DESC>Build player-owned render IR reports</DESC>
// <VERS>END OF VERSION: 0.1.0</VERS>
