// <FILE>tui-vfx-style/src/types/cls_shader_context.rs</FILE> - <DESC>Context passed to StyleShader for spatial effects</DESC>
// <VERS>VERSION: 1.0.0</VERS>
// <WCTX>Adding screen-space context to shaders</WCTX>
// <CLOG>Initial implementation with local coords, screen offset, and animation state</CLOG>

use mixed_signals::traits::Phase;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use std::sync::Arc;

/// Runtime scalar value exposed to spatial shaders during render.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ShaderRuntimeParamValue {
    Integer(i64),
    Float(f64),
    Boolean(bool),
    Text(String),
}

impl ShaderRuntimeParamValue {
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

/// Context passed to StyleShader implementations for spatial effects.
///
/// This struct provides shaders with complete information about their rendering context,
/// including both local (widget-relative) and screen-absolute coordinates.
///
/// # Coordinate Systems
///
/// - **Local coordinates** (`local_x`, `local_y`): Position within the widget (0,0 = top-left of widget)
/// - **Screen coordinates**: `screen_x + local_x`, `screen_y + local_y` gives absolute screen position
///
/// # Use Cases
///
/// - **Widget-relative effects**: Use `local_x`, `local_y` for effects like highlights, sweeps
/// - **Screen-space effects**: Use screen coords for effects that span multiple widgets or align to screen edges
/// - **Phase-aware effects**: Use `phase` to vary behavior during enter/dwell/exit
#[derive(Debug, Clone)]
pub struct ShaderContext {
    /// Local X coordinate within widget (0 = left edge of widget)
    pub local_x: u16,
    /// Local Y coordinate within widget (0 = top edge of widget)
    pub local_y: u16,
    /// Widget width in cells
    pub width: u16,
    /// Widget height in cells
    pub height: u16,
    /// Screen X offset - widget's left edge in absolute screen coordinates
    pub screen_x: u16,
    /// Screen Y offset - widget's top edge in absolute screen coordinates
    pub screen_y: u16,
    /// Animation progress (0.0 to 1.0) - phase-based or loop time
    pub t: f64,
    /// Current animation phase (Entering/Dwelling/Exiting/Finished)
    pub phase: Option<Phase>,
    /// Render-time runtime parameter map for shader bindings.
    pub runtime_params: Arc<ShaderRuntimeParams>,
}

impl ShaderContext {
    /// Create a new shader context.
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
            local_x,
            local_y,
            width,
            height,
            screen_x,
            screen_y,
            t,
            phase,
            runtime_params: runtime_params.unwrap_or_default(),
        }
    }

    /// Get absolute screen X coordinate for this cell.
    #[inline]
    pub fn screen_cell_x(&self) -> u16 {
        self.screen_x.saturating_add(self.local_x)
    }

    /// Get absolute screen Y coordinate for this cell.
    #[inline]
    pub fn screen_cell_y(&self) -> u16 {
        self.screen_y.saturating_add(self.local_y)
    }

    /// Get normalized local X position (0.0 = left, 1.0 = right).
    #[inline]
    pub fn normalized_x(&self) -> f32 {
        if self.width > 0 {
            self.local_x as f32 / self.width as f32
        } else {
            0.0
        }
    }

    /// Get normalized local Y position (0.0 = top, 1.0 = bottom).
    #[inline]
    pub fn normalized_y(&self) -> f32 {
        if self.height > 0 {
            self.local_y as f32 / self.height as f32
        } else {
            0.0
        }
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
}

impl Default for ShaderContext {
    fn default() -> Self {
        Self {
            local_x: 0,
            local_y: 0,
            width: 0,
            height: 0,
            screen_x: 0,
            screen_y: 0,
            t: 0.0,
            phase: None,
            runtime_params: Arc::default(),
        }
    }
}

// <FILE>tui-vfx-style/src/types/cls_shader_context.rs</FILE> - <DESC>Context passed to StyleShader for spatial effects</DESC>
// <VERS>END OF VERSION: 1.0.0</VERS>
