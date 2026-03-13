// <FILE>crates/tui-vfx-compositor/tests/pipeline/test_orc_render_pipeline.rs</FILE> - <DESC>L2 render pipeline tests with Grid trait</DESC>
// <VERS>VERSION: 5.4.0</VERS>
// <WCTX>Color-inert glyph detection for shadow grading replacement</WCTX>
// <CLOG>Add integration tests for color-inert glyph replacement in grade-underlying pipeline</CLOG>

use mixed_signals::prelude::SignalOrFloat;
use std::borrow::Cow;
use tui_vfx_compositor::pipeline::{CompositionOptions, ShadowSpec, render_pipeline};
use tui_vfx_compositor::types::{
    ApplyTo, FilterSpec, MaskCombineMode, MaskSpec, SamplerSpec, WipeDirection,
};
use tui_vfx_shadow::{ShadowConfig, ShadowEdges};
use tui_vfx_types::{Cell, Color, Grid, GridExt, OwnedGrid};

/// Helper to create a source grid with content
fn create_source_grid(width: usize, height: usize, fill_char: char) -> OwnedGrid {
    let mut grid = OwnedGrid::new(width, height);
    for y in 0..height {
        for x in 0..width {
            let cell = Cell {
                ch: fill_char,
                fg: Color::WHITE,
                bg: Color::BLACK,
                ..Default::default()
            };
            grid.set(x, y, cell);
        }
    }
    grid
}

// ============================================================================
// BASIC PIPELINE TESTS
// ============================================================================

#[test]
fn test_pipeline_direct_copy_no_effects() {
    let source = create_source_grid(10, 5, 'X');
    let mut dest = OwnedGrid::new(10, 5);

    render_pipeline(
        &source,
        &mut dest,
        10,
        5,
        0,
        0,
        CompositionOptions::default(),
        None,
    );

    // Content should be copied directly
    assert_eq!(dest.get(0, 0).unwrap().ch, 'X');
    assert_eq!(dest.get(9, 4).unwrap().ch, 'X');
}

#[test]
fn test_pipeline_with_offset() {
    let source = create_source_grid(5, 5, 'S');
    let mut dest = OwnedGrid::new(20, 20);

    // Fill dest with dots first
    dest.fill(Cell {
        ch: '.',
        ..Default::default()
    });

    render_pipeline(
        &source,
        &mut dest,
        5,
        5,
        10, // offset_x
        10, // offset_y
        CompositionOptions::default(),
        None,
    );

    // Offset position should have source content
    assert_eq!(dest.get(10, 10).unwrap().ch, 'S');
    assert_eq!(dest.get(14, 14).unwrap().ch, 'S');

    // Non-offset positions should still have dots
    assert_eq!(dest.get(0, 0).unwrap().ch, '.');
    assert_eq!(dest.get(5, 5).unwrap().ch, '.');
}

// ============================================================================
// MASK TESTS
// ============================================================================

#[test]
fn test_mask_wipe_at_zero_hides_all() {
    let source = create_source_grid(10, 5, 'X');
    let mut dest = OwnedGrid::new(10, 5);

    // Pre-fill dest with dots
    dest.fill(Cell {
        ch: '.',
        ..Default::default()
    });

    render_pipeline(
        &source,
        &mut dest,
        10,
        5,
        0,
        0,
        CompositionOptions {
            masks: Cow::Owned(vec![MaskSpec::Wipe {
                reveal: Some(WipeDirection::LeftToRight),
                hide: None,
                direction: None,
                soft_edge: false,
            }]),
            t: 0.0, // At t=0, wipe should hide everything
            ..Default::default()
        },
        None,
    );

    // Dest should still have dots (nothing copied)
    assert_eq!(dest.get(0, 0).unwrap().ch, '.');
}

#[test]
fn test_mask_wipe_at_one_shows_all() {
    let source = create_source_grid(10, 5, 'X');
    let mut dest = OwnedGrid::new(10, 5);

    render_pipeline(
        &source,
        &mut dest,
        10,
        5,
        0,
        0,
        CompositionOptions {
            masks: Cow::Owned(vec![MaskSpec::Wipe {
                reveal: Some(WipeDirection::LeftToRight),
                hide: None,
                direction: None,
                soft_edge: false,
            }]),
            t: 1.0, // At t=1, wipe should show everything
            ..Default::default()
        },
        None,
    );

    // Source content should be visible
    assert_eq!(dest.get(0, 0).unwrap().ch, 'X');
}

#[test]
fn test_mask_checkers_creates_pattern() {
    let source = create_source_grid(10, 10, 'X');
    let mut dest = OwnedGrid::new(10, 10);

    // Pre-fill dest
    dest.fill(Cell {
        ch: '.',
        ..Default::default()
    });

    render_pipeline(
        &source,
        &mut dest,
        10,
        10,
        0,
        0,
        CompositionOptions {
            masks: Cow::Owned(vec![MaskSpec::Checkers { cell_size: 1 }]),
            t: 0.5, // Partial reveal
            ..Default::default()
        },
        None,
    );

    // Should have a checkerboard pattern - some X, some .
    let cell_00 = dest.get(0, 0).unwrap().ch;
    let cell_01 = dest.get(0, 1).unwrap().ch;

    // Checkerboard should alternate
    assert_ne!(cell_00, cell_01, "Checkerboard should alternate");
}

#[test]
fn test_multi_mask_any_mode() {
    let source = create_source_grid(10, 5, 'X');
    let mut dest = OwnedGrid::new(10, 5);

    render_pipeline(
        &source,
        &mut dest,
        10,
        5,
        0,
        0,
        CompositionOptions {
            masks: vec![
                MaskSpec::Checkers { cell_size: 1 },
                MaskSpec::Wipe {
                    reveal: Some(WipeDirection::LeftToRight),
                    hide: None,
                    direction: None,
                    soft_edge: false,
                },
            ]
            .into(),
            mask_combine_mode: MaskCombineMode::Any,
            t: 1.0,
            ..Default::default()
        },
        None,
    );

    // With Any mode and t=1.0, content should be visible
    assert_eq!(dest.get(0, 0).unwrap().ch, 'X');
}

// ============================================================================
// FILTER TESTS
// ============================================================================

#[test]
fn test_filter_dim_reduces_brightness() {
    let mut source = OwnedGrid::new(5, 5);
    source.fill(Cell {
        ch: 'X',
        fg: Color::WHITE,
        bg: Color::BLACK,
        ..Default::default()
    });

    let mut dest = OwnedGrid::new(5, 5);

    render_pipeline(
        &source,
        &mut dest,
        5,
        5,
        0,
        0,
        CompositionOptions {
            filters: Cow::Owned(vec![FilterSpec::Dim {
                factor: SignalOrFloat::Static(0.5),
                apply_to: ApplyTo::Both,
            }]),
            t: 1.0,
            ..Default::default()
        },
        None,
    );

    // FG should be dimmed (WHITE -> ~half brightness)
    let cell = dest.get(0, 0).unwrap();
    assert!(cell.fg.r < 200, "Red component should be dimmed");
}

#[test]
fn test_filter_invert_swaps_colors() {
    let mut source = OwnedGrid::new(5, 5);
    source.fill(Cell {
        ch: 'X',
        fg: Color::rgb(255, 0, 0), // Red
        bg: Color::rgb(0, 0, 255), // Blue
        ..Default::default()
    });

    let mut dest = OwnedGrid::new(5, 5);

    render_pipeline(
        &source,
        &mut dest,
        5,
        5,
        0,
        0,
        CompositionOptions {
            filters: Cow::Owned(vec![FilterSpec::Invert {
                apply_to: ApplyTo::Both,
            }]),
            t: 1.0,
            ..Default::default()
        },
        None,
    );

    // Invert filter swaps fg and bg colors
    // fg (Red) should now be Blue, bg (Blue) should now be Red
    let cell = dest.get(0, 0).unwrap();
    assert_eq!(
        cell.fg,
        Color::rgb(0, 0, 255),
        "FG should be Blue (swapped from BG)"
    );
    assert_eq!(
        cell.bg,
        Color::rgb(255, 0, 0),
        "BG should be Red (swapped from FG)"
    );
}

#[test]
fn test_filter_vignette_darkens_edges() {
    let mut source = OwnedGrid::new(20, 10);
    source.fill(Cell {
        ch: 'X',
        fg: Color::WHITE,
        bg: Color::WHITE,
        ..Default::default()
    });

    let mut dest = OwnedGrid::new(20, 10);

    render_pipeline(
        &source,
        &mut dest,
        20,
        10,
        0,
        0,
        CompositionOptions {
            filters: Cow::Owned(vec![FilterSpec::Vignette {
                strength: SignalOrFloat::Static(0.8),
                radius: SignalOrFloat::Static(0.3),
            }]),
            t: 1.0,
            ..Default::default()
        },
        None,
    );

    // Center should be brighter than edges
    let center = dest.get(10, 5).unwrap();
    let corner = dest.get(0, 0).unwrap();

    assert!(
        center.fg.r > corner.fg.r,
        "Center should be brighter than corner"
    );
}

#[test]
fn test_filter_crt_creates_scanlines() {
    let mut source = OwnedGrid::new(10, 10);
    source.fill(Cell {
        ch: 'X',
        fg: Color::WHITE,
        bg: Color::WHITE,
        ..Default::default()
    });

    let mut dest = OwnedGrid::new(10, 10);

    render_pipeline(
        &source,
        &mut dest,
        10,
        10,
        0,
        0,
        CompositionOptions {
            filters: Cow::Owned(vec![FilterSpec::Crt {
                scanline_strength: SignalOrFloat::Static(0.5),
                glow: SignalOrFloat::Static(0.0),
            }]),
            t: 1.0,
            ..Default::default()
        },
        None,
    );

    // Even and odd rows should have different brightness
    let row_0 = dest.get(5, 0).unwrap();
    let row_1 = dest.get(5, 1).unwrap();

    assert_ne!(
        row_0.fg.r, row_1.fg.r,
        "CRT scanlines should create alternating brightness"
    );
}

// ============================================================================
// SAMPLER TESTS
// ============================================================================

#[test]
fn test_sampler_none_passthrough() {
    let source = create_source_grid(10, 5, 'X');
    let mut dest = OwnedGrid::new(10, 5);

    render_pipeline(
        &source,
        &mut dest,
        10,
        5,
        0,
        0,
        CompositionOptions {
            sampler_spec: Some(SamplerSpec::None),
            t: 1.0,
            ..Default::default()
        },
        None,
    );

    // Content should pass through unchanged
    assert_eq!(dest.get(0, 0).unwrap().ch, 'X');
}

// ============================================================================
// COMBINED EFFECT TESTS
// ============================================================================

#[test]
fn test_combined_mask_and_filter() {
    let source = create_source_grid(10, 5, 'X');
    let mut dest = OwnedGrid::new(10, 5);

    dest.fill(Cell {
        ch: '.',
        ..Default::default()
    });

    render_pipeline(
        &source,
        &mut dest,
        10,
        5,
        0,
        0,
        CompositionOptions {
            masks: Cow::Owned(vec![MaskSpec::Wipe {
                reveal: Some(WipeDirection::LeftToRight),
                hide: None,
                direction: None,
                soft_edge: false,
            }]),
            filters: Cow::Owned(vec![FilterSpec::Dim {
                factor: SignalOrFloat::Static(0.5),
                apply_to: ApplyTo::Both,
            }]),
            t: 1.0,
            ..Default::default()
        },
        None,
    );

    // Should have X (revealed by wipe) with dimmed colors
    let cell = dest.get(0, 0).unwrap();
    assert_eq!(cell.ch, 'X');
}

// ============================================================================
// EDGE CASE TESTS
// ============================================================================

#[test]
fn test_zero_dimensions_no_panic() {
    let source = OwnedGrid::new(0, 0);
    let mut dest = OwnedGrid::new(0, 0);

    render_pipeline(
        &source,
        &mut dest,
        0,
        0,
        0,
        0,
        CompositionOptions {
            masks: Cow::Owned(vec![MaskSpec::Checkers { cell_size: 2 }]),
            filters: Cow::Owned(vec![FilterSpec::Dim {
                factor: SignalOrFloat::Static(0.5),
                apply_to: ApplyTo::Both,
            }]),
            t: 0.5,
            ..Default::default()
        },
        None,
    );

    // Test passes if no panic occurs
}

#[test]
fn test_t_boundaries() {
    let t_values = vec![0.0, 0.001, 0.5, 0.999, 1.0];

    for t in t_values {
        let source = create_source_grid(10, 5, 'X');
        let mut dest = OwnedGrid::new(10, 5);

        render_pipeline(
            &source,
            &mut dest,
            10,
            5,
            0,
            0,
            CompositionOptions {
                masks: Cow::Owned(vec![MaskSpec::Wipe {
                    reveal: Some(WipeDirection::BottomToTop),
                    hide: None,
                    direction: None,
                    soft_edge: true,
                }]),
                t,
                ..Default::default()
            },
            None,
        );
    }
    // Test passes if no panic occurs for any t value
}

#[test]
fn test_empty_effects_passthrough() {
    let source = create_source_grid(10, 5, 'X');
    let mut dest = OwnedGrid::new(10, 5);

    render_pipeline(
        &source,
        &mut dest,
        10,
        5,
        0,
        0,
        CompositionOptions {
            masks: Cow::Borrowed(&[]),
            filters: Cow::Borrowed(&[]),
            sampler_spec: None,
            t: 1.0,
            ..Default::default()
        },
        None,
    );

    // No effects = direct copy
    assert_eq!(dest.get(0, 0).unwrap().ch, 'X');
}

// ============================================================================
// SHADOW TESTS
// ============================================================================

#[test]
fn test_shadow_extends_render_area() {
    // Create a 10x5 element with shadow offset (2, 1)
    // Expected render area: 12x6
    let source = create_source_grid(10, 5, 'X');
    let mut dest = OwnedGrid::new(20, 20);

    // Fill dest with dots
    dest.fill(Cell {
        ch: '.',
        ..Default::default()
    });

    let shadow_config = ShadowConfig::new(Color::BLACK.with_alpha(200))
        .with_offset(2, 1)
        .with_edges(ShadowEdges::BOTTOM_RIGHT);

    render_pipeline(
        &source,
        &mut dest,
        10,
        5,
        0,
        0,
        CompositionOptions {
            shadow: Some(ShadowSpec::new(shadow_config)),
            t: 1.0,
            ..Default::default()
        },
        None,
    );

    // Element content should be present at (0,0) to (9,4)
    assert_eq!(dest.get(0, 0).unwrap().ch, 'X', "Element content at origin");
    assert_eq!(dest.get(9, 4).unwrap().ch, 'X', "Element content at corner");

    // Shadow should extend to (11,5) - the extended area
    // For HalfBlock with offset (2,1), right edge shadow is at x=10-11
    // x=10 uses 25% shadow with bg=shadow, x=11 uses 50% shadow with fg=shadow
    // Right edge spans y=1 to y=4 (adjusted by offset_y)
    let solid_shadow = dest.get(11, 2).unwrap();
    assert_ne!(
        solid_shadow.fg,
        Color::TRANSPARENT,
        "Right-edge shadow should be rendered at x=11"
    );

    // Soft edge shadow at x=10 uses fg for half-block character (bg is transparent surface)
    let soft_edge_shadow = dest.get(10, 2).unwrap();
    assert_ne!(
        soft_edge_shadow.fg,
        Color::TRANSPARENT,
        "Soft edge shadow should be rendered at x=10 (fg carries shadow in half-block)"
    );

    // Bottom edge shadow at y=5, adjusted x range (x=2 to x=9 due to offset)
    // With soft edges, the bottom row uses LOWER_HALF with fg=shadow
    let bottom_shadow = dest.get(5, 5).unwrap();
    assert_ne!(
        bottom_shadow.fg,
        Color::TRANSPARENT,
        "Bottom shadow soft edge should have fg color"
    );

    // Beyond extended area should still be dots
    assert_eq!(dest.get(15, 10).unwrap().ch, '.', "Beyond shadow area");
}

/// Helper to check if a cell has shadow (either in fg or bg)
fn has_shadow_color(cell: &Cell) -> bool {
    cell.fg.a > 0 || cell.bg.a > 0
}

#[test]
fn test_shadow_with_wipe_mask() {
    // Test that shadow wipes in sync with element
    let source = create_source_grid(10, 5, 'X');
    let mut dest = OwnedGrid::new(20, 20);

    // Fill dest with dots
    dest.fill(Cell {
        ch: '.',
        ..Default::default()
    });

    let shadow_config = ShadowConfig::new(Color::BLACK.with_alpha(200))
        .with_offset(2, 1)
        .with_edges(ShadowEdges::BOTTOM_RIGHT);

    // Wipe at t=0 should hide everything (element and shadow)
    render_pipeline(
        &source,
        &mut dest,
        10,
        5,
        0,
        0,
        CompositionOptions {
            shadow: Some(ShadowSpec::new(shadow_config)),
            masks: Cow::Owned(vec![MaskSpec::Wipe {
                reveal: Some(WipeDirection::TopToBottom),
                hide: None,
                direction: None,
                soft_edge: false,
            }]),
            t: 0.0,
            ..Default::default()
        },
        None,
    );

    // At t=0, nothing should be revealed
    assert_eq!(dest.get(0, 0).unwrap().ch, '.', "Element hidden at t=0");
    assert_eq!(dest.get(11, 2).unwrap().ch, '.', "Shadow hidden at t=0");

    // Now test at t=1.0 (fully revealed)
    dest.fill(Cell {
        ch: '.',
        ..Default::default()
    });

    render_pipeline(
        &source,
        &mut dest,
        10,
        5,
        0,
        0,
        CompositionOptions {
            shadow: Some(ShadowSpec::new(shadow_config)),
            masks: Cow::Owned(vec![MaskSpec::Wipe {
                reveal: Some(WipeDirection::TopToBottom),
                hide: None,
                direction: None,
                soft_edge: false,
            }]),
            t: 1.0,
            ..Default::default()
        },
        None,
    );

    // At t=1.0, element and shadow should be visible
    assert_eq!(dest.get(0, 0).unwrap().ch, 'X', "Element visible at t=1");
    // Check solid shadow at x=11 (not soft edge at x=10)
    assert!(
        has_shadow_color(dest.get(11, 2).unwrap()),
        "Shadow visible at t=1"
    );
}

#[test]
fn test_shadow_partial_wipe() {
    // Test that at t=0.5, top half is revealed
    let source = create_source_grid(10, 10, 'X');
    let mut dest = OwnedGrid::new(20, 20);

    dest.fill(Cell {
        ch: '.',
        ..Default::default()
    });

    let shadow_config = ShadowConfig::new(Color::BLACK.with_alpha(200))
        .with_offset(2, 1)
        .with_edges(ShadowEdges::BOTTOM_RIGHT);

    // Extended area is 12x11 for a 10x10 element with offset (2,1)
    // At t=0.5 with top-to-bottom wipe, approximately top 5.5 rows visible
    render_pipeline(
        &source,
        &mut dest,
        10,
        10,
        0,
        0,
        CompositionOptions {
            shadow: Some(ShadowSpec::new(shadow_config)),
            masks: Cow::Owned(vec![MaskSpec::Wipe {
                reveal: Some(WipeDirection::TopToBottom),
                hide: None,
                direction: None,
                soft_edge: false,
            }]),
            t: 0.5,
            ..Default::default()
        },
        None,
    );

    // Top of element should be visible
    assert_eq!(dest.get(5, 0).unwrap().ch, 'X', "Top of element visible");

    // Bottom of element should be hidden (row 9 of element, row 9 of extended area)
    // At t=0.5, extended height 11 -> reveal up to row ~5
    assert_eq!(dest.get(5, 9).unwrap().ch, '.', "Bottom of element hidden");

    // Right shadow starts at y=1 (due to offset_y=1), check y=2 which should be visible
    // Use x=11 for solid shadow (x=10 is soft edge)
    assert!(
        has_shadow_color(dest.get(11, 2).unwrap()),
        "Top portion of right shadow visible"
    );

    // Bottom shadow (at row 10) should be hidden
    assert_eq!(dest.get(5, 10).unwrap().ch, '.', "Bottom shadow hidden");
}

#[test]
fn test_shadow_with_offset() {
    // Test shadow rendering at a non-zero offset position
    let source = create_source_grid(5, 5, 'X');
    let mut dest = OwnedGrid::new(20, 20);

    dest.fill(Cell {
        ch: '.',
        ..Default::default()
    });

    // Use offset (2, 2) so we have both soft edge and solid cells
    let shadow_config = ShadowConfig::new(Color::BLACK.with_alpha(200))
        .with_offset(2, 2)
        .with_edges(ShadowEdges::BOTTOM_RIGHT);

    render_pipeline(
        &source,
        &mut dest,
        5,
        5,
        10, // Place at x=10
        5,  // Place at y=5
        CompositionOptions {
            shadow: Some(ShadowSpec::new(shadow_config)),
            t: 1.0,
            ..Default::default()
        },
        None,
    );

    // Element should be at (10,5) to (14,9)
    assert_eq!(dest.get(10, 5).unwrap().ch, 'X', "Element at offset");
    assert_eq!(dest.get(14, 9).unwrap().ch, 'X', "Element corner at offset");

    // Shadow right edge: x=15-16 (offset_x=2), y=7-9 (adjusted by offset_y=2)
    // x=16 is solid shadow (x=15 is soft edge)
    assert!(
        has_shadow_color(dest.get(16, 8).unwrap()),
        "Shadow solid at offset"
    );

    // Shadow bottom edge: y=10-11 (offset_y=2), x=12-14 (adjusted by offset_x=2)
    // y=11 is solid shadow (y=10 is soft edge)
    assert!(
        has_shadow_color(dest.get(13, 11).unwrap()),
        "Shadow bottom solid at offset"
    );
}

#[test]
fn test_shadow_progress_controls_opacity() {
    // Test that shadow fades with progress
    let source = create_source_grid(10, 5, 'X');
    let mut dest_half = OwnedGrid::new(20, 20);
    let mut dest_full = OwnedGrid::new(20, 20);

    let shadow_config = ShadowConfig::new(Color::BLACK.with_alpha(200))
        .with_offset(2, 1)
        .with_edges(ShadowEdges::BOTTOM_RIGHT);

    // Render at t=0.5
    render_pipeline(
        &source,
        &mut dest_half,
        10,
        5,
        0,
        0,
        CompositionOptions {
            shadow: Some(ShadowSpec::new(shadow_config)),
            t: 0.5, // Half progress = half shadow opacity
            ..Default::default()
        },
        None,
    );

    // Render at t=1.0
    render_pipeline(
        &source,
        &mut dest_full,
        10,
        5,
        0,
        0,
        CompositionOptions {
            shadow: Some(ShadowSpec::new(shadow_config)),
            t: 1.0, // Full progress = full shadow opacity
            ..Default::default()
        },
        None,
    );

    // Check solid shadow at x=11 (not soft edge at x=10)
    // Right shadow spans y=1-4 due to offset_y=1
    let shadow_half = dest_half.get(11, 2).unwrap();
    let shadow_full = dest_full.get(11, 2).unwrap();

    // Both should have shadow (non-transparent bg for solid cells)
    assert!(has_shadow_color(shadow_half), "Shadow at t=0.5");
    assert!(has_shadow_color(shadow_full), "Shadow at t=1.0");

    // Full opacity shadow should have higher alpha
    // For solid cells, shadow is in bg
    assert!(
        shadow_full.bg.a >= shadow_half.bg.a,
        "Full shadow should have >= alpha ({}) than half shadow ({})",
        shadow_full.bg.a,
        shadow_half.bg.a
    );
}

// ============================================================================
// GRADE-UNDERLYING SHADOW TESTS (Phase 1)
// ============================================================================

/// BT.601 luma for test assertions.
fn bt601_luma(c: Color) -> f32 {
    0.299 * c.r as f32 + 0.587 * c.g as f32 + 0.114 * c.b as f32
}

/// Saturation span for test assertions.
fn saturation_span(c: Color) -> u8 {
    let max = c.r.max(c.g).max(c.b);
    let min = c.r.min(c.g).min(c.b);
    max - min
}

/// Create a dest grid filled with colored content for grade-underlying tests.
/// Uses the canonical sample colors from the plan.
fn create_grade_dest_grid(width: usize, height: usize) -> OwnedGrid {
    let mut grid = OwnedGrid::new(width, height);
    for y in 0..height {
        for x in 0..width {
            let cell = Cell {
                ch: 'A',
                fg: Color::rgb(220, 180, 80),
                bg: Color::rgb(90, 110, 140),
                mods: tui_vfx_types::Modifiers {
                    bold: true,
                    ..Default::default()
                },
                mod_alpha: Some(200),
            };
            grid.set(x, y, cell);
        }
    }
    grid
}

#[test]
fn test_shadow_grade_underlying_preserves_destination_glyphs() {
    let source = create_source_grid(10, 5, 'X');
    let mut dest = create_grade_dest_grid(20, 20);

    let shadow_config = ShadowConfig::new(Color::BLACK.with_alpha(180))
        .with_offset(2, 1)
        .with_edges(ShadowEdges::BOTTOM_RIGHT)
        .with_style(tui_vfx_shadow::ShadowStyle::Solid)
        .with_dramatic_grade();

    render_pipeline(
        &source,
        &mut dest,
        10,
        5,
        0,
        0,
        CompositionOptions {
            shadow: Some(ShadowSpec::new(shadow_config)),
            t: 1.0,
            ..Default::default()
        },
        None,
    );

    // Shadow region: right edge at x=10..12, y=0..5
    // Check that destination glyphs are preserved in shadow region
    let shadow_cell = dest.get(11, 2).unwrap();
    assert_eq!(
        shadow_cell.ch, 'A',
        "Grade-underlying must preserve destination glyph, got '{}'",
        shadow_cell.ch
    );

    // Check a few more shadow cells
    let shadow_cell2 = dest.get(10, 3).unwrap();
    assert_eq!(
        shadow_cell2.ch, 'A',
        "Grade-underlying must preserve destination glyph at second position"
    );
}

#[test]
fn test_shadow_grade_underlying_preserves_destination_modifiers() {
    let source = create_source_grid(10, 5, 'X');
    let mut dest = create_grade_dest_grid(20, 20);

    let shadow_config = ShadowConfig::new(Color::BLACK.with_alpha(180))
        .with_offset(2, 1)
        .with_edges(ShadowEdges::BOTTOM_RIGHT)
        .with_style(tui_vfx_shadow::ShadowStyle::Solid)
        .with_dramatic_grade();

    render_pipeline(
        &source,
        &mut dest,
        10,
        5,
        0,
        0,
        CompositionOptions {
            shadow: Some(ShadowSpec::new(shadow_config)),
            t: 1.0,
            ..Default::default()
        },
        None,
    );

    let shadow_cell = dest.get(11, 2).unwrap();
    assert!(
        shadow_cell.mods.bold,
        "Grade-underlying must preserve destination modifiers (bold)"
    );
    assert_eq!(
        shadow_cell.mod_alpha,
        Some(200),
        "Grade-underlying must preserve destination mod_alpha"
    );
}

#[test]
fn test_shadow_grade_underlying_is_visibly_dramatic() {
    let source = create_source_grid(10, 5, 'X');
    let mut dest = create_grade_dest_grid(20, 20);

    let original_fg = Color::rgb(220, 180, 80);
    let original_bg = Color::rgb(90, 110, 140);
    let original_fg_luma = bt601_luma(original_fg);
    let original_bg_luma = bt601_luma(original_bg);

    let shadow_config = ShadowConfig::new(Color::BLACK.with_alpha(180))
        .with_offset(2, 1)
        .with_edges(ShadowEdges::BOTTOM_RIGHT)
        .with_style(tui_vfx_shadow::ShadowStyle::Solid)
        .with_dramatic_grade();

    render_pipeline(
        &source,
        &mut dest,
        10,
        5,
        0,
        0,
        CompositionOptions {
            shadow: Some(ShadowSpec::new(shadow_config)),
            t: 1.0,
            ..Default::default()
        },
        None,
    );

    let shadow_cell = dest.get(11, 2).unwrap();
    let graded_fg_luma = bt601_luma(shadow_cell.fg);
    let graded_bg_luma = bt601_luma(shadow_cell.bg);

    // Background luma reduction must be at least 20 points
    let bg_luma_drop = original_bg_luma - graded_bg_luma;
    assert!(
        bg_luma_drop >= 20.0,
        "BG luma must drop by at least 20 points, got {:.1} (from {:.1} to {:.1})",
        bg_luma_drop,
        original_bg_luma,
        graded_bg_luma,
    );

    // Foreground luma reduction must be at least 8 points
    let fg_luma_drop = original_fg_luma - graded_fg_luma;
    assert!(
        fg_luma_drop >= 8.0,
        "FG luma must drop by at least 8 points, got {:.1} (from {:.1} to {:.1})",
        fg_luma_drop,
        original_fg_luma,
        graded_fg_luma,
    );

    // Background luma reduction must exceed foreground by at least 5 points
    // (dramatic preset: bg_dim=0.58 vs fg_dim=0.28, but absolute drop also
    // depends on starting luma, so margin is ~6.4 with canonical sample colors)
    assert!(
        bg_luma_drop >= fg_luma_drop + 5.0,
        "BG luma drop ({:.1}) must exceed FG luma drop ({:.1}) by at least 5",
        bg_luma_drop,
        fg_luma_drop,
    );

    // Foreground saturation reduction must be non-zero
    let original_fg_sat = saturation_span(original_fg);
    let graded_fg_sat = saturation_span(shadow_cell.fg);
    assert!(
        graded_fg_sat < original_fg_sat,
        "FG saturation must decrease (was {}, now {})",
        original_fg_sat,
        graded_fg_sat,
    );

    // Background saturation must also decrease
    let original_bg_sat = saturation_span(original_bg);
    let graded_bg_sat = saturation_span(shadow_cell.bg);
    assert!(
        graded_bg_sat < original_bg_sat,
        "BG saturation must decrease (was {}, now {})",
        original_bg_sat,
        graded_bg_sat,
    );

    // Glyph and modifiers preserved (regression)
    assert_eq!(shadow_cell.ch, 'A');
    assert!(shadow_cell.mods.bold);
}

#[test]
fn test_shadow_grade_underlying_uses_stronger_bg_grading_than_fg() {
    let source = create_source_grid(10, 5, 'X');
    let mut dest = create_grade_dest_grid(20, 20);

    let original_bg = Color::rgb(90, 110, 140);
    let original_fg = Color::rgb(220, 180, 80);

    let shadow_config = ShadowConfig::new(Color::BLACK.with_alpha(180))
        .with_offset(2, 1)
        .with_edges(ShadowEdges::BOTTOM_RIGHT)
        .with_style(tui_vfx_shadow::ShadowStyle::Solid)
        .with_dramatic_grade();

    render_pipeline(
        &source,
        &mut dest,
        10,
        5,
        0,
        0,
        CompositionOptions {
            shadow: Some(ShadowSpec::new(shadow_config)),
            t: 1.0,
            ..Default::default()
        },
        None,
    );

    let shadow_cell = dest.get(11, 2).unwrap();

    // bg grading must be stronger than fg grading
    let bg_luma_drop = bt601_luma(original_bg) - bt601_luma(shadow_cell.bg);
    let fg_luma_drop = bt601_luma(original_fg) - bt601_luma(shadow_cell.fg);
    assert!(
        bg_luma_drop > fg_luma_drop,
        "BG luma drop ({:.1}) must exceed FG drop ({:.1})",
        bg_luma_drop,
        fg_luma_drop,
    );
}

// ============================================================================
// GRADE-UNDERLYING SHADOW TESTS (Phase 2 — animation and gradient coverage)
// ============================================================================

#[test]
fn test_shadow_grade_underlying_progress_controls_visibility() {
    let source = create_source_grid(10, 5, 'X');
    let original_bg = Color::rgb(90, 110, 140);
    let original_bg_luma = bt601_luma(original_bg);

    let shadow_config = ShadowConfig::new(Color::BLACK.with_alpha(180))
        .with_offset(2, 2)
        .with_edges(ShadowEdges::BOTTOM_RIGHT)
        .with_style(tui_vfx_shadow::ShadowStyle::Solid)
        .with_dramatic_grade();

    // At t=0.0, shadow should be invisible (progress controls shadow opacity)
    let mut dest_t0 = create_grade_dest_grid(20, 20);
    render_pipeline(
        &source,
        &mut dest_t0,
        10,
        5,
        0,
        0,
        CompositionOptions {
            shadow: Some(ShadowSpec::new(shadow_config.clone())),
            t: 0.0,
            ..Default::default()
        },
        None,
    );

    // At t=1.0, shadow should be fully visible with grading
    let mut dest_t1 = create_grade_dest_grid(20, 20);
    render_pipeline(
        &source,
        &mut dest_t1,
        10,
        5,
        0,
        0,
        CompositionOptions {
            shadow: Some(ShadowSpec::new(shadow_config)),
            t: 1.0,
            ..Default::default()
        },
        None,
    );

    // Shadow region cell at t=1.0 must show grading
    let cell_t1 = dest_t1.get(11, 2).unwrap();
    let bg_luma_t1 = bt601_luma(cell_t1.bg);
    assert!(
        original_bg_luma - bg_luma_t1 > 10.0,
        "At t=1.0, shadow cell BG must be visibly graded (luma drop {:.1})",
        original_bg_luma - bg_luma_t1,
    );

    // At t=0.0, the shadow cell should be closer to the original than at t=1.0
    // (either ungraded or much less graded)
    let cell_t0 = dest_t0.get(11, 2).unwrap();
    let bg_luma_t0 = bt601_luma(cell_t0.bg);
    let drop_t0 = original_bg_luma - bg_luma_t0;
    let drop_t1 = original_bg_luma - bg_luma_t1;
    assert!(
        drop_t1 > drop_t0 + 5.0,
        "Grading at t=1.0 ({:.1}) must be stronger than at t=0.0 ({:.1})",
        drop_t1,
        drop_t0,
    );
}

#[test]
fn test_shadow_grade_underlying_gradient_softens_penumbra() {
    let source = create_source_grid(10, 5, 'X');
    let original_bg = Color::rgb(90, 110, 140);
    let original_bg_luma = bt601_luma(original_bg);

    // Gradient shadow with multiple layers creates varying alpha across the shadow
    let shadow_config = ShadowConfig::new(Color::BLACK.with_alpha(200))
        .with_offset(3, 3)
        .with_edges(ShadowEdges::BOTTOM_RIGHT)
        .with_style(tui_vfx_shadow::ShadowStyle::Gradient { layers: 3 })
        .with_dramatic_grade();

    let mut dest = create_grade_dest_grid(20, 20);
    render_pipeline(
        &source,
        &mut dest,
        10,
        5,
        0,
        0,
        CompositionOptions {
            shadow: Some(ShadowSpec::new(shadow_config)),
            t: 1.0,
            ..Default::default()
        },
        None,
    );

    // Sample cells at different distances from the element edge
    // Closer to element = denser shadow = stronger grading
    // Farther from element = softer shadow = weaker grading
    // With offset (3,3) and gradient layers=3, the rightmost column should have
    // weaker grading than the column closest to the element
    let inner_cell = dest.get(10, 3).unwrap(); // closest shadow column to element
    let outer_cell = dest.get(12, 3).unwrap(); // farther shadow column

    let inner_drop = original_bg_luma - bt601_luma(inner_cell.bg);
    let outer_drop = original_bg_luma - bt601_luma(outer_cell.bg);

    // Inner shadow should have stronger grading (or at least not weaker) than outer
    // This validates that gradient alpha correctly scales grade intensity
    assert!(
        inner_drop >= outer_drop,
        "Inner shadow grading ({:.1}) must be >= outer ({:.1}) — gradient should soften penumbra",
        inner_drop,
        outer_drop,
    );

    // Both should show some grading effect (non-zero drop)
    assert!(
        inner_drop > 1.0,
        "Inner shadow must show some grading (drop={:.1})",
        inner_drop,
    );
}

// ============================================================================
// COLOR-INERT GLYPH REPLACEMENT TESTS (Pipeline integration)
// ============================================================================

/// Create a dest grid filled with emoji for color-inert replacement tests.
fn create_emoji_dest_grid(width: usize, height: usize) -> OwnedGrid {
    let mut grid = OwnedGrid::new(width, height);
    for y in 0..height {
        for x in 0..width {
            let cell = Cell {
                ch: '🚀',
                fg: Color::rgb(220, 180, 80),
                bg: Color::rgb(90, 110, 140),
                ..Default::default()
            };
            grid.set(x, y, cell);
        }
    }
    grid
}

#[test]
fn test_shadow_grade_underlying_replaces_emoji_with_dramatic_grade() {
    let source = create_source_grid(10, 5, 'X');
    let mut dest = create_emoji_dest_grid(20, 20);

    let shadow_config = ShadowConfig::new(Color::BLACK.with_alpha(180))
        .with_offset(2, 1)
        .with_edges(ShadowEdges::BOTTOM_RIGHT)
        .with_style(tui_vfx_shadow::ShadowStyle::Solid)
        .with_dramatic_grade(); // dramatic() sets replacement_char = Some('·')

    render_pipeline(
        &source,
        &mut dest,
        10,
        5,
        0,
        0,
        CompositionOptions {
            shadow: Some(ShadowSpec::new(shadow_config)),
            t: 1.0,
            ..Default::default()
        },
        None,
    );

    // Shadow region cell should have emoji replaced with middle dot
    let shadow_cell = dest.get(11, 2).unwrap();
    assert_eq!(
        shadow_cell.ch, '\u{00B7}',
        "Emoji in shadow region should be replaced with middle dot, got '{}'",
        shadow_cell.ch,
    );

    // Element region should still have 'X' (source content overwrites)
    assert_eq!(dest.get(0, 0).unwrap().ch, 'X');
}

#[test]
fn test_shadow_grade_underlying_preserves_emoji_when_replacement_none() {
    let source = create_source_grid(10, 5, 'X');
    let mut dest = create_emoji_dest_grid(20, 20);

    use tui_vfx_shadow::{ShadowCompositeMode, ShadowGradeConfig};
    let grade = ShadowGradeConfig {
        replacement_char: None, // no replacement
        ..ShadowGradeConfig::dramatic()
    };
    let shadow_config = ShadowConfig::new(Color::BLACK.with_alpha(180))
        .with_offset(2, 1)
        .with_edges(ShadowEdges::BOTTOM_RIGHT)
        .with_style(tui_vfx_shadow::ShadowStyle::Solid)
        .with_composite_mode(ShadowCompositeMode::GradeUnderlying)
        .with_grade(grade);

    render_pipeline(
        &source,
        &mut dest,
        10,
        5,
        0,
        0,
        CompositionOptions {
            shadow: Some(ShadowSpec::new(shadow_config)),
            t: 1.0,
            ..Default::default()
        },
        None,
    );

    // Shadow region cell should still have emoji (no replacement configured)
    let shadow_cell = dest.get(11, 2).unwrap();
    assert_eq!(
        shadow_cell.ch, '🚀',
        "Emoji should be preserved when replacement_char is None, got '{}'",
        shadow_cell.ch,
    );
}

// <FILE>crates/tui-vfx-compositor/tests/pipeline/test_orc_render_pipeline.rs</FILE> - <DESC>L2 render pipeline tests with Grid trait</DESC>
// <VERS>END OF VERSION: 5.4.0</VERS>
