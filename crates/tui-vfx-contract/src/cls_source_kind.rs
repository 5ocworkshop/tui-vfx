// <FILE>crates/tui-vfx-contract/src/cls_source_kind.rs</FILE> - <DESC>Surface source kind vocabulary</DESC>
// <VERS>VERSION: 0.1.0</VERS>
// <WCTX>New kernel Phase H0: distinguish source families from effects.</WCTX>
// <CLOG>0.1.0: INIT — add source kind vocabulary for text, card, image, ANSI, command-capture, procedural, asset, and custom sources.</CLOG>

/// Broad source family for a descriptor that produces an initial semantic surface.
#[derive(
    Clone, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize, schemars::JsonSchema,
)]
#[serde(tag = "kind", rename_all = "camelCase", deny_unknown_fields)]
pub enum SourceKind {
    /// Text source that lays out text into a semantic surface.
    Text,
    /// Structured card/panel source that may generate border, background, and text roles.
    Card,
    /// Image-like source material selected by typed inputs or asset refs.
    Image,
    /// ANSI/VTE-styled source material that produces cells with authored styles.
    Ansi,
    /// Offline command-capture source material; H0 declares shape but does not execute commands.
    CommandCapture,
    /// Algorithmic source driven by typed inputs and optional assets.
    Procedural,
    /// Source backed primarily by external asset material.
    AssetBacked,
    /// Source that references or produces a scene layer surface.
    SceneLayer,
    /// Project-defined source kind with explicit stable name.
    Custom {
        /// Stable custom source kind name.
        name: String,
    },
}

// <FILE>crates/tui-vfx-contract/src/cls_source_kind.rs</FILE> - <DESC>Surface source kind vocabulary</DESC>
// <VERS>END OF VERSION: 0.1.0</VERS>
