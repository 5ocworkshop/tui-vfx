// <FILE>crates/tui-vfx-compost/src/runtime/fnc_resolve_value_source.rs</FILE> - <DESC>Resolve canonical ValueSource values for native compost</DESC>
// <VERS>VERSION: 0.1.0</VERS>
// <WCTX>Runtime value resolution supports literal values and centralizes rejection for unresolved runtime sources.</WCTX>
// <CLOG>0.1.0: INIT — add literal resolver and shared non-literal rejection.</CLOG>

use tui_vfx_contract::ValueSource;

use crate::runtime::{ResolvedValue, RuntimeContext, RuntimeValueError};

pub(crate) fn resolve_value_source<'a>(
    source: &'a ValueSource,
    _context: &RuntimeContext,
) -> Result<ResolvedValue<'a>, RuntimeValueError> {
    match source {
        ValueSource::Literal { value } => Ok(ResolvedValue::literal(value)),
        ValueSource::Parameter { .. } => Err(RuntimeValueError::unsupported("parameter")),
        ValueSource::Signal { .. } => Err(RuntimeValueError::unsupported("signal")),
        ValueSource::GraphValue { .. } => Err(RuntimeValueError::unsupported("graphValue")),
        ValueSource::Map { .. } => Err(RuntimeValueError::unsupported("map")),
        ValueSource::SampledField { .. } => Err(RuntimeValueError::unsupported("sampledField")),
        ValueSource::SignalExpression { .. } => {
            Err(RuntimeValueError::unsupported("signalExpression"))
        }
        ValueSource::PhaseProgress { .. } => Err(RuntimeValueError::unsupported("phaseProgress")),
        ValueSource::Clock { .. } => Err(RuntimeValueError::unsupported("clock")),
    }
}

// <FILE>crates/tui-vfx-compost/src/runtime/fnc_resolve_value_source.rs</FILE> - <DESC>Resolve canonical ValueSource values for native compost</DESC>
// <VERS>END OF VERSION: 0.1.0</VERS>
