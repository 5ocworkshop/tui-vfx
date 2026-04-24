// <FILE>crates/tui-vfx-style/tests/models/test_fnc_apply_style_effects_to_scene.rs</FILE> - <DESC>Focused tests for scene-wide non-spatial style application</DESC>
// <VERS>VERSION: 0.1.0</VERS>
// <WCTX>Move non-spatial style-effect execution closer to the style/runtime surface by proving the style crate can mutate semantic scenes directly with region-aware timing behavior.</WCTX>
// <CLOG>0.1.0: add role-aware and loop-aware coverage for apply_style_effects_to_scene.</CLOG>

use tui_vfx_style::models::{StyleEffect, StyleRegion, apply_style_effects_to_scene};
use tui_vfx_types::{Cell, Color, Grid, Modifiers, OwnedGrid, RoleMap, RoleTag, SemanticScene};

#[test]
fn apply_style_effects_to_scene_respects_style_regions() {
    let mut grid = OwnedGrid::new(2, 1);
    grid.set(
        0,
        0,
        Cell::styled('A', Color::RED, Color::BLACK, Modifiers::NONE),
    );
    grid.set(
        1,
        0,
        Cell::styled('B', Color::RED, Color::BLACK, Modifiers::NONE),
    );
    let mut roles = RoleMap::all_background(2, 1);
    roles.set((0, 0), RoleTag::Text);
    let mut scene = SemanticScene::new(grid, roles);

    apply_style_effects_to_scene(
        &mut scene,
        &[(
            StyleEffect::Pulse {
                frequency: 1.0,
                color: Color::BLUE,
            },
            StyleRegion::Role(RoleTag::Text),
        )],
        0.25,
        Some(0.25),
    );

    assert_eq!(scene.grid().get(0, 0).unwrap().fg, Color::BLUE);
    assert_eq!(scene.grid().get(1, 0).unwrap().fg, Color::RED);
}

#[test]
fn apply_style_effects_to_scene_uses_loop_t_for_modulation_effects() {
    let mut grid = OwnedGrid::new(1, 1);
    grid.set(
        0,
        0,
        Cell::styled('A', Color::RED, Color::BLACK, Modifiers::NONE),
    );
    let roles = RoleMap::all_background(1, 1);
    let mut scene = SemanticScene::new(grid, roles);

    apply_style_effects_to_scene(
        &mut scene,
        &[(StyleEffect::Rainbow { speed: 1.0 }, StyleRegion::All)],
        0.0,
        Some(0.5),
    );

    let fg = scene.grid().get(0, 0).unwrap().fg;
    assert_ne!(fg, Color::RED);
}
