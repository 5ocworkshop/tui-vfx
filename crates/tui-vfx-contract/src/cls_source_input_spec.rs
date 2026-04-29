// <FILE>crates/tui-vfx-contract/src/cls_source_input_spec.rs</FILE> - <DESC>Source descriptor input specification DTO</DESC>
// <VERS>VERSION: 0.2.0</VERS>
// <WCTX>K2.13 schema decision burn-down: allow source descriptor inputs to be explicitly optional.</WCTX>
// <CLOG>0.2.0: MINOR — add optional source input flag without changing required-by-default validation.
// 0.1.0: INIT — reuse ValueSpec for source input specs with bindability and mutability metadata.</CLOG>

use crate::{DescriptorValidationError, RuntimeMutability, ValueSpec};

/// Descriptor-local specification for one source input.
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize, schemars::JsonSchema)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct SourceInputSpec {
    /// Optional human-facing display name for documentation and catalogs.
    pub display_name: Option<String>,
    /// Optional human-facing description for documentation and catalogs.
    pub description: Option<String>,
    /// Typed value contract, reusing the shared v3.1 value vocabulary.
    pub value: ValueSpec,
    /// Whether this input may be omitted even when it has no value default.
    #[serde(default)]
    pub optional: bool,
    /// Whether recipe/graph/runtime value sources may bind this input.
    pub bindable: bool,
    /// When this source input value may change during the source lifecycle.
    pub runtime_mutability: RuntimeMutability,
}

impl SourceInputSpec {
    /// Validate the nested value contract for this input.
    pub fn validate(&self) -> Result<(), DescriptorValidationError> {
        self.value.validate()
    }
}

// <FILE>crates/tui-vfx-contract/src/cls_source_input_spec.rs</FILE> - <DESC>Source descriptor input specification DTO</DESC>
// <VERS>END OF VERSION: 0.2.0</VERS>
