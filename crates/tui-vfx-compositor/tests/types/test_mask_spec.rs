// <FILE>tui-vfx-compositor/tests/types/test_mask_spec.rs</FILE> - <DESC>Tests for MaskSpec</DESC>
// <VERS>VERSION: 1.1.0</VERS>
// <WCTX>Ergonomic reveal/hide schema for intuitive mask direction semantics</WCTX>
// <CLOG>Update tests for new reveal/hide/direction optional fields</CLOG>

use tui_vfx_compositor::types::{DitherMatrix, IrisShape, MaskSpec, Orientation, WipeDirection};

#[test]
fn test_mask_spec_default_is_none() {
    let spec = MaskSpec::default();
    assert_eq!(spec, MaskSpec::None);
}

#[test]
fn test_wipe_direction_serde_roundtrip() {
    for dir in [
        WipeDirection::LeftToRight,
        WipeDirection::RightToLeft,
        WipeDirection::TopToBottom,
        WipeDirection::BottomToTop,
    ] {
        let json = serde_json::to_string(&dir).unwrap();
        let parsed: WipeDirection = serde_json::from_str(&json).unwrap();
        assert_eq!(dir, parsed);
    }
}

#[test]
fn test_orientation_serde_roundtrip() {
    for orient in [Orientation::Horizontal, Orientation::Vertical] {
        let json = serde_json::to_string(&orient).unwrap();
        let parsed: Orientation = serde_json::from_str(&json).unwrap();
        assert_eq!(orient, parsed);
    }
}

#[test]
fn test_iris_shape_serde_roundtrip() {
    for shape in [IrisShape::Circle, IrisShape::Diamond, IrisShape::Box] {
        let json = serde_json::to_string(&shape).unwrap();
        let parsed: IrisShape = serde_json::from_str(&json).unwrap();
        assert_eq!(shape, parsed);
    }
}

#[test]
fn test_dither_matrix_serde_roundtrip() {
    for matrix in [DitherMatrix::Bayer4, DitherMatrix::Bayer8] {
        let json = serde_json::to_string(&matrix).unwrap();
        let parsed: DitherMatrix = serde_json::from_str(&json).unwrap();
        assert_eq!(matrix, parsed);
    }
}

#[test]
fn test_mask_spec_wipe_serde_roundtrip() {
    let spec = MaskSpec::Wipe {
        reveal: Some(WipeDirection::TopToBottom),
        hide: None,
        direction: None,
        soft_edge: true,
    };
    let json = serde_json::to_string(&spec).unwrap();
    let parsed: MaskSpec = serde_json::from_str(&json).unwrap();
    assert_eq!(spec, parsed);
}

#[test]
fn test_mask_spec_dissolve_serde_roundtrip() {
    let spec = MaskSpec::Dissolve {
        seed: 42,
        chunk_size: 2,
    };
    let json = serde_json::to_string(&spec).unwrap();
    let parsed: MaskSpec = serde_json::from_str(&json).unwrap();
    assert_eq!(spec, parsed);
}

#[test]
fn test_mask_spec_checkers_serde_roundtrip() {
    let spec = MaskSpec::Checkers { cell_size: 4 };
    let json = serde_json::to_string(&spec).unwrap();
    let parsed: MaskSpec = serde_json::from_str(&json).unwrap();
    assert_eq!(spec, parsed);
}

#[test]
fn test_mask_spec_blinds_serde_roundtrip() {
    let spec = MaskSpec::Blinds {
        orientation: Orientation::Vertical,
        count: 8,
    };
    let json = serde_json::to_string(&spec).unwrap();
    let parsed: MaskSpec = serde_json::from_str(&json).unwrap();
    assert_eq!(spec, parsed);
}

#[test]
fn test_mask_spec_iris_serde_roundtrip() {
    let spec = MaskSpec::Iris {
        shape: IrisShape::Circle,
        soft_edge: false,
    };
    let json = serde_json::to_string(&spec).unwrap();
    let parsed: MaskSpec = serde_json::from_str(&json).unwrap();
    assert_eq!(spec, parsed);
}

#[test]
fn test_mask_spec_diamond_serde_roundtrip() {
    let spec = MaskSpec::Diamond { soft_edge: true };
    let json = serde_json::to_string(&spec).unwrap();
    let parsed: MaskSpec = serde_json::from_str(&json).unwrap();
    assert_eq!(spec, parsed);
}

#[test]
fn test_mask_spec_noise_dither_serde_roundtrip() {
    let spec = MaskSpec::NoiseDither {
        seed: 123,
        matrix: DitherMatrix::Bayer8,
    };
    let json = serde_json::to_string(&spec).unwrap();
    let parsed: MaskSpec = serde_json::from_str(&json).unwrap();
    assert_eq!(spec, parsed);
}

// <FILE>tui-vfx-compositor/tests/types/test_mask_spec.rs</FILE> - <DESC>Tests for MaskSpec</DESC>
// <VERS>END OF VERSION: 1.1.0</VERS>
