// <FILE>crates/tui-vfx-compositor-next/src/v31/validation/fnc_validate_direct_source_inputs.rs</FILE> - <DESC>Validate direct v3.1 source inputs</DESC>
// <VERS>VERSION: 0.1.0</VERS>
// <WCTX>Source validation is separate from shader validation so recipe loading stays file-centric.</WCTX>
// <CLOG>0.1.0: INIT — extract direct source input validation.</CLOG>

use tui_vfx_contract::{SourceInputId, SourceInstanceId, SourceSpec, Value, ValueSource};

use crate::v31::V31LoadError;

pub(crate) fn validate_direct_source_inputs(
    source_id: &SourceInstanceId,
    source: &SourceSpec,
) -> Result<(), V31LoadError> {
    let has_message = literal_source_input(source, "message").is_ok();
    let has_text = literal_source_input(source, "text").is_ok();
    if !has_message && !has_text {
        return Err(source_input_error(
            source_id,
            source,
            "message",
            "Direct v3.1 rendering requires a literal message/text source input.",
        ));
    }

    for input in source.inputs.keys() {
        match input.as_str() {
            "message" | "text" | "width" | "height" | "foreground" | "background"
            | "borderStyle" | "borderTrim" => {
                literal_source_input(source, input.as_str()).map_err(|reason| {
                    source_input_error(source_id, source, input.as_str(), &reason)
                })?;
            }
            other => {
                return Err(source_input_error(
                    source_id,
                    source,
                    other,
                    "Direct v3.1 source rendering only supports literal card/text styling inputs.",
                ));
            }
        }
    }
    Ok(())
}

fn literal_source_input<'a>(source: &'a SourceSpec, id: &str) -> Result<&'a Value, String> {
    match source.inputs.get(&SourceInputId::new(id)) {
        Some(ValueSource::Literal { value }) => Ok(value),
        Some(_) => Err(format!(
            "Direct v3.1 rendering requires literal source input `{id}`."
        )),
        None => Err(format!(
            "Direct v3.1 rendering requires source input `{id}`."
        )),
    }
}

fn source_input_error(
    source_id: &SourceInstanceId,
    source: &SourceSpec,
    input: &str,
    reason: &str,
) -> V31LoadError {
    V31LoadError::UnsupportedSourceInput {
        source_id: source_id.as_str().to_string(),
        source: source.source.as_str().to_string(),
        input: input.to_string(),
        reason: reason.to_string(),
    }
}

// <FILE>crates/tui-vfx-compositor-next/src/v31/validation/fnc_validate_direct_source_inputs.rs</FILE> - <DESC>Validate direct v3.1 source inputs</DESC>
// <VERS>END OF VERSION: 0.1.0</VERS>
