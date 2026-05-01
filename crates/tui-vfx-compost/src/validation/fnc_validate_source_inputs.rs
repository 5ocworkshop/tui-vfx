// <FILE>crates/tui-vfx-compost/src/validation/fnc_validate_source_inputs.rs</FILE> - <DESC>Validate native v3.1 source inputs for direct rendering</DESC>
// <VERS>VERSION: 0.4.0</VERS>
// <WCTX>Compost validates source.card inputs through the native runtime value resolver before materialization.</WCTX>
// <CLOG>0.4.0: MINOR — route source ValueSource handling through the runtime value resolver.
// 0.3.0: PATCH — enforce source.card descriptor dimension bounds during load validation.
// 0.2.1: PATCH — centralize source.card constants and source input error construction.</CLOG>

use tui_vfx_contract::{SourceInputId, SourceInstanceId, SourceSpec, Value};

use crate::LoadError;
use crate::runtime::{RuntimeContext, resolve_value_source};

const REQUIRED_SOURCE_CARD_INPUTS: [&str; 5] =
    ["message", "width", "height", "foreground", "background"];
const SOURCE_CARD_HEIGHT_MAX: i64 = 256;
const SOURCE_CARD_DESCRIPTOR: &str = "source.card";
const SOURCE_CARD_WIDTH_MAX: i64 = 512;

pub(crate) fn validate_source_inputs(
    source_id: &SourceInstanceId,
    source: &SourceSpec,
) -> Result<(), LoadError> {
    if source.source_descriptor.as_str() != SOURCE_CARD_DESCRIPTOR {
        return Err(LoadError::UnsupportedSourceDescriptor {
            source_id: source_id.as_str().to_string(),
            descriptor: source.source_descriptor.as_str().to_string(),
            reason: "source materialization currently supports source.card only".to_string(),
        });
    }

    let context = RuntimeContext::load_time();
    for (input, value_source) in &source.inputs {
        let value = resolve_value_source(value_source, &context)
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
        "borderStyle" | "borderTrim" => matches!(value, Value::Enum(value) if value == "none"),
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
