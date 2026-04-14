// <FILE>crates/tui-vfx-probe/src/cls_probe_operational_analysis.rs</FILE> - <DESC>Structured operational analysis for direct probe reports</DESC>
// <VERS>VERSION: 0.3.0</VERS>
// <WCTX>Direct engine stage-by-stage success/failure reporting</WCTX>
// <CLOG>MINOR: Add configured_instances to per-effect operational rows so SQL consumers can see when same-name effects are aggregated instead of pretending instance identity is unique</CLOG>

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ProbeOperationalStatus {
    Success,
    Warning,
    Failure,
    Inactive,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ProbeEffectOperationalAnalysis {
    pub effect: String,
    pub configured: bool,
    pub configured_instances: usize,
    pub touched_cells: usize,
    pub observed_event_count: usize,
    pub status: ProbeOperationalStatus,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ProbeStageOperationalAnalysis {
    pub stage: String,
    pub configured: bool,
    pub configured_count: usize,
    pub touched_cells: usize,
    pub observed_event_count: usize,
    pub observed_effects: Vec<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub effects: Vec<ProbeEffectOperationalAnalysis>,
    pub status: ProbeOperationalStatus,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ProbeCombinedOperationalAnalysis {
    pub status: ProbeOperationalStatus,
    pub error_diagnostics: usize,
    pub warning_diagnostics: usize,
    pub failing_stages: Vec<String>,
    pub diagnostic_codes: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ProbeOperationalAnalysis {
    pub scope: String,
    pub frame_count: usize,
    pub stages: Vec<ProbeStageOperationalAnalysis>,
    pub combined: ProbeCombinedOperationalAnalysis,
}

// <FILE>crates/tui-vfx-probe/src/cls_probe_operational_analysis.rs</FILE> - <DESC>Structured operational analysis for direct probe reports</DESC>
// <VERS>END OF VERSION: 0.3.0</VERS>
