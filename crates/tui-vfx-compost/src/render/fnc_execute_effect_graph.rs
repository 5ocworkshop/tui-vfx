// <FILE>crates/tui-vfx-compost/src/render/fnc_execute_effect_graph.rs</FILE> - <DESC>Execute native effect topology for one sampled cell</DESC>
// <VERS>VERSION: 0.6.0</VERS>
// <WCTX>Graph execution preserves sequence state, node scopes, node-local write policies, parallel branch snapshots, and per-cell trace evidence before deterministic merge.</WCTX>
// <CLOG>0.6.0: MINOR — allocate topology synthetic trace indices with deterministic pre-order identity.
// 0.5.0: MINOR — use stable authored and topology-local trace indices.
// 0.4.0: MINOR — emit trace evidence from actual node execution decisions.
// 0.3.0: MINOR — honor canonical node scopes during graph execution.
// 0.2.0: MINOR — carry non-default node-local write policies with executed stage output.
// 0.1.0: INIT — execute sequence/parallel graph steps with graph-value publication and merge isolation.</CLOG>

use std::collections::{BTreeMap, BTreeSet};

use tui_vfx_contract::{
    CellChannel, CellWritePolicy, CoordinateSpace, GraphStep, GraphValueId, RoleSpace,
    RoleWritePolicy, ScopeEvalInput, Value,
};
use tui_vfx_types::Style;

use crate::render::{
    CellStageTrace, EffectStack, EffectStage, RenderError, RenderSkipReason, RenderStageKind,
    SampleContext, explicit_node_write_mask, is_node_active, parallel_cell_trace,
    publish_node_outputs, resolve_shader_phase_t,
};
use crate::runtime::RuntimeContext;
use crate::shaders::LinearGradientNode;

#[derive(Clone, Debug)]
pub(crate) struct EffectGraphResult {
    pub(crate) style: Style,
    pub(crate) cell_write_policy: Option<CellWritePolicy>,
    pub(crate) role_write_policy: Option<RoleWritePolicy>,
    context: RuntimeContext,
    channels: BTreeSet<CellChannel>,
    published_values: BTreeMap<GraphValueId, Value>,
    pub(crate) stage_traces: Vec<CellStageTrace>,
}

#[allow(clippy::too_many_arguments)]
pub(crate) fn execute_effect_graph(
    stack: &EffectStack<'_>,
    sample: &SampleContext,
    base_style: Style,
    context: &RuntimeContext,
    scope_input: &ScopeEvalInput,
    local_x: u16,
    local_y: u16,
    width: u16,
    height: u16,
    screen_x: u16,
    screen_y: u16,
) -> Result<EffectGraphResult, RenderError> {
    match stack.topology() {
        Some(step) => {
            let mut next_synthetic_stage_index = stack.stage_count();
            execute_step(
                step,
                stack,
                sample,
                base_style,
                context,
                scope_input,
                local_x,
                local_y,
                width,
                height,
                screen_x,
                screen_y,
                &mut next_synthetic_stage_index,
            )
        }
        None => execute_stages(
            stack.indexed_stages(),
            sample,
            base_style,
            context,
            scope_input,
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
    scope_input: &ScopeEvalInput,
    local_x: u16,
    local_y: u16,
    width: u16,
    height: u16,
    screen_x: u16,
    screen_y: u16,
    next_synthetic_stage_index: &mut usize,
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
                [(stack.stage_index_for_node(node).unwrap_or(0), stage)],
                sample,
                base_style,
                context,
                scope_input,
                local_x,
                local_y,
                width,
                height,
                screen_x,
                screen_y,
            )
        }
        GraphStep::Sequence { children } => execute_sequence(
            children,
            stack,
            sample,
            base_style,
            context,
            scope_input,
            local_x,
            local_y,
            width,
            height,
            screen_x,
            screen_y,
            next_synthetic_stage_index,
        ),
        GraphStep::Parallel { children, .. } => execute_parallel(
            children,
            stack,
            sample,
            base_style,
            context,
            scope_input,
            local_x,
            local_y,
            width,
            height,
            screen_x,
            screen_y,
            next_synthetic_stage_index,
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
    scope_input: &ScopeEvalInput,
    local_x: u16,
    local_y: u16,
    width: u16,
    height: u16,
    screen_x: u16,
    screen_y: u16,
    next_synthetic_stage_index: &mut usize,
) -> Result<EffectGraphResult, RenderError> {
    let mut result = EffectGraphResult::new(base_style, context.clone());
    for child in children {
        let child_result = execute_step(
            child,
            stack,
            sample,
            result.style,
            &result.context,
            scope_input,
            local_x,
            local_y,
            width,
            height,
            screen_x,
            screen_y,
            next_synthetic_stage_index,
        )?;
        result.style = child_result.style;
        result.context = child_result.context;
        result.select_write_policies(
            child_result.cell_write_policy,
            child_result.role_write_policy,
        );
        result.channels.extend(child_result.channels);
        result
            .published_values
            .extend(child_result.published_values);
        result.stage_traces.extend(child_result.stage_traces);
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
    scope_input: &ScopeEvalInput,
    local_x: u16,
    local_y: u16,
    width: u16,
    height: u16,
    screen_x: u16,
    screen_y: u16,
    next_synthetic_stage_index: &mut usize,
) -> Result<EffectGraphResult, RenderError> {
    let mut result = EffectGraphResult::new(base_style, context.clone());
    let parallel_stage_index = *next_synthetic_stage_index;
    *next_synthetic_stage_index += 1;
    for child in children {
        let branch = execute_step(
            child,
            stack,
            sample,
            base_style,
            context,
            scope_input,
            local_x,
            local_y,
            width,
            height,
            screen_x,
            screen_y,
            next_synthetic_stage_index,
        )?;
        result.style = merge_style_channels(result.style, branch.style, &branch.channels);
        result.select_write_policies(branch.cell_write_policy, branch.role_write_policy);
        for (id, value) in branch.published_values {
            result.context.set_graph_value(id.clone(), value.clone());
            result.published_values.insert(id, value);
        }
        result.channels.extend(branch.channels);
        result.stage_traces.extend(branch.stage_traces);
    }
    result
        .stage_traces
        .push(parallel_cell_trace(parallel_stage_index));
    result.channels = style_changed_channels(base_style, result.style);
    Ok(result)
}

#[allow(clippy::too_many_arguments)]
fn execute_stages<'a>(
    stages: impl IntoIterator<Item = (usize, EffectStage<'a>)>,
    sample: &SampleContext,
    base_style: Style,
    context: &RuntimeContext,
    scope_input: &ScopeEvalInput,
    local_x: u16,
    local_y: u16,
    width: u16,
    height: u16,
    screen_x: u16,
    screen_y: u16,
) -> Result<EffectGraphResult, RenderError> {
    let mut result = EffectGraphResult::new(base_style, context.clone());
    for (stage_index, stage) in stages {
        if !is_node_active(stage.node(), sample) {
            result.stage_traces.push(CellStageTrace::skipped(
                stage_index,
                stage_family_kind(stage),
                stage.node().effect.as_str(),
                RenderSkipReason::InactiveLifecycle,
            ));
            continue;
        }
        if stage.node().scope.as_ref().is_some_and(|scope| {
            !scope.matches(
                scope_input,
                CoordinateSpace::default(),
                RoleSpace::default(),
            )
        }) {
            result.stage_traces.push(CellStageTrace::skipped(
                stage_index,
                stage_family_kind(stage),
                stage.node().effect.as_str(),
                RenderSkipReason::ScopeMatchedZeroCells,
            ));
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
        result.select_write_policies(
            stage
                .node()
                .cell_write_policy
                .filter(|policy| *policy != CellWritePolicy::WriteCell),
            stage
                .node()
                .role_write_policy
                .as_ref()
                .filter(|policy| !matches!(policy, RoleWritePolicy::PreserveDestination))
                .cloned(),
        );
        result
            .published_values
            .extend(publish_node_outputs(stage.node(), &mut result.context)?);
        result.channels.extend(output_channels);
        result.stage_traces.push(CellStageTrace::finished(
            stage_index,
            stage_family_kind(stage),
            stage.node().effect.as_str(),
        ));
    }
    Ok(result)
}

fn stage_family_kind(stage: EffectStage<'_>) -> RenderStageKind {
    match stage.family() {
        crate::render::EffectFamily::Content => RenderStageKind::Content,
        crate::render::EffectFamily::Style => RenderStageKind::Style,
        crate::render::EffectFamily::Shader => RenderStageKind::Shader,
        crate::render::EffectFamily::Filter => RenderStageKind::Filter,
        crate::render::EffectFamily::Mask => RenderStageKind::Mask,
        crate::render::EffectFamily::Sampler => RenderStageKind::Sampler,
        crate::render::EffectFamily::Unknown => RenderStageKind::Unknown,
    }
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
            cell_write_policy: None,
            role_write_policy: None,
            context,
            channels: BTreeSet::new(),
            published_values: BTreeMap::new(),
            stage_traces: Vec::new(),
        }
    }

    fn select_write_policies(
        &mut self,
        cell_write_policy: Option<CellWritePolicy>,
        role_write_policy: Option<RoleWritePolicy>,
    ) {
        if cell_write_policy.is_some() {
            self.cell_write_policy = cell_write_policy;
        }
        if role_write_policy.is_some() {
            self.role_write_policy = role_write_policy;
        }
    }
}

// <FILE>crates/tui-vfx-compost/src/render/fnc_execute_effect_graph.rs</FILE> - <DESC>Execute native effect topology for one sampled cell</DESC>
// <VERS>END OF VERSION: 0.6.0</VERS>
