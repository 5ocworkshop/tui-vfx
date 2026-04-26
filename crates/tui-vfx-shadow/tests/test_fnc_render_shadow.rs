// <FILE>crates/tui-vfx-shadow/tests/test_fnc_render_shadow.rs</FILE> - <DESC>Integration tests for the role-aware render_shadow_into_scene entrypoint — back-compat with None source_region and role-filter behavior with Some source_region, plus destination RoleTag::Shadow write-back</DESC>
// <VERS>VERSION: 0.1.1</VERS>
// <WCTX>Sub-plan A Phase A.3.5 — TDD red→green for shadow stage writing RoleTag::Shadow into destination roles; back-compat (None) produces today's rect-based extrusion; Some(role) uses role-filtered bbox for extrusion.</WCTX>
// <CLOG>0.1.1: collapse nested if-let in count_nonempty_cells helper into a let-chain to clear clippy::collapsible_if under -D warnings.</CLOG>

//! Integration tests for `render_shadow_into_scene` (the role-aware shadow
//! stage entrypoint) introduced in Sub-plan A Phase A.3.5.
//!
//! Scope:
//! 1. **Back-compat**: when `source_region` is `None`, shadow cells match
//!    the legacy `render_shadow(grid, element_rect, ...)` output.
//! 2. **Role filter**: when `source_region` is `Some(role)`, extrusion is
//!    restricted to the bounding rectangle of role-matched source cells.
//! 3. **Role write-back**: every cell the shadow stage wrote (shadow cells
//!    outside the source rect) has `RoleTag::Shadow` in the destination
//!    `RoleMap`.
//! 4. **Tiny-rect safety**: 1×1 and 2×1 sources do not panic.

use tui_vfx_shadow::{
    ShadowConfig, ShadowEdges, ShadowStyle, render_shadow, render_shadow_into_scene,
};
use tui_vfx_types::{Cell, Color, Grid, OwnedGrid, Rect, RoleMap, RoleTag, SemanticScene};

fn filled_grid(width: usize, height: usize, rect: Rect) -> OwnedGrid {
    let mut g = OwnedGrid::new(width, height);
    for y in rect.y..rect.y + rect.height {
        for x in rect.x..rect.x + rect.width {
            g.set(x as usize, y as usize, Cell::new('X'));
        }
    }
    g
}

fn card_config() -> ShadowConfig {
    ShadowConfig::new(Color::BLACK.with_alpha(200))
        .with_style(ShadowStyle::HalfBlock)
        .with_offset(2, 1)
        .with_edges(ShadowEdges::BOTTOM_RIGHT)
}

/// Count cells whose character is non-space OR whose bg alpha is non-zero.
fn count_nonempty_cells(g: &OwnedGrid) -> usize {
    let mut n = 0usize;
    for y in 0..g.height() {
        for x in 0..g.width() {
            if let Some(cell) = g.get(x, y)
                && (cell.ch != ' ' || cell.bg.a != 0 || cell.fg.a != 0)
            {
                n += 1;
            }
        }
    }
    n
}

#[test]
fn back_compat_none_source_region_matches_legacy_render() {
    // Legacy: render_shadow directly into a scratch grid.
    let mut legacy = OwnedGrid::new(20, 10);
    let element_rect = Rect::new(5, 2, 8, 4);
    let config = card_config();
    render_shadow(&mut legacy, element_rect, &config, 1.0);

    // New path: render_shadow_into_scene with source_region None — should
    // produce the same shadow cells on the underlying grid.
    let source_grid = filled_grid(20, 10, element_rect);
    let source_roles = RoleMap::empty(20, 10);
    let mut scene =
        SemanticScene::from_grid_with_default_role(OwnedGrid::new(20, 10), RoleTag::Background);
    render_shadow_into_scene(
        &source_grid,
        &source_roles,
        &mut scene,
        element_rect,
        &config,
        1.0,
    );

    // Cells must match bit-for-bit on the grid portion.
    for y in 0..10 {
        for x in 0..20 {
            let legacy_cell = legacy.get(x, y);
            let scene_cell = scene.grid().get(x, y);
            assert_eq!(
                legacy_cell, scene_cell,
                "mismatch at ({x}, {y}): legacy={:?} scene={:?}",
                legacy_cell, scene_cell
            );
        }
    }
}

#[test]
fn shadow_stage_writes_shadow_role_tag_at_produced_cells() {
    let element_rect = Rect::new(5, 2, 8, 4);
    let source_grid = filled_grid(20, 10, element_rect);
    let source_roles = RoleMap::empty(20, 10);
    let mut scene =
        SemanticScene::from_grid_with_default_role(OwnedGrid::new(20, 10), RoleTag::Background);
    let config = card_config();

    render_shadow_into_scene(
        &source_grid,
        &source_roles,
        &mut scene,
        element_rect,
        &config,
        1.0,
    );

    // For every cell the shadow stage produced (non-empty on the destination
    // grid), the destination role map must carry RoleTag::Shadow.
    let mut shadow_cells = 0usize;
    for y in 0..scene.grid().height() {
        for x in 0..scene.grid().width() {
            if let Some(cell) = scene.grid().get(x, y) {
                let produced = cell.ch != ' ' || cell.bg.a != 0 || cell.fg.a != 0;
                if produced {
                    let role = scene.role((x as u16, y as u16));
                    assert_eq!(
                        role,
                        Some(RoleTag::Shadow),
                        "cell ({x},{y}) produced by shadow stage but role is {:?}",
                        role
                    );
                    shadow_cells += 1;
                }
            }
        }
    }
    assert!(
        shadow_cells > 0,
        "expected some shadow cells; got {shadow_cells}"
    );
}

#[test]
fn source_region_border_restricts_extrusion_to_role_bbox() {
    // Source is a 20x10 grid where only a 2-row border strip has Border role
    // (rows 2..4, the top two rows of the element). Setting source_region
    // Border should make shadow extrude from that 2-row bounding rect, not
    // the full 8x4 element rect.
    let full_rect = Rect::new(5, 2, 8, 4);
    let border_rect = Rect::new(5, 2, 8, 2);
    let source_grid = filled_grid(20, 10, full_rect);
    let mut source_roles = RoleMap::new_with_default(20, 10, RoleTag::Text);
    for y in border_rect.y..border_rect.y + border_rect.height {
        for x in border_rect.x..border_rect.x + border_rect.width {
            source_roles.set((x, y), RoleTag::Border);
        }
    }

    let mut scene =
        SemanticScene::from_grid_with_default_role(OwnedGrid::new(20, 10), RoleTag::Background);
    let config = card_config().with_source_region(RoleTag::Border);
    render_shadow_into_scene(
        &source_grid,
        &source_roles,
        &mut scene,
        full_rect,
        &config,
        1.0,
    );

    // Shadow should extrude from the BORDER bbox (smaller) — meaning no
    // shadow cells appear below the border bbox's bottom edge + offset_y.
    // border bbox bottom row is y=3; with offset_y=1 the shadow extends to
    // at most y=4. Anything at y>=5 must be empty.
    for y in 5..10u16 {
        for x in 0..20u16 {
            let cell = scene
                .grid()
                .get(x as usize, y as usize)
                .expect("cell in bounds");
            let produced = cell.ch != ' ' || cell.bg.a != 0 || cell.fg.a != 0;
            assert!(
                !produced,
                "shadow leaked below border bbox at ({x},{y}): {:?}",
                cell
            );
        }
    }

    // And at least one shadow cell must still be produced (sanity).
    assert!(count_nonempty_cells(scene.grid()) > 0);
}

#[test]
fn source_region_with_no_matches_produces_no_shadow() {
    let full_rect = Rect::new(5, 2, 8, 4);
    let source_grid = filled_grid(20, 10, full_rect);
    // Whole grid is Text, so Border source_region will match nothing.
    let source_roles = RoleMap::new_with_default(20, 10, RoleTag::Text);
    let mut scene =
        SemanticScene::from_grid_with_default_role(OwnedGrid::new(20, 10), RoleTag::Background);
    let config = card_config().with_source_region(RoleTag::Border);

    render_shadow_into_scene(
        &source_grid,
        &source_roles,
        &mut scene,
        full_rect,
        &config,
        1.0,
    );

    assert_eq!(count_nonempty_cells(scene.grid()), 0);
}

#[test]
fn tiny_1x1_source_does_not_panic() {
    let element_rect = Rect::new(0, 0, 1, 1);
    let source_grid = filled_grid(4, 4, element_rect);
    let source_roles = RoleMap::all_background(4, 4);
    let mut scene =
        SemanticScene::from_grid_with_default_role(OwnedGrid::new(4, 4), RoleTag::Background);
    let config = card_config();
    render_shadow_into_scene(
        &source_grid,
        &source_roles,
        &mut scene,
        element_rect,
        &config,
        1.0,
    );
    // No panic; no assertion on content beyond survival.
}

#[test]
fn tiny_2x1_source_does_not_panic() {
    let element_rect = Rect::new(0, 0, 2, 1);
    let source_grid = filled_grid(6, 4, element_rect);
    let source_roles = RoleMap::all_background(6, 4);
    let mut scene =
        SemanticScene::from_grid_with_default_role(OwnedGrid::new(6, 4), RoleTag::Background);
    let config = card_config();
    render_shadow_into_scene(
        &source_grid,
        &source_roles,
        &mut scene,
        element_rect,
        &config,
        1.0,
    );
}

// <FILE>crates/tui-vfx-shadow/tests/test_fnc_render_shadow.rs</FILE> - <DESC>Integration tests for render_shadow_into_scene</DESC>
// <VERS>END OF VERSION: 0.1.0</VERS>
