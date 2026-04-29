// <FILE>crates/tui-vfx-contract-cli/src/cls_cli_options.rs</FILE> - <DESC>Contract CLI option DTO</DESC>
// <VERS>VERSION: 0.2.0</VERS>
// <WCTX>New kernel Phase J2: support descriptor-pack validation flags.</WCTX>
// <CLOG>0.2.0: MINOR — add descriptor pack file and directory options.
// 0.1.0: INIT — add parsed validate-recipe option container.</CLOG>

/// Parsed options for the `validate-recipe` command.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct CliOptions {
    /// Recursively collect JSON files from directory arguments.
    pub recursive: bool,
    /// Emit JSON reports. JSON is the default and only J1 output format.
    pub json: bool,
    /// Files or directories requested by the user.
    pub paths: Vec<String>,
    /// Descriptor pack files loaded before recipe validation.
    pub descriptor_packs: Vec<String>,
    /// Descriptor pack directories loaded before recipe validation.
    pub descriptor_pack_dirs: Vec<String>,
}

// <FILE>crates/tui-vfx-contract-cli/src/cls_cli_options.rs</FILE> - <DESC>Contract CLI option DTO</DESC>
// <VERS>END OF VERSION: 0.2.0</VERS>
