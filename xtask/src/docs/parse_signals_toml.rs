// <FILE>xtask/src/docs/parse_signals_toml.rs</FILE> - <DESC>Parse docs/templates/signals.toml editorial overlay (engine / direct-API audience)</DESC>
// <VERS>VERSION: 0.2.1</VERS>
// <WCTX>Packet 67 — engine/API doc relabel: drop the is_parallel_channel field initializer in the unknown_signal_in_toml_fails_validation test now that SignalDoc no longer carries that field.</WCTX>
// <CLOG>0.2.1: drop is_parallel_channel from the test-fixture SignalDoc so the test compiles after the SignalDoc field removal in extract_signals_rustdoc.rs v0.3.0</CLOG>

use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;

/// Root structure of docs/templates/signals.toml.
///
/// Only-overrides per Q2 decision: entries appear only when editorial enrichment
/// is needed beyond what rustdoc provides. The Core 12 list is data-driven here
/// per Q3 so editors can update the cheatsheet without touching xtask code.
#[derive(Debug, Default, Deserialize, Serialize)]
pub struct SignalsManifest {
    /// Metadata about the manifest itself.
    pub meta: MetaSection,
    /// The Core 12 curated list (ordered).
    #[serde(default)]
    pub core_12: Core12Section,
    /// Editorial overrides keyed by SignalSpec discriminant (snake_case).
    #[serde(default)]
    pub signals: HashMap<String, SignalEntry>,
}

/// Manifest metadata.
#[derive(Debug, Default, Deserialize, Serialize)]
pub struct MetaSection {
    /// Overlay version string (semver).
    pub version: String,
    /// Human-readable description.
    #[serde(default)]
    pub description: String,
}

/// The Core 12 curated list of highest-leverage signals for recipe authoring.
#[derive(Debug, Default, Deserialize, Serialize)]
pub struct Core12Section {
    /// Ordered list of SignalSpec discriminants (snake_case) for the Core 12 cheatsheet.
    #[serde(default)]
    pub order: Vec<String>,
}

/// Editorial entry for a single signal (only-overrides).
///
/// All fields are optional; the autogen uses rustdoc for everything else.
#[derive(Debug, Default, Clone, Deserialize, Serialize)]
pub struct SignalEntry {
    /// Concrete use cases for recipe authors (short tags, e.g. "pulsing").
    #[serde(default)]
    pub use_cases: Vec<String>,
    /// Extended recipe-author hint that goes beyond the rustdoc summary.
    #[serde(default)]
    pub recipe_hint: Option<String>,
}

/// Parse `docs/templates/signals.toml`.
///
/// The path is resolved relative to the tui-vfx workspace root via
/// `CARGO_MANIFEST_DIR/..`, so this works under both `cargo xtask` (CWD =
/// workspace root) and `cargo test -p xtask` (CWD = `xtask/`).
pub fn parse() -> Result<SignalsManifest> {
    let path: PathBuf = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .expect("CARGO_MANIFEST_DIR (xtask/) always has a parent (workspace root)")
        .join("docs/templates/signals.toml");
    let contents = fs::read_to_string(&path)
        .with_context(|| format!("Failed to read {}", path.display()))?;
    toml::from_str(&contents).with_context(|| "Failed to parse docs/templates/signals.toml")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn signals_toml_parses_successfully() {
        let manifest = parse().expect("signals.toml should parse without error");
        assert!(!manifest.meta.version.is_empty(), "meta.version should be set");
        assert_eq!(
            manifest.core_12.order.len(),
            12,
            "Core 12 should have exactly 12 entries"
        );
    }

    #[test]
    fn core_12_contains_expected_entries() {
        let manifest = parse().expect("signals.toml should parse");
        let order = &manifest.core_12.order;
        for expected in &["sine", "triangle", "ramp", "adsr", "clamp"] {
            assert!(
                order.contains(&expected.to_string()),
                "Core 12 should contain `{expected}`"
            );
        }
    }

    #[test]
    fn editorial_entries_have_recipe_hints() {
        let manifest = parse().expect("signals.toml should parse");
        let sine = manifest.signals.get("sine").expect("sine entry should exist");
        assert!(
            sine.recipe_hint.is_some(),
            "sine editorial entry should have a recipe_hint"
        );
        assert!(
            !sine.use_cases.is_empty(),
            "sine editorial entry should have use_cases"
        );
    }

    #[test]
    fn unknown_signal_in_toml_fails_validation() {
        use super::super::validate_signals;
        use super::super::extract_signals_rustdoc::{SignalDoc, SignalFamily, SignalsRustdocData};

        let mut data = SignalsRustdocData::default();
        data.by_discriminant.insert(
            "sine".into(),
            SignalDoc {
                discriminant: "sine".into(),
                struct_name: "Sine".into(),
                family: SignalFamily::Oscillator,
                summary: "Sine wave oscillator.".into(),
                description: "Sine wave oscillator.".into(),
                fields: Vec::new(),
            },
        );

        let mut toml = SignalsManifest::default();
        toml.signals.insert("not_a_real_signal".into(), SignalEntry::default());

        let err = validate_signals::validate(&data, &toml)
            .expect_err("validation should fail for unknown signal");
        let msg = err.to_string();
        assert!(
            msg.contains("not_a_real_signal"),
            "error message should name the unknown signal; got: {msg}"
        );
        assert!(
            msg.contains("does not exist"),
            "error message should say 'does not exist'; got: {msg}"
        );
    }
}

// <FILE>xtask/src/docs/parse_signals_toml.rs</FILE> - <DESC>Parse docs/templates/signals.toml editorial overlay (engine / direct-API audience)</DESC>
// <VERS>END OF VERSION: 0.2.1</VERS>
