// <FILE>crates/tui-vfx-compost/src/loader/cls_load_error.rs</FILE> - <DESC>Native v3.1 recipe load diagnostics</DESC>
// <VERS>VERSION: 0.7.0</VERS>
// <WCTX>Load errors describe canonical v3.1 acceptance failures for currently supported substrate behavior.</WCTX>
// <CLOG>0.7.0: MINOR — add batched substrate rejection diagnostics.</CLOG>

use std::error::Error;
use std::fmt;

/// Error returned while accepting a canonical v3.1 recipe for compost rendering.
#[derive(Debug)]
pub enum LoadError {
    /// The recipe or graph version is not the native v3.1 contract.
    UnsupportedVersion {
        /// Recipe document version.
        recipe_version: String,
        /// Graph document version.
        graph_version: String,
    },

    /// The canonical contract validator rejected the recipe.
    Contract {
        /// Debug-formatted contract validation error.
        message: String,
    },

    /// A source input is currently unsupported by the native direct renderer.
    UnsupportedSourceInput {
        /// Source instance id.
        source_id: String,
        /// Source input id.
        input: String,
        /// Human-readable reason.
        reason: String,
    },

    /// A source descriptor is currently unsupported by the native direct renderer.
    UnsupportedSourceDescriptor {
        /// Source instance id.
        source_id: String,
        /// Source descriptor id.
        descriptor: String,
        /// Human-readable reason.
        reason: String,
    },

    /// A graph node input is currently unsupported by the native direct renderer.
    UnsupportedInput {
        /// Graph-local node id.
        node_id: String,
        /// Effect id.
        effect: String,
        /// Effect input id.
        input: String,
        /// Human-readable reason.
        reason: String,
    },

    /// A scene element policy is currently unsupported by the native direct renderer.
    UnsupportedSceneElementPolicy {
        /// Scene element id.
        element_id: String,
        /// Scene element policy field.
        policy: String,
        /// Human-readable reason.
        reason: String,
    },

    /// The native renderer does not yet support this effect.
    UnsupportedEffect {
        /// Graph-local node id.
        node_id: String,
        /// Effect id.
        effect: String,
    },

    /// The native renderer does not yet support this effect family.
    UnsupportedEffectFamily {
        /// Graph-local node id.
        node_id: String,
        /// Effect id.
        effect: String,
        /// Canonical effect family prefix.
        family: String,
        /// Human-readable reason.
        reason: String,
    },

    /// A graph node timing/lifecycle field is currently unsupported.
    UnsupportedNodeTiming {
        /// Graph-local node id.
        node_id: String,
        /// Effect id.
        effect: String,
        /// Timing/lifecycle field.
        field: String,
        /// Human-readable reason.
        reason: String,
    },

    /// A graph merge policy is currently unsupported by native compost rendering.
    UnsupportedGraphMergePolicy {
        /// Graph field containing the unsupported merge policy.
        field: String,
        /// Human-readable reason.
        reason: String,
    },

    /// A node-local write policy is currently unsupported by native compost rendering.
    UnsupportedNodeWritePolicy {
        /// Graph-local node id.
        node_id: String,
        /// Effect id.
        effect: String,
        /// Node write policy field.
        field: String,
        /// Human-readable reason.
        reason: String,
    },
}

impl fmt::Display for LoadError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::UnsupportedVersion {
                recipe_version,
                graph_version,
            } => write!(
                formatter,
                "unsupported recipe/graph version: recipe={recipe_version}, graph={graph_version}"
            ),
            Self::Contract { message } => {
                write!(formatter, "contract validation failed: {message}")
            }
            Self::UnsupportedSourceInput {
                source_id,
                input,
                reason,
            } => write!(
                formatter,
                "unsupported source input {source_id}.{input}: {reason}"
            ),
            Self::UnsupportedSourceDescriptor {
                source_id,
                descriptor,
                reason,
            } => write!(
                formatter,
                "unsupported source descriptor {source_id}: {descriptor}: {reason}"
            ),
            Self::UnsupportedInput {
                node_id,
                effect,
                input,
                reason,
            } => write!(
                formatter,
                "unsupported input {node_id}.{input} for {effect}: {reason}"
            ),
            Self::UnsupportedSceneElementPolicy {
                element_id,
                policy,
                reason,
            } => write!(
                formatter,
                "unsupported scene element policy {element_id}.{policy}: {reason}"
            ),
            Self::UnsupportedEffect { node_id, effect } => {
                write!(formatter, "unsupported effect {node_id}: {effect}")
            }
            Self::UnsupportedEffectFamily {
                node_id,
                effect,
                family,
                reason,
            } => write!(
                formatter,
                "unsupported {family} effect family {node_id}: {effect}: {reason}"
            ),
            Self::UnsupportedNodeTiming {
                node_id,
                effect,
                field,
                reason,
            } => write!(
                formatter,
                "unsupported node timing {node_id}.{field} for {effect}: {reason}"
            ),
            Self::UnsupportedGraphMergePolicy { field, reason } => {
                write!(
                    formatter,
                    "unsupported graph merge policy {field}: {reason}"
                )
            }
            Self::UnsupportedNodeWritePolicy {
                node_id,
                effect,
                field,
                reason,
            } => write!(
                formatter,
                "unsupported node write policy {node_id}.{field} for {effect}: {reason}"
            ),
        }
    }
}

impl Error for LoadError {}

impl From<tui_vfx_contract::DescriptorValidationError> for LoadError {
    fn from(value: tui_vfx_contract::DescriptorValidationError) -> Self {
        Self::Contract {
            message: format!("{value:?}"),
        }
    }
}

// <FILE>crates/tui-vfx-compost/src/loader/cls_load_error.rs</FILE> - <DESC>Native v3.1 recipe load diagnostics</DESC>
// <VERS>END OF VERSION: 0.7.0</VERS>
