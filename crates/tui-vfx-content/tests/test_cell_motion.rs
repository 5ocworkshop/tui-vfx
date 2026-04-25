// <FILE>crates/tui-vfx-content/tests/cell_motion/test_fnc_apply_cell_motion.rs</FILE> - <DESC>Tests for V3 content cell-motion scheduler</DESC>
// <VERS>VERSION: 0.1.0</VERS>
// <WCTX>V3 Packet 1: lock pure content-local per-cell motion scheduler semantics.</WCTX>
// <CLOG>0.1.0: add Packet 1 scheduler coverage for selection, collision, timing, reduced motion, stats, and non-zero frames.</CLOG>

use tui_vfx_content::cell_motion::*;
use tui_vfx_geometry::types::Anchor;
use tui_vfx_types::{Cell, Grid, OwnedGrid, RoleMap, RoleTag, SemanticScene};

fn scene(rows: &[&str]) -> SemanticScene {
    let h = rows.len();
    let w = rows.first().map_or(0, |r| r.chars().count());
    let mut grid = OwnedGrid::new(w, h);
    let mut roles = RoleMap::new_with_default(w as u16, h as u16, RoleTag::Text);
    for (y, row) in rows.iter().enumerate() {
        for (x, ch) in row.chars().enumerate() {
            if ch != '.' {
                grid.set(x, y, Cell::new(ch));
            } else {
                roles.set((x as u16, y as u16), RoleTag::Background);
            }
        }
    }
    SemanticScene::new(grid, roles)
}

fn ch(s: &SemanticScene, x: u16, y: u16) -> char {
    s.cell((x, y)).unwrap().ch
}

fn timing(ms: u64) -> CellMotionTiming {
    CellMotionTiming {
        phase: CellMotionPhase::Enter,
        phase_elapsed_ms: ms,
        phase_t: 0.0,
        absolute_t_ms: ms as f64,
        reduced_motion: false,
        seed: 0,
    }
}

fn spec(from: CellPlacement, to: CellPlacement) -> CellMotionSpec {
    CellMotionSpec {
        enter: Some(CellMotionPhaseSpec {
            duration_ms: 100,
            from,
            to,
            ..Default::default()
        }),
        exit: None,
    }
}

fn apply(sc: &SemanticScene, sp: CellMotionSpec, ms: u64) -> CellMotionResult {
    apply_cell_motion(
        sc,
        &sp,
        &timing(ms),
        sc.area(),
        &CellMotionOptions::default(),
    )
}

#[test]
fn cell_motion_middle_out_center_to_authored() {
    let sc = scene(&["ABC"]);
    let res = apply(
        &sc,
        spec(
            CellPlacement::Origin {
                anchor: Anchor::Center,
                basis: CellPlacementBasis::SelectionBounds,
            },
            CellPlacement::Authored,
        ),
        0,
    );
    assert_eq!(ch(&res.scene, 1, 0), 'C');
    assert_eq!(res.stats.collision_count, 2);
}

#[test]
fn cell_motion_unselected_cells_remain_unchanged() {
    let sc = scene(&["ABC"]);
    let mut ph = CellMotionPhaseSpec {
        duration_ms: 100,
        from: CellPlacement::AuthoredOffset { dx: 1, dy: 0 },
        to: CellPlacement::AuthoredOffset { dx: 1, dy: 0 },
        ..Default::default()
    };
    ph.scope = Some(CellMotionScope::Cells {
        cells: vec![(0, 0).into()],
    });
    let res = apply(
        &sc,
        CellMotionSpec {
            enter: Some(ph),
            exit: None,
        },
        100,
    );
    assert_eq!(ch(&res.scene, 1, 0), 'A');
    assert_eq!(ch(&res.scene, 2, 0), 'C');
}

#[test]
fn cell_motion_selected_cells_vacate_authored_positions() {
    let sc = scene(&["A."]);
    let res = apply(
        &sc,
        spec(
            CellPlacement::AuthoredOffset { dx: 1, dy: 0 },
            CellPlacement::AuthoredOffset { dx: 1, dy: 0 },
        ),
        100,
    );
    assert_eq!(ch(&res.scene, 0, 0), ' ');
    assert_eq!(ch(&res.scene, 1, 0), 'A');
}

#[test]
fn cell_motion_collision_source_order() {
    let sc = scene(&["AB"]);
    let res = apply(
        &sc,
        spec(
            CellPlacement::Absolute { x: 0, y: 0 },
            CellPlacement::Absolute { x: 0, y: 0 },
        ),
        100,
    );
    assert_eq!(ch(&res.scene, 0, 0), 'B');
    assert_eq!(res.stats.collision_count, 1);
}

#[test]
fn cell_motion_collision_preserve_existing() {
    let sc = scene(&["AB"]);
    let mut ph = CellMotionPhaseSpec {
        duration_ms: 100,
        from: CellPlacement::Absolute { x: 1, y: 0 },
        to: CellPlacement::Absolute { x: 1, y: 0 },
        collision: CellCollisionMode::PreserveExisting,
        ..Default::default()
    };
    ph.scope = Some(CellMotionScope::Cells {
        cells: vec![(0, 0).into()],
    });
    let res = apply(
        &sc,
        CellMotionSpec {
            enter: Some(ph),
            exit: None,
        },
        100,
    );
    assert_eq!(ch(&res.scene, 1, 0), 'B');
    assert_eq!(res.stats.collision_count, 1);
}

#[test]
fn cell_motion_nearest_to_completion_tie_breaks() {
    let sc = scene(&["AB"]);
    let ph = CellMotionPhaseSpec {
        duration_ms: 100,
        from: CellPlacement::Absolute { x: 0, y: 0 },
        to: CellPlacement::Absolute { x: 0, y: 0 },
        collision: CellCollisionMode::NearestToCompletion,
        ..Default::default()
    };
    let res = apply(
        &sc,
        CellMotionSpec {
            enter: Some(ph),
            exit: None,
        },
        50,
    );
    assert_eq!(ch(&res.scene, 0, 0), 'B');
}

#[test]
fn cell_motion_zero_duration_enter_teleports_to_to() {
    let sc = scene(&["A."]);
    let sp = CellMotionSpec {
        enter: Some(CellMotionPhaseSpec {
            duration_ms: 0,
            from: CellPlacement::Authored,
            to: CellPlacement::Absolute { x: 1, y: 0 },
            ..Default::default()
        }),
        exit: None,
    };
    let res = apply(&sc, sp, 0);
    assert_eq!(ch(&res.scene, 1, 0), 'A');
}

#[test]
fn cell_motion_zero_duration_exit_hides() {
    let sc = scene(&["A"]);
    let sp = CellMotionSpec {
        enter: None,
        exit: Some(CellMotionPhaseSpec {
            duration_ms: 0,
            from: CellPlacement::Authored,
            to: CellPlacement::Authored,
            ..Default::default()
        }),
    };
    let mut t = timing(0);
    t.phase = CellMotionPhase::Exit;
    let res = apply_cell_motion(&sc, &sp, &t, sc.area(), &CellMotionOptions::default());
    assert_eq!(ch(&res.scene, 0, 0), ' ');
}

#[test]
fn cell_motion_stagger_longer_than_phase_hides_before_start() {
    let sc = scene(&["A"]);
    let ph = CellMotionPhaseSpec {
        duration_ms: 100,
        from: CellPlacement::Authored,
        to: CellPlacement::Authored,
        stagger: CellStagger::ByIndex { stride_ms: 999 },
        ..Default::default()
    };
    let res = apply(
        &sc,
        CellMotionSpec {
            enter: Some(ph),
            exit: None,
        },
        10,
    );
    assert_eq!(res.stats.hidden_before_start_count, 0);
}

#[test]
fn cell_motion_random_stagger_is_deterministic() {
    let sc = scene(&["AB"]);
    let ph = CellMotionPhaseSpec {
        duration_ms: 100,
        from: CellPlacement::Authored,
        to: CellPlacement::Authored,
        stagger: CellStagger::Random {
            seed: 7,
            max_offset_ms: 50,
        },
        ..Default::default()
    };
    let sp = CellMotionSpec {
        enter: Some(ph),
        exit: None,
    };
    assert_eq!(
        apply(&sc, sp.clone(), 100).stats.max_stagger_offset_ms,
        apply(&sc, sp, 100).stats.max_stagger_offset_ms
    );
}

#[test]
fn cell_motion_clips_out_of_bounds() {
    let sc = scene(&["A"]);
    let res = apply(
        &sc,
        spec(
            CellPlacement::Absolute { x: -1, y: 0 },
            CellPlacement::Absolute { x: -1, y: 0 },
        ),
        100,
    );
    assert_eq!(res.stats.clipped_actor_count, 1);
}

#[test]
fn cell_motion_preserves_role_tags() {
    let mut sc = scene(&["A."]);
    sc.roles_mut().set((0, 0), RoleTag::Title);
    let res = apply(
        &sc,
        spec(
            CellPlacement::Absolute { x: 1, y: 0 },
            CellPlacement::Absolute { x: 1, y: 0 },
        ),
        100,
    );
    assert_eq!(res.scene.role((1, 0)), Some(RoleTag::Title));
}

#[test]
fn cell_motion_non_empty_uses_cell_is_empty_contract() {
    let sc = scene(&["."]);
    let res = apply(
        &sc,
        spec(CellPlacement::Authored, CellPlacement::Authored),
        100,
    );
    assert_eq!(res.stats.selected_actor_count, 0);
}

#[test]
fn cell_motion_wide_grapheme_smoke_currently_cell_based() {
    let sc = scene(&["界界"]);
    let res = apply(
        &sc,
        spec(
            CellPlacement::Origin {
                anchor: Anchor::Center,
                basis: CellPlacementBasis::SelectionBounds,
            },
            CellPlacement::Authored,
        ),
        100,
    );
    assert_eq!(res.stats.selected_actor_count, 2);
}

#[test]
fn cell_motion_reduced_motion_enter_places_to_after_stagger() {
    let sc = scene(&["A."]);
    let mut t = timing(10);
    t.reduced_motion = true;
    let res = apply_cell_motion(
        &sc,
        &spec(
            CellPlacement::Authored,
            CellPlacement::Absolute { x: 1, y: 0 },
        ),
        &t,
        sc.area(),
        &CellMotionOptions::default(),
    );
    assert_eq!(ch(&res.scene, 1, 0), 'A');
}

#[test]
fn cell_motion_reduced_motion_exit_hides_after_stagger() {
    let sc = scene(&["A"]);
    let sp = CellMotionSpec {
        enter: None,
        exit: Some(CellMotionPhaseSpec {
            duration_ms: 100,
            from: CellPlacement::Authored,
            to: CellPlacement::Authored,
            ..Default::default()
        }),
    };
    let mut t = timing(10);
    t.phase = CellMotionPhase::Exit;
    t.reduced_motion = true;
    let res = apply_cell_motion(&sc, &sp, &t, sc.area(), &CellMotionOptions::default());
    assert_eq!(ch(&res.scene, 0, 0), ' ');
}

#[test]
fn cell_motion_quantize_steps_rejects_zero_and_one() {
    let mut ph = CellMotionPhaseSpec {
        quantize_steps: Some(0),
        ..Default::default()
    };
    assert_eq!(ph.validate(), Err(CellMotionError::InvalidQuantizeSteps));
    ph.quantize_steps = Some(1);
    assert_eq!(ph.validate(), Err(CellMotionError::InvalidQuantizeSteps));
}

#[test]
fn cell_motion_no_unsigned_underflow_before_stagger() {
    let sc = scene(&["A"]);
    let ph = CellMotionPhaseSpec {
        duration_ms: 100,
        from: CellPlacement::Authored,
        to: CellPlacement::Authored,
        stagger: CellStagger::ByIndex {
            stride_ms: u64::MAX,
        },
        ..Default::default()
    };
    let _ = apply(
        &sc,
        CellMotionSpec {
            enter: Some(ph),
            exit: None,
        },
        0,
    );
}

#[test]
fn cell_motion_stats_count_baseline_overwrites() {
    let sc = scene(&["AB"]);
    let mut ph = CellMotionPhaseSpec {
        duration_ms: 100,
        from: CellPlacement::Absolute { x: 1, y: 0 },
        to: CellPlacement::Absolute { x: 1, y: 0 },
        ..Default::default()
    };
    ph.scope = Some(CellMotionScope::Cells {
        cells: vec![(0, 0).into()],
    });
    let res = apply(
        &sc,
        CellMotionSpec {
            enter: Some(ph),
            exit: None,
        },
        100,
    );
    assert_eq!(res.stats.baseline_overwrite_count, 1);
}

#[test]
fn cell_motion_non_zero_local_frame_returns_scene_local_coordinates() {
    let sc = scene(&["A."]);
    let frame = tui_vfx_types::Rect::new(10, 5, 2, 1);
    let res = apply_cell_motion(
        &sc,
        &spec(CellPlacement::Authored, CellPlacement::Authored),
        &timing(100),
        frame,
        &CellMotionOptions::default(),
    );
    assert_eq!(ch(&res.scene, 0, 0), 'A');
    assert_eq!(res.stats.clipped_actor_count, 0);
}

// <FILE>crates/tui-vfx-content/tests/cell_motion/test_fnc_apply_cell_motion.rs</FILE>
// <VERS>END OF VERSION: 0.1.0</VERS>
