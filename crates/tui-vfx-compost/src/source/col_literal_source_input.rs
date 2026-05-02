// <FILE>crates/tui-vfx-compost/src/source/col_literal_source_input.rs</FILE> - <DESC>Read resolved source inputs</DESC>
// <VERS>VERSION: 0.5.0</VERS>
// <WCTX>Source materialization uses the same runtime resolver as node validation.</WCTX>
// <CLOG>0.5.0: MINOR — add optional enum source input support for card border materialization.
// 0.4.0: MINOR — add optional boolean source input support for source.text styling.
// 0.3.0: MINOR — allow source.card materialization to apply descriptor-equivalent color defaults.</CLOG>
// 0.2.0: MINOR — resolve source.card inputs from RuntimeContext before materialization.</CLOG>

use std::collections::BTreeMap;

use tui_vfx_contract::{SourceInputId, StructuredValue, Value, ValueSource};
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

pub(crate) fn literal_bool(
    inputs: &BTreeMap<SourceInputId, ValueSource>,
    id: &str,
    context: &RuntimeContext,
) -> Result<bool, RenderError> {
    match resolved_input(inputs, id, context)? {
        Value::Boolean(value) => Ok(value),
        value => Err(RenderError::Unsupported(format!(
            "source input `{id}` expected boolean, found {:?}",
            value.kind()
        ))),
    }
}

pub(crate) fn literal_enum(
    inputs: &BTreeMap<SourceInputId, ValueSource>,
    id: &str,
    context: &RuntimeContext,
) -> Result<String, RenderError> {
    match resolved_input(inputs, id, context)? {
        Value::Enum(value) | Value::String(value) => Ok(value),
        value => Err(RenderError::Unsupported(format!(
            "source input `{id}` expected enum/string, found {:?}",
            value.kind()
        ))),
    }
}

pub(crate) fn literal_structured(
    inputs: &BTreeMap<SourceInputId, ValueSource>,
    id: &str,
    context: &RuntimeContext,
) -> Result<StructuredValue, RenderError> {
    match resolved_input(inputs, id, context)? {
        Value::Structured(value) => Ok(value),
        value => Err(RenderError::Unsupported(format!(
            "source input `{id}` expected structured, found {:?}",
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

pub(crate) fn optional_bool(
    inputs: &BTreeMap<SourceInputId, ValueSource>,
    id: &str,
    default: bool,
    context: &RuntimeContext,
) -> Result<bool, RenderError> {
    if !inputs.contains_key(&SourceInputId::new(id)) {
        return Ok(default);
    }
    literal_bool(inputs, id, context)
}

pub(crate) fn optional_enum(
    inputs: &BTreeMap<SourceInputId, ValueSource>,
    id: &str,
    default: &str,
    context: &RuntimeContext,
) -> Result<String, RenderError> {
    if !inputs.contains_key(&SourceInputId::new(id)) {
        return Ok(default.to_string());
    }
    literal_enum(inputs, id, context)
}

pub(crate) fn optional_structured(
    inputs: &BTreeMap<SourceInputId, ValueSource>,
    id: &str,
    context: &RuntimeContext,
) -> Result<Option<StructuredValue>, RenderError> {
    if !inputs.contains_key(&SourceInputId::new(id)) {
        return Ok(None);
    }
    literal_structured(inputs, id, context).map(Some)
}

// <FILE>crates/tui-vfx-compost/src/source/col_literal_source_input.rs</FILE> - <DESC>Read resolved source inputs</DESC>
// <VERS>END OF VERSION: 0.5.0</VERS>
