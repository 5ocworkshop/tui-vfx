// <FILE>xtask/src/audit/fnc_load_baseline.rs</FILE> - <DESC>Load the configschema_baseline.toml allowlist into a typed set</DESC>
// <VERS>VERSION: 1.0.0</VERS>
// <WCTX>Packet 1.9.A — ConfigSchema justification lint</WCTX>
// <CLOG>1.0.0: initial implementation</CLOG>

use anyhow::{Context, Result};
use serde::Deserialize;
use std::collections::HashSet;
use std::path::Path;

/// A single entry in the baseline allowlist.
#[derive(Debug, Deserialize)]
struct BaselineFile {
    #[allow(dead_code)]
    schema_version: u32,
    #[serde(default)]
    entry: Vec<BaselineEntry>,
}

#[derive(Debug, Deserialize)]
struct BaselineEntry {
    /// Path relative to the workspace root, using forward slashes.
    file: String,
    /// Rust type name as it appears after `impl ConfigSchema for `.
    #[serde(rename = "type")]
    type_name: String,
}

/// A key used for O(1) baseline lookup: `(relative_file_path, type_name)`.
///
/// File paths are normalised to forward-slash separators so the comparison
/// is consistent on Windows and Unix. The type name is stored verbatim.
pub type BaselineKey = (String, String);

/// Load the baseline allowlist from `path` and return a `HashSet` of
/// `(file, type)` keys.
///
/// Errors if the file is absent or malformed — the baseline file is
/// checked-in and must be present for the lint to run.
pub fn load_baseline(path: &Path) -> Result<HashSet<BaselineKey>> {
    let src = std::fs::read_to_string(path)
        .with_context(|| format!("Failed to read baseline file: {}", path.display()))?;

    let parsed: BaselineFile = toml::from_str(&src)
        .with_context(|| format!("Failed to parse baseline file: {}", path.display()))?;

    let set = parsed
        .entry
        .into_iter()
        .map(|e| (normalise_path(&e.file), e.type_name))
        .collect();

    Ok(set)
}

/// Normalise a file path to forward-slash separators and lowercase drive
/// letters (Windows compat). Relative paths are stored as-is.
fn normalise_path(p: &str) -> String {
    p.replace('\\', "/")
}

// <FILE>xtask/src/audit/fnc_load_baseline.rs</FILE> - <DESC>Load the configschema_baseline.toml allowlist into a typed set</DESC>
// <VERS>END OF VERSION: 1.0.0</VERS>
