// <FILE>crates/tui-vfx-compost/src/validation/fnc_validate_scene_element_policies.rs</FILE> - <DESC>Validate scene element policies supported by compost rendering</DESC>
// <VERS>VERSION: 0.7.0</VERS>
// <WCTX>Accept simple scene-relative anchor placementRules whose resolved placement is the canonical render position; reject only sibling-relative anchors, absolute-rect rules, and motion-bearing rules that require native resolution.</WCTX>
// <CLOG>0.9.0: MINOR — accept shadow sourceRegion now that shadow geometry evaluates source scopes.
// 0.8.0: MINOR — accept graphBinding.timing now that render orchestration derives element-local graph samples.
// 0.7.0: MINOR — accept anchor-kind placementRule when sibling_layer and placement_motion are absent (resolved placement is authoritative).
// 0.6.0: MINOR — accept native visibility, warn clipping, overflow, and no-op scroll factor substrate.
// 0.5.1: PATCH — use capability-based unsupported-policy reasons instead of schedule language.
// 0.5.0: MINOR — accept copied and explicit role writes after role policy execution lands.
// 0.4.0: MINOR — accept skipTransparentEmpty after cell write policy substrate lands.
// 0.3.0: PATCH — reject element-local graph timing until timing substrate executes it.
// 0.2.0: PATCH — reject unsupported scene-element semantics instead of silently ignoring them.
// 0.1.0: INIT — add scene element policy validation.</CLOG>

use tui_vfx_contract::{
    CellWritePolicy, ClipPolicy, RecipeSceneElement, RoleWritePolicy, SceneElementOverflowPolicy,
    SceneElementPlacementRule,
};

use crate::LoadError;

pub(crate) fn validate_scene_element_policies(
    element: &RecipeSceneElement,
) -> Result<(), LoadError> {
    if let Some(rule) = &element.placement_rule {
        match rule {
            SceneElementPlacementRule::Anchor {
                sibling_layer,
                placement_motion,
                ..
            } => {
                if sibling_layer.is_some() {
                    return unsupported_policy(
                        element,
                        "placementRule.siblingLayer",
                        "sibling-relative anchors require native placement resolution",
                    );
                }
                if placement_motion.is_some() {
                    return unsupported_policy(
                        element,
                        "placementRule.placementMotion",
                        "placement motion requires motion-aware scene rendering",
                    );
                }
            }
            SceneElementPlacementRule::Absolute { .. } => {
                return unsupported_policy(
                    element,
                    "placementRule",
                    "absolute placement rectangles require native placement resolution",
                );
            }
        }
    }

    if element.placement_motion.is_some() {
        return unsupported_policy(
            element,
            "placementMotion",
            "placement motion requires motion-aware scene rendering",
        );
    }

    if let Some(surface) = &element.surface
        && surface.base_style.is_some()
    {
        return unsupported_policy(
            element,
            "surface.baseStyle",
            "base style requires typed native surface style rendering",
        );
    }

    match element.overflow {
        None
        | Some(SceneElementOverflowPolicy::Clip)
        | Some(SceneElementOverflowPolicy::Hide)
        | Some(SceneElementOverflowPolicy::Wrap) => {}
    }

    match element.clip_policy {
        ClipPolicy::Clip | ClipPolicy::Warn => {}
    }

    match element.cell_write_policy {
        CellWritePolicy::WriteCell | CellWritePolicy::SkipTransparentEmpty => {}
    }

    match &element.role_write_policy {
        RoleWritePolicy::PreserveDestination
        | RoleWritePolicy::CopySampledSource
        | RoleWritePolicy::SetExplicit { .. } => Ok(()),
    }
}

fn unsupported_policy(
    element: &RecipeSceneElement,
    policy: &str,
    reason: &str,
) -> Result<(), LoadError> {
    Err(LoadError::UnsupportedSceneElementPolicy {
        element_id: element.id.as_str().to_string(),
        policy: policy.to_string(),
        reason: reason.to_string(),
    })
}

// <FILE>crates/tui-vfx-compost/src/validation/fnc_validate_scene_element_policies.rs</FILE> - <DESC>Validate scene element policies supported by compost rendering</DESC>
// <VERS>END OF VERSION: 0.7.0</VERS>
