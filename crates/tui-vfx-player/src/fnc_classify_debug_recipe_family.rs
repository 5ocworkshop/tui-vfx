// <FILE>crates/tui-vfx-player/src/fnc_classify_debug_recipe_family.rs</FILE> - <DESC>Classify debug recipe paths into family buckets</DESC>
// <VERS>VERSION: 0.1.0</VERS>
// <WCTX>New kernel Phase K2.1 de-slop: isolate path-based family classification.</WCTX>
// <CLOG>0.1.0: INIT — split family classification from root inventory traversal.</CLOG>

use std::path::Path;

/// Return the stable family bucket for a path relative to a debug recipe root.
pub(crate) fn classify_debug_recipe_family(relative_path: &Path) -> String {
    let components = relative_components(relative_path);
    let Some(first) = components.first().map(String::as_str) else {
        return "other".to_string();
    };
    match first {
        "baseline.json" | "_DEPRECATED_baseline.json" => "baseline".to_string(),
        "shaders" => classify_shader_family(&components),
        "filters" | "masks" | "samplers" | "styles" | "content" | "scene" | "shadows"
        | "complex" | "event_driven_dwell" | "signals" | "easings" | "subcell_shapes"
        | "motion_routes" | "loopback" | "bindable_rates" | "fixtures" => first.to_string(),
        _ => "other".to_string(),
    }
}

fn classify_shader_family(components: &[String]) -> String {
    match components.get(1).map(String::as_str) {
        Some("primitives") => "shaders/primitives".to_string(),
        Some("compositions") => "shaders/compositions".to_string(),
        _ => "shaders/primitives".to_string(),
    }
}

fn relative_components(path: &Path) -> Vec<String> {
    path.components()
        .map(|component| component.as_os_str().to_string_lossy().into_owned())
        .collect()
}

// <FILE>crates/tui-vfx-player/src/fnc_classify_debug_recipe_family.rs</FILE> - <DESC>Classify debug recipe paths into family buckets</DESC>
// <VERS>END OF VERSION: 0.1.0</VERS>
