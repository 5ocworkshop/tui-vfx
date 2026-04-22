// <FILE>crates/tui-vfx-probe/src/cls_probe_operational_analysis.rs</FILE> - <DESC>Structured operational analysis for direct probe reports</DESC>
// <VERS>VERSION: 0.4.0</VERS>
// <WCTX>Direct engine stage-by-stage success/failure reporting</WCTX>
// <CLOG>0.4.0: add optional grouped V3 shader-family labels to per-effect operational rows so direct probe analysis can report overall shader categories as well as concrete names.
// MINOR: Add configured_instances to per-effect operational rows so SQL consumers can see when same-name effects are aggregated instead of pretending instance identity is unique</CLOG>

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
    /// Concrete configured/observed effect label (for example `BorderSweep#1`).
    pub effect: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Optional grouped V3 family label for the effect (for example `traveling_band`).
    pub family: Option<String>,
    /// Whether this row came from configured inventory rather than only observation.
    pub configured: bool,
    /// How many configured instances share this effect label.
    pub configured_instances: usize,
    /// Number of unique cells this effect touched.
    pub touched_cells: usize,
    /// Total event count attributed to this effect.
    pub observed_event_count: usize,
    /// Operational status for this effect row.
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
// <VERS>END OF VERSION: 0.4.0</VERS>
