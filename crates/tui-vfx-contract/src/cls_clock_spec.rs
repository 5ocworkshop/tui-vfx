// <FILE>crates/tui-vfx-contract/src/cls_clock_spec.rs</FILE> - <DESC>Recipe lifecycle clock DTO</DESC>
// <VERS>VERSION: 0.1.0</VERS>
// <WCTX>New kernel Phase I0: define recipe-level time sample-space contracts.</WCTX>
// <CLOG>0.1.0: INIT — add clock mode and optional loop period validation.</CLOG>

use crate::{ClockMode, DescriptorValidationError, DurationSpec};

/// Recipe-level clock contract for lifecycle phase timing.
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize, schemars::JsonSchema)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct ClockSpec {
    /// Time sample-space mode.
    pub clock_mode: ClockMode,
    /// Loop period required only when mode is `looping`.
    pub period: Option<DurationSpec>,
}

impl ClockSpec {
    /// Validate loop-period requirements and duration shape.
    pub fn validate(&self) -> Result<(), DescriptorValidationError> {
        match (self.clock_mode, self.period) {
            (ClockMode::Looping, Some(period)) => period.validate(),
            (ClockMode::Looping, None) => Err(DescriptorValidationError::MissingClockPeriod),
            (ClockMode::Monotonic, Some(_)) => {
                Err(DescriptorValidationError::UnexpectedClockPeriod)
            }
            (ClockMode::Monotonic, None) => Ok(()),
        }
    }
}

// <FILE>crates/tui-vfx-contract/src/cls_clock_spec.rs</FILE> - <DESC>Recipe lifecycle clock DTO</DESC>
// <VERS>END OF VERSION: 0.1.0</VERS>
