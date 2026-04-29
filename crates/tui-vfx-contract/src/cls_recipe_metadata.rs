// <FILE>crates/tui-vfx-contract/src/cls_recipe_metadata.rs</FILE> - <DESC>Canonical recipe human metadata DTO</DESC>
// <VERS>VERSION: 0.1.0</VERS>
// <WCTX>New kernel Phase H1: carry human-facing recipe metadata without authoring syntax.</WCTX>
// <CLOG>0.1.0: INIT — add metadata carried by canonical recipe documents.</CLOG>

/// Human-facing metadata for a canonical recipe document.
#[derive(
    Clone, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize, schemars::JsonSchema,
)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct RecipeMetadata {
    /// Optional human-facing recipe title.
    pub title: Option<String>,
    /// Optional longer recipe description.
    pub description: Option<String>,
    /// Optional authors or owning teams.
    #[serde(default)]
    pub authors: Vec<String>,
    /// Optional catalog/search tags.
    #[serde(default)]
    pub tags: Vec<String>,
}

// <FILE>crates/tui-vfx-contract/src/cls_recipe_metadata.rs</FILE> - <DESC>Canonical recipe human metadata DTO</DESC>
// <VERS>END OF VERSION: 0.1.0</VERS>
