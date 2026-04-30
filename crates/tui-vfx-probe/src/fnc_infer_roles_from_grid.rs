// <FILE>crates/tui-vfx-probe/src/fnc_infer_roles_from_grid.rs</FILE> - <DESC>Infer a source RoleMap from a ProbeGridSpec using glyph content</DESC>
// <VERS>VERSION: 1.0.0</VERS>
// <WCTX>§8.7 probe-fidelity fix — replace all-Background placeholder in run_probe with content-inferred roles so role-scoped shaders match cells correctly</WCTX>
// <CLOG>1.0.0: initial — infer RoleTag::Text for non-empty-character cells, RoleTag::Background for blank cells; used by run_probe when source_roles is None to replace the all-Background placeholder</CLOG>

//! Glyph-content-based role inference for probe source grids.
//!
//! The probe has no producer-side role map for legacy schema_v1 recipes.
//! Rather than defaulting every cell to `RoleTag::Background` (which
//! causes all `Role(Text)`-scoped shaders to match zero cells), this
//! helper infers a role map from the rendered glyph content:
//!
//! - Cells whose `ch` is not a whitespace character → `RoleTag::Text`
//! - All other cells → `RoleTag::Background`
//!
//! This is the lightest contract that makes `Role(Text)` scope
//! predicates work correctly for legacy recipes whose `region: "TextOnly"`
//! lowers to that scope. It does not attempt to distinguish between
//! `Text`, `Title`, `Caption`, `Border`, `Icon`, or other first-class
//! role variants — those distinctions require a producer-supplied role
//! map (Sub-plan C territory).
//!
//! # Limitations
//!
//! - Non-text roles (`Border`, `Icon`, etc.) are not inferred; all
//!   non-blank cells get `Text`. Recipes targeting `Role(Border)` or
//!   similar will receive incorrect role assignments until Sub-plan C
//!   delivers real role tagging.
//! - Space characters (`' '`) are treated as background even when
//!   colored. The probe source is rendered without spatial shaders, so
//!   the fg/bg color of space cells is unlikely to carry meaningful
//!   semantics for role inference purposes.

use tui_vfx_types::{RoleMap, RoleTag};

use crate::cls_probe_grid_spec::ProbeGridSpec;

/// Infer a `RoleMap` from a `ProbeGridSpec` using glyph content.
///
/// Each cell whose `ch` is not a whitespace character is assigned
/// `RoleTag::Text`; all other cells receive `RoleTag::Background`.
///
/// This heuristic is sufficient for legacy `region: "TextOnly"` recipes
/// (which lower to `Role(Text)` scope) while keeping the inference
/// self-contained within the probe crate. Producer-supplied role maps
/// (Sub-plan C) will replace this heuristic for recipes that use
/// `Role(Border)`, `Role(Icon)`, or other non-text first-class roles.
///
/// # Panics
///
/// Does not panic. Cells are iterated from the `ProbeGridSpec.cells`
/// slice in row-major order; out-of-bounds coordinates cannot arise.
pub fn infer_roles_from_grid(grid: &ProbeGridSpec) -> RoleMap {
    let w = grid.width;
    let h = grid.height;
    let mut role_map = RoleMap::all_background(w, h);
    for (idx, cell) in grid.cells.iter().enumerate() {
        if !cell.ch.is_whitespace() {
            let x = (idx % w as usize) as u16;
            let y = (idx / w as usize) as u16;
            role_map.set((x, y), RoleTag::Text);
        }
    }
    role_map
}

#[cfg(test)]
mod tests {
    use super::*;
    use tui_vfx_types::{Cell, Color, Modifiers};

    fn empty_cell() -> Cell {
        Cell::default()
    }

    fn text_cell(ch: char) -> Cell {
        Cell::new(ch)
    }

    fn space_cell_with_color() -> Cell {
        Cell::styled(
            ' ',
            Color::new(255, 255, 255, 255),
            Color::new(0, 0, 0, 255),
            Modifiers::NONE,
        )
    }

    fn grid(width: u16, height: u16, cells: Vec<Cell>) -> ProbeGridSpec {
        ProbeGridSpec {
            width,
            height,
            cells,
        }
    }

    /// All cells empty → all Background.
    #[test]
    fn infers_all_background_for_empty_grid() {
        let spec = grid(3, 2, vec![empty_cell(); 6]);
        let roles = infer_roles_from_grid(&spec);
        for y in 0..2 {
            for x in 0..3 {
                assert_eq!(
                    roles.get((x, y)),
                    Some(RoleTag::Background),
                    "expected Background at ({x},{y})"
                );
            }
        }
    }

    /// Cells with non-whitespace characters → Text; blank cells → Background.
    #[test]
    fn infers_text_for_non_whitespace_cells() {
        // 3-wide, 1-tall: [text, blank, text]
        let spec = grid(3, 1, vec![text_cell('A'), empty_cell(), text_cell('Z')]);
        let roles = infer_roles_from_grid(&spec);
        assert_eq!(
            roles.get((0, 0)),
            Some(RoleTag::Text),
            "first cell should be Text"
        );
        assert_eq!(
            roles.get((1, 0)),
            Some(RoleTag::Background),
            "blank cell should be Background"
        );
        assert_eq!(
            roles.get((2, 0)),
            Some(RoleTag::Text),
            "third cell should be Text"
        );
    }

    /// Space character with non-zero alpha → Background (colored spaces are
    /// background fill, not text content for role-inference purposes).
    #[test]
    fn treats_colored_space_as_background() {
        let spec = grid(1, 1, vec![space_cell_with_color()]);
        let roles = infer_roles_from_grid(&spec);
        assert_eq!(
            roles.get((0, 0)),
            Some(RoleTag::Background),
            "space with color should remain Background in glyph-based inference"
        );
    }
}

// <FILE>crates/tui-vfx-probe/src/fnc_infer_roles_from_grid.rs</FILE> - <DESC>Infer a source RoleMap from a ProbeGridSpec using glyph content</DESC>
// <VERS>END OF VERSION: 1.0.0</VERS>
