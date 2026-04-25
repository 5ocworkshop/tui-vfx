// <FILE>crates/tui-vfx-content/src/cell_motion/fnc_sample_cell_motion_position.rs</FILE> - <DESC>Sample and snap one cell-motion actor position</DESC>
// <VERS>VERSION: 0.1.0</VERS>
// <WCTX>V3 Packet 1: keep geometry path sampling shared with host/layer motion.</WCTX>
// <CLOG>0.1.0: add cell-motion path sampling helper.</CLOG>

use tui_vfx_geometry::{
    transitions::interpolate_position,
    types::{PathType, Position, SnappingStrategy},
};

pub(crate) fn sample_cell_motion_position(
    from: Position,
    via: Option<Position>,
    to: Position,
    t: f64,
    path: &PathType,
    snap: &SnappingStrategy,
) -> Position {
    let resolved = match (path, via) {
        (PathType::Bezier { .. }, Some(v)) => PathType::Bezier {
            control_x: v.x as f32,
            control_y: v.y as f32,
        },
        _ => path.clone(),
    };
    let (x, y) = interpolate_position(from, to, t, &resolved);
    match snap {
        SnappingStrategy::Floor => Position::new(x.floor() as i32, y.floor() as i32),
        SnappingStrategy::Round | SnappingStrategy::Stochastic { .. } => {
            Position::new(x.round() as i32, y.round() as i32)
        }
    }
}

// <FILE>crates/tui-vfx-content/src/cell_motion/fnc_sample_cell_motion_position.rs</FILE>
// <VERS>END OF VERSION: 0.1.0</VERS>
