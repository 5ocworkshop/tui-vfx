// <FILE>tui-vfx-style/src/models/cls_color_config.rs</FILE> - <DESC>Serde-friendly representation of Color</DESC>
// <VERS>VERSION: 1.4.0 - 2026-01-12</VERS>
// <WCTX>L2/L3 abstraction: tui-style-fx uses mixed-types</WCTX>
// <CLOG>Fixed Color→ColorConfig mapping to use Light* variants for bright colors</CLOG>

use serde::{Deserialize, Deserializer, Serialize};
use tui_vfx_types::Color;

/// Stable, serde-friendly representation of [`ratatui::style::Color`].
///
/// We avoid enabling Ratatui's `serde` feature so the logic crates remain lightweight and
/// configuration formats remain under our control.
///
/// # RGB Shorthand
///
/// For RGB colors, you can use either the full tagged format or a shorthand:
/// - Full: `{"type": "rgb", "r": 255, "g": 128, "b": 0}`
/// - Shorthand: `{"r": 255, "g": 128, "b": 0}`
///
/// Both formats are equivalent and produce the same `ColorConfig::Rgb` value.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, tui_vfx_core::ConfigSchema)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum ColorConfig {
    Reset,
    Black,
    Red,
    Green,
    Yellow,
    Blue,
    Magenta,
    Cyan,
    /// Standard gray (ANSI color 7, "light gray" in some contexts)
    Gray,
    /// Alias for Gray - allows "light_gray" in JSON
    #[serde(alias = "light_gray")]
    LightGray,
    DarkGray,
    LightRed,
    LightGreen,
    LightYellow,
    LightBlue,
    LightMagenta,
    LightCyan,
    White,
    #[serde(alias = "rgb")]
    Rgb {
        r: u8,
        g: u8,
        b: u8,
    },
    Indexed {
        value: u8,
    },
}

/// Helper struct for RGB shorthand deserialization.
#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct RgbShorthand {
    r: u8,
    g: u8,
    b: u8,
}

/// Internal tagged representation for regular deserialization.
#[derive(Deserialize)]
#[serde(tag = "type", rename_all = "snake_case", deny_unknown_fields)]
enum TaggedColorConfig {
    Reset,
    Black,
    Red,
    Green,
    Yellow,
    Blue,
    Magenta,
    Cyan,
    Gray,
    #[serde(alias = "light_gray")]
    LightGray,
    DarkGray,
    LightRed,
    LightGreen,
    LightYellow,
    LightBlue,
    LightMagenta,
    LightCyan,
    White,
    #[serde(alias = "rgb")]
    Rgb {
        r: u8,
        g: u8,
        b: u8,
    },
    Indexed {
        value: u8,
    },
}

impl From<TaggedColorConfig> for ColorConfig {
    fn from(tagged: TaggedColorConfig) -> Self {
        match tagged {
            TaggedColorConfig::Reset => ColorConfig::Reset,
            TaggedColorConfig::Black => ColorConfig::Black,
            TaggedColorConfig::Red => ColorConfig::Red,
            TaggedColorConfig::Green => ColorConfig::Green,
            TaggedColorConfig::Yellow => ColorConfig::Yellow,
            TaggedColorConfig::Blue => ColorConfig::Blue,
            TaggedColorConfig::Magenta => ColorConfig::Magenta,
            TaggedColorConfig::Cyan => ColorConfig::Cyan,
            TaggedColorConfig::Gray => ColorConfig::Gray,
            TaggedColorConfig::LightGray => ColorConfig::LightGray,
            TaggedColorConfig::DarkGray => ColorConfig::DarkGray,
            TaggedColorConfig::LightRed => ColorConfig::LightRed,
            TaggedColorConfig::LightGreen => ColorConfig::LightGreen,
            TaggedColorConfig::LightYellow => ColorConfig::LightYellow,
            TaggedColorConfig::LightBlue => ColorConfig::LightBlue,
            TaggedColorConfig::LightMagenta => ColorConfig::LightMagenta,
            TaggedColorConfig::LightCyan => ColorConfig::LightCyan,
            TaggedColorConfig::White => ColorConfig::White,
            TaggedColorConfig::Rgb { r, g, b } => ColorConfig::Rgb { r, g, b },
            TaggedColorConfig::Indexed { value } => ColorConfig::Indexed { value },
        }
    }
}

impl<'de> Deserialize<'de> for ColorConfig {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        use serde::de::Error;

        // Deserialize into a generic JSON value first
        let value = serde_json::Value::deserialize(deserializer)?;

        // Check if it's an object with r, g, b but no "type" field (RGB shorthand)
        if let serde_json::Value::Object(ref map) = value {
            if !map.contains_key("type")
                && map.contains_key("r")
                && map.contains_key("g")
                && map.contains_key("b")
            {
                // Try RGB shorthand
                let shorthand: RgbShorthand = serde_json::from_value(value.clone())
                    .map_err(|e| D::Error::custom(format!("Invalid RGB shorthand: {}", e)))?;
                return Ok(ColorConfig::Rgb {
                    r: shorthand.r,
                    g: shorthand.g,
                    b: shorthand.b,
                });
            }
        }

        // Fall back to tagged format
        let tagged: TaggedColorConfig = serde_json::from_value(value)
            .map_err(|e| D::Error::custom(format!("Invalid color config: {}", e)))?;
        Ok(tagged.into())
    }
}

impl From<Color> for ColorConfig {
    fn from(value: Color) -> Self {
        // tui_vfx_types::Color is always RGB, so convert to Rgb variant
        // Check for known constants first - map to Light* variants for bright colors
        // since the base variants (Red, Blue, etc.) are dim ANSI colors
        if value == Color::TRANSPARENT {
            Self::Reset
        } else if value == Color::BLACK {
            Self::Black
        } else if value == Color::WHITE {
            Self::White
        } else if value == Color::RED {
            Self::LightRed // Bright red (255,0,0)
        } else if value == Color::GREEN {
            Self::LightGreen // Bright green (0,255,0)
        } else if value == Color::BLUE {
            Self::LightBlue // Bright blue (0,0,255)
        } else if value == Color::YELLOW {
            Self::LightYellow // Bright yellow (255,255,0)
        } else if value == Color::CYAN {
            Self::LightCyan // Bright cyan (0,255,255)
        } else if value == Color::MAGENTA {
            Self::LightMagenta // Bright magenta (255,0,255)
        } else {
            // Default to RGB representation
            Self::Rgb {
                r: value.r,
                g: value.g,
                b: value.b,
            }
        }
    }
}

impl From<ColorConfig> for Color {
    fn from(value: ColorConfig) -> Self {
        match value {
            ColorConfig::Reset => Color::TRANSPARENT,
            ColorConfig::Black => Color::BLACK,
            ColorConfig::Red => Color::rgb(128, 0, 0), // Standard ANSI dim red
            ColorConfig::Green => Color::rgb(0, 128, 0),
            ColorConfig::Yellow => Color::rgb(128, 128, 0),
            ColorConfig::Blue => Color::rgb(0, 0, 128),
            ColorConfig::Magenta => Color::rgb(128, 0, 128),
            ColorConfig::Cyan => Color::rgb(0, 128, 128),
            ColorConfig::Gray | ColorConfig::LightGray => Color::rgb(192, 192, 192),
            ColorConfig::DarkGray => Color::rgb(128, 128, 128),
            ColorConfig::LightRed => Color::RED,
            ColorConfig::LightGreen => Color::GREEN,
            ColorConfig::LightYellow => Color::YELLOW,
            ColorConfig::LightBlue => Color::BLUE,
            ColorConfig::LightMagenta => Color::MAGENTA,
            ColorConfig::LightCyan => Color::CYAN,
            ColorConfig::White => Color::WHITE,
            ColorConfig::Rgb { r, g, b } => Color::rgb(r, g, b),
            ColorConfig::Indexed { value } => {
                // Convert indexed color to RGB approximation using 6x6x6 cube
                if value < 16 {
                    // Basic ANSI colors
                    match value {
                        0 => Color::BLACK,
                        1 => Color::rgb(128, 0, 0),
                        2 => Color::rgb(0, 128, 0),
                        3 => Color::rgb(128, 128, 0),
                        4 => Color::rgb(0, 0, 128),
                        5 => Color::rgb(128, 0, 128),
                        6 => Color::rgb(0, 128, 128),
                        7 => Color::rgb(192, 192, 192),
                        8 => Color::rgb(128, 128, 128),
                        9 => Color::RED,
                        10 => Color::GREEN,
                        11 => Color::YELLOW,
                        12 => Color::BLUE,
                        13 => Color::MAGENTA,
                        14 => Color::CYAN,
                        _ => Color::WHITE,
                    }
                } else if value < 232 {
                    // 6x6x6 color cube
                    let idx = value - 16;
                    let r = idx / 36;
                    let g = (idx % 36) / 6;
                    let b = idx % 6;
                    let map_val = |v: u8| -> u8 {
                        match v {
                            0 => 0,
                            1 => 95,
                            2 => 135,
                            3 => 175,
                            4 => 215,
                            _ => 255,
                        }
                    };
                    Color::rgb(map_val(r), map_val(g), map_val(b))
                } else {
                    // Grayscale ramp
                    let gray = 8 + (value - 232) * 10;
                    Color::rgb(gray, gray, gray)
                }
            }
        }
    }
}

// <FILE>tui-vfx-style/src/models/cls_color_config.rs</FILE> - <DESC>Serde-friendly representation of Color</DESC>
// <VERS>END OF VERSION: 1.4.0 - 2026-01-12</VERS>
