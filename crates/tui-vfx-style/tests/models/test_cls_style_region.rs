// <FILE>tui-vfx-style/tests/models/test_cls_style_region.rs</FILE> - <DESC>Tests for StyleRegion</DESC>
// <VERS>VERSION: 3.1.0</VERS>
// <WCTX>Phase 0 P0.2 — cell-coordinate BindableU16 bindings</WCTX>
// <CLOG>Add Cell-binding tests covering resolved() borrow/owned paths, should_style, bounding_rect, and safe-fail on unresolved bindings</CLOG>

use std::borrow::Cow;
use tui_vfx_style::models::{BindableU16, StyleRegion};
use tui_vfx_style::traits::ShaderRuntimeParams;

#[test]
fn test_all_region_styles_everything() {
    let region = StyleRegion::All;
    // All cells should be styled
    assert!(region.should_style(0, 0, 10, 5));
    assert!(region.should_style(5, 2, 10, 5));
    assert!(region.should_style(9, 4, 10, 5));
}

#[test]
fn test_border_only_styles_edges() {
    let region = StyleRegion::BorderOnly;
    let (w, h) = (10, 5);

    // Top edge
    assert!(region.should_style(0, 0, w, h));
    assert!(region.should_style(5, 0, w, h));
    assert!(region.should_style(9, 0, w, h));

    // Bottom edge
    assert!(region.should_style(0, 4, w, h));
    assert!(region.should_style(5, 4, w, h));
    assert!(region.should_style(9, 4, w, h));

    // Left edge
    assert!(region.should_style(0, 2, w, h));

    // Right edge
    assert!(region.should_style(9, 2, w, h));

    // Interior should NOT be styled
    assert!(!region.should_style(1, 1, w, h));
    assert!(!region.should_style(5, 2, w, h));
    assert!(!region.should_style(8, 3, w, h));
}

#[test]
fn test_text_only_styles_interior() {
    let region = StyleRegion::TextOnly;
    let (w, h) = (10, 5);

    // Interior cells should be styled
    assert!(region.should_style(1, 1, w, h));
    assert!(region.should_style(5, 2, w, h));
    assert!(region.should_style(8, 3, w, h));

    // Border cells should NOT be styled
    assert!(!region.should_style(0, 0, w, h));
    assert!(!region.should_style(9, 0, w, h));
    assert!(!region.should_style(0, 4, w, h));
    assert!(!region.should_style(9, 4, w, h));
    assert!(!region.should_style(0, 2, w, h));
    assert!(!region.should_style(9, 2, w, h));
}

#[test]
fn test_small_widget_edge_cases() {
    // 1x1 widget - only border
    let region = StyleRegion::BorderOnly;
    assert!(region.should_style(0, 0, 1, 1));

    let region = StyleRegion::TextOnly;
    assert!(!region.should_style(0, 0, 1, 1));

    // 2x2 widget - all border, no interior
    let region = StyleRegion::BorderOnly;
    assert!(region.should_style(0, 0, 2, 2));
    assert!(region.should_style(1, 0, 2, 2));
    assert!(region.should_style(0, 1, 2, 2));
    assert!(region.should_style(1, 1, 2, 2));

    let region = StyleRegion::TextOnly;
    assert!(!region.should_style(0, 0, 2, 2));
    assert!(!region.should_style(1, 1, 2, 2));

    // 3x3 widget - has 1 interior cell
    let region = StyleRegion::TextOnly;
    assert!(region.should_style(1, 1, 3, 3));
    assert!(!region.should_style(0, 0, 3, 3));
}

#[test]
fn test_serde_roundtrip() {
    let regions = [
        StyleRegion::All,
        StyleRegion::TextOnly,
        StyleRegion::BorderOnly,
        StyleRegion::BackgroundOnly,
    ];

    for region in regions {
        let json = serde_json::to_string(&region).unwrap();
        let parsed: StyleRegion = serde_json::from_str(&json).unwrap();
        assert_eq!(region, parsed);
    }
}

#[test]
fn test_default_is_all() {
    assert_eq!(StyleRegion::default(), StyleRegion::All);
}

// ============================================================================
// Row Targeting Tests (v2.0.0)
// ============================================================================

#[test]
fn test_rows_matches_specified_rows() {
    let region = StyleRegion::Rows(vec![0, 2, 4]);
    let (w, h) = (10, 5);

    // Specified rows should match (any x value)
    assert!(region.should_style(0, 0, w, h));
    assert!(region.should_style(5, 0, w, h));
    assert!(region.should_style(9, 0, w, h));
    assert!(region.should_style(0, 2, w, h));
    assert!(region.should_style(5, 2, w, h));
    assert!(region.should_style(0, 4, w, h));

    // Non-specified rows should NOT match
    assert!(!region.should_style(0, 1, w, h));
    assert!(!region.should_style(5, 1, w, h));
    assert!(!region.should_style(0, 3, w, h));
    assert!(!region.should_style(9, 3, w, h));
}

#[test]
fn test_rows_single_row() {
    // Single row targeting (e.g., for progress indicator)
    let region = StyleRegion::Rows(vec![0]);
    let (w, h) = (20, 3);

    // Only row 0 matches
    assert!(region.should_style(0, 0, w, h));
    assert!(region.should_style(10, 0, w, h));
    assert!(region.should_style(19, 0, w, h));

    // Other rows don't match
    assert!(!region.should_style(0, 1, w, h));
    assert!(!region.should_style(0, 2, w, h));
}

#[test]
fn test_rows_empty_matches_nothing() {
    let region = StyleRegion::Rows(vec![]);
    let (w, h) = (10, 5);

    // Empty row list matches nothing
    assert!(!region.should_style(0, 0, w, h));
    assert!(!region.should_style(5, 2, w, h));
    assert!(!region.should_style(9, 4, w, h));
}

#[test]
fn test_rows_out_of_bounds_safe() {
    // Rows beyond widget height - should be safe (just won't match real cells)
    let region = StyleRegion::Rows(vec![99, 100]);
    let (w, h) = (10, 5);

    // In-bounds rows don't match (not in list)
    assert!(!region.should_style(0, 0, w, h));
    assert!(!region.should_style(0, 4, w, h));

    // Out-of-bounds row in list - won't panic, just matches if y equals it
    // (though such cells won't exist in practice)
    assert!(region.should_style(0, 99, w, h));
}

#[test]
fn test_row_range_matches_range() {
    let region = StyleRegion::RowRange { start: 1, end: 4 };
    let (w, h) = (10, 5);

    // Rows in [1, 4) should match
    assert!(region.should_style(0, 1, w, h));
    assert!(region.should_style(5, 2, w, h));
    assert!(region.should_style(9, 3, w, h));

    // Rows outside range should NOT match
    assert!(!region.should_style(0, 0, w, h)); // Before start
    assert!(!region.should_style(0, 4, w, h)); // At end (exclusive)
    assert!(!region.should_style(0, 5, w, h)); // After end
}

#[test]
fn test_row_range_single_row() {
    // Range of exactly one row
    let region = StyleRegion::RowRange { start: 2, end: 3 };
    let (w, h) = (10, 5);

    assert!(!region.should_style(0, 1, w, h));
    assert!(region.should_style(0, 2, w, h));
    assert!(!region.should_style(0, 3, w, h));
}

#[test]
fn test_row_range_full_widget() {
    // Range covering entire widget
    let region = StyleRegion::RowRange { start: 0, end: 5 };
    let (w, h) = (10, 5);

    assert!(region.should_style(0, 0, w, h));
    assert!(region.should_style(5, 2, w, h));
    assert!(region.should_style(9, 4, w, h));
}

#[test]
fn test_row_range_inverted_matches_nothing() {
    // Inverted range (start >= end) should match nothing
    let region = StyleRegion::RowRange { start: 5, end: 2 };
    let (w, h) = (10, 5);

    assert!(!region.should_style(0, 0, w, h));
    assert!(!region.should_style(0, 2, w, h));
    assert!(!region.should_style(0, 4, w, h));
    assert!(!region.should_style(0, 5, w, h));
}

#[test]
fn test_row_range_empty_matches_nothing() {
    // Empty range (start == end)
    let region = StyleRegion::RowRange { start: 2, end: 2 };
    let (w, h) = (10, 5);

    assert!(!region.should_style(0, 1, w, h));
    assert!(!region.should_style(0, 2, w, h));
    assert!(!region.should_style(0, 3, w, h));
}

#[test]
fn test_serde_rows_roundtrip() {
    let region = StyleRegion::Rows(vec![0, 2, 4]);
    let json = serde_json::to_string(&region).unwrap();
    let parsed: StyleRegion = serde_json::from_str(&json).unwrap();
    assert_eq!(region, parsed);

    // Verify JSON structure
    assert!(json.contains("Rows"));
    assert!(json.contains("[0,2,4]"));
}

#[test]
fn test_serde_row_range_roundtrip() {
    let region = StyleRegion::RowRange { start: 1, end: 5 };
    let json = serde_json::to_string(&region).unwrap();
    let parsed: StyleRegion = serde_json::from_str(&json).unwrap();
    assert_eq!(region, parsed);

    // Verify JSON structure
    assert!(json.contains("RowRange"));
    assert!(json.contains("start"));
    assert!(json.contains("end"));
}

#[test]
fn test_serde_existing_variants_unchanged() {
    // Verify existing simple variants still serialize to simple strings
    let all_json = serde_json::to_string(&StyleRegion::All).unwrap();
    assert_eq!(all_json, "\"All\"");

    let text_json = serde_json::to_string(&StyleRegion::TextOnly).unwrap();
    assert_eq!(text_json, "\"TextOnly\"");

    let border_json = serde_json::to_string(&StyleRegion::BorderOnly).unwrap();
    assert_eq!(border_json, "\"BorderOnly\"");

    let bg_json = serde_json::to_string(&StyleRegion::BackgroundOnly).unwrap();
    assert_eq!(bg_json, "\"BackgroundOnly\"");
}

// ============================================================================
// Modulo Targeting Tests (v3.0.0)
// ============================================================================

use tui_vfx_style::models::ModuloAxis;

#[test]
fn test_modulo_horizontal_every_other_row() {
    // Every other row starting from 0 (rows 0, 2, 4, ...)
    let region = StyleRegion::Modulo {
        axis: ModuloAxis::Horizontal,
        modulus: 2,
        remainder: 0,
    };
    let (w, h) = (10, 6);

    // Even rows match (any x)
    assert!(region.should_style(0, 0, w, h));
    assert!(region.should_style(5, 0, w, h));
    assert!(region.should_style(0, 2, w, h));
    assert!(region.should_style(9, 4, w, h));

    // Odd rows don't match
    assert!(!region.should_style(0, 1, w, h));
    assert!(!region.should_style(5, 3, w, h));
    assert!(!region.should_style(9, 5, w, h));
}

#[test]
fn test_modulo_horizontal_odd_rows() {
    // Every other row starting from 1 (rows 1, 3, 5, ...)
    let region = StyleRegion::Modulo {
        axis: ModuloAxis::Horizontal,
        modulus: 2,
        remainder: 1,
    };
    let (w, h) = (10, 6);

    // Odd rows match
    assert!(region.should_style(0, 1, w, h));
    assert!(region.should_style(5, 3, w, h));
    assert!(region.should_style(9, 5, w, h));

    // Even rows don't match
    assert!(!region.should_style(0, 0, w, h));
    assert!(!region.should_style(5, 2, w, h));
    assert!(!region.should_style(9, 4, w, h));
}

#[test]
fn test_modulo_vertical_every_other_column() {
    // Every other column starting from 0 (columns 0, 2, 4, ...)
    let region = StyleRegion::Modulo {
        axis: ModuloAxis::Vertical,
        modulus: 2,
        remainder: 0,
    };
    let (w, h) = (6, 10);

    // Even columns match (any y)
    assert!(region.should_style(0, 0, w, h));
    assert!(region.should_style(0, 5, w, h));
    assert!(region.should_style(2, 0, w, h));
    assert!(region.should_style(4, 9, w, h));

    // Odd columns don't match
    assert!(!region.should_style(1, 0, w, h));
    assert!(!region.should_style(3, 5, w, h));
    assert!(!region.should_style(5, 9, w, h));
}

#[test]
fn test_modulo_every_third_row() {
    // Every third row (rows 0, 3, 6, ...)
    let region = StyleRegion::Modulo {
        axis: ModuloAxis::Horizontal,
        modulus: 3,
        remainder: 0,
    };
    let (w, h) = (10, 10);

    assert!(region.should_style(0, 0, w, h));
    assert!(region.should_style(0, 3, w, h));
    assert!(region.should_style(0, 6, w, h));
    assert!(region.should_style(0, 9, w, h));

    assert!(!region.should_style(0, 1, w, h));
    assert!(!region.should_style(0, 2, w, h));
    assert!(!region.should_style(0, 4, w, h));
    assert!(!region.should_style(0, 5, w, h));
}

#[test]
fn test_modulo_with_offset_remainder() {
    // Every third row, but offset by 1 (rows 1, 4, 7, ...)
    let region = StyleRegion::Modulo {
        axis: ModuloAxis::Horizontal,
        modulus: 3,
        remainder: 1,
    };
    let (w, h) = (10, 10);

    assert!(region.should_style(0, 1, w, h));
    assert!(region.should_style(0, 4, w, h));
    assert!(region.should_style(0, 7, w, h));

    assert!(!region.should_style(0, 0, w, h));
    assert!(!region.should_style(0, 2, w, h));
    assert!(!region.should_style(0, 3, w, h));
}

#[test]
fn test_modulo_one_matches_everything() {
    // Modulo 1 with remainder 0 matches everything
    let region = StyleRegion::Modulo {
        axis: ModuloAxis::Horizontal,
        modulus: 1,
        remainder: 0,
    };
    let (w, h) = (10, 5);

    assert!(region.should_style(0, 0, w, h));
    assert!(region.should_style(0, 1, w, h));
    assert!(region.should_style(0, 2, w, h));
    assert!(region.should_style(5, 3, w, h));
    assert!(region.should_style(9, 4, w, h));
}

#[test]
fn test_modulo_zero_matches_nothing() {
    // Modulo 0 is invalid - should match nothing (safe fallback)
    let region = StyleRegion::Modulo {
        axis: ModuloAxis::Horizontal,
        modulus: 0,
        remainder: 0,
    };
    let (w, h) = (10, 5);

    assert!(!region.should_style(0, 0, w, h));
    assert!(!region.should_style(5, 2, w, h));
    assert!(!region.should_style(9, 4, w, h));
}

#[test]
fn test_modulo_remainder_exceeds_modulus() {
    // Remainder >= modulus should match nothing
    let region = StyleRegion::Modulo {
        axis: ModuloAxis::Horizontal,
        modulus: 2,
        remainder: 5, // impossible: no number % 2 == 5
    };
    let (w, h) = (10, 10);

    assert!(!region.should_style(0, 0, w, h));
    assert!(!region.should_style(0, 1, w, h));
    assert!(!region.should_style(0, 5, w, h));
}

#[test]
fn test_serde_modulo_roundtrip() {
    let region = StyleRegion::Modulo {
        axis: ModuloAxis::Horizontal,
        modulus: 2,
        remainder: 0,
    };
    let json = serde_json::to_string(&region).unwrap();
    let parsed: StyleRegion = serde_json::from_str(&json).unwrap();
    assert_eq!(region, parsed);

    // Verify JSON structure
    assert!(json.contains("Modulo"));
    assert!(json.contains("axis"));
    assert!(json.contains("modulus"));
    assert!(json.contains("remainder"));
}

#[test]
fn test_serde_modulo_axis_roundtrip() {
    let horizontal = ModuloAxis::Horizontal;
    let vertical = ModuloAxis::Vertical;

    let h_json = serde_json::to_string(&horizontal).unwrap();
    let v_json = serde_json::to_string(&vertical).unwrap();

    assert_eq!(
        serde_json::from_str::<ModuloAxis>(&h_json).unwrap(),
        horizontal
    );
    assert_eq!(
        serde_json::from_str::<ModuloAxis>(&v_json).unwrap(),
        vertical
    );
}

// --- Phase 0 P0.2: Cell coordinate binding tests -----------------------

fn cell(x: u16, y: u16) -> StyleRegion {
    StyleRegion::Cell {
        x: BindableU16::Literal(x),
        y: BindableU16::Literal(y),
    }
}

fn cell_bound(x: BindableU16, y: BindableU16) -> StyleRegion {
    StyleRegion::Cell { x, y }
}

#[test]
fn cell_with_literal_coords_styles_exact_cell() {
    let region = cell(5, 3);
    let w = 10;
    let h = 6;
    assert!(region.should_style(5, 3, w, h));
    assert!(!region.should_style(5, 2, w, h));
    assert!(!region.should_style(4, 3, w, h));
    assert!(!region.should_style(0, 0, w, h));
}

#[test]
fn cell_with_literal_bounding_rect_is_1x1_at_coord() {
    let region = cell(7, 2);
    assert_eq!(region.bounding_rect(), Some((7, 2, 1, 1)));
}

#[test]
fn cell_resolved_with_literal_coords_borrows() {
    // A Cell whose coordinates are already literals should avoid the clone
    // and return a Cow::Borrowed.
    let region = cell(5, 3);
    let rp = ShaderRuntimeParams::new();
    let resolved = region.resolved(&rp);
    assert!(matches!(resolved, Cow::Borrowed(_)));
}

#[test]
fn cell_resolved_with_binding_lowers_to_literal() {
    let region = cell_bound(
        BindableU16::Binding("hovered_button_x".to_string()),
        BindableU16::Literal(5),
    );
    let mut rp = ShaderRuntimeParams::new();
    rp.insert("hovered_button_x", 37_u16);

    let resolved = region.resolved(&rp);
    assert!(matches!(resolved, Cow::Owned(_)));

    match resolved.as_ref() {
        StyleRegion::Cell {
            x: BindableU16::Literal(xl),
            y: BindableU16::Literal(yl),
        } => {
            assert_eq!(*xl, 37);
            assert_eq!(*yl, 5);
        }
        other => panic!(
            "expected resolved Cell with literal coords, got {:?}",
            other
        ),
    }
}

#[test]
fn cell_resolved_post_resolution_styles_the_runtime_coordinate() {
    let region = cell_bound(
        BindableU16::Binding("hover_x".to_string()),
        BindableU16::Literal(2),
    );
    let mut rp = ShaderRuntimeParams::new();
    rp.insert("hover_x", 20_u16);

    let resolved = region.resolved(&rp);
    assert!(resolved.should_style(20, 2, 40, 10));
    assert!(!resolved.should_style(19, 2, 40, 10));
}

#[test]
fn cell_resolved_missing_binding_falls_back_to_zero() {
    // Missing runtime param: binding lowers to Literal(0), so the region
    // points at the top-left cell. Safe default for "not yet hovered."
    let region = cell_bound(
        BindableU16::Binding("missing".to_string()),
        BindableU16::Binding("also_missing".to_string()),
    );
    let rp = ShaderRuntimeParams::new();

    let resolved = region.resolved(&rp);
    assert!(resolved.should_style(0, 0, 40, 10));
    assert!(!resolved.should_style(5, 5, 40, 10));
}

#[test]
fn cell_unresolved_binding_should_style_is_safe_noop() {
    // Calling should_style directly on a Cell with Binding coords (bypassing
    // resolved()) must not panic and must not match anywhere.
    let region = cell_bound(
        BindableU16::Binding("x".to_string()),
        BindableU16::Literal(3),
    );
    assert!(!region.should_style(0, 3, 10, 10));
    assert!(!region.should_style(20, 3, 30, 10));
}

#[test]
fn cell_unresolved_binding_bounding_rect_is_none() {
    // A Cell with an unresolved Binding cannot report its bounding rect.
    let region = cell_bound(
        BindableU16::Literal(5),
        BindableU16::Binding("y".to_string()),
    );
    assert_eq!(region.bounding_rect(), None);
}

#[test]
fn cell_binding_json_deserializes_from_mixed_literal_and_binding() {
    // StyleRegion is an externally-tagged enum (PascalCase), so a Cell
    // shows up as {"Cell": {"x": ..., "y": ...}}. Recipe fixtures can mix
    // forms: x is a raw integer, y is a tagged binding. Both get absorbed
    // by BindableU16's lenient deserialize.
    let json = r#"{
        "Cell": {
            "x": 20,
            "y": { "binding": "button_row" }
        }
    }"#;
    let parsed: StyleRegion = serde_json::from_str(json).unwrap();
    match parsed {
        StyleRegion::Cell { x, y } => {
            assert_eq!(x, BindableU16::Literal(20));
            assert_eq!(y, BindableU16::Binding("button_row".to_string()));
        }
        other => panic!("expected StyleRegion::Cell, got {:?}", other),
    }
}

#[test]
fn cell_binding_serde_roundtrip_normalizes_to_tagged_form() {
    let region = cell_bound(
        BindableU16::Literal(20),
        BindableU16::Binding("button_row".to_string()),
    );
    let json = serde_json::to_string(&region).unwrap();
    let parsed: StyleRegion = serde_json::from_str(&json).unwrap();
    assert_eq!(parsed, region);
}

// <FILE>tui-vfx-style/tests/models/test_cls_style_region.rs</FILE> - <DESC>Tests for StyleRegion</DESC>
// <VERS>END OF VERSION: 3.1.0</VERS>
