// <FILE>xtask/src/docs/api_metadata.rs</FILE> - <DESC>API metadata extraction from runtime introspection</DESC>
// <VERS>VERSION: 1.2.0</VERS>
// <WCTX>Phase 2 dramatic color-shadow rollout: docs, examples, and quality closure</WCTX>
// <CLOG>Add composite_mode and grade fields to ShadowConfig TypeDoc</CLOG>

use std::collections::HashMap;

use super::effect_metadata::{AllEffectMetadata, extract_all_metadata};

/// Complete API metadata extracted from code.
#[derive(Debug, Default)]
pub struct ApiMetadata {
    /// Effect metadata (masks, filters, samplers, shaders, styles, content, shadows)
    pub effects: AllEffectMetadata,

    /// Entry point function signatures
    pub entry_points: Vec<FunctionDoc>,

    /// Core configuration types
    pub core_types: Vec<TypeDoc>,

    /// Supporting enums organized by category
    pub enums: HashMap<String, Vec<EnumDoc>>,

    /// Effect counts per category (for inventory table)
    pub effect_counts: HashMap<String, usize>,
}

/// Documentation for a function.
#[derive(Debug, Clone)]
#[allow(dead_code)] // Fields used for TOML serialization
pub struct FunctionDoc {
    pub name: String,
    pub signature: String,
    pub description: String,
}

/// Documentation for a type/struct.
#[derive(Debug, Clone)]
#[allow(dead_code)] // Fields used for TOML serialization
pub struct TypeDoc {
    pub name: String,
    pub description: String,
    pub fields: Vec<FieldDoc>,
    pub code_block: Option<String>,
}

/// Documentation for a struct field.
#[derive(Debug, Clone)]
#[allow(dead_code)] // Fields used for TOML serialization
pub struct FieldDoc {
    pub name: String,
    pub ty: String,
    pub description: String,
}

/// Documentation for an enum.
#[derive(Debug, Clone)]
pub struct EnumDoc {
    pub name: String,
    pub description: String,
    pub variants: Vec<VariantDoc>,
}

/// Documentation for an enum variant.
#[derive(Debug, Clone)]
pub struct VariantDoc {
    pub name: String,
    pub parameters: String,
    pub description: String,
}

/// Extract all API metadata using runtime introspection.
pub fn extract_api_metadata() -> ApiMetadata {
    let effects = extract_all_metadata();

    // Calculate effect counts
    let mut effect_counts = HashMap::new();
    effect_counts.insert("masks".to_string(), effects.masks.len());
    effect_counts.insert("filters".to_string(), effects.filters.len());
    effect_counts.insert("samplers".to_string(), effects.samplers.len());
    effect_counts.insert("shaders".to_string(), effects.shaders.len());
    effect_counts.insert("styles".to_string(), effects.styles.len());
    effect_counts.insert("content".to_string(), effects.content.len());
    effect_counts.insert("shadows".to_string(), effects.shadows.len());

    ApiMetadata {
        effects,
        entry_points: extract_entry_points(),
        core_types: extract_core_types(),
        enums: extract_supporting_enums(),
        effect_counts,
    }
}

/// Extract entry point function documentation.
fn extract_entry_points() -> Vec<FunctionDoc> {
    vec![
        FunctionDoc {
            name: "render_pipeline".to_string(),
            signature: r#"pub fn render_pipeline(
    source: &dyn Grid,
    dest: &mut dyn Grid,
    width: usize,
    height: usize,
    offset_x: usize,
    offset_y: usize,
    options: CompositionOptions<'_>,
    inspector: Option<&mut dyn CompositorInspector>,
)"#
            .to_string(),
            description: "Full superset entry point accepting CompositionOptions".to_string(),
        },
        FunctionDoc {
            name: "render_pipeline_with_area".to_string(),
            signature: r#"pub fn render_pipeline_with_area(
    source: &dyn Grid,
    dest: &mut dyn Grid,
    area: RenderArea,
    options: CompositionOptions<'_>,
    inspector: Option<&mut dyn CompositorInspector>,
)"#
            .to_string(),
            description: "Convenience overload using RenderArea".to_string(),
        },
        FunctionDoc {
            name: "render_pipeline_with_spec".to_string(),
            signature: r#"pub fn render_pipeline_with_spec(
    source: &dyn Grid,
    dest: &mut dyn Grid,
    width: usize,
    height: usize,
    offset_x: usize,
    offset_y: usize,
    spec: &CompositionSpec,
    inspector: Option<&mut dyn CompositorInspector>,
)"#
            .to_string(),
            description: "Data-driven entry point using serializable CompositionSpec".to_string(),
        },
        FunctionDoc {
            name: "render_pipeline_with_spec_area".to_string(),
            signature: r#"pub fn render_pipeline_with_spec_area(
    source: &dyn Grid,
    dest: &mut dyn Grid,
    area: RenderArea,
    spec: &CompositionSpec,
    inspector: Option<&mut dyn CompositorInspector>,
)"#
            .to_string(),
            description: "Spec variant with RenderArea convenience".to_string(),
        },
    ]
}

/// Extract core type documentation.
fn extract_core_types() -> Vec<TypeDoc> {
    vec![
        TypeDoc {
            name: "CompositionOptions".to_string(),
            description: "Runtime superset configuration for the compositor pipeline".to_string(),
            fields: vec![
                FieldDoc {
                    name: "sampler_spec".to_string(),
                    ty: "Option<SamplerSpec>".to_string(),
                    description: "Optional coordinate distortion".to_string(),
                },
                FieldDoc {
                    name: "masks".to_string(),
                    ty: "Cow<'a, [MaskSpec]>".to_string(),
                    description: "Array of masks".to_string(),
                },
                FieldDoc {
                    name: "mask_combine_mode".to_string(),
                    ty: "MaskCombineMode".to_string(),
                    description: "How to combine multiple masks".to_string(),
                },
                FieldDoc {
                    name: "filters".to_string(),
                    ty: "Cow<'a, [FilterSpec]>".to_string(),
                    description: "Array of post-processing filters".to_string(),
                },
                FieldDoc {
                    name: "shader_layers".to_string(),
                    ty: "SmallVec<[ShaderWithRegion<'a>; 2]>".to_string(),
                    description: "Per-region shader layers".to_string(),
                },
                FieldDoc {
                    name: "shadow".to_string(),
                    ty: "Option<ShadowSpec>".to_string(),
                    description: "Optional integrated shadow".to_string(),
                },
                FieldDoc {
                    name: "preserve_unfilled".to_string(),
                    ty: "bool".to_string(),
                    description: "Whether to preserve unfilled cells".to_string(),
                },
                FieldDoc {
                    name: "t".to_string(),
                    ty: "f64".to_string(),
                    description: "Animation progress 0.0-1.0".to_string(),
                },
                FieldDoc {
                    name: "loop_t".to_string(),
                    ty: "Option<f64>".to_string(),
                    description: "Cyclical time for continuous effects".to_string(),
                },
                FieldDoc {
                    name: "phase".to_string(),
                    ty: "Option<Phase>".to_string(),
                    description: "Entering/Dwelling/Exiting/Finished".to_string(),
                },
            ],
            code_block: Some(
                r#"pub struct CompositionOptions<'a> {
    pub sampler_spec: Option<SamplerSpec>,
    pub masks: Cow<'a, [MaskSpec]>,
    pub mask_combine_mode: MaskCombineMode,
    pub filters: Cow<'a, [FilterSpec]>,
    pub shader_layers: SmallVec<[ShaderWithRegion<'a>; 2]>,
    pub shadow: Option<ShadowSpec>,
    pub preserve_unfilled: bool,
    pub t: f64,
    pub loop_t: Option<f64>,
    pub phase: Option<Phase>,
}"#
                .to_string(),
            ),
        },
        TypeDoc {
            name: "CompositionSpec".to_string(),
            description: "Serializable configuration for the compositor pipeline".to_string(),
            fields: vec![
                FieldDoc {
                    name: "sampler_spec".to_string(),
                    ty: "Option<SamplerSpec>".to_string(),
                    description: "Optional coordinate distortion".to_string(),
                },
                FieldDoc {
                    name: "masks".to_string(),
                    ty: "Vec<MaskSpec>".to_string(),
                    description: "Array of masks".to_string(),
                },
                FieldDoc {
                    name: "mask_combine_mode".to_string(),
                    ty: "MaskCombineMode".to_string(),
                    description: "How to combine multiple masks".to_string(),
                },
                FieldDoc {
                    name: "filters".to_string(),
                    ty: "Vec<FilterSpec>".to_string(),
                    description: "Array of post-processing filters".to_string(),
                },
                FieldDoc {
                    name: "shader_layers".to_string(),
                    ty: "Vec<ShaderLayerSpec>".to_string(),
                    description: "Per-region shader layers".to_string(),
                },
                FieldDoc {
                    name: "shadow".to_string(),
                    ty: "Option<ShadowSpec>".to_string(),
                    description: "Optional integrated shadow".to_string(),
                },
                FieldDoc {
                    name: "preserve_unfilled".to_string(),
                    ty: "bool".to_string(),
                    description: "Whether to preserve unfilled cells".to_string(),
                },
                FieldDoc {
                    name: "t".to_string(),
                    ty: "f64".to_string(),
                    description: "Animation progress 0.0-1.0".to_string(),
                },
                FieldDoc {
                    name: "loop_t".to_string(),
                    ty: "Option<f64>".to_string(),
                    description: "Cyclical time for continuous effects".to_string(),
                },
                FieldDoc {
                    name: "phase".to_string(),
                    ty: "Option<Phase>".to_string(),
                    description: "Entering/Dwelling/Exiting/Finished".to_string(),
                },
            ],
            code_block: Some(
                r#"pub struct CompositionSpec {
    pub sampler_spec: Option<SamplerSpec>,
    pub masks: Vec<MaskSpec>,
    pub mask_combine_mode: MaskCombineMode,
    pub filters: Vec<FilterSpec>,
    pub shader_layers: Vec<ShaderLayerSpec>,
    pub shadow: Option<ShadowSpec>,
    pub preserve_unfilled: bool,
    pub t: f64,
    pub loop_t: Option<f64>,
    pub phase: Option<Phase>,
}"#
                .to_string(),
            ),
        },
        TypeDoc {
            name: "ShaderWithRegion".to_string(),
            description: "Runtime shader instance with region".to_string(),
            fields: vec![
                FieldDoc {
                    name: "shader".to_string(),
                    ty: "&'a dyn StyleShader".to_string(),
                    description: "The shader implementation".to_string(),
                },
                FieldDoc {
                    name: "region".to_string(),
                    ty: "StyleRegion".to_string(),
                    description: "Region to apply the shader".to_string(),
                },
            ],
            code_block: Some(
                r#"pub struct ShaderWithRegion<'a> {
    pub shader: &'a dyn StyleShader,
    pub region: StyleRegion,
}"#
                .to_string(),
            ),
        },
        TypeDoc {
            name: "ShaderLayerSpec".to_string(),
            description: "Serializable shader layer specification".to_string(),
            fields: vec![
                FieldDoc {
                    name: "shader".to_string(),
                    ty: "SpatialShaderType".to_string(),
                    description: "The shader type".to_string(),
                },
                FieldDoc {
                    name: "region".to_string(),
                    ty: "StyleRegion".to_string(),
                    description: "Region to apply the shader".to_string(),
                },
            ],
            code_block: Some(
                r#"pub struct ShaderLayerSpec {
    pub shader: SpatialShaderType,
    pub region: StyleRegion,
}"#
                .to_string(),
            ),
        },
        TypeDoc {
            name: "ShadowConfig".to_string(),
            description: "Configuration for shadow rendering".to_string(),
            fields: vec![
                FieldDoc {
                    name: "style".to_string(),
                    ty: "ShadowStyle".to_string(),
                    description: "Shadow rendering style".to_string(),
                },
                FieldDoc {
                    name: "offset_x".to_string(),
                    ty: "i8".to_string(),
                    description: "Horizontal offset".to_string(),
                },
                FieldDoc {
                    name: "offset_y".to_string(),
                    ty: "i8".to_string(),
                    description: "Vertical offset".to_string(),
                },
                FieldDoc {
                    name: "color".to_string(),
                    ty: "Color".to_string(),
                    description: "Shadow color".to_string(),
                },
                FieldDoc {
                    name: "surface_color".to_string(),
                    ty: "Option<Color>".to_string(),
                    description: "Background for blending".to_string(),
                },
                FieldDoc {
                    name: "edges".to_string(),
                    ty: "ShadowEdges".to_string(),
                    description: "Which edges to render".to_string(),
                },
                FieldDoc {
                    name: "soft_edges".to_string(),
                    ty: "bool".to_string(),
                    description: "Use half-blocks for smoothness".to_string(),
                },
                FieldDoc {
                    name: "composite_mode".to_string(),
                    ty: "ShadowCompositeMode".to_string(),
                    description: "Compositing mode (glyph overlay or grade underlying)".to_string(),
                },
                FieldDoc {
                    name: "grade".to_string(),
                    ty: "Option<ShadowGradeConfig>".to_string(),
                    description: "Color grading parameters for grade-underlying mode".to_string(),
                },
            ],
            code_block: Some(
                r#"pub struct ShadowConfig {
    pub style: ShadowStyle,
    pub offset_x: i8,
    pub offset_y: i8,
    pub color: Color,
    pub surface_color: Option<Color>,
    pub edges: ShadowEdges,
    pub soft_edges: bool,
    pub composite_mode: ShadowCompositeMode,
    pub grade: Option<ShadowGradeConfig>,
}"#
                .to_string(),
            ),
        },
    ]
}

/// Extract supporting enum documentation organized by category.
fn extract_supporting_enums() -> HashMap<String, Vec<EnumDoc>> {
    let mut enums = HashMap::new();

    // Mask-related enums
    enums.insert(
        "masks".to_string(),
        vec![
            EnumDoc {
                name: "MaskCombineMode".to_string(),
                description: "How to combine multiple masks".to_string(),
                variants: vec![
                    VariantDoc {
                        name: "All".to_string(),
                        parameters: String::new(),
                        description: "Visible only if ALL masks pass (AND)".to_string(),
                    },
                    VariantDoc {
                        name: "Any".to_string(),
                        parameters: String::new(),
                        description: "Visible if ANY mask passes (OR)".to_string(),
                    },
                    VariantDoc {
                        name: "Blend".to_string(),
                        parameters: "{ ratio }".to_string(),
                        description: "Smooth blend between masks (ratio 0.0 → 1.0)".to_string(),
                    },
                ],
            },
            EnumDoc {
                name: "WipeDirection".to_string(),
                description: "Direction for wipe transitions (16 variants)".to_string(),
                variants: vec![
                    // Cardinal
                    VariantDoc {
                        name: "LeftToRight".to_string(),
                        parameters: String::new(),
                        description: "Cardinal".to_string(),
                    },
                    VariantDoc {
                        name: "RightToLeft".to_string(),
                        parameters: String::new(),
                        description: "Cardinal".to_string(),
                    },
                    VariantDoc {
                        name: "TopToBottom".to_string(),
                        parameters: String::new(),
                        description: "Cardinal".to_string(),
                    },
                    VariantDoc {
                        name: "BottomToTop".to_string(),
                        parameters: String::new(),
                        description: "Cardinal".to_string(),
                    },
                    // Diagonal
                    VariantDoc {
                        name: "TopLeftToBottomRight".to_string(),
                        parameters: String::new(),
                        description: "Diagonal".to_string(),
                    },
                    VariantDoc {
                        name: "TopRightToBottomLeft".to_string(),
                        parameters: String::new(),
                        description: "Diagonal".to_string(),
                    },
                    VariantDoc {
                        name: "BottomLeftToTopRight".to_string(),
                        parameters: String::new(),
                        description: "Diagonal".to_string(),
                    },
                    VariantDoc {
                        name: "BottomRightToTopLeft".to_string(),
                        parameters: String::new(),
                        description: "Diagonal".to_string(),
                    },
                    // Aliases
                    VariantDoc {
                        name: "FromLeft".to_string(),
                        parameters: String::new(),
                        description: "Alias".to_string(),
                    },
                    VariantDoc {
                        name: "FromRight".to_string(),
                        parameters: String::new(),
                        description: "Alias".to_string(),
                    },
                    VariantDoc {
                        name: "FromTop".to_string(),
                        parameters: String::new(),
                        description: "Alias".to_string(),
                    },
                    VariantDoc {
                        name: "FromBottom".to_string(),
                        parameters: String::new(),
                        description: "Alias".to_string(),
                    },
                    // Center-out
                    VariantDoc {
                        name: "HorizontalCenterOut".to_string(),
                        parameters: String::new(),
                        description: "Center-out (curtains opening)".to_string(),
                    },
                    VariantDoc {
                        name: "VerticalCenterOut".to_string(),
                        parameters: String::new(),
                        description: "Center-out (curtains opening)".to_string(),
                    },
                    // Edges-in
                    VariantDoc {
                        name: "HorizontalEdgesIn".to_string(),
                        parameters: String::new(),
                        description: "Edges-in (curtains closing)".to_string(),
                    },
                    VariantDoc {
                        name: "VerticalEdgesIn".to_string(),
                        parameters: String::new(),
                        description: "Edges-in (curtains closing)".to_string(),
                    },
                ],
            },
            EnumDoc {
                name: "Orientation".to_string(),
                description: "Orientation for effects".to_string(),
                variants: vec![
                    VariantDoc {
                        name: "Horizontal".to_string(),
                        parameters: String::new(),
                        description: String::new(),
                    },
                    VariantDoc {
                        name: "Vertical".to_string(),
                        parameters: String::new(),
                        description: String::new(),
                    },
                ],
            },
            EnumDoc {
                name: "IrisShape".to_string(),
                description: "Shape for iris mask".to_string(),
                variants: vec![
                    VariantDoc {
                        name: "Circle".to_string(),
                        parameters: String::new(),
                        description: String::new(),
                    },
                    VariantDoc {
                        name: "Diamond".to_string(),
                        parameters: String::new(),
                        description: String::new(),
                    },
                    VariantDoc {
                        name: "Box".to_string(),
                        parameters: String::new(),
                        description: String::new(),
                    },
                ],
            },
            EnumDoc {
                name: "DitherMatrix".to_string(),
                description: "Dither matrix for noise dither mask".to_string(),
                variants: vec![
                    VariantDoc {
                        name: "Bayer4".to_string(),
                        parameters: String::new(),
                        description: String::new(),
                    },
                    VariantDoc {
                        name: "Bayer8".to_string(),
                        parameters: String::new(),
                        description: String::new(),
                    },
                ],
            },
            EnumDoc {
                name: "RadialOrigin".to_string(),
                description: "Origin point for radial effects".to_string(),
                variants: vec![
                    VariantDoc {
                        name: "Center".to_string(),
                        parameters: String::new(),
                        description: String::new(),
                    },
                    VariantDoc {
                        name: "TopLeft".to_string(),
                        parameters: String::new(),
                        description: String::new(),
                    },
                    VariantDoc {
                        name: "TopRight".to_string(),
                        parameters: String::new(),
                        description: String::new(),
                    },
                    VariantDoc {
                        name: "BottomLeft".to_string(),
                        parameters: String::new(),
                        description: String::new(),
                    },
                    VariantDoc {
                        name: "BottomRight".to_string(),
                        parameters: String::new(),
                        description: String::new(),
                    },
                    VariantDoc {
                        name: "Custom".to_string(),
                        parameters: "{ x, y }".to_string(),
                        description: String::new(),
                    },
                ],
            },
            EnumDoc {
                name: "CellularPattern".to_string(),
                description: "Pattern for cellular mask".to_string(),
                variants: vec![
                    VariantDoc {
                        name: "Voronoi".to_string(),
                        parameters: String::new(),
                        description: String::new(),
                    },
                    VariantDoc {
                        name: "Hexagonal".to_string(),
                        parameters: String::new(),
                        description: String::new(),
                    },
                    VariantDoc {
                        name: "Organic".to_string(),
                        parameters: String::new(),
                        description: String::new(),
                    },
                ],
            },
        ],
    );

    // Filter-related enums
    enums.insert(
        "filters".to_string(),
        vec![
            EnumDoc {
                name: "ApplyTo".to_string(),
                description: "Which color channels to apply effect to".to_string(),
                variants: vec![
                    VariantDoc {
                        name: "Foreground".to_string(),
                        parameters: String::new(),
                        description: "Alias: fg".to_string(),
                    },
                    VariantDoc {
                        name: "Background".to_string(),
                        parameters: String::new(),
                        description: "Alias: bg".to_string(),
                    },
                    VariantDoc {
                        name: "Both".to_string(),
                        parameters: String::new(),
                        description: "Default".to_string(),
                    },
                ],
            },
            EnumDoc {
                name: "PatternType".to_string(),
                description: "Pattern for pattern fill filter".to_string(),
                variants: vec![
                    VariantDoc {
                        name: "Single".to_string(),
                        parameters: "{ char }".to_string(),
                        description: String::new(),
                    },
                    VariantDoc {
                        name: "Checkerboard".to_string(),
                        parameters: "{ char_a, char_b }".to_string(),
                        description: String::new(),
                    },
                    VariantDoc {
                        name: "HorizontalLines".to_string(),
                        parameters: "{ line_char, spacing }".to_string(),
                        description: String::new(),
                    },
                    VariantDoc {
                        name: "VerticalLines".to_string(),
                        parameters: "{ line_char, spacing }".to_string(),
                        description: String::new(),
                    },
                ],
            },
            EnumDoc {
                name: "BraillePatternType".to_string(),
                description: "Pattern for braille dust filter".to_string(),
                variants: vec![
                    VariantDoc {
                        name: "SingleDot".to_string(),
                        parameters: String::new(),
                        description: String::new(),
                    },
                    VariantDoc {
                        name: "OneToTwoDots".to_string(),
                        parameters: String::new(),
                        description: String::new(),
                    },
                    VariantDoc {
                        name: "OneToThreeDots".to_string(),
                        parameters: String::new(),
                        description: String::new(),
                    },
                    VariantDoc {
                        name: "OneToFourDots".to_string(),
                        parameters: String::new(),
                        description: String::new(),
                    },
                ],
            },
            EnumDoc {
                name: "MotionBlurDirection".to_string(),
                description: "Direction for motion blur".to_string(),
                variants: vec![
                    VariantDoc {
                        name: "Left".to_string(),
                        parameters: String::new(),
                        description: String::new(),
                    },
                    VariantDoc {
                        name: "Right".to_string(),
                        parameters: String::new(),
                        description: String::new(),
                    },
                    VariantDoc {
                        name: "Up".to_string(),
                        parameters: String::new(),
                        description: String::new(),
                    },
                    VariantDoc {
                        name: "Down".to_string(),
                        parameters: String::new(),
                        description: String::new(),
                    },
                ],
            },
            EnumDoc {
                name: "SubPixelBarDirection".to_string(),
                description: "Direction for sub-pixel progress bar".to_string(),
                variants: vec![
                    VariantDoc {
                        name: "Horizontal".to_string(),
                        parameters: String::new(),
                        description: "▏▎▍▌▋▊▉█".to_string(),
                    },
                    VariantDoc {
                        name: "Vertical".to_string(),
                        parameters: String::new(),
                        description: "▁▂▃▄▅▆▇█".to_string(),
                    },
                ],
            },
        ],
    );

    // Sampler-related enums
    enums.insert(
        "samplers".to_string(),
        vec![
            EnumDoc {
                name: "Axis".to_string(),
                description: "Axis for wave effects".to_string(),
                variants: vec![
                    VariantDoc {
                        name: "X".to_string(),
                        parameters: String::new(),
                        description: String::new(),
                    },
                    VariantDoc {
                        name: "Y".to_string(),
                        parameters: String::new(),
                        description: String::new(),
                    },
                ],
            },
            EnumDoc {
                name: "RippleCenter".to_string(),
                description: "Center point for ripple effect".to_string(),
                variants: vec![
                    VariantDoc {
                        name: "Center".to_string(),
                        parameters: String::new(),
                        description: String::new(),
                    },
                    VariantDoc {
                        name: "Point".to_string(),
                        parameters: "{ x, y }".to_string(),
                        description: String::new(),
                    },
                ],
            },
        ],
    );

    // Shadow-related enums
    enums.insert(
        "shadows".to_string(),
        vec![EnumDoc {
            name: "ShadowEdges".to_string(),
            description: "Which edges to render shadows on (bitflags)".to_string(),
            variants: vec![
                VariantDoc {
                    name: "RIGHT".to_string(),
                    parameters: String::new(),
                    description: String::new(),
                },
                VariantDoc {
                    name: "BOTTOM".to_string(),
                    parameters: String::new(),
                    description: String::new(),
                },
                VariantDoc {
                    name: "LEFT".to_string(),
                    parameters: String::new(),
                    description: String::new(),
                },
                VariantDoc {
                    name: "TOP".to_string(),
                    parameters: String::new(),
                    description: String::new(),
                },
                VariantDoc {
                    name: "BOTTOM_RIGHT".to_string(),
                    parameters: String::new(),
                    description: "Convenience".to_string(),
                },
                VariantDoc {
                    name: "TOP_LEFT".to_string(),
                    parameters: String::new(),
                    description: "Convenience".to_string(),
                },
                VariantDoc {
                    name: "ALL".to_string(),
                    parameters: String::new(),
                    description: "Convenience".to_string(),
                },
            ],
        }],
    );

    enums
}

// <FILE>xtask/src/docs/api_metadata.rs</FILE> - <DESC>API metadata extraction from runtime introspection</DESC>
// <VERS>END OF VERSION: 1.2.0</VERS>
