// <FILE>src/recipe_schema/mod.rs</FILE> - <DESC>Recipe schema module with template inheritance and additive scene/continuous blocks</DESC>
// <VERS>VERSION: 1.3.0</VERS>
// <WCTX>Sub-plan B Phase B.1 — extend the recipe schema with additive scene and continuous blocks while preserving the existing template-inheritance API.</WCTX>
// <CLOG>1.3.0: add scene + validator modules, RaContinuousConfig, RaClock, and public re-exports for the additive Sub-plan B schema surface.
// 1.2.0: Added interactions module for interactive element schema</CLOG>

//! V2 recipe schema and parser with template inheritance support.
//!
//! This module provides the V2 configuration schema and parsing functions.
//! V2 recipes support template inheritance via the `extends` field, enabling
//! DRY recipe authoring by inheriting layout, styling, and effects from base templates.
//!
//! # Template Inheritance
//! Recipes can reference template files using the `extends` field:
//! ```json
//! {
//!   "schema_version": 2,
//!   "extends": "themes/computer_base.json",
//!   "message": "Hello, World!"
//! }
//! ```
//!
//! # Modules
//! - `config` - V2 configuration types and enums
//! - `parser` - JSON recipe parsing with template resolution (sync + async)
//! - `functions` - Template resolution utilities
//! - `interactions` - Interactive element schema types

pub mod config;
pub mod functions;
pub mod interactions;
pub mod parser;
pub mod scene;
pub mod validator;

pub mod cls_ra_continuous_config;
pub mod enum_ra_clock;

pub use cls_ra_continuous_config::RaContinuousConfig;
pub use config::*;
pub use enum_ra_clock::RaClock;
pub use parser::{json_recipe_dyn, json_recipe_dyn_from_file};
pub use scene::*;
pub use validator::{
    ValidationIssue, ValidationSeverity, validate_continuous_block, validate_scene_block,
};

// Re-export template resolution functions for advanced use cases
pub use functions::{
    CircularReferenceError, TemplatePathError, TemplateResolutionError, deep_merge_json,
    resolve_recipe_with_template, resolve_template_path, validate_no_circular_ref,
};

// <FILE>src/recipe_schema/mod.rs</FILE> - <DESC>Recipe schema module with template inheritance and additive scene/continuous blocks</DESC>
// <VERS>END OF VERSION: 1.3.0</VERS>
