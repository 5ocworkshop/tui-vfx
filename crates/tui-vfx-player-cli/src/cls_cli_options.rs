// <FILE>crates/tui-vfx-player-cli/src/cls_cli_options.rs</FILE> - <DESC>Player CLI parsed option DTO</DESC>
// <VERS>VERSION: 0.5.0</VERS>
// <WCTX>K2.12 schema lock: parse offender-ledger output flag.</WCTX>
// <CLOG>0.5.0: MINOR — add schema-readiness offender detail flag.
// 0.4.0: MINOR — add migration mapping family filter.
// 0.3.1: PATCH — collapse historical option metadata into latest-change context.</CLOG>

use tui_vfx_contract::LifecyclePhase;

/// Parsed options for player CLI commands.
#[derive(Clone, Debug, PartialEq)]
pub struct CliOptions {
    /// Recursively collect JSON files from directory arguments.
    pub recursive: bool,
    /// Emit JSON reports.
    pub json: bool,
    /// Recipe files or directories to render.
    pub paths: Vec<String>,
    /// Descriptor pack files to load.
    pub descriptor_packs: Vec<String>,
    /// Descriptor pack directories to load.
    pub descriptor_pack_dirs: Vec<String>,
    /// Requested lifecycle phase.
    pub phase: LifecyclePhase,
    /// Requested normalized phase progress.
    pub phase_t: f64,
    /// Optional requested loop progress.
    pub loop_t: Option<f64>,
    /// Optional frame width override.
    pub width: Option<usize>,
    /// Optional frame height override.
    pub height: Option<usize>,
    /// Legacy debug recipe root for migration-gap reports.
    pub legacy_root: Option<String>,
    /// Canonical v3.1 debug recipe root for migration-gap reports.
    pub v31_root: Option<String>,
    /// Optional legacy family filter for migration mapping reports.
    pub family: Option<String>,
    /// Include per-record schema-readiness offender rows.
    pub include_offenders: bool,
    /// Number of timeline frames to sample.
    pub frames: usize,
    /// Starting sample for frame-diff reports.
    pub from_sample_t: f64,
    /// Ending sample for frame-diff reports.
    pub to_sample_t: f64,
}

impl Default for CliOptions {
    fn default() -> Self {
        Self {
            recursive: false,
            json: true,
            paths: Vec::new(),
            descriptor_packs: Vec::new(),
            descriptor_pack_dirs: Vec::new(),
            phase: LifecyclePhase::Dwell,
            phase_t: 1.0,
            loop_t: None,
            width: None,
            height: None,
            legacy_root: None,
            v31_root: None,
            family: None,
            include_offenders: false,
            frames: 1,
            from_sample_t: 0.0,
            to_sample_t: 1.0,
        }
    }
}

// <FILE>crates/tui-vfx-player-cli/src/cls_cli_options.rs</FILE> - <DESC>Player CLI parsed option DTO</DESC>
// <VERS>END OF VERSION: 0.5.0</VERS>
