// <FILE>xtask/src/docs/validate_api.rs</FILE> - <DESC>Validate API documentation coverage and freshness</DESC>
// <VERS>VERSION: 1.1.1</VERS>
// <WCTX>Rustfmt normalization for API validation</WCTX>
// <CLOG>Apply formatting updates after clippy run</CLOG>

use super::api_metadata::ApiMetadata;
use super::gen_api;
use super::parse_api_toml::ApiDocsManifest;
use anyhow::{Context, Result, bail};
use std::fs;

const API_PATH: &str = "docs/generated/API.md";

/// Check that API.md is up-to-date with current code and TOML.
pub fn check_freshness(api: &ApiMetadata, toml: &ApiDocsManifest) -> Result<()> {
    // Generate what the output should be
    let expected = gen_api::generate(api, toml)?;

    // Read current file
    let current =
        fs::read_to_string(API_PATH).with_context(|| format!("Failed to read {}", API_PATH))?;

    // Compare
    if expected != current {
        bail!(
            "API.md is out of date. Run `cargo xtask docs api` to regenerate.\n\
             Diff: {} bytes expected vs {} bytes actual",
            expected.len(),
            current.len()
        );
    }

    println!("✓ API.md is up-to-date");
    Ok(())
}

/// Validate that api_docs.toml covers all code entities.
pub fn validate_coverage(api: &ApiMetadata, toml: &ApiDocsManifest) -> Result<()> {
    let mut issues = Vec::new();

    // Check entry points are documented
    for func in &api.entry_points {
        if !toml.entry_points.functions.contains_key(&func.name) {
            issues.push(format!(
                "Entry point `{}` not documented in api_docs.toml [entry_points]",
                func.name
            ));
        }
    }

    // Check core types are documented
    for type_doc in &api.core_types {
        if !toml.types.contains_key(&type_doc.name) {
            issues.push(format!(
                "Core type `{}` not documented in api_docs.toml [types]",
                type_doc.name
            ));
        }
    }

    // Check effect specs are documented
    let spec_checks = [
        ("MaskSpec", &api.effects.masks),
        ("FilterSpec", &api.effects.filters),
        ("SamplerSpec", &api.effects.samplers),
    ];

    for (spec_name, _effects) in spec_checks {
        if !toml.specs.contains_key(spec_name) {
            issues.push(format!(
                "Spec `{}` not documented in api_docs.toml [specs]",
                spec_name
            ));
        }

        // Note: We don't check individual effect variants here since they come
        // from code introspection. The TOML provides editorial content, not
        // the authoritative list of variants.
    }

    // Check shader section exists
    if !toml.specs.contains_key("SpatialShaderType") {
        issues.push("SpatialShaderType not documented in api_docs.toml [specs]".to_string());
    }

    // Check shadow section exists
    for shadow_type in ["ShadowConfig", "ShadowStyle", "ShadowEdges"] {
        if !toml.specs.contains_key(shadow_type) {
            issues.push(format!(
                "Shadow type `{}` not documented in api_docs.toml [specs]",
                shadow_type
            ));
        }
    }

    // Check content section exists
    if !toml.specs.contains_key("ContentEffect") {
        issues.push("ContentEffect not documented in api_docs.toml [specs]".to_string());
    }

    // Check prelude is documented
    if toml.prelude.usage.is_empty() {
        issues.push("Prelude usage not documented in api_docs.toml [prelude]".to_string());
    }

    // Report results
    if issues.is_empty() {
        println!("✓ api_docs.toml coverage validated");
        Ok(())
    } else {
        eprintln!("API documentation coverage issues found:\n");
        for issue in &issues {
            eprintln!("  - {}", issue);
        }
        bail!("{} coverage issues found", issues.len());
    }
}

/// Generate TOML stubs for undocumented items.
pub fn scaffold_missing(api: &ApiMetadata, toml: &ApiDocsManifest) -> String {
    let mut output = String::new();
    output.push_str("# Scaffolded stubs for undocumented API items\n");
    output.push_str("# Copy relevant sections to api_docs.toml\n\n");

    // Check entry points
    let mut missing_entries = Vec::new();
    for func in &api.entry_points {
        if !toml.entry_points.functions.contains_key(&func.name) {
            missing_entries.push(&func.name);
        }
    }

    if !missing_entries.is_empty() {
        output.push_str("# Missing entry points:\n");
        for name in missing_entries {
            output.push_str(&format!(
                r#"[entry_points.{}]
order = 0
description = ""
notes = ""

"#,
                name
            ));
        }
    }

    // Check core types
    let mut missing_types = Vec::new();
    for type_doc in &api.core_types {
        if !toml.types.contains_key(&type_doc.name) {
            missing_types.push(&type_doc.name);
        }
    }

    if !missing_types.is_empty() {
        output.push_str("# Missing core types:\n");
        for name in missing_types {
            output.push_str(&format!(
                r#"[types.{}]
part = ""
subtitle = ""
notes = ""

"#,
                name
            ));
        }
    }

    // Check specs
    let missing_specs: Vec<&str> = [
        "MaskSpec",
        "FilterSpec",
        "SamplerSpec",
        "SpatialShaderType",
        "ContentEffect",
    ]
    .into_iter()
    .filter(|s| !toml.specs.contains_key(*s))
    .collect();

    if !missing_specs.is_empty() {
        output.push_str("# Missing specs:\n");
        for name in missing_specs {
            output.push_str(&format!(
                r#"[specs.{}]
part = ""
section_title = "{}"
description = ""

"#,
                name, name
            ));
        }
    }

    if output.lines().count() <= 3 {
        "# All API items are documented!\n".to_string()
    } else {
        output
    }
}

// <FILE>xtask/src/docs/validate_api.rs</FILE> - <DESC>Validate API documentation coverage and freshness</DESC>
// <VERS>END OF VERSION: 1.1.1</VERS>
