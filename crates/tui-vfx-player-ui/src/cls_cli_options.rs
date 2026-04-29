// <FILE>crates/tui-vfx-player-ui/src/cls_cli_options.rs</FILE> - <DESC>Visual player UI parsed options</DESC>
// <VERS>VERSION: 0.1.0</VERS>
// <WCTX>Player UI: configure one-recipe and recipe-root visual shell runs.</WCTX>
// <CLOG>0.1.0: INIT — add descriptor pack, recipe, dimensions, script, and one-shot options.</CLOG>

use std::path::PathBuf;

/// Parsed options for the player UI command.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct CliOptions {
    /// Canonical v3.1 recipe path to load.
    pub recipe_path: PathBuf,
    /// Canonical v3.1 recipe browser root.
    pub recipes_root: Option<PathBuf>,
    /// Descriptor pack files to load before rendering.
    pub descriptor_packs: Vec<String>,
    /// Descriptor pack directories to load before rendering.
    pub descriptor_pack_dirs: Vec<String>,
    /// Optional frame width override.
    pub width: Option<usize>,
    /// Optional frame height override.
    pub height: Option<usize>,
    /// Run one render and exit instead of entering the prompt loop.
    pub once: bool,
    /// Comma-separated command script for deterministic tests/manual demos.
    pub script: Option<String>,
    /// Suppress ANSI screen clear markers in output.
    pub no_clear: bool,
}

// <FILE>crates/tui-vfx-player-ui/src/cls_cli_options.rs</FILE> - <DESC>Visual player UI parsed options</DESC>
// <VERS>END OF VERSION: 0.1.0</VERS>
