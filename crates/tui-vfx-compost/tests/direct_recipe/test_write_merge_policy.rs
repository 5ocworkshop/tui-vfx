// <FILE>crates/tui-vfx-compost/tests/direct_recipe/test_write_merge_policy.rs</FILE> - <DESC>Compost write and merge policy substrate tests</DESC>
// <VERS>VERSION: 0.4.0</VERS>
// <WCTX>Write substrate tests cover final-cell skip behavior, role writes, node policies, graph value publication, and parallel merge semantics.</WCTX>
// <CLOG>0.4.0: MINOR — cover native graph execution, graph values, parallel merge, and writeChannels masking.
// 0.3.1: PATCH — name unsupported-policy tests with present capability language.
// 0.3.0: MINOR — cover copied source roles and explicit role writes.
// 0.2.0: MINOR — cover final-cell skip policy and node-local policy rejection.</CLOG>

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

fn graph_value_source(id: &str, fallback: f64) -> serde_json::Value {
    serde_json::json!({
        "kind": "graphValue",
        "id": id,
        "fallback": { "kind": "number", "value": fallback }
    })
}

fn output_from_input(input_id: &str) -> serde_json::Value {
    serde_json::json!({
        "outputSource": { "kind": "input", "id": input_id }
    })
}

fn output_from_effect_output(output_id: &str) -> serde_json::Value {
    serde_json::json!({
        "outputSource": { "kind": "effectOutput", "id": output_id }
    })
}

fn literal_number(value: f64) -> serde_json::Value {
    serde_json::json!({ "kind": "literal", "value": { "kind": "number", "value": value } })
}

fn white_gradient_literal() -> serde_json::Value {
    serde_json::json!({
        "kind": "literal",
        "value": {
            "kind": "gradient",
            "value": {
                "space": "rgb",
                "stops": [
                    { "position": 0.0, "color": { "r": 255, "g": 255, "b": 255, "a": 255 } },
                    { "position": 1.0, "color": { "r": 255, "g": 255, "b": 255, "a": 255 } }
                ]
            }
        }
    })
}

fn target_channel_parameter() -> serde_json::Value {
    serde_json::json!({
        "id": "targetChannel",
        "displayName": "Target Channel",
        "description": null,
        "value": {
            "kind": "enum",
            "default": { "kind": "enum", "value": "background" },
            "range": null,
            "allowedValues": ["foreground", "background"],
            "unit": null,
            "semantic": null
        },
        "bindable": true
    })
}

fn target_channel_parameter_source() -> serde_json::Value {
    serde_json::json!({
        "kind": "parameter",
        "id": "targetChannel",
        "fallback": { "kind": "enum", "value": "background" }
    })
}

fn clone_gradient_node(recipe: &serde_json::Value, id: &str) -> serde_json::Value {
    let mut node = recipe["graph"]["nodes"]["gradient"].clone();
    node["id"] = serde_json::Value::String(id.to_string());
    node
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
fn copy_sampled_source_role_writes_generated_text_role() {
    let mut recipe = linear_gradient_recipe_value();
    recipe["scenes"][0]["elements"][0]["roleWritePolicy"] =
        serde_json::json!({ "kind": "copySampledSource" });

    let frame = render_recipe_value(recipe);

    assert_eq!(frame.grid.cell((0, 0)).unwrap().ch, 'A');
    assert_eq!(frame.grid.role((0, 0)), Some(RoleTag::Text));
}

#[test]
fn set_explicit_role_writes_requested_role() {
    let mut recipe = linear_gradient_recipe_value();
    let explicit_role = serde_json::to_value(RoleTag::Highlight).expect("serialize role");
    recipe["scenes"][0]["elements"][0]["roleWritePolicy"] =
        serde_json::json!({ "kind": "setExplicit", "role": explicit_role });

    let frame = render_recipe_value(recipe);

    assert_eq!(frame.grid.cell((0, 0)).unwrap().ch, 'A');
    assert_eq!(frame.grid.role((0, 0)), Some(RoleTag::Highlight));
}

#[test]
fn parallel_child_order_merges_supported_shader_branches() {
    let mut recipe = linear_gradient_recipe_value();
    let mut background_node = clone_gradient_node(&recipe, "backgroundGradient");
    background_node["inputs"]["channelTarget"] = serde_json::json!({ "kind": "literal", "value": { "kind": "enum", "value": "background" } });
    recipe["graph"]["nodes"]["backgroundGradient"] = background_node;
    recipe["graph"]["order"] = serde_json::json!(["gradient", "backgroundGradient"]);
    recipe["graph"]["topology"] = serde_json::json!({
        "kind": "parallel",
        "children": [
            { "kind": "node", "node": "gradient" },
            { "kind": "node", "node": "backgroundGradient" }
        ],
        "mergePolicy": "childOrderLastWriterWins",
        "valueMergePolicy": "childOrderLastWriterWins"
    });

    let frame = render_recipe_value(recipe);
    let cell = frame.grid.cell((0, 0)).unwrap();

    assert_eq!(cell.ch, 'A');
    assert_eq!(cell.fg, Color::RED);
    assert_eq!(cell.bg, Color::RED);
}

#[test]
fn parallel_noop_branch_does_not_restore_earlier_channel_delta() {
    let mut recipe = linear_gradient_recipe_value();
    let mut noop_foreground = clone_gradient_node(&recipe, "noopForeground");
    noop_foreground["inputs"]["intensity"] = literal_number(0.0);
    recipe["graph"]["nodes"]["noopForeground"] = noop_foreground;
    recipe["graph"]["order"] = serde_json::json!(["gradient", "noopForeground"]);
    recipe["graph"]["topology"] = serde_json::json!({
        "kind": "parallel",
        "children": [
            { "kind": "node", "node": "gradient" },
            { "kind": "node", "node": "noopForeground" }
        ],
        "mergePolicy": "childOrderLastWriterWins",
        "valueMergePolicy": "childOrderLastWriterWins"
    });

    let frame = render_recipe_value(recipe);

    assert_eq!(frame.grid.cell((0, 0)).unwrap().fg, Color::RED);
}

#[test]
fn parallel_nested_sequence_uses_final_branch_channel_delta() {
    let mut recipe = linear_gradient_recipe_value();
    let mut temporary_red = clone_gradient_node(&recipe, "temporaryRed");
    temporary_red["inputs"]["intensity"] = literal_number(1.0);
    let mut restore_white = clone_gradient_node(&recipe, "restoreWhite");
    restore_white["inputs"]["gradient"] = white_gradient_literal();
    recipe["graph"]["nodes"]["temporaryRed"] = temporary_red;
    recipe["graph"]["nodes"]["restoreWhite"] = restore_white;
    recipe["graph"]["order"] = serde_json::json!(["gradient", "temporaryRed", "restoreWhite"]);
    recipe["graph"]["topology"] = serde_json::json!({
        "kind": "parallel",
        "children": [
            { "kind": "node", "node": "gradient" },
            {
                "kind": "sequence",
                "children": [
                    { "kind": "node", "node": "temporaryRed" },
                    { "kind": "node", "node": "restoreWhite" }
                ]
            }
        ],
        "mergePolicy": "childOrderLastWriterWins",
        "valueMergePolicy": "childOrderLastWriterWins"
    });

    let frame = render_recipe_value(recipe);

    assert_eq!(frame.grid.cell((0, 0)).unwrap().fg, Color::RED);
}

#[test]
fn parallel_same_channel_conflict_rejects_when_requested() {
    let mut recipe = linear_gradient_recipe_value();
    let foreground_node = clone_gradient_node(&recipe, "foregroundGradient");
    recipe["graph"]["nodes"]["foregroundGradient"] = foreground_node;
    recipe["graph"]["order"] = serde_json::json!(["gradient", "foregroundGradient"]);
    recipe["graph"]["topology"] = serde_json::json!({
        "kind": "parallel",
        "children": [
            { "kind": "node", "node": "gradient" },
            { "kind": "node", "node": "foregroundGradient" }
        ],
        "mergePolicy": "errorOnSameChannelConflict",
        "valueMergePolicy": "childOrderLastWriterWins"
    });

    let error = load_recipe_error(recipe);

    assert!(matches!(
        error,
        LoadError::UnsupportedGraphMergePolicy { field, reason }
            if field == "graph.topology.mergePolicy"
                && reason.contains("branches write the same channel")
    ));
}

#[test]
fn explicit_write_channels_limit_node_style_output() {
    let mut recipe = linear_gradient_recipe_value();
    recipe["graph"]["nodes"]["gradient"]["inputs"]["channelTarget"] =
        serde_json::json!({ "kind": "literal", "value": { "kind": "enum", "value": "both" } });
    recipe["graph"]["nodes"]["gradient"]["writeChannels"] = serde_json::json!(["foreground"]);

    let frame = render_recipe_value(recipe);
    let cell = frame.grid.cell((0, 0)).unwrap();

    assert_eq!(cell.fg, Color::RED);
    assert_eq!(cell.bg, Color::BLACK);
}

#[test]
fn dynamic_channel_target_executes_without_explicit_write_filter() {
    let mut recipe = linear_gradient_recipe_value();
    recipe["graph"]["parameters"]["targetChannel"] = target_channel_parameter();
    recipe["graph"]["nodes"]["gradient"]["inputs"]["channelTarget"] =
        target_channel_parameter_source();

    let frame = render_recipe_value(recipe);
    let cell = frame.grid.cell((0, 0)).unwrap();

    assert_eq!(cell.fg, Color::WHITE);
    assert_eq!(cell.bg, Color::RED);
}

#[test]
fn sequence_graph_value_output_feeds_later_node_input() {
    let mut recipe = linear_gradient_recipe_value();
    let mut consumer = clone_gradient_node(&recipe, "consumerGradient");
    consumer["inputs"]["intensity"] = graph_value_source("sharedIntensity", 1.0);
    recipe["graph"]["nodes"]["gradient"]["inputs"]["intensity"] =
        serde_json::json!({ "kind": "literal", "value": { "kind": "number", "value": 0.0 } });
    recipe["graph"]["nodes"]["gradient"]["outputs"] =
        serde_json::json!({ "sharedIntensity": output_from_input("intensity") });
    recipe["graph"]["nodes"]["consumerGradient"] = consumer;
    recipe["graph"]["order"] = serde_json::json!(["gradient", "consumerGradient"]);
    recipe["graph"]["topology"] = serde_json::json!({
        "kind": "sequence",
        "children": [
            { "kind": "node", "node": "gradient" },
            { "kind": "node", "node": "consumerGradient" }
        ]
    });

    let frame = render_recipe_value(recipe);

    assert_eq!(frame.grid.cell((0, 0)).unwrap().fg, Color::WHITE);
}

#[test]
fn topology_reachable_missing_output_input_rejects_at_load_time() {
    let mut recipe = linear_gradient_recipe_value();
    let mut topology_only = clone_gradient_node(&recipe, "topologyOnly");
    topology_only["outputs"] = serde_json::json!({
        "sharedIntensity": output_from_input("missingIntensity")
    });
    recipe["graph"]["nodes"]["topologyOnly"] = topology_only;
    recipe["graph"]["topology"] = serde_json::json!({
        "kind": "node",
        "node": "topologyOnly"
    });

    let error = load_recipe_error(recipe);

    assert!(
        matches!(
            error,
            LoadError::UnsupportedInput {
                ref node_id,
                ref input,
                ref reason,
                ..
            } if node_id == "topologyOnly"
                && input == "missingIntensity"
                && reason.contains("existing node input")
        ) || matches!(
            error,
            LoadError::Contract { ref message }
                if message.contains("UnknownNodeOutputInput")
                    && message.contains("missingIntensity")
        ),
        "expected load-time missing output input rejection, got {error:?}"
    );
}

#[test]
fn topology_reachable_effect_output_publication_rejects_at_load_time() {
    let mut recipe = linear_gradient_recipe_value();
    let mut topology_only = clone_gradient_node(&recipe, "topologyOnly");
    topology_only["outputs"] = serde_json::json!({
        "sharedIntensity": output_from_effect_output("computedIntensity")
    });
    recipe["graph"]["nodes"]["topologyOnly"] = topology_only;
    recipe["graph"]["topology"] = serde_json::json!({
        "kind": "node",
        "node": "topologyOnly"
    });

    let error = load_recipe_error(recipe);

    assert!(
        matches!(
            error,
            LoadError::UnsupportedInput {
                ref node_id,
                ref input,
                ref reason,
                ..
            } if node_id == "topologyOnly"
                && input == "computedIntensity"
                && reason.contains("effect output capture")
        ) || matches!(
            error,
            LoadError::Contract { ref message }
                if message.contains("UnknownEffectOutput")
                    && message.contains("computedIntensity")
        ),
        "expected load-time effect-output publication rejection, got {error:?}"
    );
}

#[test]
fn parallel_branches_do_not_read_sibling_graph_value_outputs() {
    let mut recipe = linear_gradient_recipe_value();
    let mut publisher = clone_gradient_node(&recipe, "publisherGradient");
    publisher["inputs"]["intensity"] =
        serde_json::json!({ "kind": "literal", "value": { "kind": "number", "value": 0.0 } });
    publisher["outputs"] = serde_json::json!({ "sharedIntensity": output_from_input("intensity") });
    let mut sibling_consumer = clone_gradient_node(&recipe, "siblingConsumer");
    sibling_consumer["inputs"]["intensity"] = graph_value_source("sharedIntensity", 1.0);
    recipe["graph"]["nodes"] = serde_json::json!({
        "publisherGradient": publisher,
        "siblingConsumer": sibling_consumer
    });
    recipe["graph"]["order"] = serde_json::json!(["publisherGradient", "siblingConsumer"]);
    recipe["graph"]["topology"] = serde_json::json!({
        "kind": "parallel",
        "children": [
            { "kind": "node", "node": "publisherGradient" },
            { "kind": "node", "node": "siblingConsumer" }
        ],
        "mergePolicy": "childOrderLastWriterWins",
        "valueMergePolicy": "childOrderLastWriterWins"
    });

    let frame = render_recipe_value(recipe);

    assert_eq!(frame.grid.cell((0, 0)).unwrap().fg, Color::RED);
}

#[test]
fn parallel_graph_value_child_order_last_writer_wins() {
    let mut recipe = linear_gradient_recipe_value();
    let mut publisher_a = clone_gradient_node(&recipe, "publisherA");
    publisher_a["inputs"]["intensity"] =
        serde_json::json!({ "kind": "literal", "value": { "kind": "number", "value": 0.0 } });
    publisher_a["outputs"] =
        serde_json::json!({ "sharedIntensity": output_from_input("intensity") });
    let mut publisher_b = clone_gradient_node(&recipe, "publisherB");
    publisher_b["inputs"]["intensity"] =
        serde_json::json!({ "kind": "literal", "value": { "kind": "number", "value": 1.0 } });
    publisher_b["outputs"] =
        serde_json::json!({ "sharedIntensity": output_from_input("intensity") });
    let mut consumer = clone_gradient_node(&recipe, "consumerGradient");
    consumer["inputs"]["intensity"] = graph_value_source("sharedIntensity", 0.0);
    recipe["graph"]["nodes"] = serde_json::json!({
        "publisherA": publisher_a,
        "publisherB": publisher_b,
        "consumerGradient": consumer
    });
    recipe["graph"]["order"] = serde_json::json!(["publisherA", "publisherB", "consumerGradient"]);
    recipe["graph"]["topology"] = serde_json::json!({
        "kind": "sequence",
        "children": [
            {
                "kind": "parallel",
                "children": [
                    { "kind": "node", "node": "publisherA" },
                    { "kind": "node", "node": "publisherB" }
                ],
                "mergePolicy": "childOrderLastWriterWins",
                "valueMergePolicy": "childOrderLastWriterWins"
            },
            { "kind": "node", "node": "consumerGradient" }
        ]
    });

    let frame = render_recipe_value(recipe);

    assert_eq!(frame.grid.cell((0, 0)).unwrap().fg, Color::RED);
}

#[test]
fn parallel_graph_value_publication_does_not_restore_inherited_value() {
    let mut recipe = linear_gradient_recipe_value();
    let mut initializer = clone_gradient_node(&recipe, "initializer");
    initializer["inputs"]["intensity"] = literal_number(0.0);
    initializer["outputs"] =
        serde_json::json!({ "sharedIntensity": output_from_input("intensity") });
    let mut publisher = clone_gradient_node(&recipe, "publisher");
    publisher["inputs"]["intensity"] = literal_number(1.0);
    publisher["outputs"] = serde_json::json!({ "sharedIntensity": output_from_input("intensity") });
    let silent_branch = clone_gradient_node(&recipe, "silentBranch");
    let mut consumer = clone_gradient_node(&recipe, "consumerGradient");
    consumer["inputs"]["intensity"] = graph_value_source("sharedIntensity", 0.0);
    consumer["inputs"]["channelTarget"] = serde_json::json!({ "kind": "literal", "value": { "kind": "enum", "value": "background" } });
    recipe["graph"]["nodes"] = serde_json::json!({
        "initializer": initializer,
        "publisher": publisher,
        "silentBranch": silent_branch,
        "consumerGradient": consumer
    });
    recipe["graph"]["order"] = serde_json::json!([
        "initializer",
        "publisher",
        "silentBranch",
        "consumerGradient"
    ]);
    recipe["graph"]["topology"] = serde_json::json!({
        "kind": "sequence",
        "children": [
            { "kind": "node", "node": "initializer" },
            {
                "kind": "parallel",
                "children": [
                    { "kind": "node", "node": "publisher" },
                    { "kind": "node", "node": "silentBranch" }
                ],
                "mergePolicy": "childOrderLastWriterWins",
                "valueMergePolicy": "childOrderLastWriterWins"
            },
            { "kind": "node", "node": "consumerGradient" }
        ]
    });

    let frame = render_recipe_value(recipe);

    assert_eq!(frame.grid.cell((0, 0)).unwrap().bg, Color::RED);
}

#[test]
fn parallel_graph_value_conflict_rejects_when_requested() {
    let mut recipe = linear_gradient_recipe_value();
    let mut publisher_a = clone_gradient_node(&recipe, "publisherA");
    publisher_a["outputs"] =
        serde_json::json!({ "sharedIntensity": output_from_input("intensity") });
    let mut publisher_b = clone_gradient_node(&recipe, "publisherB");
    publisher_b["outputs"] =
        serde_json::json!({ "sharedIntensity": output_from_input("intensity") });
    recipe["graph"]["nodes"] = serde_json::json!({
        "publisherA": publisher_a,
        "publisherB": publisher_b
    });
    recipe["graph"]["order"] = serde_json::json!(["publisherA", "publisherB"]);
    recipe["graph"]["topology"] = serde_json::json!({
        "kind": "parallel",
        "children": [
            { "kind": "node", "node": "publisherA" },
            { "kind": "node", "node": "publisherB" }
        ],
        "mergePolicy": "childOrderLastWriterWins",
        "valueMergePolicy": "errorOnSameValueConflict"
    });

    let error = load_recipe_error(recipe);

    assert!(
        matches!(
            error,
            LoadError::UnsupportedGraphMergePolicy {
                ref field,
                ref reason,
            }
                if field == "graph.topology.valueMergePolicy"
                    && reason.contains("branches publish the same graph value")
        ),
        "expected value merge conflict, got {error:?}"
    );
}

#[test]
fn parallel_dynamic_channel_target_rejects_before_surface_merge() {
    let mut recipe = linear_gradient_recipe_value();
    let dynamic_foreground = clone_gradient_node(&recipe, "dynamicForeground");
    let mut background_node = clone_gradient_node(&recipe, "backgroundGradient");
    background_node["inputs"]["channelTarget"] = target_channel_parameter_source();
    recipe["graph"]["parameters"]["targetChannel"] = target_channel_parameter();
    recipe["graph"]["nodes"] = serde_json::json!({
        "dynamicForeground": dynamic_foreground,
        "backgroundGradient": background_node
    });
    recipe["graph"]["order"] = serde_json::json!(["dynamicForeground", "backgroundGradient"]);
    recipe["graph"]["topology"] = serde_json::json!({
        "kind": "parallel",
        "children": [
            { "kind": "node", "node": "dynamicForeground" },
            { "kind": "node", "node": "backgroundGradient" }
        ],
        "mergePolicy": "childOrderLastWriterWins",
        "valueMergePolicy": "childOrderLastWriterWins"
    });

    let error = load_recipe_error(recipe);

    assert!(
        matches!(
            error,
            LoadError::UnsupportedGraphMergePolicy {
                ref field,
                ref reason,
            }
                if field == "graph.topology.mergePolicy"
                    && reason.contains("literal shader channel targets")
        ),
        "expected dynamic channel target rejection, got {error:?}"
    );
}

#[test]
fn rejects_unsupported_node_local_write_policy_precedence() {
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
// <VERS>END OF VERSION: 0.4.0</VERS>
