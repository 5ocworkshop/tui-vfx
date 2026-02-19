// <FILE>xtask/src/recipes/mod.rs</FILE> - <DESC>Recipe validation against capabilities.json</DESC>
// <VERS>VERSION: 0.2.8</VERS>
// <WCTX>Recipe validation tooling</WCTX>
// <CLOG>Allow single-value tuple enums in untagged paths</CLOG>

use anyhow::{Context, Result, anyhow};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::{HashMap, HashSet};
use std::fs;
use std::path::{Path, PathBuf};

const CAPABILITIES_JSON_PATH: &str = "docs/generated/capabilities.json";
const EFFECT_SCHEMAS_JSON_PATH: &str = "docs/generated/effect_schemas.json";

#[derive(Debug, Deserialize)]
struct CapabilitiesFile {
    #[serde(default)]
    effects: HashMap<String, HashMap<String, CapEffect>>,
}

#[derive(Debug, Deserialize)]
struct CapEffect {
    #[serde(default)]
    parameters: Vec<CapParam>,
}

#[derive(Debug, Deserialize, Clone)]
struct CapParam {
    name: String,
    #[serde(default)]
    ty: String,
}

#[derive(Debug, Clone)]
struct CapabilitiesIndex {
    categories: HashMap<String, HashMap<String, CapEffectInfo>>,
}

#[derive(Debug, Clone)]
struct CapEffectInfo {
    canonical: String,
    params: HashMap<String, CapParam>,
}

#[derive(Debug, Deserialize)]
struct EffectSchemasFile {
    categories: HashMap<String, SchemaCategory>,
}

#[derive(Debug, Deserialize)]
struct SchemaCategory {
    #[serde(default)]
    variants: HashMap<String, SchemaVariantJson>,
}

#[derive(Debug, Deserialize, Clone)]
#[allow(dead_code)]
#[serde(tag = "kind", rename_all = "lowercase")]
enum SchemaNodeJson {
    Struct {
        name: String,
        description: Option<String>,
        json_name: Option<String>,
        fields: Vec<SchemaFieldJson>,
    },
    Enum {
        name: String,
        description: Option<String>,
        json_name: Option<String>,
        tag_field: Option<String>,
        variants: Vec<SchemaVariantJson>,
    },
    Primitive {
        #[serde(rename = "type")]
        type_name: String,
        range: Option<RangeJson>,
    },
    Option {
        inner: Box<SchemaNodeJson>,
    },
    Vec {
        item: Box<SchemaNodeJson>,
    },
    Box {
        inner: Box<SchemaNodeJson>,
    },
    Opaque {
        #[serde(rename = "type")]
        type_name: String,
    },
}

#[derive(Debug, Deserialize, Clone)]
#[allow(dead_code)]
#[serde(tag = "kind", rename_all = "lowercase")]
enum SchemaVariantJson {
    Unit {
        name: String,
        description: Option<String>,
        json_value: Option<String>,
    },
    Tuple {
        name: String,
        description: Option<String>,
        json_value: Option<String>,
        items: Vec<SchemaFieldJson>,
    },
    Struct {
        name: String,
        description: Option<String>,
        json_value: Option<String>,
        fields: Vec<SchemaFieldJson>,
    },
}

#[derive(Debug, Deserialize, Clone)]
#[allow(dead_code)]
struct SchemaFieldJson {
    name: String,
    json_key: Option<String>,
    meta: FieldMetaJson,
    schema: SchemaNodeJson,
}

#[derive(Debug, Deserialize, Clone)]
#[allow(dead_code)]
struct FieldMetaJson {
    help: Option<String>,
    description: Option<String>,
    default: Option<Value>,
    range: Option<RangeJson>,
    json_key: Option<String>,
    optional: bool,
}

#[derive(Debug, Deserialize, Clone)]
#[allow(dead_code)]
struct RangeJson {
    min: Option<Value>,
    max: Option<Value>,
}

#[derive(Debug, Clone)]
struct EffectSchemasIndex {
    categories: HashMap<String, HashMap<String, SchemaVariantJson>>,
}

#[derive(Debug, Clone)]
struct ValidationContext {
    capabilities: CapabilitiesIndex,
    effect_schemas: Option<EffectSchemasIndex>,
}

#[derive(Debug, Serialize)]
struct ValidationReport {
    summary: ReportSummary,
    recipes: Vec<RecipeReport>,
}

#[derive(Debug, Serialize)]
struct ReportSummary {
    total_recipes: usize,
    files_with_errors: usize,
    files_with_warnings: usize,
    error_count: usize,
    warning_count: usize,
    info_count: usize,
}

#[derive(Debug, Serialize)]
struct RecipeReport {
    path: String,
    id: Option<String>,
    title: Option<String>,
    extends: Vec<String>,
    issues: Vec<RecipeIssue>,
}

#[derive(Debug, Serialize)]
struct RecipeIssue {
    severity: IssueSeverity,
    path: String,
    message: String,
}

#[derive(Debug, Serialize, Clone, Copy, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
enum IssueSeverity {
    Error,
    Warning,
    Info,
}

pub fn validate(recipes_dir: &Path, output_dir: &Path) -> Result<()> {
    let capabilities = load_capabilities(Path::new(CAPABILITIES_JSON_PATH))?;
    let effect_schemas = load_effect_schemas(Path::new(EFFECT_SCHEMAS_JSON_PATH)).ok();
    let ctx = ValidationContext {
        capabilities,
        effect_schemas,
    };
    let recipe_paths = collect_recipe_paths(recipes_dir)?;

    let mut reports = Vec::new();
    for recipe_path in recipe_paths {
        match load_recipe_with_extends(&recipe_path, recipes_dir) {
            Ok((merged, extends)) => {
                let report = validate_recipe(&recipe_path, &merged, &extends, &ctx);
                reports.push(report);
            }
            Err(err) => {
                reports.push(report_from_error(&recipe_path, &err));
            }
        }
    }

    let summary = summarize_reports(&reports);
    let report = ValidationReport {
        summary,
        recipes: reports,
    };

    fs::create_dir_all(output_dir)
        .with_context(|| format!("Failed to create {}", output_dir.display()))?;

    let json_path = output_dir.join("recipes_validation.json");
    let md_path = output_dir.join("recipes_validation.md");

    write_report_json(&json_path, &report)?;
    write_report_markdown(&md_path, &report)?;

    Ok(())
}

fn collect_recipe_paths(recipes_dir: &Path) -> Result<Vec<PathBuf>> {
    if !recipes_dir.exists() {
        return Err(anyhow!(
            "Recipes directory not found: {}",
            recipes_dir.display()
        ));
    }

    let mut paths = Vec::new();
    for entry in walkdir::WalkDir::new(recipes_dir)
        .into_iter()
        .filter_map(Result::ok)
    {
        let path = entry.path();
        if path.is_file() && path.extension().and_then(|s| s.to_str()) == Some("json") {
            paths.push(path.to_path_buf());
        }
    }
    paths.sort();
    Ok(paths)
}

fn load_capabilities(path: &Path) -> Result<CapabilitiesIndex> {
    let content =
        fs::read_to_string(path).with_context(|| format!("Failed to read {}", path.display()))?;
    let parsed: CapabilitiesFile = serde_json::from_str(&content)
        .with_context(|| format!("Failed to parse {}", path.display()))?;

    let mut categories = HashMap::new();
    for (category, effects) in parsed.effects {
        let mut indexed = HashMap::new();
        for (name, effect) in effects {
            let normalized = normalize_effect_key(&name);
            let mut params = HashMap::new();
            for param in effect.parameters {
                params.insert(param.name.clone(), param);
            }
            indexed.insert(
                normalized,
                CapEffectInfo {
                    canonical: name.clone(),
                    params,
                },
            );
        }
        categories.insert(category, indexed);
    }

    Ok(CapabilitiesIndex { categories })
}

fn load_recipe_with_extends(path: &Path, recipes_root: &Path) -> Result<(Value, Vec<String>)> {
    let mut visited = HashSet::new();
    load_recipe_with_extends_inner(path, recipes_root, &mut visited)
}

fn load_recipe_with_extends_inner(
    path: &Path,
    recipes_root: &Path,
    visited: &mut HashSet<PathBuf>,
) -> Result<(Value, Vec<String>)> {
    if !visited.insert(path.to_path_buf()) {
        return Err(anyhow!("extends cycle detected at {}", path.display()));
    }

    let content =
        fs::read_to_string(path).with_context(|| format!("Failed to read {}", path.display()))?;
    let mut current: Value = serde_json::from_str(&content)
        .with_context(|| format!("Failed to parse {}", path.display()))?;

    let mut chain = Vec::new();
    let extends_path = current
        .get("extends")
        .and_then(Value::as_str)
        .map(|s| s.to_string());

    if let Some(rel) = extends_path {
        let base_dir = path.parent().unwrap_or(recipes_root);
        let mut parent_path = base_dir.join(&rel);
        if !parent_path.exists() {
            parent_path = recipes_root.join(&rel);
        }
        let (parent, mut parent_chain) =
            load_recipe_with_extends_inner(&parent_path, recipes_root, visited)?;
        chain.append(&mut parent_chain);
        chain.push(rel);
        current = merge_values(parent, current);
    }

    Ok((current, chain))
}

fn merge_values(parent: Value, child: Value) -> Value {
    match (parent, child) {
        (Value::Object(mut p), Value::Object(c)) => {
            for (k, v) in c {
                let merged = if let Some(existing) = p.remove(&k) {
                    merge_values(existing, v)
                } else {
                    v
                };
                p.insert(k, merged);
            }
            Value::Object(p)
        }
        (_, child) => child,
    }
}

fn validate_recipe(
    path: &Path,
    recipe: &Value,
    extends: &[String],
    ctx: &ValidationContext,
) -> RecipeReport {
    let mut issues = Vec::new();

    if !recipe.is_object() {
        issues.push(issue_error("$", "Recipe root must be a JSON object"));
        return RecipeReport {
            path: path.display().to_string(),
            id: None,
            title: None,
            extends: extends.to_vec(),
            issues,
        };
    }

    let obj = recipe.as_object().expect("checked object");
    let id = obj.get("id").and_then(Value::as_str).map(|s| s.to_string());
    let title = obj
        .get("title")
        .and_then(Value::as_str)
        .map(|s| s.to_string());

    for key in [
        "schema_version",
        "id",
        "title",
        "description",
        "version",
        "config",
    ] {
        if !obj.contains_key(key) {
            issues.push(issue_error(&format!("$.{}", key), "Missing required field"));
        }
    }

    if let Some(config) = obj.get("config") {
        validate_config(config, ctx, &mut issues);
    } else {
        issues.push(issue_error("$.config", "Missing config object"));
    }

    RecipeReport {
        path: path.display().to_string(),
        id,
        title,
        extends: extends.to_vec(),
        issues,
    }
}

fn report_from_error(path: &Path, err: &anyhow::Error) -> RecipeReport {
    RecipeReport {
        path: path.display().to_string(),
        id: None,
        title: None,
        extends: Vec::new(),
        issues: vec![issue_error(
            "$.extends",
            &format!("Failed to resolve extends: {}", err),
        )],
    }
}

fn validate_config(config: &Value, ctx: &ValidationContext, issues: &mut Vec<RecipeIssue>) {
    let Some(cfg) = config.as_object() else {
        issues.push(issue_error("$.config", "Config must be an object"));
        return;
    };

    if let Some(message) = cfg.get("message") {
        if !message.is_string() {
            issues.push(issue_error("$.config.message", "message must be a string"));
        }
    }

    if let Some(layout) = cfg.get("layout") {
        validate_object_type(layout, "$.config.layout", issues);
    }

    if let Some(lifecycle) = cfg.get("lifecycle") {
        validate_object_type(lifecycle, "$.config.lifecycle", issues);
    }

    if let Some(border) = cfg.get("border") {
        validate_object_type(border, "$.config.border", issues);
    }

    if let Some(time) = cfg.get("time") {
        validate_object_type(time, "$.config.time", issues);
    }

    if let Some(content) = cfg.get("content") {
        validate_content(content, ctx, issues);
    }

    if let Some(pipeline) = cfg.get("pipeline") {
        validate_pipeline(pipeline, ctx, issues);
    } else {
        issues.push(issue_error("$.config.pipeline", "Missing pipeline object"));
    }
}

fn validate_object_type(value: &Value, path: &str, issues: &mut Vec<RecipeIssue>) {
    if !value.is_object() {
        issues.push(issue_error(path, "Expected object"));
    }
}

fn validate_content(value: &Value, ctx: &ValidationContext, issues: &mut Vec<RecipeIssue>) {
    let Some(obj) = value.as_object() else {
        issues.push(issue_error("$.config.content", "content must be an object"));
        return;
    };

    if let Some(effect) = obj.get("effect") {
        if effect.is_null() {
            return;
        }
        validate_effect_block(effect, "content", "$.config.content.effect", ctx, issues);
    }
}

fn validate_pipeline(value: &Value, ctx: &ValidationContext, issues: &mut Vec<RecipeIssue>) {
    let Some(obj) = value.as_object() else {
        issues.push(issue_error(
            "$.config.pipeline",
            "pipeline must be an object",
        ));
        return;
    };

    for phase in ["enter", "exit"] {
        if let Some(phase_value) = obj.get(phase) {
            if !phase_value.is_object() {
                issues.push(issue_error(
                    &format!("$.config.pipeline.{}", phase),
                    "Expected object",
                ));
            }
        }
    }

    for section in ["mask", "filter", "sampler"] {
        if let Some(section_value) = obj.get(section) {
            validate_phase_section(
                section_value,
                section,
                &format!("$.config.pipeline.{}", section),
                ctx,
                issues,
            );
        }
    }

    if let Some(style) = obj.get("style") {
        validate_style_block(style, "$.config.pipeline.style", ctx, issues);
    }

    if let Some(styles) = obj.get("styles") {
        if let Some(array) = styles.as_array() {
            for (idx, item) in array.iter().enumerate() {
                let path = format!("$.config.pipeline.styles[{}]", idx);
                validate_style_block(item, &path, ctx, issues);
            }
        } else {
            issues.push(issue_error(
                "$.config.pipeline.styles",
                "styles must be an array",
            ));
        }
    }
}

fn validate_phase_section(
    value: &Value,
    category: &str,
    path: &str,
    ctx: &ValidationContext,
    issues: &mut Vec<RecipeIssue>,
) {
    let Some(section) = value.as_object() else {
        issues.push(issue_error(path, "Expected object"));
        return;
    };

    for phase in ["enter", "exit", "dwell"] {
        if let Some(phase_value) = section.get(phase) {
            let phase_path = format!("{}.{}", path, phase);
            validate_effect_container(phase_value, category, &phase_path, ctx, issues);
        }
    }
}

fn validate_style_block(
    value: &Value,
    path: &str,
    ctx: &ValidationContext,
    issues: &mut Vec<RecipeIssue>,
) {
    let Some(obj) = value.as_object() else {
        issues.push(issue_error(path, "style block must be an object"));
        return;
    };

    if let Some(region) = obj.get("region") {
        if !(region.is_string() || region.is_object()) {
            issues.push(issue_error(
                &format!("{}.region", path),
                "region must be a string or object",
            ));
        }
    }

    if let Some(base_style) = obj.get("base_style") {
        if !base_style.is_object() {
            issues.push(issue_error(
                &format!("{}.base_style", path),
                "base_style must be an object",
            ));
        }
    }

    for key in ["enter_effect", "exit_effect", "dwell_effect"] {
        if let Some(effect) = obj.get(key) {
            if effect.is_null() {
                continue;
            }
            let effect_path = format!("{}.{}", path, key);
            validate_effect_block(effect, "styles", &effect_path, ctx, issues);
        }
    }

    if let Some(shader) = obj.get("spatial_shader") {
        let shader_path = format!("{}.spatial_shader", path);
        validate_effect_block(shader, "shaders", &shader_path, ctx, issues);
    }

    if let Some(interaction_states) = obj.get("interaction_states") {
        if !interaction_states.is_array() {
            issues.push(issue_error(
                &format!("{}.interaction_states", path),
                "interaction_states must be an array",
            ));
        }
    }

    if let Some(interaction_config) = obj.get("interaction_config") {
        if !interaction_config.is_object() {
            issues.push(issue_error(
                &format!("{}.interaction_config", path),
                "interaction_config must be an object",
            ));
        }
    }
}

fn validate_effect_container(
    value: &Value,
    category: &str,
    path: &str,
    ctx: &ValidationContext,
    issues: &mut Vec<RecipeIssue>,
) {
    if value.is_array() {
        for (idx, item) in value.as_array().unwrap().iter().enumerate() {
            let item_path = format!("{}[{}]", path, idx);
            validate_effect_block(item, category, &item_path, ctx, issues);
        }
    } else {
        validate_effect_block(value, category, path, ctx, issues);
    }
}

fn validate_effect_block(
    value: &Value,
    category: &str,
    path: &str,
    ctx: &ValidationContext,
    issues: &mut Vec<RecipeIssue>,
) {
    let Some(obj) = value.as_object() else {
        issues.push(issue_error(path, "effect must be an object"));
        return;
    };

    let Some(effect_type) = obj.get("type").and_then(Value::as_str) else {
        issues.push(issue_error(
            &format!("{}.type", path),
            "Missing effect type",
        ));
        return;
    };

    let category_key = normalize_category(category);
    let schema_variant = ctx
        .effect_schemas
        .as_ref()
        .and_then(|schemas| lookup_schema_variant(schemas, category_key, effect_type));
    let effect_info = lookup_effect(&ctx.capabilities, category_key, effect_type);

    if schema_variant.is_none() && effect_info.is_none() {
        issues.push(issue_error(
            &format!("{}.type", path),
            &format!("Unknown {} effect: {}", category_key, effect_type),
        ));
        return;
    }

    if let Some(variant) = schema_variant {
        validate_effect_against_schema(obj, &variant, path, issues);
    } else if let Some(info) = effect_info.clone() {
        validate_effect_against_capabilities(obj, &info, path, issues);
    }

    if category_key == "styles" && effect_type.eq_ignore_ascii_case("spatial") {
        if let Some(shader) = obj.get("shader") {
            let shader_path = format!("{}.shader", path);
            validate_effect_block(shader, "shaders", &shader_path, ctx, issues);
        } else {
            issues.push(issue_warning(
                &format!("{}.shader", path),
                "Spatial style effect missing shader",
            ));
        }
    }
}

fn validate_effect_against_capabilities(
    obj: &serde_json::Map<String, Value>,
    effect_info: &CapEffectInfo,
    path: &str,
    issues: &mut Vec<RecipeIssue>,
) {
    for (param_name, param_value) in obj.iter() {
        if param_name == "type" {
            continue;
        }
        match effect_info.params.get(param_name) {
            Some(param) => {
                if !param.ty.is_empty() {
                    if let Some(message) = validate_param_type(&param.ty, param_value) {
                        issues.push(issue_warning(
                            &format!("{}.{}", path, param_name),
                            &format!("{} (expected {})", message, param.ty),
                        ));
                    }
                }
            }
            None => {
                issues.push(issue_info(
                    &format!("{}.{}", path, param_name),
                    &format!("Unknown parameter for {}", effect_info.canonical),
                ));
            }
        }
    }
}

fn validate_effect_against_schema(
    obj: &serde_json::Map<String, Value>,
    variant: &SchemaVariantJson,
    path: &str,
    issues: &mut Vec<RecipeIssue>,
) {
    if let SchemaVariantJson::Tuple { items, .. } = variant {
        if items.len() == 1 {
            let mut payload = obj.clone();
            payload.remove("type");
            let value = Value::Object(payload);
            validate_value_against_schema(&value, &items[0].schema, path, issues);
            return;
        }
    }

    let fields = match variant {
        SchemaVariantJson::Unit { .. } => Vec::new(),
        SchemaVariantJson::Tuple { items, .. } => items.clone(),
        SchemaVariantJson::Struct { fields, .. } => fields.clone(),
    };

    let field_map = build_field_map(&fields);

    for (param_name, param_value) in obj.iter() {
        if param_name == "type" {
            continue;
        }
        let Some(field) = field_map.get(param_name) else {
            issues.push(issue_warning(
                &format!("{}.{}", path, param_name),
                "Unknown parameter for schema variant",
            ));
            continue;
        };
        validate_value_against_schema(
            param_value,
            &field.schema,
            &format!("{}.{}", path, param_name),
            issues,
        );
    }
}

fn build_field_map(fields: &[SchemaFieldJson]) -> HashMap<String, SchemaFieldJson> {
    let mut map = HashMap::new();
    for field in fields {
        map.insert(field.name.clone(), field.clone());
        if let Some(json_key) = &field.json_key {
            map.insert(json_key.clone(), field.clone());
        }
        if let Some(json_key) = &field.meta.json_key {
            map.insert(json_key.clone(), field.clone());
        }
    }
    map
}

fn validate_value_against_schema(
    value: &Value,
    schema: &SchemaNodeJson,
    path: &str,
    issues: &mut Vec<RecipeIssue>,
) {
    match schema {
        SchemaNodeJson::Primitive { type_name, .. } => {
            if let Some(message) = validate_param_type(type_name, value) {
                issues.push(issue_warning(path, &message));
            }
        }
        SchemaNodeJson::Option { inner } => {
            if value.is_null() {
                return;
            }
            validate_value_against_schema(value, inner, path, issues);
        }
        SchemaNodeJson::Vec { item } => {
            if let Some(items) = value.as_array() {
                for (idx, entry) in items.iter().enumerate() {
                    let item_path = format!("{}[{}]", path, idx);
                    validate_value_against_schema(entry, item, &item_path, issues);
                }
            } else {
                issues.push(issue_warning(path, "Expected array"));
            }
        }
        SchemaNodeJson::Box { inner } => {
            validate_value_against_schema(value, inner, path, issues);
        }
        SchemaNodeJson::Struct { fields, .. } => {
            let Some(obj) = value.as_object() else {
                issues.push(issue_warning(path, "Expected object"));
                return;
            };
            let field_map = build_field_map(fields);
            for (key, val) in obj.iter() {
                let Some(field) = field_map.get(key) else {
                    issues.push(issue_warning(
                        &format!("{}.{}", path, key),
                        "Unknown field for struct schema",
                    ));
                    continue;
                };
                validate_value_against_schema(
                    val,
                    &field.schema,
                    &format!("{}.{}", path, key),
                    issues,
                );
            }
        }
        SchemaNodeJson::Enum {
            tag_field,
            variants,
            ..
        } => {
            validate_enum_value(value, tag_field.as_deref(), variants, path, issues);
        }
        SchemaNodeJson::Opaque { .. } => {}
    }
}

fn validate_enum_value(
    value: &Value,
    tag_field: Option<&str>,
    variants: &[SchemaVariantJson],
    path: &str,
    issues: &mut Vec<RecipeIssue>,
) {
    if let Some(s) = value.as_str() {
        if tag_field.is_none()
            && variants
                .iter()
                .any(|variant| variant_accepts_value(value, variant))
        {
            return;
        }
        if lookup_variant_by_value(variants, s).is_none() {
            issues.push(issue_warning(path, "Unknown enum variant"));
        }
        return;
    }

    let Some(obj) = value.as_object() else {
        if variants
            .iter()
            .any(|variant| variant_accepts_value(value, variant))
        {
            return;
        }
        issues.push(issue_warning(path, "Expected string or object for enum"));
        return;
    };

    if let Some(tag) = tag_field {
        if let Some(tag_value) = obj.get(tag).and_then(Value::as_str) {
            if let Some(variant) = lookup_variant_by_value(variants, tag_value) {
                validate_enum_struct_fields(obj, tag, variant, path, issues);
            } else {
                issues.push(issue_warning(path, "Unknown enum tag value"));
            }
            return;
        }
        if let Some(variant) = variants
            .iter()
            .find(|variant| variant_accepts_value(value, variant))
        {
            validate_enum_inner_value(value, variant, path, issues);
            return;
        }
        issues.push(issue_warning(path, "Missing enum tag field"));
        return;
    }

    if let Some(variant) = variants
        .iter()
        .find(|variant| variant_accepts_value(value, variant))
    {
        validate_enum_inner_value(value, variant, path, issues);
        return;
    }

    if obj.len() == 1 {
        if let Some((key, inner)) = obj.iter().next() {
            if let Some(variant) = lookup_variant_by_value(variants, key) {
                validate_enum_inner_value(inner, variant, path, issues);
                return;
            }
        }
    }

    issues.push(issue_warning(path, "Unable to interpret enum object"));
}

fn variant_accepts_value(value: &Value, variant: &SchemaVariantJson) -> bool {
    match variant {
        SchemaVariantJson::Unit {
            name, json_value, ..
        } => {
            if let Some(s) = value.as_str() {
                let normalized = normalize_effect_key(s);
                let name_match = normalize_effect_key(name) == normalized;
                let json_match = json_value
                    .as_ref()
                    .map(|val| normalize_effect_key(val) == normalized)
                    .unwrap_or(false);
                return name_match || json_match;
            }
            value.is_null()
        }
        SchemaVariantJson::Tuple { items, .. } => {
            if items.len() == 1 {
                schema_accepts_value(value, &items[0].schema)
            } else {
                value
                    .as_array()
                    .map(|arr| {
                        arr.len() == items.len()
                            && arr
                                .iter()
                                .zip(items.iter())
                                .all(|(item, field)| schema_accepts_value(item, &field.schema))
                    })
                    .unwrap_or(false)
            }
        }
        SchemaVariantJson::Struct { fields, .. } => {
            let Some(obj) = value.as_object() else {
                return false;
            };
            let field_map = build_field_map(fields);
            obj.keys().all(|key| field_map.contains_key(key))
        }
    }
}

fn enum_variant_accepts_object(
    obj: &serde_json::Map<String, Value>,
    tag_field: &str,
    variant: &SchemaVariantJson,
) -> bool {
    match variant {
        SchemaVariantJson::Unit { .. } => obj.len() == 1 && obj.contains_key(tag_field),
        SchemaVariantJson::Struct { fields, .. } => {
            let field_map = build_field_map(fields);
            obj.iter().all(|(key, val)| {
                if key == tag_field {
                    return true;
                }
                field_map
                    .get(key)
                    .map(|field| schema_accepts_value(val, &field.schema))
                    .unwrap_or(false)
            })
        }
        SchemaVariantJson::Tuple { items, .. } => {
            if items.len() == 1 {
                let mut payload = obj.clone();
                payload.remove(tag_field);
                return schema_accepts_value(&Value::Object(payload), &items[0].schema);
            }
            let field_map = build_field_map(items);
            obj.iter().all(|(key, val)| {
                if key == tag_field {
                    return true;
                }
                field_map
                    .get(key)
                    .map(|field| schema_accepts_value(val, &field.schema))
                    .unwrap_or(false)
            })
        }
    }
}

fn schema_accepts_value(value: &Value, schema: &SchemaNodeJson) -> bool {
    match schema {
        SchemaNodeJson::Primitive { type_name, .. } => {
            validate_param_type(type_name, value).is_none()
        }
        SchemaNodeJson::Option { inner } => value.is_null() || schema_accepts_value(value, inner),
        SchemaNodeJson::Vec { item } => value
            .as_array()
            .map(|items| items.iter().all(|entry| schema_accepts_value(entry, item)))
            .unwrap_or(false),
        SchemaNodeJson::Box { inner } => schema_accepts_value(value, inner),
        SchemaNodeJson::Struct { fields, .. } => {
            let Some(obj) = value.as_object() else {
                return false;
            };
            let field_map = build_field_map(fields);
            obj.iter().all(|(key, val)| {
                field_map
                    .get(key)
                    .map(|field| schema_accepts_value(val, &field.schema))
                    .unwrap_or(false)
            })
        }
        SchemaNodeJson::Enum {
            tag_field,
            variants,
            ..
        } => {
            if let Some(tag) = tag_field.as_deref() {
                if let Some(obj) = value.as_object() {
                    if let Some(tag_value) = obj.get(tag).and_then(Value::as_str) {
                        if let Some(variant) = lookup_variant_by_value(variants, tag_value) {
                            return enum_variant_accepts_object(obj, tag, variant);
                        }
                        return false;
                    }
                }
            }
            variants
                .iter()
                .any(|variant| variant_accepts_value(value, variant))
        }
        SchemaNodeJson::Opaque { .. } => true,
    }
}

fn validate_enum_struct_fields(
    obj: &serde_json::Map<String, Value>,
    tag_field: &str,
    variant: &SchemaVariantJson,
    path: &str,
    issues: &mut Vec<RecipeIssue>,
) {
    match variant {
        SchemaVariantJson::Struct { fields, .. } => {
            let field_map = build_field_map(fields);
            for (key, val) in obj.iter() {
                if key == tag_field {
                    continue;
                }
                let Some(field) = field_map.get(key) else {
                    issues.push(issue_warning(
                        &format!("{}.{}", path, key),
                        "Unknown enum field",
                    ));
                    continue;
                };
                validate_value_against_schema(
                    val,
                    &field.schema,
                    &format!("{}.{}", path, key),
                    issues,
                );
            }
        }
        SchemaVariantJson::Tuple { items, .. } => {
            if items.len() == 1 {
                let mut payload = obj.clone();
                payload.remove(tag_field);
                let value = Value::Object(payload);
                validate_value_against_schema(&value, &items[0].schema, path, issues);
                return;
            }
            let field_map = build_field_map(items);
            for (key, val) in obj.iter() {
                if key == tag_field {
                    continue;
                }
                let Some(field) = field_map.get(key) else {
                    issues.push(issue_warning(
                        &format!("{}.{}", path, key),
                        "Unknown enum field",
                    ));
                    continue;
                };
                validate_value_against_schema(
                    val,
                    &field.schema,
                    &format!("{}.{}", path, key),
                    issues,
                );
            }
        }
        SchemaVariantJson::Unit { .. } => (),
    }
}

fn validate_enum_inner_value(
    value: &Value,
    variant: &SchemaVariantJson,
    path: &str,
    issues: &mut Vec<RecipeIssue>,
) {
    match variant {
        SchemaVariantJson::Unit { .. } => {
            if !value.is_null() {
                issues.push(issue_warning(path, "Expected null for unit enum variant"));
            }
        }
        SchemaVariantJson::Struct { fields, .. } => {
            let Some(obj) = value.as_object() else {
                issues.push(issue_warning(path, "Expected object for enum variant"));
                return;
            };
            let field_map = build_field_map(fields);
            for (key, val) in obj.iter() {
                let Some(field) = field_map.get(key) else {
                    issues.push(issue_warning(
                        &format!("{}.{}", path, key),
                        "Unknown enum field",
                    ));
                    continue;
                };
                validate_value_against_schema(
                    val,
                    &field.schema,
                    &format!("{}.{}", path, key),
                    issues,
                );
            }
        }
        SchemaVariantJson::Tuple { items, .. } => {
            if let Some(array) = value.as_array() {
                for (idx, item) in array.iter().enumerate() {
                    if let Some(field) = items.get(idx) {
                        validate_value_against_schema(
                            item,
                            &field.schema,
                            &format!("{}[{}]", path, idx),
                            issues,
                        );
                    }
                }
            } else if items.len() == 1 {
                validate_value_against_schema(value, &items[0].schema, path, issues);
            } else {
                issues.push(issue_warning(path, "Expected array for enum tuple variant"));
            }
        }
    }
}

fn lookup_variant_by_value<'a>(
    variants: &'a [SchemaVariantJson],
    value: &str,
) -> Option<&'a SchemaVariantJson> {
    let normalized = normalize_effect_key(value);
    variants.iter().find(|variant| {
        let (name, json_value) = match variant {
            SchemaVariantJson::Unit {
                name, json_value, ..
            } => (name, json_value.as_ref()),
            SchemaVariantJson::Tuple {
                name, json_value, ..
            } => (name, json_value.as_ref()),
            SchemaVariantJson::Struct {
                name, json_value, ..
            } => (name, json_value.as_ref()),
        };
        let name_norm = normalize_effect_key(name);
        let json_norm = json_value
            .as_ref()
            .map(|v| normalize_effect_key(v))
            .unwrap_or_default();
        normalized == name_norm || (!json_norm.is_empty() && normalized == json_norm)
    })
}

fn lookup_schema_variant(
    schemas: &EffectSchemasIndex,
    category: &str,
    effect_type: &str,
) -> Option<SchemaVariantJson> {
    let normalized = normalize_effect_key(effect_type);
    let category_map = schemas.categories.get(category)?;
    category_map.get(&normalized).cloned()
}

fn normalize_category(category: &str) -> &str {
    match category {
        "mask" => "masks",
        "filter" => "filters",
        "sampler" => "samplers",
        other => other,
    }
}

fn load_effect_schemas(path: &Path) -> Result<EffectSchemasIndex> {
    let content =
        fs::read_to_string(path).with_context(|| format!("Failed to read {}", path.display()))?;
    let parsed: EffectSchemasFile = serde_json::from_str(&content)
        .with_context(|| format!("Failed to parse {}", path.display()))?;

    let mut categories = HashMap::new();
    for (category, schema) in parsed.categories {
        let mut variants = HashMap::new();
        for (key, variant) in schema.variants {
            variants.insert(normalize_effect_key(&key), variant);
        }
        categories.insert(category, variants);
    }

    Ok(EffectSchemasIndex { categories })
}

fn lookup_effect(
    capabilities: &CapabilitiesIndex,
    category: &str,
    effect_type: &str,
) -> Option<CapEffectInfo> {
    let normalized = normalize_effect_key(effect_type);
    let category_map = capabilities.categories.get(category)?;
    category_map.get(&normalized).cloned()
}

fn normalize_effect_key(name: &str) -> String {
    let mut out = String::new();
    let mut prev_lower = false;
    for ch in name.chars() {
        if ch.is_uppercase() {
            if prev_lower {
                out.push('_');
            }
            for lower in ch.to_lowercase() {
                out.push(lower);
            }
            prev_lower = false;
        } else if ch == '-' || ch == ' ' {
            out.push('_');
            prev_lower = false;
        } else {
            out.push(ch.to_ascii_lowercase());
            prev_lower = ch.is_ascii_lowercase() || ch.is_ascii_digit();
        }
    }
    out
}

fn validate_param_type(ty: &str, value: &Value) -> Option<String> {
    let normalized = ty.trim();
    if normalized.is_empty() {
        return None;
    }

    let base = normalized
        .strip_prefix("Option<")
        .and_then(|t| t.strip_suffix('>'))
        .unwrap_or(normalized);

    let expected = if base.starts_with("Vec<") {
        "array"
    } else {
        match base {
            "bool" => "bool",
            "String" => "string",
            "str" => "string",
            "u8" | "u16" | "u32" | "u64" | "usize" | "i8" | "i16" | "i32" | "i64" | "isize"
            | "f32" | "f64" => "number",
            _ => return None,
        }
    };

    let matches = match expected {
        "array" => value.is_array(),
        "bool" => value.is_boolean(),
        "string" => value.is_string(),
        "number" => value.is_number(),
        _ => true,
    };

    if matches {
        None
    } else {
        Some(format!("Expected {}", expected))
    }
}

fn issue_error(path: &str, message: &str) -> RecipeIssue {
    RecipeIssue {
        severity: IssueSeverity::Error,
        path: path.to_string(),
        message: message.to_string(),
    }
}

fn issue_warning(path: &str, message: &str) -> RecipeIssue {
    RecipeIssue {
        severity: IssueSeverity::Warning,
        path: path.to_string(),
        message: message.to_string(),
    }
}

fn issue_info(path: &str, message: &str) -> RecipeIssue {
    RecipeIssue {
        severity: IssueSeverity::Info,
        path: path.to_string(),
        message: message.to_string(),
    }
}

fn summarize_reports(reports: &[RecipeReport]) -> ReportSummary {
    let mut files_with_errors = 0;
    let mut files_with_warnings = 0;
    let mut error_count = 0;
    let mut warning_count = 0;
    let mut info_count = 0;

    for report in reports {
        let mut has_error = false;
        let mut has_warning = false;
        for issue in &report.issues {
            match issue.severity {
                IssueSeverity::Error => {
                    has_error = true;
                    error_count += 1;
                }
                IssueSeverity::Warning => {
                    has_warning = true;
                    warning_count += 1;
                }
                IssueSeverity::Info => info_count += 1,
            }
        }
        if has_error {
            files_with_errors += 1;
        }
        if has_warning {
            files_with_warnings += 1;
        }
    }

    ReportSummary {
        total_recipes: reports.len(),
        files_with_errors,
        files_with_warnings,
        error_count,
        warning_count,
        info_count,
    }
}

fn write_report_json(path: &Path, report: &ValidationReport) -> Result<()> {
    let json = serde_json::to_string_pretty(report)
        .context("Failed to serialize recipe validation report")?;
    fs::write(path, json).with_context(|| format!("Failed to write {}", path.display()))?;
    Ok(())
}

fn write_report_markdown(path: &Path, report: &ValidationReport) -> Result<()> {
    let mut lines = Vec::new();
    lines.push("# Recipe Validation Report".to_string());
    lines.push("".to_string());
    lines.push(format!("- Total recipes: {}", report.summary.total_recipes));
    lines.push(format!(
        "- Files with errors: {}",
        report.summary.files_with_errors
    ));
    lines.push(format!(
        "- Files with warnings: {}",
        report.summary.files_with_warnings
    ));
    lines.push(format!("- Error count: {}", report.summary.error_count));
    lines.push(format!("- Warning count: {}", report.summary.warning_count));
    lines.push(format!("- Info count: {}", report.summary.info_count));
    lines.push("".to_string());

    for recipe in &report.recipes {
        if recipe.issues.is_empty() {
            continue;
        }
        lines.push(format!("## {}", recipe.path));
        if let Some(id) = &recipe.id {
            lines.push(format!("- id: {}", id));
        }
        if let Some(title) = &recipe.title {
            lines.push(format!("- title: {}", title));
        }
        if !recipe.extends.is_empty() {
            lines.push(format!("- extends: {}", recipe.extends.join(" -> ")));
        }
        lines.push("".to_string());
        for issue in &recipe.issues {
            lines.push(format!(
                "- [{}] {} - {}",
                format!("{:?}", issue.severity).to_lowercase(),
                issue.path,
                issue.message
            ));
        }
        lines.push("".to_string());
    }

    fs::write(path, lines.join("\n"))
        .with_context(|| format!("Failed to write {}", path.display()))?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn normalize_effect_key_handles_pascal_case() {
        assert_eq!(normalize_effect_key("GlistenSweep"), "glisten_sweep");
        assert_eq!(normalize_effect_key("FadeOut"), "fade_out");
        assert_eq!(normalize_effect_key("Crt"), "crt");
        assert_eq!(normalize_effect_key("linear_gradient"), "linear_gradient");
    }

    #[test]
    fn merge_values_prefers_child_and_merges_objects() {
        let parent = json!({"a": {"b": 1, "c": 2}, "d": 3});
        let child = json!({"a": {"b": 9}, "e": 4});
        let merged = merge_values(parent, child);
        assert_eq!(merged["a"]["b"], 9);
        assert_eq!(merged["a"]["c"], 2);
        assert_eq!(merged["d"], 3);
        assert_eq!(merged["e"], 4);
    }

    #[test]
    fn validate_param_type_checks_primitives() {
        assert!(validate_param_type("f32", &json!(1.2)).is_none());
        assert!(validate_param_type("String", &json!("ok")).is_none());
        assert!(validate_param_type("bool", &json!(true)).is_none());
        assert!(validate_param_type("Vec<u8>", &json!([1, 2])).is_none());
        assert!(validate_param_type("f32", &json!("no")).is_some());
        assert!(validate_param_type("String", &json!(5)).is_some());
    }

    #[test]
    fn lookup_effect_matches_snake_case() {
        let mut categories = HashMap::new();
        let mut filters = HashMap::new();
        filters.insert(
            "glisten_sweep".to_string(),
            CapEffectInfo {
                canonical: "GlistenSweep".to_string(),
                params: HashMap::new(),
            },
        );
        categories.insert("filters".to_string(), filters);
        let index = CapabilitiesIndex { categories };
        let found = lookup_effect(&index, "filters", "glisten_sweep");
        assert!(found.is_some());
    }

    #[test]
    fn normalize_category_maps_singular() {
        assert_eq!(normalize_category("mask"), "masks");
        assert_eq!(normalize_category("filter"), "filters");
        assert_eq!(normalize_category("sampler"), "samplers");
        assert_eq!(normalize_category("styles"), "styles");
    }

    #[test]
    fn tuple_variant_accepts_struct_object() {
        let variant = SchemaVariantJson::Tuple {
            name: "Example".to_string(),
            description: None,
            json_value: Some("example".to_string()),
            items: vec![SchemaFieldJson {
                name: "0".to_string(),
                json_key: None,
                meta: FieldMetaJson {
                    help: None,
                    description: None,
                    default: None,
                    range: None,
                    json_key: None,
                    optional: false,
                },
                schema: SchemaNodeJson::Struct {
                    name: "ExampleSpec".to_string(),
                    description: None,
                    json_name: None,
                    fields: vec![SchemaFieldJson {
                        name: "speed".to_string(),
                        json_key: None,
                        meta: FieldMetaJson {
                            help: None,
                            description: None,
                            default: None,
                            range: None,
                            json_key: None,
                            optional: false,
                        },
                        schema: SchemaNodeJson::Primitive {
                            type_name: "f32".to_string(),
                            range: None,
                        },
                    }],
                },
            }],
        };

        let mut issues = Vec::new();
        let value = serde_json::json!({
            "type": "example",
            "speed": 1.0
        });
        let obj = value.as_object().expect("object");
        validate_effect_against_schema(obj, &variant, "$.effect", &mut issues);
        assert!(
            issues.is_empty(),
            "expected tuple variant to accept struct object"
        );
    }
}

// <FILE>xtask/src/recipes/mod.rs</FILE> - <DESC>Recipe validation against capabilities.json</DESC>
// <VERS>END OF VERSION: 0.2.6</VERS>
