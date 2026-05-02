// <FILE>crates/tui-vfx-compost/src/source/col_literal_source_input.rs</FILE> - <DESC>Read resolved source inputs</DESC>
// <VERS>VERSION: 0.3.0</VERS>
// <WCTX>Source materialization uses the same runtime resolver as node validation.</WCTX>
// <CLOG>0.3.0: MINOR — allow source.card materialization to apply descriptor-equivalent color defaults.</CLOG>
// 0.2.0: MINOR — resolve source.card inputs from RuntimeContext before materialization.</CLOG>

use std::collections::BTreeMap;

use tui_vfx_contract::{SourceInputId, Value, ValueSource};
use tui_vfx_types::Color;

use crate::RenderError;
use crate::runtime::{RuntimeContext, resolve_value_source};

fn resolved_input(
    inputs: &BTreeMap<SourceInputId, ValueSource>,
    id: &str,
    context: &RuntimeContext,
) -> Result<Value, RenderError> {
    match inputs.get(&SourceInputId::new(id)) {
        Some(source) => resolve_value_source(source, context)
            .map(|value| value.value().clone())
            .map_err(|error| RenderError::Unsupported(error.reason())),
        None => Err(RenderError::Unsupported(format!(
            "load-validated source is missing input `{id}`"
        ))),
    }
}

pub(crate) fn literal_text(
    inputs: &BTreeMap<SourceInputId, ValueSource>,
    id: &str,
    context: &RuntimeContext,
) -> Result<String, RenderError> {
    match resolved_input(inputs, id, context)? {
        Value::Text(value) | Value::String(value) => Ok(value),
        value => Err(RenderError::Unsupported(format!(
            "source input `{id}` expected text/string, found {:?}",
            value.kind()
        ))),
    }
}

pub(crate) fn literal_integer(
    inputs: &BTreeMap<SourceInputId, ValueSource>,
    id: &str,
    context: &RuntimeContext,
) -> Result<usize, RenderError> {
    match resolved_input(inputs, id, context)? {
        Value::Integer(value) if value >= 0 => Ok(value as usize),
        value => Err(RenderError::Unsupported(format!(
            "source input `{id}` expected non-negative integer, found {:?}",
            value.kind()
        ))),
    }
}

pub(crate) fn literal_color(
    inputs: &BTreeMap<SourceInputId, ValueSource>,
    id: &str,
    context: &RuntimeContext,
) -> Result<Color, RenderError> {
    match resolved_input(inputs, id, context)? {
        Value::Color(value) => Ok(value),
        value => Err(RenderError::Unsupported(format!(
            "source input `{id}` expected color, found {:?}",
            value.kind()
        ))),
    }
}

pub(crate) fn optional_color(
    inputs: &BTreeMap<SourceInputId, ValueSource>,
    id: &str,
    default: Color,
    context: &RuntimeContext,
) -> Result<Color, RenderError> {
    if !inputs.contains_key(&SourceInputId::new(id)) {
        return Ok(default);
    }
    literal_color(inputs, id, context)
}

// <FILE>crates/tui-vfx-compost/src/source/col_literal_source_input.rs</FILE> - <DESC>Read resolved source inputs</DESC>
// <VERS>END OF VERSION: 0.3.0</VERS>
