// <FILE>crates/tui-vfx-contract/src/cls_phase_spec.rs</FILE> - <DESC>Named lifecycle phase DTO</DESC>
// <VERS>VERSION: 0.1.0</VERS>
// <WCTX>New kernel Phase I0: couple lifecycle phase names to strict timing semantics.</WCTX>
// <CLOG>0.1.0: INIT — add phase spec validation.</CLOG>

use std::collections::BTreeMap;

use crate::{
    DescriptorValidationError, LifecyclePhase, ParameterId, ParameterSpec, PhaseTiming, SignalId,
    SignalSpec,
};

/// One named lifecycle interval and its timing semantics.
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize, schemars::JsonSchema)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct PhaseSpec {
    /// Named lifecycle interval.
    pub phase: LifecyclePhase,
    /// Timing semantics for this interval.
    pub timing: PhaseTiming,
}

impl PhaseSpec {
    /// Validate timing compatibility for the named phase.
    pub fn validate(
        &self,
        parameters: &BTreeMap<ParameterId, ParameterSpec>,
        signals: &BTreeMap<SignalId, SignalSpec>,
    ) -> Result<(), DescriptorValidationError> {
        if !matches!(self.phase, LifecyclePhase::Dwell)
            && matches!(self.timing, PhaseTiming::Dwell { .. })
        {
            return Err(DescriptorValidationError::DwellTimingOnNonDwellPhase {
                phase: self.phase,
            });
        }
        self.timing.validate(parameters, signals)
    }
}

// <FILE>crates/tui-vfx-contract/src/cls_phase_spec.rs</FILE> - <DESC>Named lifecycle phase DTO</DESC>
// <VERS>END OF VERSION: 0.1.0</VERS>
