// <FILE>crates/tui-vfx-compost/src/shaders/col_shader_input.rs</FILE> - <DESC>Read resolved canonical shader node inputs</DESC>
// <VERS>VERSION: 0.2.0</VERS>
// <WCTX>Runtime reads canonical NodeSpec inputs through the compost resolver instead of assuming literals.</WCTX>
// <CLOG>0.2.0: MINOR — resolve non-literal shader inputs from RuntimeContext.</CLOG>

use tui_vfx_contract::{EffectInputId, GradientSpec, NodeSpec, Value, ValueSource};

use crate::runtime::{RuntimeContext, resolve_value_source};

fn resolved_input(node: &NodeSpec, id: &str, context: &RuntimeContext) -> Value {
    match node.inputs.get(&EffectInputId::new(id)) {
        Some(source) => resolve_value_source(source, context)
            .map(|resolved| resolved.value().clone())
            .unwrap_or_else(|error| {
                panic!(
                    "load-validated compost node `{}` could not resolve `{}`: {}",
                    node.effect.as_str(),
                    id,
                    error.reason()
                )
            }),
        None => panic!(
            "load-validated compost node `{}` missing input `{}`",
            node.effect.as_str(),
            id
        ),
    }
}

pub(crate) fn number_input(node: &NodeSpec, id: &str, context: &RuntimeContext) -> f64 {
    resolved_input(node, id, context)
        .as_range_number()
        .unwrap_or(0.0)
}

pub(crate) fn enum_input(node: &NodeSpec, id: &str, context: &RuntimeContext) -> String {
    resolved_input(node, id, context)
        .as_enum_value()
        .unwrap_or("")
        .to_string()
}

pub(crate) fn gradient_input(
    node: &NodeSpec,
    id: &str,
    context: &RuntimeContext,
) -> Option<GradientSpec> {
    match node.inputs.get(&EffectInputId::new(id)) {
        Some(ValueSource::Literal {
            value: Value::Gradient(gradient),
        }) => Some(gradient.clone()),
        Some(source) => match resolve_value_source(source, context).ok()?.value() {
            Value::Gradient(gradient) => Some(gradient.clone()),
            _ => None,
        },
        None => None,
    }
}

// <FILE>crates/tui-vfx-compost/src/shaders/col_shader_input.rs</FILE> - <DESC>Read resolved canonical shader node inputs</DESC>
// <VERS>END OF VERSION: 0.2.0</VERS>
