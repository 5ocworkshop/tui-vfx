// <FILE>crates/tui-vfx-compost/src/samplers/cls_sampler_axis.rs</FILE> - <DESC>Shared sampler axis input enum</DESC>
// <VERS>VERSION: 0.1.0</VERS>
// <WCTX>Legacy sampler ports use an Axis enum; v3.1 descriptors expose the same x/y vocabulary as a closed enum input.</WCTX>
// <CLOG>0.1.0: INIT — add x/y sampler axis mapping helpers.</CLOG>

/// Axis along which a coordinate sampler applies displacement.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum SamplerAxis {
    /// Horizontal axis.
    X,
    /// Vertical axis.
    Y,
}

impl SamplerAxis {
    /// Descriptor/default string for this axis.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::X => "x",
            Self::Y => "y",
        }
    }

    /// Allowed descriptor values in canonical order.
    pub fn allowed_values() -> Vec<String> {
        [Self::X, Self::Y]
            .into_iter()
            .map(|axis| axis.as_str().to_string())
            .collect()
    }
}

// <FILE>crates/tui-vfx-compost/src/samplers/cls_sampler_axis.rs</FILE> - <DESC>Shared sampler axis input enum</DESC>
// <VERS>END OF VERSION: 0.1.0</VERS>
