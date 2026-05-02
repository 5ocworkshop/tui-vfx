// <FILE>crates/tui-vfx-contract/src/cls_transition_braille_order.rs</FILE> - <DESC>Subcell traversal order enum for visibility.braille tracks</DESC>
// <VERS>VERSION: 0.1.0</VERS>
// <WCTX>Phase A of canonicalize completion: model the braille track's subcellOrder parameter declared by the v3.1 expansion table.</WCTX>
// <CLOG>0.1.0: INIT — add raster/morton/spiral subcell order variants matching the schema expansion table.</CLOG>

/// Subcell traversal order for a `visibility.braille` transition track.
///
/// Each braille cell holds eight dot positions; the canonical track lights
/// them in a deterministic order so the reveal pattern is reproducible. The
/// v3.1 author-side expansion table declares this parameter as a closed
/// enum (`raster`, `morton`, `spiral`).
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
pub enum TransitionBrailleOrder {
    /// Row-major raster traversal order.
    Raster,
    /// Z-order (Morton) curve traversal.
    Morton,
    /// Spiral traversal expanding outward from a central anchor.
    Spiral,
}

// <FILE>crates/tui-vfx-contract/src/cls_transition_braille_order.rs</FILE> - <DESC>Subcell traversal order enum for visibility.braille tracks</DESC>
// <VERS>END OF VERSION: 0.1.0</VERS>
