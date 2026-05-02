// <FILE>crates/tui-vfx-contract/src/cls_transition_preset.rs</FILE> - <DESC>Author-facing transition preset vocabulary</DESC>
// <VERS>VERSION: 0.3.0</VERS>
// <WCTX>Phase A of canonicalize completion: add a compose intent variant so multi-track transitions carry author-meaningful labels rather than first-sub-preset aliases.</WCTX>
// <CLOG>0.3.0: MINOR — add compose preset variant for compose-form (multi-track) transitions.</CLOG>

/// Author-facing transition preset names preserved as non-executable intent metadata.
///
/// Each variant names a closed-vocabulary transition shape declared by the
/// v3.1 author-side expansion table. The `compose` variant labels
/// multi-track transitions written with the `compose: "<mode>"` author form;
/// the executable structure (multiple tracks under one envelope) is already
/// expressed by `TransitionSpec.tracks` regardless of intent.
#[derive(
    Clone,
    Copy,
    Debug,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    serde::Serialize,
    serde::Deserialize,
    schemars::JsonSchema,
)]
#[serde(rename_all = "camelCase")]
pub enum TransitionPreset {
    /// Between-surface blend from prior to next surface.
    Crossfade,
    /// Opacity-only reveal between alpha 0 and 1.
    Fade,
    /// Directional coverage reveal.
    Wipe,
    /// Aperture reveal around a focal point.
    Iris,
    /// Banded slat reveal across the surface.
    Blinds,
    /// Coordinated from/to surface displacement.
    Push,
    /// Per-cell randomized or ordered visibility reveal.
    Dissolve,
    /// Between-surface content correspondence transform.
    Morph,
    /// Stipple-pattern visibility reveal.
    Stippled,
    /// Braille-pattern visibility reveal.
    Braille,
    /// Multi-track composed transition; track list expresses the executable shape.
    Compose,
}

// <FILE>crates/tui-vfx-contract/src/cls_transition_preset.rs</FILE> - <DESC>Author-facing transition preset vocabulary</DESC>
// <VERS>END OF VERSION: 0.3.0</VERS>
