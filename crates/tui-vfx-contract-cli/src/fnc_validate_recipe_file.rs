// <FILE>crates/tui-vfx-contract-cli/src/fnc_validate_recipe_file.rs</FILE> - <DESC>Validate one canonical RecipeDocument JSON file</DESC>
// <VERS>VERSION: 0.3.0</VERS>
// <WCTX>New kernel Phase J2: validate recipes against loaded descriptor catalogs.</WCTX>
// <CLOG>0.3.0: MINOR — call catalog-aware validation for descriptor pack refs.
// 0.2.0: MINOR — add code/path/hint diagnostics and report status.
// 0.1.0: INIT — add contract-only recipe file validation.</CLOG>

use std::path::Path;

use crate::{
    cls_validation_error_report::ValidationErrorReport, cls_validation_report::ValidationReport,
};
use tui_vfx_contract::{DescriptorCatalog, RecipeDocument};

/// Validate one canonical v3.1 recipe file through serde and contract checks.
pub fn validate_recipe_file(path: &Path, catalog: &DescriptorCatalog) -> ValidationReport {
    let path_label = path.display().to_string();
    let text = match std::fs::read_to_string(path) {
        Ok(text) => text,
        Err(error) => {
            return failed(
                path_label.clone(),
                "readFailed".to_string(),
                path_label,
                error.to_string(),
                Some("Ensure the path exists and points to a readable recipe JSON file."),
                serde_json::Value::Null,
            );
        }
    };
    let recipe: RecipeDocument = match serde_json::from_str(&text) {
        Ok(recipe) => recipe,
        Err(error) => {
            return failed(
                path_label,
                "deserializeFailed".to_string(),
                "$".to_string(),
                error.to_string(),
                Some("Fix JSON syntax and canonical RecipeDocument field shapes."),
                serde_json::json!({ "line": error.line(), "column": error.column() }),
            );
        }
    };
    match recipe.validate_with_catalog(catalog) {
        Ok(()) => ValidationReport {
            path: path_label,
            status: "ok",
            valid: true,
            errors: vec![],
            warnings: vec![],
        },
        Err(error) => {
            let details = serde_json::to_value(&error).unwrap_or(serde_json::Value::Null);
            failed(
                path_label,
                contract_error_code(&details),
                "$".to_string(),
                format!("{error:?}"),
                Some(
                    "Inspect the structured details and align descriptors, refs, inputs, and lifecycle declarations with tui-vfx-contract.",
                ),
                details,
            )
        }
    }
}

fn failed(
    path: String,
    code: String,
    error_path: String,
    message: String,
    hint: Option<&str>,
    details: serde_json::Value,
) -> ValidationReport {
    ValidationReport {
        path,
        status: "error",
        valid: false,
        errors: vec![ValidationErrorReport {
            code,
            path: error_path,
            message,
            hint: hint.map(str::to_string),
            details,
        }],
        warnings: vec![],
    }
}

fn contract_error_code(details: &serde_json::Value) -> String {
    details
        .get("kind")
        .and_then(serde_json::Value::as_str)
        .unwrap_or("contractValidationFailed")
        .to_string()
}

// <FILE>crates/tui-vfx-contract-cli/src/fnc_validate_recipe_file.rs</FILE> - <DESC>Validate one canonical RecipeDocument JSON file</DESC>
// <VERS>END OF VERSION: 0.3.0</VERS>
