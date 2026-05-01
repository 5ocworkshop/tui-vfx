// <FILE>crates/tui-vfx-compost/src/validation/orc_validate_render_contract.rs</FILE> - <DESC>Validate native direct-render contract at recipe load</DESC>
// <VERS>VERSION: 0.6.0</VERS>
// <WCTX>Validation dispatches supported native primitives, effect family slots, node timing, graph merge policy, and scene policies at load time.</WCTX>
// <CLOG>0.6.0: MINOR — reject unsupported batched substrate semantics at load time.</CLOG>

use tui_vfx_contract::RecipeDocument;

use crate::LoadError;
use crate::render::{EffectFamily, has_parallel_surface_merge, is_node_active};
use crate::validation::shaders::validate_shader_inputs;

use super::{validate_scene_element_policies, validate_source_inputs};

const UNSUPPORTED_EFFECT_FAMILY_REASON: &str = "effect stack has a native family slot, but this effect has no signed runtime implementation yet";
const UNSUPPORTED_PARALLEL_MERGE_REASON: &str = "parallel graph topology is deferred until native surface and graph-value merge semantics are implemented";
const UNSUPPORTED_NODE_WRITE_POLICY_REASON: &str = "node-local write policy precedence is deferred until per-stage write policy execution is implemented";

pub(crate) fn validate_render_contract(recipe: &RecipeDocument) -> Result<(), LoadError> {
    for (source_id, source) in &recipe.sources {
        validate_source_inputs(source_id, source)?;
    }

    for scene in &recipe.scenes {
        for element in &scene.elements {
            validate_scene_element_policies(element)?;
            if has_parallel_surface_merge(
                element
                    .graph_binding
                    .as_ref()
                    .and_then(|binding| binding.topology.as_ref()),
            ) {
                return Err(LoadError::UnsupportedGraphMergePolicy {
                    field: "scene.element.graphBinding.topology.mergePolicy".to_string(),
                    reason: UNSUPPORTED_PARALLEL_MERGE_REASON.to_string(),
                });
            }
        }
    }

    if has_parallel_surface_merge(recipe.graph.topology.as_ref()) {
        return Err(LoadError::UnsupportedGraphMergePolicy {
            field: "graph.topology.mergePolicy".to_string(),
            reason: UNSUPPORTED_PARALLEL_MERGE_REASON.to_string(),
        });
    }

    for node_id in &recipe.graph.order {
        let Some(node) = recipe.graph.nodes.get(node_id) else {
            continue;
        };
        if !is_node_active(node) {
            return Err(LoadError::UnsupportedNodeTiming {
                node_id: node_id.as_str().to_string(),
                effect: node.effect.as_str().to_string(),
                field: "activePhases".to_string(),
                reason: "timing substrate does not yet carry lifecycle phase state".to_string(),
            });
        }
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
        match (
            EffectFamily::from_effect_id(node.effect.as_str()),
            node.effect.as_str(),
        ) {
            (EffectFamily::Shader, "shader.linearGradient") => {
                validate_shader_inputs(node_id, node)?;
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

// <FILE>crates/tui-vfx-compost/src/validation/orc_validate_render_contract.rs</FILE> - <DESC>Validate native direct-render contract at recipe load</DESC>
// <VERS>END OF VERSION: 0.6.0</VERS>
