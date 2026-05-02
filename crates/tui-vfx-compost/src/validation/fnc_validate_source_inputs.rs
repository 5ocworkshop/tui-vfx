// <FILE>crates/tui-vfx-compost/src/validation/fnc_validate_source_inputs.rs</FILE> - <DESC>Validate native v3.1 source inputs for direct rendering</DESC>
// <VERS>VERSION: 0.5.0</VERS>
// <WCTX>Accept the full set of borderStyle values declared by the source.card descriptor and the borderConfig structured payload that preserves title/titleAlign/frame extras through canonicalize.</WCTX>
// <CLOG>0.5.0: MINOR — accept all source.card descriptor borderStyle values plus the borderConfig structured input emitted for object-form borders.</CLOG>

use tui_vfx_contract::{SourceInputId, SourceInstanceId, SourceSpec, Value};

use crate::LoadError;
use crate::runtime::{RuntimeContext, resolve_value_source};

const REQUIRED_SOURCE_CARD_INPUTS: [&str; 5] =
    ["message", "width", "height", "foreground", "background"];
const SOURCE_CARD_HEIGHT_MAX: i64 = 256;
const SOURCE_CARD_DESCRIPTOR: &str = "source.card";
const SOURCE_CARD_WIDTH_MAX: i64 = 512;
/// borderStyle enum values accepted by source.card. Mirrors the descriptor
/// declaration plus the `custom` author-shorthand which lands when the
/// border block carries a `frame:` glyph map without an explicit `type:`.
const SOURCE_CARD_BORDER_STYLES: [&str; 5] = ["none", "plain", "rounded", "double", "custom"];
const SOURCE_CARD_BORDER_TRIMS: [&str; 1] = ["none"];

pub(crate) fn validate_source_inputs(
    source_id: &SourceInstanceId,
    source: &SourceSpec,
    context: &RuntimeContext,
) -> Result<(), LoadError> {
    if source.source_descriptor.as_str() != SOURCE_CARD_DESCRIPTOR {
        return Err(LoadError::UnsupportedSourceDescriptor {
            source_id: source_id.as_str().to_string(),
            descriptor: source.source_descriptor.as_str().to_string(),
            reason: "source materialization currently supports source.card only".to_string(),
        });
    }

    for (input, value_source) in &source.inputs {
        let value = resolve_value_source(value_source, context)
            .map_err(|error| source_input_error(source_id, input.as_str(), error.reason()))?;
        validate_source_card_input(source_id, input, value.value())?;
    }
    for required in REQUIRED_SOURCE_CARD_INPUTS {
        if !source.inputs.contains_key(&SourceInputId::new(required)) {
            return Err(source_input_error(
                source_id,
                required,
                "required source.card input is missing",
            ));
        }
    }
    Ok(())
}

fn validate_source_card_input(
    source_id: &SourceInstanceId,
    input: &SourceInputId,
    value: &Value,
) -> Result<(), LoadError> {
    let valid = match input.as_str() {
        "message" => matches!(value, Value::Text(_) | Value::String(_)),
        "width" => source_dimension_in_range(value, SOURCE_CARD_WIDTH_MAX),
        "height" => source_dimension_in_range(value, SOURCE_CARD_HEIGHT_MAX),
        "foreground" | "background" => matches!(value, Value::Color(_)),
        "borderStyle" => matches!(value, Value::Enum(name) if SOURCE_CARD_BORDER_STYLES.contains(&name.as_str())),
        "borderTrim" => matches!(value, Value::Enum(name) if SOURCE_CARD_BORDER_TRIMS.contains(&name.as_str())),
        "borderConfig" => matches!(value, Value::Structured(_)),
        "bold" => matches!(value, Value::Boolean(_)),
        _ => {
            return Err(source_input_error(
                source_id,
                input.as_str(),
                "source.card input is not supported by current materialization",
            ));
        }
    };

    if valid {
        return Ok(());
    }

    Err(source_input_error(
        source_id,
        input.as_str(),
        format!(
            "source.card input has unsupported literal kind {:?}",
            value.kind()
        ),
    ))
}

fn source_dimension_in_range(value: &Value, max: i64) -> bool {
    matches!(value, Value::Integer(size) if (1..=max).contains(size))
}

fn source_input_error(
    source_id: &SourceInstanceId,
    input: &str,
    reason: impl Into<String>,
) -> LoadError {
    LoadError::UnsupportedSourceInput {
        source_id: source_id.as_str().to_string(),
        input: input.to_string(),
        reason: reason.into(),
    }
}

// <FILE>crates/tui-vfx-compost/src/validation/fnc_validate_source_inputs.rs</FILE> - <DESC>Validate native v3.1 source inputs for direct rendering</DESC>
// <VERS>END OF VERSION: 0.4.0</VERS>
