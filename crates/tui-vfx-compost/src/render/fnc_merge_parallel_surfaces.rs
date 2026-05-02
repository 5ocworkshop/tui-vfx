// <FILE>crates/tui-vfx-compost/src/render/fnc_merge_parallel_surfaces.rs</FILE> - <DESC>Parallel graph merge policy validation helpers</DESC>
// <VERS>VERSION: 0.2.0</VERS>
// <WCTX>Parallel topology is supported when requested merge policies have deterministic native conflict semantics.</WCTX>
// <CLOG>0.2.0: MINOR — detect same-channel and same-graph-value conflicts for error-on-conflict merge policies.
// 0.1.0: INIT — detect parallel graph steps for load-time rejection.</CLOG>

use std::collections::{BTreeMap, BTreeSet};

use tui_vfx_contract::{
    CellChannel, GraphStep, GraphValueId, GraphValueMergePolicy, NodeSpec, ParallelMergePolicy,
    Value, ValueSource,
};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum ParallelMergeConflict {
    Surface,
    GraphValue,
    DynamicSurfaceChannels,
}

pub(crate) fn parallel_merge_conflict(
    step: Option<&GraphStep>,
    nodes: &BTreeMap<tui_vfx_contract::NodeId, NodeSpec>,
) -> Option<ParallelMergeConflict> {
    match step {
        Some(GraphStep::Parallel {
            children,
            merge_policy,
            value_merge_policy,
        }) => parallel_step_conflict(children, *merge_policy, *value_merge_policy, nodes),
        Some(GraphStep::Sequence { children }) => children
            .iter()
            .find_map(|child| parallel_merge_conflict(Some(child), nodes)),
        Some(GraphStep::Node { .. }) | None => None,
    }
}

fn parallel_step_conflict(
    children: &[GraphStep],
    merge_policy: ParallelMergePolicy,
    value_merge_policy: GraphValueMergePolicy,
    nodes: &BTreeMap<tui_vfx_contract::NodeId, NodeSpec>,
) -> Option<ParallelMergeConflict> {
    if has_dynamic_step_channel(children, nodes) {
        return Some(ParallelMergeConflict::DynamicSurfaceChannels);
    }
    if merge_policy == ParallelMergePolicy::ErrorOnSameChannelConflict
        && has_duplicate_branch_channel(children, nodes)
    {
        return Some(ParallelMergeConflict::Surface);
    }
    if value_merge_policy == GraphValueMergePolicy::ErrorOnSameValueConflict
        && has_duplicate_branch_value(children, nodes)
    {
        return Some(ParallelMergeConflict::GraphValue);
    }
    children
        .iter()
        .find_map(|child| parallel_merge_conflict(Some(child), nodes))
}

fn has_dynamic_step_channel(
    children: &[GraphStep],
    nodes: &BTreeMap<tui_vfx_contract::NodeId, NodeSpec>,
) -> bool {
    children
        .iter()
        .any(|child| step_has_dynamic_channel(child, nodes))
}

fn step_has_dynamic_channel(
    step: &GraphStep,
    nodes: &BTreeMap<tui_vfx_contract::NodeId, NodeSpec>,
) -> bool {
    match step {
        GraphStep::Node { node } => nodes.get(node).is_some_and(node_has_dynamic_channel_target),
        GraphStep::Sequence { children } | GraphStep::Parallel { children, .. } => {
            has_dynamic_step_channel(children, nodes)
        }
    }
}

fn has_duplicate_branch_channel(
    children: &[GraphStep],
    nodes: &BTreeMap<tui_vfx_contract::NodeId, NodeSpec>,
) -> bool {
    let mut seen = BTreeSet::new();
    children.iter().any(|child| {
        collect_step_channels(child, nodes)
            .into_iter()
            .any(|channel| !seen.insert(channel))
    })
}

fn has_duplicate_branch_value(
    children: &[GraphStep],
    nodes: &BTreeMap<tui_vfx_contract::NodeId, NodeSpec>,
) -> bool {
    let mut seen = BTreeSet::new();
    children.iter().any(|child| {
        collect_step_values(child, nodes)
            .into_iter()
            .any(|value_id| !seen.insert(value_id))
    })
}

fn collect_step_channels(
    step: &GraphStep,
    nodes: &BTreeMap<tui_vfx_contract::NodeId, NodeSpec>,
) -> BTreeSet<CellChannel> {
    let mut channels = BTreeSet::new();
    match step {
        GraphStep::Node { node } => {
            if let Some(spec) = nodes.get(node) {
                channels.extend(static_parallel_surface_channels(spec));
            }
        }
        GraphStep::Sequence { children } | GraphStep::Parallel { children, .. } => {
            for child in children {
                channels.extend(collect_step_channels(child, nodes));
            }
        }
    }
    channels
}

fn collect_step_values(
    step: &GraphStep,
    nodes: &BTreeMap<tui_vfx_contract::NodeId, NodeSpec>,
) -> BTreeSet<GraphValueId> {
    let mut values = BTreeSet::new();
    match step {
        GraphStep::Node { node } => {
            if let Some(spec) = nodes.get(node) {
                values.extend(spec.outputs.keys().cloned());
            }
        }
        GraphStep::Sequence { children } | GraphStep::Parallel { children, .. } => {
            for child in children {
                values.extend(collect_step_values(child, nodes));
            }
        }
    }
    values
}

pub(crate) fn explicit_node_write_mask(node: &NodeSpec) -> Option<BTreeSet<CellChannel>> {
    (!node.write_channels.is_empty()).then(|| node.write_channels.iter().copied().collect())
}

pub(crate) fn static_parallel_surface_channels(node: &NodeSpec) -> BTreeSet<CellChannel> {
    if !node.write_channels.is_empty() {
        return node.write_channels.iter().copied().collect();
    }
    match node.effect.as_str() {
        "shader.linearGradient" => linear_gradient_channels(node),
        _ => BTreeSet::new(),
    }
}

fn linear_gradient_channels(node: &NodeSpec) -> BTreeSet<CellChannel> {
    match literal_enum_input(node, "channelTarget").as_deref() {
        Some("background") => BTreeSet::from([CellChannel::Background]),
        Some("both") => BTreeSet::from([CellChannel::Foreground, CellChannel::Background]),
        _ => BTreeSet::from([CellChannel::Foreground]),
    }
}

fn node_has_dynamic_channel_target(node: &NodeSpec) -> bool {
    node.write_channels.is_empty()
        && matches!(node.effect.as_str(), "shader.linearGradient")
        && match node
            .inputs
            .get(&tui_vfx_contract::EffectInputId::new("channelTarget"))
        {
            Some(ValueSource::Literal {
                value: Value::Enum(_),
            })
            | None => false,
            Some(_) => true,
        }
}

fn literal_enum_input(node: &NodeSpec, id: &str) -> Option<String> {
    match node.inputs.get(&tui_vfx_contract::EffectInputId::new(id)) {
        Some(ValueSource::Literal {
            value: Value::Enum(value),
        }) => Some(value.clone()),
        _ => None,
    }
}

// <FILE>crates/tui-vfx-compost/src/render/fnc_merge_parallel_surfaces.rs</FILE> - <DESC>Parallel graph merge policy validation helpers</DESC>
// <VERS>END OF VERSION: 0.2.0</VERS>
