// <FILE>crates/tui-vfx-contract/src/cls_preview_loopback_spec.rs</FILE> - <DESC>Preview loopback signal provider DTO</DESC>
// <VERS>VERSION: 0.3.0</VERS>
// <WCTX>v3.1 recipe migration parity: preserve authored loopback/demo signal providers separately from host trigger semantics.</WCTX>
// <CLOG>0.3.0: MINOR — add typed signal expression loopback form.
// 0.2.0: MINOR — add authored numeric static and signal loopback forms.
// 0.1.0: INIT — add literal, numeric-ramp, and structured signal-expression preview loopback contracts.</CLOG>

use crate::{
    DescriptorValidationError, DurationSpec, SignalExpressionSpec, StructuredValue, Value,
    ValueKind, ValueSpec,
};

/// Preview/demo value provider for a host signal.
///
/// A preview loopback is not trigger semantics and does not replace a host
/// signal. Hosts still win when they provide a runtime value. The loopback
/// exists so deterministic players, documentation generators, and migration
/// validators can replay a recipe without silently dropping authored demo
/// bindings.
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize, schemars::JsonSchema)]
#[serde(tag = "kind", rename_all = "camelCase", deny_unknown_fields)]
pub enum PreviewLoopbackSpec {
    /// Static typed preview value.
    Literal {
        /// Typed literal emitted when the host does not provide the signal.
        value: Value,
    },
    /// Authored numeric static/literal loopback value.
    ///
    /// This maps authored forms such as `loopback: 1.0`,
    /// `loopback: { "static": 1.0 }`, and `loopback: { "literal": 1.0 }`.
    /// The player coerces the sampled number to the declared signal kind, so
    /// integer/u16-style bindings round and clamp rather than silently dropping
    /// the loopback.
    NumericStatic {
        /// Numeric value emitted when the host does not provide the signal.
        value: f64,
    },
    /// Authored numeric signal loopback expression.
    ///
    /// This maps authored forms such as
    /// `loopback: { "signal": { "type": "ramp", ... } }`. The expression is
    /// preserved losslessly as descriptor-owned structured data while preview
    /// tooling executes recognized deterministic signal shapes.
    NumericSignal {
        /// Curated signal expression payload.
        expression: StructuredValue,
        /// Optional typed fallback when the expression cannot be sampled.
        fallback: Option<Value>,
    },
    /// Numeric ramp preview value sampled over the active player phase.
    ///
    /// Kept as a convenient v3.1-native shorthand; recipe migrations should prefer
    /// `numericSignal` when preserving the original signal declaration.
    NumericRamp {
        /// Numeric value at normalized phase progress zero.
        start: f64,
        /// Numeric value at normalized phase progress one.
        end: f64,
        /// Authored ramp duration for tooling and player timing hints.
        duration: DurationSpec,
        /// Whether preview playback should repeat after the authored duration.
        repeat: bool,
    },
    /// Typed deterministic numeric signal expression.
    Expression {
        /// Canonical numeric expression.
        expression: SignalExpressionSpec,
        /// Optional typed fallback when the expression cannot be sampled.
        fallback: Option<Value>,
    },
    /// Legacy structured signal expression preserved from an authoring facade.
    ///
    /// This is the lossless escape hatch for curated authored signal expressions
    /// whose runtime sampler has not yet been promoted into this clean-room
    /// contract crate. Preview tooling may execute recognized shapes; otherwise
    /// they fall back to the optional typed value.
    SignalExpression {
        /// Descriptor-owned signal expression payload.
        expression: StructuredValue,
        /// Optional typed fallback when the expression cannot be sampled.
        fallback: Option<Value>,
    },
}

impl PreviewLoopbackSpec {
    /// Validate this preview provider against the signal's typed value contract.
    pub fn validate_for_signal(&self, value: &ValueSpec) -> Result<(), DescriptorValidationError> {
        match self {
            Self::Literal { value: literal } => value.validate_value(literal),
            Self::NumericStatic { value: numeric } => {
                validate_numeric_signal_kind(value.kind)?;
                validate_finite_number(*numeric)
            }
            Self::NumericSignal { fallback, .. } => {
                validate_numeric_signal_kind(value.kind)?;
                if let Some(fallback) = fallback {
                    value.validate_value(fallback)?;
                }
                Ok(())
            }
            Self::NumericRamp {
                start,
                end,
                duration,
                ..
            } => {
                validate_numeric_signal_kind(value.kind)?;
                validate_finite_number(*start)?;
                validate_finite_number(*end)?;
                duration.validate()
            }
            Self::SignalExpression { fallback, .. } => {
                validate_numeric_signal_kind(value.kind)?;
                if let Some(fallback) = fallback {
                    value.validate_value(fallback)?;
                }
                Ok(())
            }
            Self::Expression {
                expression,
                fallback,
            } => {
                validate_numeric_signal_kind(value.kind)?;
                expression.validate()?;
                if let Some(fallback) = fallback {
                    value.validate_value(fallback)?;
                }
                Ok(())
            }
        }
    }
}

fn validate_numeric_signal_kind(kind: ValueKind) -> Result<(), DescriptorValidationError> {
    if matches!(
        kind,
        ValueKind::Integer | ValueKind::Number | ValueKind::Duration
    ) {
        Ok(())
    } else {
        Err(DescriptorValidationError::ValueKindMismatch {
            expected: kind,
            actual: ValueKind::Number,
        })
    }
}

fn validate_finite_number(value: f64) -> Result<(), DescriptorValidationError> {
    if value.is_finite() {
        Ok(())
    } else {
        Err(DescriptorValidationError::NonFiniteNumericValue { value })
    }
}

// <FILE>crates/tui-vfx-contract/src/cls_preview_loopback_spec.rs</FILE> - <DESC>PreviewLoopbackSpec</DESC>
// <VERS>END OF VERSION: 0.3.0</VERS>
