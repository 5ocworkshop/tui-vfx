// <FILE>crates/tui-vfx-contract-cli/src/cls_descriptor_pack_load.rs</FILE> - <DESC>Loaded descriptor catalog and report metadata</DESC>
// <VERS>VERSION: 0.1.0</VERS>
// <WCTX>New kernel Phase J2: carry descriptor catalog and report entries together.</WCTX>
// <CLOG>0.1.0: INIT — add loaded descriptor pack bundle.</CLOG>

use tui_vfx_contract::DescriptorCatalog;

use crate::cls_descriptor_pack_report::DescriptorPackReport;

/// Descriptor catalog plus machine-readable loaded-pack metadata.
#[derive(Clone, Debug)]
pub struct DescriptorPackLoad {
    /// Loaded descriptor catalog.
    pub catalog: DescriptorCatalog,
    /// Report entries for loaded descriptor packs.
    pub reports: Vec<DescriptorPackReport>,
}

// <FILE>crates/tui-vfx-contract-cli/src/cls_descriptor_pack_load.rs</FILE> - <DESC>Loaded descriptor catalog and report metadata</DESC>
// <VERS>END OF VERSION: 0.1.0</VERS>
