// <FILE>crates/tui-vfx-contract/src/cls_transition_match_kind.rs</FILE> - <DESC>Correspondence match kind enum for relation.morph tracks</DESC>
// <VERS>VERSION: 0.1.0</VERS>
// <WCTX>Phase A of canonicalize completion: model the morph track's match parameter declared by the v3.1 expansion table.</WCTX>
// <CLOG>0.1.0: INIT — add glyph/block/outline match kind variants matching the schema expansion table.</CLOG>

/// Correspondence match kind for a `relation.morph` transition track.
///
/// Selects how the runtime pairs source and target cells for the morph
/// transform: per-glyph identity matching, contiguous-block matching, or
/// silhouette/outline matching. The v3.1 author-side expansion table
/// declares this parameter as a closed enum (`glyph`, `block`, `outline`).
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
pub enum TransitionMatchKind {
    /// Match cells with the same glyph identity across surfaces.
    Glyph,
    /// Match contiguous cell blocks across surfaces.
    Block,
    /// Match the surface silhouette/outline across surfaces.
    Outline,
}

// <FILE>crates/tui-vfx-contract/src/cls_transition_match_kind.rs</FILE> - <DESC>Correspondence match kind enum for relation.morph tracks</DESC>
// <VERS>END OF VERSION: 0.1.0</VERS>
