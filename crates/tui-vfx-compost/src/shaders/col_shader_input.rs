// <FILE>crates/tui-vfx-compost/src/shaders/col_shader_input.rs</FILE> - <DESC>Read load-validated canonical shader node literal inputs</DESC>
// <VERS>VERSION: 0.1.0</VERS>
// <WCTX>Runtime reads canonical NodeSpec inputs directly; this is not a legacy field-name mapping layer.</WCTX>
// <CLOG>0.1.0: INIT — add literal shader input accessors.</CLOG>

use tui_vfx_contract::{EffectInputId, GradientSpec, NodeSpec, Value, ValueSource};

fn literal_input<'a>(node: &'a NodeSpec, id: &str) -> &'a Value {
    match node.inputs.get(&EffectInputId::new(id)) {
        Some(ValueSource::Literal { value }) => value,
        _ => panic!(
            "load-validated compost node `{}` missing literal `{}`",
            node.effect.as_str(),
            id
        ),
    }
}

pub(crate) fn number_input(node: &NodeSpec, id: &str) -> f64 {
    literal_input(node, id).as_range_number().unwrap_or(0.0)
}

pub(crate) fn enum_input<'a>(node: &'a NodeSpec, id: &str) -> &'a str {
    literal_input(node, id).as_enum_value().unwrap_or("")
}

pub(crate) fn gradient_input<'a>(node: &'a NodeSpec, id: &str) -> Option<&'a GradientSpec> {
    match node.inputs.get(&EffectInputId::new(id)) {
        Some(ValueSource::Literal {
            value: Value::Gradient(gradient),
        }) => Some(gradient),
        _ => None,
    }
}

// <FILE>crates/tui-vfx-compost/src/shaders/col_shader_input.rs</FILE> - <DESC>Read load-validated canonical shader node literal inputs</DESC>
// <VERS>END OF VERSION: 0.1.0</VERS>
