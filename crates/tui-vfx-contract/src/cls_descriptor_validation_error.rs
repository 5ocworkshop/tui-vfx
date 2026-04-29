// <FILE>crates/tui-vfx-contract/src/cls_descriptor_validation_error.rs</FILE> - <DESC>Descriptor capability validation error enum</DESC>
// <VERS>VERSION: 0.5.0</VERS>
// <WCTX>New kernel Phase G4: report graph value and node output validation failures.</WCTX>
// <CLOG>0.5.0: MINOR — add effect output, graph value, and node output validation errors.
// 0.4.0: MINOR — add graph and node validation errors.
// 0.3.0: MINOR — add parameter, signal, source-kind, map, and binding validation errors.
// 0.2.0: MINOR — add input id, value kind, range, and enum validation errors.
// 0.1.0: INIT — add structured validation errors for scope, write policy, and channel checks.</CLOG>

use crate::{
    CellChannel, CellWritePolicy, EffectId, EffectInputId, EffectOutputId, GraphId, GraphValueId,
    NodeId, ParameterId, RoleWritePolicyKind, ScopeKind, SignalId, ValueKind,
};

/// Structured error returned when a request exceeds descriptor capabilities.
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize, schemars::JsonSchema)]
#[serde(tag = "kind", rename_all = "camelCase", deny_unknown_fields)]
pub enum DescriptorValidationError {
    /// Requested scope kind is not declared in descriptor support.
    UnsupportedScopeKind {
        /// Scope kind requested by the operation.
        requested: ScopeKind,
    },
    /// Requested cell write policy is not declared in descriptor support.
    UnsupportedCellWritePolicy {
        /// Cell write policy requested by the operation.
        requested: CellWritePolicy,
    },
    /// Requested role write policy kind is not declared in descriptor support.
    UnsupportedRoleWritePolicy {
        /// Role write policy kind requested by the operation.
        requested: RoleWritePolicyKind,
    },
    /// Requested channel write is outside descriptor-declared write access.
    UndeclaredWriteChannel {
        /// Cell channel requested for writing.
        channel: CellChannel,
    },
    /// Descriptor contains an input id outside the accepted local identifier shape.
    InvalidInputId {
        /// Invalid descriptor-local input id.
        id: EffectInputId,
    },
    /// Descriptor contains an output id outside the accepted local identifier shape.
    InvalidEffectOutputId {
        /// Invalid descriptor-local output id.
        id: EffectOutputId,
    },
    /// Node contains a graph value id outside the accepted identifier shape.
    InvalidGraphValueId {
        /// Invalid graph-local value id.
        id: GraphValueId,
    },

    /// Graph id is outside the accepted identifier shape.
    InvalidGraphId {
        /// Invalid graph id.
        id: GraphId,
    },
    /// Node id is outside the accepted identifier shape.
    InvalidNodeId {
        /// Invalid graph-local node id.
        id: NodeId,
    },
    /// Graph parameter map key does not match the nested parameter id.
    ParameterIdMismatch {
        /// Parameter map key.
        key: ParameterId,
        /// Parameter id stored in the value.
        parameter: ParameterId,
    },
    /// Graph signal map key does not match the nested signal id.
    SignalIdMismatch {
        /// Signal map key.
        key: SignalId,
        /// Signal id stored in the value.
        signal: SignalId,
    },
    /// Graph node map key does not match the nested node id.
    NodeIdMismatch {
        /// Node map key.
        key: NodeId,
        /// Node id stored in the value.
        node: NodeId,
    },
    /// Graph effect map key does not match the nested effect descriptor id.
    EffectIdMismatch {
        /// Effect map key.
        key: EffectId,
        /// Effect id stored in the descriptor.
        effect: EffectId,
    },
    /// Node references an effect descriptor not declared in the graph.
    UnknownEffect {
        /// Missing effect id.
        id: EffectId,
    },
    /// Node input references an input id not declared by the effect descriptor.
    UnknownNodeInput {
        /// Effect id whose input map was checked.
        effect: EffectId,
        /// Missing effect input id.
        input: EffectInputId,
    },
    /// Node output references an effect output not declared by the effect descriptor.
    UnknownEffectOutput {
        /// Effect id whose output map was checked.
        effect: EffectId,
        /// Missing descriptor-local output id.
        output: EffectOutputId,
    },
    /// Node output re-emits an input not declared by the effect descriptor.
    UnknownNodeOutputInput {
        /// Effect id whose input map was checked.
        effect: EffectId,
        /// Missing descriptor-local input id.
        input: EffectInputId,
    },
    /// Node omits an effect input that has no descriptor default.
    MissingRequiredNodeInput {
        /// Effect id whose input is required.
        effect: EffectId,
        /// Missing effect input id.
        input: EffectInputId,
    },
    /// Node order references a node id not declared in the graph.
    UnknownOrderNode {
        /// Missing ordered node id.
        id: NodeId,
    },
    /// Node order repeats the same node id.
    DuplicateOrderNode {
        /// Duplicated ordered node id.
        id: NodeId,
    },
    /// Graph declares a node that is absent from deterministic node order.
    NodeMissingFromOrder {
        /// Unordered node id.
        id: NodeId,
    },
    /// Parameter id is outside the accepted identifier shape.
    InvalidParameterId {
        /// Invalid parameter id.
        id: ParameterId,
    },
    /// Signal id is outside the accepted identifier shape.
    InvalidSignalId {
        /// Invalid signal id.
        id: SignalId,
    },
    /// Value source references an undeclared parameter.
    UnknownParameter {
        /// Missing parameter id.
        id: ParameterId,
    },
    /// Value source references an undeclared signal.
    UnknownSignal {
        /// Missing signal id.
        id: SignalId,
    },
    /// Value source references an undeclared graph-local value.
    UnknownGraphValue {
        /// Missing graph value id.
        id: GraphValueId,
    },
    /// Graph-local value source was used outside a node input context.
    GraphValueSourceNotAllowed {
        /// Graph value id that was rejected.
        id: GraphValueId,
    },
    /// Binding target references an undeclared parameter.
    UnknownBindingParameterTarget {
        /// Missing target parameter id.
        id: ParameterId,
    },
    /// Binding targets a parameter that is not declared bindable.
    ParameterNotBindable {
        /// Non-bindable target parameter id.
        id: ParameterId,
    },
    /// Map range is missing a bound needed for declarative mapping.
    IncompleteMapRange {
        /// Range label, such as `input` or `output`.
        range: String,
    },
    /// Value source kind does not match the expected target kind.
    SourceKindMismatch {
        /// Expected target value kind.
        expected: ValueKind,
        /// Actual source value kind.
        actual: ValueKind,
    },
    /// Map value source was applied to a non-numeric source kind.
    NonNumericMapSource {
        /// Actual non-numeric source kind.
        actual: ValueKind,
    },
    /// Value kind does not match the expected value spec kind.
    ValueKindMismatch {
        /// Expected value kind declared by the spec.
        expected: ValueKind,
        /// Actual value kind carried by the literal.
        actual: ValueKind,
    },
    /// Numeric value is not finite and cannot be represented safely in JSON contracts.
    NonFiniteNumericValue {
        /// Non-finite numeric value that failed validation.
        value: f64,
    },
    /// Numeric range bound is not finite and cannot be represented safely in JSON contracts.
    NonFiniteNumericRangeBound {
        /// Non-finite range bound that failed validation.
        value: f64,
    },
    /// Numeric range was declared for a non-numeric value kind.
    RangeOnNonNumericKind {
        /// Non-numeric value kind that carried range metadata.
        value_kind: ValueKind,
    },
    /// Numeric range minimum is greater than its maximum.
    InvalidNumericRange {
        /// Inclusive minimum value when present.
        min: Option<f64>,
        /// Inclusive maximum value when present.
        max: Option<f64>,
    },
    /// Numeric value is outside the declared inclusive range.
    NumericValueOutOfRange {
        /// Numeric value that failed range validation.
        value: f64,
        /// Inclusive minimum value when present.
        min: Option<f64>,
        /// Inclusive maximum value when present.
        max: Option<f64>,
    },
    /// Enum specs must declare at least one allowed value.
    EmptyEnumAllowedValues,
    /// Enum value is not one of the declared allowed values.
    EnumValueNotAllowed {
        /// Rejected enum value.
        value: String,
    },
}

impl DescriptorValidationError {
    /// Return the requested scope kind when this is a scope support error.
    pub fn requested_scope_kind(&self) -> Option<ScopeKind> {
        match self {
            Self::UnsupportedScopeKind { requested } => Some(*requested),
            _ => None,
        }
    }

    /// Return the requested role policy kind when this is a role policy error.
    pub fn requested_role_policy_kind(&self) -> Option<RoleWritePolicyKind> {
        match self {
            Self::UnsupportedRoleWritePolicy { requested } => Some(*requested),
            _ => None,
        }
    }

    /// Return the requested channel when this is a channel access error.
    pub fn requested_channel(&self) -> Option<CellChannel> {
        match self {
            Self::UndeclaredWriteChannel { channel } => Some(*channel),
            _ => None,
        }
    }
}

// <FILE>crates/tui-vfx-contract/src/cls_descriptor_validation_error.rs</FILE> - <DESC>Descriptor capability validation error enum</DESC>
// <VERS>END OF VERSION: 0.5.0</VERS>
