// <FILE>crates/tui-vfx-compost/src/render/fnc_execute_effect_graph.rs</FILE> - <DESC>Execute native effect topology for one sampled cell</DESC>
// <VERS>VERSION: 0.1.0</VERS>
// <WCTX>Graph execution preserves sequence state while parallel branches read identical style/value snapshots before deterministic merge.</WCTX>
// <CLOG>0.1.0: INIT — execute sequence/parallel graph steps with graph-value publication and merge isolation.</CLOG>

use std::collections::{BTreeMap, BTreeSet};

use tui_vfx_contract::{CellChannel, GraphStep, GraphValueId, Value};
use tui_vfx_types::Style;

use crate::render::{
    EffectStack, EffectStage, RenderError, SampleContext, explicit_node_write_mask, is_node_active,
    publish_node_outputs, resolve_shader_phase_t,
};
use crate::runtime::RuntimeContext;
use crate::shaders::LinearGradientNode;

#[derive(Clone, Debug)]
pub(crate) struct EffectGraphResult {
    pub(crate) style: Style,
    context: RuntimeContext,
    channels: BTreeSet<CellChannel>,
    published_values: BTreeMap<GraphValueId, Value>,
}

#[allow(clippy::too_many_arguments)]
pub(crate) fn execute_effect_graph(
    stack: &EffectStack<'_>,
    sample: &SampleContext,
    base_style: Style,
    context: &RuntimeContext,
    local_x: u16,
    local_y: u16,
    width: u16,
    height: u16,
    screen_x: u16,
    screen_y: u16,
) -> Result<EffectGraphResult, RenderError> {
    match stack.topology() {
        Some(step) => execute_step(
            step, stack, sample, base_style, context, local_x, local_y, width, height, screen_x,
            screen_y,
        ),
        None => execute_stages(
            stack.ordered_stages(),
            sample,
            base_style,
            context,
            local_x,
            local_y,
            width,
            height,
            screen_x,
            screen_y,
        ),
    }
}

#[allow(clippy::too_many_arguments)]
fn execute_step(
    step: &GraphStep,
    stack: &EffectStack<'_>,
    sample: &SampleContext,
    base_style: Style,
    context: &RuntimeContext,
    local_x: u16,
    local_y: u16,
    width: u16,
    height: u16,
    screen_x: u16,
    screen_y: u16,
) -> Result<EffectGraphResult, RenderError> {
    match step {
        GraphStep::Node { node } => {
            let Some(stage) = stack.stage_for_node(node) else {
                return Err(RenderError::Unsupported(format!(
                    "native render references missing node `{}`",
                    node.as_str()
                )));
            };
            execute_stages(
                [stage],
                sample,
                base_style,
                context,
                local_x,
                local_y,
                width,
                height,
                screen_x,
                screen_y,
            )
        }
        GraphStep::Sequence { children } => execute_sequence(
            children, stack, sample, base_style, context, local_x, local_y, width, height,
            screen_x, screen_y,
        ),
        GraphStep::Parallel { children, .. } => execute_parallel(
            children, stack, sample, base_style, context, local_x, local_y, width, height,
            screen_x, screen_y,
        ),
    }
}

#[allow(clippy::too_many_arguments)]
fn execute_sequence(
    children: &[GraphStep],
    stack: &EffectStack<'_>,
    sample: &SampleContext,
    base_style: Style,
    context: &RuntimeContext,
    local_x: u16,
    local_y: u16,
    width: u16,
    height: u16,
    screen_x: u16,
    screen_y: u16,
) -> Result<EffectGraphResult, RenderError> {
    let mut result = EffectGraphResult::new(base_style, context.clone());
    for child in children {
        let child_result = execute_step(
            child,
            stack,
            sample,
            result.style,
            &result.context,
            local_x,
            local_y,
            width,
            height,
            screen_x,
            screen_y,
        )?;
        result.style = child_result.style;
        result.context = child_result.context;
        result.channels.extend(child_result.channels);
        result
            .published_values
            .extend(child_result.published_values);
    }
    result.channels = style_changed_channels(base_style, result.style);
    Ok(result)
}

#[allow(clippy::too_many_arguments)]
fn execute_parallel(
    children: &[GraphStep],
    stack: &EffectStack<'_>,
    sample: &SampleContext,
    base_style: Style,
    context: &RuntimeContext,
    local_x: u16,
    local_y: u16,
    width: u16,
    height: u16,
    screen_x: u16,
    screen_y: u16,
) -> Result<EffectGraphResult, RenderError> {
    let mut result = EffectGraphResult::new(base_style, context.clone());
    for child in children {
        let branch = execute_step(
            child, stack, sample, base_style, context, local_x, local_y, width, height, screen_x,
            screen_y,
        )?;
        result.style = merge_style_channels(result.style, branch.style, &branch.channels);
        for (id, value) in branch.published_values {
            result.context.set_graph_value(id.clone(), value.clone());
            result.published_values.insert(id, value);
        }
        result.channels.extend(branch.channels);
    }
    result.channels = style_changed_channels(base_style, result.style);
    Ok(result)
}

#[allow(clippy::too_many_arguments)]
fn execute_stages<'a>(
    stages: impl IntoIterator<Item = EffectStage<'a>>,
    sample: &SampleContext,
    base_style: Style,
    context: &RuntimeContext,
    local_x: u16,
    local_y: u16,
    width: u16,
    height: u16,
    screen_x: u16,
    screen_y: u16,
) -> Result<EffectGraphResult, RenderError> {
    let mut result = EffectGraphResult::new(base_style, context.clone());
    for stage in stages {
        if !is_node_active(stage.node(), sample) {
            continue;
        }
        let shader = supported_shader_stage(stage)?;
        let input_style = result.style;
        let rendered_style = shader.style_at(
            local_x,
            local_y,
            width,
            height,
            screen_x,
            screen_y,
            resolve_shader_phase_t(sample),
            input_style,
            &result.context,
        );
        let output_channels = effective_output_channels(
            &style_changed_channels(input_style, rendered_style),
            explicit_node_write_mask(stage.node()).as_ref(),
        );
        result.style = merge_style_channels(input_style, rendered_style, &output_channels);
        result
            .published_values
            .extend(publish_node_outputs(stage.node(), &mut result.context)?);
        result.channels.extend(output_channels);
    }
    Ok(result)
}

fn supported_shader_stage<'a>(
    stage: EffectStage<'a>,
) -> Result<LinearGradientNode<'a>, RenderError> {
    match stage.node().effect.as_str() {
        "shader.linearGradient" => LinearGradientNode::new(stage.node()),
        _ => Err(RenderError::Unsupported(format!(
            "effect stage `{}` uses unsupported {} family effect `{}`",
            stage.node_id().as_str(),
            stage.family().as_str(),
            stage.node().effect.as_str()
        ))),
    }
}

fn merge_style_channels(mut base: Style, branch: Style, channels: &BTreeSet<CellChannel>) -> Style {
    if channels.contains(&CellChannel::Foreground) {
        base.fg = branch.fg;
    }
    if channels.contains(&CellChannel::Background) {
        base.bg = branch.bg;
    }
    if channels.contains(&CellChannel::Modifiers) {
        base.mods = branch.mods;
    }
    base
}

fn style_changed_channels(base: Style, rendered: Style) -> BTreeSet<CellChannel> {
    let mut channels = BTreeSet::new();
    if base.fg != rendered.fg {
        channels.insert(CellChannel::Foreground);
    }
    if base.bg != rendered.bg {
        channels.insert(CellChannel::Background);
    }
    if base.mods != rendered.mods {
        channels.insert(CellChannel::Modifiers);
    }
    channels
}

fn effective_output_channels(
    changed_channels: &BTreeSet<CellChannel>,
    write_channels: Option<&BTreeSet<CellChannel>>,
) -> BTreeSet<CellChannel> {
    match write_channels {
        Some(mask) => changed_channels.intersection(mask).copied().collect(),
        None => changed_channels.clone(),
    }
}

impl EffectGraphResult {
    fn new(style: Style, context: RuntimeContext) -> Self {
        Self {
            style,
            context,
            channels: BTreeSet::new(),
            published_values: BTreeMap::new(),
        }
    }
}

// <FILE>crates/tui-vfx-compost/src/render/fnc_execute_effect_graph.rs</FILE> - <DESC>Execute native effect topology for one sampled cell</DESC>
// <VERS>END OF VERSION: 0.1.0</VERS>
