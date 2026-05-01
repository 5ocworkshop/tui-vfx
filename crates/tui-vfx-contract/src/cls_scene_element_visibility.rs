// <FILE>crates/tui-vfx-contract/src/cls_scene_element_visibility.rs</FILE> - <DESC>Scene element visibility DTO</DESC>
// <VERS>VERSION: 0.1.0</VERS>
// <WCTX>v3.1 scene parity: preserve always, phase, and binding-backed predicate visibility semantics.</WCTX>
// <CLOG>0.1.0: INIT — add typed scene visibility policy with predicate validation.</CLOG>

use std::collections::BTreeMap;

use crate::{
    DescriptorValidationError, LifecyclePhase, ParameterId, ParameterSpec, SignalId, SignalSpec,
    ValuePredicate, ValueSource,
};

/// Visibility policy for one recipe scene element.
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize, schemars::JsonSchema)]
#[serde(tag = "kind", rename_all = "camelCase", deny_unknown_fields)]
pub enum SceneElementVisibility {
    /// Element is always visible.
    Always,
    /// Element is visible only for the listed lifecycle phases.
    Phase {
        /// Lifecycle phases where the element is visible.
        phases: Vec<LifecyclePhase>,
    },
    /// Element visibility is decided by evaluating a typed value source.
    Predicate {
        /// Value source used by the visibility predicate.
        predicate_source: ValueSource,
        /// Predicate that decides whether the element is visible.
        predicate: ValuePredicate,
    },
}

impl SceneElementVisibility {
    /// Validate references and predicate compatibility for this visibility rule.
    pub fn validate(
        &self,
        parameters: &BTreeMap<ParameterId, ParameterSpec>,
        signals: &BTreeMap<SignalId, SignalSpec>,
    ) -> Result<(), DescriptorValidationError> {
        match self {
            Self::Always | Self::Phase { .. } => Ok(()),
            Self::Predicate {
                predicate_source,
                predicate,
            } => {
                let kind = predicate_source.infer_kind(parameters, signals)?;
                predicate.validate_for_kind(kind)
            }
        }
    }
}

// <FILE>crates/tui-vfx-contract/src/cls_scene_element_visibility.rs</FILE> - <DESC>SceneElementVisibility</DESC>
// <VERS>END OF VERSION: 0.1.0</VERS>
