// <FILE>crates/tui-vfx-compost/tests/direct_recipe/test_write_merge_policy.rs</FILE> - <DESC>Compost write and merge policy substrate tests</DESC>
// <VERS>VERSION: 0.2.0</VERS>
// <WCTX>Write substrate tests cover final-cell skip behavior, node policy rejection, and explicit parallel merge rejection.</WCTX>
// <CLOG>0.2.0: MINOR — cover final-cell skip policy and node-local policy rejection.</CLOG>

use crate::support::{linear_gradient_recipe_value, primitive_catalog, recipe_from_value};
use tui_vfx_compost::{LoadError, LoadedRecipe, SampleContext, render_recipe};
use tui_vfx_types::{Color, RoleTag};

fn load_recipe_error(recipe: serde_json::Value) -> LoadError {
    let catalog = primitive_catalog();
    LoadedRecipe::load(recipe_from_value(recipe), &catalog).expect_err("recipe should fail load")
}

fn render_recipe_value(recipe: serde_json::Value) -> tui_vfx_compost::Frame {
    let catalog = primitive_catalog();
    let loaded = LoadedRecipe::load(recipe_from_value(recipe), &catalog).expect("load recipe");
    render_recipe(&loaded, &SampleContext::default()).expect("render recipe")
}

fn transparent_color() -> serde_json::Value {
    serde_json::json!({ "r": 0, "g": 0, "b": 0, "a": 0 })
}

fn make_empty_overlay(recipe: &mut serde_json::Value) -> serde_json::Value {
    recipe["sources"]["emptyOverlay"] = recipe["sources"]["mainCard"].clone();
    recipe["sources"]["emptyOverlay"]["inputs"]["message"]["value"]["value"] =
        serde_json::Value::String("   ".to_string());
    recipe["sources"]["emptyOverlay"]["inputs"]["foreground"]["value"]["value"] =
        transparent_color();
    recipe["sources"]["emptyOverlay"]["inputs"]["background"]["value"]["value"] =
        transparent_color();

    let mut overlay = recipe["scenes"][0]["elements"][0].clone();
    overlay["id"] = serde_json::Value::String("emptyOverlay".to_string());
    overlay["zIndex"] = serde_json::Value::Number(1.into());
    overlay["sourceInstance"] = serde_json::Value::String("emptyOverlay".to_string());
    overlay["cellWritePolicy"] = serde_json::Value::String("skipTransparentEmpty".to_string());
    overlay
}

#[test]
fn skip_transparent_empty_cell_write_preserves_destination_cell_and_role() {
    let mut recipe = linear_gradient_recipe_value();
    let overlay = make_empty_overlay(&mut recipe);
    recipe["graph"]["nodes"]["gradient"]["inputs"]["intensity"]["value"]["value"] =
        serde_json::Value::Number(0.into());
    recipe["scenes"][0]["elements"]
        .as_array_mut()
        .expect("scene elements")
        .push(overlay);

    let frame = render_recipe_value(recipe);

    assert_eq!(frame.grid.cell((0, 0)).unwrap().ch, 'A');
    assert_eq!(frame.grid.cell((1, 0)).unwrap().ch, 'B');
    assert_eq!(frame.grid.cell((2, 0)).unwrap().ch, 'C');
    assert_eq!(frame.grid.role((0, 0)), Some(RoleTag::Background));
}

#[test]
fn skip_transparent_empty_cell_write_checks_final_effect_output() {
    let mut recipe = linear_gradient_recipe_value();
    let overlay = make_empty_overlay(&mut recipe);
    recipe["graph"]["nodes"]["gradient"]["inputs"]["channelTarget"]["value"]["value"] =
        serde_json::Value::String("background".to_string());
    recipe["scenes"][0]["elements"]
        .as_array_mut()
        .expect("scene elements")
        .push(overlay);

    let frame = render_recipe_value(recipe);

    assert_eq!(frame.grid.cell((0, 0)).unwrap().ch, ' ');
    assert_eq!(frame.grid.cell((0, 0)).unwrap().bg, Color::RED);
}

#[test]
fn rejects_parallel_graph_merge_policy_until_surface_merge_exists() {
    let mut recipe = linear_gradient_recipe_value();
    recipe["graph"]["topology"] = serde_json::json!({
        "kind": "parallel",
        "children": [{ "kind": "node", "node": "gradient" }],
        "mergePolicy": "childOrderLastWriterWins",
        "valueMergePolicy": "childOrderLastWriterWins"
    });

    let error = load_recipe_error(recipe);

    assert!(matches!(
        error,
        LoadError::UnsupportedGraphMergePolicy { field, .. }
            if field == "graph.topology.mergePolicy"
    ));
}

#[test]
fn rejects_node_local_write_policies_until_stage_precedence_exists() {
    let mut role_recipe = linear_gradient_recipe_value();
    role_recipe["graph"]["nodes"]["gradient"]["roleWritePolicy"] =
        serde_json::json!({ "kind": "copySampledSource" });

    let role_error = load_recipe_error(role_recipe);

    assert!(matches!(
        role_error,
        LoadError::UnsupportedNodeWritePolicy {
            node_id,
            effect,
            field,
            ..
        } if node_id == "gradient"
            && effect == "shader.linearGradient"
            && field == "roleWritePolicy"
    ));

    let mut cell_recipe = linear_gradient_recipe_value();
    cell_recipe["graph"]["nodes"]["gradient"]["cellWritePolicy"] =
        serde_json::Value::String("skipTransparentEmpty".to_string());

    let cell_error = load_recipe_error(cell_recipe);

    assert!(matches!(
        cell_error,
        LoadError::UnsupportedNodeWritePolicy {
            node_id,
            effect,
            field,
            ..
        } if node_id == "gradient"
            && effect == "shader.linearGradient"
            && field == "cellWritePolicy"
    ));
}

// <FILE>crates/tui-vfx-compost/tests/direct_recipe/test_write_merge_policy.rs</FILE> - <DESC>Compost write and merge policy substrate tests</DESC>
// <VERS>END OF VERSION: 0.2.0</VERS>
