// <FILE>crates/tui-vfx-contract/src/cls_lifecycle_spec.rs</FILE> - <DESC>Recipe lifecycle DTO</DESC>
// <VERS>VERSION: 0.1.0</VERS>
// <WCTX>New kernel Phase I0: define recipe-level time, lifecycle, and trigger contracts.</WCTX>
// <CLOG>0.1.0: INIT — add recipe lifecycle clock and enter/dwell/exit phase validation.</CLOG>

use std::collections::{BTreeMap, BTreeSet};

use crate::{
    ClockSpec, DescriptorValidationError, LifecyclePhase, ParameterId, ParameterSpec, PhaseSpec,
    SignalId, SignalSpec,
};

/// Recipe-level lifecycle contract from enter through dwell and exit to finished.
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize, schemars::JsonSchema)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct LifecycleSpec {
    /// Clock that defines the recipe-level time sample space.
    pub clock: ClockSpec,
    /// Ordered phase contracts. The initial profile is enter, dwell, then exit.
    pub phases: Vec<PhaseSpec>,
}

impl LifecycleSpec {
    /// Validate the clock and required enter, dwell, exit phase contracts.
    pub fn validate(
        &self,
        parameters: &BTreeMap<ParameterId, ParameterSpec>,
        signals: &BTreeMap<SignalId, SignalSpec>,
    ) -> Result<(), DescriptorValidationError> {
        self.clock.validate()?;
        validate_phase_set(&self.phases)?;
        for phase in &self.phases {
            phase.validate(parameters, signals)?;
        }
        Ok(())
    }
}

fn validate_phase_set(phases: &[PhaseSpec]) -> Result<(), DescriptorValidationError> {
    let expected = [
        LifecyclePhase::Enter,
        LifecyclePhase::Dwell,
        LifecyclePhase::Exit,
    ];
    let mut seen = BTreeSet::new();
    for phase in phases {
        if !seen.insert(phase.phase) {
            return Err(DescriptorValidationError::DuplicateLifecyclePhase { phase: phase.phase });
        }
    }
    for required in expected {
        if !seen.contains(&required) {
            return Err(DescriptorValidationError::MissingLifecyclePhase { phase: required });
        }
    }
    for (index, required) in expected.into_iter().enumerate() {
        if phases.get(index).map(|phase| phase.phase) != Some(required) {
            return Err(DescriptorValidationError::UnexpectedLifecyclePhaseOrder {
                expected: required,
                actual: phases.get(index).map(|phase| phase.phase),
            });
        }
    }
    Ok(())
}

// <FILE>crates/tui-vfx-contract/src/cls_lifecycle_spec.rs</FILE> - <DESC>Recipe lifecycle DTO</DESC>
// <VERS>END OF VERSION: 0.1.0</VERS>
