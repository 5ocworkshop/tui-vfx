// <FILE>crates/tui-vfx-contract/src/cls_transition_stipple_pattern.rs</FILE> - <DESC>Stipple pattern enum for visibility.stippled tracks</DESC>
// <VERS>VERSION: 0.1.0</VERS>
// <WCTX>Phase A of canonicalize completion: model the stippled track's pattern parameter declared by the v3.1 expansion table.</WCTX>
// <CLOG>0.1.0: INIT — add ordered/bayer/blueNoise stipple pattern variants matching the schema expansion table.</CLOG>

/// Stipple pattern selection for a `visibility.stippled` transition track.
///
/// The v3.1 author-side expansion table declares this parameter as a closed
/// enum (`ordered`, `bayer`, `blueNoise`); the canonical track carries the
/// selected pattern verbatim so the runtime can dispatch on it without
/// re-parsing author-side strings.
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
pub enum TransitionStipplePattern {
    /// Deterministic ordered (matrix) stipple pattern.
    Ordered,
    /// Bayer-matrix stipple pattern.
    Bayer,
    /// Pre-computed blue-noise stipple pattern.
    BlueNoise,
}

// <FILE>crates/tui-vfx-contract/src/cls_transition_stipple_pattern.rs</FILE> - <DESC>Stipple pattern enum for visibility.stippled tracks</DESC>
// <VERS>END OF VERSION: 0.1.0</VERS>
