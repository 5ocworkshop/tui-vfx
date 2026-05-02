// <FILE>crates/tui-vfx-contract/tests/test_canonicalize_corpus.rs</FILE> - <DESC>Mass corpus harness over schemas/v3.1/authoring/corpus pairs</DESC>
// <VERS>VERSION: 0.1.0</VERS>
// <WCTX>Phase 3 of canonicalize: iterate every corpus shorthand/canonical pair and report canonicalize and round-trip status.</WCTX>
// <CLOG>0.1.0: INIT — driver harness; reports per-pair canonicalize success vs round-trip mismatch with full diffs at failure.</CLOG>

use std::collections::BTreeMap;
use std::path::{Path, PathBuf};

use serde_json::Value;
use tui_vfx_contract::RecipeDocument;
use tui_vfx_contract::canonicalize::canonicalize_recipe_with_templates;

fn corpus_root() -> PathBuf {
    PathBuf::from(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../schemas/v3.1/authoring/corpus"
    ))
}

/// Load every JSON file under `<shorthand>/themes/` into a template map keyed
/// by `themes/<stem>` so author-side `extends: "themes/<stem>"` resolves.
fn load_templates(shorthand_dir: &Path) -> BTreeMap<String, Value> {
    let mut templates = BTreeMap::new();
    let themes_dir = shorthand_dir.join("themes");
    let Ok(entries) = std::fs::read_dir(&themes_dir) else {
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
        let Ok(text) = std::fs::read_to_string(&path) else {
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

#[derive(Debug, PartialEq)]
enum PairStatus {
    /// Canonicalize succeeded and the output matched the corpus canonical fixture.
    RoundTripOk,
    /// Canonicalize ran cleanly but the corpus canonical fixture has a stale shape;
    /// the shorthand-side path is healthy.
    CanonicalizeOkCanonicalDrifted,
    /// Canonicalize errored on the shorthand input.
    CanonicalizeFailed { error: String },
    /// Canonicalize succeeded; the canonical fixture parses but doesn't match.
    Mismatch,
}

fn classify(
    name: &str,
    shorthand: &Value,
    canonical_str: &str,
    templates: &BTreeMap<String, Value>,
) -> PairStatus {
    let canonicalized = match canonicalize_recipe_with_templates(shorthand.clone(), templates) {
        Ok(recipe) => recipe,
        Err(e) => {
            return PairStatus::CanonicalizeFailed {
                error: format!("{e}"),
            };
        }
    };
    let lhs = match serde_json::to_value(&canonicalized) {
        Ok(v) => v,
        Err(e) => {
            return PairStatus::CanonicalizeFailed {
                error: format!("serialize after canonicalize failed for {name}: {e}"),
            };
        }
    };

    let canonical_value: Value = match serde_json::from_str(canonical_str) {
        Ok(v) => v,
        Err(_) => return PairStatus::CanonicalizeOkCanonicalDrifted,
    };
    let canonical_recipe: RecipeDocument = match serde_json::from_value(canonical_value) {
        Ok(r) => r,
        Err(_) => return PairStatus::CanonicalizeOkCanonicalDrifted,
    };
    let rhs = match serde_json::to_value(&canonical_recipe) {
        Ok(v) => v,
        Err(_) => return PairStatus::CanonicalizeOkCanonicalDrifted,
    };

    if lhs == rhs {
        PairStatus::RoundTripOk
    } else {
        PairStatus::Mismatch
    }
}

#[test]
fn corpus_round_trip_summary() {
    let corpus = corpus_root();
    let shorthand_dir = corpus.join("shorthand");
    let canonical_dir = corpus.join("canonical");

    let mut entries: Vec<PathBuf> = std::fs::read_dir(&shorthand_dir)
        .expect("read shorthand directory")
        .filter_map(|r| r.ok().map(|e| e.path()))
        .filter(|p| p.extension().is_some_and(|ext| ext == "json"))
        .collect();
    entries.sort();

    let templates = load_templates(&shorthand_dir);

    let mut results: Vec<PairResult> = Vec::with_capacity(entries.len());

    for path in &entries {
        let name = path.file_stem().and_then(|s| s.to_str()).unwrap_or("");
        let shorthand_str = match std::fs::read_to_string(path) {
            Ok(s) => s,
            Err(e) => {
                results.push(PairResult {
                    name: name.into(),
                    status: PairStatus::CanonicalizeFailed {
                        error: format!("read shorthand failed: {e}"),
                    },
                });
                continue;
            }
        };
        let shorthand: Value = match serde_json::from_str(&shorthand_str) {
            Ok(v) => v,
            Err(e) => {
                results.push(PairResult {
                    name: name.into(),
                    status: PairStatus::CanonicalizeFailed {
                        error: format!("parse shorthand failed: {e}"),
                    },
                });
                continue;
            }
        };

        let canonical_path = canonical_dir.join(format!("{name}.json"));
        let canonical_str = std::fs::read_to_string(&canonical_path).unwrap_or_default();

        results.push(PairResult {
            name: name.into(),
            status: classify(name, &shorthand, &canonical_str, &templates),
        });
    }

    let mut round_trip = 0usize;
    let mut canonicalize_ok_drifted = 0usize;
    let mut canonicalize_failed = 0usize;
    let mut mismatch = 0usize;

    let mut failures: Vec<&PairResult> = Vec::new();
    let mut drifts: Vec<&PairResult> = Vec::new();

    for result in &results {
        match &result.status {
            PairStatus::RoundTripOk => round_trip += 1,
            PairStatus::CanonicalizeOkCanonicalDrifted => {
                canonicalize_ok_drifted += 1;
                drifts.push(result);
            }
            PairStatus::CanonicalizeFailed { .. } => {
                canonicalize_failed += 1;
                failures.push(result);
            }
            PairStatus::Mismatch => {
                mismatch += 1;
                failures.push(result);
            }
        }
    }

    println!("\n=== corpus round-trip summary ===");
    println!("total pairs:         {}", results.len());
    println!("round-trip ok:       {round_trip}");
    println!("canonicalize ok,");
    println!("  canonical drifted: {canonicalize_ok_drifted}");
    println!("canonicalize failed: {canonicalize_failed}");
    println!("structural mismatch: {mismatch}");

    if !failures.is_empty() {
        println!("\nfailures (need helper work):");
        for f in &failures {
            match &f.status {
                PairStatus::CanonicalizeFailed { error } => {
                    println!("  - {} → canonicalize: {error}", f.name);
                }
                PairStatus::Mismatch => {
                    println!("  - {} → mismatch (canonical fixture out of sync)", f.name);
                }
                _ => {}
            }
        }
    }

    if !drifts.is_empty() {
        println!("\ndrifted canonicals (regen paperwork):");
        for d in &drifts {
            println!("  - {}", d.name);
        }
    }

    // The harness intentionally does NOT fail when pairs are stale; it's a
    // progress radar. The dedicated round-trip tests in
    // test_canonicalize_recipe.rs are the gating tests. The harness only fails
    // if not a single pair could be canonicalized — that would mean the basic
    // pipeline regressed.
    assert!(
        round_trip + canonicalize_ok_drifted > 0,
        "no shorthand recipes canonicalized cleanly — basic pipeline regression"
    );
}

// <FILE>crates/tui-vfx-contract/tests/test_canonicalize_corpus.rs</FILE> - <DESC>Mass corpus harness over schemas/v3.1/authoring/corpus pairs</DESC>
// <VERS>END OF VERSION: 0.1.0</VERS>
