// <FILE>crates/tui-vfx-contract/src/cls_value_predicate.rs</FILE> - <DESC>Lifecycle trigger value predicate DTO</DESC>
// <VERS>VERSION: 0.1.1</VERS>
// <WCTX>K2.13 schema decision burn-down: include gradient presence in truthy validation.</WCTX>
// <CLOG>0.1.1: PATCH — allow gradient values in truthy predicate kind validation.
// 0.1.0: INIT — add explicit value predicate vocabulary and kind validation.</CLOG>

use crate::{DescriptorValidationError, Value, ValueKind};

/// Typed predicate evaluated against a lifecycle trigger value source.
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize, schemars::JsonSchema)]
#[serde(tag = "kind", rename_all = "camelCase", deny_unknown_fields)]
pub enum ValuePredicate {
    /// Source value must be boolean true.
    IsTrue,
    /// Source value must be boolean false.
    IsFalse,
    /// Source value must be a non-zero integer, number, or duration.
    NonZero,
    /// Source value must be a non-empty string or text value.
    NonEmpty,
    /// Source value must equal the typed literal.
    Equals {
        /// Typed literal used for equality comparison.
        value: Value,
    },
    /// Source value must not equal the typed literal.
    NotEquals {
        /// Typed literal used for inequality comparison.
        value: Value,
    },
    /// Source value must be greater than the typed numeric literal.
    GreaterThan {
        /// Typed numeric literal used as the exclusive lower bound.
        value: Value,
    },
    /// Source value must be less than the typed numeric literal.
    LessThan {
        /// Typed numeric literal used as the exclusive upper bound.
        value: Value,
    },
    /// Convenience level predicate with documented per-kind truthiness behavior.
    ///
    /// The contract truth table is boolean true, integer non-zero, finite number
    /// non-zero, string/text non-empty, color or gradient value present, and finite duration
    /// non-zero. Null, enum, role, scope, and rect have no I0 truth rule.
    Truthy,
}

impl ValuePredicate {
    /// Validate that this predicate is meaningful for the source value kind.
    pub fn validate_for_kind(&self, kind: ValueKind) -> Result<(), DescriptorValidationError> {
        match self {
            Self::IsTrue | Self::IsFalse => require_kind(self, kind, &[ValueKind::Boolean]),
            Self::NonZero => require_kind(
                self,
                kind,
                &[ValueKind::Integer, ValueKind::Number, ValueKind::Duration],
            ),
            Self::NonEmpty => require_kind(self, kind, &[ValueKind::String, ValueKind::Text]),
            Self::Equals { value } | Self::NotEquals { value } => {
                validate_literal_kind(self, kind, value)
            }
            Self::GreaterThan { value } | Self::LessThan { value } => {
                require_kind(
                    self,
                    kind,
                    &[ValueKind::Integer, ValueKind::Number, ValueKind::Duration],
                )?;
                validate_literal_kind(self, kind, value)
            }
            Self::Truthy => require_kind(
                self,
                kind,
                &[
                    ValueKind::Boolean,
                    ValueKind::Integer,
                    ValueKind::Number,
                    ValueKind::String,
                    ValueKind::Text,
                    ValueKind::Color,
                    ValueKind::Gradient,
                    ValueKind::Duration,
                ],
            ),
        }
    }

    /// Stable kind label used in validation errors.
    pub const fn label(&self) -> &'static str {
        match self {
            Self::IsTrue => "isTrue",
            Self::IsFalse => "isFalse",
            Self::NonZero => "nonZero",
            Self::NonEmpty => "nonEmpty",
            Self::Equals { .. } => "equals",
            Self::NotEquals { .. } => "notEquals",
            Self::GreaterThan { .. } => "greaterThan",
            Self::LessThan { .. } => "lessThan",
            Self::Truthy => "truthy",
        }
    }
}

fn validate_literal_kind(
    predicate: &ValuePredicate,
    expected: ValueKind,
    value: &Value,
) -> Result<(), DescriptorValidationError> {
    let actual = value.kind();
    if actual == expected {
        Ok(())
    } else {
        Err(DescriptorValidationError::PredicateValueKindMismatch {
            predicate: predicate.label().to_string(),
            expected,
            actual,
        })
    }
}

fn require_kind(
    predicate: &ValuePredicate,
    actual: ValueKind,
    allowed: &[ValueKind],
) -> Result<(), DescriptorValidationError> {
    if allowed.contains(&actual) {
        Ok(())
    } else {
        Err(DescriptorValidationError::PredicateKindMismatch {
            predicate: predicate.label().to_string(),
            actual,
        })
    }
}

// <FILE>crates/tui-vfx-contract/src/cls_value_predicate.rs</FILE> - <DESC>Lifecycle trigger value predicate DTO</DESC>
// <VERS>END OF VERSION: 0.1.1</VERS>
