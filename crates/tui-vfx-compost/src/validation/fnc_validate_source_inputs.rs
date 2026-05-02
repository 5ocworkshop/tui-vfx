// <FILE>crates/tui-vfx-compost/src/validation/fnc_validate_source_inputs.rs</FILE> - <DESC>Validate native v3.1 source inputs for direct rendering</DESC>
// <VERS>VERSION: 0.7.0</VERS>
// <WCTX>Drop foreground/background from the hardcoded required list now that the source.card descriptor marks them optional with safe defaults; bare-card author shorthand without explicit colors must load.</WCTX>
// <CLOG>0.7.0: MINOR — validate source.text inputs with inferred optional width/height.
// 0.6.0: MINOR — only message/width/height are mandatory inputs at validation time; foreground and background fall back to descriptor defaults.</CLOG>

use std::collections::BTreeMap;

use tui_vfx_contract::{
    AssetId, AssetKind, AssetLocator, AssetSpec, SourceInputId, SourceInstanceId, SourceSpec,
    StructuredValue, Value,
};

use crate::LoadError;
use crate::runtime::{RuntimeContext, resolve_value_source};

const REQUIRED_SOURCE_CARD_INPUTS: [&str; 3] = ["message", "width", "height"];
const SOURCE_CARD_HEIGHT_MAX: i64 = 256;
const SOURCE_CARD_DESCRIPTOR: &str = "source.card";
const SOURCE_CARD_WIDTH_MAX: i64 = 512;
const SOURCE_PROCEDURAL_DESCRIPTOR: &str = "source.procedural";
const SOURCE_TEXT_DESCRIPTOR: &str = "source.text";
/// borderStyle enum values accepted by source.card. Mirrors the descriptor
/// declaration plus the `custom` author-shorthand which lands when the
/// border block carries a `frame:` glyph map without an explicit `type:`.
const SOURCE_CARD_BORDER_STYLES: [&str; 5] = ["none", "plain", "rounded", "double", "custom"];
const SOURCE_CARD_BORDER_TRIMS: [&str; 1] = ["none"];

pub(crate) fn validate_source_inputs(
    source_id: &SourceInstanceId,
    source: &SourceSpec,
    assets: &BTreeMap<AssetId, AssetSpec>,
    context: &RuntimeContext,
) -> Result<(), LoadError> {
    ensure_supported_source_descriptor(source_id, source.source_descriptor.as_str())?;
    for (input, value_source) in &source.inputs {
        let value = resolve_value_source(value_source, context)
            .map_err(|error| source_input_error(source_id, input.as_str(), error.reason()))?;
        match source.source_descriptor.as_str() {
            SOURCE_CARD_DESCRIPTOR => validate_source_card_input(source_id, input, value.value())?,
            SOURCE_PROCEDURAL_DESCRIPTOR => {
                validate_source_procedural_input(source_id, input, value.value())?
            }
            SOURCE_TEXT_DESCRIPTOR => validate_source_text_input(source_id, input, value.value())?,
            _ => unreachable!("descriptor support checked before source input validation"),
        }
    }
    match source.source_descriptor.as_str() {
        SOURCE_CARD_DESCRIPTOR => {
            for required in REQUIRED_SOURCE_CARD_INPUTS {
                if !source.inputs.contains_key(&SourceInputId::new(required)) {
                    return Err(source_input_error(
                        source_id,
                        required,
                        "required source.card input is missing",
                    ));
                }
            }
            validate_source_card_cross_fields(source_id, source, context)?;
        }
        SOURCE_TEXT_DESCRIPTOR if !source.inputs.contains_key(&SourceInputId::new("message")) => {
            return Err(source_input_error(
                source_id,
                "message",
                "required source.text input is missing",
            ));
        }
        SOURCE_TEXT_DESCRIPTOR => {}
        SOURCE_PROCEDURAL_DESCRIPTOR => {
            for required in ["generator", "width", "height"] {
                if !source.inputs.contains_key(&SourceInputId::new(required)) {
                    return Err(source_input_error(
                        source_id,
                        required,
                        "required source.procedural input is missing",
                    ));
                }
            }
            validate_source_procedural_cross_fields(source_id, source, assets, context)?;
        }
        _ => {}
    }
    Ok(())
}

fn ensure_supported_source_descriptor(
    source_id: &SourceInstanceId,
    descriptor: &str,
) -> Result<(), LoadError> {
    match descriptor {
        SOURCE_CARD_DESCRIPTOR | SOURCE_TEXT_DESCRIPTOR | SOURCE_PROCEDURAL_DESCRIPTOR => Ok(()),
        descriptor => Err(LoadError::UnsupportedSourceDescriptor {
            source_id: source_id.as_str().to_string(),
            descriptor: descriptor.to_string(),
            reason: "source materialization currently supports source.card, source.text, and source.procedural".to_string(),
        }),
    }
}

fn validate_source_procedural_cross_fields(
    source_id: &SourceInstanceId,
    source: &SourceSpec,
    assets: &BTreeMap<AssetId, AssetSpec>,
    context: &RuntimeContext,
) -> Result<(), LoadError> {
    let Some(generator_source) = source.inputs.get(&SourceInputId::new("generator")) else {
        return Ok(());
    };
    let generator_value = resolve_value_source(generator_source, context)
        .map_err(|error| source_input_error(source_id, "generator", error.reason()))?;
    let generator = match generator_value.value() {
        Value::String(value) | Value::Text(value) => value.as_str(),
        _ => return Ok(()),
    };

    match generator {
        "braille_flag_field" => {
            validate_braille_flag_field_params(source_id, source, assets, context)
        }
        unsupported => Err(source_input_error(
            source_id,
            "generator",
            format!("source.procedural generator `{unsupported}` is not supported"),
        )),
    }
}

fn validate_braille_flag_field_params(
    source_id: &SourceInstanceId,
    source: &SourceSpec,
    assets: &BTreeMap<AssetId, AssetSpec>,
    context: &RuntimeContext,
) -> Result<(), LoadError> {
    let Some(params_source) = source.inputs.get(&SourceInputId::new("params")) else {
        return Err(source_input_error(
            source_id,
            "params",
            "braille_flag_field requires params.asset",
        ));
    };
    let params_value = resolve_value_source(params_source, context)
        .map_err(|error| source_input_error(source_id, "params", error.reason()))?;
    let Value::Structured(params) = params_value.value() else {
        return Ok(());
    };
    let Some(asset_reference) = structured_object_string(params, &["asset"]) else {
        return Err(source_input_error(
            source_id,
            "params",
            "braille_flag_field params must include asset string",
        ));
    };
    let Some(asset_id) = asset_reference.strip_prefix("$asset:") else {
        return Err(source_input_error(
            source_id,
            "params",
            "braille_flag_field asset must use $asset:<id>",
        ));
    };
    validate_braille_asset(source_id, asset_id, assets)?;
    Ok(())
}

fn validate_braille_asset(
    source_id: &SourceInstanceId,
    asset_id: &str,
    assets: &BTreeMap<AssetId, AssetSpec>,
) -> Result<(), LoadError> {
    let Some(asset) = assets.get(&AssetId::new(asset_id)) else {
        return Err(source_input_error(
            source_id,
            "params",
            format!("unknown braille_flag_field asset `{asset_id}`"),
        ));
    };
    if !matches!(asset.kind, AssetKind::BrailleDotfield) {
        return Err(source_input_error(
            source_id,
            "params",
            format!("braille_flag_field asset `{asset_id}` must be brailleDotfield"),
        ));
    }
    if asset.format.as_str() != "tui-vfx.braille_flag_asset.v1" {
        return Err(source_input_error(
            source_id,
            "params",
            format!(
                "braille_flag_field asset `{asset_id}` has unsupported format `{}`",
                asset.format.as_str()
            ),
        ));
    }
    if !matches!(asset.locator, AssetLocator::Path { .. }) {
        return Err(source_input_error(
            source_id,
            "params",
            format!("braille_flag_field asset `{asset_id}` must use a path locator"),
        ));
    }
    Ok(())
}

fn validate_source_card_cross_fields(
    source_id: &SourceInstanceId,
    source: &SourceSpec,
    context: &RuntimeContext,
) -> Result<(), LoadError> {
    let Some(style_source) = source.inputs.get(&SourceInputId::new("borderStyle")) else {
        return Ok(());
    };
    let style_value = resolve_value_source(style_source, context)
        .map_err(|error| source_input_error(source_id, "borderStyle", error.reason()))?;
    let (Value::Enum(style) | Value::String(style)) = style_value.value() else {
        return Ok(());
    };
    if style != "custom" {
        return Ok(());
    }
    let Some(config_source) = source.inputs.get(&SourceInputId::new("borderConfig")) else {
        return Err(source_input_error(
            source_id,
            "borderConfig",
            "custom borderStyle requires borderConfig.frame",
        ));
    };
    let config_value = resolve_value_source(config_source, context)
        .map_err(|error| source_input_error(source_id, "borderConfig", error.reason()))?;
    let Value::Structured(config) = config_value.value() else {
        return Ok(());
    };
    if structured_object_array_len(config, &["frame", "corners"]) == Some(4)
        && structured_object_array_len(config, &["frame", "edges"]) == Some(4)
    {
        return Ok(());
    }
    Err(source_input_error(
        source_id,
        "borderConfig",
        "custom borderConfig.frame must include four corners and four edges",
    ))
}

fn structured_object_string<'a>(value: &'a StructuredValue, path: &[&str]) -> Option<&'a str> {
    let mut current = value;
    for key in path {
        let StructuredValue::Object(values) = current else {
            return None;
        };
        current = values.get(*key)?;
    }
    let StructuredValue::String(text) = current else {
        return None;
    };
    Some(text)
}

fn structured_object_array_len(value: &StructuredValue, path: &[&str]) -> Option<usize> {
    let mut current = value;
    for key in path {
        let StructuredValue::Object(values) = current else {
            return None;
        };
        current = values.get(*key)?;
    }
    let StructuredValue::Array(values) = current else {
        return None;
    };
    Some(values.len())
}

fn validate_source_procedural_input(
    source_id: &SourceInstanceId,
    input: &SourceInputId,
    value: &Value,
) -> Result<(), LoadError> {
    let valid = match input.as_str() {
        "generator" => matches!(value, Value::String(_) | Value::Text(_)),
        "seed" => matches!(value, Value::Integer(_)),
        "width" => source_dimension_in_range(value, SOURCE_CARD_WIDTH_MAX),
        "height" => source_dimension_in_range(value, SOURCE_CARD_HEIGHT_MAX),
        "params" => matches!(value, Value::Structured(_)),
        _ => {
            return Err(source_input_error(
                source_id,
                input.as_str(),
                "source.procedural input is not supported by current materialization",
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
            "source.procedural input has unsupported literal kind {:?}",
            value.kind()
        ),
    ))
}

fn validate_source_text_input(
    source_id: &SourceInstanceId,
    input: &SourceInputId,
    value: &Value,
) -> Result<(), LoadError> {
    let valid = match input.as_str() {
        "message" => matches!(value, Value::Text(_) | Value::String(_)),
        "width" => source_dimension_in_range(value, SOURCE_CARD_WIDTH_MAX),
        "height" => source_dimension_in_range(value, SOURCE_CARD_HEIGHT_MAX),
        "foreground" | "background" => matches!(value, Value::Color(_)),
        "bold" => matches!(value, Value::Boolean(_)),
        _ => {
            return Err(source_input_error(
                source_id,
                input.as_str(),
                "source.text input is not supported by current materialization",
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
            "source.text input has unsupported literal kind {:?}",
            value.kind()
        ),
    ))
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
        "borderStyle" => {
            matches!(value, Value::Enum(name) if SOURCE_CARD_BORDER_STYLES.contains(&name.as_str()))
        }
        "borderTrim" => {
            matches!(value, Value::Enum(name) if SOURCE_CARD_BORDER_TRIMS.contains(&name.as_str()))
        }
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
// <VERS>END OF VERSION: 0.7.0</VERS>
