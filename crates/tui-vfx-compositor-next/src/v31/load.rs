// <FILE>crates/tui-vfx-compositor-next/src/v31/load.rs</FILE> - <DESC>Direct v3.1 recipe load validation</DESC>
// <VERS>VERSION: 0.1.0</VERS>
// <WCTX>Validate canonical v3.1 recipes before direct compositor-next rendering.</WCTX>
// <CLOG>0.1.0: INIT — add strict v3.1 and literal-input direct-render acceptance gates.</CLOG>

use tui_vfx_contract::{
    DescriptorCatalog, DescriptorValidationError, EffectInputId, NodeId, NodeSpec, RecipeDocument,
    SourceInputId, SourceInstanceId, SourceSpec, Value, ValueSource,
};

use super::render::V31RenderError;

/// Canonical v3.1 recipe accepted after load-time validation.
#[derive(Clone, Debug, PartialEq)]
pub struct LoadedV31Recipe {
    recipe: RecipeDocument,
}

impl LoadedV31Recipe {
    /// Validate a canonical v3.1 recipe once at load time.
    pub fn load(recipe: RecipeDocument, catalog: &DescriptorCatalog) -> Result<Self, V31LoadError> {
        recipe.validate_with_catalog(catalog)?;
        if recipe.version != "3.1" || recipe.graph.version != "3.1" {
            return Err(V31LoadError::UnsupportedVersion {
                recipe_version: recipe.version.clone(),
                graph_version: recipe.graph.version.clone(),
            });
        }
        validate_direct_render_contract(&recipe)?;
        Ok(Self { recipe })
    }

    /// Borrow the validated canonical recipe document.
    pub fn recipe(&self) -> &RecipeDocument {
        &self.recipe
    }
}

/// Error returned while accepting a recipe into the direct v3.1 renderer.
#[derive(Clone, Debug, PartialEq)]
pub enum V31LoadError {
    /// Canonical recipe validation failed before compositor-next rendering.
    Validation(DescriptorValidationError),
    /// This module only accepts v3.1 recipe and graph contracts.
    UnsupportedVersion {
        /// Recipe document version.
        recipe_version: String,
        /// Graph contract version.
        graph_version: String,
    },
    /// The first vertical slice only accepts literal inputs it can render directly.
    UnsupportedDirectInput {
        /// Graph node id containing the unsupported input.
        node_id: String,
        /// Effect descriptor id on the graph node.
        effect: String,
        /// Effect input id.
        input: String,
        /// Stable explanation of the unsupported input shape.
        reason: String,
    },
    /// The first vertical slice only accepts literal source inputs it can render directly.
    UnsupportedSourceInput {
        /// Recipe-local source instance id.
        source_id: String,
        /// Source descriptor id on the source instance.
        source: String,
        /// Source input id.
        input: String,
        /// Stable explanation of the unsupported input shape.
        reason: String,
    },
}

impl From<DescriptorValidationError> for V31LoadError {
    fn from(value: DescriptorValidationError) -> Self {
        Self::Validation(value)
    }
}

fn validate_direct_render_contract(recipe: &RecipeDocument) -> Result<(), V31LoadError> {
    for (source_id, source) in &recipe.sources {
        validate_direct_source_inputs(source_id, source)?;
    }
    for (node_id, node) in &recipe.graph.nodes {
        match node.effect.as_str() {
            "shader.linearGradient" => validate_linear_gradient_direct_inputs(node_id, node)?,
            "shader.highlighter" => validate_highlighter_direct_inputs(node_id, node)?,
            "shader.glistenBand" => validate_glisten_band_direct_inputs(node_id, node)?,
            "shader.focusField" => validate_focus_field_direct_inputs(node_id, node)?,
            _ => {}
        }
    }
    Ok(())
}

fn validate_direct_source_inputs(
    source_id: &SourceInstanceId,
    source: &SourceSpec,
) -> Result<(), V31LoadError> {
    let has_message = literal_source_input(source, "message").is_ok();
    let has_text = literal_source_input(source, "text").is_ok();
    if !has_message && !has_text {
        return Err(source_input_error(
            source_id,
            source,
            "message",
            "Direct v3.1 rendering requires a literal message/text source input.",
        ));
    }
    for input in source.inputs.keys() {
        literal_source_input(source, input.as_str()).map_err(|error| {
            source_input_error(source_id, source, input.as_str(), error.as_str())
        })?;
    }
    for input in ["width", "height"] {
        if source.inputs.contains_key(&SourceInputId::new(input))
            && literal_source_input(source, input)
                .ok()
                .and_then(Value::as_range_number)
                .is_none()
        {
            return Err(source_input_error(
                source_id,
                source,
                input,
                "Direct v3.1 rendering requires literal numeric source dimensions.",
            ));
        }
    }
    Ok(())
}

fn literal_source_input<'a>(source: &'a SourceSpec, id: &str) -> Result<&'a Value, String> {
    match source.inputs.get(&SourceInputId::new(id)) {
        Some(ValueSource::Literal { value }) => Ok(value),
        Some(_) => Err(format!(
            "Direct v3.1 rendering requires literal source input `{id}`."
        )),
        None => Err(format!(
            "Direct v3.1 rendering requires source input `{id}`."
        )),
    }
}

fn source_input_error(
    source_id: &SourceInstanceId,
    source: &SourceSpec,
    input: &str,
    reason: &str,
) -> V31LoadError {
    V31LoadError::UnsupportedSourceInput {
        source_id: source_id.as_str().to_string(),
        source: source.source.as_str().to_string(),
        input: input.to_string(),
        reason: reason.to_string(),
    }
}

fn literal_value<'a>(node: &'a NodeSpec, id: &str) -> Result<&'a Value, V31RenderError> {
    match node.inputs.get(&EffectInputId::new(id)) {
        Some(ValueSource::Literal { value }) => Ok(value),
        Some(_) => Err(V31RenderError::Unsupported(format!(
            "Direct v3.1 rendering requires literal input `{id}` for `{}`.",
            node.effect.as_str()
        ))),
        None => Err(V31RenderError::Unsupported(format!(
            "Direct v3.1 rendering requires input `{id}` for `{}`.",
            node.effect.as_str()
        ))),
    }
}

fn validate_linear_gradient_direct_inputs(
    node_id: &NodeId,
    node: &NodeSpec,
) -> Result<(), V31LoadError> {
    require_declared_inputs_literal(node_id, node)?;

    require_literal_input(node_id, node, "angleDeg")?;
    require_literal_input(node_id, node, "intensity")?;
    require_literal_input(node_id, node, "applyTo")?;
    if node.inputs.contains_key(&EffectInputId::new("gradient")) {
        require_literal_input(node_id, node, "gradient")?;
    } else {
        require_literal_input(node_id, node, "startColor")?;
        require_literal_input(node_id, node, "endColor")?;
        require_literal_input(node_id, node, "colorSpace")?;
    }
    Ok(())
}

fn require_literal_input(
    node_id: &NodeId,
    node: &NodeSpec,
    input: &str,
) -> Result<(), V31LoadError> {
    literal_direct_value(node_id, node, input).map(|_| ())
}

fn validate_glisten_band_direct_inputs(
    node_id: &NodeId,
    node: &NodeSpec,
) -> Result<(), V31LoadError> {
    require_declared_inputs_literal(node_id, node)?;

    require_color_input(node_id, node, "color")?;
    require_integer_valued_number_input(node_id, node, "bandWidth")?;
    for input in ["blendStrength", "angleDeg", "speed"] {
        if node.inputs.contains_key(&EffectInputId::new(input)) {
            require_number_input(node_id, node, input)?;
        }
    }
    if node.inputs.contains_key(&EffectInputId::new("direction")) {
        require_enum_value(node_id, node, "direction", &["leftToRight", "rightToLeft"])?;
    }
    for input in ["head", "tail"] {
        if node.inputs.contains_key(&EffectInputId::new(input)) {
            return Err(direct_input_error(
                node_id,
                node,
                input,
                "shader.glistenBand numeric head/tail band-position fields are not supported by direct v3.1 rendering.",
            ));
        }
    }
    Ok(())
}

fn validate_highlighter_direct_inputs(
    node_id: &NodeId,
    node: &NodeSpec,
) -> Result<(), V31LoadError> {
    require_declared_inputs_literal(node_id, node)?;

    for input in [
        "color",
        "bandWidth",
        "blendStrength",
        "textContrast",
        "mode",
        "softEdge",
        "direction",
        "rowMask",
        "applyTo",
    ] {
        require_literal_input(node_id, node, input)?;
    }
    require_color_input(node_id, node, "color")?;
    require_number_input(node_id, node, "bandWidth")?;
    require_number_input(node_id, node, "blendStrength")?;
    let text_contrast = require_number_input(node_id, node, "textContrast")?;
    if text_contrast > 0.0 {
        return Err(direct_input_error(
            node_id,
            node,
            "textContrast",
            "shader.highlighter textContrast values above 0.0 are not supported by direct v3.1 rendering.",
        ));
    }
    require_enum_value(node_id, node, "mode", &["band"])?;
    require_bool_input(node_id, node, "softEdge")?;
    require_enum_value(
        node_id,
        node,
        "direction",
        &["leftToRight", "rightToLeft", "topToBottom", "bottomToTop"],
    )?;
    require_integer_input(node_id, node, "rowMask")?;
    require_enum_value(
        node_id,
        node,
        "applyTo",
        &["foreground", "background", "both"],
    )?;
    Ok(())
}

fn validate_focus_field_direct_inputs(
    node_id: &NodeId,
    node: &NodeSpec,
) -> Result<(), V31LoadError> {
    require_declared_inputs_literal(node_id, node)?;

    require_color_input(node_id, node, "color")?;
    require_integer_valued_number_input(node_id, node, "centerX")?;
    require_integer_valued_number_input(node_id, node, "centerY")?;
    require_integer_valued_number_input(node_id, node, "radius")?;

    if node.inputs.contains_key(&EffectInputId::new("intensity")) {
        require_number_input(node_id, node, "intensity")?;
    }
    if node.inputs.contains_key(&EffectInputId::new("applyTo")) {
        require_enum_value(
            node_id,
            node,
            "applyTo",
            &["foreground", "background", "both"],
        )?;
    }
    if node.inputs.contains_key(&EffectInputId::new("shape")) {
        require_enum_value(node_id, node, "shape", &["circle", "ellipse"])?;
    }

    for input in [
        "radiusX",
        "radiusY",
        "feather",
        "rectHeight",
        "rectWidth",
        "rectX",
        "rectY",
    ] {
        if node.inputs.contains_key(&EffectInputId::new(input)) {
            return Err(direct_input_error(
                node_id,
                node,
                input,
                &format!(
                    "shader.focusField input `{input}` is not supported by direct v3.1 rendering."
                ),
            ));
        }
    }
    Ok(())
}

fn require_declared_inputs_literal(node_id: &NodeId, node: &NodeSpec) -> Result<(), V31LoadError> {
    for input in node.inputs.keys() {
        literal_direct_value(node_id, node, input.as_str())?;
    }
    Ok(())
}

fn require_color_input(node_id: &NodeId, node: &NodeSpec, input: &str) -> Result<(), V31LoadError> {
    match literal_direct_value(node_id, node, input)? {
        Value::Color(_) => Ok(()),
        value => Err(direct_input_error(
            node_id,
            node,
            input,
            &format!(
                "Direct v3.1 rendering expected color input `{input}` but found `{:?}`.",
                value.kind()
            ),
        )),
    }
}

fn require_number_input(
    node_id: &NodeId,
    node: &NodeSpec,
    input: &str,
) -> Result<f64, V31LoadError> {
    literal_direct_value(node_id, node, input)?
        .as_range_number()
        .ok_or_else(|| {
            direct_input_error(
                node_id,
                node,
                input,
                &format!("Direct v3.1 rendering expected numeric input `{input}`."),
            )
        })
}

fn require_integer_valued_number_input(
    node_id: &NodeId,
    node: &NodeSpec,
    input: &str,
) -> Result<f64, V31LoadError> {
    let value = require_number_input(node_id, node, input)?;
    if value.fract().abs() <= f64::EPSILON {
        Ok(value)
    } else {
        Err(direct_input_error(
            node_id,
            node,
            input,
            &format!(
                "{} `{input}` value `{value}` must be an integer-valued number for direct v3.1 rendering.",
                node.effect.as_str()
            ),
        ))
    }
}

fn require_bool_input(node_id: &NodeId, node: &NodeSpec, input: &str) -> Result<(), V31LoadError> {
    match literal_direct_value(node_id, node, input)? {
        Value::Boolean(_) => Ok(()),
        value => Err(direct_input_error(
            node_id,
            node,
            input,
            &format!(
                "Direct v3.1 rendering expected boolean input `{input}` but found `{:?}`.",
                value.kind()
            ),
        )),
    }
}

fn require_integer_input(
    node_id: &NodeId,
    node: &NodeSpec,
    input: &str,
) -> Result<(), V31LoadError> {
    match literal_direct_value(node_id, node, input)? {
        Value::Integer(_) => Ok(()),
        value => Err(direct_input_error(
            node_id,
            node,
            input,
            &format!(
                "Direct v3.1 rendering expected integer input `{input}` but found `{:?}`.",
                value.kind()
            ),
        )),
    }
}

fn require_enum_value(
    node_id: &NodeId,
    node: &NodeSpec,
    input: &str,
    allowed: &[&str],
) -> Result<(), V31LoadError> {
    match literal_direct_value(node_id, node, input)?.as_enum_value() {
        Some(value) if allowed.contains(&value) => Ok(()),
        Some(value) => Err(direct_input_error(
            node_id,
            node,
            input,
            &format!(
                "{} `{value}` is not supported by direct v3.1 rendering.",
                node.effect.as_str()
            ),
        )),
        None => Err(direct_input_error(
            node_id,
            node,
            input,
            &format!("Direct v3.1 rendering expected enum input `{input}`."),
        )),
    }
}

fn direct_input_error(
    node_id: &NodeId,
    node: &NodeSpec,
    input: &str,
    reason: &str,
) -> V31LoadError {
    V31LoadError::UnsupportedDirectInput {
        node_id: node_id.as_str().to_string(),
        effect: node.effect.as_str().to_string(),
        input: input.to_string(),
        reason: reason.to_string(),
    }
}

fn literal_direct_value<'a>(
    node_id: &NodeId,
    node: &'a NodeSpec,
    input: &str,
) -> Result<&'a Value, V31LoadError> {
    literal_value(node, input).map_err(|error| {
        let reason = match &error {
            V31RenderError::Unsupported(reason) => reason.as_str(),
        };
        direct_input_error(node_id, node, input, reason)
    })
}

// <FILE>crates/tui-vfx-compositor-next/src/v31/load.rs</FILE> - <DESC>Direct v3.1 recipe load validation</DESC>
// <VERS>END OF VERSION: 0.1.0</VERS>
