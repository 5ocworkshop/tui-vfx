// <FILE>crates/tui-vfx-contract/src/cls_gradient_spec.rs</FILE> - <DESC>Canonical gradient value DTO</DESC>
// <VERS>VERSION: 0.1.0</VERS>
// <WCTX>K2.13 schema decision burn-down: represent gradient stops as a typed canonical value.</WCTX>
// <CLOG>0.1.0: INIT — add gradient stop and color-space payload for shader inputs.</CLOG>

use tui_vfx_types::Color;

use crate::DescriptorValidationError;

/// One color stop in a canonical gradient value.
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize, schemars::JsonSchema)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct GradientStop {
    /// Normalized stop position in the inclusive 0..1 range.
    pub position: f64,
    /// Color sampled at this stop.
    pub color: Color,
}

/// Canonical gradient value for descriptor inputs.
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize, schemars::JsonSchema)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct GradientSpec {
    /// Ordered or author-provided gradient stops.
    pub stops: Vec<GradientStop>,
    /// Color interpolation space, such as `rgb` or `hct`.
    pub space: String,
}

impl GradientSpec {
    /// Validate gradient stop structure.
    pub fn validate(&self) -> Result<(), DescriptorValidationError> {
        if self.stops.len() < 2 {
            return Err(DescriptorValidationError::GradientRequiresAtLeastTwoStops);
        }
        for stop in &self.stops {
            if !stop.position.is_finite() {
                return Err(DescriptorValidationError::NonFiniteNumericValue {
                    value: stop.position,
                });
            }
            if !(0.0..=1.0).contains(&stop.position) {
                return Err(DescriptorValidationError::NumericValueOutOfRange {
                    value: stop.position,
                    min: Some(0.0),
                    max: Some(1.0),
                });
            }
        }
        Ok(())
    }
}

// <FILE>crates/tui-vfx-contract/src/cls_gradient_spec.rs</FILE> - <DESC>Canonical gradient value DTO</DESC>
// <VERS>END OF VERSION: 0.1.0</VERS>
