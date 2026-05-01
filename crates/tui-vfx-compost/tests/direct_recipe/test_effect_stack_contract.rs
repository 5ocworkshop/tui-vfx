// <FILE>crates/tui-vfx-compost/tests/direct_recipe/test_effect_stack_contract.rs</FILE> - <DESC>Compost effect stack substrate tests</DESC>
// <VERS>VERSION: 0.1.1</VERS>
// <WCTX>Effect stack substrate tests record native family slots and reject unsupported families at load time.</WCTX>
// <CLOG>0.1.1: PATCH — use effect-family terminology in helpers and test names.
// 0.1.0: INIT — add RED coverage for effect-family diagnostics and stack order.</CLOG>

use crate::support::{linear_gradient_recipe_value, primitive_catalog, recipe_from_value};
use tui_vfx_compost::{LoadError, LoadedRecipe, SampleContext, render_recipe};

fn load_recipe_error(recipe: serde_json::Value) -> LoadError {
    let catalog = primitive_catalog();
    LoadedRecipe::load(recipe_from_value(recipe), &catalog).expect_err("recipe should fail load")
}

fn replace_gradient_node_effect(recipe: &mut serde_json::Value, effect: &str) {
    let gradient_node = &mut recipe["graph"]["nodes"]["gradient"];
    gradient_node["effect"] = serde_json::Value::String(effect.to_string());
    gradient_node["inputs"] = serde_json::json!({});
}

fn assert_unsupported_family(error: LoadError, effect: &str, family: &str) {
    assert!(
        matches!(
            &error,
            LoadError::UnsupportedEffectFamily {
                node_id,
                effect: actual_effect,
                family: actual_family,
                ..
            } if node_id == "gradient" && actual_effect == effect && actual_family == family
        ),
        "expected unsupported `{family}` family diagnostic for `{effect}`, got: {error}"
    );
}

#[test]
fn rejects_unsupported_effect_families_with_family_diagnostics() {
    for (effect, family) in [
        ("content.typewriter", "content"),
        ("style.fadeIn", "style"),
        ("filter.dim", "filter"),
        ("mask.wipe", "mask"),
        ("sampler.sineWave", "sampler"),
    ] {
        let mut recipe = linear_gradient_recipe_value();
        replace_gradient_node_effect(&mut recipe, effect);

        let error = load_recipe_error(recipe);

        assert_unsupported_family(error, effect, family);
    }
}

#[test]
fn supported_shader_stack_preserves_authored_order() {
    let catalog = primitive_catalog();
    let mut recipe = linear_gradient_recipe_value();
    let second = recipe["graph"]["nodes"]["gradient"].clone();
    recipe["graph"]["nodes"]["secondGradient"] = second;
    recipe["graph"]["nodes"]["secondGradient"]["id"] =
        serde_json::Value::String("secondGradient".to_string());
    recipe["graph"]["order"] = serde_json::json!(["gradient", "secondGradient"]);

    let loaded = LoadedRecipe::load(recipe_from_value(recipe), &catalog).expect("load recipe");
    let frame = render_recipe(&loaded, &SampleContext::default()).expect("render recipe");

    assert_eq!(
        frame.applied_effect_kinds,
        vec!["shader.linearGradient", "shader.linearGradient"]
    );
}

// <FILE>crates/tui-vfx-compost/tests/direct_recipe/test_effect_stack_contract.rs</FILE> - <DESC>Compost effect stack substrate tests</DESC>
// <VERS>END OF VERSION: 0.1.1</VERS>
