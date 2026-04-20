// <FILE>crates/tui-vfx-content/src/pool/cls_preset_pool.rs</FILE> - <DESC>Pool of curated (text, effect, ...) bundles. Lets authors pair specific text with specific effects so per-launch variety is a choice from a hand-crafted matrix instead of N×M combinatorics across independent pools.</DESC>
// <VERS>VERSION: 0.2.0</VERS>
// <WCTX>Stage 1.5 of the splash library + VFX integration plan — extend Preset with image_name + font_name slots so curated bundles can pair text+effect+logo+font together.</WCTX>
// <CLOG>0.2.0: add Preset.image_name + Preset.font_name (rocketsplash asset-map keys) and with_* constructors.
// 0.1.0: initial; Preset + PresetPool with all-Option fields for forward compat (Phase 2 will add speaker, shader_override, shadow_override, etc.).</CLOG>

use serde::{Deserialize, Serialize};

use super::col_pool_policy::PoolPolicy;
use super::fnc_pick_index::pick_index;
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
/// Reach for `Preset` / [`PresetPool`] when you want **curated
/// craftsmanship**: "angry line → glitch," "calm line → typewriter,"
/// "brand heading → split-flap reveal." For max surprise across every
/// combination, use independent [`TextPool`](super::TextPool) +
/// [`EffectPool`](super::EffectPool) instead.
#[derive(
    Clone, Debug, Default, PartialEq, Serialize, Deserialize, tui_vfx_core::ConfigSchema,
)]
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

/// Pool of curated [`Preset`] bundles.
///
/// # Precedence with other pools
///
/// When a `ContentConfig` carries both a `PresetPool` and independent
/// [`TextPool`](super::TextPool) / [`EffectPool`](super::EffectPool)
/// fields, the preset pool wins if non-empty — the idea is that a
/// curated bundle represents deliberate authorial intent and should
/// override ad-hoc combinatorics. If the preset pool is empty, the
/// independent pools are consulted; if those are also empty, the
/// static `content.text` / `content.effect` are used.
///
/// (The precedence logic lives on `ContentConfig::resolved_*`
/// accessors, not on `PresetPool` itself — the pool just picks; the
/// schema decides which slot wins.)
#[derive(
    Clone, Debug, Default, PartialEq, Serialize, Deserialize, tui_vfx_core::ConfigSchema,
)]
pub struct PresetPool {
    /// Pool entries — each a curated bundle.
    #[serde(default)]
    pub items: Vec<Preset>,

    /// How [`PresetPool::pick`] selects an entry.
    #[serde(default)]
    pub policy: PoolPolicy,
}

impl PresetPool {
    /// Construct a new preset pool.
    pub fn new(items: Vec<Preset>, policy: PoolPolicy) -> Self {
        Self { items, policy }
    }

    /// Pick one preset according to this pool's policy. Returns `None`
    /// when the pool is empty.
    pub fn pick(&self) -> Option<&Preset> {
        pick_index(self.items.len(), self.policy).map(|idx| &self.items[idx])
    }

    /// True if the pool has no items.
    pub fn is_empty(&self) -> bool {
        self.items.is_empty()
    }
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::{ScrambleCharset, TypewriterCursor};
    use mixed_signals::prelude::SignalOrFloat;

    fn tw() -> ContentEffect {
        ContentEffect::Typewriter {
            speed_variance: SignalOrFloat::Static(0.0),
            cursor: Some(TypewriterCursor::block()),
        }
    }

    fn sc() -> ContentEffect {
        ContentEffect::Scramble {
            resolve_pace: SignalOrFloat::Static(1.0),
            charset: ScrambleCharset::Alphanumeric,
            seed: 0,
        }
    }

    #[test]
    fn empty_pool_returns_none() {
        let pool = PresetPool::default();
        assert!(pool.pick().is_none());
        assert!(pool.is_empty());
    }

    #[test]
    fn first_only_returns_first_preset() {
        let pool = PresetPool::new(
            vec![
                Preset::with_text_and_effect("angry line", sc()),
                Preset::with_text_and_effect("calm line", tw()),
            ],
            PoolPolicy::FirstOnly,
        );
        let picked = pool.pick().unwrap();
        assert_eq!(picked.text.as_deref(), Some("angry line"));
        assert!(matches!(picked.effect, Some(ContentEffect::Scramble { .. })));
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
        assert!(matches!(preset.effect, Some(ContentEffect::Typewriter { .. })));
    }

    #[test]
    fn serde_roundtrip_preserves_presets() {
        let pool = PresetPool::new(
            vec![
                Preset::with_text_and_effect("ignition", tw()),
                Preset::with_text("static line"),
            ],
            PoolPolicy::Random,
        );
        let json = serde_json::to_string(&pool).unwrap();
        let back: PresetPool = serde_json::from_str(&json).unwrap();
        assert_eq!(pool, back);
    }

    #[test]
    fn serde_skips_none_fields_on_output() {
        let pool = PresetPool::new(
            vec![Preset::with_text("just text")],
            PoolPolicy::FirstOnly,
        );
        let json = serde_json::to_string(&pool).unwrap();
        assert!(
            !json.contains("effect"),
            "Preset with effect:None should omit the field from JSON output; got: {json}"
        );
        assert!(!json.contains("image_name"));
        assert!(!json.contains("font_name"));
    }

    #[test]
    fn preset_with_image_and_font_asset_names() {
        let preset = Preset::with_text_and_effect("Corporate mode", tw())
            .with_image("logo_light")
            .with_font("bold_20");
        assert_eq!(preset.text.as_deref(), Some("Corporate mode"));
        assert!(matches!(preset.effect, Some(ContentEffect::Typewriter { .. })));
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
    fn preset_only_asset_fields_without_text_or_effect() {
        let preset = Preset::default().with_image("base_logo").with_font("default_font");
        assert!(preset.text.is_none());
        assert!(preset.effect.is_none());
        assert_eq!(preset.image_name.as_deref(), Some("base_logo"));
        assert_eq!(preset.font_name.as_deref(), Some("default_font"));
    }
}

// <FILE>crates/tui-vfx-content/src/pool/cls_preset_pool.rs</FILE>
// <VERS>END OF VERSION: 0.2.0</VERS>
