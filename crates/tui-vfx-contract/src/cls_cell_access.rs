// <FILE>crates/tui-vfx-contract/src/cls_cell_access.rs</FILE> - <DESC>Effect cell channel access DTO</DESC>
// <VERS>VERSION: 0.1.0</VERS>
// <WCTX>New kernel Phase E1: declare descriptor cell read/write channels.</WCTX>
// <CLOG>0.1.0: INIT — add channel read/write capability declaration.</CLOG>

use crate::CellChannel;

/// Cell channels an effect may read or write.
#[derive(
    Clone, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize, schemars::JsonSchema,
)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct CellAccess {
    /// Cell channels the effect may inspect.
    pub reads: Vec<CellChannel>,
    /// Cell channels the effect may write.
    pub writes: Vec<CellChannel>,
}

impl CellAccess {
    /// Return true when this descriptor declares read access to a channel.
    pub fn can_read(&self, channel: CellChannel) -> bool {
        self.reads.contains(&channel)
    }

    /// Return true when this descriptor declares write access to a channel.
    pub fn can_write(&self, channel: CellChannel) -> bool {
        self.writes.contains(&channel)
    }
}

// <FILE>crates/tui-vfx-contract/src/cls_cell_access.rs</FILE> - <DESC>Effect cell channel access DTO</DESC>
// <VERS>END OF VERSION: 0.1.0</VERS>
