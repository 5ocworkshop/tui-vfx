// <FILE>xtask/src/docs/gen_api.rs</FILE> - <DESC>Generate API.md from code metadata + api_docs.toml</DESC>
// <VERS>VERSION: 1.2.0</VERS>
// <WCTX>Phase 2 dramatic color-shadow rollout: docs, examples, and quality closure</WCTX>
// <CLOG>Render ShadowCompositeMode and ShadowGradeConfig sections in shadows part</CLOG>

use super::api_metadata::ApiMetadata;
use super::effect_metadata::EffectMetadata;
use super::parse_api_toml::ApiDocsManifest;
use anyhow::{Context, Result};
use std::collections::HashMap;
use std::fs;

const OUTPUT_PATH: &str = "docs/generated/API.md";

/// Generate API.md from API metadata and TOML manifest.
pub fn generate(api: &ApiMetadata, toml: &ApiDocsManifest) -> Result<String> {
    let mut output = String::new();

    // Header with OFPF metadata
    write_header(&mut output, toml);

    // Title and intro
    output.push_str(&format!("# {}\n\n", toml.structure.title));
    output.push_str(&toml.structure.intro);
    output.push_str("\n\n---\n\n");

    // Effect inventory summary table
    write_effect_inventory(&mut output, api);

    // Entry points section
    write_entry_points(&mut output, api, toml);

    // Quick start example
    write_quick_start(&mut output, toml);

    // Part 1: Compositor Pipeline
    write_compositor_section(&mut output, api, toml);

    // Part 2: Shadows
    write_shadows_section(&mut output, api, toml);

    // Part 3: Style System
    write_style_section(&mut output, api, toml);

    // Part 4: Content Transformers
    write_content_section(&mut output, api, toml);

    // Part 5: Geometry & Motion
    write_geometry_section(&mut output, toml);

    // Part 6: Prelude
    write_prelude_section(&mut output, toml);

    // Footer
    write_footer(&mut output, toml);

    Ok(output)
}

/// Generate and write API.md to disk.
pub fn generate_and_write(api: &ApiMetadata, toml: &ApiDocsManifest) -> Result<()> {
    let output = generate(api, toml)?;
    fs::write(OUTPUT_PATH, &output).with_context(|| format!("Failed to write {}", OUTPUT_PATH))?;
    Ok(())
}

fn write_header(output: &mut String, toml: &ApiDocsManifest) {
    output.push_str(&format!(
        "<!-- <FILE>{}</FILE> - <DESC>Complete TUI-VFX API documentation</DESC> -->\n",
        OUTPUT_PATH
    ));
    output.push_str(&format!(
        "<!-- <VERS>VERSION: {}</VERS> -->\n",
        toml.meta.version
    ));
    output.push_str("<!-- <WCTX>Generated API documentation</WCTX> -->\n");
    output.push_str("<!-- <CLOG>Auto-generated from code + api_docs.toml</CLOG> -->\n\n");
}

fn write_footer(output: &mut String, toml: &ApiDocsManifest) {
    output.push_str("---\n\n");
    output.push_str(&format!(
        "<!-- <FILE>{}</FILE> - <DESC>Complete TUI-VFX API documentation</DESC> -->\n",
        OUTPUT_PATH
    ));
    output.push_str(&format!(
        "<!-- <VERS>END OF VERSION: {}</VERS> -->\n",
        toml.meta.version
    ));
}

fn write_effect_inventory(output: &mut String, api: &ApiMetadata) {
    output.push_str("## Effect Inventory Summary\n\n");
    output.push_str("| Category | Count (variants) | Primary API |\n");
    output.push_str("| --- | --- | --- |\n");

    let categories = [
        ("Masks", "masks", "tui_vfx_compositor::types::MaskSpec"),
        (
            "Filters",
            "filters",
            "tui_vfx_compositor::types::FilterSpec",
        ),
        (
            "Samplers",
            "samplers",
            "tui_vfx_compositor::types::SamplerSpec",
        ),
        (
            "Spatial Shaders",
            "shaders",
            "tui_vfx_style::models::SpatialShaderType",
        ),
        (
            "Style Effects",
            "styles",
            "tui_vfx_style::models::StyleEffect",
        ),
        (
            "Content Transformers",
            "content",
            "tui_vfx_content::types::ContentEffect",
        ),
        (
            "Shadows",
            "shadows",
            "tui_vfx_shadow::ShadowConfig / ShadowSpec",
        ),
    ];

    for (name, key, api_path) in categories {
        let count = api.effect_counts.get(key).copied().unwrap_or(0);
        let count_display = if key == "shadows" {
            format!("{} styles", count)
        } else if count > 0 {
            // Most categories have a None variant we don't count
            format!("{} (+ `None`)", count.saturating_sub(1))
        } else {
            count.to_string()
        };
        output.push_str(&format!(
            "| {} | {} | `{}` |\n",
            name, count_display, api_path
        ));
    }

    output.push_str("| Geometry & Motion | 20+ | `tui_vfx_geometry::types::*` |\n");
    output.push_str("\n---\n\n");
}

fn write_entry_points(output: &mut String, api: &ApiMetadata, toml: &ApiDocsManifest) {
    output.push_str("## Unified Entry Points\n\n");

    for func in &api.entry_points {
        // Get editorial content from TOML
        let entry = toml
            .entry_points
            .functions
            .get(&func.name)
            .cloned()
            .unwrap_or_default();

        // Title with optional subtitle
        if entry.subtitle.is_empty() {
            output.push_str(&format!("### `{}`\n\n", func.name));
        } else {
            output.push_str(&format!("### `{}` ({})\n\n", func.name, entry.subtitle));
        }

        // Code block with signature
        output.push_str("```rust\n");
        output.push_str(&func.signature);
        output.push_str("\n```\n\n");

        // Notes from TOML
        if !entry.notes.is_empty() {
            output.push_str(&entry.notes);
            output.push_str("\n\n");
        }
    }

    output.push_str("---\n\n");
}

fn write_quick_start(output: &mut String, toml: &ApiDocsManifest) {
    if let Some(example) = toml.examples.get("quick_start") {
        output.push_str(&format!("## {}\n\n", example.title));
        output.push_str("```rust\n");
        output.push_str(example.code.trim());
        output.push_str("\n```\n\n");
        output.push_str("---\n\n");
    }
}

fn write_compositor_section(output: &mut String, api: &ApiMetadata, toml: &ApiDocsManifest) {
    output.push_str("# Part 1: Compositor Pipeline\n\n");

    // CompositionOptions
    write_core_type(output, "CompositionOptions", api, toml);

    // CompositionSpec
    write_core_type(output, "CompositionSpec", api, toml);

    // Shader Layers
    output.push_str("## Shader Layers\n\n");
    for type_name in ["ShaderWithRegion", "ShaderLayerSpec"] {
        if let Some(type_doc) = api.core_types.iter().find(|t| t.name == type_name) {
            if let Some(code) = &type_doc.code_block {
                output.push_str("```rust\n");
                output.push_str(code);
                output.push_str("\n```\n\n");
            }
        }
    }
    output.push_str(
        "Use `ShaderWithRegion` for runtime shader instances and `ShaderLayerSpec` for serialized specs.\n\n",
    );

    // Timing
    write_timing_section(output, toml);

    // Render Order
    write_render_order(output, toml);

    // MaskSpec
    write_effect_spec_section(output, "MaskSpec", &api.effects.masks, api, toml);

    // FilterSpec
    write_effect_spec_section(output, "FilterSpec", &api.effects.filters, api, toml);

    // SamplerSpec
    write_effect_spec_section(output, "SamplerSpec", &api.effects.samplers, api, toml);
}

fn write_core_type(output: &mut String, name: &str, api: &ApiMetadata, toml: &ApiDocsManifest) {
    let type_entry = toml.types.get(name);
    let type_doc = api.core_types.iter().find(|t| t.name == name);

    // Section title with optional subtitle
    let subtitle = type_entry.map(|e| e.subtitle.as_str()).unwrap_or("");
    if subtitle.is_empty() {
        output.push_str(&format!("## {}\n\n", name));
    } else {
        output.push_str(&format!("## {} ({})\n\n", name, subtitle));
    }

    // Code block
    if let Some(doc) = type_doc {
        if let Some(code) = &doc.code_block {
            output.push_str("```rust\n");
            output.push_str(code);
            output.push_str("\n```\n\n");
        }
    }

    // Builder methods
    if let Some(entry) = type_entry {
        if !entry.builder_methods.is_empty() {
            let title = if entry.builder_title.is_empty() {
                "Builder methods:"
            } else {
                &entry.builder_title
            };
            output.push_str(&format!("**{}**\n", title));
            for method in &entry.builder_methods {
                output.push_str(&format!("- {}\n", method));
            }
            output.push('\n');
        }

        // Notes
        if !entry.notes.is_empty() {
            output.push_str("**Notes:**\n");
            output.push_str(&entry.notes);
            output.push_str("\n\n");
        }
    }
}

fn write_timing_section(output: &mut String, toml: &ApiDocsManifest) {
    if !toml.timing.section_title.is_empty() {
        output.push_str(&format!("## {}\n\n", toml.timing.section_title));
    } else {
        output.push_str("## Timing: `t`, `loop_t`, `phase`\n\n");
    }

    output.push_str(&format!("- `t`: {}\n", toml.timing.t));
    output.push_str(&format!("- `loop_t`: {}\n", toml.timing.loop_t));
    output.push_str(&format!("- `phase`: {}\n", toml.timing.phase));
    output.push('\n');
}

fn write_render_order(output: &mut String, toml: &ApiDocsManifest) {
    output.push_str("## Render Order\n\n");

    let mut steps = toml.render_order.steps.clone();
    steps.sort_by_key(|s| s.order);

    for step in &steps {
        if step.description.is_empty() {
            output.push_str(&format!("{}. {}\n", step.order, step.name));
        } else {
            output.push_str(&format!(
                "{}. {} ({})\n",
                step.order, step.name, step.description
            ));
        }
    }

    output.push_str("\n---\n\n");
}

fn write_effect_spec_section(
    output: &mut String,
    spec_name: &str,
    effects: &HashMap<String, EffectMetadata>,
    api: &ApiMetadata,
    toml: &ApiDocsManifest,
) {
    let spec_entry = toml.specs.get(spec_name);
    let count = effects.len().saturating_sub(1); // Subtract None variant

    output.push_str(&format!("## {} ({} effects)\n\n", spec_name, count));

    // Description
    if let Some(entry) = spec_entry {
        if !entry.description.is_empty() {
            output.push_str(&entry.description);
            output.push_str("\n\n");
        }
    }

    // Effect table
    output.push_str("| Variant | Description | Parameters |\n");
    output.push_str("| --- | --- | --- |\n");

    let mut sorted: Vec<_> = effects.iter().collect();
    sorted.sort_by_key(|(name, _)| name.as_str());

    for (name, effect) in sorted {
        let params = if effect.parameters.is_empty() {
            "-".to_string()
        } else {
            effect
                .parameters
                .iter()
                .map(|(k, _)| format!("`{}`", k))
                .collect::<Vec<_>>()
                .join(", ")
        };
        output.push_str(&format!(
            "| `{}` | {} | {} |\n",
            name, effect.description, params
        ));
    }

    output.push('\n');

    // Additional notes from TOML
    if let Some(entry) = spec_entry {
        let mut sorted_notes: Vec<_> = entry.notes.iter().collect();
        sorted_notes.sort_by_key(|(key, _)| key.as_str());
        for (note_key, note) in sorted_notes {
            if !note.title.is_empty() {
                output.push_str(&format!("### {}\n\n", note.title));
            } else {
                output.push_str(&format!("### {}\n\n", note_key));
            }
            output.push_str(&note.content);
            output.push_str("\n\n");
        }

        // Supporting enums
        write_supporting_enums(output, spec_name, api, toml);
    }

    output.push_str("---\n\n");
}

fn write_supporting_enums(
    output: &mut String,
    spec_name: &str,
    api: &ApiMetadata,
    _toml: &ApiDocsManifest,
) {
    // Get enums from API metadata for this spec category
    let category = match spec_name {
        "MaskSpec" => "masks",
        "FilterSpec" => "filters",
        "SamplerSpec" => "samplers",
        _ => return,
    };

    if let Some(enums) = api.enums.get(category) {
        for enum_doc in enums {
            output.push_str(&format!("### {}\n\n", enum_doc.name));

            if !enum_doc.description.is_empty() {
                output.push_str(&enum_doc.description);
                output.push_str("\n\n");
            }

            // Group variants by description if they have categories
            let has_categories = enum_doc
                .variants
                .iter()
                .any(|v| !v.description.is_empty() && v.description != v.name);

            if has_categories && enum_doc.name == "WipeDirection" {
                // Special handling for WipeDirection with grouped variants
                let groups: Vec<(&str, Vec<&str>)> = vec![
                    (
                        "Cardinal",
                        vec!["LeftToRight", "RightToLeft", "TopToBottom", "BottomToTop"],
                    ),
                    (
                        "Diagonal",
                        vec![
                            "TopLeftToBottomRight",
                            "TopRightToBottomLeft",
                            "BottomLeftToTopRight",
                            "BottomRightToTopLeft",
                        ],
                    ),
                    (
                        "Aliases",
                        vec!["FromLeft", "FromRight", "FromTop", "FromBottom"],
                    ),
                    (
                        "Center-out (curtains opening)",
                        vec!["HorizontalCenterOut", "VerticalCenterOut"],
                    ),
                    (
                        "Edges-in (curtains closing)",
                        vec!["HorizontalEdgesIn", "VerticalEdgesIn"],
                    ),
                ];

                for (group, variants) in groups {
                    output.push_str(&format!("{}:\n`{}`\n\n", group, variants.join("`, `")));
                }
            } else if enum_doc.name == "MaskCombineMode" {
                // Table format for MaskCombineMode
                output.push_str("| Mode | Behavior |\n");
                output.push_str("| --- | --- |\n");
                for variant in &enum_doc.variants {
                    let name = if variant.parameters.is_empty() {
                        format!("`{}`", variant.name)
                    } else {
                        format!("`{} {}`", variant.name, variant.parameters)
                    };
                    output.push_str(&format!("| {} | {} |\n", name, variant.description));
                }
                output.push('\n');
            } else {
                // Simple list format
                let variant_names: Vec<String> = enum_doc
                    .variants
                    .iter()
                    .map(|v| {
                        if v.parameters.is_empty() {
                            format!("`{}`", v.name)
                        } else {
                            format!("`{} {}`", v.name, v.parameters)
                        }
                    })
                    .collect();
                output.push_str(&variant_names.join(", "));
                output.push_str("\n\n");
            }
        }
    }
}

fn write_shadows_section(output: &mut String, api: &ApiMetadata, toml: &ApiDocsManifest) {
    output.push_str("# Part 2: Shadows\n\n");

    // Intro
    if let Some(entry) = toml.specs.get("ShadowConfig") {
        if !entry.intro.is_empty() {
            output.push_str(&entry.intro);
            output.push_str("\n\n");
        }
    }

    // ShadowConfig
    if let Some(type_doc) = api.core_types.iter().find(|t| t.name == "ShadowConfig") {
        output.push_str("## ShadowConfig\n\n");
        if let Some(code) = &type_doc.code_block {
            output.push_str("```rust\n");
            output.push_str(code);
            output.push_str("\n```\n\n");
        }
    }

    // ShadowStyle
    output.push_str("## ShadowStyle\n");
    let mut sorted_shadows: Vec<_> = api.effects.shadows.iter().collect();
    sorted_shadows.sort_by_key(|(name, _)| name.as_str());
    for (name, effect) in sorted_shadows {
        output.push_str(&format!("- `{}` — {}\n", name, effect.description));
    }
    output.push('\n');

    // ShadowEdges
    if let Some(enums) = api.enums.get("shadows") {
        for enum_doc in enums {
            output.push_str(&format!("## {} (bitflags)\n", enum_doc.name));
            let main: Vec<_> = enum_doc
                .variants
                .iter()
                .filter(|v| v.description.is_empty())
                .map(|v| format!("`{}`", v.name))
                .collect();
            let convenience: Vec<_> = enum_doc
                .variants
                .iter()
                .filter(|v| !v.description.is_empty())
                .map(|v| format!("`{}`", v.name))
                .collect();

            output.push_str(&main.join(", "));
            if !convenience.is_empty() {
                output.push_str(&format!(", plus convenience {}", convenience.join(", ")));
            }
            output.push_str(".\n\n");
        }
    }

    // Rule
    if let Some(entry) = toml.specs.get("ShadowEdges") {
        if !entry.rule.is_empty() {
            output.push_str(&format!("**Rule:** {}\n\n", entry.rule));
        }
    }

    // ShadowCompositeMode
    if let Some(entry) = toml.specs.get("ShadowCompositeMode") {
        output.push_str(&format!("## {}\n\n", entry.section_title));
        for variant in &entry.variants {
            if let Some(table) = variant.as_table() {
                let name = table.get("name").and_then(|v| v.as_str()).unwrap_or("");
                let note = table.get("note").and_then(|v| v.as_str()).unwrap_or("");
                output.push_str(&format!("- `{}` {}\n", name, note));
            } else if let Some(s) = variant.as_str() {
                output.push_str(&format!("- `{}`\n", s));
            }
        }
        output.push('\n');
    }

    // ShadowGradeConfig
    if let Some(entry) = toml.specs.get("ShadowGradeConfig") {
        output.push_str(&format!("## {}\n\n", entry.section_title));
        if !entry.description.is_empty() {
            output.push_str(&entry.description);
            output.push_str("\n\n");
        }
    }

    // Compositor integration example
    if let Some(entry) = toml.specs.get("ShadowSpec") {
        output.push_str("## Compositor integration\n\n");
        if !entry.example.is_empty() {
            output.push_str(&entry.example);
            output.push('\n');
        }
        if !entry.see_also.is_empty() {
            output.push_str(&entry.see_also);
            output.push_str("\n\n");
        }
    }

    output.push_str("---\n\n");
}

fn write_style_section(output: &mut String, api: &ApiMetadata, toml: &ApiDocsManifest) {
    output.push_str("# Part 3: Style System\n\n");

    // StyleRegion
    if let Some(entry) = toml.specs.get("StyleRegion") {
        output.push_str("## StyleRegion\n\n");
        output.push_str(&entry.description);
        output.push_str("\n\n");

        let variants: Vec<String> = entry
            .variants
            .iter()
            .filter_map(|v| v.as_str().map(|s| format!("`{}`", s)))
            .collect();
        output.push_str(&variants.join(", "));
        output.push_str("\n\n");

        output.push_str("`ModuloAxis`: `Horizontal`, `Vertical`\n\n");
    }

    // SpatialShaderType
    output.push_str("## Spatial Shaders (`SpatialShaderType`)\n\n");
    output.push_str(
        "These are serializable shader variants for use in `CompositionSpec` and `ShaderLayerSpec`.\n\n",
    );

    output.push_str("| Shader | Parameters |\n");
    output.push_str("| --- | --- |\n");

    let mut sorted: Vec<_> = api.effects.shaders.iter().collect();
    sorted.sort_by_key(|(name, _)| name.as_str());

    for (name, effect) in sorted {
        let params = if effect.parameters.is_empty() {
            "-".to_string()
        } else {
            effect
                .parameters
                .iter()
                .map(|(k, _)| format!("`{}`", k))
                .collect::<Vec<_>>()
                .join(", ")
        };
        output.push_str(&format!("| `{}` | {} |\n", name, params));
    }
    output.push('\n');

    // Shader supporting enums
    if let Some(entry) = toml.specs.get("SpatialShaderType") {
        if !entry.enums.items.is_empty() {
            output.push_str("### Shader-specific supporting enums\n\n");
            for item in &entry.enums.items {
                output.push_str(&format!(
                    "- `{}`: {}\n",
                    item.name,
                    item.variants
                        .iter()
                        .map(|v| format!("`{}`", v))
                        .collect::<Vec<_>>()
                        .join(", ")
                ));
            }
            output.push('\n');
        }

        // Runtime-only shaders note
        if let Some(note) = entry.notes.get("runtime_only") {
            output.push_str(&format!("### {}\n\n", note.title));
            output.push_str(&note.content);
            output.push_str("\n\n");
        }
    }

    output.push_str("---\n\n");

    // StyleEffect
    output.push_str("## StyleEffect (temporal effects)\n\n");
    output.push_str("```rust\n");
    output.push_str("pub enum StyleEffect {\n");
    let mut sorted_styles: Vec<_> = api.effects.styles.iter().collect();
    sorted_styles.sort_by_key(|(name, _)| name.as_str());
    for (name, effect) in sorted_styles {
        let params = if effect.parameters.is_empty() {
            String::new()
        } else {
            format!(
                " {{ {} }}",
                effect
                    .parameters
                    .iter()
                    .map(|(k, v)| format!("{}: {}", k, v))
                    .collect::<Vec<_>>()
                    .join(", ")
            )
        };
        output.push_str(&format!("    {}{},\n", name, params));
    }
    output.push_str("}\n");
    output.push_str("```\n\n");

    // Additional style types (from TOML)
    for type_name in [
        "StyleConfig",
        "StyleLayer",
        "StyleTransition",
        "FadeSpec",
        "ColorConfig",
    ] {
        if let Some(entry) = toml.specs.get(type_name) {
            output.push_str(&format!("### {}\n\n", entry.section_title.as_str()));

            if !entry.description.is_empty() {
                output.push_str(&entry.description);
                output.push_str("\n\n");
            }
        }
    }

    output.push_str("---\n\n");
}

fn write_content_section(output: &mut String, api: &ApiMetadata, toml: &ApiDocsManifest) {
    output.push_str("# Part 4: Content Transformers\n\n");

    if let Some(entry) = toml.specs.get("ContentEffect") {
        if !entry.description.is_empty() {
            output.push_str(&entry.description);
            output.push_str("\n\n");
        }
    }

    // ContentEffect enum
    output.push_str("```rust\n");
    output.push_str("pub enum ContentEffect {\n");
    let mut sorted_content: Vec<_> = api.effects.content.iter().collect();
    sorted_content.sort_by_key(|(name, _)| name.as_str());
    for (name, effect) in sorted_content {
        let params = if effect.parameters.is_empty() {
            String::new()
        } else {
            format!(
                " {{ {} }}",
                effect
                    .parameters
                    .iter()
                    .map(|(k, v)| format!("{}: {}", k, v))
                    .collect::<Vec<_>>()
                    .join(", ")
            )
        };
        output.push_str(&format!("    {}{},\n", name, params));
    }
    output.push_str("}\n");
    output.push_str("```\n\n");

    // Supporting types from TOML
    output.push_str("### Supporting types\n\n");

    if let Some(entry) = toml.specs.get("ContentEffect") {
        for item in &entry.enums.items {
            output.push_str(&format!(
                "**{}:** {}\n\n",
                item.name,
                item.variants
                    .iter()
                    .map(|v| format!("`{}`", v))
                    .collect::<Vec<_>>()
                    .join(", ")
            ));
        }

        // MorphProgression (special handling for long list)
        if let Some(morph) = toml.specs.get("ContentEffect") {
            if !morph.enums.variants.is_empty() {
                output.push_str("**MorphProgression:**\n");
                output.push_str(
                    &morph
                        .enums
                        .variants
                        .iter()
                        .map(|v| format!("`{}`", v))
                        .collect::<Vec<_>>()
                        .join(", "),
                );
                output.push_str("\n\n");
            }
        }
    }

    output.push_str("---\n\n");
}

fn write_geometry_section(output: &mut String, toml: &ApiDocsManifest) {
    output.push_str("# Part 5: Geometry & Motion\n\n");
    output.push_str(
        "These types drive transitions, motion paths, easing, and layout resolution.\n\n",
    );

    // TransitionSpec, MotionSpec, etc. - simplified output from TOML
    let geometry_types = [
        "TransitionSpec",
        "MotionSpec",
        "SlideDirection",
        "PathType",
        "EasingCurve",
        "EasingType",
        "SnappingStrategy",
        "PlacementSpec",
        "Origin",
        "Shake",
    ];

    for type_name in geometry_types {
        if let Some(entry) = toml.specs.get(type_name) {
            output.push_str(&format!("## {}\n\n", entry.section_title));

            if !entry.description.is_empty() {
                output.push_str(&entry.description);
                output.push_str("\n\n");
            }

            if !entry.variants.is_empty() {
                let variants: Vec<String> = entry
                    .variants
                    .iter()
                    .filter_map(|v| v.as_str().map(|s| format!("`{}`", s)))
                    .collect();
                output.push_str(&variants.join(", "));
                output.push_str("\n\n");
            }
        }
    }

    output.push_str("---\n\n");
}

fn write_prelude_section(output: &mut String, toml: &ApiDocsManifest) {
    output.push_str("# Part 6: Prelude & Imports\n\n");

    output.push_str(&toml.prelude.recommendation);
    output.push_str("\n\n");

    output.push_str("```rust\n");
    output.push_str(&toml.prelude.usage);
    output.push_str("\n```\n\n");

    output.push_str("The prelude includes:\n\n");

    output.push_str("```rust\n");
    output.push_str("// Types\n");
    output.push_str(&toml.prelude.exports.types);
    output.push_str("\n\n");
    output.push_str("// Core schema\n");
    output.push_str(&toml.prelude.exports.core_schema);
    output.push_str("\n\n");
    output.push_str("// Geometry\n");
    output.push_str(&toml.prelude.exports.geometry);
    output.push_str("\n\n");
    output.push_str("// Compositor pipeline\n");
    output.push_str(&toml.prelude.exports.compositor_pipeline);
    output.push_str("\n\n");
    output.push_str("// Compositor types\n");
    output.push_str(&toml.prelude.exports.compositor_types);
    output.push_str("\n\n");
    output.push_str("// Style\n");
    output.push_str(&toml.prelude.exports.style);
    output.push_str("\n\n");
    output.push_str("// Content\n");
    output.push_str(&toml.prelude.exports.content);
    output.push_str("\n\n");
    output.push_str("// Shadows\n");
    output.push_str(&toml.prelude.exports.shadows);
    output.push_str("\n```\n\n");
}

// <FILE>xtask/src/docs/gen_api.rs</FILE> - <DESC>Generate API.md from code metadata + api_docs.toml</DESC>
// <VERS>END OF VERSION: 1.2.0</VERS>
