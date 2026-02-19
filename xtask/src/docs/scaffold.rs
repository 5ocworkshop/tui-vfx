// <FILE>xtask/src/docs/scaffold.rs</FILE> - <DESC>Generate TOML stubs for undocumented effects</DESC>
// <VERS>VERSION: 1.1.1</VERS>
// <WCTX>Address clippy useless_format warnings</WCTX>
// <CLOG>Replace format! wrappers with direct string literals</CLOG>

use super::effect_metadata::{EffectMetadata, extract_all_metadata};
use super::parse_toml::{self, EffectEntry};
use anyhow::Result;
use owo_colors::OwoColorize;
use std::collections::HashMap;
use std::fs;
use std::io::Write;

/// Scaffold TOML stubs for effects not yet documented.
///
/// If `write` is true, appends stubs directly to capabilities.toml.
/// Otherwise, outputs to stdout.
pub fn scaffold(write: bool) -> Result<()> {
    println!("{}", "Scaffolding TOML stubs...".bold());

    // Load existing TOML
    let toml_data = parse_toml::parse()?;

    // Extract metadata from code
    let metadata = extract_all_metadata();

    // Find missing entries
    let mut stubs = Vec::new();

    stubs.extend(find_missing_entries(
        &metadata.masks,
        &toml_data.effects.masks,
        "masks",
    ));
    stubs.extend(find_missing_entries(
        &metadata.filters,
        &toml_data.effects.filters,
        "filters",
    ));
    stubs.extend(find_missing_entries(
        &metadata.samplers,
        &toml_data.effects.samplers,
        "samplers",
    ));
    stubs.extend(find_missing_entries(
        &metadata.shaders,
        &toml_data.effects.shaders,
        "shaders",
    ));
    stubs.extend(find_missing_entries(
        &metadata.styles,
        &toml_data.effects.styles,
        "styles",
    ));
    stubs.extend(find_missing_entries(
        &metadata.content,
        &toml_data.effects.content,
        "content",
    ));
    stubs.extend(find_missing_entries(
        &metadata.shadows,
        &toml_data.effects.shadows,
        "shadows",
    ));

    if stubs.is_empty() {
        println!("{}", "  ✓ All effects are documented".green());
        return Ok(());
    }

    // Generate TOML output
    let toml_output = generate_toml_stubs(&stubs);

    if write {
        // Append to capabilities.toml
        let mut file = fs::OpenOptions::new()
            .append(true)
            .open("docs/templates/capabilities.toml")?;

        writeln!(file)?;
        writeln!(
            file,
            "# ════════════════════════════════════════════════════════════════════════════════"
        )?;
        writeln!(
            file,
            "# Generated stubs - fill in use_cases, energy, and ai_hint"
        )?;
        writeln!(
            file,
            "# ════════════════════════════════════════════════════════════════════════════════"
        )?;
        writeln!(file)?;
        write!(file, "{}", toml_output)?;

        println!(
            "{}",
            format!(
                "  ✓ Added {} stub(s) to docs/templates/capabilities.toml",
                stubs.len()
            )
            .green()
            .bold()
        );
    } else {
        // Output to stdout
        println!("{}", "Generated stubs:".bold());
        println!();
        println!("{}", toml_output);
        println!(
            "{}",
            format!(
                "Run 'cargo xtask docs scaffold --write' to append {} stub(s) to docs/templates/capabilities.toml",
                stubs.len()
            )
            .dimmed()
        );
    }

    Ok(())
}

#[derive(Debug)]
struct StubEntry {
    category: String,
    name: String,
    description: String,
    parameters: Vec<(String, String)>,
}

fn find_missing_entries(
    metadata: &HashMap<String, EffectMetadata>,
    existing: &HashMap<String, EffectEntry>,
    category: &str,
) -> Vec<StubEntry> {
    metadata
        .iter()
        .filter(|(name, _)| {
            // Skip "None" variants
            *name != "None" && !existing.contains_key(*name)
        })
        .map(|(name, meta)| StubEntry {
            category: category.to_string(),
            name: name.clone(),
            description: meta.description.clone(),
            parameters: meta.parameters.clone(),
        })
        .collect()
}

fn generate_toml_stubs(stubs: &[StubEntry]) -> String {
    let mut output = String::new();

    // Group by category
    let mut by_category: HashMap<&str, Vec<&StubEntry>> = HashMap::new();
    for stub in stubs {
        by_category.entry(&stub.category).or_default().push(stub);
    }

    for (category, entries) in by_category {
        for entry in entries {
            output.push_str(&format!("[effects.{}.{}]\n", category, entry.name));
            output.push_str(&format!(
                "summary = \"{}\"\n",
                escape_toml_string(&entry.description)
            ));
            output.push_str(
                "use_cases = [] # TODO: Add use cases like [\"transitions\", \"loading\"]\n",
            );
            output
                .push_str("energy = \"\" # TODO: Set to \"calm\", \"moderate\", or \"intense\"\n");
            output.push_str("complexity = \"simple\" # simple, moderate, or advanced\n");
            output.push_str("premium = false\n");

            // Generate ai_hint with parameter info
            let param_info = if entry.parameters.is_empty() {
                String::new()
            } else {
                let params: Vec<String> = entry
                    .parameters
                    .iter()
                    .map(|(name, _)| name.clone())
                    .collect();
                format!(" Parameters: {}.", params.join(", "))
            };

            output.push_str(&format!(
                "ai_hint = \"TODO: Document {}.{}\"\n",
                param_info, entry.name
            ));

            output.push('\n');
        }
    }

    output
}

fn escape_toml_string(s: &str) -> String {
    s.replace('\\', "\\\\")
        .replace('"', "\\\"")
        .replace('\n', "\\n")
}

// <FILE>xtask/src/docs/scaffold.rs</FILE> - <DESC>Generate TOML stubs for undocumented effects</DESC>
// <VERS>END OF VERSION: 1.1.1</VERS>
