// <FILE>crates/tui-vfx-compositor/tests/pipeline/test_fnc_grade_shadow_cell.rs</FILE> - <DESC>Unit tests for color-inert glyph replacement in grade_shadow_cell</DESC>
// <VERS>VERSION: 0.1.0</VERS>
// <WCTX>Color-inert glyph detection for shadow grading replacement</WCTX>
// <CLOG>Initial creation with replacement and preservation test cases</CLOG>

use tui_vfx_compositor::pipeline::fnc_grade_shadow_cell::grade_shadow_cell;
use tui_vfx_shadow::ShadowGradeConfig;
use tui_vfx_types::{Cell, Color};

/// Create a shadow cell with the given alpha (simulates shadow coverage).
fn shadow_cell_with_alpha(alpha: u8) -> Cell {
    Cell {
        ch: ' ',
        fg: Color::BLACK.with_alpha(alpha),
        bg: Color::BLACK.with_alpha(alpha),
        ..Default::default()
    }
}

/// Create a dest cell with the given character and colors.
fn dest_cell(ch: char) -> Cell {
    Cell {
        ch,
        fg: Color::rgb(220, 180, 80),
        bg: Color::rgb(90, 110, 140),
        ..Default::default()
    }
}

// ============================================================================
// COLOR-INERT GLYPH REPLACEMENT TESTS
// ============================================================================

#[test]
fn emoji_replaced_when_replacement_char_set() {
    let shadow = shadow_cell_with_alpha(200);
    let dest = dest_cell('🚀'); // emoji is color-inert
    let grade = ShadowGradeConfig::dramatic(); // has replacement_char = Some('·')

    let result = grade_shadow_cell(&shadow, &dest, Color::BLACK, &grade);

    assert_eq!(
        result.ch, '\u{00B7}',
        "Emoji should be replaced with middle dot, got '{}'",
        result.ch,
    );
}

#[test]
fn pua_replaced_when_replacement_char_set() {
    let shadow = shadow_cell_with_alpha(200);
    let dest = dest_cell('\u{F121}'); // nerd font icon (PUA)
    let grade = ShadowGradeConfig::dramatic();

    let result = grade_shadow_cell(&shadow, &dest, Color::BLACK, &grade);

    assert_eq!(
        result.ch, '\u{00B7}',
        "PUA glyph should be replaced with middle dot, got '{}'",
        result.ch,
    );
}

#[test]
fn normal_glyph_preserved_with_replacement_char_set() {
    let shadow = shadow_cell_with_alpha(200);
    let dest = dest_cell('A'); // normal ASCII - not color-inert
    let grade = ShadowGradeConfig::dramatic();

    let result = grade_shadow_cell(&shadow, &dest, Color::BLACK, &grade);

    assert_eq!(
        result.ch, 'A',
        "Normal glyph should be preserved, got '{}'",
        result.ch,
    );
}

#[test]
fn emoji_preserved_when_replacement_char_none() {
    let shadow = shadow_cell_with_alpha(200);
    let dest = dest_cell('🚀');
    let grade = ShadowGradeConfig {
        fg_dim_strength: 0.28,
        bg_dim_strength: 0.58,
        replacement_char: None, // no replacement
        ..Default::default()
    };

    let result = grade_shadow_cell(&shadow, &dest, Color::BLACK, &grade);

    assert_eq!(
        result.ch, '🚀',
        "Emoji should be preserved when replacement_char is None, got '{}'",
        result.ch,
    );
}

#[test]
fn no_replacement_at_zero_coverage() {
    let shadow = shadow_cell_with_alpha(0); // zero coverage -> early return
    let dest = dest_cell('🚀');
    let grade = ShadowGradeConfig::dramatic();

    let result = grade_shadow_cell(&shadow, &dest, Color::BLACK, &grade);

    // Zero coverage returns dest_cell unchanged (early return path)
    assert_eq!(
        result.ch, '🚀',
        "At zero coverage, dest cell should be returned unchanged, got '{}'",
        result.ch,
    );
}

#[test]
fn box_drawing_preserved_with_replacement_char_set() {
    let shadow = shadow_cell_with_alpha(200);
    let dest = dest_cell('─'); // box drawing - not color-inert
    let grade = ShadowGradeConfig::dramatic();

    let result = grade_shadow_cell(&shadow, &dest, Color::BLACK, &grade);

    assert_eq!(
        result.ch, '─',
        "Box drawing should be preserved, got '{}'",
        result.ch,
    );
}

#[test]
fn grading_still_applied_to_colors_when_glyph_replaced() {
    let shadow = shadow_cell_with_alpha(200);
    let dest = dest_cell('🚀');
    let grade = ShadowGradeConfig::dramatic();
    let original_bg = dest.bg;

    let result = grade_shadow_cell(&shadow, &dest, Color::BLACK, &grade);

    // Glyph should be replaced
    assert_eq!(result.ch, '\u{00B7}');

    // But colors should still be graded (bg should be dimmer)
    let original_luma = 0.299 * original_bg.r as f32 + 0.587 * original_bg.g as f32 + 0.114 * original_bg.b as f32;
    let graded_luma = 0.299 * result.bg.r as f32 + 0.587 * result.bg.g as f32 + 0.114 * result.bg.b as f32;
    assert!(
        graded_luma < original_luma,
        "BG should be dimmed even when glyph is replaced (original luma={:.1}, graded={:.1})",
        original_luma,
        graded_luma,
    );
}

// <FILE>crates/tui-vfx-compositor/tests/pipeline/test_fnc_grade_shadow_cell.rs</FILE> - <DESC>Unit tests for color-inert glyph replacement in grade_shadow_cell</DESC>
// <VERS>END OF VERSION: 0.1.0</VERS>
