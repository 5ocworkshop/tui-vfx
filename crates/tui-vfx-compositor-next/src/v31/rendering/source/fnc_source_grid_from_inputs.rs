// <FILE>crates/tui-vfx-compositor-next/src/v31/rendering/source/fnc_source_grid_from_inputs.rs</FILE> - <DESC>Create source grid from direct v3.1 source inputs</DESC>
// <VERS>VERSION: 0.1.0</VERS>
// <WCTX>Translate load-validated literal source inputs into an OwnedGrid before compositor execution.</WCTX>
// <CLOG>0.1.0: INIT — extract source input to grid conversion.</CLOG>

use std::collections::BTreeMap;

use tui_vfx_contract::{SourceInputId, Value, ValueSource};
use tui_vfx_types::OwnedGrid;

use super::col_source_grid_from_text::source_grid_from_text;
use crate::v31::V31RenderError;

pub(crate) fn source_grid_from_inputs(
    inputs: &BTreeMap<SourceInputId, ValueSource>,
    fallback_width: usize,
    fallback_height: usize,
) -> Result<OwnedGrid, V31RenderError> {
    let text = inputs
        .get(&SourceInputId::new("message"))
        .or_else(|| inputs.get(&SourceInputId::new("text")))
        .and_then(literal_text)
        .ok_or_else(|| {
            V31RenderError::Unsupported(
                "Direct v3.1 rendering requires a literal text/message source.".to_string(),
            )
        })?;
    let width = inputs
        .get(&SourceInputId::new("width"))
        .and_then(literal_number)
        .map(|value| value.max(1.0) as usize)
        .unwrap_or(fallback_width);
    let height = inputs
        .get(&SourceInputId::new("height"))
        .and_then(literal_number)
        .map(|value| value.max(1.0) as usize)
        .unwrap_or(fallback_height);
    Ok(source_grid_from_text(text, width, height))
}

fn literal_text(source: &ValueSource) -> Option<&str> {
    match source {
        ValueSource::Literal {
            value: Value::Text(value) | Value::String(value),
        } => Some(value.as_str()),
        _ => None,
    }
}

fn literal_number(source: &ValueSource) -> Option<f64> {
    match source {
        ValueSource::Literal { value } => value.as_range_number(),
        _ => None,
    }
}

// <FILE>crates/tui-vfx-compositor-next/src/v31/rendering/source/fnc_source_grid_from_inputs.rs</FILE> - <DESC>Create source grid from direct v3.1 source inputs</DESC>
// <VERS>END OF VERSION: 0.1.0</VERS>
