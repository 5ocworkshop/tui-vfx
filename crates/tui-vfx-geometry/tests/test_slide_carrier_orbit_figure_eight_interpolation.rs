// <FILE>crates/tui-vfx-geometry/tests/test_slide_carrier_orbit_figure_eight_interpolation.rs</FILE> - <DESC>CarrierOrbit/helix and FigureEight path interpolation coverage</DESC>
// <VERS>VERSION: 0.1.0</VERS>
// <WCTX>V3 path primitive expansion from whoa/cellophane adjacent-review: add helix and infinity authoring support while preserving substrate naming.</WCTX>
// <CLOG>0.1.0: verify CarrierOrbit endpoint safety, helix alias deserialization, FigureEight center crossing, and infinity alias deserialization.</CLOG>

use tui_vfx_geometry::transitions::interpolate_position;
use tui_vfx_geometry::types::{PathType, Position};

fn close(actual: f32, expected: f32) {
    assert!(
        (actual - expected).abs() < 0.001,
        "expected {actual} ~= {expected}"
    );
}

#[test]
fn carrier_orbit_preserves_endpoints_and_deviates_from_carrier() {
    let path = PathType::CarrierOrbit {
        rotations: 1.0,
        radius: 4.0,
        phase: 0.0,
        direction: 1.0,
    };
    let from = Position::new(0, 0);
    let to = Position::new(10, 0);
    let start = interpolate_position(from, to, 0.0, &path);
    let quarter = interpolate_position(from, to, 0.25, &path);
    let end = interpolate_position(from, to, 1.0, &path);
    close(start.0, 0.0);
    close(start.1, 0.0);
    close(end.0, 10.0);
    close(end.1, 0.0);
    assert!(quarter.1.abs() > 2.0);
}

#[test]
fn helix_alias_deserializes_to_carrier_orbit() {
    let path: PathType = serde_json::from_value(serde_json::json!({
        "type": "helix",
        "rotations": 2.0,
        "radius": 3.0,
        "direction": -1.0
    }))
    .expect("helix alias should deserialize");

    assert!(matches!(
        path,
        PathType::CarrierOrbit {
            rotations: 2.0,
            radius: 3.0,
            phase: 0.0,
            direction: -1.0
        }
    ));
}

#[test]
fn figure_eight_crosses_center_and_preserves_endpoints() {
    let path = PathType::FigureEight {
        width: 5.0,
        height: 4.0,
        phase: 0.0,
    };
    let from = Position::new(0, 0);
    let to = Position::new(10, 0);
    let start = interpolate_position(from, to, 0.0, &path);
    let center = interpolate_position(from, to, 0.5, &path);
    let end = interpolate_position(from, to, 1.0, &path);
    close(start.0, 0.0);
    close(start.1, 0.0);
    close(center.0, 5.0);
    close(center.1, 0.0);
    close(end.0, 10.0);
    close(end.1, 0.0);
}

#[test]
fn infinity_alias_deserializes_to_figure_eight() {
    let path: PathType = serde_json::from_value(serde_json::json!({
        "type": "infinity",
        "width": 4.0,
        "height": 2.0
    }))
    .expect("infinity alias should deserialize");

    assert!(matches!(
        path,
        PathType::FigureEight {
            width: 4.0,
            height: 2.0,
            phase: 0.0
        }
    ));
}

// <FILE>crates/tui-vfx-geometry/tests/test_slide_carrier_orbit_figure_eight_interpolation.rs</FILE> - <DESC>CarrierOrbit/helix and FigureEight path interpolation coverage</DESC>
// <VERS>END OF VERSION: 0.1.0</VERS>
