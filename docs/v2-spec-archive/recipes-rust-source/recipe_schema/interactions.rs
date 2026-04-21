// <FILE>src/recipe_schema/interactions.rs</FILE> - <DESC>Schema types for interaction state configuration in recipes</DESC>
// <VERS>VERSION: 0.3.0</VERS>
// <WCTX>Add character override support for bar width animation</WCTX>
// <CLOG>Add character field to GeometryOverridesJson for HLL-style bars</CLOG>

use crate::interactions::{GeometryOverrides, StateCompositionMode, StateStyleConfig};
use mixed_signals::easing::EasingType;
use serde::{Deserialize, Serialize};
use tui_vfx_style::models::ColorConfig;
use tui_vfx_types::{Color, Modifiers};

/// Configuration for interaction behavior in recipes.
#[derive(Clone, Debug, tui_vfx_core::ConfigSchema, Serialize, Deserialize)]
pub struct InteractionConfig {
    /// Unique identifier for this interactive element (for L3 targeting)
    pub element_id: String,

    /// Transition duration in milliseconds
    pub transition_duration_ms: u64,

    /// Easing function for transitions
    pub easing: EasingType,

    /// State composition mode
    #[serde(default)]
    pub state_composition: StateCompositionMode,

    /// Accessibility configuration
    #[serde(default)]
    pub accessibility: AccessibilityConfig,
}

/// Key-value pair for state name and style configuration.
///
/// Used in RaStylePipelineConfig to represent interaction states as a list.
#[derive(Clone, Debug, tui_vfx_core::ConfigSchema, Serialize, Deserialize)]
pub struct StateStyleEntry {
    /// State name (e.g., "hover", "focus", "active")
    pub state: String,

    /// Style configuration for this state
    #[serde(flatten)]
    pub config: StateStyleConfigJson,
}

/// Accessibility configuration for WCAG compliance.
#[derive(Clone, Debug, tui_vfx_core::ConfigSchema, Serialize, Deserialize)]
pub struct AccessibilityConfig {
    /// Focus must be visible (WCAG 2.4.7)
    #[serde(default = "default_true")]
    pub focus_visible_required: bool,

    /// Minimum contrast ratio (WCAG 1.4.3 Level AA = 3.0)
    #[serde(default = "default_contrast_ratio")]
    pub min_contrast_ratio: f32,

    /// Support reduced motion preference
    #[serde(default = "default_true")]
    pub reduce_motion_compliant: bool,
}

impl Default for AccessibilityConfig {
    fn default() -> Self {
        Self {
            focus_visible_required: true,
            min_contrast_ratio: 3.0,
            reduce_motion_compliant: true,
        }
    }
}

fn default_true() -> bool {
    true
}

fn default_contrast_ratio() -> f32 {
    3.0
}

/// JSON representation of state style configuration.
///
/// This is the schema type for parsing from recipe JSON.
/// It converts to the internal `StateStyleConfig` type.
#[derive(Clone, Debug, tui_vfx_core::ConfigSchema, Serialize, Deserialize)]
pub struct StateStyleConfigJson {
    /// Background color for this state
    #[serde(skip_serializing_if = "Option::is_none")]
    pub background: Option<ColorConfig>,

    /// Foreground color for this state
    #[serde(skip_serializing_if = "Option::is_none")]
    pub foreground: Option<ColorConfig>,

    /// Text modifiers for this state (e.g., ["BOLD", "ITALIC"])
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifiers: Vec<String>,

    /// Opacity for this state (0.0 = transparent, 1.0 = opaque)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub opacity: Option<f32>,

    /// Geometry overrides for this state
    #[serde(skip_serializing_if = "Option::is_none")]
    pub geometry: Option<GeometryOverridesJson>,
}

/// JSON representation of geometry overrides.
#[derive(Clone, Debug, tui_vfx_core::ConfigSchema, Serialize, Deserialize)]
pub struct GeometryOverridesJson {
    /// Width of accent bar (e.g., left edge highlight)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub accent_bar_width: Option<u16>,

    /// Thickness of outline/border
    #[serde(skip_serializing_if = "Option::is_none")]
    pub outline_thickness: Option<u16>,

    /// Color of outline/border
    #[serde(skip_serializing_if = "Option::is_none")]
    pub outline_color: Option<ColorConfig>,

    /// Character override (e.g., "▌" → "█" for bar width animation)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub character: Option<char>,
}

/// Helper to convert ColorConfig to Color
fn color_config_to_color(config: ColorConfig) -> Color {
    // ColorConfig implements From<ColorConfig> for Color
    Color::from(config)
}

/// Helper to convert Vec<String> to Modifiers
fn strings_to_modifiers(strings: &[String]) -> Option<Modifiers> {
    if strings.is_empty() {
        return None;
    }

    let mut modifiers = Modifiers {
        bold: false,
        italic: false,
        underline: false,
        dim: false,
        reverse: false,
        strikethrough: false,
        slow_blink: false,
        rapid_blink: false,
        hidden: false,
    };

    for s in strings {
        match s.to_uppercase().as_str() {
            "BOLD" => modifiers.bold = true,
            "ITALIC" => modifiers.italic = true,
            "UNDERLINE" | "UNDERLINED" => modifiers.underline = true,
            "DIM" => modifiers.dim = true,
            "REVERSE" | "REVERSED" => modifiers.reverse = true,
            "STRIKETHROUGH" | "CROSSED_OUT" => modifiers.strikethrough = true,
            "SLOW_BLINK" => modifiers.slow_blink = true,
            "RAPID_BLINK" => modifiers.rapid_blink = true,
            "HIDDEN" => modifiers.hidden = true,
            _ => {} // Ignore unknown modifiers
        }
    }
    Some(modifiers)
}

/// Convert JSON geometry to internal geometry
impl From<GeometryOverridesJson> for GeometryOverrides {
    fn from(json: GeometryOverridesJson) -> Self {
        Self {
            accent_bar_width: json.accent_bar_width,
            outline_thickness: json.outline_thickness,
            outline_color: json.outline_color.map(color_config_to_color),
            character: json.character,
        }
    }
}

/// Convert JSON state style to internal state style
impl From<StateStyleConfigJson> for StateStyleConfig {
    fn from(json: StateStyleConfigJson) -> Self {
        Self {
            background: json.background.map(color_config_to_color),
            foreground: json.foreground.map(color_config_to_color),
            modifiers: strings_to_modifiers(&json.modifiers),
            opacity: json.opacity,
            geometry: json.geometry.map(Into::into).unwrap_or_default(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_interaction_config_defaults() {
        let config = serde_json::from_str::<InteractionConfig>(
            r#"{
                "element_id": "test",
                "transition_duration_ms": 150,
                "easing": "Linear"
            }"#,
        )
        .unwrap();

        assert_eq!(config.state_composition, StateCompositionMode::Layered);
        assert!(config.accessibility.focus_visible_required);
        assert_eq!(config.accessibility.min_contrast_ratio, 3.0);
    }

    #[test]
    fn test_state_style_config_json_conversion() {
        let json = StateStyleConfigJson {
            background: Some(ColorConfig::Rgb {
                r: 35,
                g: 75,
                b: 50,
            }),
            foreground: Some(ColorConfig::Rgb {
                r: 255,
                g: 255,
                b: 255,
            }),
            modifiers: vec![],
            opacity: Some(0.8),
            geometry: Some(GeometryOverridesJson {
                accent_bar_width: Some(2),
                outline_thickness: Some(1),
                outline_color: Some(ColorConfig::Rgb {
                    r: 200,
                    g: 175,
                    b: 50,
                }),
                character: None,
            }),
        };

        let internal: StateStyleConfig = json.into();

        assert_eq!(internal.background, Some(Color::rgb(35, 75, 50)));
        assert_eq!(internal.foreground, Some(Color::rgb(255, 255, 255)));
        assert_eq!(internal.opacity, Some(0.8));
        assert_eq!(internal.geometry.accent_bar_width, Some(2));
        assert_eq!(internal.geometry.outline_thickness, Some(1));
        assert_eq!(
            internal.geometry.outline_color,
            Some(Color::rgb(200, 175, 50))
        );
    }

    #[test]
    fn test_geometry_overrides_conversion() {
        let json = GeometryOverridesJson {
            accent_bar_width: Some(2),
            outline_thickness: Some(1),
            outline_color: Some(ColorConfig::Rgb {
                r: 200,
                g: 175,
                b: 50,
            }),
            character: None,
        };

        let internal: GeometryOverrides = json.into();

        assert_eq!(internal.accent_bar_width, Some(2));
        assert_eq!(internal.outline_thickness, Some(1));
        assert_eq!(internal.outline_color, Some(Color::rgb(200, 175, 50)));
    }
}

// <FILE>src/recipe_schema/interactions.rs</FILE> - <DESC>Schema types for interaction state configuration in recipes</DESC>
// <VERS>END OF VERSION: 0.3.0</VERS>
