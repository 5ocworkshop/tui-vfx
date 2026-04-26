// <FILE>crates/tui-vfx-content/src/mechanical/cls_resolved_cycle.rs</FILE> - <DESC>Internal types holding resolved face grids and routes derived from MechanicalCycleConfig</DESC>
// <VERS>VERSION: 0.1.0</VERS>
// <WCTX>Phase 2 of mechanical circular content cycles plan: data carriers shared by cycle resolution and route building.</WCTX>
// <CLOG>0.1.0: introduce ResolvedMechanicalFace, ResolvedMechanicalCycle, MechanicalCycleRoute, NumericRouteHint.</CLOG>

use crate::types::{CycleDirectionPolicy, CycleWrapMode};
#[cfg(test)]
use tui_vfx_types::Grid;
use tui_vfx_types::OwnedGrid;

/// One face after string-to-grid normalization.
///
/// `value` is the original authored string (used for face lookup and
/// error messages); `grid` is the normalized `OwnedGrid` padded to the
/// mechanism tile size.
#[derive(Debug, Clone)]
pub(crate) struct ResolvedMechanicalFace {
    pub(crate) value: String,
    pub(crate) grid: OwnedGrid,
}

/// A face cycle resolved from `MechanicalContentSource` against a
/// specific mechanism tile size.
#[derive(Debug, Clone)]
pub(crate) struct ResolvedMechanicalCycle {
    pub(crate) faces: Vec<ResolvedMechanicalFace>,
    pub(crate) wrap: CycleWrapMode,
}

impl ResolvedMechanicalCycle {
    /// Returns the index of `value` in `faces`, or `None` if the face
    /// is not present.
    pub(crate) fn index_of(&self, value: &str) -> Option<usize> {
        self.faces.iter().position(|f| f.value == value)
    }
}

/// One concrete route through a [`ResolvedMechanicalCycle`].
///
/// `faces` always includes both endpoints. `selected_direction` is the
/// resolved direction the route walked, after `Shortest`/`NumericDelta`
/// reduction.
#[derive(Debug, Clone)]
pub(crate) struct MechanicalCycleRoute {
    pub(crate) faces: Vec<ResolvedMechanicalFace>,
    pub(crate) selected_direction: CycleDirectionPolicy,
}

/// External hint that disambiguates `CycleDirectionPolicy::NumericDelta`
/// when source-vs-target numeric comparison is ambiguous (e.g., the
/// digits change but the multi-tile carry direction is decided one
/// level higher than the per-tile route helper).
///
/// `Increment` resolves to `Forward`; `Decrement` resolves to `Reverse`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum NumericRouteHint {
    Increment,
    Decrement,
}

#[cfg(test)]
mod tests {
    use super::*;
    use tui_vfx_types::Cell;

    fn face(value: &str, dim: usize) -> ResolvedMechanicalFace {
        let mut grid = OwnedGrid::new(dim, dim);
        for (i, ch) in value.chars().enumerate() {
            grid.set(i % dim, i / dim, Cell::new(ch));
        }
        ResolvedMechanicalFace {
            value: value.into(),
            grid,
        }
    }

    #[test]
    fn index_of_returns_position_for_present_face() {
        let cycle = ResolvedMechanicalCycle {
            faces: vec![face("0", 1), face("1", 1), face("2", 1)],
            wrap: CycleWrapMode::Circular,
        };
        assert_eq!(cycle.index_of("0"), Some(0));
        assert_eq!(cycle.index_of("1"), Some(1));
        assert_eq!(cycle.index_of("2"), Some(2));
    }

    #[test]
    fn index_of_returns_none_for_absent_face() {
        let cycle = ResolvedMechanicalCycle {
            faces: vec![face("A", 1)],
            wrap: CycleWrapMode::Circular,
        };
        assert_eq!(cycle.index_of("Z"), None);
    }
}

// <FILE>crates/tui-vfx-content/src/mechanical/cls_resolved_cycle.rs</FILE>
// <VERS>END OF VERSION: 0.1.0</VERS>
