// <FILE>crates/tui-vfx-compost/src/source/col_literal_source_input.rs</FILE> - <DESC>Read load-validated literal source inputs</DESC>
// <VERS>VERSION: 0.1.0</VERS>
// <WCTX>Runtime reads already load-validated canonical source fields directly.</WCTX>
// <CLOG>0.1.0: INIT — add literal source input accessors.</CLOG>

use std::collections::BTreeMap;

use tui_vfx_contract::{SourceInputId, Value, ValueSource};
use tui_vfx_types::Color;

use crate::RenderError;

fn literal_input<'a>(
    inputs: &'a BTreeMap<SourceInputId, ValueSource>,
    id: &str,
) -> Result<&'a Value, RenderError> {
    match inputs.get(&SourceInputId::new(id)) {
        Some(ValueSource::Literal { value }) => Ok(value),
        _ => Err(RenderError::Unsupported(format!(
            "load-validated source is missing literal `{id}`"
        ))),
    }
}

pub(crate) fn literal_text(
    inputs: &BTreeMap<SourceInputId, ValueSource>,
    id: &str,
) -> Result<String, RenderError> {
    match literal_input(inputs, id)? {
        Value::Text(value) | Value::String(value) => Ok(value.clone()),
        value => Err(RenderError::Unsupported(format!(
            "source input `{id}` expected text/string, found {:?}",
            value.kind()
        ))),
    }
}

pub(crate) fn literal_integer(
    inputs: &BTreeMap<SourceInputId, ValueSource>,
    id: &str,
) -> Result<usize, RenderError> {
    match literal_input(inputs, id)? {
        Value::Integer(value) if *value >= 0 => Ok(*value as usize),
        value => Err(RenderError::Unsupported(format!(
            "source input `{id}` expected non-negative integer, found {:?}",
            value.kind()
        ))),
    }
}

pub(crate) fn literal_color(
    inputs: &BTreeMap<SourceInputId, ValueSource>,
    id: &str,
) -> Result<Color, RenderError> {
    match literal_input(inputs, id)? {
        Value::Color(value) => Ok(*value),
        value => Err(RenderError::Unsupported(format!(
            "source input `{id}` expected color, found {:?}",
            value.kind()
        ))),
    }
}

// <FILE>crates/tui-vfx-compost/src/source/col_literal_source_input.rs</FILE> - <DESC>Read load-validated literal source inputs</DESC>
// <VERS>END OF VERSION: 0.1.0</VERS>
