// <FILE>src/v2/functions/fnc_validate_template_refs.rs</FILE> - <DESC>Circular reference detection for template inheritance</DESC>
// <VERS>VERSION: 1.0.0</VERS>
// <WCTX>Implementing Recipe Template Inheritance per PRD v1.1.0</WCTX>
// <CLOG>Initial creation - Prevents infinite loops from circular template references</CLOG>

use std::collections::HashSet;
use std::path::{Path, PathBuf};

/// Error type for circular reference detection
#[derive(Debug, thiserror::Error)]
#[error("Circular template reference detected\n  Chain: {}\n\nSuggestion: Check the 'extends' fields in these recipes to break the cycle.", format_chain(.chain))]
pub struct CircularReferenceError {
    pub chain: Vec<PathBuf>,
}

fn format_chain(chain: &[PathBuf]) -> String {
    chain
        .iter()
        .map(|p| p.display().to_string())
        .collect::<Vec<_>>()
        .join(" → ")
}

/// Detect circular template references by checking if a path has already been visited
/// during recursive template resolution.
///
/// # Arguments
/// * `recipe_path` - Path being checked for circular reference
/// * `visited` - Set of paths already visited in resolution chain
///
/// # Returns
/// * `Ok(())` if no cycle detected
/// * `Err(CircularReferenceError)` if cycle found, with full chain showing the cycle
///
/// # Examples
/// ```ignore
/// let mut visited = HashSet::new();
/// visited.insert(PathBuf::from("a.json"));
/// visited.insert(PathBuf::from("b.json"));
///
/// // This would detect a cycle if we try to visit "a.json" again
/// let result = validate_no_circular_ref(Path::new("a.json"), &visited);
/// assert!(result.is_err());
/// ```
pub fn validate_no_circular_ref(
    recipe_path: &Path,
    visited: &HashSet<PathBuf>,
) -> Result<(), CircularReferenceError> {
    if visited.contains(recipe_path) {
        // Cycle detected - build chain for error message
        let mut chain: Vec<PathBuf> = visited.iter().cloned().collect();
        chain.sort_by(|a, b| a.as_os_str().cmp(b.as_os_str()));
        chain.push(recipe_path.to_path_buf());

        Err(CircularReferenceError { chain })
    } else {
        Ok(())
    }
}

// <FILE>src/v2/functions/fnc_validate_template_refs.rs</FILE> - <DESC>Circular reference detection for template inheritance</DESC>
// <VERS>END OF VERSION: 1.0.0</VERS>
