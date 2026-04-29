// <FILE>crates/tui-vfx-contract/src/cls_source_output_size.rs</FILE> - <DESC>Source output size behavior DTO</DESC>
// <VERS>VERSION: 0.1.0</VERS>
// <WCTX>New kernel Phase H0: describe produced surface sizing semantics.</WCTX>
// <CLOG>0.1.0: INIT — add input-driven, fixed, and host-driven source size modes.</CLOG>

/// Declares how a source determines the size of its produced semantic surface.
#[derive(
    Clone, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize, schemars::JsonSchema,
)]
#[serde(tag = "kind", rename_all = "camelCase", deny_unknown_fields)]
pub enum SourceOutputSize {
    /// Output size is derived from source inputs or assets.
    InputDriven,
    /// Output size is fixed by the descriptor.
    Fixed {
        /// Fixed output width in cells.
        width: u16,
        /// Fixed output height in cells.
        height: u16,
    },
    /// Output size is provided by the host or containing scene/recipe context.
    HostDriven,
}

// <FILE>crates/tui-vfx-contract/src/cls_source_output_size.rs</FILE> - <DESC>Source output size behavior DTO</DESC>
// <VERS>END OF VERSION: 0.1.0</VERS>
