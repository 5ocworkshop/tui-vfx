// <FILE>crates/tui-vfx-contract/src/cls_dwell_policy.rs</FILE> - <DESC>Lifecycle dwell policy DTO</DESC>
// <VERS>VERSION: 0.1.0</VERS>
// <WCTX>New kernel Phase I0: model dwell duration and trigger caps without legacy fallback names.</WCTX>
// <CLOG>0.1.0: INIT — add fixed and trigger-terminated dwell policies.</CLOG>

use std::collections::BTreeMap;

use crate::{
    DescriptorValidationError, DurationSpec, ParameterId, ParameterSpec, SignalId, SignalSpec,
    TriggerSpec,
};

/// Policy controlling how long the dwell lifecycle phase remains active.
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize, schemars::JsonSchema)]
#[serde(tag = "kind", rename_all = "camelCase", deny_unknown_fields)]
pub enum DwellPolicy {
    /// Dwell lasts for a fixed duration.
    Fixed {
        /// Duration of the dwell interval.
        duration: DurationSpec,
    },
    /// Dwell lasts until a lifecycle trigger fires, optionally capped by maxDuration.
    Until {
        /// Trigger that can end the dwell interval.
        trigger: TriggerSpec,
        /// Maximum dwell duration before progressing even if the trigger never fires.
        #[serde(rename = "maxDuration")]
        max_duration: Option<DurationSpec>,
    },
}

impl DwellPolicy {
    /// Validate nested duration and trigger contracts.
    pub fn validate(
        &self,
        parameters: &BTreeMap<ParameterId, ParameterSpec>,
        signals: &BTreeMap<SignalId, SignalSpec>,
    ) -> Result<(), DescriptorValidationError> {
        match self {
            Self::Fixed { duration } => duration.validate(),
            Self::Until {
                trigger,
                max_duration,
            } => {
                trigger.validate(parameters, signals)?;
                if let Some(duration) = max_duration {
                    duration.validate()?;
                }
                Ok(())
            }
        }
    }
}

// <FILE>crates/tui-vfx-contract/src/cls_dwell_policy.rs</FILE> - <DESC>Lifecycle dwell policy DTO</DESC>
// <VERS>END OF VERSION: 0.1.0</VERS>
