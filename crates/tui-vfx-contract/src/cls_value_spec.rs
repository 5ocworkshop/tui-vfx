// <FILE>crates/tui-vfx-contract/src/cls_value_spec.rs</FILE> - <DESC>Effect input value specification DTO</DESC>
// <VERS>VERSION: 0.1.0</VERS>
// <WCTX>New kernel Phase F1: declare typed defaults, numeric ranges, enums, units, and semantics.</WCTX>
// <CLOG>0.1.0: INIT — add ValueSpec with local validation for defaults, ranges, and enum domains.</CLOG>

use crate::{DescriptorValidationError, NumericRange, Value, ValueKind};

/// Contract specification for one typed effect input value.
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize, schemars::JsonSchema)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct ValueSpec {
    /// Closed kind expected for values accepted by this spec.
    pub kind: ValueKind,
    /// Optional typed default value for the input.
    pub default: Option<Value>,
    /// Optional inclusive range for numeric, integer, and duration values.
    pub range: Option<NumericRange>,
    /// Allowed string values when `kind` is `enum`.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub allowed_values: Vec<String>,
    /// Optional human-facing unit label such as `ratio` or `seconds`.
    pub unit: Option<String>,
    /// Optional human-facing semantic hint such as `opacity`.
    pub semantic: Option<String>,
}

impl ValueSpec {
    /// Validate the spec-level invariants and the default value when present.
    pub fn validate(&self) -> Result<(), DescriptorValidationError> {
        if self.range.is_some()
            && !matches!(
                self.kind,
                ValueKind::Integer | ValueKind::Number | ValueKind::Duration
            )
        {
            return Err(DescriptorValidationError::RangeOnNonNumericKind {
                value_kind: self.kind,
            });
        }

        if let Some(range) = self.range {
            if let Some(value) = range.min.filter(|value| !value.is_finite()) {
                return Err(DescriptorValidationError::NonFiniteNumericRangeBound { value });
            }
            if let Some(value) = range.max.filter(|value| !value.is_finite()) {
                return Err(DescriptorValidationError::NonFiniteNumericRangeBound { value });
            }
            if !range.is_ordered() {
                return Err(DescriptorValidationError::InvalidNumericRange {
                    min: range.min,
                    max: range.max,
                });
            }
        }

        if self.kind == ValueKind::Enum && self.allowed_values.is_empty() {
            return Err(DescriptorValidationError::EmptyEnumAllowedValues);
        }

        if let Some(default) = &self.default {
            self.validate_value(default)?;
        }

        Ok(())
    }

    /// Validate one typed value against this spec.
    pub fn validate_value(&self, value: &Value) -> Result<(), DescriptorValidationError> {
        let actual = value.kind();
        if actual != self.kind {
            return Err(DescriptorValidationError::ValueKindMismatch {
                expected: self.kind,
                actual,
            });
        }

        if let Some(numeric_value) = value.as_range_number() {
            if !numeric_value.is_finite() {
                return Err(DescriptorValidationError::NonFiniteNumericValue {
                    value: numeric_value,
                });
            }

            if let Some(range) = self.range
                && !range.contains(numeric_value)
            {
                return Err(DescriptorValidationError::NumericValueOutOfRange {
                    value: numeric_value,
                    min: range.min,
                    max: range.max,
                });
            }
        }

        if self.kind == ValueKind::Enum {
            let Some(enum_value) = value.as_enum_value() else {
                return Err(DescriptorValidationError::ValueKindMismatch {
                    expected: ValueKind::Enum,
                    actual,
                });
            };
            if !self
                .allowed_values
                .iter()
                .any(|allowed| allowed == enum_value)
            {
                return Err(DescriptorValidationError::EnumValueNotAllowed {
                    value: enum_value.to_string(),
                });
            }
        }

        Ok(())
    }
}

// <FILE>crates/tui-vfx-contract/src/cls_value_spec.rs</FILE> - <DESC>Effect input value specification DTO</DESC>
// <VERS>END OF VERSION: 0.1.0</VERS>
