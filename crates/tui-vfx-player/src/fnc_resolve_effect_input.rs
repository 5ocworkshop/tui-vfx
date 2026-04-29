// <FILE>crates/tui-vfx-player/src/fnc_resolve_effect_input.rs</FILE> - <DESC>Resolve graph node effect inputs</DESC>
// <VERS>VERSION: 0.2.0</VERS>
// <WCTX>Primitive adapter work: share typed effect input resolution.</WCTX>
// <CLOG>0.2.0: MINOR — add color input helpers for styled primitive adapters.
// 0.1.0: INIT — add numeric and integer node input helpers.</CLOG>

use tui_vfx_contract::{EffectInputId, NodeSpec, Value};

/// RGBA color resolved from a canonical effect input.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) struct ResolvedColor {
    pub(crate) r: u8,
    pub(crate) g: u8,
    pub(crate) b: u8,
    pub(crate) a: u8,
}

impl ResolvedColor {
    /// Build a resolved color from channel values.
    pub(crate) const fn new(r: u8, g: u8, b: u8, a: u8) -> Self {
        Self { r, g, b, a }
    }

    /// Build an opaque RGB resolved color.
    pub(crate) const fn rgb(r: u8, g: u8, b: u8) -> Self {
        Self::new(r, g, b, 255)
    }

    /// Linear interpolation between two colors.
    pub(crate) fn lerp(self, other: Self, t: f32) -> Self {
        let t = t.clamp(0.0, 1.0);
        let inv_t = 1.0 - t;
        Self::new(
            (self.r as f32 * inv_t + other.r as f32 * t + 0.5) as u8,
            (self.g as f32 * inv_t + other.g as f32 * t + 0.5) as u8,
            (self.b as f32 * inv_t + other.b as f32 * t + 0.5) as u8,
            (self.a as f32 * inv_t + other.a as f32 * t + 0.5) as u8,
        )
    }
}

use crate::{PlayerSampleRequest, fnc_resolve_value_source::resolve_value_source};

/// Resolve an effect input as a floating-point number.
pub(crate) fn resolve_effect_number(
    node: &NodeSpec,
    request: &PlayerSampleRequest,
    input_id: &str,
    fallback: f64,
) -> f64 {
    match node
        .inputs
        .get(&EffectInputId::new(input_id))
        .and_then(|source| resolve_value_source(source, &request.signals))
    {
        Some(Value::Number(value) | Value::Duration(value)) => value,
        Some(Value::Integer(value)) => value as f64,
        _ => fallback,
    }
}

/// Resolve an effect input as an integer.
pub(crate) fn resolve_effect_integer(
    node: &NodeSpec,
    request: &PlayerSampleRequest,
    input_id: &str,
    fallback: i64,
) -> i64 {
    match node
        .inputs
        .get(&EffectInputId::new(input_id))
        .and_then(|source| resolve_value_source(source, &request.signals))
    {
        Some(Value::Integer(value)) => value,
        Some(Value::Number(value) | Value::Duration(value)) => value.round() as i64,
        _ => fallback,
    }
}

/// Resolve an effect input as an RGBA color.
pub(crate) fn resolve_effect_color(
    node: &NodeSpec,
    request: &PlayerSampleRequest,
    input_id: &str,
    fallback: ResolvedColor,
) -> ResolvedColor {
    match node
        .inputs
        .get(&EffectInputId::new(input_id))
        .and_then(|source| resolve_value_source(source, &request.signals))
    {
        Some(Value::Color(value)) => ResolvedColor::new(value.r, value.g, value.b, value.a),
        _ => fallback,
    }
}

// <FILE>crates/tui-vfx-player/src/fnc_resolve_effect_input.rs</FILE> - <DESC>Resolve graph node effect inputs</DESC>
// <VERS>END OF VERSION: 0.2.0</VERS>
