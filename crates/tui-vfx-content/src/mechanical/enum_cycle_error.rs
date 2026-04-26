// <FILE>crates/tui-vfx-content/src/mechanical/enum_cycle_error.rs</FILE> - <DESC>Structured errors emitted by mechanical cycle resolution and route building</DESC>
// <VERS>VERSION: 0.1.0</VERS>
// <WCTX>Phase 2 of mechanical circular content cycles plan: error vocabulary that route resolver returns to Odometer/SplitFlap callers.</WCTX>
// <CLOG>0.1.0: introduce MechanicalCycleError covering empty/duplicate/zero-weight/overflow source cases plus route-building failures.</CLOG>

use std::fmt;

/// Error returned by cycle resolution and route building.
///
/// The variants are intentionally fine-grained so the recipe validator
/// can surface specific authoring fixes rather than a single opaque
/// `InvalidConfig` message.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum MechanicalCycleError {
    /// `Ordered`, `Randomized`, or `Weighted` source supplied an empty
    /// face list.
    EmptyFaces,
    /// A face string in `Ordered`/`Randomized`/`Weighted` was empty.
    EmptyFaceValue,
    /// Two faces in `Ordered`/`Randomized`/`Weighted` had the same
    /// `value`. Recipes must combine them (or use `Weighted` with a
    /// single entry whose weight equals the duplicate count).
    DuplicateFace { value: String },
    /// A `WeightedCycleFace` had `weight = 0`.
    ZeroWeight { value: String },
    /// Sum of weights does not fit in `u32`.
    WeightOverflow,
    /// `wrap = Circular` requires at least two distinct faces.
    CircularRequiresAtLeastTwoFaces,
    /// A face string parses to a grid larger than the mechanism tile.
    FaceExceedsTile {
        value: String,
        face_w: u16,
        face_h: u16,
        tile_w: u16,
        tile_h: u16,
    },
    /// Source or target face is not present in the resolved cycle and
    /// the missing-face policy is `Error`.
    MissingFace { value: String },
    /// `wrap = Bounded` rejects routes that would have to traverse past
    /// either endpoint.
    BoundedRouteImpossible {
        from: String,
        to: String,
        direction: &'static str,
    },
    /// `extra_rotations > 0` requires `wrap = Circular`.
    ExtraRotationsRequireCircular,
    /// `direction = Shortest` on a non-circular cycle when endpoints are
    /// not both reachable in a single direction.
    ShortestRequiresCircular,
    /// `direction = NumericDelta` requires faces to be exactly the
    /// decimal digits `0`..=`9`.
    NumericDeltaRequiresDigits,
    /// `direction = Authored` is reserved; recipes must not select it
    /// until a public override source surface is defined.
    AuthoredDirectionReserved,
}

impl fmt::Display for MechanicalCycleError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EmptyFaces => write!(f, "cycle face list is empty"),
            Self::EmptyFaceValue => write!(f, "cycle face value is empty"),
            Self::DuplicateFace { value } => {
                write!(f, "cycle face {value:?} appears more than once")
            }
            Self::ZeroWeight { value } => {
                write!(f, "cycle face {value:?} has weight zero")
            }
            Self::WeightOverflow => {
                write!(f, "cycle weight sum overflows u32")
            }
            Self::CircularRequiresAtLeastTwoFaces => {
                write!(f, "circular cycle requires at least two distinct faces")
            }
            Self::FaceExceedsTile {
                value,
                face_w,
                face_h,
                tile_w,
                tile_h,
            } => write!(
                f,
                "face {value:?} ({face_w}x{face_h}) exceeds tile size ({tile_w}x{tile_h})",
            ),
            Self::MissingFace { value } => {
                write!(f, "face {value:?} is not present in the resolved cycle")
            }
            Self::BoundedRouteImpossible {
                from,
                to,
                direction,
            } => write!(
                f,
                "bounded cycle has no {direction} route from {from:?} to {to:?}",
            ),
            Self::ExtraRotationsRequireCircular => {
                write!(f, "extra_rotations > 0 requires wrap: circular")
            }
            Self::ShortestRequiresCircular => {
                write!(f, "direction: shortest requires wrap: circular")
            }
            Self::NumericDeltaRequiresDigits => write!(
                f,
                "direction: numeric_delta requires faces to be exactly decimal digits 0..=9",
            ),
            Self::AuthoredDirectionReserved => {
                write!(f, "direction: authored is reserved and not yet supported")
            }
        }
    }
}

impl std::error::Error for MechanicalCycleError {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn display_message_includes_face_value() {
        let err = MechanicalCycleError::DuplicateFace {
            value: "BAR".into(),
        };
        let msg = err.to_string();
        assert!(msg.contains("BAR"), "{msg}");
    }

    #[test]
    fn face_exceeds_tile_message_includes_dimensions() {
        let err = MechanicalCycleError::FaceExceedsTile {
            value: "###\n###\n###\n###".into(),
            face_w: 3,
            face_h: 4,
            tile_w: 3,
            tile_h: 3,
        };
        let msg = err.to_string();
        assert!(msg.contains("3x4"), "{msg}");
        assert!(msg.contains("3x3"), "{msg}");
    }
}

// <FILE>crates/tui-vfx-content/src/mechanical/enum_cycle_error.rs</FILE>
// <VERS>END OF VERSION: 0.1.0</VERS>
