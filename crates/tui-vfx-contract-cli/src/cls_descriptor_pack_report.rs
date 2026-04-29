// <FILE>crates/tui-vfx-contract-cli/src/cls_descriptor_pack_report.rs</FILE> - <DESC>Loaded descriptor pack report entry</DESC>
// <VERS>VERSION: 0.1.0</VERS>
// <WCTX>New kernel Phase J2: include descriptor-pack context in validation reports.</WCTX>
// <CLOG>0.1.0: INIT — add descriptor pack id/path report DTO.</CLOG>

/// Descriptor pack loaded for one validation run.
#[derive(Clone, Debug, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DescriptorPackReport {
    /// Loaded descriptor pack id.
    pub id: String,
    /// Filesystem path that supplied the descriptor pack.
    pub path: String,
}

// <FILE>crates/tui-vfx-contract-cli/src/cls_descriptor_pack_report.rs</FILE> - <DESC>Loaded descriptor pack report entry</DESC>
// <VERS>END OF VERSION: 0.1.0</VERS>
