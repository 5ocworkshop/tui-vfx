// <FILE>crates/tui-vfx-compost/src/render/fnc_render_recipe.rs</FILE> - <DESC>Render a loaded v3.1 recipe through native compost modules</DESC>
// <VERS>VERSION: 0.1.1</VERS>
// <WCTX>Render directly from canonical RecipeDocument: source materialization, graph traversal, and native shader execution only.</WCTX>
// <CLOG>0.1.1: PATCH — read scene element sourceInstance after the contract naming audit.</CLOG>

use tui_vfx_contract::{RecipeDocument, RecipeSceneElement};
use tui_vfx_types::{Grid, OwnedGrid, RoleTag, SemanticScene, Style};

use crate::LoadedRecipe;
use crate::render::{Frame, RenderError, SampleContext, collect_graph_step_nodes};
use crate::shaders::LinearGradientNode;
use crate::source::source_grid_from_inputs;

/// Execute a load-validated canonical v3.1 recipe through compost.
pub fn render_recipe(loaded: &LoadedRecipe, sample: &SampleContext) -> Result<Frame, RenderError> {
    let recipe = loaded.recipe();
    let scene = recipe
        .scenes
        .first()
        .ok_or_else(|| RenderError::Unsupported("recipe has no scene to render".to_string()))?;
    let element = scene
        .elements
        .first()
        .ok_or_else(|| RenderError::Unsupported("recipe scene has no element".to_string()))?;
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

    let source_grid = source_grid_from_inputs(&source.inputs)?;
    let mut destination = SemanticScene::from_grid_with_default_role(
        OwnedGrid::new(scene.width, scene.height),
        RoleTag::Background,
    );
    let shader_nodes = shader_nodes_for_element(recipe, element)?;
    let applied_effect_kinds = shader_nodes
        .iter()
        .map(|shader| shader.effect_id().to_string())
        .collect::<Vec<_>>();

    render_source_with_shaders(
        &source_grid,
        &mut destination,
        element,
        sample,
        &shader_nodes,
    );

    Ok(Frame {
        recipe_id: recipe.id.as_str().to_string(),
        width: scene.width,
        height: scene.height,
        grid: destination,
        applied_effect_kinds,
    })
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
    element: &RecipeSceneElement,
    sample: &SampleContext,
    shaders: &[LinearGradientNode<'_>],
) {
    let offset_x = element.placement.x.max(0) as usize;
    let offset_y = element.placement.y.max(0) as usize;
    for local_y in 0..source.height() {
        for local_x in 0..source.width() {
            let dest_x = offset_x + local_x;
            let dest_y = offset_y + local_y;
            if dest_x >= destination.grid().width() || dest_y >= destination.grid().height() {
                continue;
            }
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
            destination
                .roles_mut()
                .set((dest_x as u16, dest_y as u16), RoleTag::Text);
        }
    }
}

// <FILE>crates/tui-vfx-compost/src/render/fnc_render_recipe.rs</FILE> - <DESC>Render a loaded v3.1 recipe through native compost modules</DESC>
// <VERS>END OF VERSION: 0.1.1</VERS>
