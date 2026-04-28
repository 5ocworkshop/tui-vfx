// <FILE>crates/tui-vfx-contract/src/cls_parameter_spec.rs</FILE> - <DESC>Public parameter specification DTO</DESC>
// <VERS>VERSION: 0.1.0</VERS>
// <WCTX>New kernel Phase F2: declare public recipe parameter contracts.</WCTX>
// <CLOG>0.1.0: INIT — add parameter id, metadata, value spec, bindability, and validation.</CLOG>

use crate::{DescriptorValidationError, ParameterId, ValueSpec};

/// Public recipe parameter contract.
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize, schemars::JsonSchema)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct ParameterSpec {
    /// Stable public parameter identifier.
    pub id: ParameterId,
    /// Optional human-facing display name.
    pub display_name: Option<String>,
    /// Optional human-facing description.
    pub description: Option<String>,
    /// Typed value contract, including any parameter default.
    pub value: ValueSpec,
    /// Whether later runtime layers may bind this parameter declaratively.
    pub bindable: bool,
}

impl ParameterSpec {
    /// Validate the parameter id and nested value contract.
    pub fn validate(&self) -> Result<(), DescriptorValidationError> {
        if !self.id.is_valid() {
            return Err(DescriptorValidationError::InvalidParameterId {
                id: self.id.clone(),
            });
        }

        self.value.validate()
    }
}

// <FILE>crates/tui-vfx-contract/src/cls_parameter_spec.rs</FILE> - <DESC>Public parameter specification DTO</DESC>
// <VERS>END OF VERSION: 0.1.0</VERS>
