// <FILE>src/recipe_schema/functions/mod.rs</FILE> - <DESC>Template resolution functions module</DESC>
// <VERS>VERSION: 1.1.0</VERS>
// <WCTX>Phase 0 P0.8 adds parametric variant expansion alongside the existing extends-based template inheritance so a single recipe file can collapse a whole family via a shared template + variants array</WCTX>
// <CLOG>Register the new fnc_expand_variants module and re-export its expand_variants function and ExpandVariantsError type next to the template-inheritance helpers</CLOG>

//! Template resolution and variant expansion functions for recipe files.
//!
//! This module provides two orthogonal recipe composition mechanisms:
//!
//! 1. **Template inheritance** via the `extends` field — one recipe references
//!    another file as a base and overrides individual fields on top of it.
//!    Handled by `fnc_resolve_recipe_template` and friends.
//! 2. **Parametric variants** via a top-level `template` + `variants` array —
//!    one file expands to N concrete recipes by deep-merging each variant
//!    into the shared template. Handled by `fnc_expand_variants`.
//!
//! # Security
//! The path resolution function enforces strict security boundaries to prevent
//! path traversal attacks. All template files must reside within the project root.
//!
//! # Architecture
//! - `fnc_resolve_recipe_template` - Main orchestrator (recursive)
//! - `fnc_deep_merge_json` - JSON-level merge (preserves user intent vs serde defaults)
//! - `fnc_resolve_template_path` - Secure path resolution with validation
//! - `fnc_validate_template_refs` - Circular reference detection
//! - `fnc_expand_variants` - Parametric variant expansion (Phase 0 P0.8)

mod fnc_deep_merge_json;
mod fnc_expand_variants;
mod fnc_resolve_recipe_template;
mod fnc_resolve_template_path;
mod fnc_validate_template_refs;

// Re-export functions
pub use fnc_deep_merge_json::deep_merge_json;
pub use fnc_expand_variants::{ExpandVariantsError, expand_variants, is_variants_recipe};
pub use fnc_resolve_recipe_template::{TemplateResolutionError, resolve_recipe_with_template};
pub use fnc_resolve_template_path::{TemplatePathError, resolve_template_path};
pub use fnc_validate_template_refs::{CircularReferenceError, validate_no_circular_ref};

// <FILE>src/recipe_schema/functions/mod.rs</FILE> - <DESC>Template resolution functions module</DESC>
// <VERS>END OF VERSION: 1.1.0</VERS>
