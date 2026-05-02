// <FILE>crates/tui-vfx-compost/tests/direct_recipe/test_render_observability.rs</FILE> - <DESC>Compost render observability tests</DESC>
// <VERS>VERSION: 0.4.0</VERS>
// <WCTX>Frame observability tests record native trace events and skipped-work diagnostics.</WCTX>
// <CLOG>0.4.0: MINOR — cover disjoint synthetic trace identities for nested graph and shadow stages.
// 0.3.0: MINOR — cover partial scope and topology stage identity trace truthfulness.
// 0.2.0: MINOR — cover lifecycle skip, shadow, parallel merge, and zero-scope observability.
// 0.1.0: INIT — add RED coverage for trace event identity and clipping diagnostics.</CLOG>

use crate::support::{linear_gradient_recipe_value, primitive_catalog, recipe_from_value};
use tui_vfx_compost::{LoadedRecipe, SampleContext, render_recipe};
use tui_vfx_contract::{DescriptorCatalog, DescriptorPackId, EffectId, LifecyclePhase, ScopeKind};
use tui_vfx_types::Color;

fn source_with_message(message: &str, width: i64, height: i64) -> serde_json::Value {
    let mut source = linear_gradient_recipe_value()["sources"]["mainCard"].clone();
    source["inputs"]["message"]["value"]["value"] = serde_json::Value::String(message.to_string());
    source["inputs"]["width"]["value"]["value"] = serde_json::Value::Number(width.into());
    source["inputs"]["height"]["value"]["value"] = serde_json::Value::Number(height.into());
    source
}

fn clone_gradient_node(recipe: &serde_json::Value, id: &str) -> serde_json::Value {
    let mut node = recipe["graph"]["nodes"]["gradient"].clone();
    node["id"] = serde_json::Value::String(id.to_string());
    node
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

fn render_recipe_value(recipe: serde_json::Value) -> tui_vfx_compost::Frame {
    render_recipe_value_at(recipe, &SampleContext::default())
}

fn render_recipe_value_at(
    recipe: serde_json::Value,
    sample: &SampleContext,
) -> tui_vfx_compost::Frame {
    let catalog = primitive_catalog();
    render_recipe_value_with_catalog_at(recipe, sample, &catalog)
}

fn render_recipe_value_with_catalog_at(
    recipe: serde_json::Value,
    sample: &SampleContext,
    catalog: &DescriptorCatalog,
) -> tui_vfx_compost::Frame {
    let loaded = LoadedRecipe::load(recipe_from_value(recipe), catalog).expect("load recipe");
    render_recipe(&loaded, sample).expect("render recipe")
}

fn primitive_catalog_with_column_range_scope() -> DescriptorCatalog {
    let mut catalog = primitive_catalog();
    let pack = catalog
        .packs
        .get_mut(&DescriptorPackId::new("v3.1.primitive"))
        .expect("primitive descriptor pack");
    let effect = pack
        .effects
        .get_mut(&EffectId::new("shader.linearGradient"))
        .expect("linear gradient descriptor");
    if !effect.scope_support.kinds.contains(&ScopeKind::ColumnRange) {
        effect.scope_support.kinds.push(ScopeKind::ColumnRange);
    }
    catalog
}

#[test]
fn trace_events_identify_scene_element_stage_and_effect() {
    let frame = render_recipe_value(linear_gradient_recipe_value());

    assert_eq!(frame.trace_events.len(), 1);
    let event = &frame.trace_events[0];
    assert_eq!(event.scene_id, "mainScene");
    assert_eq!(event.element_id, "mainElement");
    assert_eq!(event.stage_index, 0);
    assert_eq!(event.effect, "shader.linearGradient");
}

#[test]
fn diagnostics_explain_fully_clipped_elements() {
    let mut recipe = linear_gradient_recipe_value();
    recipe["scenes"][0]["elements"][0]["placement"] = serde_json::json!({ "x": 100, "y": 100 });

    let frame = render_recipe_value(recipe);

    assert!(frame.applied_effect_kinds.is_empty());
    assert!(
        frame
            .diagnostics
            .iter()
            .any(|diagnostic| diagnostic.message.contains("fully clipped"))
    );
}

#[test]
fn trace_events_explain_invisible_element_skip() {
    let mut recipe = linear_gradient_recipe_value();
    recipe["scenes"][0]["elements"][0]["visibility"] =
        serde_json::json!({ "kind": "phase", "phases": ["enter"] });

    let hidden = render_recipe_value(recipe.clone());
    let visible = render_recipe_value_at(
        recipe,
        &SampleContext::default().with_lifecycle_phase(LifecyclePhase::Enter),
    );

    assert!(hidden.applied_effect_kinds.is_empty());
    assert!(hidden.trace_events.iter().any(|event| {
        event.scene_id == "mainScene"
            && event.element_id == "mainElement"
            && event.stage_kind == "element"
            && event.status == "skipped"
            && event.skip_reason.as_deref() == Some("visibility")
    }));
    assert!(
        visible
            .trace_events
            .iter()
            .any(|event| event.effect == "shader.linearGradient" && event.status == "finished")
    );
}

#[test]
fn trace_events_identify_shadow_stage() {
    let mut recipe = linear_gradient_recipe_value();
    recipe["scenes"][0]["width"] = serde_json::json!(6);
    recipe["scenes"][0]["height"] = serde_json::json!(5);
    recipe["sources"]["mainCard"] = source_with_message("ABCD\nEFGH\nIJKL", 4, 3);
    recipe["scenes"][0]["elements"][0]["surface"] = surface_shadow();

    let frame = render_recipe_value(recipe);

    assert!(frame.trace_events.iter().any(|event| {
        event.scene_id == "mainScene"
            && event.element_id == "mainElement"
            && event.stage_kind == "shadow"
            && event.effect == "surface.shadow"
            && event.status == "finished"
            && event.cells_matched > 0
    }));
}

#[test]
fn trace_events_identify_parallel_branch_merge() {
    let mut recipe = linear_gradient_recipe_value();
    recipe["graph"]["topology"] = serde_json::json!({
        "kind": "parallel",
        "children": [
            { "kind": "node", "node": "gradient" }
        ],
        "mergePolicy": "childOrderLastWriterWins",
        "valueMergePolicy": "childOrderLastWriterWins"
    });

    let frame = render_recipe_value(recipe);

    assert!(frame.trace_events.iter().any(|event| {
        event.stage_kind == "parallel"
            && event.effect == "graph.parallel"
            && event.status == "finished"
            && event.cells_matched == 3
    }));
}

#[test]
fn trace_events_assign_distinct_nested_parallel_stage_indices() {
    let mut recipe = linear_gradient_recipe_value();
    let nested_node = clone_gradient_node(&recipe, "nestedGradient");
    recipe["graph"]["nodes"]["nestedGradient"] = nested_node;
    recipe["graph"]["order"] = serde_json::json!(["gradient", "nestedGradient"]);
    recipe["graph"]["topology"] = serde_json::json!({
        "kind": "parallel",
        "children": [
            { "kind": "node", "node": "gradient" },
            {
                "kind": "parallel",
                "children": [
                    { "kind": "node", "node": "nestedGradient" }
                ],
                "mergePolicy": "childOrderLastWriterWins",
                "valueMergePolicy": "childOrderLastWriterWins"
            }
        ],
        "mergePolicy": "childOrderLastWriterWins",
        "valueMergePolicy": "childOrderLastWriterWins"
    });

    let frame = render_recipe_value(recipe);

    let parallel_stage_indices: Vec<_> = frame
        .trace_events
        .iter()
        .filter(|event| event.stage_kind == "parallel")
        .map(|event| event.stage_index)
        .collect();
    assert_eq!(parallel_stage_indices, vec![2, 3]);
}

#[test]
fn trace_events_keep_shadow_indices_after_graph_synthetic_indices() {
    let mut recipe = linear_gradient_recipe_value();
    recipe["scenes"][0]["width"] = serde_json::json!(6);
    recipe["scenes"][0]["height"] = serde_json::json!(5);
    recipe["sources"]["mainCard"] = source_with_message("ABCD\nEFGH\nIJKL", 4, 3);
    recipe["scenes"][0]["elements"][0]["surface"] = surface_shadow();
    recipe["graph"]["topology"] = serde_json::json!({
        "kind": "parallel",
        "children": [
            { "kind": "node", "node": "gradient" }
        ],
        "mergePolicy": "childOrderLastWriterWins",
        "valueMergePolicy": "childOrderLastWriterWins"
    });

    let frame = render_recipe_value(recipe);

    let parallel_index = frame
        .trace_events
        .iter()
        .find(|event| event.stage_kind == "parallel")
        .expect("parallel trace event")
        .stage_index;
    let shadow_index = frame
        .trace_events
        .iter()
        .find(|event| event.stage_kind == "shadow")
        .expect("shadow trace event")
        .stage_index;
    assert_eq!(parallel_index, 1);
    assert_eq!(shadow_index, 2);
}

#[test]
fn trace_events_explain_zero_cell_scope_skip() {
    let mut recipe = linear_gradient_recipe_value();
    recipe["graph"]["nodes"]["gradient"]["scope"] =
        serde_json::json!({ "kind": "role", "role": "Border" });

    let frame = render_recipe_value(recipe);

    assert_eq!(frame.grid.cell((0, 0)).unwrap().fg, Color::WHITE);
    assert!(frame.applied_effect_kinds.is_empty());
    assert!(frame.trace_events.iter().any(|event| {
        event.stage_kind == "shader"
            && event.effect == "shader.linearGradient"
            && event.status == "skipped"
            && event.skip_reason.as_deref() == Some("scopeMatchedZeroCells")
            && event.cells_matched == 0
            && event.cells_skipped == 3
    }));
}

#[test]
fn trace_events_report_partial_scope_as_finished_with_skipped_cells() {
    let mut recipe = linear_gradient_recipe_value();
    recipe["graph"]["nodes"]["gradient"]["scope"] =
        serde_json::json!({ "kind": "columnRange", "start": 0, "end": 2 });

    let catalog = primitive_catalog_with_column_range_scope();
    let frame = render_recipe_value_with_catalog_at(recipe, &SampleContext::default(), &catalog);

    let shader_events: Vec<_> = frame
        .trace_events
        .iter()
        .filter(|event| event.effect == "shader.linearGradient")
        .collect();
    assert_eq!(shader_events.len(), 1);
    assert_eq!(shader_events[0].status, "finished");
    assert_eq!(shader_events[0].cells_matched, 2);
    assert_eq!(shader_events[0].cells_skipped, 1);
    assert!(shader_events[0].skip_reason.is_none());
}

#[test]
fn wrapped_trace_events_keep_source_local_scope_coordinates() {
    let mut recipe = linear_gradient_recipe_value();
    recipe["scenes"][0]["elements"][0]["overflow"] = serde_json::json!("wrap");
    recipe["graph"]["nodes"]["gradient"]["scope"] =
        serde_json::json!({ "kind": "columnRange", "start": 0, "end": 2 });

    let catalog = primitive_catalog_with_column_range_scope();
    let frame = render_recipe_value_with_catalog_at(recipe, &SampleContext::default(), &catalog);

    let shader_event = frame
        .trace_events
        .iter()
        .find(|event| event.effect == "shader.linearGradient")
        .expect("shader trace event");
    assert_eq!(shader_event.status, "finished");
    assert_eq!(shader_event.cells_matched, 2);
    assert_eq!(shader_event.cells_skipped, 1);
    assert_eq!(frame.grid.cell((2, 0)).unwrap().fg, Color::WHITE);
}

#[test]
fn trace_events_preserve_topology_node_stage_indices() {
    let mut recipe = linear_gradient_recipe_value();
    let default_node = clone_gradient_node(&recipe, "defaultGradient");
    recipe["graph"]["nodes"]["defaultGradient"] = default_node;
    recipe["graph"]["order"] = serde_json::json!(["gradient", "defaultGradient"]);
    recipe["graph"]["topology"] = serde_json::json!({
        "kind": "sequence",
        "children": [
            { "kind": "node", "node": "gradient" },
            { "kind": "node", "node": "defaultGradient" }
        ]
    });

    let frame = render_recipe_value(recipe);

    let stage_indices: Vec<_> = frame
        .trace_events
        .iter()
        .filter(|event| event.effect == "shader.linearGradient" && event.status == "finished")
        .map(|event| event.stage_index)
        .collect();
    assert_eq!(stage_indices, vec![0, 1]);
}

// <FILE>crates/tui-vfx-compost/tests/direct_recipe/test_render_observability.rs</FILE> - <DESC>Compost render observability tests</DESC>
// <VERS>END OF VERSION: 0.4.0</VERS>
