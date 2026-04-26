// <FILE>crates/tui-vfx-style/src/traits/cls_shader_context.rs</FILE> - <DESC>Context passed to StyleShader for spatial effects; composes VfxCellContext via Deref. Hosts ShaderRuntimeParams and its RuntimeParamsRead impl that bridges into tui-vfx-core's bindable family.</DESC>
// <VERS>VERSION: 2.1.0</VERS>
// <WCTX>Buy-once sweep finding 1.2.A Phase 1.5 — implement tui_vfx_core::bindable::RuntimeParamsRead for ShaderRuntimeParams so the inherent evaluate methods on each VfxBindable specialization can consult this map without tui-vfx-core having to depend on tui-vfx-style.</WCTX>
// <CLOG>2.1.0: implement RuntimeParamsRead for ShaderRuntimeParams (forwards to existing get_u16 / get_text / get_f32 inherent methods); fix file-path drift in metadata header.</CLOG>

use mixed_signals::traits::Phase;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use std::ops::Deref;
use std::sync::Arc;
use tui_vfx_types::{Color, RoleMap, RoleTag, VfxCellContext};

/// Runtime scalar value exposed to spatial shaders during render.
///
/// The `Rgb` variant carries an opaque RGB triple (alpha is always 255);
/// use it when a recipe binding needs to supply a concrete color at
/// runtime — e.g. `FadeToCanvas.canvas_color_binding` reading the current
/// terminal background, or a tint that tracks a theme-mode flip. Variant
/// order matters for `serde(untagged)` deserialization: scalar variants
/// come first so single JSON numbers / booleans / strings hit their
/// expected match arm, and `Rgb` is last so it only matches JSON objects
/// shaped like `{"r": <u8>, "g": <u8>, "b": <u8>}`.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ShaderRuntimeParamValue {
    Integer(i64),
    Float(f64),
    Boolean(bool),
    Text(String),
    /// Opaque RGB triple. Deserializes from `{"r": ..., "g": ..., "b": ...}`.
    Rgb {
        /// Red channel (0-255).
        r: u8,
        /// Green channel (0-255).
        g: u8,
        /// Blue channel (0-255).
        b: u8,
    },
}

impl ShaderRuntimeParamValue {
    /// Return a stable human-readable kind for this runtime value.
    pub fn kind_name(&self) -> &'static str {
        match self {
            Self::Integer(_) => "integer",
            Self::Float(_) => "float",
            Self::Boolean(_) => "boolean",
            Self::Text(_) => "text",
            Self::Rgb { .. } => "rgb",
        }
    }

    /// Attempt to coerce this runtime value to f32.
    pub fn as_f32(&self) -> Option<f32> {
        match self {
            Self::Integer(value) => Some(*value as f32),
            Self::Float(value) if value.is_finite() => Some(*value as f32),
            _ => None,
        }
    }

    /// Attempt to coerce this runtime value to u16.
    pub fn as_u16(&self) -> Option<u16> {
        match self {
            Self::Integer(value) => u16::try_from(*value).ok(),
            Self::Float(value) if value.is_finite() && *value >= 0.0 => {
                let rounded = value.round();
                if rounded <= u16::MAX as f64 {
                    Some(rounded as u16)
                } else {
                    None
                }
            }
            _ => None,
        }
    }

    /// Attempt to read this runtime value as an opaque RGB `Color`.
    /// Returns `None` for non-Rgb variants.
    pub fn as_color(&self) -> Option<Color> {
        match self {
            Self::Rgb { r, g, b } => Some(Color::rgb(*r, *g, *b)),
            _ => None,
        }
    }

    /// Attempt to read this runtime value as a string slice. Returns
    /// `None` for non-Text variants. Used by `BindableString::Binding`
    /// resolution and any future bindable that consumes string-shaped
    /// host parameters (font names, asset names, locale tokens, etc.).
    pub fn as_text(&self) -> Option<&str> {
        match self {
            Self::Text(value) => Some(value.as_str()),
            _ => None,
        }
    }
}

impl From<&ShaderRuntimeParamValue> for serde_json::Value {
    fn from(value: &ShaderRuntimeParamValue) -> Self {
        match value {
            ShaderRuntimeParamValue::Integer(inner) => serde_json::Value::from(*inner),
            ShaderRuntimeParamValue::Float(inner) => serde_json::Value::from(*inner),
            ShaderRuntimeParamValue::Boolean(inner) => serde_json::Value::from(*inner),
            ShaderRuntimeParamValue::Text(inner) => serde_json::Value::from(inner.clone()),
            ShaderRuntimeParamValue::Rgb { r, g, b } => {
                serde_json::json!({"r": *r, "g": *g, "b": *b})
            }
        }
    }
}

/// Declares that a shader can bind a runtime parameter to a named config field.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ShaderRuntimeBindingRequest {
    pub field: String,
    pub binding: String,
    pub expected_type: String,
}

/// Reports how a shader binding resolved against the supplied runtime params.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ShaderRuntimeBindingStatus {
    Resolved,
    Coerced,
    Missing,
    FallbackStatic,
}

/// Resolution record for one shader runtime binding.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ShaderRuntimeBindingResolution {
    pub field: String,
    pub binding: String,
    pub expected_type: String,
    pub status: ShaderRuntimeBindingStatus,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub supplied_kind: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub supplied_value: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub effective_value: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fallback_value: Option<serde_json::Value>,
}

impl From<u16> for ShaderRuntimeParamValue {
    fn from(value: u16) -> Self {
        Self::Integer(i64::from(value))
    }
}

impl From<u32> for ShaderRuntimeParamValue {
    fn from(value: u32) -> Self {
        Self::Integer(i64::from(value))
    }
}

impl From<usize> for ShaderRuntimeParamValue {
    fn from(value: usize) -> Self {
        Self::Integer(value as i64)
    }
}

impl From<i64> for ShaderRuntimeParamValue {
    fn from(value: i64) -> Self {
        Self::Integer(value)
    }
}

impl From<f32> for ShaderRuntimeParamValue {
    fn from(value: f32) -> Self {
        Self::Float(f64::from(value))
    }
}

impl From<f64> for ShaderRuntimeParamValue {
    fn from(value: f64) -> Self {
        Self::Float(value)
    }
}

impl From<bool> for ShaderRuntimeParamValue {
    fn from(value: bool) -> Self {
        Self::Boolean(value)
    }
}

impl From<String> for ShaderRuntimeParamValue {
    fn from(value: String) -> Self {
        Self::Text(value)
    }
}

impl From<&str> for ShaderRuntimeParamValue {
    fn from(value: &str) -> Self {
        Self::Text(value.to_owned())
    }
}

impl From<Color> for ShaderRuntimeParamValue {
    fn from(value: Color) -> Self {
        Self::Rgb {
            r: value.r,
            g: value.g,
            b: value.b,
        }
    }
}

/// Runtime parameter map exposed to spatial shaders during render.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(transparent)]
pub struct ShaderRuntimeParams(pub BTreeMap<String, ShaderRuntimeParamValue>);

impl ShaderRuntimeParams {
    /// Create an empty runtime parameter map.
    pub fn new() -> Self {
        Self::default()
    }

    /// Insert a runtime parameter.
    pub fn insert(
        &mut self,
        key: impl Into<String>,
        value: impl Into<ShaderRuntimeParamValue>,
    ) -> Option<ShaderRuntimeParamValue> {
        self.0.insert(key.into(), value.into())
    }

    /// Fetch a runtime parameter by key.
    pub fn get(&self, key: &str) -> Option<&ShaderRuntimeParamValue> {
        self.0.get(key)
    }

    /// Fetch a runtime parameter coerced to f32.
    pub fn get_f32(&self, key: &str) -> Option<f32> {
        self.get(key).and_then(ShaderRuntimeParamValue::as_f32)
    }

    /// Fetch a runtime parameter coerced to u16.
    pub fn get_u16(&self, key: &str) -> Option<u16> {
        self.get(key).and_then(ShaderRuntimeParamValue::as_u16)
    }

    /// Fetch a runtime parameter interpreted as an opaque RGB `Color`.
    /// Returns `None` for non-Rgb variants or unknown keys.
    pub fn get_color(&self, key: &str) -> Option<Color> {
        self.get(key).and_then(ShaderRuntimeParamValue::as_color)
    }

    /// Fetch a runtime parameter interpreted as a string slice.
    /// Returns `None` for non-Text variants or unknown keys.
    pub fn get_text(&self, key: &str) -> Option<&str> {
        self.get(key).and_then(ShaderRuntimeParamValue::as_text)
    }
}

impl tui_vfx_core::bindable::RuntimeParamsRead for ShaderRuntimeParams {
    fn get_u16(&self, key: &str) -> Option<u16> {
        Self::get_u16(self, key)
    }
    fn get_text(&self, key: &str) -> Option<&str> {
        Self::get_text(self, key)
    }
    fn get_f32(&self, key: &str) -> Option<f32> {
        Self::get_f32(self, key)
    }
}

impl<K, V> FromIterator<(K, V)> for ShaderRuntimeParams
where
    K: Into<String>,
    V: Into<ShaderRuntimeParamValue>,
{
    fn from_iter<T: IntoIterator<Item = (K, V)>>(iter: T) -> Self {
        let mut params = Self::new();
        for (key, value) in iter {
            params.insert(key, value);
        }
        params
    }
}

/// Context passed to `StyleShader` implementations for spatial render effects.
///
/// Composes a [`VfxCellContext`] sub-bundle (`cell`) that carries the seven
/// cell-spatial fields shared with `Filter`, `Mask`, and `Sampler` parameter
/// bundles. A `Deref<Target = VfxCellContext>` impl makes field access
/// ergonomic: `ctx.local_x` desugars to `ctx.cell.local_x` with no runtime
/// cost.
///
/// The four derived-coordinate methods (`screen_cell_x`, `screen_cell_y`,
/// `normalized_x`, `normalized_y`) live on [`VfxCellContext`] and are
/// reached via `Deref` — do not re-add them here.
///
/// # Coordinate Systems
///
/// - **Local coordinates** (`local_x`, `local_y`): position within the
///   widget; `(0, 0)` is the widget's top-left.
/// - **Screen coordinates**: `ctx.screen_cell_x()` / `ctx.screen_cell_y()`
///   give the cell's absolute screen position.
///
/// # Phase-aware shaders
///
/// Use `ctx.phase` / `ctx.is_entering()` / `ctx.is_dwelling()` /
/// `ctx.is_exiting()` to vary behavior across animation lifecycle phases.
#[derive(Debug, Clone)]
pub struct ShaderContext {
    /// Cell-spatial sub-bundle (`local_x`, `local_y`, `width`, `height`,
    /// `screen_x`, `screen_y`, `t`). Shared shape with `Filter`, `Mask`,
    /// and `Sampler` parameter bundles.
    ///
    /// Reach through `Deref` for ergonomic field access: `ctx.local_x`
    /// desugars to `ctx.cell.local_x`. Derived-coordinate methods
    /// (`screen_cell_x`, `screen_cell_y`, `normalized_x`, `normalized_y`)
    /// also live here and are available via `Deref`.
    pub cell: VfxCellContext,
    /// Style-only: animation phase. `None` when the shader is not
    /// phase-driven.
    pub phase: Option<Phase>,
    /// Style-only: render-time runtime parameter map for shader bindings.
    pub runtime_params: Arc<ShaderRuntimeParams>,
    /// Style-only: per-cell semantic role map carried from the source
    /// `SemanticScene`.
    ///
    /// Shaders MAY consult `roles.get((local_x, local_y))` to branch on
    /// the current cell's semantic role (or any other cell's role) in
    /// addition to geometric coordinates. The compositor populates this
    /// `Arc` once per pipeline invocation and clones it per cell, so
    /// reading is O(1) and does not allocate.
    ///
    /// Call sites that have no role information should pass an empty
    /// `Arc<RoleMap>` via `Arc::default()` (or let `ShaderContext::new`
    /// default it). Out-of-bounds reads return `None`, so a 0×0 default
    /// is safe to pair with any coordinate.
    pub roles: Arc<RoleMap>,
}

impl Deref for ShaderContext {
    type Target = VfxCellContext;
    #[inline]
    fn deref(&self) -> &VfxCellContext {
        &self.cell
    }
}

impl ShaderContext {
    /// Create a new shader context.
    ///
    /// The argument list is unchanged from before the `VfxCellContext`
    /// composition refactor (F.2); all 30 in-tree call sites compile
    /// without modification. Internally the seven spatial args are
    /// forwarded to [`VfxCellContext::new`].
    ///
    /// The `roles` field defaults to an empty `Arc<RoleMap>`; call
    /// [`Self::with_roles`] to attach a real role map.
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        local_x: u16,
        local_y: u16,
        width: u16,
        height: u16,
        screen_x: u16,
        screen_y: u16,
        t: f64,
        phase: Option<Phase>,
        runtime_params: Option<Arc<ShaderRuntimeParams>>,
    ) -> Self {
        Self {
            cell: VfxCellContext::new(local_x, local_y, width, height, screen_x, screen_y, t),
            phase,
            runtime_params: runtime_params.unwrap_or_default(),
            roles: Arc::default(),
        }
    }

    /// Attach a per-cell role map to this context.
    ///
    /// Builder variant so callers can opt into role awareness without
    /// widening [`Self::new`]'s argument list. The role map is shared via
    /// `Arc`; cloning this context is a cheap atomic refcount bump.
    pub fn with_roles(mut self, roles: Arc<RoleMap>) -> Self {
        self.roles = roles;
        self
    }

    /// Fetch the role tag at `(x, y)` from this context's role map.
    ///
    /// Returns `None` for out-of-bounds coordinates or when the role map
    /// is empty (the default). Prefer this over `ctx.roles.get(...)` at
    /// call sites that want a clean one-liner.
    pub fn role_at(&self, pos: (u16, u16)) -> Option<RoleTag> {
        self.roles.get(pos)
    }

    /// Check if currently in entering/start phase.
    #[inline]
    pub fn is_entering(&self) -> bool {
        matches!(self.phase, Some(Phase::Start))
    }

    /// Check if currently in dwelling/active phase.
    #[inline]
    pub fn is_dwelling(&self) -> bool {
        matches!(self.phase, Some(Phase::Active))
    }

    /// Check if currently in exiting/end phase.
    #[inline]
    pub fn is_exiting(&self) -> bool {
        matches!(self.phase, Some(Phase::End))
    }

    /// Fetch a render-time runtime parameter by key.
    pub fn runtime_param(&self, key: &str) -> Option<&ShaderRuntimeParamValue> {
        self.runtime_params.get(key)
    }

    /// Fetch a render-time runtime parameter coerced to f32.
    pub fn runtime_param_f32(&self, key: &str) -> Option<f32> {
        self.runtime_params.get_f32(key)
    }

    /// Fetch a render-time runtime parameter coerced to u16.
    pub fn runtime_param_u16(&self, key: &str) -> Option<u16> {
        self.runtime_params.get_u16(key)
    }

    /// Fetch a render-time runtime parameter interpreted as an opaque
    /// RGB `Color`. Returns `None` for non-Rgb variants or unknown keys.
    pub fn runtime_param_color(&self, key: &str) -> Option<Color> {
        self.runtime_params.get_color(key)
    }
}

impl Default for ShaderContext {
    fn default() -> Self {
        Self::new(0, 0, 0, 0, 0, 0, 0.0, None, None)
    }
}

// <FILE>crates/tui-vfx-style/src/traits/cls_shader_context.rs</FILE> - <DESC>Context passed to StyleShader for spatial effects; composes VfxCellContext via Deref. Hosts ShaderRuntimeParams and its RuntimeParamsRead impl that bridges into tui-vfx-core's bindable family.</DESC>
// <VERS>END OF VERSION: 2.1.0</VERS>
