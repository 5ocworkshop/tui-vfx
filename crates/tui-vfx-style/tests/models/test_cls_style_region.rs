// <FILE>tui-vfx-style/tests/models/test_cls_style_region.rs</FILE> - <DESC>Tests for StyleRegion</DESC>
// <VERS>VERSION: 3.0.0 - 2025-12-31</VERS>
// <WCTX>Modulo targeting for StyleRegion</WCTX>
// <CLOG>Added tests for Modulo variant with axis, modulus, remainder</CLOG>

use tui_vfx_style::models::StyleRegion;

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

// <FILE>tui-vfx-style/tests/models/test_cls_style_region.rs</FILE> - <DESC>Tests for StyleRegion</DESC>
// <VERS>END OF VERSION: 3.0.0 - 2025-12-31</VERS>
