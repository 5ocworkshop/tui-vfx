// <FILE>crates/tui-vfx-contract/src/cls_signal_spec.rs</FILE> - <DESC>Host signal specification DTO</DESC>
// <VERS>VERSION: 0.1.0</VERS>
// <WCTX>New kernel Phase F2: declare host/runtime-provided signal contracts.</WCTX>
// <CLOG>0.1.0: INIT — add signal id, metadata, value spec, requirement policy, and validation.</CLOG>

use crate::{DescriptorValidationError, PreviewLoopbackSpec, SignalId, ValueSpec};

/// Host/runtime-provided signal contract.
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize, schemars::JsonSchema)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct SignalSpec {
    /// Stable host/runtime signal identifier.
    pub id: SignalId,
    /// Optional human-facing display name.
    pub display_name: Option<String>,
    /// Optional human-facing description.
    pub description: Option<String>,
    /// Typed value contract, including any fallback default.
    pub value: ValueSpec,
    /// Optional deterministic preview/demo provider used only when the host does not supply the signal.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub preview_loopback: Option<PreviewLoopbackSpec>,
    /// Whether a host must provide this signal when no default is declared.
    pub required: bool,
}

impl SignalSpec {
    /// Validate the signal id and nested value contract.
    pub fn validate(&self) -> Result<(), DescriptorValidationError> {
        if !self.id.is_valid() {
            return Err(DescriptorValidationError::InvalidSignalId {
                id: self.id.clone(),
            });
        }

        self.value.validate()?;
        if let Some(loopback) = &self.preview_loopback {
            loopback.validate_for_signal(&self.value)?;
        }
        Ok(())
    }
}

// <FILE>crates/tui-vfx-contract/src/cls_signal_spec.rs</FILE> - <DESC>Host signal specification DTO</DESC>
// <VERS>END OF VERSION: 0.1.0</VERS>
