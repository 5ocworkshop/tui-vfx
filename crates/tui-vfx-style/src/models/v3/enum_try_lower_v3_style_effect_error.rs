// <FILE>tui-vfx-style/src/models/v3/enum_try_lower_v3_style_effect_error.rs</FILE> - <DESC>Error type for lowering grouped V3 overall style effects back into the legacy runtime surface</DESC>
// <VERS>VERSION: 0.1.0</VERS>
// <WCTX>Decision 2 runtime follow-on — grouped V3 overall effect identity now needs a reversible bridge into the executable StyleEffect surface so non-spatial/composed effect parity can move beyond family labels.</WCTX>
// <CLOG>0.1.0: define explicit failure modes for grouped V3 overall effect lowering back into StyleEffect.</CLOG>

//! Error type for lowering grouped V3 overall style effects back into the
//! legacy runtime surface.

use std::fmt;

use crate::models::TryLowerV3SpatialShaderError;

/// Failure modes when converting grouped V3 overall effect values back into the
/// executable legacy [`crate::models::StyleEffect`] surface.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TryLowerV3StyleEffectError {
    /// The wrapped legacy effect does not actually belong to the grouped V3 family
    /// variant that was used to construct the value.
    MismatchedVariant {
        expected_family: &'static str,
        actual_effect: &'static str,
    },
    /// A grouped V3 spatial family could not lower back into the current executable
    /// spatial shader surface.
    Spatial(TryLowerV3SpatialShaderError),
}

impl fmt::Display for TryLowerV3StyleEffectError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::MismatchedVariant {
                expected_family,
                actual_effect,
            } => write!(
                f,
                "grouped V3 style effect variant `{expected_family}` cannot lower legacy effect `{actual_effect}`"
            ),
            Self::Spatial(error) => write!(f, "{error}"),
        }
    }
}

impl std::error::Error for TryLowerV3StyleEffectError {}

impl From<TryLowerV3SpatialShaderError> for TryLowerV3StyleEffectError {
    fn from(value: TryLowerV3SpatialShaderError) -> Self {
        Self::Spatial(value)
    }
}

// <FILE>tui-vfx-style/src/models/v3/enum_try_lower_v3_style_effect_error.rs</FILE> - <DESC>Error type for lowering grouped V3 overall style effects back into the legacy runtime surface</DESC>
// <VERS>END OF VERSION: 0.1.0</VERS>
