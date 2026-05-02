// <FILE>crates/tui-vfx-compost/src/validation/orc_validate_render_contract.rs</FILE> - <DESC>Validate native direct-render contract at recipe load</DESC>
// <VERS>VERSION: 0.8.1</VERS>
// <WCTX>Validation dispatches supported native primitives, effect family slots, node timing, graph merge policy, and scene policies across order and topology references at load time.</WCTX>
// <CLOG>0.8.1: PATCH — validate reachable node graph-value publication contracts at load time.
// 0.8.0: MINOR — validate every node reachable from root or element-local topology.
// 0.7.1: PATCH — use capability-based unsupported graph/write policy reasons.
// 0.7.0: MINOR — allow activePhases because lifecycle gating executes during render.
// 0.6.0: MINOR — reject unsupported batched substrate semantics at load time.</CLOG>

use std::collections::BTreeSet;

use tui_vfx_contract::{NodeId, NodeOutputSource, NodeSpec, RecipeDocument};

use crate::LoadError;
use crate::render::{
    EffectFamily, ParallelMergeConflict, collect_graph_step_nodes, parallel_merge_conflict,
};
use crate::runtime::RuntimeContext;
use crate::validation::shaders::validate_shader_inputs;

use super::{validate_scene_element_policies, validate_source_inputs};

const UNSUPPORTED_EFFECT_FAMILY_REASON: &str =
    "effect stack has a native family slot, but this effect has no signed runtime implementation";
const SAME_CHANNEL_CONFLICT_REASON: &str = "parallel graph topology requested errorOnSameChannelConflict and branches write the same channel";
const SAME_VALUE_CONFLICT_REASON: &str = "parallel graph topology requested errorOnSameValueConflict and branches publish the same graph value";
const DYNAMIC_CHANNEL_TARGET_REASON: &str = "parallel graph topology requires literal shader channel targets for deterministic surface merge";
const UNSUPPORTED_NODE_WRITE_POLICY_REASON: &str =
    "node-local write policy precedence requires per-stage write policy execution";
const UNSUPPORTED_EFFECT_OUTPUT_REASON: &str =
    "effect-output publication requires native effect output capture";
const MISSING_OUTPUT_INPUT_REASON: &str =
    "input-sourced graph value publication requires an existing node input";

pub(crate) fn validate_render_contract(recipe: &RecipeDocument) -> Result<(), LoadError> {
    let runtime_context = RuntimeContext::load_time().with_graph_defaults(&recipe.graph);

    for (source_id, source) in &recipe.sources {
        validate_source_inputs(source_id, source, &runtime_context)?;
    }

    for scene in &recipe.scenes {
        for element in &scene.elements {
            validate_scene_element_policies(element)?;
            if let Some(conflict) = parallel_merge_conflict(
                element
                    .graph_binding
                    .as_ref()
                    .and_then(|binding| binding.topology.as_ref()),
                &recipe.graph.nodes,
            ) {
                return parallel_merge_error("scene.element.graphBinding.topology", conflict);
            }
        }
    }

    if let Some(conflict) =
        parallel_merge_conflict(recipe.graph.topology.as_ref(), &recipe.graph.nodes)
    {
        return parallel_merge_error("graph.topology", conflict);
    }

    for node_id in render_contract_node_ids(recipe) {
        let Some(node) = recipe.graph.nodes.get(node_id) else {
            continue;
        };
        if node
            .cell_write_policy
            .is_some_and(|policy| policy != tui_vfx_contract::CellWritePolicy::WriteCell)
        {
            return Err(LoadError::UnsupportedNodeWritePolicy {
                node_id: node_id.as_str().to_string(),
                effect: node.effect.as_str().to_string(),
                field: "cellWritePolicy".to_string(),
                reason: UNSUPPORTED_NODE_WRITE_POLICY_REASON.to_string(),
            });
        }
        if node.role_write_policy.as_ref().is_some_and(|policy| {
            !matches!(
                policy,
                tui_vfx_contract::RoleWritePolicy::PreserveDestination
            )
        }) {
            return Err(LoadError::UnsupportedNodeWritePolicy {
                node_id: node_id.as_str().to_string(),
                effect: node.effect.as_str().to_string(),
                field: "roleWritePolicy".to_string(),
                reason: UNSUPPORTED_NODE_WRITE_POLICY_REASON.to_string(),
            });
        }
        validate_node_outputs(node_id, node)?;
        match (
            EffectFamily::from_effect_id(node.effect.as_str()),
            node.effect.as_str(),
        ) {
            (EffectFamily::Shader, "shader.linearGradient") => {
                validate_shader_inputs(node_id, node, &runtime_context)?;
            }
            (family, effect) => {
                return Err(LoadError::UnsupportedEffectFamily {
                    node_id: node_id.as_str().to_string(),
                    effect: effect.to_string(),
                    family: family.as_str().to_string(),
                    reason: UNSUPPORTED_EFFECT_FAMILY_REASON.to_string(),
                });
            }
        }
    }
    Ok(())
}

fn validate_node_outputs(node_id: &NodeId, node: &NodeSpec) -> Result<(), LoadError> {
    for output in node.outputs.values() {
        match &output.output_source {
            NodeOutputSource::Input { id } => {
                if !node.inputs.contains_key(id) {
                    return Err(LoadError::UnsupportedInput {
                        node_id: node_id.as_str().to_string(),
                        effect: node.effect.as_str().to_string(),
                        input: id.as_str().to_string(),
                        reason: MISSING_OUTPUT_INPUT_REASON.to_string(),
                    });
                }
            }
            NodeOutputSource::EffectOutput { id } => {
                return Err(LoadError::UnsupportedInput {
                    node_id: node_id.as_str().to_string(),
                    effect: node.effect.as_str().to_string(),
                    input: id.as_str().to_string(),
                    reason: UNSUPPORTED_EFFECT_OUTPUT_REASON.to_string(),
                });
            }
        }
    }
    Ok(())
}

fn render_contract_node_ids(recipe: &RecipeDocument) -> BTreeSet<&NodeId> {
    let mut node_ids = BTreeSet::new();
    node_ids.extend(recipe.graph.order.iter());
    let mut topology_node_ids = Vec::new();
    collect_graph_step_nodes(recipe.graph.topology.as_ref(), &mut topology_node_ids);
    for scene in &recipe.scenes {
        for element in &scene.elements {
            collect_graph_step_nodes(
                element
                    .graph_binding
                    .as_ref()
                    .and_then(|binding| binding.topology.as_ref()),
                &mut topology_node_ids,
            );
        }
    }
    for node_id in &topology_node_ids {
        if let Some((key, _)) = recipe.graph.nodes.get_key_value(node_id) {
            node_ids.insert(key);
        }
    }
    node_ids
}

fn parallel_merge_error(root: &str, conflict: ParallelMergeConflict) -> Result<(), LoadError> {
    let (field, reason) = match conflict {
        ParallelMergeConflict::Surface => ("mergePolicy", SAME_CHANNEL_CONFLICT_REASON),
        ParallelMergeConflict::GraphValue => ("valueMergePolicy", SAME_VALUE_CONFLICT_REASON),
        ParallelMergeConflict::DynamicSurfaceChannels => {
            ("mergePolicy", DYNAMIC_CHANNEL_TARGET_REASON)
        }
    };
    Err(LoadError::UnsupportedGraphMergePolicy {
        field: format!("{root}.{field}"),
        reason: reason.to_string(),
    })
}

// <FILE>crates/tui-vfx-compost/src/validation/orc_validate_render_contract.rs</FILE> - <DESC>Validate native direct-render contract at recipe load</DESC>
// <VERS>END OF VERSION: 0.8.1</VERS>
