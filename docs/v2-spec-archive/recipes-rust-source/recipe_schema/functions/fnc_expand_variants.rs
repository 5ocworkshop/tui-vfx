// <FILE>src/recipe_schema/functions/fnc_expand_variants.rs</FILE> - <DESC>Expand a template+variants recipe Value into N concrete recipe Values</DESC>
// <VERS>VERSION: 1.0.0</VERS>
// <WCTX>Phase 0 P0.8 parametric variants schema: collapse recipe families into a single file carrying a shared template plus a variants array; the expander deep-merges each variant into the template and returns one concrete Value per entry so downstream parsers see plain recipes</WCTX>
// <CLOG>New recursive expander that detects template+variants recipes, enforces id uniqueness, and produces expanded recipe Values via deep_merge_json; plain recipes (no top-level variants field) pass through as a single-element Vec</CLOG>

//! Variant expansion for template+variants recipe files.
//!
//! A template+variants recipe replaces N near-identical recipe files with one
//! file carrying a shared `template` object and a `variants` array of parameter
//! sets. At load time the expander deep-merges each variant into the template
//! to produce N concrete recipe Values, each of which can then be fed through
//! the existing single-recipe validation and parsing pipeline.
//!
//! # Shape
//!
//! ```json
//! {
//!   "schema_version": 1,
//!   "template": {
//!     "schema_version": 1,
//!     "title": "<overridden per variant>",
//!     "config": { "layout": { "width": 55, "height": 7 } }
//!   },
//!   "variants": [
//!     { "id": "gtd.baseline.easing.back_in",  "title": "Back In",  "config": { ... } },
//!     { "id": "gtd.baseline.easing.back_out", "title": "Back Out", "config": { ... } }
//!   ]
//! }
//! ```
//!
//! # Merge semantics
//!
//! Each variant is deep-merged into the *template* using [`deep_merge_json`],
//! with variant fields winning. Arrays replace wholesale (no element-wise
//! merge). `null` values in a variant delete the corresponding template field.
//!
//! # Validation
//!
//! - `variants` must be a non-empty array.
//! - `template` must exist and be an object.
//! - Each variant must be an object.
//! - Each variant must carry a non-empty `id` string field.
//! - Variant ids must be unique within a file.

use std::collections::HashSet;

use serde_json::Value;

use super::fnc_deep_merge_json::deep_merge_json;

/// Errors that can occur during template+variants expansion.
#[derive(Debug, thiserror::Error)]
pub enum ExpandVariantsError {
    /// Top-level `variants` field exists but is not an array.
    #[error("top-level 'variants' must be an array")]
    VariantsNotArray,

    /// Top-level `variants` array is present but empty.
    #[error("'variants' array must not be empty")]
    EmptyVariants,

    /// Top-level `template` field is missing or not an object.
    #[error("template+variants recipe must have a 'template' object")]
    MissingOrInvalidTemplate,

    /// A variant entry is not a JSON object.
    #[error("variant at index {index} must be a JSON object")]
    VariantNotObject {
        /// Zero-based index of the offending variant entry.
        index: usize,
    },

    /// A variant entry is missing its required `id` field.
    #[error("variant at index {index} is missing required string field 'id'")]
    MissingVariantId {
        /// Zero-based index of the offending variant entry.
        index: usize,
    },

    /// Two or more variant entries share the same `id`.
    #[error("variant id '{id}' is declared more than once in the variants array")]
    DuplicateVariantId {
        /// The duplicated id string.
        id: String,
    },
}

/// Returns `true` if `value` is a template+variants recipe file (top-level
/// `variants` array present). A plain single-recipe file returns `false`.
pub fn is_variants_recipe(value: &Value) -> bool {
    value.get("variants").is_some()
}

/// Expands a template+variants recipe Value into one concrete Value per
/// variant, or returns `vec![value]` unchanged if the input is a plain recipe.
///
/// Each returned Value is a deep-merge of the template with the matching
/// variant's override fields, ready for downstream parsing via
/// [`crate::recipe::from_value`].
pub fn expand_variants(value: Value) -> Result<Vec<Value>, ExpandVariantsError> {
    if !is_variants_recipe(&value) {
        return Ok(vec![value]);
    }

    let variants_array = value
        .get("variants")
        .and_then(Value::as_array)
        .ok_or(ExpandVariantsError::VariantsNotArray)?;

    if variants_array.is_empty() {
        return Err(ExpandVariantsError::EmptyVariants);
    }

    let template = value
        .get("template")
        .filter(|v| v.is_object())
        .cloned()
        .ok_or(ExpandVariantsError::MissingOrInvalidTemplate)?;

    let mut seen_ids: HashSet<String> = HashSet::new();
    let mut expanded: Vec<Value> = Vec::with_capacity(variants_array.len());
    for (index, variant) in variants_array.iter().enumerate() {
        if !variant.is_object() {
            return Err(ExpandVariantsError::VariantNotObject { index });
        }
        let id = variant
            .get("id")
            .and_then(Value::as_str)
            .filter(|s| !s.is_empty())
            .ok_or(ExpandVariantsError::MissingVariantId { index })?
            .to_string();
        if !seen_ids.insert(id.clone()) {
            return Err(ExpandVariantsError::DuplicateVariantId { id });
        }
        let merged = deep_merge_json(template.clone(), variant.clone());
        expanded.push(merged);
    }

    Ok(expanded)
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    fn template_variants_sample() -> Value {
        json!({
            "schema_version": 1,
            "template": {
                "schema_version": 1,
                "description": "shared easing template",
                "version": "2.0.0",
                "config": {
                    "layout": {"width": 55, "height": 7, "anchor": "middle_left"},
                    "lifecycle": {"auto_dismiss_ms": 3000}
                }
            },
            "variants": [
                {
                    "id": "gtd.baseline.easing.back_in",
                    "title": "Back In",
                    "config": {"pipeline": {"enter": {"easing": "back_in"}}}
                },
                {
                    "id": "gtd.baseline.easing.back_out",
                    "title": "Back Out",
                    "config": {
                        "layout": {"anchor": "top_center"},
                        "pipeline": {"enter": {"easing": "back_out"}}
                    }
                }
            ]
        })
    }

    #[test]
    fn plain_recipe_passes_through_unchanged() {
        let plain = json!({
            "schema_version": 1,
            "id": "toasts.whisper",
            "title": "Whisper",
            "description": "borderless",
            "version": "1.0.0",
            "config": {}
        });
        let expanded = expand_variants(plain.clone()).unwrap();
        assert_eq!(expanded, vec![plain]);
    }

    #[test]
    fn template_plus_variants_produces_one_entry_per_variant_with_deep_merge() {
        let value = template_variants_sample();
        let expanded = expand_variants(value).unwrap();
        assert_eq!(expanded.len(), 2);

        // First variant: anchor inherits from template, easing from variant
        assert_eq!(expanded[0]["id"], json!("gtd.baseline.easing.back_in"));
        assert_eq!(expanded[0]["title"], json!("Back In"));
        assert_eq!(expanded[0]["description"], json!("shared easing template"));
        assert_eq!(
            expanded[0]["config"]["layout"]["anchor"],
            json!("middle_left")
        );
        assert_eq!(expanded[0]["config"]["layout"]["width"], json!(55));
        assert_eq!(
            expanded[0]["config"]["pipeline"]["enter"]["easing"],
            json!("back_in")
        );
        assert_eq!(
            expanded[0]["config"]["lifecycle"]["auto_dismiss_ms"],
            json!(3000)
        );

        // Second variant: anchor overridden, width inherited, easing overridden
        assert_eq!(
            expanded[1]["config"]["layout"]["anchor"],
            json!("top_center")
        );
        assert_eq!(expanded[1]["config"]["layout"]["width"], json!(55));
        assert_eq!(
            expanded[1]["config"]["pipeline"]["enter"]["easing"],
            json!("back_out")
        );
    }

    #[test]
    fn empty_variants_array_errors() {
        let value = json!({
            "schema_version": 1,
            "template": {"schema_version": 1, "config": {}},
            "variants": []
        });
        let err = expand_variants(value).unwrap_err();
        assert!(matches!(err, ExpandVariantsError::EmptyVariants));
    }

    #[test]
    fn missing_template_errors() {
        let value = json!({
            "schema_version": 1,
            "variants": [{"id": "a"}]
        });
        let err = expand_variants(value).unwrap_err();
        assert!(matches!(err, ExpandVariantsError::MissingOrInvalidTemplate));
    }

    #[test]
    fn non_object_template_errors() {
        let value = json!({
            "schema_version": 1,
            "template": "not-an-object",
            "variants": [{"id": "a"}]
        });
        let err = expand_variants(value).unwrap_err();
        assert!(matches!(err, ExpandVariantsError::MissingOrInvalidTemplate));
    }

    #[test]
    fn variant_missing_id_errors_with_index() {
        let value = json!({
            "schema_version": 1,
            "template": {"schema_version": 1, "config": {}},
            "variants": [
                {"id": "first"},
                {"title": "nameless"}
            ]
        });
        let err = expand_variants(value).unwrap_err();
        assert!(matches!(
            err,
            ExpandVariantsError::MissingVariantId { index: 1 }
        ));
    }

    #[test]
    fn duplicate_variant_id_errors() {
        let value = json!({
            "schema_version": 1,
            "template": {"schema_version": 1, "config": {}},
            "variants": [
                {"id": "dup"},
                {"id": "dup"}
            ]
        });
        let err = expand_variants(value).unwrap_err();
        match err {
            ExpandVariantsError::DuplicateVariantId { id } => assert_eq!(id, "dup"),
            other => panic!("expected DuplicateVariantId, got {other:?}"),
        }
    }

    #[test]
    fn non_object_variant_errors_with_index() {
        let value = json!({
            "schema_version": 1,
            "template": {"schema_version": 1, "config": {}},
            "variants": [
                {"id": "first"},
                "string-not-object"
            ]
        });
        let err = expand_variants(value).unwrap_err();
        assert!(matches!(
            err,
            ExpandVariantsError::VariantNotObject { index: 1 }
        ));
    }

    #[test]
    fn is_variants_recipe_detects_top_level_variants_array() {
        assert!(is_variants_recipe(&json!({"variants": []})));
        assert!(!is_variants_recipe(&json!({"id": "plain"})));
    }
}

// <FILE>src/recipe_schema/functions/fnc_expand_variants.rs</FILE> - <DESC>Expand a template+variants recipe Value into N concrete recipe Values</DESC>
// <VERS>END OF VERSION: 1.0.0</VERS>
