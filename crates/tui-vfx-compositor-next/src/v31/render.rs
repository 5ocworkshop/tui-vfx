// <FILE>crates/tui-vfx-compositor-next/src/v31/render.rs</FILE> - <DESC>Direct canonical v3.1 recipe rendering</DESC>
// <VERS>VERSION: 0.1.0</VERS>
// <WCTX>Render load-validated canonical v3.1 recipes through compositor-next without transition-seam code.</WCTX>
// <CLOG>0.1.0: INIT — add direct shader.linearGradient rendering.</CLOG>

use std::collections::{BTreeMap, BTreeSet};

use crate::pipeline::{CompositionSpec, ShaderLayerSpec, render_pipeline_with_spec};
use tui_vfx_contract::{
    EffectInputId, GraphStep, NodeId, NodeSpec, RecipeDocument, RecipeSceneElement, SourceInputId,
    Value, ValueSource,
};
use tui_vfx_style::models::{
    ColorSpace, Gradient, LinearGradientApplyTo, LinearGradientShader, SpatialShaderType,
    StyleRegion,
};
use tui_vfx_types::{Cell, Color, Grid, OwnedGrid, RoleMap, RoleTag, SemanticScene};

use super::load::LoadedV31Recipe;

/// Explicit sample context for direct v3.1 compositor-next rendering.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct V31SampleContext {
    /// Normalized lifecycle/sample progress for phase-driven effects.
    pub phase_t: f64,
}

impl Default for V31SampleContext {
    fn default() -> Self {
        Self { phase_t: 0.0 }
    }
}

/// Rendered frame from the direct v3.1 compositor-next path.
#[derive(Clone, Debug)]
pub struct V31Frame {
    /// Canonical recipe id rendered into this frame.
    pub recipe_id: String,
    /// Frame width in cells.
    pub width: usize,
    /// Frame height in cells.
    pub height: usize,
    /// Semantic scene produced by compositor-next.
    pub grid: SemanticScene,
    /// Non-fatal direct-render diagnostics.
    pub diagnostics: Vec<String>,
    /// Effect descriptor ids applied by this direct v3.1 render.
    pub applied_effect_kinds: Vec<String>,
}

/// Error returned by direct v3.1 compositor-next rendering.
#[derive(Clone, Debug, PartialEq)]
pub enum V31RenderError {
    /// The direct v3.1 lane does not yet support the requested shape.
    Unsupported(String),
}

/// Render a load-validated canonical v3.1 recipe through compositor-next directly.
pub fn render_v31_recipe(
    loaded: &LoadedV31Recipe,
    sample: &V31SampleContext,
) -> Result<V31Frame, V31RenderError> {
    let recipe = loaded.recipe();
    let scene = recipe
        .scenes
        .first()
        .ok_or_else(|| V31RenderError::Unsupported("Recipe has no scene to render.".to_string()))?;
    let element = scene.elements.first().ok_or_else(|| {
        V31RenderError::Unsupported("Recipe scene has no source element to render.".to_string())
    })?;
    let source = recipe.sources.get(&element.source).ok_or_else(|| {
        V31RenderError::Unsupported(format!(
            "Recipe scene element `{}` references missing source `{}`.",
            element.id.as_str(),
            element.source.as_str()
        ))
    })?;
    let source_grid = source_grid_from_inputs(&source.inputs, scene.width, scene.height)?;
    let source_roles = RoleMap::new_with_default(
        source_grid.width() as u16,
        source_grid.height() as u16,
        RoleTag::Text,
    );
    let mut destination = SemanticScene::from_grid_with_default_role(
        OwnedGrid::new(scene.width, scene.height),
        RoleTag::Background,
    );
    let (spec, applied_effect_kinds) = composition_spec_for_element(recipe, element, sample)?;

    render_pipeline_with_spec(
        &source_grid,
        &source_roles,
        &mut destination,
        source_grid.width(),
        source_grid.height(),
        element.placement.x.max(0) as usize,
        element.placement.y.max(0) as usize,
        &spec,
        None,
    );

    Ok(V31Frame {
        recipe_id: recipe.id.as_str().to_string(),
        width: scene.width,
        height: scene.height,
        grid: destination,
        diagnostics: vec![],
        applied_effect_kinds,
    })
}

fn composition_spec_for_element(
    recipe: &RecipeDocument,
    element: &RecipeSceneElement,
    sample: &V31SampleContext,
) -> Result<(CompositionSpec, Vec<String>), V31RenderError> {
    let mut node_ids = Vec::new();
    let topology = element
        .pipeline
        .as_ref()
        .and_then(|pipeline| pipeline.topology.as_ref())
        .or(recipe.graph.topology.as_ref());
    collect_graph_step_nodes(topology, &mut node_ids);
    if node_ids.is_empty() {
        node_ids.extend(recipe.graph.order.iter().cloned());
    }

    let mut spec = CompositionSpec {
        t: sample.phase_t,
        ..CompositionSpec::default()
    };
    let mut applied_effect_kinds = Vec::new();
    for node_id in node_ids {
        let node = recipe.graph.nodes.get(&node_id).ok_or_else(|| {
            V31RenderError::Unsupported(format!(
                "Direct v3.1 rendering references missing node `{}`.",
                node_id.as_str()
            ))
        })?;
        append_node_to_composition(node, &mut spec, &mut applied_effect_kinds)?;
    }
    Ok((spec, applied_effect_kinds))
}

fn collect_graph_step_nodes(step: Option<&GraphStep>, node_ids: &mut Vec<NodeId>) {
    let Some(step) = step else { return };
    match step {
        GraphStep::Node { node } => node_ids.push(node.clone()),
        GraphStep::Sequence { children } | GraphStep::Parallel { children, .. } => {
            let mut seen = BTreeSet::new();
            for child in children {
                collect_graph_step_nodes(Some(child), node_ids);
            }
            node_ids.retain(|node| seen.insert(node.clone()));
        }
    }
}

fn append_node_to_composition(
    node: &NodeSpec,
    spec: &mut CompositionSpec,
    applied_effect_kinds: &mut Vec<String>,
) -> Result<(), V31RenderError> {
    match node.effect.as_str() {
        "shader.linearGradient" => {
            spec.shader_layers.push(ShaderLayerSpec {
                shader: SpatialShaderType::LinearGradient(linear_gradient_shader(node)?),
                region: StyleRegion::All,
            });
            applied_effect_kinds.push(node.effect.as_str().to_string());
            Ok(())
        }
        other => Err(V31RenderError::Unsupported(format!(
            "Direct v3.1 rendering does not support effect `{other}`."
        ))),
    }
}

fn linear_gradient_shader(node: &NodeSpec) -> Result<LinearGradientShader, V31RenderError> {
    Ok(LinearGradientShader {
        gradient: gradient_input(node)?,
        angle_deg: number_input(node, "angleDeg") as f32,
        apply_to: apply_to_input(node, "applyTo")?,
        intensity: number_input(node, "intensity") as f32,
    })
}

fn gradient_input(node: &NodeSpec) -> Result<Gradient, V31RenderError> {
    if let Some(Value::Gradient(gradient)) = optional_literal_value(node, "gradient") {
        return Ok(Gradient {
            stops: gradient
                .stops
                .iter()
                .map(|stop| (stop.position as f32, stop.color))
                .collect(),
            space: color_space_name(&gradient.space)?,
        });
    }

    let start = color_input(node, "startColor")?;
    let end = color_input(node, "endColor")?;
    Ok(Gradient {
        stops: vec![(0.0, start), (1.0, end)],
        space: color_space_input(node, "colorSpace")?,
    })
}

fn source_grid_from_inputs(
    inputs: &BTreeMap<SourceInputId, ValueSource>,
    fallback_width: usize,
    fallback_height: usize,
) -> Result<OwnedGrid, V31RenderError> {
    let text = inputs
        .get(&SourceInputId::new("message"))
        .or_else(|| inputs.get(&SourceInputId::new("text")))
        .and_then(literal_text)
        .ok_or_else(|| {
            V31RenderError::Unsupported(
                "Direct v3.1 rendering requires a literal text/message source.".to_string(),
            )
        })?;
    let width = inputs
        .get(&SourceInputId::new("width"))
        .and_then(literal_number)
        .map(|value| value.max(1.0) as usize)
        .unwrap_or(fallback_width);
    let height = inputs
        .get(&SourceInputId::new("height"))
        .and_then(literal_number)
        .map(|value| value.max(1.0) as usize)
        .unwrap_or(fallback_height);
    Ok(source_grid_from_text(text, width, height))
}

fn literal_text(source: &ValueSource) -> Option<&str> {
    match source {
        ValueSource::Literal {
            value: Value::Text(value) | Value::String(value),
        } => Some(value.as_str()),
        _ => None,
    }
}

fn literal_number(source: &ValueSource) -> Option<f64> {
    match source {
        ValueSource::Literal { value } => value.as_range_number(),
        _ => None,
    }
}

fn optional_literal_value<'a>(node: &'a NodeSpec, id: &str) -> Option<&'a Value> {
    match node.inputs.get(&EffectInputId::new(id)) {
        Some(ValueSource::Literal { value }) => Some(value),
        _ => None,
    }
}

fn literal_value<'a>(node: &'a NodeSpec, id: &str) -> Result<&'a Value, V31RenderError> {
    match node.inputs.get(&EffectInputId::new(id)) {
        Some(ValueSource::Literal { value }) => Ok(value),
        Some(_) => Err(V31RenderError::Unsupported(format!(
            "Direct v3.1 rendering requires literal input `{id}` for `{}`.",
            node.effect.as_str()
        ))),
        None => Err(V31RenderError::Unsupported(format!(
            "Direct v3.1 rendering requires input `{id}` for `{}`.",
            node.effect.as_str()
        ))),
    }
}

fn color_input(node: &NodeSpec, id: &str) -> Result<Color, V31RenderError> {
    match literal_value(node, id)? {
        Value::Color(value) => Ok(*value),
        value => Err(V31RenderError::Unsupported(format!(
            "Direct v3.1 rendering expected color input `{id}` but found `{:?}`.",
            value.kind()
        ))),
    }
}

fn number_input(node: &NodeSpec, id: &str) -> f64 {
    literal_value(node, id)
        .ok()
        .and_then(Value::as_range_number)
        .expect("direct v3.1 load validates required numeric literals")
}

fn color_space_input(node: &NodeSpec, id: &str) -> Result<ColorSpace, V31RenderError> {
    color_space_name(literal_value(node, id)?.as_enum_value().ok_or_else(|| {
        V31RenderError::Unsupported(format!(
            "Direct v3.1 rendering expected enum input `{id}` for shader.linearGradient."
        ))
    })?)
}

fn color_space_name(value: &str) -> Result<ColorSpace, V31RenderError> {
    match value {
        "rgb" => Ok(ColorSpace::Rgb),
        "hct" => Ok(ColorSpace::Hct),
        other => Err(V31RenderError::Unsupported(format!(
            "shader.linearGradient colorSpace `{other}` is not supported by direct v3.1 rendering."
        ))),
    }
}

fn apply_to_input(node: &NodeSpec, id: &str) -> Result<LinearGradientApplyTo, V31RenderError> {
    match literal_value(node, id)?.as_enum_value() {
        Some("foreground") => Ok(LinearGradientApplyTo::Foreground),
        Some("background") => Ok(LinearGradientApplyTo::Background),
        Some("both") => Ok(LinearGradientApplyTo::Both),
        Some(value) => Err(V31RenderError::Unsupported(format!(
            "shader.linearGradient applyTo `{value}` is not supported by direct v3.1 rendering."
        ))),
        None => Err(V31RenderError::Unsupported(format!(
            "Direct v3.1 rendering expected enum input `{id}` for shader.linearGradient."
        ))),
    }
}

fn source_grid_from_text(text: &str, width: usize, height: usize) -> OwnedGrid {
    let mut grid = OwnedGrid::new(width, height);
    let lines: Vec<&str> = text.lines().collect();
    let lines = if lines.is_empty() { vec![text] } else { lines };
    for (y, line) in lines.into_iter().take(height).enumerate() {
        for (x, ch) in line.chars().take(width).enumerate() {
            grid.set(
                x,
                y,
                Cell {
                    ch,
                    fg: Color::WHITE,
                    bg: Color::BLACK,
                    ..Default::default()
                },
            );
        }
    }
    grid
}

// <FILE>crates/tui-vfx-compositor-next/src/v31/render.rs</FILE> - <DESC>Direct canonical v3.1 recipe rendering</DESC>
// <VERS>END OF VERSION: 0.1.0</VERS>
