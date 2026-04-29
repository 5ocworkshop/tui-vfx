// <FILE>crates/tui-vfx-contract/src/cls_effect_input_spec.rs</FILE> - <DESC>Effect descriptor input specification DTO</DESC>
// <VERS>VERSION: 0.2.0</VERS>
// <WCTX>K2.13 schema decision burn-down: allow descriptor inputs to be explicitly optional.</WCTX>
// <CLOG>0.2.0: MINOR — add optional input flag without changing required-by-default validation.
// 0.1.0: INIT — add input metadata, bindability, mutability, and validation wrapper.</CLOG>

use crate::{DescriptorValidationError, RuntimeMutability, ValueSpec};

/// Descriptor-local specification for one effect input.
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize, schemars::JsonSchema)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct EffectInputSpec {
    /// Optional human-facing display name for documentation and catalogs.
    pub display_name: Option<String>,
    /// Optional human-facing description for documentation and catalogs.
    pub description: Option<String>,
    /// Typed value contract, including default, range, enum values, unit, and semantic hints.
    pub value: ValueSpec,
    /// Whether this input may be omitted even when it has no value default.
    #[serde(default)]
    pub optional: bool,
    /// Whether later phases may bind this input to a source; no binding model exists in F1.
    pub bindable: bool,
    /// When this input value may change during the effect lifecycle.
    pub runtime_mutability: RuntimeMutability,
}

impl EffectInputSpec {
    /// Validate the nested value contract for this input.
    pub fn validate(&self) -> Result<(), DescriptorValidationError> {
        self.value.validate()
    }
}

// <FILE>crates/tui-vfx-contract/src/cls_effect_input_spec.rs</FILE> - <DESC>Effect descriptor input specification DTO</DESC>
// <VERS>END OF VERSION: 0.2.0</VERS>
