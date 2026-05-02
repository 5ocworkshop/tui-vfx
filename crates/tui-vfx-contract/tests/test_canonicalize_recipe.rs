// <FILE>crates/tui-vfx-contract/tests/test_canonicalize_recipe.rs</FILE> - <DESC>Integration tests for the authoring shorthand canonicalize entry point</DESC>
// <VERS>VERSION: 0.2.0</VERS>
// <WCTX>Phase 2 of canonicalize: extend round-trip coverage to filter_dim and confirm channel-scoping lifts to writeChannels.</WCTX>
// <CLOG>0.2.0: MINOR — add filter_dim round-trip plus an explicit Gap 1 channel-scoping verification gate.
// 0.1.0: INIT — round-trip test for baseline.json plus idempotence on canonical input.</CLOG>

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

const SHORTHAND_FILTER_DIM: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../schemas/v3.1/authoring/corpus/shorthand/filter_dim.json"
));

const CANONICAL_FILTER_DIM: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../schemas/v3.1/authoring/corpus/canonical/filter_dim.json"
));

fn assert_round_trip(label: &str, shorthand_str: &str, canonical_str: &str) {
    let shorthand: Value = serde_json::from_str(shorthand_str).expect("shorthand parses");
    let canonical_value: Value = serde_json::from_str(canonical_str).expect("canonical parses");

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
            "{label} round-trip mismatch\n--- canonicalize output ---\n{lhs}\n--- corpus canonical ---\n{rhs}"
        );
    }
}

#[test]
fn baseline_shorthand_round_trips_to_canonical() {
    assert_round_trip("baseline", SHORTHAND_BASELINE, CANONICAL_BASELINE);
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

#[test]
fn filter_dim_shorthand_round_trips_to_canonical() {
    assert_round_trip("filter_dim", SHORTHAND_FILTER_DIM, CANONICAL_FILTER_DIM);
}

/// Channel-scoping verification gate (response-memo verification request).
///
/// Asserts that author-side `scope: { channel: "<X>" }` lifts to
/// `NodeSpec.writeChannels = ["<X>"]` and that no `scope.kind = "channel"`
/// (the rejected Gap 1 Option A shape) ever appears in canonicalize output.
#[test]
fn filter_dim_channel_scope_lifts_to_write_channels() {
    let shorthand: Value = serde_json::from_str(SHORTHAND_FILTER_DIM).expect("shorthand parses");
    let canonicalized = canonicalize_recipe(shorthand).expect("canonicalize succeeds");
    let value = serde_json::to_value(&canonicalized).expect("serialize succeeds");

    let exit_node = value
        .pointer("/graph/nodes/dim1")
        .expect("second filter.dim node present");
    assert_eq!(
        exit_node.get("writeChannels").cloned().unwrap_or_default(),
        serde_json::json!(["foreground"]),
        "channel scope must lift to writeChannels"
    );
    assert!(
        !value_contains_channel_scope(&value),
        "no NodeSpec.scope variant may use kind = 'channel' (rejected Option A)"
    );
}

fn value_contains_channel_scope(value: &Value) -> bool {
    match value {
        Value::Object(map) => {
            if map.get("kind").and_then(Value::as_str) == Some("channel") {
                return true;
            }
            map.values().any(value_contains_channel_scope)
        }
        Value::Array(arr) => arr.iter().any(value_contains_channel_scope),
        _ => false,
    }
}

// <FILE>crates/tui-vfx-contract/tests/test_canonicalize_recipe.rs</FILE> - <DESC>Integration tests for the authoring shorthand canonicalize entry point</DESC>
// <VERS>END OF VERSION: 0.1.0</VERS>
