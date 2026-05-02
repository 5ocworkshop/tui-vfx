// <FILE>crates/tui-vfx-compost/src/runtime/fnc_resolve_numeric_value_source.rs</FILE> - <DESC>Resolve numeric nested value sources</DESC>
// <VERS>VERSION: 0.1.0</VERS>
// <WCTX>Map and sampled-field sources share one numeric nested-source check.</WCTX>
// <CLOG>0.1.0: INIT — split numeric extraction from value-source dispatch.</CLOG>

use tui_vfx_contract::ValueSource;

use crate::runtime::{RuntimeContext, RuntimeValueError, resolve_value_source};

pub(crate) fn resolve_numeric_value_source(
    source: &ValueSource,
    context: &RuntimeContext,
    source_kind: &'static str,
) -> Result<f64, RuntimeValueError> {
    resolve_value_source(source, context)?
        .value()
        .as_range_number()
        .ok_or_else(|| RuntimeValueError::unavailable(source_kind, "nested value is not numeric"))
}

// <FILE>crates/tui-vfx-compost/src/runtime/fnc_resolve_numeric_value_source.rs</FILE> - <DESC>Resolve numeric nested value sources</DESC>
// <VERS>END OF VERSION: 0.1.0</VERS>
