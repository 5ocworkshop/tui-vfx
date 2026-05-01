// <FILE>crates/tui-vfx-compost/src/validation/fnc_validate_source_inputs.rs</FILE> - <DESC>Validate native v3.1 source inputs for direct rendering</DESC>
// <VERS>VERSION: 0.1.0</VERS>
// <WCTX>Compost currently materializes literal source.card text/color inputs only.</WCTX>
// <CLOG>0.1.0: INIT — add source input load validation.</CLOG>

use tui_vfx_contract::{SourceInstanceId, SourceSpec, ValueSource};

use crate::LoadError;

pub(crate) fn validate_source_inputs(
    source_id: &SourceInstanceId,
    source: &SourceSpec,
) -> Result<(), LoadError> {
    for (input, value_source) in &source.inputs {
        if !matches!(value_source, ValueSource::Literal { .. }) {
            return Err(LoadError::UnsupportedSourceInput {
                source_id: source_id.as_str().to_string(),
                input: input.as_str().to_string(),
                reason: "native compost rendering currently accepts literal source inputs only"
                    .to_string(),
            });
        }
    }
    for required in ["message", "width", "height", "foreground", "background"] {
        if !source
            .inputs
            .contains_key(&tui_vfx_contract::SourceInputId::new(required))
        {
            return Err(LoadError::UnsupportedSourceInput {
                source_id: source_id.as_str().to_string(),
                input: required.to_string(),
                reason: "required source.card input is missing".to_string(),
            });
        }
    }
    Ok(())
}

// <FILE>crates/tui-vfx-compost/src/validation/fnc_validate_source_inputs.rs</FILE> - <DESC>Validate native v3.1 source inputs for direct rendering</DESC>
// <VERS>END OF VERSION: 0.1.0</VERS>
