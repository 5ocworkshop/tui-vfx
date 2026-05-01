// <FILE>primitives/shader/linear_gradient/generated/linear_gradient_accessors.rs</FILE> - <DESC>Descriptor-derived accessor skeleton for shader.linearGradient</DESC>
// <VERS>VERSION: 0.1.0</VERS>
// <WCTX>Generated Primitive Workbench scaffold; connects descriptor fields to future typed runtime input extraction.</WCTX>
// <CLOG>0.1.0: INIT — reserve generated accessor names for compositor-next linear gradient slice.</CLOG>

pub const DESCRIPTOR_ID: &str = "shader.linearGradient";
pub const INPUT_NAMES: &[&str] = &[
    "startColor",
    "endColor",
    "colorSpace",
    "angleDeg",
    "intensity",
    "gradient",
    "applyTo",
];

pub fn is_supported_input(name: &str) -> bool {
    INPUT_NAMES.contains(&name)
}

// <FILE>primitives/shader/linear_gradient/generated/linear_gradient_accessors.rs</FILE>
// <VERS>END OF VERSION: 0.1.0</VERS>
