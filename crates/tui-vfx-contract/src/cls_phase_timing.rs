// <FILE>crates/tui-vfx-contract/src/cls_phase_timing.rs</FILE> - <DESC>Lifecycle phase timing DTO</DESC>
// <VERS>VERSION: 0.1.0</VERS>
// <WCTX>New kernel Phase I0: distinguish fixed phase timing from dwell policy timing.</WCTX>
// <CLOG>0.1.0: INIT — add phase timing contract.</CLOG>

use std::collections::BTreeMap;

use crate::{
    DescriptorValidationError, DurationSpec, DwellPolicy, ParameterId, ParameterSpec, SignalId,
    SignalSpec,
};

/// Timing semantics for one named lifecycle phase.
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize, schemars::JsonSchema)]
#[serde(tag = "kind", rename_all = "camelCase", deny_unknown_fields)]
pub enum PhaseTiming {
    /// Phase lasts for a fixed duration.
    Fixed {
        /// Duration of the phase interval.
        duration: DurationSpec,
    },
    /// Dwell-specific timing policy.
    Dwell {
        /// Policy controlling dwell completion.
        policy: DwellPolicy,
    },
}

impl PhaseTiming {
    /// Validate nested timing contracts.
    pub fn validate(
        &self,
        parameters: &BTreeMap<ParameterId, ParameterSpec>,
        signals: &BTreeMap<SignalId, SignalSpec>,
    ) -> Result<(), DescriptorValidationError> {
        match self {
            Self::Fixed { duration } => duration.validate(),
            Self::Dwell { policy } => policy.validate(parameters, signals),
        }
    }
}

// <FILE>crates/tui-vfx-contract/src/cls_phase_timing.rs</FILE> - <DESC>Lifecycle phase timing DTO</DESC>
// <VERS>END OF VERSION: 0.1.0</VERS>
