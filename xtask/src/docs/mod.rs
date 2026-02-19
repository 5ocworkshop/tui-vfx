// <FILE>xtask/src/docs/mod.rs</FILE> - <DESC>Documentation generation module</DESC>
// <VERS>VERSION: 1.4.0</VERS>
// <WCTX>Emit capabilities.json from merged manifest</WCTX>
// <CLOG>Add effect_schemas.json export</CLOG>

mod api_metadata;
mod effect_metadata;
mod extract_rustdoc;
mod gen_ai_context;
mod gen_api;
mod gen_effect_schemas;
mod gen_json;
mod gen_markdown;
mod merge;
mod parse_api_toml;
mod parse_toml;
pub mod scaffold;
mod validate_api;
mod validate_coverage;

use anyhow::{Result, bail};
use owo_colors::OwoColorize;
use std::fs;

/// Generate all documentation from code + TOML sources.
///
/// This is the main entry point that generates everything under docs/generated/:
/// - CAPABILITIES.md, capabilities.json, effect_schemas.json, ai-context.md
///   (from capabilities pipeline: rustdoc extraction + capabilities.toml)
/// - API.md
///   (from API pipeline: code metadata + api_docs.toml)
pub fn generate() -> Result<()> {
    println!("{}", "Generating documentation...".bold());

    // Step 1: Extract rustdoc JSON
    println!("  {} Extracting rustdoc JSON...", "→".dimmed());
    let rustdoc_data = extract_rustdoc::extract()?;

    // Step 2: Parse capabilities.toml
    println!("  {} Parsing capabilities.toml...", "→".dimmed());
    let toml_data = parse_toml::parse()?;

    // Step 3: Validate coverage
    println!("  {} Validating coverage...", "→".dimmed());
    validate_coverage::validate(&rustdoc_data, &toml_data)?;

    // Step 4: Merge sources
    println!("  {} Merging sources...", "→".dimmed());
    let merged = merge::merge(rustdoc_data, toml_data)?;

    // Step 5: Generate outputs
    println!("  {} Generating CAPABILITIES.md...", "→".dimmed());
    gen_markdown::generate(&merged)?;

    println!("  {} Generating capabilities.json...", "→".dimmed());
    gen_json::generate(&merged)?;

    println!("  {} Generating effect_schemas.json...", "→".dimmed());
    gen_effect_schemas::generate(&merged)?;

    println!("  {} Generating ai-context.md...", "→".dimmed());
    gen_ai_context::generate(&merged)?;

    // API.md (separate pipeline: code metadata + api_docs.toml)
    println!("  {} Extracting API metadata...", "→".dimmed());
    let api_data = api_metadata::extract_api_metadata();
    println!("  {} Parsing api_docs.toml...", "→".dimmed());
    let api_toml = parse_api_toml::parse()?;
    println!("  {} Generating API.md...", "→".dimmed());
    gen_api::generate_and_write(&api_data, &api_toml)?;

    println!(
        "{}",
        "✓ Documentation generated successfully".green().bold()
    );
    Ok(())
}

/// Check that generated docs are up-to-date.
///
/// Runs the same pipeline as `generate()` but compares output to existing
/// files instead of writing. Returns error if any files would change.
pub fn check() -> Result<()> {
    println!("{}", "Checking documentation freshness...".bold());

    let rustdoc_data = extract_rustdoc::extract()?;
    let toml_data = parse_toml::parse()?;
    validate_coverage::validate(&rustdoc_data, &toml_data)?;
    let merged = merge::merge(rustdoc_data, toml_data)?;

    let mut stale = Vec::new();

    fn check_file(path: &str, expected: &str, stale: &mut Vec<String>) {
        let current = fs::read_to_string(path).unwrap_or_default();
        if expected != current {
            stale.push(format!(
                "  {} ({} bytes expected vs {} bytes actual)",
                path,
                expected.len(),
                current.len()
            ));
        }
    }

    check_file("docs/generated/CAPABILITIES.md", &gen_markdown::render(&merged)?, &mut stale);
    check_file("docs/generated/capabilities.json", &gen_json::render(&merged)?, &mut stale);
    check_file("docs/generated/effect_schemas.json", &gen_effect_schemas::render(&merged)?, &mut stale);
    check_file("docs/generated/ai-context.md", &gen_ai_context::render(&merged)?, &mut stale);

    // API.md (separate pipeline)
    let api_data = api_metadata::extract_api_metadata();
    let api_toml = parse_api_toml::parse()?;
    let api_expected = gen_api::generate(&api_data, &api_toml)?;
    check_file("docs/generated/API.md", &api_expected, &mut stale);

    if stale.is_empty() {
        println!("{}", "✓ All generated docs are up-to-date".green().bold());
        Ok(())
    } else {
        bail!(
            "Generated docs are out of date. Run `cargo xtask docs generate` to regenerate.\n{}",
            stale.join("\n")
        );
    }
}

/// Generate only the AI context prompt.
pub fn ai_context() -> Result<()> {
    println!("{}", "Generating AI context prompt...".bold());

    let rustdoc_data = extract_rustdoc::extract()?;
    let toml_data = parse_toml::parse()?;
    validate_coverage::validate(&rustdoc_data, &toml_data)?;
    let merged = merge::merge(rustdoc_data, toml_data)?;
    gen_ai_context::generate(&merged)?;

    println!("{}", "✓ AI context prompt generated".green().bold());
    Ok(())
}

/// Generate only CAPABILITIES.md.
pub fn markdown() -> Result<()> {
    println!("{}", "Generating CAPABILITIES.md...".bold());

    let rustdoc_data = extract_rustdoc::extract()?;
    let toml_data = parse_toml::parse()?;
    validate_coverage::validate(&rustdoc_data, &toml_data)?;
    let merged = merge::merge(rustdoc_data, toml_data)?;
    gen_markdown::generate(&merged)?;

    println!("{}", "✓ CAPABILITIES.md generated".green().bold());
    Ok(())
}

/// Validate capabilities.toml covers all code variants.
pub fn validate() -> Result<()> {
    println!("{}", "Validating capabilities.toml coverage...".bold());

    let rustdoc_data = extract_rustdoc::extract()?;
    let toml_data = parse_toml::parse()?;
    validate_coverage::validate(&rustdoc_data, &toml_data)?;

    println!("{}", "✓ All variants documented".green().bold());
    Ok(())
}

/// Scaffold TOML stubs for effects not yet documented.
pub fn scaffold_toml(write: bool) -> Result<()> {
    scaffold::scaffold(write)
}

// ═══════════════════════════════════════════════════════════════════════════════
// API DOCUMENTATION
// ═══════════════════════════════════════════════════════════════════════════════

/// Generate API.md from code metadata + api_docs.toml.
pub fn api() -> Result<()> {
    println!("{}", "Generating API.md...".bold());

    println!("  {} Extracting API metadata...", "→".dimmed());
    let api_data = api_metadata::extract_api_metadata();

    println!("  {} Parsing api_docs.toml...", "→".dimmed());
    let toml_data = parse_api_toml::parse()?;

    println!("  {} Generating API.md...", "→".dimmed());
    gen_api::generate_and_write(&api_data, &toml_data)?;

    println!("{}", "✓ API.md generated successfully".green().bold());
    Ok(())
}

/// Check that API.md is up-to-date.
pub fn api_check() -> Result<()> {
    println!("{}", "Checking API.md freshness...".bold());

    let api_data = api_metadata::extract_api_metadata();
    let toml_data = parse_api_toml::parse()?;
    validate_api::check_freshness(&api_data, &toml_data)?;

    Ok(())
}

/// Validate api_docs.toml covers all code entities.
pub fn api_validate() -> Result<()> {
    println!("{}", "Validating api_docs.toml coverage...".bold());

    let api_data = api_metadata::extract_api_metadata();
    let toml_data = parse_api_toml::parse()?;
    validate_api::validate_coverage(&api_data, &toml_data)?;

    Ok(())
}

/// Scaffold TOML stubs for undocumented API items.
pub fn api_scaffold(write: bool) -> Result<()> {
    println!("{}", "Scaffolding api_docs.toml stubs...".bold());

    let api_data = api_metadata::extract_api_metadata();
    let toml_data = parse_api_toml::parse()?;
    let stubs = validate_api::scaffold_missing(&api_data, &toml_data);

    if write {
        // Append to api_docs.toml
        use std::fs::OpenOptions;
        use std::io::Write;
        let mut file = OpenOptions::new()
            .append(true)
            .open("docs/templates/api_docs.toml")?;
        writeln!(file, "\n{}", stubs)?;
        println!(
            "{}",
            "✓ Stubs written to docs/templates/api_docs.toml"
                .green()
                .bold()
        );
    } else {
        println!("{}", stubs);
    }

    Ok(())
}

// <FILE>xtask/src/docs/mod.rs</FILE> - <DESC>Documentation generation module</DESC>
// <VERS>END OF VERSION: 1.4.0</VERS>
