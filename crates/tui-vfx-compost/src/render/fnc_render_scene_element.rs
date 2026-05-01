// <FILE>crates/tui-vfx-compost/src/render/fnc_render_scene_element.rs</FILE> - <DESC>Render one source-backed scene element</DESC>
// <VERS>VERSION: 0.2.1</VERS>
// <WCTX>Element orchestration receives materialized source surfaces from the Phase 2 source substrate before clipping.</WCTX>
// <CLOG>0.2.1: PATCH — keep render imports rustfmt-aligned after source dispatch wiring.
// 0.2.0: MINOR — route source materialization through descriptor dispatch seam.
// 0.1.0: INIT — add element render orchestration for scene composition.</CLOG>

use tui_vfx_contract::{RecipeDocument, RecipeScene, RecipeSceneElement};
use tui_vfx_types::{Grid, OwnedGrid, SemanticScene, Style};

use crate::render::{
    ElementClipBounds, RenderError, SampleContext, clip_element_bounds, collect_graph_step_nodes,
};
use crate::shaders::LinearGradientNode;
use crate::source::materialize_source;

pub(crate) fn render_scene_element(
    recipe: &RecipeDocument,
    scene: &RecipeScene,
    element: &RecipeSceneElement,
    sample: &SampleContext,
    destination: &mut SemanticScene,
) -> Result<Vec<String>, RenderError> {
    let source = recipe
        .sources
        .get(&element.source_instance)
        .ok_or_else(|| {
            RenderError::Unsupported(format!(
                "scene element `{}` references missing source `{}`",
                element.id.as_str(),
                element.source_instance.as_str()
            ))
        })?;
    let source_grid = materialize_source(source)?;
    let Some(bounds) = clip_element_bounds(
        element.placement,
        source_grid.width(),
        source_grid.height(),
        scene.width,
        scene.height,
    ) else {
        return Ok(Vec::new());
    };
    let shader_nodes = shader_nodes_for_element(recipe, element)?;
    let applied_effect_kinds = shader_nodes
        .iter()
        .map(|shader| shader.effect_id().to_string())
        .collect::<Vec<_>>();

    render_source_with_shaders(&source_grid, destination, bounds, sample, &shader_nodes);

    Ok(applied_effect_kinds)
}

fn shader_nodes_for_element<'a>(
    recipe: &'a RecipeDocument,
    element: &RecipeSceneElement,
) -> Result<Vec<LinearGradientNode<'a>>, RenderError> {
    let mut node_ids = Vec::new();
    let topology = element
        .graph_binding
        .as_ref()
        .and_then(|graph_binding| graph_binding.topology.as_ref())
        .or(recipe.graph.topology.as_ref());
    collect_graph_step_nodes(topology, &mut node_ids);
    if node_ids.is_empty() {
        node_ids.extend(recipe.graph.order.iter().cloned());
    }

    node_ids
        .iter()
        .map(|node_id| {
            let node = recipe.graph.nodes.get(node_id).ok_or_else(|| {
                RenderError::Unsupported(format!(
                    "native render references missing node `{}`",
                    node_id.as_str()
                ))
            })?;
            LinearGradientNode::new(node)
        })
        .collect()
}

fn render_source_with_shaders(
    source: &OwnedGrid,
    destination: &mut SemanticScene,
    bounds: ElementClipBounds,
    sample: &SampleContext,
    shaders: &[LinearGradientNode<'_>],
) {
    for visible_y in 0..bounds.height {
        for visible_x in 0..bounds.width {
            let local_x = bounds.local_x_start + visible_x;
            let local_y = bounds.local_y_start + visible_y;
            let dest_x = bounds.dest_x_start + visible_x;
            let dest_y = bounds.dest_y_start + visible_y;
            let Some(source_cell) = source.get(local_x, local_y).copied() else {
                continue;
            };
            let mut style = Style::new(source_cell.fg, source_cell.bg, source_cell.mods);
            for shader in shaders {
                style = shader.style_at(
                    local_x as u16,
                    local_y as u16,
                    source.width() as u16,
                    source.height() as u16,
                    dest_x as u16,
                    dest_y as u16,
                    sample.phase_t,
                    style,
                );
            }
            destination.grid_mut().set(
                dest_x,
                dest_y,
                source_cell
                    .with_fg(style.fg)
                    .with_bg(style.bg)
                    .with_mods(style.mods),
            );
        }
    }
}

// <FILE>crates/tui-vfx-compost/src/render/fnc_render_scene_element.rs</FILE> - <DESC>Render one source-backed scene element</DESC>
// <VERS>END OF VERSION: 0.2.1</VERS>
