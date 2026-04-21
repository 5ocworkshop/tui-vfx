// <FILE>src/recipe_schema/validator/mod.rs</FILE> - <DESC>Validation helpers for additive scene/continuous recipe blocks</DESC>
// <VERS>VERSION: 0.1.0</VERS>
// <WCTX>Sub-plan B Phase B.1 — additive validators for the new scene and continuous schema blocks.</WCTX>
// <CLOG>0.1.0: add validation issue types plus scene/continuous block validators.</CLOG>

mod fnc_validate_continuous_block;
mod fnc_validate_scene_block;

pub use fnc_validate_continuous_block::validate_continuous_block;
pub use fnc_validate_scene_block::validate_scene_block;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ValidationSeverity {
    Warning,
    Error,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ValidationIssue {
    pub severity: ValidationSeverity,
    pub message: String,
}

impl ValidationIssue {
    pub fn warning(message: impl Into<String>) -> Self {
        Self {
            severity: ValidationSeverity::Warning,
            message: message.into(),
        }
    }

    pub fn error(message: impl Into<String>) -> Self {
        Self {
            severity: ValidationSeverity::Error,
            message: message.into(),
        }
    }
}

// <FILE>src/recipe_schema/validator/mod.rs</FILE> - <DESC>Recipe-schema validators</DESC>
// <VERS>END OF VERSION: 0.1.0</VERS>
