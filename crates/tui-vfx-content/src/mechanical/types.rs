// <FILE>crates/tui-vfx-content/src/mechanical/types.rs</FILE> - <DESC>Shared internal mechanical display data types</DESC>
// <VERS>VERSION: 0.2.0</VERS>
// <WCTX>Phase 3 shared validation for SplitFlap tile geometry.</WCTX>
// <CLOG>Add explicit SplitFlap tile validation errors.</CLOG>

use tui_vfx_types::OwnedGrid;

pub(crate) struct MechanicalSource {
    pub(crate) from: OwnedGrid,
    pub(crate) to: OwnedGrid,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum MechanicalSizing {
    PadToMax,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct MechanicalTile {
    pub(crate) width: u16,
    pub(crate) height: u16,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) enum MechanicalValidationError {
    ZeroTileDimension,
    OddCenterHingeTileHeight { height: u16 },
    UnsupportedCenterHingeTileHeight { height: u16 },
    UnsupportedVerticalHinge,
}

impl MechanicalTile {
    pub(crate) fn new(width: u16, height: u16) -> Option<Self> {
        if width == 0 || height == 0 {
            None
        } else {
            Some(Self { width, height })
        }
    }
}

pub(crate) fn validate_split_flap_tile(
    tile: MechanicalTile,
) -> Result<(), MechanicalValidationError> {
    if tile.width == 0 || tile.height == 0 {
        return Err(MechanicalValidationError::ZeroTileDimension);
    }
    if tile.height == 1 {
        if tile.width == 1 {
            return Ok(());
        }
        return Err(MechanicalValidationError::UnsupportedVerticalHinge);
    }
    if !tile.height.is_multiple_of(2) {
        return Err(MechanicalValidationError::OddCenterHingeTileHeight {
            height: tile.height,
        });
    }
    if !matches!(tile.height, 2 | 4 | 6 | 8) {
        return Err(
            MechanicalValidationError::UnsupportedCenterHingeTileHeight {
                height: tile.height,
            },
        );
    }
    Ok(())
}

// <FILE>crates/tui-vfx-content/src/mechanical/types.rs</FILE> - <DESC>Shared internal mechanical display data types</DESC>
// <VERS>END OF VERSION: 0.2.0</VERS>
