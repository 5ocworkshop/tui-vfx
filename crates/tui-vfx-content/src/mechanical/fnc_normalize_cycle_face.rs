// <FILE>crates/tui-vfx-content/src/mechanical/fnc_normalize_cycle_face.rs</FILE> - <DESC>Convert one cycle face string into an OwnedGrid padded to the mechanism tile size, rejecting oversized faces</DESC>
// <VERS>VERSION: 0.1.0</VERS>
// <WCTX>Phase 2 of mechanical circular content cycles plan: face string to tile-sized grid normalization with deterministic rejection of oversized faces.</WCTX>
// <CLOG>0.1.0: introduce normalize_cycle_face; reuse newline-aware grid_from_text and pad to tile size; reject empty and oversized faces.</CLOG>

use super::enum_cycle_error::MechanicalCycleError;
use super::fnc_grid_text::grid_from_text;
use super::types::{MechanicalSizing, MechanicalTile};
use tui_vfx_types::{Grid, OwnedGrid};

/// Normalize one face string against the mechanism tile size.
///
/// Newlines split the face into rows (`'\n'` is structure, not a
/// visible glyph). Faces smaller than the tile rectangle are padded
/// with spaces; faces larger than the tile rectangle are rejected with
/// `MechanicalCycleError::FaceExceedsTile` rather than silently
/// clipped, because silent clipping hides authoring mistakes.
///
/// Empty face values are rejected with
/// `MechanicalCycleError::EmptyFaceValue`. Recipes that want a "blank"
/// face supply a single space string.
pub(crate) fn normalize_cycle_face(
    value: &str,
    tile: MechanicalTile,
) -> Result<OwnedGrid, MechanicalCycleError> {
    if value.is_empty() {
        return Err(MechanicalCycleError::EmptyFaceValue);
    }
    let raw = grid_from_text(value, MechanicalSizing::PadToMax);
    let face_w = raw.width();
    let face_h = raw.height();
    let tile_w = tile.width as usize;
    let tile_h = tile.height as usize;
    if face_w > tile_w || face_h > tile_h {
        return Err(MechanicalCycleError::FaceExceedsTile {
            value: value.to_string(),
            face_w: face_w.min(u16::MAX as usize) as u16,
            face_h: face_h.min(u16::MAX as usize) as u16,
            tile_w: tile.width,
            tile_h: tile.height,
        });
    }
    Ok(pad_to_tile(&raw, tile_w, tile_h))
}

fn pad_to_tile(source: &OwnedGrid, tile_w: usize, tile_h: usize) -> OwnedGrid {
    let mut grid = OwnedGrid::new(tile_w, tile_h);
    for y in 0..source.height().min(tile_h) {
        for x in 0..source.width().min(tile_w) {
            if let Some(cell) = source.get(x, y) {
                grid.set(x, y, *cell);
            }
        }
    }
    grid
}

#[cfg(test)]
mod tests {
    use super::super::fnc_grid_text::grid_to_text;
    use super::*;

    fn tile(w: u16, h: u16) -> MechanicalTile {
        MechanicalTile::new(w, h).unwrap()
    }

    #[test]
    fn single_char_normalizes_to_1x1() {
        let grid = normalize_cycle_face("7", tile(1, 1)).unwrap();
        assert_eq!(grid.width(), 1);
        assert_eq!(grid.height(), 1);
        assert_eq!(grid_to_text(&grid), "7");
    }

    #[test]
    fn smaller_face_pads_with_spaces_to_tile_size() {
        let grid = normalize_cycle_face("7", tile(3, 1)).unwrap();
        assert_eq!(grid.width(), 3);
        assert_eq!(grid_to_text(&grid), "7  ");
    }

    #[test]
    fn multiline_face_normalizes_to_3x3() {
        let face = "███\n█ █\n███";
        let grid = normalize_cycle_face(face, tile(3, 3)).unwrap();
        assert_eq!(grid.width(), 3);
        assert_eq!(grid.height(), 3);
        assert_eq!(grid_to_text(&grid), face);
    }

    #[test]
    fn smaller_multiline_face_pads_to_tile() {
        let face = "AB\nCD";
        let grid = normalize_cycle_face(face, tile(3, 3)).unwrap();
        assert_eq!(grid.width(), 3);
        assert_eq!(grid.height(), 3);
        assert_eq!(grid_to_text(&grid), "AB \nCD \n   ");
    }

    #[test]
    fn oversize_face_is_rejected() {
        let face = "AAAA\nBBBB\nCCCC\nDDDD";
        let err = normalize_cycle_face(face, tile(3, 3)).unwrap_err();
        match err {
            MechanicalCycleError::FaceExceedsTile {
                face_w,
                face_h,
                tile_w,
                tile_h,
                ..
            } => {
                assert_eq!(face_w, 4);
                assert_eq!(face_h, 4);
                assert_eq!(tile_w, 3);
                assert_eq!(tile_h, 3);
            }
            other => panic!("expected FaceExceedsTile, got {other:?}"),
        }
    }

    #[test]
    fn empty_face_value_is_rejected() {
        let err = normalize_cycle_face("", tile(1, 1)).unwrap_err();
        assert_eq!(err, MechanicalCycleError::EmptyFaceValue);
    }

    #[test]
    fn single_space_face_is_accepted() {
        let grid = normalize_cycle_face(" ", tile(1, 1)).unwrap();
        assert_eq!(grid_to_text(&grid), " ");
    }

    #[test]
    fn taller_than_tile_face_is_rejected() {
        let face = "A\nB\nC\nD";
        let err = normalize_cycle_face(face, tile(1, 3)).unwrap_err();
        assert!(matches!(err, MechanicalCycleError::FaceExceedsTile { .. }));
    }

    #[test]
    fn wider_than_tile_face_is_rejected() {
        let err = normalize_cycle_face("ABCDE", tile(3, 1)).unwrap_err();
        assert!(matches!(err, MechanicalCycleError::FaceExceedsTile { .. }));
    }
}

// <FILE>crates/tui-vfx-content/src/mechanical/fnc_normalize_cycle_face.rs</FILE>
// <VERS>END OF VERSION: 0.1.0</VERS>
