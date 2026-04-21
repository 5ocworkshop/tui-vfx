// <FILE>src/v2/functions/fnc_resolve_recipe_template.rs</FILE> - <DESC>Main template resolution orchestrator for V2 recipes</DESC>
// <VERS>VERSION: 1.0.0</VERS>
// <WCTX>Implementing Recipe Template Inheritance per PRD v1.1.0</WCTX>
// <CLOG>Initial creation - Orchestrates recursive template resolution with security checks</CLOG>

use super::fnc_resolve_template_path::TemplatePathError;
use super::fnc_validate_template_refs::CircularReferenceError;
use super::{deep_merge_json, resolve_template_path, validate_no_circular_ref};
use serde_json::Value;
use std::collections::HashSet;
use std::fs;
use std::path::{Path, PathBuf};

/// Maximum recursion depth for template inheritance (prevents stack overflow)
const MAX_TEMPLATE_DEPTH: usize = 10;

/// Errors that can occur during template resolution
#[derive(Debug, thiserror::Error)]
pub enum TemplateResolutionError {
    #[error("Circular template reference: {0}")]
    CircularReference(#[from] CircularReferenceError),

    #[error("Template path error: {0}")]
    PathError(#[from] TemplatePathError),

    #[error("Failed to read template file {path}: {source}")]
    IoError {
        path: PathBuf,
        source: std::io::Error,
    },

    #[error("Invalid JSON in {path}: {source}")]
    InvalidJson {
        path: PathBuf,
        source: serde_json::Error,
    },

    #[error(
        "Maximum template inheritance depth ({MAX_TEMPLATE_DEPTH}) exceeded\n  This usually indicates a circular reference that was not caught."
    )]
    MaxDepthExceeded,
}

/// Recursively resolve a recipe's template inheritance chain, returning a fully-merged
/// JSON Value ready for deserialization.
///
/// # Process
/// 1. Check for circular references (prevents infinite loops)
/// 2. Load recipe JSON file
/// 3. Check for "extends" field
/// 4. If extends present:
///    - Resolve template path (with security validation)
///    - Recursively load template (supports multi-level inheritance)
///    - Deep merge template (base) with recipe (overlay)
/// 5. Return merged JSON Value (NOT yet deserialized to struct)
///
/// # Arguments
/// * `recipe_path` - Path to recipe JSON file
/// * `project_root` - Project root for security validation
/// * `visited` - Set of visited paths for circular reference detection
/// * `depth` - Current recursion depth
///
/// # Returns
/// Fully resolved and merged recipe as `serde_json::Value`
///
/// # Errors
/// * `CircularReference` - Template chain contains a cycle
/// * `PathError` - Template path invalid or escapes project root
/// * `IoError` - Cannot read template file
/// * `InvalidJson` - Template or recipe has malformed JSON
/// * `MaxDepthExceeded` - Template chain too deep (>10 levels)
pub fn resolve_recipe_with_template(
    recipe_path: &Path,
    project_root: &Path,
    visited: &mut HashSet<PathBuf>,
    depth: usize,
) -> Result<Value, TemplateResolutionError> {
    // Depth check (prevent stack overflow)
    if depth > MAX_TEMPLATE_DEPTH {
        return Err(TemplateResolutionError::MaxDepthExceeded);
    }

    // Circular reference check
    validate_no_circular_ref(recipe_path, visited)?;

    // Add to visited set
    visited.insert(recipe_path.to_path_buf());

    // Load recipe JSON
    let recipe_json_str =
        fs::read_to_string(recipe_path).map_err(|e| TemplateResolutionError::IoError {
            path: recipe_path.to_path_buf(),
            source: e,
        })?;

    let mut recipe_json: Value = serde_json::from_str(&recipe_json_str).map_err(|e| {
        TemplateResolutionError::InvalidJson {
            path: recipe_path.to_path_buf(),
            source: e,
        }
    })?;

    // Check for extends field
    if let Some(extends_value) = recipe_json.get("extends") {
        if let Some(template_ref) = extends_value.as_str() {
            if !template_ref.is_empty() {
                // Resolve template path (with security validation)
                let template_path = resolve_template_path(project_root, recipe_path, template_ref)?;

                // Recursively load template
                let template_json =
                    resolve_recipe_with_template(&template_path, project_root, visited, depth + 1)?;

                // Deep merge: template (base) + recipe (overlay)
                recipe_json = deep_merge_json(template_json, recipe_json);
            }
        }
    }

    // Remove extends field from final result (no longer needed)
    if let Some(obj) = recipe_json.as_object_mut() {
        obj.remove("extends");
    }

    Ok(recipe_json)
}

// <FILE>src/recipe_schema/functions/fnc_resolve_recipe_template.rs</FILE> - <DESC>Main template resolution orchestrator for V2 recipes</DESC>
// <VERS>END OF VERSION: 1.1.0</VERS>
