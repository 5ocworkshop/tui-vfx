// <FILE>crates/tui-vfx-contract/src/cls_numeric_range.rs</FILE> - <DESC>Optional numeric bounds for value specs</DESC>
// <VERS>VERSION: 0.1.0</VERS>
// <WCTX>New kernel Phase F1: validate numeric and duration effect input defaults.</WCTX>
// <CLOG>0.1.0: INIT — add min/max range DTO shared by value specs.</CLOG>

/// Optional inclusive numeric bounds for a value spec.
#[derive(
    Clone, Copy, Debug, PartialEq, serde::Serialize, serde::Deserialize, schemars::JsonSchema,
)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct NumericRange {
    /// Inclusive minimum value when present.
    pub min: Option<f64>,
    /// Inclusive maximum value when present.
    pub max: Option<f64>,
}

impl NumericRange {
    /// Return true when the range bounds are internally ordered.
    pub fn is_ordered(&self) -> bool {
        match (self.min, self.max) {
            (Some(min), Some(max)) => min <= max,
            _ => true,
        }
    }

    /// Return true when a numeric value is inside the inclusive range.
    pub fn contains(&self, value: f64) -> bool {
        self.min.is_none_or(|min| value >= min) && self.max.is_none_or(|max| value <= max)
    }
}

// <FILE>crates/tui-vfx-contract/src/cls_numeric_range.rs</FILE> - <DESC>Optional numeric bounds for value specs</DESC>
// <VERS>END OF VERSION: 0.1.0</VERS>
