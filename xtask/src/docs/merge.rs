// <FILE>xtask/src/docs/merge.rs</FILE> - <DESC>Merge rustdoc and TOML data sources</DESC>
// <VERS>VERSION: 1.0.3</VERS>
// <WCTX>Make merged manifest serializable for JSON output</WCTX>
// <CLOG>Fill missing parameter docs from ai_hint text</CLOG>

use super::extract_rustdoc::RustdocData;
use super::parse_toml::CapabilitiesManifest;
use anyhow::Result;
use serde::Serialize;
use std::collections::HashMap;

/// Merged documentation from both sources.
#[derive(Debug, Serialize)]
#[allow(dead_code)] // Some fields reserved for future use
pub struct MergedManifest {
    /// Version from TOML meta
    pub version: String,

    /// Layer taxonomy
    pub layers: HashMap<String, String>,

    /// Phase taxonomy
    pub phases: HashMap<String, String>,

    /// Effects by category
    pub effects: MergedEffects,

    /// Semantic mappings
    pub semantics: super::parse_toml::SemanticsSection,

    /// Recipes
    pub recipes: HashMap<String, super::parse_toml::Recipe>,

    /// Constraints
    pub constraints: super::parse_toml::ConstraintsSection,
}

#[derive(Debug, Default, Serialize)]
pub struct MergedEffects {
    pub masks: HashMap<String, MergedEffect>,
    pub filters: HashMap<String, MergedEffect>,
    pub samplers: HashMap<String, MergedEffect>,
    pub shaders: HashMap<String, MergedEffect>,
    pub styles: HashMap<String, MergedEffect>,
    pub content: HashMap<String, MergedEffect>,
    pub shadows: HashMap<String, MergedEffect>,
}

/// A single effect with data from both sources.
#[derive(Debug, Clone, Serialize)]
#[allow(dead_code)] // Some fields reserved for future use
pub struct MergedEffect {
    /// Effect name
    pub name: String,

    // --- From rustdoc (technical) ---
    /// Summary from rustdoc (first line of doc comment)
    pub rustdoc_summary: Option<String>,

    /// Full description from rustdoc
    pub rustdoc_description: Option<String>,

    /// Parameters from rustdoc
    pub parameters: Vec<MergedParameter>,

    // --- From TOML (editorial) ---
    /// Use cases
    pub use_cases: Vec<String>,

    /// Energy level
    pub energy: String,

    /// Complexity level
    pub complexity: String,

    /// Premium flag
    pub premium: bool,

    /// AI hint
    pub ai_hint: String,

    /// Requirements
    pub requires: Vec<String>,

    /// Pairs with
    pub pairs_with: Vec<String>,
}

#[derive(Debug, Clone, Serialize)]
#[allow(dead_code)] // Reserved for rustdoc extraction
pub struct MergedParameter {
    pub name: String,
    pub ty: String,
    pub doc: String,
    pub default: Option<String>,
}

/// Merge rustdoc data and TOML manifest.
///
/// Priority:
/// - Technical details (types, params, defaults) from rustdoc
/// - Editorial details (use_cases, hints, moods) from TOML
/// - Summary/description from rustdoc if available, else TOML
pub fn merge(rustdoc: RustdocData, toml: CapabilitiesManifest) -> Result<MergedManifest> {
    let mut effects = MergedEffects::default();

    // Merge masks
    for (name, entry) in toml.effects.masks {
        let rustdoc_effect = rustdoc.effects.get("masks").and_then(|m| m.get(&name));

        effects
            .masks
            .insert(name.clone(), merge_effect(&name, rustdoc_effect, &entry));
    }

    // Merge filters
    for (name, entry) in toml.effects.filters {
        let rustdoc_effect = rustdoc.effects.get("filters").and_then(|m| m.get(&name));

        effects
            .filters
            .insert(name.clone(), merge_effect(&name, rustdoc_effect, &entry));
    }

    // Merge samplers
    for (name, entry) in toml.effects.samplers {
        let rustdoc_effect = rustdoc.effects.get("samplers").and_then(|m| m.get(&name));

        effects
            .samplers
            .insert(name.clone(), merge_effect(&name, rustdoc_effect, &entry));
    }

    // Merge shaders
    for (name, entry) in toml.effects.shaders {
        let rustdoc_effect = rustdoc.effects.get("shaders").and_then(|m| m.get(&name));

        effects
            .shaders
            .insert(name.clone(), merge_effect(&name, rustdoc_effect, &entry));
    }

    // Merge styles
    for (name, entry) in toml.effects.styles {
        let rustdoc_effect = rustdoc.effects.get("styles").and_then(|m| m.get(&name));

        effects
            .styles
            .insert(name.clone(), merge_effect(&name, rustdoc_effect, &entry));
    }

    // Merge content transformers
    for (name, entry) in toml.effects.content {
        let rustdoc_effect = rustdoc.effects.get("content").and_then(|m| m.get(&name));

        effects
            .content
            .insert(name.clone(), merge_effect(&name, rustdoc_effect, &entry));
    }

    // Merge shadows
    for (name, entry) in toml.effects.shadows {
        let rustdoc_effect = rustdoc.effects.get("shadows").and_then(|m| m.get(&name));

        effects
            .shadows
            .insert(name.clone(), merge_effect(&name, rustdoc_effect, &entry));
    }

    Ok(MergedManifest {
        version: toml.meta.version,
        layers: toml.taxonomy.layers,
        phases: toml.taxonomy.phases,
        effects,
        semantics: toml.semantics,
        recipes: toml.recipes,
        constraints: toml.constraints,
    })
}

fn merge_effect(
    name: &str,
    rustdoc: Option<&super::extract_rustdoc::EffectDoc>,
    toml: &super::parse_toml::EffectEntry,
) -> MergedEffect {
    let mut parameters = rustdoc
        .map(|r| {
            r.parameters
                .iter()
                .map(|p| MergedParameter {
                    name: p.name.clone(),
                    ty: p.ty.clone(),
                    doc: p.doc.clone(),
                    default: p.default.clone(),
                })
                .collect::<Vec<MergedParameter>>()
        })
        .unwrap_or_default();
    fill_parameter_docs_from_ai_hint(&mut parameters, &toml.ai_hint);

    MergedEffect {
        name: name.to_string(),

        // From rustdoc
        rustdoc_summary: rustdoc.map(|r| r.summary.clone()),
        rustdoc_description: rustdoc.map(|r| r.description.clone()),
        parameters,

        // From TOML
        use_cases: toml.use_cases.clone(),
        energy: toml.energy.clone(),
        complexity: toml.complexity.clone(),
        premium: toml.premium,
        ai_hint: toml.ai_hint.clone(),
        requires: toml.notes.requires.clone(),
        pairs_with: toml.notes.pairs_with.clone(),
    }
}

fn fill_parameter_docs_from_ai_hint(parameters: &mut [MergedParameter], ai_hint: &str) {
    if parameters.is_empty() || ai_hint.trim().is_empty() {
        return;
    }
    let names: Vec<String> = parameters.iter().map(|p| p.name.clone()).collect();
    let ai_docs = extract_ai_hint_param_docs(ai_hint, &names);
    for param in parameters.iter_mut() {
        if param.doc.trim().is_empty() {
            if let Some(doc) = ai_docs.get(&param.name) {
                param.doc = doc.clone();
            }
        }
    }
}

fn extract_ai_hint_param_docs(ai_hint: &str, param_names: &[String]) -> HashMap<String, String> {
    let mut docs = HashMap::new();
    for line in ai_hint.lines() {
        let mut rest = line;
        loop {
            let Some((name, _, colon)) = find_next_param(rest, param_names) else {
                break;
            };
            let after_colon = rest[colon + 1..].trim_start();
            let (doc, remainder) = split_at_next_param(after_colon, param_names);
            let doc = doc.trim().trim_end_matches('.').trim();
            if !doc.is_empty() {
                docs.entry(name.to_string()).or_insert(doc.to_string());
            }
            if remainder.is_empty() || remainder.len() == after_colon.len() {
                break;
            }
            rest = remainder;
        }
    }
    docs
}

fn split_at_next_param<'a>(text: &'a str, param_names: &'a [String]) -> (&'a str, &'a str) {
    if let Some((_, start, _)) = find_next_param(text, param_names) {
        (&text[..start], &text[start..])
    } else {
        (text, "")
    }
}

fn find_next_param<'a>(
    text: &'a str,
    param_names: &'a [String],
) -> Option<(&'a str, usize, usize)> {
    let mut best: Option<(&'a str, usize, usize)> = None;
    for param in param_names {
        if let Some((start, colon)) = find_param_occurrence(text, param) {
            if best.is_none_or(|(_, best_start, _)| start < best_start) {
                best = Some((param.as_str(), start, colon));
            }
        }
    }
    best
}

fn find_param_occurrence(text: &str, param: &str) -> Option<(usize, usize)> {
    if param.is_empty() {
        return None;
    }
    let bytes = text.as_bytes();
    let len = bytes.len();
    let needle_len = param.len();
    let mut search = 0;
    while search + needle_len <= len {
        let pos = text[search..].find(param)?;
        let idx = search + pos;
        let end = idx + needle_len;
        if idx > 0 && is_ident_char(bytes[idx - 1]) {
            search = end;
            continue;
        }
        if end < len && is_ident_char(bytes[end]) {
            search = end;
            continue;
        }
        let mut colon = end;
        while colon < len && bytes[colon].is_ascii_whitespace() {
            colon += 1;
        }
        if colon < len && bytes[colon] == b':' {
            return Some((idx, colon));
        }
        search = end;
    }
    None
}

fn is_ident_char(byte: u8) -> bool {
    byte.is_ascii_alphanumeric() || byte == b'_'
}

#[cfg(test)]
mod tests {
    use super::{MergedParameter, fill_parameter_docs_from_ai_hint};

    #[test]
    fn fills_docs_from_ai_hint_params() {
        let mut params = vec![MergedParameter {
            name: "angle_deg".to_string(),
            ty: "f32".to_string(),
            doc: String::new(),
            default: None,
        }];

        let ai_hint = "shader: the spatial shader to apply. angle_deg: rotation in degrees.";
        fill_parameter_docs_from_ai_hint(&mut params, ai_hint);

        assert_eq!(params[0].doc, "rotation in degrees");
    }
}

// <FILE>xtask/src/docs/merge.rs</FILE> - <DESC>Merge rustdoc and TOML data sources</DESC>
// <VERS>END OF VERSION: 1.0.3</VERS>
