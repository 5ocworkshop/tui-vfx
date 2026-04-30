// <FILE>crates/tui-vfx-player/src/fnc_legacy_migration_mapping_names.rs</FILE> - <DESC>Normalize legacy migration mapping names</DESC>
// <VERS>VERSION: 0.1.0</VERS>
// <WCTX>K2.10 corpus mapping: keep legacy descriptor/source naming normalization focused.</WCTX>
// <CLOG>0.1.0: INIT — add legacy name normalization helpers.</CLOG>

/// Build a primitive descriptor id from legacy kind/type evidence.
pub(crate) fn legacy_descriptor_id(kind: &str, effect_type: &str) -> Option<String> {
    let effect = match (kind, effect_type) {
        ("sampler", "sine_wave") => "sampler.sineWave".to_string(),
        ("shader", "linear_gradient" | "gradient_overlay") => "shader.linearGradient".to_string(),
        ("shader", "border_sweep") => "shader.borderSweep".to_string(),
        ("style_effect", "color_fade") => "style.colorFade".to_string(),
        ("filter", value) => format!("filter.{}", lower_camel(value)),
        ("mask", value) => format!("mask.{}", lower_camel(value)),
        ("sampler", value) => format!("sampler.{}", lower_camel(value)),
        ("shader", value) => format!("shader.{}", lower_camel(value)),
        ("style_effect", value) => format!("style.{}", lower_camel(value)),
        _ => return None,
    };
    Some(effect)
}

/// Build a content descriptor candidate from legacy content effect evidence.
pub(crate) fn content_descriptor_id_for_content_effect(effect_type: &str) -> String {
    format!("content.{}", lower_camel(effect_type))
}

/// Normalize legacy snake_case fields to v3.1-style lower camel case.
pub(crate) fn canonical_legacy_field(field: &str) -> String {
    lower_camel(field)
}

/// Convert snake_case legacy names to lower camel case.
pub(crate) fn lower_camel(value: &str) -> String {
    let mut parts = value.split('_');
    let Some(first) = parts.next() else {
        return String::new();
    };
    let mut output = first.to_string();
    for part in parts {
        let mut chars = part.chars();
        if let Some(first_char) = chars.next() {
            output.extend(first_char.to_uppercase());
            output.push_str(chars.as_str());
        }
    }
    output
}

// <FILE>crates/tui-vfx-player/src/fnc_legacy_migration_mapping_names.rs</FILE> - <DESC>Normalize legacy migration mapping names</DESC>
// <VERS>END OF VERSION: 0.1.0</VERS>
