// <FILE>tui-vfx-content/src/types/cls_mechanical_cycle_source.rs</FILE> - <DESC>Public schema for mechanical cycle face sources (Pair / Ordered / Preset / Randomized / Weighted)</DESC>
// <VERS>VERSION: 0.2.0</VERS>
// <WCTX>Slice 6.3 of mechanical circular content cycles plan: add optional `font` field to Preset variant so preset digit cycles can expand single-char faces through a runtime-bindable font name.</WCTX>
// <CLOG>0.2.0: add Optional<BindableString> font field to MechanicalContentSource::Preset; absent leaves the existing 1-cell digit behavior unchanged. 0.1.0: introduce MechanicalContentSource, CycleWrapMode, MechanicalCyclePreset, WeightedCycleFace.</CLOG>

use serde::{Deserialize, Serialize};
use tui_vfx_style::models::BindableString;

/// Source of faces consumed by a [`MechanicalCycleConfig`].
///
/// `Pair` is the compatibility default: it carries no intermediate faces and
/// yields the existing old/new tile-roll behavior of [`Odometer`] and
/// [`SplitFlap`]. The remaining variants describe ordered or weighted face
/// supplies that downstream route building traverses between source and
/// target.
///
/// [`MechanicalCycleConfig`]: super::cls_mechanical_cycle_config::MechanicalCycleConfig
/// [`Odometer`]: super::cls_content_effect::ContentEffect::Odometer
/// [`SplitFlap`]: super::cls_content_effect::ContentEffect::SplitFlap
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, tui_vfx_core::ConfigSchema)]
#[serde(tag = "type", rename_all = "snake_case", deny_unknown_fields)]
#[derive(Default)]
pub enum MechanicalContentSource {
    /// Direct old/new exchange. No intermediate faces; route is `[from, to]`.
    #[default]
    Pair,

    /// Author-supplied ordered list of face strings. Faces are normalized
    /// against the mechanism tile size and rejected if they overflow.
    Ordered {
        faces: Vec<String>,
        #[serde(default)]
        wrap: CycleWrapMode,
    },

    /// Named preset face set. Presets are documented and tested; their exact
    /// face order is part of the public contract.
    ///
    /// When `font` is `Some`, each preset face (e.g. `"0"`, `"1"`, ...,
    /// `"9"` for `decimal_digits`) is expanded through the resolved
    /// font's glyph table before normalization, producing multi-line
    /// face strings sized to the font's cell shape (3×3 for the
    /// canonical Line 3x3 face per Intention 36). When `font` is
    /// `None`, faces remain single-character strings — current behavior.
    /// `BindableString::Literal` resolves immediately; `Binding` requires
    /// runtime parameters threaded into the resolver (Slice 6.5).
    Preset {
        preset: MechanicalCyclePreset,
        #[serde(default)]
        wrap: CycleWrapMode,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        font: Option<BindableString>,
    },

    /// Author-supplied faces shuffled deterministically once from `seed`.
    /// The shuffle output is a function of `(seed, faces)` and never observes
    /// runtime randomness.
    Randomized {
        faces: Vec<String>,
        seed: u64,
        #[serde(default)]
        wrap: CycleWrapMode,
    },

    /// Weighted reel source. Per-face `weight` controls relative frequency in
    /// the resolved cycle order; `seed` makes the order deterministic. Total
    /// weight must fit in `u32`.
    Weighted {
        faces: Vec<WeightedCycleFace>,
        seed: u64,
        #[serde(default)]
        wrap: CycleWrapMode,
    },
}


/// Whether a cycle wraps around its endpoints.
///
/// `Circular` is the natural fit for digit drums and Solari letter wheels;
/// `Bounded` rejects routes that would require traversing past either end.
#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    Default,
    Serialize,
    Deserialize,
    tui_vfx_core::ConfigSchema,
)]
#[serde(rename_all = "snake_case")]
pub enum CycleWrapMode {
    /// The cycle wraps: index `len` maps back to `0`.
    #[default]
    Circular,
    /// The cycle is bounded: indices outside `0..len` are not reachable.
    Bounded,
}

/// Documented named face preset.
///
/// Each preset's exact face list is part of the public contract and is
/// covered by tests. Adding a preset is an additive schema change; renaming
/// or reordering an existing preset is a breaking change.
#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    Serialize,
    Deserialize,
    tui_vfx_core::ConfigSchema,
)]
#[serde(rename_all = "snake_case")]
pub enum MechanicalCyclePreset {
    /// Decimal digits `"0"` through `"9"`.
    DecimalDigits,
    /// Current `SplitFlapCharset::Alpha` exactly: `' '`, `A`-`Z`, `0`-`9`,
    /// `'.', ',', '-', '!', '?'`.
    SplitFlapAlpha,
    /// Current `SplitFlapCharset::Digits` exactly: `' '`, `0`-`9`.
    SplitFlapDigits,
    /// Current `SplitFlapCharset::Uppercase` exactly: `' '`, `A`-`Z`.
    SplitFlapUppercase,
}

/// One entry in a weighted cycle face list.
#[derive(
    Debug,
    Clone,
    PartialEq,
    Eq,
    Serialize,
    Deserialize,
    tui_vfx_core::ConfigSchema,
)]
#[serde(deny_unknown_fields)]
pub struct WeightedCycleFace {
    /// Face value. Newlines split the face into multiple grid rows.
    pub value: String,
    /// Relative weight. Must be greater than zero. Validation enforces this
    /// before route resolution; runtime treats a zero weight as a recipe bug.
    pub weight: u16,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_source_is_pair() {
        assert_eq!(MechanicalContentSource::default(), MechanicalContentSource::Pair);
    }

    #[test]
    fn default_wrap_is_circular() {
        assert_eq!(CycleWrapMode::default(), CycleWrapMode::Circular);
    }

    #[test]
    fn pair_serde_roundtrip() {
        let cfg = MechanicalContentSource::Pair;
        let json = serde_json::to_string(&cfg).unwrap();
        assert_eq!(json, r#"{"type":"pair"}"#);
        let back: MechanicalContentSource = serde_json::from_str(&json).unwrap();
        assert_eq!(back, cfg);
    }

    #[test]
    fn ordered_serde_default_wrap_omitted_round_trips_to_circular() {
        let json = r#"{"type":"ordered","faces":["A","B","C"]}"#;
        let parsed: MechanicalContentSource = serde_json::from_str(json).unwrap();
        match parsed {
            MechanicalContentSource::Ordered { faces, wrap } => {
                assert_eq!(faces, vec!["A".to_string(), "B".to_string(), "C".to_string()]);
                assert_eq!(wrap, CycleWrapMode::Circular);
            }
            other => panic!("expected ordered, got {other:?}"),
        }
    }

    #[test]
    fn preset_decimal_digits_parses() {
        let json = r#"{"type":"preset","preset":"decimal_digits"}"#;
        let parsed: MechanicalContentSource = serde_json::from_str(json).unwrap();
        assert!(matches!(
            parsed,
            MechanicalContentSource::Preset {
                preset: MechanicalCyclePreset::DecimalDigits,
                wrap: CycleWrapMode::Circular,
                font: None,
            }
        ));
    }

    #[test]
    fn weighted_serde_roundtrip() {
        let cfg = MechanicalContentSource::Weighted {
            faces: vec![
                WeightedCycleFace { value: "7".into(), weight: 1 },
                WeightedCycleFace { value: "$".into(), weight: 2 },
            ],
            seed: 777,
            wrap: CycleWrapMode::Circular,
        };
        let json = serde_json::to_string(&cfg).unwrap();
        let back: MechanicalContentSource = serde_json::from_str(&json).unwrap();
        assert_eq!(back, cfg);
    }

    #[test]
    fn unknown_field_rejected() {
        let json = r#"{"type":"ordered","faces":["A"],"flavor":"strawberry"}"#;
        let parsed: Result<MechanicalContentSource, _> = serde_json::from_str(json);
        assert!(parsed.is_err(), "expected unknown field rejection, got {parsed:?}");
    }

    #[test]
    fn bounded_wrap_parses() {
        let json = r#"{"type":"ordered","faces":["A","B"],"wrap":"bounded"}"#;
        let parsed: MechanicalContentSource = serde_json::from_str(json).unwrap();
        assert!(matches!(
            parsed,
            MechanicalContentSource::Ordered { wrap: CycleWrapMode::Bounded, .. }
        ));
    }
}

// <FILE>tui-vfx-content/src/types/cls_mechanical_cycle_source.rs</FILE>
// <VERS>END OF VERSION: 0.1.0</VERS>
