// <FILE>crates/tui-vfx-contract/src/cls_transition_preset.rs</FILE> - <DESC>Author-facing transition preset vocabulary</DESC>
// <VERS>VERSION: 0.2.0</VERS>
// <WCTX>Author shorthand canonicalize: cover the corpus-witnessed fade and blinds preset names.</WCTX>
// <CLOG>0.2.0: MINOR — add fade and blinds preset variants for corpus coverage.
// 0.1.0: INIT — add classic transition preset enum.</CLOG>

/// Author-facing transition preset names preserved as non-executable intent metadata.
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
}

// <FILE>crates/tui-vfx-contract/src/cls_transition_preset.rs</FILE> - <DESC>Author-facing transition preset vocabulary</DESC>
// <VERS>END OF VERSION: 0.2.0</VERS>
