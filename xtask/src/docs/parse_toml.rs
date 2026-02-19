// <FILE>xtask/src/docs/parse_toml.rs</FILE> - <DESC>Parse capabilities.toml</DESC>
// <VERS>VERSION: 1.1.1</VERS>
// <WCTX>Support JSON export of parsed manifest sections</WCTX>
// <CLOG>Add Serialize derives for manifest sections used in JSON output</CLOG>

use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::Path;

/// Root structure of capabilities.toml.
#[derive(Debug, Deserialize, Serialize)]
pub struct CapabilitiesManifest {
    /// Metadata about the manifest itself
    pub meta: MetaSection,

    /// Taxonomy of effect layers and phases
    pub taxonomy: TaxonomySection,

    /// Effect definitions by category
    #[serde(default)]
    pub effects: EffectsSection,

    /// Semantic mappings (moods, use cases)
    #[serde(default)]
    pub semantics: SemanticsSection,

    /// Proven effect combinations
    #[serde(default)]
    pub recipes: HashMap<String, Recipe>,

    /// Constraints and compatibility notes
    #[serde(default)]
    pub constraints: ConstraintsSection,
}

#[derive(Debug, Deserialize, Serialize)]
#[allow(dead_code)] // description reserved for future use
pub struct MetaSection {
    pub version: String,
    #[serde(default)]
    pub description: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct TaxonomySection {
    #[serde(default)]
    pub layers: HashMap<String, String>,
    #[serde(default)]
    pub phases: HashMap<String, String>,
}

#[derive(Debug, Default, Deserialize, Serialize)]
pub struct EffectsSection {
    #[serde(default)]
    pub masks: HashMap<String, EffectEntry>,
    #[serde(default)]
    pub filters: HashMap<String, EffectEntry>,
    #[serde(default)]
    pub samplers: HashMap<String, EffectEntry>,
    #[serde(default)]
    pub shaders: HashMap<String, EffectEntry>,
    #[serde(default)]
    pub styles: HashMap<String, EffectEntry>,
    #[serde(default)]
    pub content: HashMap<String, EffectEntry>,
    #[serde(default)]
    pub shadows: HashMap<String, EffectEntry>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[allow(dead_code)] // Some fields reserved for future use
pub struct EffectEntry {
    /// Brief summary (may be overridden by rustdoc)
    #[serde(default)]
    pub summary: String,

    /// Use cases for this effect
    #[serde(default)]
    pub use_cases: Vec<String>,

    /// Energy level: calm, moderate, intense
    #[serde(default)]
    pub energy: String,

    /// Complexity: simple, moderate, advanced
    #[serde(default)]
    pub complexity: String,

    /// Premium/showcase effect
    #[serde(default)]
    pub premium: bool,

    /// Hint for AI assistants
    #[serde(default)]
    pub ai_hint: String,

    /// Additional notes
    #[serde(default)]
    pub notes: EffectNotes,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct EffectNotes {
    /// Requirements (e.g., "margin_cells")
    #[serde(default)]
    pub requires: Vec<String>,

    /// Effects that pair well with this one
    #[serde(default)]
    pub pairs_with: Vec<String>,
}

#[derive(Debug, Default, Deserialize, Serialize)]
pub struct SemanticsSection {
    /// Mood to effect mappings
    #[serde(default)]
    pub moods: HashMap<String, Vec<String>>,

    /// Use case to effect mappings
    #[serde(default)]
    pub use_cases: HashMap<String, UseCaseMapping>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[allow(dead_code)] // shadow reserved for future use
pub struct UseCaseMapping {
    #[serde(default)]
    pub masks: Vec<String>,
    #[serde(default)]
    pub filters: Vec<String>,
    #[serde(default)]
    pub shaders: Vec<String>,
    #[serde(default)]
    pub content: Vec<String>,
    #[serde(default)]
    pub shadow: bool,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[allow(dead_code)] // Some fields reserved for future use
pub struct Recipe {
    pub description: String,
    #[serde(default)]
    pub mood: Vec<String>,
    #[serde(default)]
    pub effects: Vec<RecipeEffect>,
    #[serde(default)]
    pub notes: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[allow(dead_code)] // layer and params reserved for future use
pub struct RecipeEffect {
    pub layer: String,
    pub effect: String,
    #[serde(default)]
    pub params: HashMap<String, toml::Value>,
}

#[derive(Debug, Default, Deserialize, Serialize)]
pub struct ConstraintsSection {
    #[serde(default)]
    pub conflicts: Vec<Conflict>,
    #[serde(default)]
    pub performance_notes: Vec<PerformanceNote>,
    #[serde(default)]
    pub terminal_compatibility: Vec<CompatibilityNote>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Conflict {
    pub a: String,
    pub b: String,
    pub reason: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct PerformanceNote {
    pub effect: String,
    pub note: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct CompatibilityNote {
    pub effect: String,
    pub note: String,
}

/// Parse capabilities.toml from the docs directory.
pub fn parse() -> Result<CapabilitiesManifest> {
    let manifest_path = Path::new("docs/templates/capabilities.toml");

    let content = fs::read_to_string(manifest_path)
        .with_context(|| format!("Failed to read {}", manifest_path.display()))?;

    let manifest: CapabilitiesManifest =
        toml::from_str(&content).with_context(|| "Failed to parse capabilities.toml")?;

    Ok(manifest)
}

// <FILE>xtask/src/docs/parse_toml.rs</FILE> - <DESC>Parse capabilities.toml</DESC>
// <VERS>END OF VERSION: 1.1.1</VERS>
