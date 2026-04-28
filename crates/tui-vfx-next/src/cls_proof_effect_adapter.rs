// <FILE>crates/tui-vfx-next/src/cls_proof_effect_adapter.rs</FILE> - <DESC>Toy proof effect adapter enum</DESC>
// <VERS>VERSION: 0.1.0</VERS>
// <WCTX>New kernel Phase G2: map contract EffectId values to toy executors.</WCTX>
// <CLOG>0.1.0: INIT — add proof-only adapter choices, not production effect ports.</CLOG>

/// Toy proof adapter registered for one effect id.
#[derive(Clone, Copy, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum ProofEffectAdapter {
    /// Copy sampled source cells and roles through the node scope.
    Copy,
    /// Replace matched cell glyphs with a resolved input glyph.
    ReplaceGlyph,
    /// Dim matched foreground/background colors by a resolved numeric factor.
    Dim,
    /// Write a resolved semantic role into matched destination cells.
    ExplicitRoleWrite,
}

// <FILE>crates/tui-vfx-next/src/cls_proof_effect_adapter.rs</FILE> - <DESC>Toy proof effect adapter enum</DESC>
// <VERS>END OF VERSION: 0.1.0</VERS>
