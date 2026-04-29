// <FILE>crates/tui-vfx-contract/src/cls_trigger_spec.rs</FILE> - <DESC>Lifecycle trigger DTO</DESC>
// <VERS>VERSION: 0.1.0</VERS>
// <WCTX>New kernel Phase I0: make lifecycle trigger condition, latch, reset, and action explicit.</WCTX>
// <CLOG>0.1.0: INIT — add trigger contract with explicit transition semantics.</CLOG>

use std::collections::BTreeMap;

use crate::{
    DescriptorValidationError, ParameterId, ParameterSpec, SignalId, SignalSpec, TriggerAction,
    TriggerCondition, TriggerLatchPolicy, TriggerResetBoundary,
};

/// Canonical lifecycle trigger with explicit condition, latch, reset, and action semantics.
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize, schemars::JsonSchema)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct TriggerSpec {
    /// Condition sampled to determine whether the trigger fires.
    pub condition: TriggerCondition,
    /// Whether the trigger remains fired after first passing.
    pub latch: TriggerLatchPolicy,
    /// Boundary at which sampled/latch state is reset.
    pub reset: TriggerResetBoundary,
    /// Lifecycle action requested when the trigger fires.
    pub action: TriggerAction,
}

impl TriggerSpec {
    /// Validate the trigger's value source and predicate compatibility.
    pub fn validate(
        &self,
        parameters: &BTreeMap<ParameterId, ParameterSpec>,
        signals: &BTreeMap<SignalId, SignalSpec>,
    ) -> Result<(), DescriptorValidationError> {
        self.condition.validate(parameters, signals)
    }
}

// <FILE>crates/tui-vfx-contract/src/cls_trigger_spec.rs</FILE> - <DESC>Lifecycle trigger DTO</DESC>
// <VERS>END OF VERSION: 0.1.0</VERS>
