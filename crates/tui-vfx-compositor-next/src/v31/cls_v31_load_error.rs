// <FILE>crates/tui-vfx-compositor-next/src/v31/cls_v31_load_error.rs</FILE> - <DESC>Direct v3.1 recipe load error type</DESC>
// <VERS>VERSION: 0.1.0</VERS>
// <WCTX>Keep load error ownership separate from validation orchestration so validators stay OFPF-sized.</WCTX>
// <CLOG>0.1.0: INIT — move V31LoadError out of the former load hub.</CLOG>

use tui_vfx_contract::DescriptorValidationError;

/// Error returned while accepting a recipe into the direct v3.1 renderer.
#[derive(Clone, Debug, PartialEq)]
pub enum V31LoadError {
    /// Canonical recipe validation failed before compositor-next rendering.
    Validation(DescriptorValidationError),
    /// This module only accepts v3.1 recipe and graph contracts.
    UnsupportedVersion {
        /// Recipe document version.
        recipe_version: String,
        /// Graph contract version.
        graph_version: String,
    },
    /// The direct lane only accepts graph inputs it can render directly.
    UnsupportedDirectInput {
        /// Graph node id containing the unsupported input.
        node_id: String,
        /// Effect descriptor id on the graph node.
        effect: String,
        /// Effect input id.
        input: String,
        /// Stable explanation of the unsupported input shape.
        reason: String,
    },
    /// The direct lane only accepts source inputs it can render directly.
    UnsupportedSourceInput {
        /// Recipe-local source instance id.
        source_id: String,
        /// Source descriptor id on the source instance.
        source: String,
        /// Source input id.
        input: String,
        /// Stable explanation of the unsupported input shape.
        reason: String,
    },
}

impl From<DescriptorValidationError> for V31LoadError {
    fn from(value: DescriptorValidationError) -> Self {
        Self::Validation(value)
    }
}

// <FILE>crates/tui-vfx-compositor-next/src/v31/cls_v31_load_error.rs</FILE> - <DESC>Direct v3.1 recipe load error type</DESC>
// <VERS>END OF VERSION: 0.1.0</VERS>
