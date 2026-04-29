// <FILE>crates/tui-vfx-player-cli/src/cls_cli_options.rs</FILE> - <DESC>Player CLI parsed option DTO</DESC>
// <VERS>VERSION: 0.2.0</VERS>
// <WCTX>New kernel Phase K2.1: parse render, inventory, and migration-gap options.</WCTX>
// <CLOG>0.2.0: MINOR — add migration-gap legacy and v3.1 root options.</CLOG>

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
        }
    }
}

// <FILE>crates/tui-vfx-player-cli/src/cls_cli_options.rs</FILE> - <DESC>Player CLI parsed option DTO</DESC>
// <VERS>END OF VERSION: 0.2.0</VERS>
