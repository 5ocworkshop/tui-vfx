// <FILE>tui-vfx-compositor/tests/pipeline/test_multiple_effects.rs</FILE> - <DESC>Tests for multiple masks, filters, shaders</DESC>
// <VERS>VERSION: 1.2.0</VERS>
// <WCTX>Ergonomic reveal/hide schema for intuitive mask direction semantics</WCTX>
// <CLOG>Update tests for new reveal/hide/direction optional fields</CLOG>

use smallvec::{SmallVec, smallvec};
use tui_vfx_compositor::types::{FilterSpec, MaskCombineMode, MaskSpec, WipeDirection};

// =============================================================================
// MaskCombineMode Tests
// =============================================================================

#[test]
fn test_mask_combine_mode_default_is_all() {
    assert_eq!(MaskCombineMode::default(), MaskCombineMode::All);
}

#[test]
fn test_mask_combine_mode_is_default() {
    assert!(MaskCombineMode::All.is_default());
    assert!(!MaskCombineMode::Any.is_default());
}

#[test]
fn test_mask_combine_mode_serde_all() {
    let mode = MaskCombineMode::All;
    let json = serde_json::to_string(&mode).unwrap();
    assert_eq!(json, r#""all""#);

    let parsed: MaskCombineMode = serde_json::from_str(&json).unwrap();
    assert_eq!(parsed, MaskCombineMode::All);
}

#[test]
fn test_mask_combine_mode_serde_any() {
    let mode = MaskCombineMode::Any;
    let json = serde_json::to_string(&mode).unwrap();
    assert_eq!(json, r#""any""#);

    let parsed: MaskCombineMode = serde_json::from_str(&json).unwrap();
    assert_eq!(parsed, MaskCombineMode::Any);
}

// =============================================================================
// SmallVec Allocation Tests
// =============================================================================

#[test]
fn test_smallvec_mask_no_heap_for_two() {
    let masks: SmallVec<[MaskSpec; 2]> = smallvec![
        MaskSpec::Wipe {
            reveal: Some(WipeDirection::LeftToRight),
            hide: None,
            direction: None,
            soft_edge: false,
        },
        MaskSpec::Dissolve {
            seed: 42,
            chunk_size: 2,
        },
    ];

    // SmallVec should not spill to heap for ≤2 elements
    assert!(
        !masks.spilled(),
        "SmallVec<[MaskSpec; 2]> should not heap-allocate for 2 masks"
    );
}

#[test]
fn test_smallvec_filter_no_heap_for_three() {
    use mixed_signals::types::SignalOrFloat;

    let filters: SmallVec<[FilterSpec; 3]> = smallvec![
        FilterSpec::Dim {
            factor: SignalOrFloat::Static(0.5),
            apply_to: tui_vfx_compositor::types::ApplyTo::Both,
        },
        FilterSpec::Invert {
            apply_to: tui_vfx_compositor::types::ApplyTo::Foreground,
        },
        FilterSpec::Crt {
            scanline_strength: SignalOrFloat::Static(0.3),
            glow: SignalOrFloat::Static(0.1),
        },
    ];

    // SmallVec should not spill to heap for ≤3 elements
    assert!(
        !filters.spilled(),
        "SmallVec<[FilterSpec; 3]> should not heap-allocate for 3 filters"
    );
}

#[test]
fn test_smallvec_mask_spills_for_three() {
    let masks: SmallVec<[MaskSpec; 2]> = smallvec![
        MaskSpec::Wipe {
            reveal: Some(WipeDirection::LeftToRight),
            hide: None,
            direction: None,
            soft_edge: false,
        },
        MaskSpec::Dissolve {
            seed: 42,
            chunk_size: 2,
        },
        MaskSpec::Checkers { cell_size: 4 },
    ];

    // SmallVec should spill to heap for >2 elements
    assert!(
        masks.spilled(),
        "SmallVec<[MaskSpec; 2]> should heap-allocate for 3 masks"
    );
}

// =============================================================================
// check_masks Logic Tests
// =============================================================================

use tui_vfx_compositor::pipeline::check_masks;

#[test]
fn test_empty_mask_list_returns_visible() {
    // Empty mask list should return true (fully visible)
    let masks: SmallVec<[MaskSpec; 2]> = SmallVec::new();

    // Empty masks = always visible, regardless of combine mode
    assert!(check_masks(0, 0, 80, 24, 0.5, &masks, MaskCombineMode::All));
    assert!(check_masks(0, 0, 80, 24, 0.5, &masks, MaskCombineMode::Any));
}

#[test]
fn test_single_mask_same_for_both_modes() {
    // Single mask in list should behave identically for All and Any modes
    let masks: SmallVec<[MaskSpec; 2]> = smallvec![MaskSpec::Wipe {
        reveal: Some(WipeDirection::LeftToRight),
        hide: None,
        direction: None,
        soft_edge: false,
    }];

    // At t=0.5, left half should be visible (x < 40)
    // Both modes should agree for single mask
    let all_result = check_masks(20, 10, 80, 24, 0.5, &masks, MaskCombineMode::All);
    let any_result = check_masks(20, 10, 80, 24, 0.5, &masks, MaskCombineMode::Any);
    assert_eq!(all_result, any_result);
}

#[test]
fn test_mask_combine_mode_all_requires_both() {
    // With MaskCombineMode::All (AND), cell must pass ALL masks
    let masks: SmallVec<[MaskSpec; 2]> = smallvec![
        // Left half visible at t=0.5
        MaskSpec::Wipe {
            reveal: Some(WipeDirection::LeftToRight),
            hide: None,
            direction: None,
            soft_edge: false,
        },
        // Top half visible at t=0.5
        MaskSpec::Wipe {
            reveal: Some(WipeDirection::TopToBottom),
            hide: None,
            direction: None,
            soft_edge: false,
        },
    ];

    // Top-left quadrant should be visible (both masks pass)
    assert!(check_masks(
        10,
        5,
        80,
        24,
        0.5,
        &masks,
        MaskCombineMode::All
    ));

    // Bottom-left should be hidden (top-to-bottom mask fails)
    assert!(!check_masks(
        10,
        20,
        80,
        24,
        0.5,
        &masks,
        MaskCombineMode::All
    ));

    // Top-right should be hidden (left-to-right mask fails)
    assert!(!check_masks(
        60,
        5,
        80,
        24,
        0.5,
        &masks,
        MaskCombineMode::All
    ));
}

#[test]
fn test_mask_combine_mode_any_requires_one() {
    // With MaskCombineMode::Any (OR), cell passes if ANY mask passes
    let masks: SmallVec<[MaskSpec; 2]> = smallvec![
        // Left half visible at t=0.5
        MaskSpec::Wipe {
            reveal: Some(WipeDirection::LeftToRight),
            hide: None,
            direction: None,
            soft_edge: false,
        },
        // Top half visible at t=0.5
        MaskSpec::Wipe {
            reveal: Some(WipeDirection::TopToBottom),
            hide: None,
            direction: None,
            soft_edge: false,
        },
    ];

    // Top-left quadrant visible (both masks pass)
    assert!(check_masks(
        10,
        5,
        80,
        24,
        0.5,
        &masks,
        MaskCombineMode::Any
    ));

    // Bottom-left visible (left-to-right mask passes)
    assert!(check_masks(
        10,
        20,
        80,
        24,
        0.5,
        &masks,
        MaskCombineMode::Any
    ));

    // Top-right visible (top-to-bottom mask passes)
    assert!(check_masks(
        60,
        5,
        80,
        24,
        0.5,
        &masks,
        MaskCombineMode::Any
    ));

    // Bottom-right hidden (both masks fail)
    assert!(!check_masks(
        60,
        20,
        80,
        24,
        0.5,
        &masks,
        MaskCombineMode::Any
    ));
}

// =============================================================================
// Filter Application Order Tests
// =============================================================================

#[test]
fn test_filter_list_preserves_order() {
    use mixed_signals::types::SignalOrFloat;

    let filters: SmallVec<[FilterSpec; 3]> = smallvec![
        FilterSpec::Dim {
            factor: SignalOrFloat::Static(0.5),
            apply_to: tui_vfx_compositor::types::ApplyTo::Both,
        },
        FilterSpec::Invert {
            apply_to: tui_vfx_compositor::types::ApplyTo::Foreground,
        },
    ];

    // Verify order is preserved
    assert!(matches!(filters[0], FilterSpec::Dim { .. }));
    assert!(matches!(filters[1], FilterSpec::Invert { .. }));
}

// <FILE>tui-vfx-compositor/tests/pipeline/test_multiple_effects.rs</FILE> - <DESC>Tests for multiple masks, filters, shaders</DESC>
// <VERS>END OF VERSION: 1.2.0</VERS>
