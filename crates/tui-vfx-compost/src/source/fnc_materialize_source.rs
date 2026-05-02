// <FILE>crates/tui-vfx-compost/src/source/fnc_materialize_source.rs</FILE> - <DESC>Materialize one canonical v3.1 source instance</DESC>
// <VERS>VERSION: 0.3.0</VERS>
// <WCTX>Source materialization dispatches descriptors before scene placement, clipping, and role-aware merge.</WCTX>
// <CLOG>0.3.0: MINOR — materialize source.text as a plain semantic text surface.
// 0.2.0: MINOR — materialize source.card as a semantic source surface.
// 0.1.2: PATCH — call the source.card grid builder through its owning module.
// 0.1.1: PATCH — simplify source descriptor dispatch around a named descriptor constant.</CLOG>

use std::collections::BTreeMap;

use tui_vfx_contract::{AssetId, AssetSpec, SourceSpec};
use tui_vfx_types::SemanticScene;

use crate::RenderError;
use crate::runtime::RuntimeContext;

use super::fnc_procedural_source_grid_from_inputs::procedural_source_grid_from_inputs;
use super::fnc_source_grid_from_inputs::source_grid_from_inputs;
use super::fnc_text_source_grid_from_inputs::text_source_grid_from_inputs;

const SOURCE_CARD_DESCRIPTOR: &str = "source.card";
const SOURCE_PROCEDURAL_DESCRIPTOR: &str = "source.procedural";
const SOURCE_TEXT_DESCRIPTOR: &str = "source.text";

pub(crate) fn materialize_source(
    source: &SourceSpec,
    assets: &BTreeMap<AssetId, AssetSpec>,
    context: &RuntimeContext,
) -> Result<SemanticScene, RenderError> {
    match source.source_descriptor.as_str() {
        SOURCE_CARD_DESCRIPTOR => source_grid_from_inputs(&source.inputs, context),
        SOURCE_PROCEDURAL_DESCRIPTOR => {
            procedural_source_grid_from_inputs(&source.inputs, assets, context)
        }
        SOURCE_TEXT_DESCRIPTOR => text_source_grid_from_inputs(&source.inputs, context),
        descriptor => Err(RenderError::Unsupported(format!(
            "load-validated source descriptor `{descriptor}` is not materializable"
        ))),
    }
}

// <FILE>crates/tui-vfx-compost/src/source/fnc_materialize_source.rs</FILE> - <DESC>Materialize one canonical v3.1 source instance</DESC>
// <VERS>END OF VERSION: 0.3.0</VERS>
