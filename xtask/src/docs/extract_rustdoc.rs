// <FILE>xtask/src/docs/extract_rustdoc.rs</FILE> - <DESC>Extract effect metadata from runtime introspection</DESC>
// <VERS>VERSION: 2.0.4</VERS>
// <WCTX>Populate parameter docs with schema type information</WCTX>
// <CLOG>Traverse nested schema fields for shader/style parameters</CLOG>

use super::effect_metadata::{AllEffectMetadata, extract_all_metadata};
use anyhow::Result;
use std::collections::HashMap;
use tui_vfx_compositor::types::{FilterSpec, MaskSpec, SamplerSpec};
use tui_vfx_content::types::ContentEffect;
use tui_vfx_core::{SchemaField, SchemaNode, SchemaVariant};
use tui_vfx_shadow::ShadowStyle;
use tui_vfx_style::models::{SpatialShaderType, StyleEffect};

/// Extracted documentation from effect metadata.
#[derive(Debug, Default)]
pub struct RustdocData {
    /// Effects by category, then by variant name.
    /// e.g., effects["masks"]["Wipe"] = EffectDoc { ... }
    pub effects: HashMap<String, HashMap<String, EffectDoc>>,
}

/// Documentation for a single effect variant.
#[derive(Debug, Clone)]
#[allow(dead_code)] // Fields used for future API compatibility
pub struct EffectDoc {
    /// The variant/struct name (e.g., "Wipe", "RigidShake")
    pub name: String,

    /// Summary line (first line of doc comment)
    pub summary: String,

    /// Full description (all doc comment text)
    pub description: String,

    /// Parameters with their documentation
    pub parameters: Vec<ParameterDoc>,
}

/// Documentation for a single parameter/field.
#[derive(Debug, Clone)]
pub struct ParameterDoc {
    /// Parameter name
    pub name: String,

    /// Rust type as string
    pub ty: String,

    /// Documentation for this parameter
    pub doc: String,

    /// Default value if known
    pub default: Option<String>,
}

/// Extract effect metadata using runtime introspection.
///
/// Uses the `terse_description()` and `key_parameters()` methods
/// on effect types to gather metadata without parsing rustdoc JSON.
///
/// Source crates:
/// - tui-vfx-compositor (masks, filters, samplers)
/// - tui-vfx-style (shaders, style effects)
/// - tui-vfx-content (content transformers)
/// - tui-vfx-shadow (shadow types)
pub fn extract() -> Result<RustdocData> {
    let all_metadata = extract_all_metadata();
    Ok(convert_metadata(all_metadata))
}

/// Convert AllEffectMetadata to RustdocData format.
fn convert_metadata(metadata: AllEffectMetadata) -> RustdocData {
    let mut effects = HashMap::new();

    effects.insert(
        "masks".to_string(),
        convert_category("masks", Some(MaskSpec::schema()), &metadata.masks),
    );
    effects.insert(
        "filters".to_string(),
        convert_category("filters", Some(FilterSpec::schema()), &metadata.filters),
    );
    effects.insert(
        "samplers".to_string(),
        convert_category("samplers", Some(SamplerSpec::schema()), &metadata.samplers),
    );
    effects.insert(
        "shaders".to_string(),
        convert_category(
            "shaders",
            Some(SpatialShaderType::schema()),
            &metadata.shaders,
        ),
    );
    effects.insert(
        "styles".to_string(),
        convert_category("styles", Some(StyleEffect::schema()), &metadata.styles),
    );
    effects.insert(
        "content".to_string(),
        convert_category("content", Some(ContentEffect::schema()), &metadata.content),
    );
    effects.insert(
        "shadows".to_string(),
        convert_category("shadows", Some(ShadowStyle::schema()), &metadata.shadows),
    );

    RustdocData { effects }
}

fn convert_category(
    _category_name: &str,
    schema: Option<SchemaNode>,
    category: &HashMap<String, super::effect_metadata::EffectMetadata>,
) -> HashMap<String, EffectDoc> {
    category
        .iter()
        .map(|(name, meta)| {
            let schema_ref = schema.as_ref();
            let doc = EffectDoc {
                name: meta.name.clone(),
                summary: meta.description.clone(),
                description: meta.description.clone(),
                parameters: meta
                    .parameters
                    .iter()
                    .map(|(param_name, param_value)| {
                        let (ty, doc) = schema_ref
                            .and_then(|schema| {
                                find_schema_field(schema, name, param_name).map(|field| {
                                    (schema_node_type(field.schema.as_ref()), field_doc(field))
                                })
                            })
                            .unwrap_or_default();
                        ParameterDoc {
                            name: param_name.clone(),
                            ty,
                            doc,
                            default: Some(param_value.clone()),
                        }
                    })
                    .collect(),
            };
            (name.clone(), doc)
        })
        .collect()
}

fn find_schema_field<'a>(
    schema: &'a SchemaNode,
    variant_name: &str,
    param_name: &str,
) -> Option<&'a SchemaField> {
    let fields = match schema {
        SchemaNode::Enum { variants, .. } => variants
            .iter()
            .find(|variant| variant_matches(variant, variant_name))
            .or_else(|| {
                variants
                    .iter()
                    .find(|variant| variant_json_value_matches(variant, variant_name))
            })
            .and_then(|variant| match variant {
                SchemaVariant::Unit { .. } => None,
                SchemaVariant::Tuple { items, .. } => Some(items.as_slice()),
                SchemaVariant::Struct { fields, .. } => Some(fields.as_slice()),
            }),
        SchemaNode::Struct { fields, .. } => Some(fields.as_slice()),
        _ => None,
    }?;

    find_field_in_fields(fields, param_name)
}

fn variant_matches(variant: &SchemaVariant, variant_name: &str) -> bool {
    match variant {
        SchemaVariant::Unit { name, .. } => name == variant_name,
        SchemaVariant::Tuple { name, .. } => name == variant_name,
        SchemaVariant::Struct { name, .. } => name == variant_name,
    }
}

fn variant_json_value_matches(variant: &SchemaVariant, variant_name: &str) -> bool {
    match variant {
        SchemaVariant::Unit { json_value, .. } => json_value.as_deref() == Some(variant_name),
        SchemaVariant::Tuple { json_value, .. } => json_value.as_deref() == Some(variant_name),
        SchemaVariant::Struct { json_value, .. } => json_value.as_deref() == Some(variant_name),
    }
}

fn find_field_in_fields<'a>(
    fields: &'a [SchemaField],
    param_name: &str,
) -> Option<&'a SchemaField> {
    if let Some(field) = fields.iter().find(|field| {
        field.name == param_name
            || field
                .json_key
                .as_deref()
                .map(|key| key == param_name)
                .unwrap_or(false)
    }) {
        return Some(field);
    }

    for field in fields {
        if let Some(found) = find_schema_field_in_node(field.schema.as_ref(), param_name) {
            return Some(found);
        }
    }
    None
}

fn find_schema_field_in_node<'a>(
    schema: &'a SchemaNode,
    param_name: &str,
) -> Option<&'a SchemaField> {
    match schema {
        SchemaNode::Struct { fields, .. } => find_field_in_fields(fields, param_name),
        SchemaNode::Enum { variants, .. } => {
            for variant in variants {
                let fields = match variant {
                    SchemaVariant::Unit { .. } => None,
                    SchemaVariant::Tuple { items, .. } => Some(items.as_slice()),
                    SchemaVariant::Struct { fields, .. } => Some(fields.as_slice()),
                };
                if let Some(fields) = fields {
                    if let Some(found) = find_field_in_fields(fields, param_name) {
                        return Some(found);
                    }
                }
            }
            None
        }
        SchemaNode::Option { inner }
        | SchemaNode::Vec { item: inner }
        | SchemaNode::Box { inner } => find_schema_field_in_node(inner, param_name),
        SchemaNode::Primitive { .. } | SchemaNode::Opaque { .. } => None,
    }
}

fn schema_node_type(node: &SchemaNode) -> String {
    match node {
        SchemaNode::Primitive { type_name, .. } => type_name.clone(),
        SchemaNode::Option { inner } => format!("Option<{}>", schema_node_type(inner)),
        SchemaNode::Vec { item } => format!("Vec<{}>", schema_node_type(item)),
        SchemaNode::Box { inner } => format!("Box<{}>", schema_node_type(inner)),
        SchemaNode::Struct { name, .. } => name.clone(),
        SchemaNode::Enum { name, .. } => name.clone(),
        SchemaNode::Opaque { type_name } => type_name.clone(),
    }
}

fn field_doc(field: &SchemaField) -> String {
    field
        .meta
        .help
        .clone()
        .or_else(|| field.meta.description.clone())
        .unwrap_or_default()
}

#[cfg(test)]
mod tests {
    use super::extract;

    #[test]
    fn filter_parameters_include_type() {
        let data = extract().expect("expected extraction to succeed");
        let filters = data.effects.get("filters").expect("missing filters");
        let glisten = filters.get("GlistenSweep").expect("missing GlistenSweep");
        let boost = glisten
            .parameters
            .iter()
            .find(|p| p.name == "boost")
            .expect("missing boost parameter");
        assert!(!boost.ty.is_empty(), "expected boost type to be populated");
    }

    #[test]
    fn shader_parameters_include_type_and_doc() {
        let data = extract().expect("expected extraction to succeed");
        let shaders = data.effects.get("shaders").expect("missing shaders");
        let ao = shaders
            .get("AmbientOcclusion")
            .expect("missing AmbientOcclusion");
        let intensity = ao
            .parameters
            .iter()
            .find(|p| p.name == "intensity")
            .expect("missing intensity parameter");
        assert!(
            !intensity.ty.is_empty(),
            "expected intensity type to be populated"
        );
        assert!(
            !intensity.doc.is_empty(),
            "expected intensity doc to be populated"
        );
    }

    #[test]
    fn shadow_parameters_include_type_and_doc() {
        let data = extract().expect("expected extraction to succeed");
        let shadows = data.effects.get("shadows").expect("missing shadows");
        let braille = shadows.get("Braille").expect("missing Braille");
        let density = braille
            .parameters
            .iter()
            .find(|p| p.name == "density")
            .expect("missing density parameter");
        assert!(
            !density.ty.is_empty(),
            "expected density type to be populated"
        );
        assert!(
            !density.doc.is_empty(),
            "expected density doc to be populated"
        );
    }

    #[test]
    fn spatial_style_parameters_include_type() {
        let data = extract().expect("expected extraction to succeed");
        let styles = data.effects.get("styles").expect("missing styles");
        let spatial = styles.get("Spatial").expect("missing Spatial");
        let angle = spatial
            .parameters
            .iter()
            .find(|p| p.name == "angle_deg")
            .expect("missing angle_deg parameter");
        assert!(!angle.ty.is_empty(), "expected angle type to be populated");
    }
}

// <FILE>xtask/src/docs/extract_rustdoc.rs</FILE> - <DESC>Extract effect metadata from runtime introspection</DESC>
// <VERS>END OF VERSION: 2.0.4</VERS>
