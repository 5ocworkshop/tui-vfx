// <FILE>crates/tui-vfx-compost/src/runtime/fnc_resolve_value_source.rs</FILE> - <DESC>Resolve canonical ValueSource values for native compost</DESC>
// <VERS>VERSION: 0.3.0</VERS>
// <WCTX>Runtime value dispatch delegates specialized math to OFPF-sized helpers.</WCTX>
// <CLOG>0.3.0: MINOR — split map, sampled field, expression, and clock resolution helpers.</CLOG>

use tui_vfx_contract::{Value, ValueSource};

use crate::runtime::{
    ResolvedValue, RuntimeContext, RuntimeValueError, evaluate_signal_expression,
    resolve_clock_value_source, resolve_mapped_value_source, resolve_sampled_field_source,
};

pub(crate) fn resolve_value_source<'a>(
    source: &'a ValueSource,
    context: &RuntimeContext,
) -> Result<ResolvedValue<'a>, RuntimeValueError> {
    match source {
        ValueSource::Literal { value } => Ok(ResolvedValue::literal(value)),
        ValueSource::Parameter { id, fallback } => resolve_optional_context_value(
            context.parameter(id),
            fallback,
            "parameter",
            "no runtime value or fallback is available",
        ),
        ValueSource::Signal { id, fallback } => resolve_optional_context_value(
            context.signal(id),
            fallback,
            "signal",
            "no runtime value or fallback is available",
        ),
        ValueSource::GraphValue { id, fallback } => resolve_optional_context_value(
            context.graph_value(id),
            fallback,
            "graphValue",
            "no graph value or fallback is available",
        ),
        ValueSource::Map {
            from,
            input,
            output,
            clamp,
        } => resolve_mapped_value_source(from, *input, *output, *clamp, context),
        ValueSource::SampledField {
            field,
            x,
            y,
            fallback,
        } => resolve_sampled_field_source(field, x, y, fallback, context),
        ValueSource::SignalExpression { expression, .. } => {
            Ok(ResolvedValue::owned(Value::Number(
                evaluate_signal_expression(expression, context.effective_loop_t()),
            )))
        }
        ValueSource::PhaseProgress { phase } => Ok(ResolvedValue::owned(Value::Number(
            context.phase_progress(*phase),
        ))),
        ValueSource::Clock { clock } => Ok(resolve_clock_value_source(*clock, context)),
    }
}

fn resolve_optional_context_value<'a>(
    context_value: Option<&Value>,
    fallback: &Option<Value>,
    source_kind: &'static str,
    unavailable: &'static str,
) -> Result<ResolvedValue<'a>, RuntimeValueError> {
    context_value
        .cloned()
        .or_else(|| fallback.clone())
        .map(ResolvedValue::owned)
        .ok_or_else(|| RuntimeValueError::unavailable(source_kind, unavailable))
}

#[cfg(test)]
mod tests {
    use tui_vfx_contract::{LifecyclePhase, ValueSource};

    use crate::render::SampleContext;
    use crate::runtime::{RuntimeContext, resolve_value_source};

    #[test]
    fn phase_progress_only_reports_matching_lifecycle_phase() {
        let context = RuntimeContext::from_sample(
            &SampleContext::new(0.6).with_lifecycle_phase(LifecyclePhase::Enter),
        );

        assert_eq!(phase_value(&context, LifecyclePhase::Enter), 0.6);
        assert_eq!(phase_value(&context, LifecyclePhase::Exit), 0.0);
    }

    fn phase_value(context: &RuntimeContext, phase: LifecyclePhase) -> f64 {
        resolve_value_source(&ValueSource::PhaseProgress { phase }, context)
            .expect("resolve phase progress")
            .value()
            .as_range_number()
            .expect("phase progress resolves number")
    }
}

// <FILE>crates/tui-vfx-compost/src/runtime/fnc_resolve_value_source.rs</FILE> - <DESC>Resolve canonical ValueSource values for native compost</DESC>
// <VERS>END OF VERSION: 0.3.0</VERS>
