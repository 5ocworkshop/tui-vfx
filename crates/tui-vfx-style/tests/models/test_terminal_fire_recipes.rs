// <FILE>tui-vfx-style/tests/models/test_terminal_fire_recipes.rs</FILE> - <DESC>End-to-end validation that the five fire-shader debug recipes parse through SpatialShaderType</DESC>
// <VERS>VERSION: 0.1.0</VERS>
// <WCTX>Substitute validation for the 5 shader_terminal_fire_*_v3.json fixtures while the parent's pipeline-validator is broken by an unrelated in-flight lane. Loads each recipe from disk, walks to config.pipeline.step.payload, deserializes through SpatialShaderType.</WCTX>
// <CLOG>0.1.0: initial recipe-shape coverage. Asserts payload deserializes, name() == "TerminalFire", mode survives the parse, and key_parameters() returns the expected catalog entries.</CLOG>

use std::path::PathBuf;
use tui_vfx_style::models::{FireMode, SpatialShaderType};

fn recipes_dir() -> PathBuf {
    // Worktree layout: ../mixed-signals, ../tui-vfx-recipes are siblings of
    // the worktree directory. CARGO_MANIFEST_DIR points at the style crate's
    // Cargo.toml, so step up to the worktree root then across to the recipes
    // sibling.
    let manifest = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let worktree_root = manifest
        .parent() // crates/
        .and_then(|p| p.parent()) // worktree root
        .expect("locate worktree root from CARGO_MANIFEST_DIR");
    worktree_root
        .parent() // the directory holding the worktree + sibling repos
        .expect("locate worktree parent")
        .join("tui-vfx-recipes")
        .join("recipes/debug_recipes/shaders/primitives")
}

fn load_payload(path: &str) -> serde_json::Value {
    let full = recipes_dir().join(path);
    let bytes =
        std::fs::read(&full).unwrap_or_else(|e| panic!("read recipe {}: {}", full.display(), e));
    let value: serde_json::Value =
        serde_json::from_slice(&bytes).expect("recipe must be valid JSON");
    value
        .pointer("/config/pipeline/step/payload")
        .cloned()
        .unwrap_or_else(|| {
            panic!(
                "recipe {} missing /config/pipeline/step/payload",
                full.display()
            )
        })
}

fn assert_recipe_deserializes(filename: &str, expected_mode: FireMode) {
    let payload = load_payload(filename);
    let shader: SpatialShaderType = serde_json::from_value(payload)
        .unwrap_or_else(|e| panic!("recipe {} payload failed to deserialize: {}", filename, e));

    assert_eq!(
        shader.name(),
        "TerminalFire",
        "recipe {} did not deserialize as TerminalFire",
        filename
    );

    let SpatialShaderType::TerminalFire(fire) = shader else {
        unreachable!("name() check above guarantees this branch");
    };
    assert_eq!(
        fire.mode, expected_mode,
        "recipe {} had unexpected mode {:?}",
        filename, fire.mode
    );

    let key_names: Vec<&'static str> = SpatialShaderType::TerminalFire(fire.clone())
        .key_parameters()
        .into_iter()
        .map(|(k, _)| k)
        .collect();
    for required in ["mode", "base_width", "rise_speed", "intensity", "sparks"] {
        assert!(
            key_names.contains(&required),
            "recipe {}: key_parameters() missing {} (got {:?})",
            filename,
            required,
            key_names
        );
    }
}

#[test]
fn shader_terminal_fire_v3_recipe_deserializes_as_flame() {
    assert_recipe_deserializes("shader_terminal_fire_v3.json", FireMode::Flame);
}

#[test]
fn shader_terminal_fire_candle_v3_recipe_deserializes_as_candle() {
    assert_recipe_deserializes("shader_terminal_fire_candle_v3.json", FireMode::Candle);
}

#[test]
fn shader_terminal_fire_campfire_v3_recipe_deserializes_as_campfire() {
    assert_recipe_deserializes("shader_terminal_fire_campfire_v3.json", FireMode::Campfire);
}

#[test]
fn shader_terminal_fire_embers_v3_recipe_deserializes_as_embers() {
    assert_recipe_deserializes("shader_terminal_fire_embers_v3.json", FireMode::Embers);
}

#[test]
fn shader_terminal_fire_smoke_plume_v3_recipe_deserializes_as_smoke_plume() {
    assert_recipe_deserializes(
        "shader_terminal_fire_smoke_plume_v3.json",
        FireMode::SmokePlume,
    );
}

// <FILE>tui-vfx-style/tests/models/test_terminal_fire_recipes.rs</FILE>
// <VERS>END OF VERSION: 0.1.0</VERS>
