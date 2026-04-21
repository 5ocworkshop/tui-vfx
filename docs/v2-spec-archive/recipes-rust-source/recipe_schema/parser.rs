// <FILE>src/v2/parser.rs</FILE> - <DESC>V2 JSON recipe parser with template inheritance support</DESC>
// <VERS>VERSION: 2.1.0</VERS>
// <WCTX>feat-20251224-135811: Recipe Template Inheritance</WCTX>
// <CLOG>MINOR: Added json_recipe_dyn_from_file with template resolution support; extends field added to RaJsonRecipeDefinition</CLOG>

use crate::recipe_schema::RaRecipeConfig;
use crate::recipe_schema::functions::resolve_recipe_with_template;
use crate::recipes::{RecipeMeta, RustSnippet};
use crate::registry::DynRecipe;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashSet;
use std::path::Path;
use std::sync::Arc;

/// V2 JSON recipe definition matching the schema.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RaJsonRecipeDefinition {
    pub schema_version: u32,

    /// Optional: Path to base template recipe to extend.
    /// If present, this recipe inherits all fields from the template,
    /// overriding only the fields explicitly specified here.
    ///
    /// Path can be relative (to recipe's directory) or absolute (from project root).
    /// Example: `"themes/computer_base.json"` or `"/themes/global/base.json"`
    #[serde(skip_serializing_if = "Option::is_none")]
    pub extends: Option<String>,

    pub id: String,
    pub title: String,
    pub description: String,
    pub version: Option<Value>,
    pub last_updated: Option<Value>,
    pub config: RaRecipeConfig,
}

/// V2 DynRecipe implementation.
pub struct RaJsonRecipeDyn {
    meta: RecipeMeta,
    config: RaRecipeConfig,
}

impl DynRecipe for RaJsonRecipeDyn {
    fn meta(&self) -> RecipeMeta {
        self.meta.clone()
    }
    fn schema(&self) -> tui_vfx_core::SchemaNode {
        <RaRecipeConfig as tui_vfx_core::ConfigSchema>::schema()
    }
    fn default_config_json(&self) -> Value {
        serde_json::to_value(&self.config).unwrap_or(Value::Null)
    }
    fn export_rust_from_json(&self, cfg: &Value) -> Option<RustSnippet> {
        let _parsed: RaRecipeConfig = serde_json::from_value(cfg.clone()).ok()?;
        // V2 Rust snippet generation
        Some(RustSnippet {
            uses: vec!["use tui_vfx_recipes::recipe_schema::RaRecipeConfig;".to_string()],
            body: "// V2 config - use RaRecipeConfig methods to build notification".to_string(),
        })
    }
}

/// Parse a JSON recipe string into a DynRecipe.
///
/// Detects schema_version and parses accordingly:
/// - schema_version >= 1: Parse as Ra recipe
/// - schema_version 0 or missing: Returns error
pub fn json_recipe_dyn(json: &str) -> Result<Box<dyn DynRecipe + Send + Sync>, String> {
    let value: Value = serde_json::from_str(json).map_err(|e| {
        let snippet = if json.len() > 200 {
            format!("{}...", &json[..200])
        } else {
            json.to_string()
        };
        format!("Failed to parse JSON recipe: {}. Input: {}", e, snippet)
    })?;

    let id_hint = value
        .get("id")
        .and_then(|v| v.as_str())
        .unwrap_or("<unknown>")
        .to_string();
    let title_hint = value
        .get("title")
        .and_then(|v| v.as_str())
        .unwrap_or("<unknown>")
        .to_string();

    // Detect schema version
    let schema_version = value
        .get("schema_version")
        .and_then(|v| v.as_u64())
        .unwrap_or(1) as u32;

    if schema_version >= 1 {
        // Parse as V2 recipe
        let def: RaJsonRecipeDefinition = serde_json::from_value(value).map_err(|e| {
            format!(
                "Failed to parse V2 JSON recipe definition for id={} title={}: {}",
                id_hint, title_hint, e
            )
        })?;
        let meta = RecipeMeta {
            id: Arc::from(def.id),
            title: Arc::from(def.title),
            description: Arc::from(def.description),
        };
        Ok(Box::new(RaJsonRecipeDyn {
            meta,
            config: def.config,
        }))
    } else {
        // V1 recipes are deprecated - direct users to ratatui-notifications
        Err(format!(
            "V1 recipes are deprecated. Recipe id={} title={} uses schema_version=1. \
             Use ratatui_notifications::recipes::json_recipe_dyn() for V1 compatibility.",
            id_hint, title_hint
        ))
    }
}

/// Parse a V2 JSON recipe from a file path, with automatic template resolution.
///
/// This function handles template inheritance via the `extends` field. If a recipe
/// references a template, it will be recursively loaded and merged before parsing.
///
/// # Arguments
/// * `recipe_path` - Path to the JSON recipe file
/// * `project_root` - Project root directory for security validation (templates must be within this)
///
/// # Returns
/// A `DynRecipe` trait object ready for use with AnimationManager
///
/// # Errors
/// Returns error if:
/// - Recipe file cannot be read
/// - JSON is malformed
/// - Template resolution fails (circular ref, path traversal, template not found)
/// - Recipe schema is invalid
///
/// # Example
/// ```ignore
/// use std::path::Path;
///
/// let project_root = Path::new("/usr/projects/tui-vfx-recipes");
/// let recipe_path = Path::new("/usr/projects/tui-vfx-recipes/recipes/greeting.json");
///
/// let dyn_recipe = json_recipe_dyn_from_file(recipe_path, project_root)?;
/// ```
pub fn json_recipe_dyn_from_file(
    recipe_path: &Path,
    project_root: &Path,
) -> Result<Box<dyn DynRecipe + Send + Sync>, String> {
    // Resolve templates recursively
    let mut visited = HashSet::new();
    let resolved_json = resolve_recipe_with_template(recipe_path, project_root, &mut visited, 0)
        .map_err(|e| format!("Template resolution failed for {:?}: {}", recipe_path, e))?;

    // Convert JSON Value to string for json_recipe_dyn
    let json_str = serde_json::to_string(&resolved_json)
        .map_err(|e| format!("Failed to serialize resolved recipe: {}", e))?;

    // Parse using existing string-based parser
    json_recipe_dyn(&json_str)
}

// <FILE>src/recipe_schema/parser.rs</FILE> - <DESC>V2 JSON recipe parser with template inheritance support</DESC>
// <VERS>END OF VERSION: 2.2.0</VERS>
