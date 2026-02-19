// <FILE>xtask/src/docs/parse_api_toml.rs</FILE> - <DESC>Parse api_docs.toml</DESC>
// <VERS>VERSION: 1.1.0</VERS>
// <WCTX>Documentation file reorganization</WCTX>
// <CLOG>Move TOML to docs/templates/, add allow(dead_code) for TOML fields</CLOG>

use anyhow::{Context, Result};
use serde::Deserialize;
use std::collections::HashMap;
use std::fs;
use std::path::Path;

/// Root structure of api_docs.toml.
#[derive(Debug, Deserialize)]
#[allow(dead_code)] // Fields needed for TOML deserialization
pub struct ApiDocsManifest {
    /// Metadata about the manifest itself
    pub meta: MetaSection,

    /// Document structure (title, intro, parts)
    pub structure: StructureSection,

    /// Effect inventory configuration
    #[serde(default)]
    pub inventory: InventorySection,

    /// Entry point function documentation
    #[serde(default)]
    pub entry_points: EntryPointsSection,

    /// Core type documentation
    #[serde(default)]
    pub types: HashMap<String, TypeEntry>,

    /// Timing documentation
    #[serde(default)]
    pub timing: TimingSection,

    /// Render order documentation
    #[serde(default)]
    pub render_order: RenderOrderSection,

    /// Spec documentation (MaskSpec, FilterSpec, etc.)
    #[serde(default)]
    pub specs: HashMap<String, SpecEntry>,

    /// Prelude documentation
    #[serde(default)]
    pub prelude: PreludeSection,

    /// Code examples
    #[serde(default)]
    pub examples: HashMap<String, ExampleEntry>,
}

#[derive(Debug, Deserialize)]
#[allow(dead_code)] // Fields needed for TOML deserialization
pub struct MetaSection {
    pub version: String,
    #[serde(default)]
    pub description: String,
}

#[derive(Debug, Deserialize)]
#[allow(dead_code)] // Fields needed for TOML deserialization
pub struct StructureSection {
    pub title: String,
    pub intro: String,
    #[serde(default)]
    pub parts: HashMap<String, PartEntry>,
}

#[derive(Debug, Clone, Deserialize)]
#[allow(dead_code)] // Fields needed for TOML deserialization
pub struct PartEntry {
    pub order: u32,
    pub title: String,
    #[serde(default)]
    pub description: String,
}

#[derive(Debug, Default, Deserialize)]
#[allow(dead_code)] // Fields needed for TOML deserialization
pub struct InventorySection {
    #[serde(default)]
    pub description: String,
    #[serde(default)]
    pub categories: HashMap<String, InventoryCategory>,
}

#[derive(Debug, Clone, Deserialize)]
#[allow(dead_code)] // Fields needed for TOML deserialization
pub struct InventoryCategory {
    pub api_path: String,
}

#[derive(Debug, Default, Deserialize)]
#[allow(dead_code)] // Fields needed for TOML deserialization
pub struct EntryPointsSection {
    #[serde(default)]
    pub section_title: String,
    #[serde(flatten)]
    pub functions: HashMap<String, EntryPointEntry>,
}

#[derive(Debug, Clone, Default, Deserialize)]
#[allow(dead_code)] // Fields needed for TOML deserialization
pub struct EntryPointEntry {
    #[serde(default)]
    pub order: u32,
    #[serde(default)]
    pub subtitle: String,
    #[serde(default)]
    pub description: String,
    #[serde(default)]
    pub notes: String,
}

#[derive(Debug, Clone, Deserialize)]
#[allow(dead_code)] // Fields needed for TOML deserialization
pub struct TypeEntry {
    #[serde(default)]
    pub part: String,
    #[serde(default)]
    pub subtitle: String,
    #[serde(default)]
    pub description: String,
    #[serde(default)]
    pub notes: String,
    #[serde(default)]
    pub builder_methods: Vec<String>,
    #[serde(default)]
    pub builder_title: String,
}

#[derive(Debug, Default, Deserialize)]
pub struct TimingSection {
    #[serde(default)]
    pub section_title: String,
    #[serde(default)]
    pub t: String,
    #[serde(default)]
    pub loop_t: String,
    #[serde(default)]
    pub phase: String,
}

#[derive(Debug, Default, Deserialize)]
#[allow(dead_code)] // Fields needed for TOML deserialization
pub struct RenderOrderSection {
    #[serde(default)]
    pub section_title: String,
    #[serde(default)]
    pub steps: Vec<RenderOrderStep>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct RenderOrderStep {
    pub order: u32,
    pub name: String,
    #[serde(default)]
    pub description: String,
}

#[derive(Debug, Clone, Default, Deserialize)]
#[allow(dead_code)] // Fields needed for TOML deserialization
pub struct SpecEntry {
    #[serde(default)]
    pub part: String,
    #[serde(default)]
    pub section_title: String,
    #[serde(default)]
    pub description: String,
    #[serde(default)]
    pub intro: String,
    #[serde(default)]
    pub variants: Vec<toml::Value>,
    #[serde(default)]
    pub convenience: Vec<String>,
    #[serde(default)]
    pub rule: String,
    #[serde(default)]
    pub example: String,
    #[serde(default)]
    pub see_also: String,
    #[serde(default)]
    pub notes: HashMap<String, SpecNote>,
    #[serde(default)]
    pub enums: SpecEnums,
    #[serde(default)]
    pub types: HashMap<String, SpecType>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(untagged)]
#[allow(dead_code)] // Variants needed for TOML deserialization
pub enum SpecVariant {
    Simple(String),
    WithNote { name: String, note: String },
}

#[derive(Debug, Clone, Default, Deserialize)]
pub struct SpecNote {
    #[serde(default)]
    pub title: String,
    #[serde(default)]
    pub content: String,
}

#[derive(Debug, Clone, Default, Deserialize)]
#[allow(dead_code)] // Fields needed for TOML deserialization
pub struct SpecEnums {
    #[serde(default)]
    pub title: String,
    #[serde(default)]
    pub items: Vec<EnumItem>,
    #[serde(default)]
    pub variants: Vec<String>,
    #[serde(default)]
    pub default: String,
    #[serde(default)]
    pub aliases: Vec<String>,
    #[serde(default)]
    pub notes: String,
    #[serde(default)]
    pub groups: Vec<EnumGroup>,
    #[serde(default)]
    pub table: Vec<TableRow>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct EnumItem {
    pub name: String,
    pub variants: Vec<String>,
}

#[derive(Debug, Clone, Deserialize)]
#[allow(dead_code)] // Fields needed for TOML deserialization
pub struct EnumGroup {
    pub name: String,
    pub variants: Vec<String>,
}

#[derive(Debug, Clone, Deserialize)]
#[allow(dead_code)] // Fields needed for TOML deserialization
pub struct TableRow {
    pub mode: String,
    pub behavior: String,
}

#[derive(Debug, Clone, Deserialize)]
#[allow(dead_code)] // Fields needed for TOML deserialization
pub struct SpecType {
    pub kind: String,
    #[serde(default)]
    pub fields: Vec<String>,
}

#[derive(Debug, Default, Deserialize)]
#[allow(dead_code)] // Fields needed for TOML deserialization
pub struct PreludeSection {
    #[serde(default)]
    pub section_title: String,
    #[serde(default)]
    pub recommendation: String,
    #[serde(default)]
    pub usage: String,
    #[serde(default)]
    pub exports: PreludeExports,
}

#[derive(Debug, Default, Deserialize)]
pub struct PreludeExports {
    #[serde(default)]
    pub types: String,
    #[serde(default)]
    pub core_schema: String,
    #[serde(default)]
    pub geometry: String,
    #[serde(default)]
    pub compositor_pipeline: String,
    #[serde(default)]
    pub compositor_types: String,
    #[serde(default)]
    pub style: String,
    #[serde(default)]
    pub content: String,
    #[serde(default)]
    pub shadows: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct ExampleEntry {
    pub title: String,
    pub code: String,
}

/// Parse api_docs.toml from the docs directory.
pub fn parse() -> Result<ApiDocsManifest> {
    let manifest_path = Path::new("docs/templates/api_docs.toml");

    let content = fs::read_to_string(manifest_path)
        .with_context(|| format!("Failed to read {}", manifest_path.display()))?;

    let manifest: ApiDocsManifest =
        toml::from_str(&content).with_context(|| "Failed to parse api_docs.toml")?;

    Ok(manifest)
}

// <FILE>xtask/src/docs/parse_api_toml.rs</FILE> - <DESC>Parse api_docs.toml</DESC>
// <VERS>END OF VERSION: 1.1.0</VERS>
