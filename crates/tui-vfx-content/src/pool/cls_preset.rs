// <FILE>crates/tui-vfx-content/src/pool/cls_preset.rs</FILE> - <DESC>Curated content bundle (the item type held by PresetPool). Pairs text + effect + asset references so per-launch variety is a deliberate choice from a hand-crafted matrix instead of N×M combinatorics across independent pools.</DESC>
// <VERS>VERSION: 0.1.0</VERS>
// <WCTX>Buy-once sweep finding 1.2.B — the Preset item type extracts to its own file so PresetPool can collapse to `pub type PresetPool = Pool&lt;Preset&gt;;` alongside the other pool aliases. Behavior verbatim from the previous cls_preset_pool.rs (v0.2.0).</WCTX>
// <CLOG>0.1.0: Preset moves from cls_preset_pool.rs to its own file with the same shape and with_* constructors. No behavioral change — the pool side now lives as a Pool&lt;Preset&gt; alias.</CLOG>

use serde::{Deserialize, Serialize};

use crate::types::ContentEffect;

/// A curated content bundle. Each [`Preset`] groups a text (optional)
/// with an effect (optional), and in the future may bundle additional
/// pipeline overrides (speaker metadata, shader overrides, shadow
/// overrides, easing overrides, duration overrides, etc.).
///
/// All fields are `Option` so authors set only what they want to
/// override per variant — anything left `None` inherits from the
/// recipe's outer defaults (the static `content.text`, `content.effect`,
/// and so on). Forward-compatible by construction: new optional fields
/// can be added to `Preset` without breaking existing JSON recipes.
///
/// # When to use
///
/// Reach for `Preset` / [`PresetPool`](super::PresetPool) when you want
/// **curated craftsmanship**: "angry line → glitch," "calm line →
/// typewriter," "brand heading → split-flap reveal." For max surprise
/// across every combination, use independent
/// [`TextPool`](super::TextPool) + [`EffectPool`](super::EffectPool)
/// instead.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize, tui_vfx_core::ConfigSchema)]
pub struct Preset {
    /// Override the content's text for this variant. `None` → inherit
    /// the recipe's static `content.text` or fall through to
    /// [`TextPool`](super::TextPool) resolution.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub text: Option<String>,

    /// Override the content's effect for this variant. `None` → inherit
    /// the recipe's static `content.effect` or fall through to
    /// [`EffectPool`](super::EffectPool) resolution.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub effect: Option<ContentEffect>,

    /// Override the rocketsplash image for this variant, as an asset-map
    /// key the caller resolves to `.rss` bytes. `None` → inherit from
    /// [`ImagePool`](super::ImagePool) or the static logo slot.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub image_name: Option<String>,

    /// Override the rocketsplash font for this variant, as an asset-map
    /// key the caller resolves to `.rsf` bytes. `None` → inherit from
    /// [`FontPool`](super::FontPool) or the static font slot.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub font_name: Option<String>,
    // Phase 2 extension slots (additive, non-breaking):
    //   pub speaker: Option<String>,
    //   pub shader_override: Option<ShaderSpec>,
    //   pub shadow_override: Option<ShadowSpec>,
    //   pub duration_override_ms: Option<u32>,
    //   pub easing_override: Option<EasingCurve>,
}

impl Preset {
    /// Build a preset with just text (other fields inherit).
    pub fn with_text(text: impl Into<String>) -> Self {
        Self {
            text: Some(text.into()),
            ..Self::default()
        }
    }

    /// Build a preset with just an effect (other fields inherit).
    pub fn with_effect(effect: ContentEffect) -> Self {
        Self {
            effect: Some(effect),
            ..Self::default()
        }
    }

    /// Build a preset bundling text and effect together.
    pub fn with_text_and_effect(text: impl Into<String>, effect: ContentEffect) -> Self {
        Self {
            text: Some(text.into()),
            effect: Some(effect),
            ..Self::default()
        }
    }

    /// Chain a rocketsplash image asset key onto this preset.
    pub fn with_image(mut self, image_name: impl Into<String>) -> Self {
        self.image_name = Some(image_name.into());
        self
    }

    /// Chain a rocketsplash font asset key onto this preset.
    pub fn with_font(mut self, font_name: impl Into<String>) -> Self {
        self.font_name = Some(font_name.into());
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::{ScrambleCharset, TypewriterCursor};
    use tui_vfx_core::bindable::VfxBindableValue;

    fn tw() -> ContentEffect {
        ContentEffect::Typewriter {
            speed_variance: VfxBindableValue::Literal(0.0),
            cursor: Some(TypewriterCursor::block()),
        }
    }

    fn sc() -> ContentEffect {
        ContentEffect::Scramble {
            resolve_pace: VfxBindableValue::Literal(1.0),
            charset: ScrambleCharset::Alphanumeric,
            seed: 0,
        }
    }

    #[test]
    fn partial_preset_with_text_only() {
        let preset = Preset::with_text("just text");
        assert_eq!(preset.text.as_deref(), Some("just text"));
        assert!(preset.effect.is_none());
    }

    #[test]
    fn partial_preset_with_effect_only() {
        let preset = Preset::with_effect(tw());
        assert!(preset.text.is_none());
        assert!(matches!(
            preset.effect,
            Some(ContentEffect::Typewriter { .. })
        ));
    }

    #[test]
    fn preset_with_text_and_effect_pairs_them() {
        let preset = Preset::with_text_and_effect("angry line", sc());
        assert_eq!(preset.text.as_deref(), Some("angry line"));
        assert!(matches!(
            preset.effect,
            Some(ContentEffect::Scramble { .. })
        ));
    }

    #[test]
    fn preset_with_image_and_font_asset_names() {
        let preset = Preset::with_text_and_effect("Corporate mode", tw())
            .with_image("logo_light")
            .with_font("bold_20");
        assert_eq!(preset.image_name.as_deref(), Some("logo_light"));
        assert_eq!(preset.font_name.as_deref(), Some("bold_20"));
    }

    #[test]
    fn preset_asset_fields_serde_roundtrip() {
        let preset = Preset::with_text("Party mode")
            .with_image("logo_halloween")
            .with_font("script_40");
        let json = serde_json::to_string(&preset).unwrap();
        assert!(json.contains("logo_halloween"));
        assert!(json.contains("script_40"));
        let back: Preset = serde_json::from_str(&json).unwrap();
        assert_eq!(preset, back);
    }

    #[test]
    fn preset_serde_skips_none_fields() {
        let preset = Preset::with_text("just text");
        let json = serde_json::to_string(&preset).unwrap();
        assert!(!json.contains("effect"));
        assert!(!json.contains("image_name"));
        assert!(!json.contains("font_name"));
    }

    #[test]
    fn preset_only_asset_fields_without_text_or_effect() {
        let preset = Preset::default()
            .with_image("base_logo")
            .with_font("default_font");
        assert!(preset.text.is_none());
        assert!(preset.effect.is_none());
        assert_eq!(preset.image_name.as_deref(), Some("base_logo"));
        assert_eq!(preset.font_name.as_deref(), Some("default_font"));
    }
}

// <FILE>crates/tui-vfx-content/src/pool/cls_preset.rs</FILE>
// <VERS>END OF VERSION: 0.1.0</VERS>
