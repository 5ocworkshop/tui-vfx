// <FILE>tui-vfx-style/src/models/cls_style_config.rs</FILE> - <DESC>Serde-friendly representation of Ratatui Style</DESC>
// <VERS>VERSION: 1.0.1 - 2025-12-18T09:16:57Z</VERS>
// <WCTX>Clippy fix: field_reassign_with_default</WCTX>
// <CLOG>Used struct literal for Style initialization</CLOG>

use crate::models::ColorConfig;
use serde::{Deserialize, Serialize};
use tui_vfx_types::{Color, Modifiers, Style};
#[derive(
    Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, tui_vfx_core::ConfigSchema,
)]
#[serde(rename_all = "snake_case")]
pub enum ModifierConfig {
    Bold,
    Dim,
    Italic,
    Underlined,
    SlowBlink,
    RapidBlink,
    Reversed,
    Hidden,
    CrossedOut,
}
fn modifiers_to_vec(mods: Modifiers) -> Vec<ModifierConfig> {
    let mut out = Vec::new();
    if mods.bold {
        out.push(ModifierConfig::Bold);
    }
    if mods.dim {
        out.push(ModifierConfig::Dim);
    }
    if mods.italic {
        out.push(ModifierConfig::Italic);
    }
    if mods.underline {
        out.push(ModifierConfig::Underlined);
    }
    if mods.reverse {
        out.push(ModifierConfig::Reversed);
    }
    if mods.strikethrough {
        out.push(ModifierConfig::CrossedOut);
    }
    out
}
fn vec_to_modifiers(mods: &[ModifierConfig]) -> Modifiers {
    let mut out = Modifiers::NONE;
    for modifier in mods {
        match modifier {
            ModifierConfig::Bold => out.bold = true,
            ModifierConfig::Dim => out.dim = true,
            ModifierConfig::Italic => out.italic = true,
            ModifierConfig::Underlined => out.underline = true,
            ModifierConfig::SlowBlink => {} // Not supported in tui_vfx_types::Modifiers
            ModifierConfig::RapidBlink => {} // Not supported in tui_vfx_types::Modifiers
            ModifierConfig::Reversed => out.reverse = true,
            ModifierConfig::Hidden => {} // Not supported in tui_vfx_types::Modifiers
            ModifierConfig::CrossedOut => out.strikethrough = true,
        };
    }
    out
}
/// Stable, serde-friendly representation of [`ratatui::style::Style`].
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, tui_vfx_core::ConfigSchema)]
#[serde(deny_unknown_fields)]
pub struct StyleConfig {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub fg: Option<ColorConfig>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub bg: Option<ColorConfig>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub add_modifier: Vec<ModifierConfig>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub sub_modifier: Vec<ModifierConfig>,
}
impl From<Style> for StyleConfig {
    fn from(value: Style) -> Self {
        // Convert non-transparent colors to Some, transparent to None
        let fg = if value.fg == Color::TRANSPARENT {
            None
        } else {
            Some(ColorConfig::from(value.fg))
        };
        let bg = if value.bg == Color::TRANSPARENT {
            None
        } else {
            Some(ColorConfig::from(value.bg))
        };
        Self {
            fg,
            bg,
            add_modifier: modifiers_to_vec(value.mods),
            sub_modifier: Vec::new(), // mixed_types doesn't have sub_modifier concept
        }
    }
}
impl From<StyleConfig> for Style {
    fn from(value: StyleConfig) -> Self {
        Style {
            fg: value.fg.map(Color::from).unwrap_or(Color::TRANSPARENT),
            bg: value.bg.map(Color::from).unwrap_or(Color::TRANSPARENT),
            mods: vec_to_modifiers(&value.add_modifier),
        }
    }
}

// <FILE>tui-vfx-style/src/models/cls_style_config.rs</FILE> - <DESC>Serde-friendly representation of Ratatui Style</DESC>
// <VERS>END OF VERSION: 1.0.1 - 2025-12-18T09:16:57Z</VERS>
