// <FILE>xtask/src/docs/gen_effect_schemas.rs</FILE> - <DESC>Generate effect_schemas.json from ConfigSchema</DESC>
// <VERS>VERSION: 0.1.1</VERS>
// <WCTX>Export full schema metadata for recipe validation</WCTX>
// <CLOG>Add test coverage for schema export</CLOG>

use super::merge::MergedManifest;
use anyhow::{Context, Result};
use serde_json::{Map, Value};
use std::collections::BTreeMap;
use std::fs;
use std::path::Path;
use tui_vfx_compositor::types::{FilterSpec, MaskSpec, SamplerSpec};
use tui_vfx_content::types::ContentEffect;
use tui_vfx_core::{FieldMeta, Range, ScalarValue, SchemaField, SchemaNode, SchemaVariant};
use tui_vfx_shadow::ShadowStyle;
use tui_vfx_style::models::{SpatialShaderType, StyleEffect};

const OUTPUT_PATH: &str = "docs/generated/effect_schemas.json";

pub fn generate(manifest: &MergedManifest) -> Result<()> {
    write_schemas(manifest, Path::new(OUTPUT_PATH))
}

/// Render effect_schemas.json content without writing to disk.
pub fn render(manifest: &MergedManifest) -> Result<String> {
    let mut root = Map::new();
    root.insert(
        "version".to_string(),
        Value::String(manifest.version.clone()),
    );

    let mut categories = Map::new();
    categories.insert("masks".to_string(), category_schema(MaskSpec::schema()));
    categories.insert("filters".to_string(), category_schema(FilterSpec::schema()));
    categories.insert(
        "samplers".to_string(),
        category_schema(SamplerSpec::schema()),
    );
    categories.insert(
        "shaders".to_string(),
        category_schema(SpatialShaderType::schema()),
    );
    categories.insert("styles".to_string(), category_schema(StyleEffect::schema()));
    categories.insert(
        "content".to_string(),
        category_schema(ContentEffect::schema()),
    );
    categories.insert(
        "shadows".to_string(),
        category_schema(ShadowStyle::schema()),
    );

    root.insert("categories".to_string(), Value::Object(categories));

    serde_json::to_string_pretty(&Value::Object(root))
        .context("Failed to serialize effect_schemas.json")
}

fn write_schemas(manifest: &MergedManifest, path: &Path) -> Result<()> {
    let output = render(manifest)?;

    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)
            .with_context(|| format!("Failed to create {}", parent.display()))?;
    }

    fs::write(path, output).with_context(|| format!("Failed to write {}", path.display()))?;

    Ok(())
}

fn category_schema(schema: SchemaNode) -> Value {
    let enum_schema = schema_node_to_value(&schema);
    let mut variants_map: BTreeMap<String, Value> = BTreeMap::new();

    if let SchemaNode::Enum { variants, .. } = schema {
        for variant in variants {
            if let Some((key, value)) = schema_variant_entry(&variant) {
                variants_map.insert(key, value);
            }
        }
    }

    let mut category = Map::new();
    category.insert("enum_schema".to_string(), enum_schema);
    category.insert(
        "variants".to_string(),
        Value::Object(variants_map.into_iter().collect()),
    );
    Value::Object(category)
}

fn schema_variant_entry(variant: &SchemaVariant) -> Option<(String, Value)> {
    let (name, json_value) = match variant {
        SchemaVariant::Unit {
            name, json_value, ..
        } => (name.as_str(), json_value.as_deref()),
        SchemaVariant::Tuple {
            name, json_value, ..
        } => (name.as_str(), json_value.as_deref()),
        SchemaVariant::Struct {
            name, json_value, ..
        } => (name.as_str(), json_value.as_deref()),
    };

    let key = json_value.unwrap_or(name).to_string();
    Some((key, schema_variant_to_value(variant)))
}

fn schema_variant_to_value(variant: &SchemaVariant) -> Value {
    match variant {
        SchemaVariant::Unit {
            name,
            description,
            json_value,
        } => {
            serde_json::json!({
                "kind": "unit",
                "name": name,
                "description": description,
                "json_value": json_value,
            })
        }
        SchemaVariant::Tuple {
            name,
            description,
            json_value,
            items,
        } => {
            let items_json: Vec<Value> = items.iter().map(schema_field_to_value).collect();
            serde_json::json!({
                "kind": "tuple",
                "name": name,
                "description": description,
                "json_value": json_value,
                "items": items_json,
            })
        }
        SchemaVariant::Struct {
            name,
            description,
            json_value,
            fields,
        } => {
            let fields_json: Vec<Value> = fields.iter().map(schema_field_to_value).collect();
            serde_json::json!({
                "kind": "struct",
                "name": name,
                "description": description,
                "json_value": json_value,
                "fields": fields_json,
            })
        }
    }
}

fn schema_field_to_value(field: &SchemaField) -> Value {
    serde_json::json!({
        "name": field.name,
        "json_key": field.json_key,
        "meta": field_meta_to_value(&field.meta),
        "schema": schema_node_to_value(&field.schema),
    })
}

fn field_meta_to_value(meta: &FieldMeta) -> Value {
    serde_json::json!({
        "help": meta.help,
        "description": meta.description,
        "default": meta.default.as_ref().map(scalar_to_value),
        "range": meta.range.as_ref().map(range_to_value),
        "json_key": meta.json_key,
        "optional": meta.optional,
    })
}

fn range_to_value(range: &Range) -> Value {
    serde_json::json!({
        "min": range.min.as_ref().map(scalar_to_value),
        "max": range.max.as_ref().map(scalar_to_value),
    })
}

fn scalar_to_value(value: &ScalarValue) -> Value {
    match value {
        ScalarValue::Bool(b) => Value::Bool(*b),
        ScalarValue::Number(n) => {
            if let Ok(i) = n.parse::<i64>() {
                Value::Number(i.into())
            } else if let Ok(u) = n.parse::<u64>() {
                Value::Number(u.into())
            } else if let Ok(f) = n.parse::<f64>() {
                serde_json::Number::from_f64(f)
                    .map(Value::Number)
                    .unwrap_or_else(|| Value::String(n.clone()))
            } else {
                Value::String(n.clone())
            }
        }
        ScalarValue::String(s) => Value::String(s.clone()),
        ScalarValue::Char(c) => Value::String(c.to_string()),
    }
}

fn schema_node_to_value(node: &SchemaNode) -> Value {
    match node {
        SchemaNode::Struct {
            name,
            description,
            json_name,
            fields,
        } => {
            let fields_json: Vec<Value> = fields.iter().map(schema_field_to_value).collect();
            serde_json::json!({
                "kind": "struct",
                "name": name,
                "description": description,
                "json_name": json_name,
                "fields": fields_json,
            })
        }
        SchemaNode::Enum {
            name,
            description,
            json_name,
            tag_field,
            variants,
        } => {
            let variants_json: Vec<Value> = variants.iter().map(schema_variant_to_value).collect();
            serde_json::json!({
                "kind": "enum",
                "name": name,
                "description": description,
                "json_name": json_name,
                "tag_field": tag_field,
                "variants": variants_json,
            })
        }
        SchemaNode::Primitive { type_name, range } => {
            serde_json::json!({
                "kind": "primitive",
                "type": type_name,
                "range": range.as_ref().map(range_to_value),
            })
        }
        SchemaNode::Option { inner } => {
            serde_json::json!({
                "kind": "option",
                "inner": schema_node_to_value(inner),
            })
        }
        SchemaNode::Vec { item } => {
            serde_json::json!({
                "kind": "vec",
                "item": schema_node_to_value(item),
            })
        }
        SchemaNode::Box { inner } => {
            serde_json::json!({
                "kind": "box",
                "inner": schema_node_to_value(inner),
            })
        }
        SchemaNode::Opaque { type_name } => {
            serde_json::json!({
                "kind": "opaque",
                "type": type_name,
            })
        }
    }
}

#[cfg(test)]
mod tests {
    use super::write_schemas;
    use crate::docs::merge::{MergedEffects, MergedManifest};
    use crate::docs::parse_toml::{ConstraintsSection, SemanticsSection};
    use std::collections::HashMap;
    use std::env;
    use std::fs;
    use std::time::{SystemTime, UNIX_EPOCH};

    #[test]
    fn writes_effect_schemas_json() {
        let manifest = MergedManifest {
            version: "0.0.0".to_string(),
            layers: HashMap::new(),
            phases: HashMap::new(),
            effects: MergedEffects::default(),
            semantics: SemanticsSection::default(),
            recipes: HashMap::new(),
            constraints: ConstraintsSection::default(),
        };

        let stamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("time went backwards")
            .as_nanos();
        let mut path = env::temp_dir();
        path.push(format!("tui-vfx-effect-schemas-{}.json", stamp));

        let result = write_schemas(&manifest, &path);
        assert!(result.is_ok(), "expected schema generation to succeed");

        let content = fs::read_to_string(&path).expect("expected schema file to exist");
        let json: serde_json::Value = serde_json::from_str(&content).expect("expected valid JSON");
        assert!(json.get("version").is_some());
        assert!(json.get("categories").is_some());

        let _ = fs::remove_file(&path);
    }
}

// <FILE>xtask/src/docs/gen_effect_schemas.rs</FILE> - <DESC>Generate effect_schemas.json from ConfigSchema</DESC>
// <VERS>END OF VERSION: 0.1.1</VERS>
