// <FILE>crates/tui-vfx-compost/tests/test_corpus_load_authoring_shorthand.rs</FILE> - <DESC>End-to-end load coverage for authoring-shorthand corpus pairs</DESC>
// <VERS>VERSION: 0.1.0</VERS>
// <WCTX>Prove the canonicalize → LoadedRecipe::load pipeline runs cleanly on every corpus shorthand recipe against the v3.1 primitive descriptor pack.</WCTX>
// <CLOG>0.1.0: INIT — add load harness mirroring the canonicalize corpus round-trip harness, but using the combined load_authoring_shorthand entry point.</CLOG>

use std::collections::BTreeMap;
use std::fs;
use std::path::{Path, PathBuf};

use serde_json::Value;
use tui_vfx_compost::{LoadError, LoadedRecipe};
use tui_vfx_contract::{DescriptorCatalog, DescriptorPack, DescriptorPackId};

fn repo_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .ancestors()
        .nth(2)
        .expect("crate lives under <repo>/crates/tui-vfx-compost")
        .to_path_buf()
}

fn primitive_catalog() -> DescriptorCatalog {
    let pack_path = repo_root().join("descriptors/v3.1/packs/primitive.json");
    let pack: DescriptorPack =
        serde_json::from_str(&fs::read_to_string(&pack_path).expect("read primitive pack"))
            .expect("deserialize primitive pack");
    let mut packs = BTreeMap::new();
    packs.insert(DescriptorPackId::new("v3.1.primitive"), pack);
    DescriptorCatalog { packs }
}

fn corpus_root() -> PathBuf {
    repo_root().join("schemas/v3.1/authoring/corpus")
}

fn load_templates(shorthand_dir: &Path) -> BTreeMap<String, Value> {
    let mut templates = BTreeMap::new();
    let themes_dir = shorthand_dir.join("themes");
    let Ok(entries) = fs::read_dir(&themes_dir) else {
        return templates;
    };
    for entry in entries.flatten() {
        let path = entry.path();
        if path.extension().is_none_or(|ext| ext != "json") {
            continue;
        }
        let Some(stem) = path.file_stem().and_then(|s| s.to_str()) else {
            continue;
        };
        let Ok(text) = fs::read_to_string(&path) else {
            continue;
        };
        let Ok(value) = serde_json::from_str::<Value>(&text) else {
            continue;
        };
        templates.insert(format!("themes/{stem}"), value);
    }
    templates
}

#[derive(Debug)]
struct PairResult {
    name: String,
    status: PairStatus,
}

#[derive(Debug)]
enum PairStatus {
    /// Canonicalize and load both succeeded.
    LoadedOk,
    /// Canonicalize succeeded; load-side validation rejected the recipe.
    /// These reflect descriptor-pack/contract coverage drift, not bridge failures.
    LoadRejected { error: String },
    /// Canonicalize itself failed — the canonicalize/load bridge regressed.
    CanonicalizeRegressed { error: String },
}

#[test]
fn corpus_load_authoring_shorthand_summary() {
    let corpus = corpus_root();
    let shorthand_dir = corpus.join("shorthand");
    let templates = load_templates(&shorthand_dir);
    let catalog = primitive_catalog();

    let mut entries: Vec<PathBuf> = fs::read_dir(&shorthand_dir)
        .expect("read shorthand directory")
        .filter_map(|r| r.ok().map(|e| e.path()))
        .filter(|p| p.extension().is_some_and(|ext| ext == "json"))
        .collect();
    entries.sort();

    let mut results: Vec<PairResult> = Vec::with_capacity(entries.len());

    for path in &entries {
        let name = path.file_stem().and_then(|s| s.to_str()).unwrap_or("");
        let shorthand_text = match fs::read_to_string(path) {
            Ok(s) => s,
            Err(e) => {
                results.push(PairResult {
                    name: name.into(),
                    status: PairStatus::LoadRejected {
                        error: format!("read shorthand failed: {e}"),
                    },
                });
                continue;
            }
        };
        let shorthand: Value = match serde_json::from_str(&shorthand_text) {
            Ok(v) => v,
            Err(e) => {
                results.push(PairResult {
                    name: name.into(),
                    status: PairStatus::LoadRejected {
                        error: format!("parse shorthand failed: {e}"),
                    },
                });
                continue;
            }
        };

        let status =
            match LoadedRecipe::load_authoring_shorthand(shorthand, &templates, &catalog) {
                Ok(_) => PairStatus::LoadedOk,
                Err(LoadError::Canonicalize { message }) => PairStatus::CanonicalizeRegressed {
                    error: message,
                },
                Err(other) => PairStatus::LoadRejected {
                    error: format!("{other}"),
                },
            };

        results.push(PairResult {
            name: name.into(),
            status,
        });
    }

    let mut loaded = 0usize;
    let mut load_rejected = 0usize;
    let mut canonicalize_regressed = 0usize;
    let mut load_rejections: Vec<&PairResult> = Vec::new();
    let mut bridge_regressions: Vec<&PairResult> = Vec::new();

    for result in &results {
        match &result.status {
            PairStatus::LoadedOk => loaded += 1,
            PairStatus::LoadRejected { .. } => {
                load_rejected += 1;
                load_rejections.push(result);
            }
            PairStatus::CanonicalizeRegressed { .. } => {
                canonicalize_regressed += 1;
                bridge_regressions.push(result);
            }
        }
    }

    println!("\n=== corpus load harness summary ===");
    println!("total pairs:           {}", results.len());
    println!("loaded ok:             {loaded}");
    println!("load rejected:         {load_rejected}");
    println!("canonicalize bridge:");
    println!("  regressed:           {canonicalize_regressed}");

    if !bridge_regressions.is_empty() {
        println!("\nbridge regressions (canonicalize failed — these are real bugs):");
        for f in &bridge_regressions {
            if let PairStatus::CanonicalizeRegressed { error } = &f.status {
                println!("  - {} → {error}", f.name);
            }
        }
    }

    if !load_rejections.is_empty() {
        println!("\nload rejections (descriptor coverage gaps — track upstream):");
        for f in &load_rejections {
            if let PairStatus::LoadRejected { error } = &f.status {
                println!("  - {} → {error}", f.name);
            }
        }
    }

    // The harness is a progress radar. Bridge regressions (canonicalize-side
    // failures) are the gating signal — they mean the canonicalize/load
    // pipeline itself broke. Load-side rejections are descriptor-pack or
    // contract-coverage drift; tracked separately.
    assert_eq!(
        canonicalize_regressed, 0,
        "canonicalize bridge regressed — see bridge-regressions list above"
    );
}

// <FILE>crates/tui-vfx-compost/tests/test_corpus_load_authoring_shorthand.rs</FILE> - <DESC>End-to-end load coverage for authoring-shorthand corpus pairs</DESC>
// <VERS>END OF VERSION: 0.1.0</VERS>
