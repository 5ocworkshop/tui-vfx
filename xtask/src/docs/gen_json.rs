// <FILE>xtask/src/docs/gen_json.rs</FILE> - <DESC>Generate capabilities.json from merged data</DESC>
// <VERS>VERSION: 1.0.1</VERS>
// <WCTX>Emit machine-readable capabilities manifest</WCTX>
// <CLOG>Generate deterministic capabilities.json output</CLOG>

use super::merge::MergedManifest;
use anyhow::{Context, Result};
use serde_json::Value;
use std::fs;
use std::path::Path;

const OUTPUT_PATH: &str = "docs/generated/capabilities.json";

/// Generate capabilities.json from merged manifest.
pub fn generate(manifest: &MergedManifest) -> Result<()> {
    write_manifest(manifest, Path::new(OUTPUT_PATH))
}

/// Render capabilities.json content without writing to disk.
pub fn render(manifest: &MergedManifest) -> Result<String> {
    let mut value =
        serde_json::to_value(manifest).context("Failed to serialize manifest to JSON")?;
    sort_json_value(&mut value);
    serde_json::to_string_pretty(&value).context("Failed to format manifest JSON")
}

fn write_manifest(manifest: &MergedManifest, path: &Path) -> Result<()> {
    let output = render(manifest)?;

    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)
            .with_context(|| format!("Failed to create {}", parent.display()))?;
    }

    fs::write(path, output).with_context(|| format!("Failed to write {}", path.display()))?;
    Ok(())
}

fn sort_json_value(value: &mut Value) {
    match value {
        Value::Object(map) => {
            let mut entries: Vec<(String, Value)> =
                map.iter().map(|(k, v)| (k.clone(), v.clone())).collect();
            for (_, v) in entries.iter_mut() {
                sort_json_value(v);
            }
            entries.sort_by(|a, b| a.0.cmp(&b.0));
            let mut sorted = serde_json::Map::new();
            for (k, v) in entries {
                sorted.insert(k, v);
            }
            *map = sorted;
        }
        Value::Array(items) => {
            for item in items.iter_mut() {
                sort_json_value(item);
            }
        }
        _ => {}
    }
}

#[cfg(test)]
mod tests {
    use super::write_manifest;
    use crate::docs::merge::{MergedEffects, MergedManifest};
    use crate::docs::parse_toml::{ConstraintsSection, SemanticsSection};
    use std::collections::HashMap;
    use std::env;
    use std::fs;
    use std::time::{SystemTime, UNIX_EPOCH};

    #[test]
    fn writes_capabilities_json() {
        let manifest = MergedManifest {
            version: "0.0.0".to_string(),
            layers: HashMap::new(),
            phases: HashMap::new(),
            effects: MergedEffects::default(),
            semantics: SemanticsSection::default(),
            recipes: HashMap::new(),
            constraints: ConstraintsSection::default(),
        };

        let stamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("time went backwards")
            .as_nanos();
        let mut path = env::temp_dir();
        path.push(format!("tui-vfx-capabilities-{}.json", stamp));

        let result = write_manifest(&manifest, &path);
        assert!(result.is_ok(), "expected JSON generation to succeed");

        let content = fs::read_to_string(&path).expect("expected JSON file to exist");
        let json: serde_json::Value = serde_json::from_str(&content).expect("expected valid JSON");
        assert!(json.get("version").is_some());
        assert!(json.get("effects").is_some());
        assert!(json.get("semantics").is_some());
        assert!(json.get("recipes").is_some());
        assert!(json.get("constraints").is_some());

        let _ = fs::remove_file(&path);
    }
}

// <FILE>xtask/src/docs/gen_json.rs</FILE> - <DESC>Generate capabilities.json from merged data</DESC>
// <VERS>END OF VERSION: 1.0.1</VERS>
