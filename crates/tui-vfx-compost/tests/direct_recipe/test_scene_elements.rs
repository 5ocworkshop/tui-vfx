// <FILE>crates/tui-vfx-compost/tests/direct_recipe/test_scene_elements.rs</FILE> - <DESC>Compost scene element substrate tests</DESC>
// <VERS>VERSION: 0.5.0</VERS>
// <WCTX>Scene substrate tests prove multi-element composition, z ordering, signed placement clipping, and strict unsupported-policy rejection.</WCTX>
// <CLOG>0.5.0: MINOR — prove native visibility, warn clipping, hide overflow, and wrap overflow execution.
// 0.4.2: PATCH — assert stable one-scene unsupported diagnostic wording.
// 0.4.1: PATCH — use unsupported-policy language instead of schedule language in test names.
// 0.4.0: MINOR — drop stale role-policy rejection now that role writes execute natively.
// 0.3.3: PATCH — keep scene test imports rustfmt-aligned.
// 0.3.2: PATCH — keep vertical clipping fixture line-aware after source materialization.
// 0.3.1: PATCH — assert graphBinding.timing diagnostic path explicitly.
// 0.3.0: PATCH — add graph-binding timing rejection regression.
// 0.2.1: PATCH — centralize repeated scene sizing and render fixture setup.
// 0.2.0: PATCH — add unsupported scene-element policy rejection regressions.
// 0.1.0: INIT — add RED tests for canonical v3.1 scene element rendering.</CLOG>

use crate::support::{linear_gradient_recipe_value, primitive_catalog, recipe_from_value};
use tui_vfx_compost::{Frame, LoadedRecipe, SampleContext, render_recipe};
use tui_vfx_contract::LifecyclePhase;
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

fn load_error_for_element_policy(policy: &str, value: serde_json::Value) -> String {
    let catalog = primitive_catalog();
    let mut recipe = linear_gradient_recipe_value();
    recipe["scenes"][0]["elements"][0][policy] = value;

    LoadedRecipe::load(recipe_from_value(recipe), &catalog)
        .expect_err("policy should be rejected before render")
        .to_string()
}

fn set_scene_size(recipe: &mut serde_json::Value, width: i64, height: i64) {
    recipe["scenes"][0]["width"] = serde_json::Value::Number(width.into());
    recipe["scenes"][0]["height"] = serde_json::Value::Number(height.into());
}

fn render_test_recipe(recipe: serde_json::Value) -> Frame {
    render_test_recipe_at(recipe, SampleContext::default())
}

fn render_test_recipe_at(recipe: serde_json::Value, sample: SampleContext) -> Frame {
    let catalog = primitive_catalog();
    let loaded = LoadedRecipe::load(recipe_from_value(recipe), &catalog).expect("load recipe");

    render_recipe(&loaded, &sample).expect("render scene")
}

fn assert_rejects_element_policy_path(
    policy: &str,
    expected_policy_path: &str,
    value: serde_json::Value,
) {
    let error = load_error_for_element_policy(policy, value);

    assert!(
        error.contains(&format!(
            "unsupported scene element policy mainElement.{expected_policy_path}"
        )),
        "expected {expected_policy_path} rejection after setting {policy}, got: {error}"
    );
}

#[test]
fn renders_multiple_scene_elements_into_one_frame() {
    let mut recipe = linear_gradient_recipe_value();
    set_scene_size(&mut recipe, 3, 1);
    recipe["sources"]["leftCard"] = source_with_message("A", 1, 1);
    recipe["sources"]["rightCard"] = source_with_message("B", 1, 1);
    recipe["scenes"][0]["elements"] = serde_json::json!([
        element_with_source("leftElement", "leftCard", 0, 0, 0),
        element_with_source("rightElement", "rightCard", 2, 0, 0)
    ]);

    let frame = render_test_recipe(recipe);

    assert_eq!(frame.grid.cell((0, 0)).unwrap().ch, 'A');
    assert_eq!(frame.grid.cell((1, 0)).unwrap().ch, ' ');
    assert_eq!(frame.grid.cell((2, 0)).unwrap().ch, 'B');
    assert_eq!(
        frame.applied_effect_kinds,
        vec!["shader.linearGradient", "shader.linearGradient"]
    );
    assert_eq!(frame.grid.role((0, 0)), Some(RoleTag::Background));
    assert_eq!(frame.grid.role((2, 0)), Some(RoleTag::Background));
}

#[test]
fn paints_higher_z_index_after_lower_z_index() {
    let mut recipe = linear_gradient_recipe_value();
    set_scene_size(&mut recipe, 1, 1);
    recipe["sources"]["highCard"] = source_with_message("H", 1, 1);
    recipe["sources"]["lowCard"] = source_with_message("L", 1, 1);
    recipe["scenes"][0]["elements"] = serde_json::json!([
        element_with_source("highElement", "highCard", 0, 0, 10),
        element_with_source("lowElement", "lowCard", 0, 0, 0)
    ]);

    let frame = render_test_recipe(recipe);

    assert_eq!(frame.grid.cell((0, 0)).unwrap().ch, 'H');
}

#[test]
fn paints_higher_negative_z_index_after_lower_negative_z_index() {
    let mut recipe = linear_gradient_recipe_value();
    set_scene_size(&mut recipe, 1, 1);
    recipe["sources"]["highCard"] = source_with_message("H", 1, 1);
    recipe["sources"]["lowCard"] = source_with_message("L", 1, 1);
    recipe["scenes"][0]["elements"] = serde_json::json!([
        element_with_source("highElement", "highCard", 0, 0, -1),
        element_with_source("lowElement", "lowCard", 0, 0, -10)
    ]);

    let frame = render_test_recipe(recipe);

    assert_eq!(frame.grid.cell((0, 0)).unwrap().ch, 'H');
}

#[test]
fn preserves_declaration_order_for_equal_z_index_elements() {
    let mut recipe = linear_gradient_recipe_value();
    set_scene_size(&mut recipe, 1, 1);
    recipe["sources"]["firstCard"] = source_with_message("F", 1, 1);
    recipe["sources"]["secondCard"] = source_with_message("S", 1, 1);
    recipe["scenes"][0]["elements"] = serde_json::json!([
        element_with_source("firstElement", "firstCard", 0, 0, 7),
        element_with_source("secondElement", "secondCard", 0, 0, 7)
    ]);

    let frame = render_test_recipe(recipe);

    assert_eq!(frame.grid.cell((0, 0)).unwrap().ch, 'S');
}

#[test]
fn clips_negative_and_overflow_placement_without_rebasing_source_origin() {
    let mut recipe = linear_gradient_recipe_value();
    set_scene_size(&mut recipe, 2, 2);
    recipe["sources"]["negativeCard"] = source_with_message("ABC", 3, 1);
    recipe["sources"]["overflowCard"] = source_with_message("XYZ", 3, 1);
    recipe["scenes"][0]["elements"] = serde_json::json!([
        element_with_source("negativeElement", "negativeCard", -1, 0, 0),
        element_with_source("overflowElement", "overflowCard", 1, 1, 0)
    ]);

    let frame = render_test_recipe(recipe);

    assert_eq!(frame.grid.cell((0, 0)).unwrap().ch, 'B');
    assert_eq!(frame.grid.cell((1, 0)).unwrap().ch, 'C');
    assert_eq!(frame.grid.cell((0, 1)).unwrap().ch, ' ');
    assert_eq!(frame.grid.cell((1, 1)).unwrap().ch, 'X');
}

#[test]
fn clips_vertical_negative_and_bottom_overflow_placement() {
    let mut recipe = linear_gradient_recipe_value();
    set_scene_size(&mut recipe, 2, 2);
    recipe["sources"]["topClipCard"] = source_with_message("AB\nCD", 2, 2);
    recipe["sources"]["bottomClipCard"] = source_with_message("WX\nYZ", 2, 2);
    recipe["scenes"][0]["elements"] = serde_json::json!([
        element_with_source("topClipElement", "topClipCard", 0, -1, 0),
        element_with_source("bottomClipElement", "bottomClipCard", 0, 1, 1)
    ]);

    let frame = render_test_recipe(recipe);

    assert_eq!(frame.grid.cell((0, 0)).unwrap().ch, 'C');
    assert_eq!(frame.grid.cell((1, 0)).unwrap().ch, 'D');
    assert_eq!(frame.grid.cell((0, 1)).unwrap().ch, 'W');
    assert_eq!(frame.grid.cell((1, 1)).unwrap().ch, 'X');
}

#[test]
fn clipped_higher_z_element_overpaints_only_visible_cells() {
    let mut recipe = linear_gradient_recipe_value();
    set_scene_size(&mut recipe, 2, 1);
    recipe["sources"]["lowCard"] = source_with_message("LO", 2, 1);
    recipe["sources"]["highCard"] = source_with_message("XY", 2, 1);
    recipe["scenes"][0]["elements"] = serde_json::json!([
        element_with_source("lowElement", "lowCard", 0, 0, 0),
        element_with_source("highElement", "highCard", -1, 0, 1)
    ]);

    let frame = render_test_recipe(recipe);

    assert_eq!(frame.grid.cell((0, 0)).unwrap().ch, 'Y');
    assert_eq!(frame.grid.cell((1, 0)).unwrap().ch, 'O');
}

#[test]
fn fully_clipped_element_does_not_paint_or_report_applied_effect() {
    let mut recipe = linear_gradient_recipe_value();
    set_scene_size(&mut recipe, 1, 1);
    recipe["sources"]["outsideCard"] = source_with_message("X", 1, 1);
    recipe["scenes"][0]["elements"] = serde_json::json!([element_with_source(
        "outsideElement",
        "outsideCard",
        1,
        0,
        0
    )]);

    let frame = render_test_recipe(recipe);

    assert_eq!(frame.grid.cell((0, 0)).unwrap().ch, ' ');
    assert!(frame.applied_effect_kinds.is_empty());
}

#[test]
fn rejects_empty_scene_elements_with_existing_diagnostic() {
    let catalog = primitive_catalog();
    let mut recipe = linear_gradient_recipe_value();
    recipe["scenes"][0]["elements"] = serde_json::json!([]);

    let loaded = LoadedRecipe::load(recipe_from_value(recipe), &catalog).expect("load recipe");
    let err = render_recipe(&loaded, &SampleContext::default()).expect_err("render rejects scene");

    assert_eq!(
        err.to_string(),
        "unsupported render shape: recipe scene has no element"
    );
}

#[test]
fn rejects_multiple_scenes_instead_of_silently_dropping_later_scenes() {
    let catalog = primitive_catalog();
    let mut recipe = linear_gradient_recipe_value();
    recipe["scenes"] =
        serde_json::json!([recipe["scenes"][0].clone(), recipe["scenes"][0].clone()]);

    let loaded = LoadedRecipe::load(recipe_from_value(recipe), &catalog).expect("load recipe");
    let err = render_recipe(&loaded, &SampleContext::default()).expect_err("render rejects scene");

    assert_eq!(
        err.to_string(),
        "unsupported render shape: render_recipe requires exactly one scene per sample"
    );
}

#[test]
fn warn_clip_policy_renders_visible_cells_and_reports_diagnostic() {
    let mut recipe = linear_gradient_recipe_value();
    set_scene_size(&mut recipe, 1, 1);
    recipe["sources"]["mainCard"] = source_with_message("AB", 2, 1);
    recipe["scenes"][0]["elements"][0]["placement"] = serde_json::json!({ "x": -1, "y": 0 });
    recipe["scenes"][0]["elements"][0]["clipPolicy"] = serde_json::json!("warn");

    let frame = render_test_recipe(recipe);

    assert_eq!(frame.grid.cell((0, 0)).unwrap().ch, 'B');
    assert!(frame.diagnostics.iter().any(|diagnostic| {
        diagnostic.message.contains("mainElement")
            && diagnostic.message.contains("clipped")
            && diagnostic.message.contains("warn")
    }));
}

#[test]
fn hide_overflow_skips_partially_outside_element() {
    let mut recipe = linear_gradient_recipe_value();
    set_scene_size(&mut recipe, 1, 1);
    recipe["sources"]["mainCard"] = source_with_message("AB", 2, 1);
    recipe["scenes"][0]["elements"][0]["overflow"] = serde_json::json!("hide");

    let frame = render_test_recipe(recipe);

    assert_eq!(frame.grid.cell((0, 0)).unwrap().ch, ' ');
    assert!(frame.applied_effect_kinds.is_empty());
    assert!(frame.diagnostics.iter().any(|diagnostic| {
        diagnostic.message.contains("mainElement") && diagnostic.message.contains("hidden")
    }));
}

#[test]
fn wrap_overflow_wraps_cells_into_scene_bounds() {
    let mut recipe = linear_gradient_recipe_value();
    set_scene_size(&mut recipe, 2, 1);
    recipe["sources"]["mainCard"] = source_with_message("ABC", 3, 1);
    recipe["scenes"][0]["elements"][0]["placement"] = serde_json::json!({ "x": 1, "y": 0 });
    recipe["scenes"][0]["elements"][0]["overflow"] = serde_json::json!("wrap");

    let frame = render_test_recipe(recipe);

    assert_eq!(frame.grid.cell((0, 0)).unwrap().ch, 'B');
    assert_eq!(frame.grid.cell((1, 0)).unwrap().ch, 'C');
    assert_eq!(frame.applied_effect_kinds, vec!["shader.linearGradient"]);
}

#[test]
fn phase_visibility_renders_only_during_matching_lifecycle_phase() {
    let mut recipe = linear_gradient_recipe_value();
    recipe["scenes"][0]["elements"][0]["visibility"] =
        serde_json::json!({ "kind": "phase", "phases": ["enter"] });

    let hidden_frame = render_test_recipe(recipe.clone());
    let visible_frame = render_test_recipe_at(
        recipe,
        SampleContext::default().with_lifecycle_phase(LifecyclePhase::Enter),
    );

    assert_eq!(hidden_frame.grid.cell((0, 0)).unwrap().ch, ' ');
    assert!(hidden_frame.applied_effect_kinds.is_empty());
    assert_eq!(visible_frame.grid.cell((0, 0)).unwrap().ch, 'A');
    assert_eq!(
        visible_frame.applied_effect_kinds,
        vec!["shader.linearGradient"]
    );
}

#[test]
fn predicate_visibility_uses_runtime_resolver_value() {
    let mut recipe = linear_gradient_recipe_value();
    recipe["graph"]["parameters"]["showElement"] = serde_json::json!({
        "id": "showElement",
        "displayName": "Show element",
        "description": null,
        "value": {
            "kind": "boolean",
            "default": { "kind": "boolean", "value": false },
            "range": null,
            "allowedValues": [],
            "unit": null,
            "semantic": null
        },
        "bindable": true
    });
    recipe["scenes"][0]["elements"][0]["visibility"] = serde_json::json!({
        "kind": "predicate",
        "predicate_source": {
            "kind": "parameter",
            "id": "showElement",
            "fallback": { "kind": "boolean", "value": true }
        },
        "predicate": { "kind": "isTrue" }
    });

    let frame = render_test_recipe(recipe);

    assert_eq!(frame.grid.cell((0, 0)).unwrap().ch, ' ');
    assert!(frame.applied_effect_kinds.is_empty());
}

#[test]
fn empty_surface_is_preserved_as_noop() {
    let mut recipe = linear_gradient_recipe_value();
    recipe["scenes"][0]["elements"][0]["surface"] =
        serde_json::json!({ "baseStyle": null, "shadow": null });

    let frame = render_test_recipe(recipe);

    assert_eq!(frame.grid.cell((0, 0)).unwrap().ch, 'A');
}

#[test]
fn scroll_factor_is_preserved_as_noop_without_scroll_runtime_input() {
    let mut recipe = linear_gradient_recipe_value();
    recipe["scenes"][0]["elements"][0]["scrollFactor"] = serde_json::json!({ "x": 0.5, "y": 1.25 });

    let frame = render_test_recipe(recipe);

    assert_eq!(frame.grid.cell((0, 0)).unwrap().ch, 'A');
    assert_eq!(frame.applied_effect_kinds, vec!["shader.linearGradient"]);
}

#[test]
fn rejects_unsupported_graph_binding_timing_at_load_time() {
    assert_rejects_element_policy_path(
        "graphBinding",
        "graphBinding.timing",
        serde_json::json!({
            "graph": "mainGraph",
            "timing": { "enterMs": 120 },
            "topology": { "kind": "node", "node": "gradient" }
        }),
    );
}

// <FILE>crates/tui-vfx-compost/tests/direct_recipe/test_scene_elements.rs</FILE> - <DESC>Compost scene element substrate tests</DESC>
// <VERS>END OF VERSION: 0.5.0</VERS>
