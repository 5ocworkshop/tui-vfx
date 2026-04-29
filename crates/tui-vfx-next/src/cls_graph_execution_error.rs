// <FILE>crates/tui-vfx-next/src/cls_graph_execution_error.rs</FILE> - <DESC>Proof graph execution failure enum</DESC>
// <VERS>VERSION: 0.3.0</VERS>
// <WCTX>New kernel Phase G4: report graph value bus execution conflicts.</WCTX>
// <CLOG>0.3.0: MINOR — add missing graph value and parallel value conflict errors.
// 0.2.0: MINOR — add same-channel parallel merge conflict error.
// 0.1.0: INIT — add structured graph execution errors for proof tests.</CLOG>

use crate::{
    CellChannel, DescriptorValidationError, EffectId, EffectInputId, GraphValueId, NodeId,
    ParameterId, SignalId, ValueKind,
};

/// Structured failure returned by proof graph execution.
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(tag = "kind", rename_all = "camelCase", deny_unknown_fields)]
pub enum GraphExecutionError {
    /// Graph validation failed before execution began.
    GraphValidation {
        /// Underlying contract validation error.
        error: DescriptorValidationError,
    },
    /// No proof adapter was registered for a graph effect id.
    MissingProofAdapter {
        /// Effect id with no proof adapter.
        effect: EffectId,
    },
    /// Parameter source had no snapshot value and no parameter default.
    MissingParameterValue {
        /// Parameter id that could not resolve.
        id: ParameterId,
    },
    /// Signal source had no snapshot value, fallback, or signal default.
    MissingSignalValue {
        /// Signal id that could not resolve.
        id: SignalId,
    },
    /// Value source map resolved from a non-numeric value.
    NonNumericResolvedMapSource {
        /// Actual resolved value kind.
        actual: ValueKind,
    },
    /// Graph-local value was not available in this node's value-bus snapshot.
    MissingGraphValue {
        /// Missing graph value id.
        id: GraphValueId,
    },

    /// Proof adapter required an input that was not resolved.
    MissingProofInput {
        /// Effect id being executed.
        effect: EffectId,
        /// Input id required by the proof adapter.
        input: EffectInputId,
    },
    /// Proof adapter received an input with an unsupported value kind.
    UnsupportedProofInput {
        /// Effect id being executed.
        effect: EffectId,
        /// Input id that failed adapter conversion.
        input: EffectInputId,
        /// Expected input kind.
        expected: ValueKind,
        /// Actual input kind.
        actual: ValueKind,
    },
    /// Parallel branches wrote the same cell channel under error-on-conflict policy.
    ParallelMergeConflict {
        /// Destination x coordinate.
        x: usize,
        /// Destination y coordinate.
        y: usize,
        /// Cell channel written by more than one branch.
        channel: CellChannel,
        /// Node that first wrote this channel.
        prior_node: NodeId,
        /// Later node that conflicted with the prior write.
        conflicting_node: NodeId,
    },
    /// Parallel branches published the same graph value under error-on-conflict policy.
    ParallelValueMergeConflict {
        /// Graph value id published by more than one branch.
        id: GraphValueId,
        /// Node that first published this value.
        prior_node: NodeId,
        /// Later node that conflicted with the prior publication.
        conflicting_node: NodeId,
    },
}

impl From<DescriptorValidationError> for GraphExecutionError {
    fn from(error: DescriptorValidationError) -> Self {
        Self::GraphValidation { error }
    }
}

// <FILE>crates/tui-vfx-next/src/cls_graph_execution_error.rs</FILE> - <DESC>Proof graph execution failure enum</DESC>
// <VERS>END OF VERSION: 0.3.0</VERS>
