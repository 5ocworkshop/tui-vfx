// <FILE>crates/tui-vfx-player/src/fnc_build_control_catalog_report.rs</FILE> - <DESC>Build player control catalog reports</DESC>
// <VERS>VERSION: 0.1.0</VERS>
// <WCTX>Control catalog CLI: derive studio controls from descriptors and optional recipe usage.</WCTX>
// <CLOG>0.1.0: INIT — build descriptor and recipe-aware control catalog reports.</CLOG>

use std::path::Path;

use tui_vfx_contract::{
    DescriptorCatalog, EffectInputSpec, RecipeDocument, SourceInputSpec, ValueKind, ValueSource,
    ValueSpec,
};

use crate::{
    DescriptorPackReport, PlayerControlCatalogControl, PlayerControlCatalogReport,
    PlayerControlCatalogSummary,
};

/// Build a descriptor-derived player control catalog with optional recipe usage annotations.
pub fn build_control_catalog_report(
    descriptor_packs: Vec<DescriptorPackReport>,
    catalog: &DescriptorCatalog,
    recipe_path: Option<&Path>,
) -> Result<PlayerControlCatalogReport, String> {
    let recipe = recipe_path.map(read_recipe).transpose()?;
    let recipe_label = recipe_path.map(|path| path.display().to_string());
    let mut controls = Vec::new();
    for pack in catalog.packs.values() {
        for (source_id, descriptor) in &pack.source_descriptors {
            for (input_name, input) in &descriptor.inputs {
                controls.push(source_control(
                    source_id.as_str(),
                    input_name.as_str(),
                    input,
                    recipe.as_ref(),
                ));
            }
        }
        for (effect_id, descriptor) in &pack.effects {
            for (input_name, input) in &descriptor.inputs {
                controls.push(effect_control(
                    effect_id.as_str(),
                    input_name.as_str(),
                    input,
                    recipe.as_ref(),
                ));
            }
        }
    }
    controls.sort_by(|left, right| left.id.cmp(&right.id));
    let summary = summarize_controls(&controls);
    Ok(PlayerControlCatalogReport {
        schema_version: "v3.1.player.controlCatalog.1",
        descriptor_packs,
        recipe: recipe_label,
        summary,
        controls,
        warnings: Vec::new(),
    })
}

fn read_recipe(path: &Path) -> Result<RecipeDocument, String> {
    let text = std::fs::read_to_string(path)
        .map_err(|error| format!("read `{}` failed: {error}", path.display()))?;
    serde_json::from_str(&text)
        .map_err(|error| format!("parse `{}` failed: {error}", path.display()))
}

fn source_control(
    descriptor_id: &str,
    input_name: &str,
    input: &SourceInputSpec,
    recipe: Option<&RecipeDocument>,
) -> PlayerControlCatalogControl {
    let (used_by, current_value) = source_usage(descriptor_id, input_name, recipe);
    base_control(
        format!("source:{descriptor_id}:{input_name}"),
        "sourceInput",
        descriptor_id,
        None,
        input_name,
        &input.value,
        input.display_name.as_deref(),
        input.description.clone(),
        input.bindable,
        input.optional,
        input.runtime_mutability,
        used_by,
        current_value,
    )
}

fn effect_control(
    descriptor_id: &str,
    input_name: &str,
    input: &EffectInputSpec,
    recipe: Option<&RecipeDocument>,
) -> PlayerControlCatalogControl {
    let (used_by, node_id, current_value) = effect_usage(descriptor_id, input_name, recipe);
    base_control(
        recipe_control_id("effect", descriptor_id, input_name, node_id.as_deref()),
        "descriptorInput",
        descriptor_id,
        node_id,
        input_name,
        &input.value,
        input.display_name.as_deref(),
        input.description.clone(),
        input.bindable,
        input.optional,
        input.runtime_mutability,
        used_by,
        current_value,
    )
}

#[allow(clippy::too_many_arguments)]
fn base_control(
    id: String,
    source_kind: &'static str,
    descriptor_id: &str,
    node_id: Option<String>,
    input_name: &str,
    value: &ValueSpec,
    display_name: Option<&str>,
    documentation: Option<String>,
    bindable: bool,
    optional: bool,
    runtime_mutability: tui_vfx_contract::RuntimeMutability,
    used_by: Vec<String>,
    current_value: Option<serde_json::Value>,
) -> PlayerControlCatalogControl {
    PlayerControlCatalogControl {
        id,
        label: display_name.unwrap_or(input_name).to_string(),
        source_kind,
        descriptor_id: descriptor_id.to_string(),
        node_id,
        input_name: input_name.to_string(),
        value_kind: value_kind_label(value.kind).to_string(),
        control_kind: control_kind(value),
        range: value.range,
        allowed_values: value.allowed_values.clone(),
        unit: value.unit.clone(),
        semantic: value.semantic.clone(),
        runtime_mutability,
        bindable,
        optional,
        default_value: value.default.as_ref().map(value_to_json),
        current_value,
        used_by,
        documentation,
    }
}

fn source_usage(
    descriptor_id: &str,
    input_name: &str,
    recipe: Option<&RecipeDocument>,
) -> (Vec<String>, Option<serde_json::Value>) {
    let Some(recipe) = recipe else {
        return (Vec::new(), None);
    };
    let mut used_by = Vec::new();
    let mut current_value = None;
    for (instance_id, source) in &recipe.sources {
        if source.source.as_str() == descriptor_id {
            let label = format!("source:{}", instance_id.as_str());
            if let Some((_, value)) = source
                .inputs
                .iter()
                .find(|(id, _)| id.as_str() == input_name)
            {
                current_value = Some(value_source_to_json(value));
            }
            used_by.push(label);
        }
    }
    (used_by, current_value)
}

fn effect_usage(
    descriptor_id: &str,
    input_name: &str,
    recipe: Option<&RecipeDocument>,
) -> (Vec<String>, Option<String>, Option<serde_json::Value>) {
    let Some(recipe) = recipe else {
        return (Vec::new(), None, None);
    };
    let mut used_by = Vec::new();
    let mut first_node = None;
    let mut current_value = None;
    for (node_id, node) in &recipe.graph.nodes {
        if node.effect.as_str() == descriptor_id {
            let node_label = node_id.as_str().to_string();
            if first_node.is_none() {
                first_node = Some(node_label.clone());
            }
            if let Some((_, value)) = node.inputs.iter().find(|(id, _)| id.as_str() == input_name) {
                current_value = Some(value_source_to_json(value));
            }
            used_by.push(format!("node:{node_label}"));
        }
    }
    (used_by, first_node, current_value)
}

fn recipe_control_id(
    kind: &str,
    descriptor_id: &str,
    input_name: &str,
    node_id: Option<&str>,
) -> String {
    match node_id {
        Some(node_id) => format!("{kind}:{descriptor_id}:{node_id}:{input_name}"),
        None => format!("{kind}:{descriptor_id}:{input_name}"),
    }
}

fn control_kind(value: &ValueSpec) -> &'static str {
    match value.kind {
        ValueKind::Integer | ValueKind::Number | ValueKind::Duration if value.range.is_some() => {
            "slider"
        }
        ValueKind::Integer | ValueKind::Number => "numericInput",
        ValueKind::Duration => "durationInput",
        ValueKind::Boolean => "toggle",
        ValueKind::Enum => "select",
        ValueKind::Color => "colorPicker",
        ValueKind::Gradient => "gradientEditor",
        ValueKind::Structured => "structuredJsonEditor",
        ValueKind::Text | ValueKind::String => "textInput",
        _ => "valueInput",
    }
}

fn value_kind_label(kind: ValueKind) -> &'static str {
    match kind {
        ValueKind::Null => "null",
        ValueKind::Boolean => "boolean",
        ValueKind::Integer => "integer",
        ValueKind::Number => "number",
        ValueKind::String => "string",
        ValueKind::Text => "text",
        ValueKind::Color => "color",
        ValueKind::Gradient => "gradient",
        ValueKind::Duration => "duration",
        ValueKind::Enum => "enum",
        ValueKind::Role => "role",
        ValueKind::Scope => "scope",
        ValueKind::Rect => "rect",
        ValueKind::Structured => "structured",
    }
}

fn value_to_json(value: &tui_vfx_contract::Value) -> serde_json::Value {
    serde_json::to_value(value).expect("contract value serializes")
}

fn value_source_to_json(value: &ValueSource) -> serde_json::Value {
    serde_json::to_value(value).expect("value source serializes")
}

fn summarize_controls(controls: &[PlayerControlCatalogControl]) -> PlayerControlCatalogSummary {
    let mut source_controls = 0;
    let mut effect_controls = 0;
    let mut recipe_used_controls = 0;
    for control in controls {
        match control.source_kind {
            "sourceInput" => source_controls += 1,
            "descriptorInput" => effect_controls += 1,
            _ => {}
        }
        if !control.used_by.is_empty() {
            recipe_used_controls += 1;
        }
    }
    PlayerControlCatalogSummary {
        controls: controls.len(),
        source_controls,
        effect_controls,
        recipe_used_controls,
    }
}

// <FILE>crates/tui-vfx-player/src/fnc_build_control_catalog_report.rs</FILE> - <DESC>Build player control catalog reports</DESC>
// <VERS>END OF VERSION: 0.1.0</VERS>
