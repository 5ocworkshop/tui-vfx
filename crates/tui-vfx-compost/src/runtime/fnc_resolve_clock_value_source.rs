// <FILE>crates/tui-vfx-compost/src/runtime/fnc_resolve_clock_value_source.rs</FILE> - <DESC>Resolve elapsed clock value sources</DESC>
// <VERS>VERSION: 0.1.0</VERS>
// <WCTX>Clock sources return elapsed seconds, never normalized phase or loop coordinates.</WCTX>
// <CLOG>0.1.0: INIT — split recipe, phase, and loop elapsed-clock semantics.</CLOG>

use tui_vfx_contract::{ClockValueSource, Value};

use crate::runtime::{ResolvedValue, RuntimeContext};

pub(crate) fn resolve_clock_value_source<'a>(
    clock: ClockValueSource,
    context: &RuntimeContext,
) -> ResolvedValue<'a> {
    ResolvedValue::owned(Value::Number(seconds_for_clock(clock, context)))
}

fn seconds_for_clock(clock: ClockValueSource, context: &RuntimeContext) -> f64 {
    let milliseconds = match clock {
        ClockValueSource::RecipeSeconds => context.absolute_time_ms,
        ClockValueSource::PhaseSeconds => context.phase_time_ms,
        ClockValueSource::LoopSeconds => context.loop_time_ms,
    };
    milliseconds.map(|ms| ms as f64 / 1000.0).unwrap_or(0.0)
}

#[cfg(test)]
mod tests {
    use tui_vfx_contract::ClockValueSource;

    use crate::render::SampleContext;
    use crate::runtime::{RuntimeContext, resolve_clock_value_source};

    #[test]
    fn clock_sources_use_distinct_elapsed_fields() {
        let context = RuntimeContext::from_sample(
            &SampleContext::new(0.9)
                .with_loop_t(0.8)
                .with_absolute_time_ms(2_000)
                .with_phase_time_ms(750)
                .with_loop_time_ms(125),
        );

        assert_eq!(clock(&context, ClockValueSource::RecipeSeconds), 2.0);
        assert_eq!(clock(&context, ClockValueSource::PhaseSeconds), 0.75);
        assert_eq!(clock(&context, ClockValueSource::LoopSeconds), 0.125);
    }

    fn clock(context: &RuntimeContext, source: ClockValueSource) -> f64 {
        resolve_clock_value_source(source, context)
            .value()
            .as_range_number()
            .expect("clock resolves number")
    }
}

// <FILE>crates/tui-vfx-compost/src/runtime/fnc_resolve_clock_value_source.rs</FILE> - <DESC>Resolve elapsed clock value sources</DESC>
// <VERS>END OF VERSION: 0.1.0</VERS>
