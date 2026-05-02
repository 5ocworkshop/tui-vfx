// <FILE>crates/tui-vfx-compost/tests/direct_recipe/test_node_write_policy.rs</FILE> - <DESC>Compost node-local write policy tests</DESC>
// <VERS>VERSION: 0.1.0</VERS>
// <WCTX>Node-local write policy tests prove stage-local cell and role policy precedence over element defaults.</WCTX>
// <CLOG>0.1.0: INIT — cover node-local cell skip and role copy policies.</CLOG>

use crate::support::{linear_gradient_recipe_value, primitive_catalog, recipe_from_value};
use tui_vfx_compost::{LoadedRecipe, SampleContext, render_recipe};
use tui_vfx_types::{Color, RoleTag};

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
    overlay
}

fn clone_gradient_node(recipe: &serde_json::Value, id: &str) -> serde_json::Value {
    let mut node = recipe["graph"]["nodes"]["gradient"].clone();
    node["id"] = serde_json::Value::String(id.to_string());
    node
}

#[test]
fn node_local_cell_write_policy_overrides_element_default() {
    let mut recipe = linear_gradient_recipe_value();
    let overlay = make_empty_overlay(&mut recipe);
    recipe["graph"]["nodes"]["gradient"]["inputs"]["intensity"]["value"]["value"] =
        serde_json::Value::Number(0.into());
    recipe["graph"]["nodes"]["gradient"]["cellWritePolicy"] =
        serde_json::Value::String("skipTransparentEmpty".to_string());
    recipe["scenes"][0]["elements"]
        .as_array_mut()
        .expect("scene elements")
        .push(overlay);

    let frame = render_recipe_value(recipe);

    assert_eq!(frame.grid.cell((0, 0)).unwrap().ch, 'A');
    assert_eq!(frame.grid.cell((0, 0)).unwrap().fg, Color::WHITE);
}

#[test]
fn later_default_node_does_not_clear_node_local_cell_policy() {
    let mut recipe = linear_gradient_recipe_value();
    let overlay = make_empty_overlay(&mut recipe);
    let mut default_node = clone_gradient_node(&recipe, "defaultGradient");
    default_node["inputs"]["intensity"]["value"]["value"] = serde_json::Value::Number(0.into());
    recipe["graph"]["nodes"]["gradient"]["inputs"]["intensity"]["value"]["value"] =
        serde_json::Value::Number(0.into());
    recipe["graph"]["nodes"]["gradient"]["cellWritePolicy"] =
        serde_json::Value::String("skipTransparentEmpty".to_string());
    recipe["graph"]["nodes"]["defaultGradient"] = default_node;
    recipe["graph"]["order"] = serde_json::json!(["gradient", "defaultGradient"]);
    recipe["scenes"][0]["elements"]
        .as_array_mut()
        .expect("scene elements")
        .push(overlay);

    let frame = render_recipe_value(recipe);

    assert_eq!(frame.grid.cell((0, 0)).unwrap().ch, 'A');
    assert_eq!(frame.grid.cell((0, 0)).unwrap().fg, Color::WHITE);
}

#[test]
fn node_local_role_policy_overrides_element_default() {
    let mut recipe = linear_gradient_recipe_value();
    recipe["graph"]["nodes"]["gradient"]["roleWritePolicy"] =
        serde_json::json!({ "kind": "copySampledSource" });

    let frame = render_recipe_value(recipe);

    assert_eq!(frame.grid.cell((0, 0)).unwrap().ch, 'A');
    assert_eq!(frame.grid.role((0, 0)), Some(RoleTag::Text));
}

#[test]
fn later_default_node_does_not_clear_node_local_role_policy() {
    let mut recipe = linear_gradient_recipe_value();
    let default_node = clone_gradient_node(&recipe, "defaultGradient");
    recipe["graph"]["nodes"]["gradient"]["roleWritePolicy"] =
        serde_json::json!({ "kind": "copySampledSource" });
    recipe["graph"]["nodes"]["defaultGradient"] = default_node;
    recipe["graph"]["order"] = serde_json::json!(["gradient", "defaultGradient"]);

    let frame = render_recipe_value(recipe);

    assert_eq!(frame.grid.cell((0, 0)).unwrap().ch, 'A');
    assert_eq!(frame.grid.role((0, 0)), Some(RoleTag::Text));
}

#[test]
fn parallel_branch_role_policy_survives_without_style_delta() {
    let mut recipe = linear_gradient_recipe_value();
    let mut policy_node = clone_gradient_node(&recipe, "policyGradient");
    policy_node["inputs"]["intensity"]["value"]["value"] = serde_json::Value::Number(0.into());
    policy_node["roleWritePolicy"] = serde_json::json!({ "kind": "copySampledSource" });
    recipe["graph"]["nodes"] = serde_json::json!({ "policyGradient": policy_node });
    recipe["graph"]["order"] = serde_json::json!(["policyGradient"]);
    recipe["graph"]["topology"] = serde_json::json!({
        "kind": "parallel",
        "children": [
            { "kind": "node", "node": "policyGradient" }
        ],
        "mergePolicy": "childOrderLastWriterWins",
        "valueMergePolicy": "childOrderLastWriterWins"
    });

    let frame = render_recipe_value(recipe);

    assert_eq!(frame.grid.cell((0, 0)).unwrap().ch, 'A');
    assert_eq!(frame.grid.role((0, 0)), Some(RoleTag::Text));
}

#[test]
fn parallel_branch_policy_survives_without_style_delta() {
    let mut recipe = linear_gradient_recipe_value();
    let overlay = make_empty_overlay(&mut recipe);
    let mut policy_node = clone_gradient_node(&recipe, "policyGradient");
    policy_node["inputs"]["intensity"]["value"]["value"] = serde_json::Value::Number(0.into());
    policy_node["cellWritePolicy"] = serde_json::Value::String("skipTransparentEmpty".to_string());
    recipe["graph"]["nodes"] = serde_json::json!({ "policyGradient": policy_node });
    recipe["graph"]["order"] = serde_json::json!(["policyGradient"]);
    recipe["graph"]["topology"] = serde_json::json!({
        "kind": "parallel",
        "children": [
            { "kind": "node", "node": "policyGradient" }
        ],
        "mergePolicy": "childOrderLastWriterWins",
        "valueMergePolicy": "childOrderLastWriterWins"
    });
    recipe["scenes"][0]["elements"]
        .as_array_mut()
        .expect("scene elements")
        .push(overlay);

    let frame = render_recipe_value(recipe);

    assert_eq!(frame.grid.cell((0, 0)).unwrap().ch, 'A');
    assert_eq!(frame.grid.cell((0, 0)).unwrap().fg, Color::WHITE);
}

// <FILE>crates/tui-vfx-compost/tests/direct_recipe/test_node_write_policy.rs</FILE> - <DESC>Compost node-local write policy tests</DESC>
// <VERS>END OF VERSION: 0.1.0</VERS>
