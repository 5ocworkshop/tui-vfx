// <FILE>crates/tui-vfx-compost/src/validation/fnc_validate_scene_element_policies.rs</FILE> - <DESC>Validate scene element policies supported by compost rendering</DESC>
// <VERS>VERSION: 0.5.1</VERS>
// <WCTX>Element policy validation supports writeCell/skipTransparentEmpty cell writes and native role writes.</WCTX>
// <CLOG>0.5.1: PATCH — use capability-based unsupported-policy reasons instead of schedule language.
// 0.5.0: MINOR — accept copied and explicit role writes after role policy execution lands.
// 0.4.0: MINOR — accept skipTransparentEmpty after cell write policy substrate lands.
// 0.3.0: PATCH — reject element-local graph timing until timing substrate executes it.
// 0.2.0: PATCH — reject unsupported scene-element semantics instead of silently ignoring them.
// 0.1.0: INIT — add scene element policy validation.</CLOG>

use tui_vfx_contract::{
    CellWritePolicy, ClipPolicy, RecipeSceneElement, RoleWritePolicy, SceneElementOverflowPolicy,
    SceneElementVisibility,
};

use crate::LoadError;

pub(crate) fn validate_scene_element_policies(
    element: &RecipeSceneElement,
) -> Result<(), LoadError> {
    if element
        .graph_binding
        .as_ref()
        .and_then(|binding| binding.timing.as_ref())
        .is_some()
    {
        return unsupported_policy(
            element,
            "graphBinding.timing",
            "element-local graph timing requires native timing resolution",
        );
    }

    if element.placement_rule.is_some() {
        return unsupported_policy(
            element,
            "placementRule",
            "declarative placement rules require native placement resolution",
        );
    }

    match &element.visibility {
        None | Some(SceneElementVisibility::Always) => {}
        Some(SceneElementVisibility::Phase { .. }) => {
            return unsupported_policy(
                element,
                "visibility",
                "phase visibility requires lifecycle-aware scene rendering",
            );
        }
        Some(SceneElementVisibility::Predicate { .. }) => {
            return unsupported_policy(
                element,
                "visibility",
                "predicate visibility requires runtime resolver integration",
            );
        }
    }

    if element.surface.is_some() {
        return unsupported_policy(
            element,
            "surface",
            "element surface styling and shadow semantics require native surface rendering",
        );
    }

    match element.overflow {
        None | Some(SceneElementOverflowPolicy::Clip) => {}
        Some(SceneElementOverflowPolicy::Hide) => {
            return unsupported_policy(
                element,
                "overflow",
                "hide overflow requires native element overflow handling",
            );
        }
        Some(SceneElementOverflowPolicy::Wrap) => {
            return unsupported_policy(
                element,
                "overflow",
                "wrap overflow requires native element overflow handling",
            );
        }
    }

    if element.placement_motion.is_some() {
        return unsupported_policy(
            element,
            "placementMotion",
            "placement motion requires motion-aware scene rendering",
        );
    }

    if element.scroll_factor.is_some() {
        return unsupported_policy(
            element,
            "scrollFactor",
            "scene scroll response requires camera/scroll runtime resolution",
        );
    }

    if element.clip_policy != ClipPolicy::Clip {
        return unsupported_policy(
            element,
            "clipPolicy",
            "warn clipping requires element-aware diagnostics",
        );
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
// <VERS>END OF VERSION: 0.5.1</VERS>
