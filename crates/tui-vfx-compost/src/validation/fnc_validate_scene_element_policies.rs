// <FILE>crates/tui-vfx-compost/src/validation/fnc_validate_scene_element_policies.rs</FILE> - <DESC>Validate scene element policies supported by compost rendering</DESC>
// <VERS>VERSION: 0.4.0</VERS>
// <WCTX>Element policy validation supports writeCell/skipTransparentEmpty cell writes and preserved destination roles.</WCTX>
// <CLOG>0.4.0: MINOR — accept skipTransparentEmpty after cell write policy substrate lands.
// 0.3.0: PATCH — reject element-local graph timing until timing substrate executes it.
// 0.2.0: PATCH — reject deferred scene-element semantics instead of silently ignoring them.
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
            "element-local graph timing is deferred until timing substrate",
        );
    }

    if element.placement_rule.is_some() {
        return unsupported_policy(
            element,
            "placementRule",
            "declarative placement rules are deferred until placement resolution substrate",
        );
    }

    match &element.visibility {
        None | Some(SceneElementVisibility::Always) => {}
        Some(SceneElementVisibility::Phase { .. }) => {
            return unsupported_policy(
                element,
                "visibility",
                "phase visibility is deferred until lifecycle-aware scene rendering",
            );
        }
        Some(SceneElementVisibility::Predicate { .. }) => {
            return unsupported_policy(
                element,
                "visibility",
                "predicate visibility is deferred until runtime resolver integration",
            );
        }
    }

    if element.surface.is_some() {
        return unsupported_policy(
            element,
            "surface",
            "element surface styling and shadow semantics are deferred until surface substrate",
        );
    }

    match element.overflow {
        None | Some(SceneElementOverflowPolicy::Clip) => {}
        Some(SceneElementOverflowPolicy::Hide) => {
            return unsupported_policy(
                element,
                "overflow",
                "hide overflow is deferred until element overflow substrate",
            );
        }
        Some(SceneElementOverflowPolicy::Wrap) => {
            return unsupported_policy(
                element,
                "overflow",
                "wrap overflow is deferred until element overflow substrate",
            );
        }
    }

    if element.placement_motion.is_some() {
        return unsupported_policy(
            element,
            "placementMotion",
            "placement motion is deferred until motion-aware scene rendering",
        );
    }

    if element.scroll_factor.is_some() {
        return unsupported_policy(
            element,
            "scrollFactor",
            "scene scroll response is deferred until camera/scroll runtime resolution",
        );
    }

    if element.clip_policy != ClipPolicy::Clip {
        return unsupported_policy(
            element,
            "clipPolicy",
            "warn clipping is deferred until element-aware diagnostics",
        );
    }

    match element.cell_write_policy {
        CellWritePolicy::WriteCell | CellWritePolicy::SkipTransparentEmpty => {}
    }

    match &element.role_write_policy {
        RoleWritePolicy::PreserveDestination => Ok(()),
        RoleWritePolicy::CopySampledSource => unsupported_policy(
            element,
            "roleWritePolicy",
            "copySampledSource is deferred until role policy substrate",
        ),
        RoleWritePolicy::SetExplicit { .. } => unsupported_policy(
            element,
            "roleWritePolicy",
            "setExplicit is deferred until role policy substrate",
        ),
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
// <VERS>END OF VERSION: 0.4.0</VERS>
