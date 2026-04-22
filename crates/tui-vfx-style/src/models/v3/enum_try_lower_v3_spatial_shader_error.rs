// <FILE>tui-vfx-style/src/models/v3/enum_try_lower_v3_spatial_shader_error.rs</FILE> - <DESC>Error type for lowering grouped V3 spatial shader families back into the legacy runtime surface</DESC>
// <VERS>VERSION: 0.1.0</VERS>
// <WCTX>Decision 2 runtime follow-on — the grouped V3 spatial family seam now needs a reversible bridge into the legacy executable shader surface so runtime consumers can start executing grouped family values directly during cutover.</WCTX>
// <CLOG>0.1.0: define explicit failure modes for grouped V3 spatial-family lowering back into SpatialShaderType.</CLOG>

//! Error type for lowering grouped V3 spatial shader families back into the
//! legacy runtime surface.

use std::fmt;

/// Failure modes when converting a grouped V3 spatial family back into the
/// executable legacy [`crate::models::SpatialShaderType`] surface.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TryLowerV3SpatialShaderError {
    /// The chosen traveling-band behavior cannot represent the supplied color policy
    /// in any current legacy shader variant.
    UnsupportedTravelingBandColorPolicy {
        behavior: &'static str,
        color_policy: &'static str,
    },
}

impl fmt::Display for TryLowerV3SpatialShaderError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::UnsupportedTravelingBandColorPolicy { behavior, color_policy } => write!(
                f,
                "traveling-band behavior `{behavior}` cannot lower the color policy `{color_policy}` into the legacy runtime surface"
            ),
        }
    }
}

impl std::error::Error for TryLowerV3SpatialShaderError {}

// <FILE>tui-vfx-style/src/models/v3/enum_try_lower_v3_spatial_shader_error.rs</FILE> - <DESC>Error type for lowering grouped V3 spatial shader families back into the legacy runtime surface</DESC>
// <VERS>END OF VERSION: 0.1.0</VERS>
