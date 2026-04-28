// <FILE>crates/tui-vfx-contract/src/cls_surface_metadata.rs</FILE> - <DESC>Semantic surface metadata DTO</DESC>
// <VERS>VERSION: 0.4.0</VERS>
// <WCTX>New kernel Phase D0 schema/reference backfill after Phase C preflight OFPF split.</WCTX>
// <CLOG>0.4.0: PATCH — add Serde/Schemars schema-reference readiness while preserving runtime behavior.
// 0.3.0: REFACTOR — extract SurfaceMetadata DTO.</CLOG>

/// Metadata attached to a semantic surface.
#[derive(
    Clone, Debug, Default, PartialEq, Eq, serde::Serialize, serde::Deserialize, schemars::JsonSchema,
)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct SurfaceMetadata {
    /// Optional producer name for diagnostics and tests.
    pub producer: Option<String>,
    /// Optional semantic layer label for future scene-layer integration.
    pub layer: Option<String>,
}

// <FILE>crates/tui-vfx-contract/src/cls_surface_metadata.rs</FILE> - <DESC>Semantic surface metadata DTO</DESC>
// <VERS>END OF VERSION: 0.4.0</VERS>
