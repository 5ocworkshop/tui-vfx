// <FILE>crates/tui-vfx-compositor/tests/test_render_pipeline_role_awareness.rs</FILE> - <DESC>Role-aware pipeline integration tests: asserts StyleRegion::Role(...) shaders target only the cells whose source role matches</DESC>
// <VERS>VERSION: 0.2.0</VERS>
// <WCTX>Sub-plan A Phase A.2 — TDD coverage for role-aware targeting. Proves the pipeline consults source_roles.get((src_x, src_y)) for Role(...) variants and skips cells whose role doesn't match.</WCTX>
// <CLOG>0.2.0: use BorderSweepShader + the CompositionOptions::with_shader_layer builder for ergonomic construction.
// 0.1.0: initial suite.</CLOG>

use tui_vfx_compositor::pipeline::{CompositionOptions, render_pipeline};
use tui_vfx_style::models::{ColorSpace, Gradient, LinearGradientShader, StyleRegion};
use tui_vfx_types::{Cell, Color, Grid, OwnedGrid, RoleMap, RoleTag, SemanticScene};

fn filled(w: usize, h: usize, ch: char) -> OwnedGrid {
    let mut g = OwnedGrid::new(w, h);
    for y in 0..h {
        for x in 0..w {
            g.set(
                x,
                y,
                Cell {
                    ch,
                    fg: Color::WHITE,
                    bg: Color::BLACK,
                    ..Default::default()
                },
            );
        }
    }
    g
}

// (a) Pipeline with RoleMap::all_background runs without panicking --------

#[test]
fn pipeline_with_all_background_roles_runs_cleanly() {
    let source = filled(6, 4, 'X');
    let source_roles = RoleMap::all_background(6, 4);
    let dest_grid = OwnedGrid::new(6, 4);
    let mut dest_scene =
        SemanticScene::from_grid_with_default_role(dest_grid, RoleTag::Background);
    render_pipeline(
        &source,
        &source_roles,
        &mut dest_scene,
        6,
        4,
        0,
        0,
        CompositionOptions::default(),
        None,
    );
    // Direct-copy fast path writes source cells into dest.
    assert_eq!(dest_scene.cell((0, 0)).unwrap().ch, 'X');
    assert_eq!(dest_scene.cell((5, 3)).unwrap().ch, 'X');
}

// Shared helper: build a source whose first row is tagged Border, second
// row is tagged Text, remainder Background; every cell contains 'X'.
fn striped_source_and_roles(w: u16, h: u16) -> (OwnedGrid, RoleMap) {
    let grid = filled(w as usize, h as usize, 'X');
    let mut roles = RoleMap::all_background(w, h);
    for x in 0..w {
        roles.set((x, 0), RoleTag::Border);
        if h >= 2 {
            roles.set((x, 1), RoleTag::Text);
        }
    }
    (grid, roles)
}

fn render_with_shader(region: StyleRegion) -> SemanticScene {
    let (source, source_roles) = striped_source_and_roles(6, 4);
    let dest_grid = OwnedGrid::new(6, 4);
    let mut dest_scene =
        SemanticScene::from_grid_with_default_role(dest_grid, RoleTag::Background);
    // LinearGradientShader paints every cell its predicate selects — a
    // reliable witness to whether the role predicate admitted each cell.
    let shader = LinearGradientShader {
        gradient: Gradient {
            stops: vec![
                (0.0, Color::rgb(20, 120, 220)),
                (1.0, Color::rgb(20, 120, 220)),
            ],
            space: ColorSpace::Rgb,
        },
        angle_deg: 0.0,
    };
    let options = CompositionOptions::default().with_shader_layer(&shader, region);
    render_pipeline(
        &source,
        &source_roles,
        &mut dest_scene,
        6,
        4,
        0,
        0,
        options,
        None,
    );
    dest_scene
}

fn cell_was_painted_by_shader(cell: &Cell) -> bool {
    // The shader writes a blue fg; default fg is WHITE. If shader ran,
    // fg is no longer white.
    cell.fg != Color::WHITE
}

// (b) Role(Border) targets only border-tagged cells -----------------------

#[test]
fn role_border_shader_only_affects_border_row() {
    let dest = render_with_shader(StyleRegion::Role(RoleTag::Border));
    // Row 0 is Border → shader ran
    assert!(
        cell_was_painted_by_shader(dest.cell((3, 0)).unwrap()),
        "cell at (3,0) with Border role should have been painted"
    );
    // Row 1 is Text → not affected
    assert!(
        !cell_was_painted_by_shader(dest.cell((3, 1)).unwrap()),
        "cell at (3,1) with Text role should NOT have been painted by a Border-role shader"
    );
    // Row 3 is Background → not affected
    assert!(
        !cell_was_painted_by_shader(dest.cell((3, 3)).unwrap()),
        "cell at (3,3) with Background role should NOT have been painted"
    );
}

// (c) Role(Text) targets only text-tagged cells --------------------------

#[test]
fn role_text_shader_only_affects_text_row() {
    let dest = render_with_shader(StyleRegion::Role(RoleTag::Text));
    assert!(
        !cell_was_painted_by_shader(dest.cell((3, 0)).unwrap()),
        "border row should be untouched by a Text-role shader"
    );
    assert!(
        cell_was_painted_by_shader(dest.cell((3, 1)).unwrap()),
        "text row should be painted"
    );
    assert!(
        !cell_was_painted_by_shader(dest.cell((3, 3)).unwrap()),
        "background row should be untouched"
    );
}

// (d) Role(Highlight) has no matching cells — nothing is painted ---------

#[test]
fn role_with_no_matching_cells_leaves_destination_untouched_by_shader() {
    let dest = render_with_shader(StyleRegion::Role(RoleTag::Highlight));
    for y in 0..4 {
        for x in 0..6 {
            let c = dest.cell((x, y)).unwrap();
            assert!(
                !cell_was_painted_by_shader(c),
                "no cell should have been painted by the Highlight shader at ({x},{y})"
            );
        }
    }
}

// <FILE>crates/tui-vfx-compositor/tests/test_render_pipeline_role_awareness.rs</FILE>
// <VERS>END OF VERSION: 0.2.0</VERS>
