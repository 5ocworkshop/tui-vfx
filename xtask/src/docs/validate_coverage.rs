// <FILE>xtask/src/docs/validate_coverage.rs</FILE> - <DESC>Validate TOML covers all code variants</DESC>
// <VERS>VERSION: 2.1.1</VERS>
// <WCTX>Rustfmt normalization for coverage validation</WCTX>
// <CLOG>Apply formatting updates after clippy run</CLOG>

use super::extract_rustdoc::{EffectDoc, RustdocData};
use super::parse_toml::{CapabilitiesManifest, EffectEntry};
use anyhow::{Result, bail};
use owo_colors::OwoColorize;
use std::collections::HashMap;

/// Validate that capabilities.toml covers all variants found in code.
///
/// Checks:
/// 1. Every enum variant in metadata has a TOML entry (missing detection)
/// 2. Every TOML entry references a real code variant (orphan detection)
/// 3. Required fields are present (use_cases, energy, ai_hint)
/// 4. ai_hint doesn't contain stub markers (TODO, STUB, or < 20 chars)
/// 5. Parameters from metadata are mentioned in ai_hint (coverage)
pub fn validate(rustdoc: &RustdocData, toml: &CapabilitiesManifest) -> Result<()> {
    let mut errors: Vec<String> = Vec::new();
    let mut warnings = 0usize;

    // Validate each category
    warnings += validate_category(
        &rustdoc.effects.get("masks"),
        &toml.effects.masks,
        "masks",
        &mut errors,
    );
    warnings += validate_category(
        &rustdoc.effects.get("filters"),
        &toml.effects.filters,
        "filters",
        &mut errors,
    );
    warnings += validate_category(
        &rustdoc.effects.get("samplers"),
        &toml.effects.samplers,
        "samplers",
        &mut errors,
    );
    warnings += validate_category(
        &rustdoc.effects.get("shaders"),
        &toml.effects.shaders,
        "shaders",
        &mut errors,
    );
    warnings += validate_category(
        &rustdoc.effects.get("styles"),
        &toml.effects.styles,
        "styles",
        &mut errors,
    );
    warnings += validate_category(
        &rustdoc.effects.get("content"),
        &toml.effects.content,
        "content",
        &mut errors,
    );
    warnings += validate_category(
        &rustdoc.effects.get("shadows"),
        &toml.effects.shadows,
        "shadows",
        &mut errors,
    );

    if !errors.is_empty() {
        eprintln!("{}", "Validation errors:".red().bold());
        for error in &errors {
            eprintln!("  {} {}", "✗".red(), error);
        }
        bail!("{} validation error(s) found", errors.len());
    }

    if warnings > 0 {
        eprintln!(
            "\n  {} {} warning(s) - run 'cargo xtask docs scaffold' to generate stubs",
            "!".yellow().bold(),
            warnings
        );
    }

    Ok(())
}

fn validate_category(
    metadata: &Option<&HashMap<String, EffectDoc>>,
    toml_entries: &HashMap<String, EffectEntry>,
    category: &str,
    _errors: &mut Vec<String>,
) -> usize {
    let mut warnings = 0usize;

    if let Some(meta) = metadata {
        // Check for missing TOML entries (effect in metadata but not in TOML)
        for (name, _doc) in meta.iter() {
            // Skip "None" variants as they don't need documentation
            if name == "None" {
                continue;
            }

            if !toml_entries.contains_key(name) {
                eprintln!(
                    "  {} {}.{}: missing TOML entry",
                    "⚠".yellow(),
                    category,
                    name
                );
                warnings += 1;
            }
        }

        // Check for orphan TOML entries (entry in TOML but not in metadata)
        for name in toml_entries.keys() {
            if !meta.contains_key(name) {
                eprintln!(
                    "  {} {}.{}: orphan TOML entry (no matching code variant)",
                    "⚠".yellow(),
                    category,
                    name
                );
                warnings += 1;
            }
        }

        // Validate field completeness, quality, and parameter coverage
        for (name, entry) in toml_entries {
            let effect_meta = meta.get(name);
            warnings += validate_effect_entry(entry, effect_meta, category, name);
        }
    } else {
        // No metadata available, just validate TOML fields
        for (name, entry) in toml_entries {
            warnings += validate_effect_entry(entry, None, category, name);
        }
    }

    warnings
}

fn validate_effect_entry(
    entry: &EffectEntry,
    metadata: Option<&EffectDoc>,
    category: &str,
    name: &str,
) -> usize {
    let mut warnings = 0usize;

    // Warn if key fields are empty (but don't fail - allows incremental migration)
    if entry.use_cases.is_empty() {
        eprintln!(
            "  {} {}.{}: missing use_cases",
            "⚠".yellow(),
            category,
            name
        );
        warnings += 1;
    }

    if entry.energy.is_empty() {
        eprintln!("  {} {}.{}: missing energy", "⚠".yellow(), category, name);
        warnings += 1;
    }

    if entry.ai_hint.is_empty() {
        eprintln!("  {} {}.{}: missing ai_hint", "⚠".yellow(), category, name);
        warnings += 1;
    } else {
        // Check for stub markers in ai_hint
        let ai_hint_lower = entry.ai_hint.to_lowercase();
        if ai_hint_lower.contains("todo") || ai_hint_lower.contains("stub") {
            eprintln!(
                "  {} {}.{}: ai_hint contains TODO/STUB marker",
                "⚠".yellow(),
                category,
                name
            );
            warnings += 1;
        } else if entry.ai_hint.len() < 20 {
            eprintln!(
                "  {} {}.{}: ai_hint too short ({} chars, recommend 20+)",
                "⚠".yellow(),
                category,
                name,
                entry.ai_hint.len()
            );
            warnings += 1;
        } else {
            // Check parameter coverage - warn if params from metadata aren't in ai_hint
            warnings += validate_parameter_coverage(entry, metadata, category, name);
        }
    }

    warnings
}

/// Check that parameters from metadata are mentioned in the ai_hint.
///
/// Warns about undocumented parameters to ensure ai_hints are comprehensive.
fn validate_parameter_coverage(
    entry: &EffectEntry,
    metadata: Option<&EffectDoc>,
    category: &str,
    name: &str,
) -> usize {
    let mut warnings = 0usize;

    if let Some(meta) = metadata {
        if meta.parameters.is_empty() {
            return 0;
        }

        let ai_hint_lower = entry.ai_hint.to_lowercase();
        let mut missing_params = Vec::new();

        for param in &meta.parameters {
            // Convert param name to lowercase for case-insensitive matching
            // Also check for snake_case to word variations (e.g., "soft_edge" matches "soft edge")
            let param_lower = param.name.to_lowercase();
            let param_spaced = param_lower.replace('_', " ");

            if !ai_hint_lower.contains(&param_lower) && !ai_hint_lower.contains(&param_spaced) {
                missing_params.push(param.name.as_str());
            }
        }

        if !missing_params.is_empty() {
            eprintln!(
                "  {} {}.{}: ai_hint missing params: {}",
                "⚠".yellow(),
                category,
                name,
                missing_params.join(", ")
            );
            warnings += 1;
        }
    }

    warnings
}

// <FILE>xtask/src/docs/validate_coverage.rs</FILE> - <DESC>Validate TOML covers all code variants</DESC>
// <VERS>END OF VERSION: 2.1.1</VERS>
