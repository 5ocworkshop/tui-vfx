use tui_vfx_geometry::transitions::interpolate_position;
use tui_vfx_geometry::types::{PathType, Position};

#[test]
fn composed_path_supports_pendulum_over_arc() {
    let from = Position::new(0, 10);
    let to = Position::new(20, 10);
    let arc = interpolate_position(from, to, 0.25, &PathType::Arc { bulge: -0.3 });
    let composed = interpolate_position(
        from,
        to,
        0.25,
        &PathType::Composed {
            route: Box::new(PathType::Arc { bulge: -0.3 }),
            dynamics: vec![PathType::Orbit {
                revolutions: 1.0,
                direction: 1.0,
            }],
        },
    );

    assert_ne!(arc, composed, "dynamic layer should perturb the carrier route");
}

#[test]
fn composed_path_with_no_dynamics_matches_route() {
    let from = Position::new(0, 0);
    let to = Position::new(10, 10);
    let route = PathType::Bezier {
        control_x: 4.0,
        control_y: 12.0,
    };
    let base = interpolate_position(from, to, 0.5, &route);
    let composed = interpolate_position(
        from,
        to,
        0.5,
        &PathType::Composed {
            route: Box::new(route.clone()),
            dynamics: vec![],
        },
    );

    assert_eq!(base, composed);
}
