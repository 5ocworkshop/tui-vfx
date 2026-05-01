// <FILE>crates/tui-vfx-compositor-next/tests/test_old_compositor_parity.rs</FILE> - <DESC>Baseline parity tests comparing copied compositor-next output with the original compositor before schema-boundary behavior changes</DESC>
// <VERS>VERSION: 0.1.0</VERS>
// <WCTX>Compositor-next Phase 2 — prove the copied runtime matches the hardened original before any vertical primitive alignment edits.</WCTX>
// <CLOG>0.1.0: add old-vs-next parity smoke for a representative mask/filter/shader render path.</CLOG>

use mixed_signals::prelude::SignalOrFloat;
use tui_vfx_style::models::{HighlighterShader, StyleRegion};
use tui_vfx_types::{Cell, Color, Grid, OwnedGrid, RoleMap, RoleTag, SemanticScene};

fn source_grid() -> OwnedGrid {
    let mut grid = OwnedGrid::new(6, 3);
    for y in 0..3 {
        for x in 0..6 {
            grid.set(
                x,
                y,
                Cell {
                    ch: char::from(b'A' + (y * 6 + x) as u8),
                    fg: Color::WHITE,
                    bg: Color::BLACK,
                    ..Default::default()
                },
            );
        }
    }
    grid
}

fn cell_snapshot(grid: &OwnedGrid) -> Vec<Cell> {
    grid.cells().to_vec()
}

fn render_old(source: &OwnedGrid, shader: &HighlighterShader) -> OwnedGrid {
    let source_roles = RoleMap::all_background(source.width() as u16, source.height() as u16);
    let destination = OwnedGrid::new(8, 5);
    let mut scene = SemanticScene::from_grid_with_default_role(destination, RoleTag::Background);
    let masks = [tui_vfx_compositor::types::MaskSpec::Checkers { cell_size: 2 }];
    let filters = [tui_vfx_compositor::types::FilterSpec::Dim {
        factor: SignalOrFloat::Static(0.75),
        apply_to: tui_vfx_compositor::types::ApplyTo::Both,
    }];
    let options = tui_vfx_compositor::pipeline::CompositionOptions::default()
        .with_masks(&masks[..])
        .with_filters(&filters[..])
        .with_shader_layer(shader, StyleRegion::All)
        .with_playback_timing(
            tui_vfx_compositor::pipeline::CompositionPlaybackTiming::new(0.5, Some(0.5), None),
        );

    tui_vfx_compositor::pipeline::render_pipeline(
        source,
        &source_roles,
        &mut scene,
        6,
        3,
        1,
        1,
        options,
        None,
    );
    scene.grid().clone()
}

fn render_next(source: &OwnedGrid, shader: &HighlighterShader) -> OwnedGrid {
    let source_roles = RoleMap::all_background(source.width() as u16, source.height() as u16);
    let destination = OwnedGrid::new(8, 5);
    let mut scene = SemanticScene::from_grid_with_default_role(destination, RoleTag::Background);
    let masks = [tui_vfx_compositor_next::types::MaskSpec::Checkers { cell_size: 2 }];
    let filters = [tui_vfx_compositor_next::types::FilterSpec::Dim {
        factor: SignalOrFloat::Static(0.75),
        apply_to: tui_vfx_compositor_next::types::ApplyTo::Both,
    }];
    let options = tui_vfx_compositor_next::pipeline::CompositionOptions::default()
        .with_masks(&masks[..])
        .with_filters(&filters[..])
        .with_shader_layer(shader, StyleRegion::All)
        .with_playback_timing(
            tui_vfx_compositor_next::pipeline::CompositionPlaybackTiming::new(0.5, Some(0.5), None),
        );

    tui_vfx_compositor_next::pipeline::render_pipeline(
        source,
        &source_roles,
        &mut scene,
        6,
        3,
        1,
        1,
        options,
        None,
    );
    scene.grid().clone()
}

#[test]
fn copied_compositor_next_matches_old_mask_filter_shader_path() {
    let source = source_grid();
    let shader = HighlighterShader::default();

    let old = render_old(&source, &shader);
    let next = render_next(&source, &shader);

    assert_eq!(old.width(), next.width());
    assert_eq!(old.height(), next.height());
    assert_eq!(cell_snapshot(&old), cell_snapshot(&next));
}

// <FILE>crates/tui-vfx-compositor-next/tests/test_old_compositor_parity.rs</FILE>
// <VERS>END OF VERSION: 0.1.0</VERS>
