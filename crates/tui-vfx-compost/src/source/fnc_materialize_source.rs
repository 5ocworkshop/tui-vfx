// <FILE>crates/tui-vfx-compost/src/source/fnc_materialize_source.rs</FILE> - <DESC>Materialize one canonical v3.1 source instance</DESC>
// <VERS>VERSION: 0.1.2</VERS>
// <WCTX>Source materialization dispatches descriptors before scene placement and clipping.</WCTX>
// <CLOG>0.1.2: PATCH — call the source.card grid builder through its owning module.
// 0.1.1: PATCH — simplify source descriptor dispatch around a named descriptor constant.</CLOG>

use tui_vfx_contract::SourceSpec;
use tui_vfx_types::OwnedGrid;

use crate::RenderError;

use super::fnc_source_grid_from_inputs::source_grid_from_inputs;

const SOURCE_CARD_DESCRIPTOR: &str = "source.card";

pub(crate) fn materialize_source(source: &SourceSpec) -> Result<OwnedGrid, RenderError> {
    match source.source_descriptor.as_str() {
        SOURCE_CARD_DESCRIPTOR => source_grid_from_inputs(&source.inputs),
        descriptor => Err(RenderError::Unsupported(format!(
            "load-validated source descriptor `{descriptor}` is not materializable"
        ))),
    }
}

// <FILE>crates/tui-vfx-compost/src/source/fnc_materialize_source.rs</FILE> - <DESC>Materialize one canonical v3.1 source instance</DESC>
// <VERS>END OF VERSION: 0.1.2</VERS>
