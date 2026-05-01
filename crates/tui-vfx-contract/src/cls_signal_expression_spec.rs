// <FILE>crates/tui-vfx-contract/src/cls_signal_expression_spec.rs</FILE> - <DESC>Typed numeric signal expression DTO</DESC>
// <VERS>VERSION: 0.1.0</VERS>
// <WCTX>v3.1 recipe-oracle pass: promote recognized preview/procedural signal shapes above opaque structured data.</WCTX>
// <CLOG>0.1.0: INIT — add deterministic numeric signal expression vocabulary.</CLOG>

use crate::DescriptorValidationError;

/// Deterministic numeric expression usable by preview loopbacks and authored value sources.
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize, schemars::JsonSchema)]
#[serde(
    tag = "kind",
    rename_all = "camelCase",
    rename_all_fields = "camelCase",
    deny_unknown_fields
)]
pub enum SignalExpressionSpec {
    /// Constant numeric expression.
    Constant {
        /// Constant numeric value.
        value: f64,
    },
    /// Repeating ramp from `start` to `end`.
    Ramp {
        /// Value at normalized expression phase zero.
        start: f64,
        /// Value at normalized expression phase one.
        end: f64,
    },
    /// Sine oscillator.
    Sine {
        /// Oscillator frequency in hertz.
        frequency_hz: f64,
        /// Oscillator amplitude.
        amplitude: f64,
        /// Numeric offset added after oscillation.
        offset: f64,
        /// Phase offset in cycles.
        phase: f64,
    },
    /// Triangle oscillator.
    Triangle {
        /// Oscillator frequency in hertz.
        frequency_hz: f64,
        /// Oscillator amplitude.
        amplitude: f64,
        /// Numeric offset added after oscillation.
        offset: f64,
        /// Phase offset in cycles.
        phase: f64,
    },
    /// Mix two numeric expressions with a 0..1 blend factor.
    Mix {
        /// First expression.
        a: Box<SignalExpressionSpec>,
        /// Second expression.
        b: Box<SignalExpressionSpec>,
        /// Blend factor where 0 selects `a` and 1 selects `b`.
        mix: f64,
    },
    /// Add two numeric expressions.
    Add {
        /// First expression.
        a: Box<SignalExpressionSpec>,
        /// Second expression.
        b: Box<SignalExpressionSpec>,
    },
    /// Multiply two numeric expressions.
    Multiply {
        /// First expression.
        a: Box<SignalExpressionSpec>,
        /// Second expression.
        b: Box<SignalExpressionSpec>,
    },
    /// Clamp a numeric expression into an inclusive range.
    Clamp {
        /// Expression input to clamp.
        input: Box<SignalExpressionSpec>,
        /// Inclusive minimum value.
        min: f64,
        /// Inclusive maximum value.
        max: f64,
    },
}

impl SignalExpressionSpec {
    /// Validate finite numeric parameters and nested expressions.
    pub fn validate(&self) -> Result<(), DescriptorValidationError> {
        match self {
            Self::Constant { value } => validate_finite(*value),
            Self::Ramp { start, end } => {
                validate_finite(*start)?;
                validate_finite(*end)
            }
            Self::Sine {
                frequency_hz,
                amplitude,
                offset,
                phase,
            }
            | Self::Triangle {
                frequency_hz,
                amplitude,
                offset,
                phase,
            } => {
                validate_finite(*frequency_hz)?;
                validate_finite(*amplitude)?;
                validate_finite(*offset)?;
                validate_finite(*phase)
            }
            Self::Mix { a, b, mix } => {
                a.validate()?;
                b.validate()?;
                validate_finite(*mix)
            }
            Self::Add { a, b } | Self::Multiply { a, b } => {
                a.validate()?;
                b.validate()
            }
            Self::Clamp { input, min, max } => {
                input.validate()?;
                validate_finite(*min)?;
                validate_finite(*max)
            }
        }
    }
}

fn validate_finite(value: f64) -> Result<(), DescriptorValidationError> {
    if value.is_finite() {
        Ok(())
    } else {
        Err(DescriptorValidationError::NonFiniteNumericValue { value })
    }
}

// <FILE>crates/tui-vfx-contract/src/cls_signal_expression_spec.rs</FILE> - <DESC>Typed numeric signal expression DTO</DESC>
// <VERS>END OF VERSION: 0.1.0</VERS>
