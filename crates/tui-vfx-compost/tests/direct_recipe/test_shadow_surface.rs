// <FILE>crates/tui-vfx-compost/tests/direct_recipe/test_shadow_surface.rs</FILE> - <DESC>Compost scene-element surface shadow tests</DESC>
// <VERS>VERSION: 0.2.0</VERS>
// <WCTX>Surface shadow regressions lock mature tui-vfx-shadow geometry and compositor blending adapted to v3.1 scene elements.</WCTX>
// <CLOG>0.2.0: MINOR — accept role-scoped shadow source regions for bordered source surfaces.
// 0.1.0: INIT — cover source-over surface shadows, shadow role tagging, baseStyle rejection, and preserveDestination material.</CLOG>

use crate::support::{linear_gradient_recipe_value, primitive_catalog, recipe_from_value};
use tui_vfx_compost::{LoadedRecipe, SampleContext, render_recipe};
use tui_vfx_types::RoleTag;

fn source_with_message(message: &str, width: i64, height: i64) -> serde_json::Value {
    let mut source = linear_gradient_recipe_value()["sources"]["mainCard"].clone();
    source["inputs"]["message"]["value"]["value"] = serde_json::Value::String(message.to_string());
    source["inputs"]["width"]["value"]["value"] = serde_json::Value::Number(width.into());
    source["inputs"]["height"]["value"]["value"] = serde_json::Value::Number(height.into());
    source
}

fn element_with_source(
    id: &str,
    source_instance: &str,
    x: i64,
    y: i64,
    z_index: i64,
) -> serde_json::Value {
    serde_json::json!({
        "id": id,
        "layer": "primary",
        "zIndex": z_index,
        "placement": { "x": x, "y": y },
        "sourceInstance": source_instance,
        "graphBinding": null,
        "clipPolicy": "clip",
        "cellWritePolicy": "writeCell",
        "roleWritePolicy": { "kind": "preserveDestination" }
    })
}

fn render_shadow_recipe(recipe: serde_json::Value) -> tui_vfx_compost::Frame {
    let catalog = primitive_catalog();
    let loaded = LoadedRecipe::load(recipe_from_value(recipe), &catalog).expect("load recipe");
    render_recipe(&loaded, &SampleContext::default()).expect("render scene")
}

fn visible_shadow_alpha(recipe: serde_json::Value, x: u16, y: u16) -> u8 {
    let frame = render_shadow_recipe(recipe);
    let cell = frame
        .grid
        .cell((x, y))
        .expect("shadow cell in scene bounds");
    cell.bg.a.max(cell.fg.a)
}

fn surface_shadow() -> serde_json::Value {
    serde_json::json!({
        "shadow": {
            "edges": ["right", "bottom"],
            "offset": { "x": 1, "y": 1 },
            "shadowColor": { "r": 0, "g": 0, "b": 0, "a": 160 },
            "softEdges": true,
            "compositeMode": "under",
            "blendMode": "sourceOver",
            "edgeCrossingPolicy": "default",
            "glyphMaterial": "solid"
        }
    })
}

#[test]
fn surface_shadow_renders_before_element_and_tags_shadow_cells() {
    let mut recipe = linear_gradient_recipe_value();
    recipe["scenes"][0]["width"] = serde_json::json!(6);
    recipe["scenes"][0]["height"] = serde_json::json!(5);
    recipe["sources"]["mainCard"] = source_with_message("ABCD\nEFGH\nIJKL", 4, 3);
    recipe["scenes"][0]["elements"][0]["surface"] = surface_shadow();

    let frame = render_shadow_recipe(recipe);

    assert_eq!(frame.grid.cell((0, 0)).unwrap().ch, 'A');
    let right_shadow = frame.grid.cell((4, 1)).unwrap();
    assert!(
        right_shadow.bg.a > 0 || right_shadow.fg.a > 0,
        "right shadow should carry visible shadow color"
    );
    assert_eq!(frame.grid.role((4, 1)), Some(RoleTag::Shadow));
    assert_eq!(frame.grid.role((0, 0)), Some(RoleTag::Background));
}

#[test]
fn default_edge_crossing_casts_shadow_from_visible_clipped_bounds() {
    let mut recipe = linear_gradient_recipe_value();
    recipe["scenes"][0]["width"] = serde_json::json!(6);
    recipe["scenes"][0]["height"] = serde_json::json!(5);
    recipe["sources"]["mainCard"] = source_with_message(
        "ABCD
EFGH
IJKL",
        4,
        3,
    );
    recipe["scenes"][0]["elements"][0]["placement"] = serde_json::json!({ "x": -2, "y": 0 });
    recipe["scenes"][0]["elements"][0]["surface"] = surface_shadow();

    let frame = render_shadow_recipe(recipe);

    assert_eq!(frame.grid.cell((0, 0)).unwrap().ch, 'C');
    assert_eq!(frame.grid.role((2, 1)), Some(RoleTag::Shadow));
    assert_eq!(frame.grid.role((4, 1)), Some(RoleTag::Background));
}

#[test]
fn fade_edge_crossing_reduces_shadow_alpha_when_source_is_clipped() {
    let mut default_recipe = linear_gradient_recipe_value();
    default_recipe["scenes"][0]["width"] = serde_json::json!(6);
    default_recipe["scenes"][0]["height"] = serde_json::json!(5);
    default_recipe["sources"]["mainCard"] = source_with_message(
        "ABCD
EFGH
IJKL",
        4,
        3,
    );
    default_recipe["scenes"][0]["elements"][0]["placement"] =
        serde_json::json!({ "x": -2, "y": 0 });
    default_recipe["scenes"][0]["elements"][0]["surface"] = surface_shadow();

    let mut fade_recipe = default_recipe.clone();
    fade_recipe["scenes"][0]["elements"][0]["surface"]["shadow"]["edgeCrossingPolicy"] =
        serde_json::json!("fade");

    let default_alpha = visible_shadow_alpha(default_recipe, 2, 1);
    let fade_alpha = visible_shadow_alpha(fade_recipe, 2, 1);

    assert!(
        fade_alpha < default_alpha,
        "fade alpha {fade_alpha} should be below default alpha {default_alpha}"
    );
}

#[test]
fn preserve_edge_crossing_does_not_rebase_negative_placement_shadow() {
    let mut recipe = linear_gradient_recipe_value();
    recipe["scenes"][0]["width"] = serde_json::json!(6);
    recipe["scenes"][0]["height"] = serde_json::json!(5);
    recipe["sources"]["mainCard"] = source_with_message("ABCD\nEFGH\nIJKL", 4, 3);
    recipe["scenes"][0]["elements"][0]["placement"] = serde_json::json!({ "x": -2, "y": 0 });
    recipe["scenes"][0]["elements"][0]["surface"] = surface_shadow();
    recipe["scenes"][0]["elements"][0]["surface"]["shadow"]["edgeCrossingPolicy"] =
        serde_json::json!("preserve");

    let frame = render_shadow_recipe(recipe);

    assert_eq!(frame.grid.role((2, 1)), Some(RoleTag::Shadow));
    assert_eq!(frame.grid.role((4, 1)), Some(RoleTag::Background));
}

#[test]
fn soft_edges_use_textured_shadow_geometry() {
    let mut recipe = linear_gradient_recipe_value();
    recipe["scenes"][0]["width"] = serde_json::json!(6);
    recipe["scenes"][0]["height"] = serde_json::json!(5);
    recipe["sources"]["mainCard"] = source_with_message(
        "ABCD
EFGH
IJKL",
        4,
        3,
    );
    recipe["scenes"][0]["elements"][0]["surface"] = surface_shadow();

    let frame = render_shadow_recipe(recipe);

    assert_ne!(frame.grid.cell((4, 1)).unwrap().ch, ' ');
}

#[test]
fn over_shadow_mode_loads_and_tags_cells() {
    let mut recipe = linear_gradient_recipe_value();
    recipe["scenes"][0]["width"] = serde_json::json!(6);
    recipe["scenes"][0]["height"] = serde_json::json!(5);
    recipe["sources"]["mainCard"] = source_with_message(
        "ABCD
EFGH
IJKL",
        4,
        3,
    );
    recipe["scenes"][0]["elements"][0]["surface"] = surface_shadow();
    recipe["scenes"][0]["elements"][0]["surface"]["shadow"]["compositeMode"] =
        serde_json::json!("over");

    let frame = render_shadow_recipe(recipe);

    assert_eq!(frame.grid.role((4, 1)), Some(RoleTag::Shadow));
}

#[test]
fn role_scoped_shadow_source_region_loads_for_bordered_sources() {
    let mut recipe = linear_gradient_recipe_value();
    recipe["scenes"][0]["width"] = serde_json::json!(6);
    recipe["scenes"][0]["height"] = serde_json::json!(5);
    recipe["sources"]["mainCard"] = source_with_message("AB", 4, 3);
    recipe["sources"]["mainCard"]["inputs"]["borderStyle"]["value"]["value"] =
        serde_json::json!("rounded");
    recipe["scenes"][0]["elements"][0]["surface"] = surface_shadow();
    recipe["scenes"][0]["elements"][0]["surface"]["shadow"]["sourceRegion"] =
        serde_json::json!({ "kind": "role", "role": "Border" });

    let frame = render_shadow_recipe(recipe);

    assert_eq!(frame.grid.role((4, 1)), Some(RoleTag::Shadow));
}

#[test]
fn multiply_shadow_blend_loads_and_tags_cells() {
    let mut recipe = linear_gradient_recipe_value();
    recipe["scenes"][0]["elements"][0]["surface"] = surface_shadow();
    recipe["scenes"][0]["elements"][0]["surface"]["shadow"]["blendMode"] =
        serde_json::json!("multiply");

    recipe["scenes"][0]["width"] = serde_json::json!(6);
    recipe["scenes"][0]["height"] = serde_json::json!(5);
    recipe["sources"]["mainCard"] = source_with_message("ABCD\nEFGH\nIJKL", 4, 3);

    let frame = render_shadow_recipe(recipe);

    assert_eq!(frame.grid.role((4, 1)), Some(RoleTag::Shadow));
}

#[test]
fn paint_outset_expands_shadow_casting_bounds() {
    let mut recipe = linear_gradient_recipe_value();
    recipe["scenes"][0]["elements"][0]["surface"] = surface_shadow();
    recipe["scenes"][0]["elements"][0]["surface"]["shadow"]["paintOutset"] = serde_json::json!({
        "left": 0,
        "right": 1,
        "top": 0,
        "bottom": 1
    });

    recipe["scenes"][0]["width"] = serde_json::json!(6);
    recipe["scenes"][0]["height"] = serde_json::json!(5);
    recipe["sources"]["mainCard"] = source_with_message("ABCD\nEFGH\nIJKL", 4, 3);

    let frame = render_shadow_recipe(recipe);

    assert_eq!(frame.grid.role((5, 1)), Some(RoleTag::Shadow));
}

#[test]
fn wrapped_shadow_elements_cast_wrapped_cell_shadows() {
    let mut recipe = linear_gradient_recipe_value();
    recipe["scenes"][0]["elements"][0]["overflow"] = serde_json::json!("wrap");
    recipe["scenes"][0]["elements"][0]["surface"] = surface_shadow();

    recipe["scenes"][0]["width"] = serde_json::json!(4);
    recipe["scenes"][0]["height"] = serde_json::json!(3);
    recipe["sources"]["mainCard"] = source_with_message("AB", 2, 1);

    let frame = render_shadow_recipe(recipe);

    assert!(
        frame
            .trace_events
            .iter()
            .any(|event| event.stage_kind == "shadow")
    );
}

#[test]
fn surface_shadow_rejects_base_style_without_rejecting_shadow_surface() {
    let mut recipe = linear_gradient_recipe_value();
    recipe["scenes"][0]["elements"][0]["surface"] = serde_json::json!({
        "baseStyle": { "kind": "text", "value": "pending" },
        "shadow": surface_shadow()["shadow"].clone()
    });

    let catalog = primitive_catalog();
    let error = LoadedRecipe::load(recipe_from_value(recipe), &catalog)
        .expect_err("baseStyle remains unsupported until it has typed rendering semantics")
        .to_string();

    assert!(
        error.contains("mainElement.surface.baseStyle"),
        "expected specific baseStyle rejection, got: {error}"
    );
}

#[test]
fn preserve_destination_shadow_material_keeps_existing_glyphs() {
    let mut recipe = linear_gradient_recipe_value();
    recipe["scenes"][0]["width"] = serde_json::json!(6);
    recipe["scenes"][0]["height"] = serde_json::json!(5);
    recipe["sources"]["mainCard"] = source_with_message(
        "ABCD
EFGH
IJKL",
        4,
        3,
    );
    recipe["sources"]["underCard"] = source_with_message("Z", 1, 1);
    let mut casting = element_with_source("castingElement", "mainCard", 0, 0, 1);
    casting["surface"] = surface_shadow();
    casting["surface"]["shadow"]["glyphMaterial"] = serde_json::json!("preserveDestination");
    recipe["scenes"][0]["elements"] = serde_json::json!([
        element_with_source("underElement", "underCard", 4, 1, 0),
        casting
    ]);

    let frame = render_shadow_recipe(recipe);

    assert_eq!(frame.grid.cell((4, 1)).unwrap().ch, 'Z');
    assert_eq!(frame.grid.role((4, 1)), Some(RoleTag::Shadow));
}
// <FILE>crates/tui-vfx-compost/tests/direct_recipe/test_shadow_surface.rs</FILE> - <DESC>Compost scene-element surface shadow tests</DESC>
// <VERS>END OF VERSION: 0.2.0</VERS>
