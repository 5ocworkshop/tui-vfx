// <FILE>src/v2/functions/fnc_resolve_template_path.rs</FILE> - <DESC>Secure template path resolution with path traversal prevention</DESC>
// <VERS>VERSION: 1.0.1</VERS>
// <WCTX>Clippy cleanup for template path resolution</WCTX>
// <CLOG>Use strip_prefix for absolute template refs</CLOG>

use std::path::{Path, PathBuf};

/// Error types for template path resolution
#[derive(Debug, thiserror::Error)]
pub enum TemplatePathError {
    #[error(
        "Path traversal attempt detected\n  Template reference: {template}\n  Resolved to: {resolved:?}\n  Project root: {project_root:?}\n\nSecurity policy: Templates must reside within the project root directory."
    )]
    PathTraversal {
        template: String,
        resolved: PathBuf,
        project_root: PathBuf,
    },

    #[error(
        "Template not found: {path:?}\n\nEnsure the template file exists and the path is correct."
    )]
    TemplateNotFound { path: PathBuf },

    #[error("Invalid recipe path: {0:?}")]
    InvalidRecipePath(PathBuf),

    #[error("Invalid project root: {0:?}")]
    InvalidProjectRoot(PathBuf),
}

/// Resolve template path reference from recipe, converting relative/absolute paths
/// to canonical filesystem paths while enforcing CRITICAL security constraint:
/// templates MUST reside within project root (prevent path traversal attacks).
///
/// # Arguments
/// * `project_root` - Absolute path to project root directory
/// * `recipe_path` - Absolute path to recipe file containing extends reference
/// * `template_ref` - Template path string from `extends` field (relative or absolute)
///
/// # Returns
/// Canonicalized absolute path to template file, validated to be within project_root
///
/// # Errors
/// * `PathTraversal` - Template reference attempts to escape project root
/// * `TemplateNotFound` - Referenced template file doesn't exist
/// * `InvalidRecipePath` - Recipe path is malformed or has no parent
/// * `InvalidProjectRoot` - Project root cannot be canonicalized
///
/// # Security
/// This function is a CRITICAL security boundary. It prevents path traversal attacks
/// like `../../../../etc/passwd` or symlinks pointing outside the project root.
///
/// # Examples
/// ```ignore
/// let project_root = Path::new("/usr/projects/tui-vfx-recipes");
/// let recipe_path = Path::new("/usr/projects/tui-vfx-recipes/recipes/greeting.json");
/// let template_ref = "themes/computer_base.json";
///
/// let template_path = resolve_template_path(project_root, recipe_path, template_ref)?;
/// // Returns: /usr/projects/tui-vfx-recipes/recipes/themes/computer_base.json
/// ```
pub fn resolve_template_path(
    project_root: &Path,
    recipe_path: &Path,
    template_ref: &str,
) -> Result<PathBuf, TemplatePathError> {
    // 1. Build candidate path
    let candidate = if let Some(stripped) = template_ref.strip_prefix('/') {
        // Absolute path reference → join with project_root (strip leading /)
        project_root.join(stripped)
    } else {
        // Relative path → resolve relative to recipe's directory
        let recipe_dir = recipe_path
            .parent()
            .ok_or_else(|| TemplatePathError::InvalidRecipePath(recipe_path.to_path_buf()))?;
        recipe_dir.join(template_ref)
    };

    // 2. Canonicalize paths (resolve symlinks, normalize .. and .)
    let canonical_template =
        candidate
            .canonicalize()
            .map_err(|_| TemplatePathError::TemplateNotFound {
                path: candidate.clone(),
            })?;

    let canonical_project_root = project_root
        .canonicalize()
        .map_err(|_| TemplatePathError::InvalidProjectRoot(project_root.to_path_buf()))?;

    // 3. SECURITY CHECK - Path Traversal Prevention
    if !canonical_template.starts_with(&canonical_project_root) {
        return Err(TemplatePathError::PathTraversal {
            template: template_ref.to_string(),
            resolved: canonical_template,
            project_root: canonical_project_root,
        });
    }

    Ok(canonical_template)
}

// <FILE>src/v2/functions/fnc_resolve_template_path.rs</FILE> - <DESC>Secure template path resolution with path traversal prevention</DESC>
// <VERS>END OF VERSION: 1.0.1</VERS>
