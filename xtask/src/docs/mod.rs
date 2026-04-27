// <FILE>xtask/src/docs/mod.rs</FILE> - <DESC>Documentation generation module</DESC>
// <VERS>VERSION: 1.5.0</VERS>
// <WCTX>Phase α + β: signal-facade — add signals pipeline</WCTX>
// <CLOG>1.5.0: wire signals pipeline (extract_signals_rustdoc, parse_signals_toml, validate_signals, merge_signals, gen_signals_markdown) into generate() and check(); add signals(), signals_check(), signals_validate() entry points</CLOG>

mod api_metadata;
mod effect_metadata;
mod extract_rustdoc;
mod extract_signals_rustdoc;
mod gen_ai_context;
mod gen_api;
mod gen_effect_schemas;
mod gen_json;
mod gen_markdown;
mod gen_signals_markdown;
mod merge;
mod merge_signals;
mod parse_api_toml;
mod parse_signals_toml;
mod parse_toml;
pub mod scaffold;
mod validate_api;
mod validate_coverage;
mod validate_signals;

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

    // API_SIGNALS_REFERENCE.md (separate pipeline: mixed-signals rustdoc + signals.toml)
    println!("  {} Extracting Signal-impl rustdoc from mixed-signals...", "→".dimmed());
    let signal_data = extract_signals_rustdoc::extract()?;
    println!("  {} Parsing signals.toml...", "→".dimmed());
    let signal_toml = parse_signals_toml::parse()?;
    println!("  {} Validating signals coverage...", "→".dimmed());
    validate_signals::validate(&signal_data, &signal_toml)?;
    println!("  {} Merging signal sources...", "→".dimmed());
    let signal_merged = merge_signals::merge(signal_data, signal_toml)?;
    println!("  {} Generating API_SIGNALS_REFERENCE.md...", "→".dimmed());
    gen_signals_markdown::generate(&signal_merged)?;

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

    check_file(
        "docs/generated/CAPABILITIES.md",
        &gen_markdown::render(&merged)?,
        &mut stale,
    );
    check_file(
        "docs/generated/capabilities.json",
        &gen_json::render(&merged)?,
        &mut stale,
    );
    check_file(
        "docs/generated/effect_schemas.json",
        &gen_effect_schemas::render(&merged)?,
        &mut stale,
    );
    check_file(
        "docs/generated/ai-context.md",
        &gen_ai_context::render(&merged)?,
        &mut stale,
    );

    // API.md (separate pipeline)
    let api_data = api_metadata::extract_api_metadata();
    let api_toml = parse_api_toml::parse()?;
    let api_expected = gen_api::generate(&api_data, &api_toml)?;
    check_file("docs/generated/API.md", &api_expected, &mut stale);

    // API_SIGNALS_REFERENCE.md (separate pipeline)
    let signal_data = extract_signals_rustdoc::extract()?;
    let signal_toml = parse_signals_toml::parse()?;
    validate_signals::validate(&signal_data, &signal_toml)?;
    let signal_merged = merge_signals::merge(signal_data, signal_toml)?;
    let signals_expected = gen_signals_markdown::render(&signal_merged)?;
    check_file(
        "docs/generated/API_SIGNALS_REFERENCE.md",
        &signals_expected,
        &mut stale,
    );

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

// ═══════════════════════════════════════════════════════════════════════════════
// SIGNAL REFERENCE DOCUMENTATION
// ═══════════════════════════════════════════════════════════════════════════════

/// Generate API_SIGNALS_REFERENCE.md from mixed-signals rustdoc + signals.toml overlay.
///
/// Runs the parallel signals pipeline:
/// 1. Walk mixed-signals/src to extract Signal-impl rustdoc (Strategy A: walkdir + line parsing)
/// 2. Parse docs/templates/signals.toml editorial overlay (only-overrides, Q2)
/// 3. Validate: editorial entries name real signals; Core 12 entries are in the catalog (Q3)
/// 4. Merge rustdoc + editorial data
/// 5. Generate docs/generated/API_SIGNALS_REFERENCE.md (Core 12 cheatsheet + full catalog by family)
pub fn signals() -> Result<()> {
    println!("{}", "Generating API_SIGNALS_REFERENCE.md...".bold());

    println!("  {} Extracting Signal-impl rustdoc from mixed-signals...", "→".dimmed());
    let signal_data = extract_signals_rustdoc::extract()?;

    println!("  {} Parsing signals.toml...", "→".dimmed());
    let toml_data = parse_signals_toml::parse()?;

    println!("  {} Validating signals coverage...", "→".dimmed());
    validate_signals::validate(&signal_data, &toml_data)?;

    println!("  {} Merging sources...", "→".dimmed());
    let merged = merge_signals::merge(signal_data, toml_data)?;

    println!("  {} Generating API_SIGNALS_REFERENCE.md...", "→".dimmed());
    gen_signals_markdown::generate(&merged)?;

    println!("{}", "✓ API_SIGNALS_REFERENCE.md generated successfully".green().bold());
    Ok(())
}

/// Check that API_SIGNALS_REFERENCE.md is up-to-date.
///
/// Runs the same pipeline as `signals()` but compares output to the existing
/// file instead of writing. Returns an error if the file would change.
pub fn signals_check() -> Result<()> {
    println!("{}", "Checking API_SIGNALS_REFERENCE.md freshness...".bold());

    let signal_data = extract_signals_rustdoc::extract()?;
    let toml_data = parse_signals_toml::parse()?;
    validate_signals::validate(&signal_data, &toml_data)?;
    let merged = merge_signals::merge(signal_data, toml_data)?;
    let expected = gen_signals_markdown::render(&merged)?;

    let current = fs::read_to_string("docs/generated/API_SIGNALS_REFERENCE.md")
        .unwrap_or_default();
    if expected == current {
        println!("{}", "✓ API_SIGNALS_REFERENCE.md is up-to-date".green().bold());
        Ok(())
    } else {
        bail!(
            "docs/generated/API_SIGNALS_REFERENCE.md is out of date. \
             Run `cargo xtask docs signals` to regenerate. \
             ({} bytes expected vs {} bytes actual)",
            expected.len(),
            current.len()
        );
    }
}

/// Validate signals.toml: every named signal exists in mixed-signals,
/// and every Core 12 entry exists in the autogen catalog.
pub fn signals_validate() -> Result<()> {
    println!("{}", "Validating signals.toml coverage...".bold());

    let signal_data = extract_signals_rustdoc::extract()?;
    let toml_data = parse_signals_toml::parse()?;
    validate_signals::validate(&signal_data, &toml_data)?;

    println!("{}", "✓ signals.toml is valid".green().bold());
    Ok(())
}

// <FILE>xtask/src/docs/mod.rs</FILE> - <DESC>Documentation generation module</DESC>
// <VERS>END OF VERSION: 1.5.0</VERS>
