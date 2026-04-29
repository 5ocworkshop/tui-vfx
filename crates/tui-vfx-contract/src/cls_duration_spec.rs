// <FILE>crates/tui-vfx-contract/src/cls_duration_spec.rs</FILE> - <DESC>Canonical lifecycle duration DTO</DESC>
// <VERS>VERSION: 0.1.0</VERS>
// <WCTX>New kernel Phase I0: represent lifecycle and trigger durations without legacy fallback names.</WCTX>
// <CLOG>0.1.0: INIT — add finite non-negative duration contract.</CLOG>

use crate::DescriptorValidationError;

/// Canonical duration used by lifecycle and trigger contracts.
#[derive(
    Clone, Copy, Debug, PartialEq, serde::Serialize, serde::Deserialize, schemars::JsonSchema,
)]
#[serde(tag = "kind", rename_all = "camelCase", deny_unknown_fields)]
pub enum DurationSpec {
    /// Duration expressed as whole milliseconds.
    Milliseconds {
        /// Non-negative millisecond count.
        value: u64,
    },
    /// Duration expressed as finite seconds.
    Seconds {
        /// Non-negative finite seconds.
        value: f64,
    },
}

impl DurationSpec {
    /// Validate the duration is finite and non-negative.
    pub fn validate(&self) -> Result<(), DescriptorValidationError> {
        match self {
            Self::Milliseconds { .. } => Ok(()),
            Self::Seconds { value } if value.is_finite() && *value >= 0.0 => Ok(()),
            Self::Seconds { value } => {
                Err(DescriptorValidationError::InvalidDuration { value: *value })
            }
        }
    }
}

// <FILE>crates/tui-vfx-contract/src/cls_duration_spec.rs</FILE> - <DESC>Canonical lifecycle duration DTO</DESC>
// <VERS>END OF VERSION: 0.1.0</VERS>
