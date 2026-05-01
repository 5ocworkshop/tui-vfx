// <FILE>primitives/shader/linear_gradient/generated/linear_gradient_inputs.rs</FILE> - <DESC>Descriptor-derived input skeleton for shader.linearGradient</DESC>
// <VERS>VERSION: 0.1.0</VERS>
// <WCTX>Generated Primitive Workbench scaffold; do not hand-edit runtime behavior here.</WCTX>
// <CLOG>0.1.0: INIT — mirror descriptor inputs for compositor-next linear gradient slice.</CLOG>

#[derive(Clone, Debug, PartialEq)]
pub struct LinearGradientInputs {
    /// Descriptor field `startColor` (color).
    pub start_color: serde_json::Value,
    /// Descriptor field `endColor` (color).
    pub end_color: serde_json::Value,
    /// Descriptor field `colorSpace` (enum).
    pub color_space: String,
    /// Descriptor field `angleDeg` (number).
    pub angle_deg: f64,
    /// Descriptor field `intensity` (number).
    pub intensity: f64,
    /// Descriptor field `gradient` (gradient).
    pub gradient: Option<serde_json::Value>,
    /// Descriptor field `applyTo` (enum).
    pub apply_to: String,
}

// <FILE>primitives/shader/linear_gradient/generated/linear_gradient_inputs.rs</FILE>
// <VERS>END OF VERSION: 0.1.0</VERS>
