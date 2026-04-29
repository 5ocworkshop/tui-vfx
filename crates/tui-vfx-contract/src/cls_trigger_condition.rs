// <FILE>crates/tui-vfx-contract/src/cls_trigger_condition.rs</FILE> - <DESC>Lifecycle trigger condition DTO</DESC>
// <VERS>VERSION: 0.1.0</VERS>
// <WCTX>New kernel Phase I0: bind lifecycle triggers to typed value predicates.</WCTX>
// <CLOG>0.1.0: INIT — add ValueSource plus ValuePredicate trigger condition.</CLOG>

use std::collections::BTreeMap;

use crate::{
    DescriptorValidationError, ParameterId, ParameterSpec, SignalId, SignalSpec, ValuePredicate,
    ValueSource,
};

/// Condition that can fire a lifecycle trigger when its predicate passes.
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize, schemars::JsonSchema)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct TriggerCondition {
    /// Source sampled to evaluate the trigger condition.
    pub source: ValueSource,
    /// Typed predicate applied to the sampled source value.
    pub predicate: ValuePredicate,
}

impl TriggerCondition {
    /// Validate source references and predicate compatibility for recipe lifecycle use.
    pub fn validate(
        &self,
        parameters: &BTreeMap<ParameterId, ParameterSpec>,
        signals: &BTreeMap<SignalId, SignalSpec>,
    ) -> Result<(), DescriptorValidationError> {
        reject_graph_value_source(&self.source)?;
        let kind = self.source.infer_kind(parameters, signals)?;
        self.predicate.validate_for_kind(kind)
    }
}

fn reject_graph_value_source(source: &ValueSource) -> Result<(), DescriptorValidationError> {
    match source {
        ValueSource::GraphValue { id, .. } => Err(
            DescriptorValidationError::RecipeLifecycleGraphValueSourceNotAllowed { id: id.clone() },
        ),
        ValueSource::Map { from, .. } => reject_graph_value_source(from),
        ValueSource::Literal { .. }
        | ValueSource::Parameter { .. }
        | ValueSource::Signal { .. } => Ok(()),
    }
}

// <FILE>crates/tui-vfx-contract/src/cls_trigger_condition.rs</FILE> - <DESC>Lifecycle trigger condition DTO</DESC>
// <VERS>END OF VERSION: 0.1.0</VERS>
