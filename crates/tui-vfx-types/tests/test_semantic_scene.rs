// <FILE>crates/tui-vfx-types/tests/test_semantic_scene.rs</FILE> - <DESC>Tests for SemanticScene foundation primitive</DESC>
// <VERS>VERSION: 0.1.0</VERS>
// <WCTX>Sub-plan A Phase A.1 — foundation primitive tests</WCTX>
// <CLOG>0.1.0: TDD red tests for new/with_metadata/from_grid_with_default_role, Buffer-style accessors, dimension-mismatch panic.</CLOG>

use tui_vfx_types::{
    Cell, Grid, OwnedGrid, RecipeId, Rect, RoleMap, RoleTag, SceneMetadata, SemanticScene,
};

fn simple_grid(w: usize, h: usize) -> OwnedGrid {
    let mut grid = OwnedGrid::new(w, h);
    // Paint a distinctive cell so parity tests can tell things apart.
    grid.set(1, 1, Cell::new('X'));
    grid
}

#[test]
fn new_constructs_with_matching_dimensions() {
    let grid = simple_grid(4, 3);
    let roles = RoleMap::empty(4, 3);
    let scene = SemanticScene::new(grid, roles);
    assert_eq!(scene.area(), Rect::new(0, 0, 4, 3));
}

#[test]
#[should_panic(expected = "dimension")]
fn new_panics_on_dimension_mismatch() {
    let grid = simple_grid(4, 3);
    let roles = RoleMap::empty(5, 3);
    let _ = SemanticScene::new(grid, roles);
}

#[test]
fn area_returns_rect_based_on_dimensions() {
    let scene = SemanticScene::new(simple_grid(10, 6), RoleMap::empty(10, 6));
    let area = scene.area();
    assert_eq!(area.x, 0);
    assert_eq!(area.y, 0);
    assert_eq!(area.width, 10);
    assert_eq!(area.height, 6);
}

#[test]
fn cell_accessor_parity_with_grid() {
    let grid = simple_grid(4, 3);
    let roles = RoleMap::empty(4, 3);
    let scene = SemanticScene::new(grid, roles);
    // cell((x, y)) returns Some(&Cell) for in-bounds
    assert_eq!(scene.cell((1, 1)).map(|c| c.ch), Some('X'));
    assert_eq!(scene.cell((0, 0)).map(|c| c.ch), Some(' '));
    assert_eq!(scene.cell((10, 10)), None);
    // Verify parity with direct grid access
    assert_eq!(scene.cell((1, 1)), scene.grid().get(1, 1));
}

#[test]
fn role_accessor_parity_with_rolemap() {
    let grid = simple_grid(4, 3);
    let mut roles = RoleMap::empty(4, 3);
    roles.set((2, 2), RoleTag::Shadow);
    let scene = SemanticScene::new(grid, roles);
    assert_eq!(scene.role((2, 2)), Some(RoleTag::Shadow));
    assert_eq!(scene.role((0, 0)), Some(RoleTag::Background));
    assert_eq!(scene.role((100, 100)), None);
    assert_eq!(scene.role((2, 2)), scene.roles().get((2, 2)));
}

#[test]
fn with_metadata_preserves_grid_and_roles() {
    let scene = SemanticScene::new(simple_grid(2, 2), RoleMap::empty(2, 2));
    let mut md = SceneMetadata::default();
    md.recipe_id = Some(RecipeId::from("my_recipe"));
    md.composer_version = Some("0.6.0".into());
    md.produced_at = Some(42);
    md.layer_count = 3;
    let scene = scene.with_metadata(md.clone());
    assert_eq!(scene.metadata().recipe_id, md.recipe_id);
    assert_eq!(scene.metadata().composer_version, md.composer_version);
    assert_eq!(scene.metadata().produced_at, md.produced_at);
    assert_eq!(scene.metadata().layer_count, md.layer_count);
    // Grid and roles still intact
    assert_eq!(scene.cell((1, 1)).map(|c| c.ch), Some('X'));
    assert_eq!(scene.role((0, 0)), Some(RoleTag::Background));
}

#[test]
fn from_grid_with_default_role_tags_every_cell() {
    let scene = SemanticScene::from_grid_with_default_role(simple_grid(3, 2), RoleTag::Image);
    for y in 0..2u16 {
        for x in 0..3u16 {
            assert_eq!(scene.role((x, y)), Some(RoleTag::Image));
        }
    }
    // Grid content preserved
    assert_eq!(scene.cell((1, 1)).map(|c| c.ch), Some('X'));
}

#[test]
fn roles_mut_allows_in_place_mutation() {
    let mut scene = SemanticScene::new(simple_grid(2, 2), RoleMap::empty(2, 2));
    scene.roles_mut().set((0, 0), RoleTag::Border);
    assert_eq!(scene.role((0, 0)), Some(RoleTag::Border));
}

// <FILE>crates/tui-vfx-types/tests/test_semantic_scene.rs</FILE> - <DESC>SemanticScene tests</DESC>
// <VERS>END OF VERSION: 0.1.0</VERS>
