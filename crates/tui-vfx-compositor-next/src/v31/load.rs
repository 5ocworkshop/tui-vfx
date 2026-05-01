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
        if node.effect.as_str() == "shader.linearGradient" {
            validate_linear_gradient_direct_inputs(node_id, node)?;
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
    for input in ["message", "text", "width", "height"] {
        if source.inputs.contains_key(&SourceInputId::new(input)) {
            literal_source_input(source, input)
                .map_err(|error| source_input_error(source_id, source, input, error.as_str()))?;
        }
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
    for input in node.inputs.keys() {
        literal_value(node, input.as_str()).map_err(|error| {
            V31LoadError::UnsupportedDirectInput {
                node_id: node_id.as_str().to_string(),
                effect: node.effect.as_str().to_string(),
                input: input.as_str().to_string(),
                reason: match error {
                    V31RenderError::Unsupported(reason) => reason,
                },
            }
        })?;
    }

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
    literal_value(node, input)
        .map(|_| ())
        .map_err(|error| V31LoadError::UnsupportedDirectInput {
            node_id: node_id.as_str().to_string(),
            effect: node.effect.as_str().to_string(),
            input: input.to_string(),
            reason: match error {
                V31RenderError::Unsupported(reason) => reason,
            },
        })
}

// <FILE>crates/tui-vfx-compositor-next/src/v31/load.rs</FILE> - <DESC>Direct v3.1 recipe load validation</DESC>
// <VERS>END OF VERSION: 0.1.0</VERS>
