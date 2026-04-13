// <FILE>crates/tui-vfx-probe/src/cls_probe_request.rs</FILE> - <DESC>Probe request DTOs</DESC>
// <VERS>VERSION: 0.2.0</VERS>
// <WCTX>Phase-1 pipeline probe implementation</WCTX>
// <CLOG>MINOR: Map probe phases onto mixed-signals Start/Active/End and keep request DTOs serializable for structured output</CLOG>

use mixed_signals::traits::Phase;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ProbeCellSelector {
    All,
    NonEmpty,
    Modified,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ProbePhase {
    Entering,
    Dwelling,
    Exiting,
}

impl ProbePhase {
    pub fn to_mixed_phase(self) -> Phase {
        match self {
            Self::Entering => Phase::Start,
            Self::Dwelling => Phase::Active,
            Self::Exiting => Phase::End,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ProbeRequest {
    pub phase: ProbePhase,
    pub sample_t: f64,
    pub cells: ProbeCellSelector,
    #[serde(default)]
    pub with_causation: bool,
}

// <FILE>crates/tui-vfx-probe/src/cls_probe_request.rs</FILE> - <DESC>Probe request DTOs</DESC>
// <VERS>END OF VERSION: 0.2.0</VERS>
