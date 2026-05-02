// <FILE>crates/tui-vfx-contract/tests/test_canonicalize_recipe.rs</FILE> - <DESC>Integration tests for the authoring shorthand canonicalize entry point</DESC>
// <VERS>VERSION: 0.1.0</VERS>
// <WCTX>Phase 1 of canonicalize: prove the baseline.json corpus pair round-trips through canonicalize_recipe.</WCTX>
// <CLOG>0.1.0: INIT — round-trip test for baseline.json plus idempotence on canonical input.</CLOG>

use serde_json::Value;
use tui_vfx_contract::RecipeDocument;
use tui_vfx_contract::canonicalize::canonicalize_recipe;

const SHORTHAND_BASELINE: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../schemas/v3.1/authoring/corpus/shorthand/baseline.json"
));

const CANONICAL_BASELINE: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../schemas/v3.1/authoring/corpus/canonical/baseline.json"
));

#[test]
fn baseline_shorthand_round_trips_to_canonical() {
    let shorthand: Value = serde_json::from_str(SHORTHAND_BASELINE).expect("shorthand parses");
    let canonical_value: Value =
        serde_json::from_str(CANONICAL_BASELINE).expect("canonical parses");

    let canonicalized = canonicalize_recipe(shorthand).expect("canonicalize succeeds");
    let canonicalized_value = serde_json::to_value(&canonicalized).expect("serialize succeeds");

    let canonical_recipe: RecipeDocument =
        serde_json::from_value(canonical_value).expect("canonical deserialize");
    let canonical_normalized =
        serde_json::to_value(&canonical_recipe).expect("canonical re-serialize");

    if canonicalized_value != canonical_normalized {
        let lhs = serde_json::to_string_pretty(&canonicalized_value).unwrap();
        let rhs = serde_json::to_string_pretty(&canonical_normalized).unwrap();
        panic!(
            "baseline round-trip mismatch\n--- canonicalize output ---\n{lhs}\n--- corpus canonical ---\n{rhs}"
        );
    }
}

#[test]
fn canonical_baseline_is_idempotent_through_canonicalize() {
    let canonical: Value = serde_json::from_str(CANONICAL_BASELINE).expect("canonical parses");
    let canonicalized =
        canonicalize_recipe(canonical.clone()).expect("canonical input canonicalizes");
    let canonicalized_value = serde_json::to_value(&canonicalized).unwrap();

    let canonical_recipe: RecipeDocument =
        serde_json::from_value(canonical).expect("canonical deserialize");
    let canonical_normalized = serde_json::to_value(&canonical_recipe).unwrap();

    assert_eq!(canonicalized_value, canonical_normalized);
}

// <FILE>crates/tui-vfx-contract/tests/test_canonicalize_recipe.rs</FILE> - <DESC>Integration tests for the authoring shorthand canonicalize entry point</DESC>
// <VERS>END OF VERSION: 0.1.0</VERS>
