// <FILE>crates/tui-vfx-contract/src/cls_binding_spec.rs</FILE> - <DESC>Declarative parameter binding specification DTO</DESC>
// <VERS>VERSION: 0.1.0</VERS>
// <WCTX>New kernel Phase F2: bind value sources to public parameters declaratively.</WCTX>
// <CLOG>0.1.0: INIT — add parameter-target binding spec and compatibility validation.</CLOG>

use std::collections::BTreeMap;

use crate::{
    BindingMode, BindingTarget, DescriptorValidationError, ParameterId, ParameterSpec, SignalId,
    SignalSpec, ValueSource,
};

/// Declarative binding from a value source to a parameter target.
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize, schemars::JsonSchema)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct BindingSpec {
    /// Target that receives the source value.
    pub target: BindingTarget,
    /// Source that provides the target value.
    pub source: ValueSource,
    /// Declarative binding application mode.
    pub mode: BindingMode,
}

impl BindingSpec {
    /// Validate this binding against declared parameters and signals.
    pub fn validate(
        &self,
        parameters: &BTreeMap<ParameterId, ParameterSpec>,
        signals: &BTreeMap<SignalId, SignalSpec>,
    ) -> Result<(), DescriptorValidationError> {
        match &self.target {
            BindingTarget::Parameter { id } => {
                let spec = parameters.get(id).ok_or_else(|| {
                    DescriptorValidationError::UnknownBindingParameterTarget { id: id.clone() }
                })?;
                spec.validate()?;
                if !spec.bindable {
                    return Err(DescriptorValidationError::ParameterNotBindable { id: id.clone() });
                }
                self.source
                    .validate_kind(spec.value.kind, parameters, signals)
            }
        }
    }
}

// <FILE>crates/tui-vfx-contract/src/cls_binding_spec.rs</FILE> - <DESC>Declarative parameter binding specification DTO</DESC>
// <VERS>END OF VERSION: 0.1.0</VERS>
