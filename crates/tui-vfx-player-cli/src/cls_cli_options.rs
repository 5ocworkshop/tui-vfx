// <FILE>crates/tui-vfx-player-cli/src/cls_cli_options.rs</FILE> - <DESC>Player CLI parsed option DTO</DESC>
// <VERS>VERSION: 0.1.0</VERS>
// <WCTX>New kernel Phase K0: parse render-recipe sampling options.</WCTX>
// <CLOG>0.1.0: INIT — add recursive, descriptor-pack, phase, size, and JSON options.</CLOG>

use tui_vfx_contract::LifecyclePhase;

/// Parsed options for `render-recipe`.
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
        }
    }
}

// <FILE>crates/tui-vfx-player-cli/src/cls_cli_options.rs</FILE> - <DESC>Player CLI parsed option DTO</DESC>
// <VERS>END OF VERSION: 0.1.0</VERS>
