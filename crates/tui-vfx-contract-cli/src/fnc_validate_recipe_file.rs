// <FILE>crates/tui-vfx-contract-cli/src/fnc_validate_recipe_file.rs</FILE> - <DESC>Validate one canonical RecipeDocument JSON file</DESC>
// <VERS>VERSION: 0.1.0</VERS>
// <WCTX>New kernel Phase J0: deserialize and validate canonical v3.1 recipe files.</WCTX>
// <CLOG>0.1.0: INIT — add contract-only recipe file validation.</CLOG>

use std::path::Path;

use crate::{
    cls_validation_error_report::ValidationErrorReport, cls_validation_report::ValidationReport,
};
use tui_vfx_contract::RecipeDocument;

/// Validate one canonical v3.1 recipe file through serde and contract checks.
pub fn validate_recipe_file(path: &Path) -> ValidationReport {
    let path_label = path.display().to_string();
    let text = match std::fs::read_to_string(path) {
        Ok(text) => text,
        Err(error) => {
            return failed(
                path_label,
                "read",
                error.to_string(),
                serde_json::Value::Null,
            );
        }
    };
    let recipe: RecipeDocument = match serde_json::from_str(&text) {
        Ok(recipe) => recipe,
        Err(error) => {
            return failed(
                path_label,
                "deserialize",
                error.to_string(),
                serde_json::json!({ "line": error.line(), "column": error.column() }),
            );
        }
    };
    match recipe.validate() {
        Ok(()) => ValidationReport {
            path: path_label,
            valid: true,
            errors: vec![],
        },
        Err(error) => failed(
            path_label,
            "contract",
            format!("{error:?}"),
            serde_json::to_value(error).unwrap_or(serde_json::Value::Null),
        ),
    }
}

fn failed(
    path: String,
    stage: &'static str,
    message: String,
    details: serde_json::Value,
) -> ValidationReport {
    ValidationReport {
        path,
        valid: false,
        errors: vec![ValidationErrorReport {
            stage,
            message,
            details,
        }],
    }
}

// <FILE>crates/tui-vfx-contract-cli/src/fnc_validate_recipe_file.rs</FILE> - <DESC>Validate one canonical RecipeDocument JSON file</DESC>
// <VERS>END OF VERSION: 0.1.0</VERS>
