// <FILE>crates/tui-vfx-next/tests/test_scene_contract.rs</FILE> - <DESC>Phase D1 scene/element/layer composition tests</DESC>
// <VERS>VERSION: 0.2.0</VERS>
// <WCTX>New kernel Phase D1 deslop: add direct scene role-write policy regression coverage.</WCTX>
// <CLOG>0.2.0: TEST — cover PreserveDestination and SetExplicit role policies during scene composition.
// 0.1.1: TEST — expect field-stable scene.element[index].id diagnostic paths.
// 0.1.0: TEST — specify placement, z/declaration overlap, skip/write, role propagation, clipping, and element diagnostics.</CLOG>

use tui_vfx_next::{
    CellWritePolicy, ClipPolicy, ElementId, ElementPlacement, LayerId, RoleWritePolicy, Scene,
    SceneElement, Surface, SurfaceDiagnosticCode,
};
use tui_vfx_types::{Cell, Color, Modifiers, RoleTag};

fn cell(ch: char) -> Cell {
    Cell::styled(ch, Color::WHITE, Color::BLACK, Modifiers::NONE)
}

fn element_surface(ch: char, role: RoleTag) -> Surface {
    let mut surface = Surface::new(1, 1, RoleTag::Background);
    surface.set_cell_and_role(0, 0, cell(ch), role);
    surface
}

fn scene_element(id: &str, surface: Surface, x: i32, y: i32, z_index: i32) -> SceneElement {
    SceneElement {
        id: ElementId::new(id),
        layer: None,
        z_index,
        placement: ElementPlacement::new(x, y),
        surface,
        clip_policy: ClipPolicy::Clip,
        cell_write_policy: CellWritePolicy::WriteCell,
        role_write_policy: RoleWritePolicy::CopySampledSource,
    }
}

#[test]
fn scene_composes_multiple_elements() {
    let left = scene_element("left", element_surface('A', RoleTag::Text), 0, 0, 0);
    let right = scene_element("right", element_surface('B', RoleTag::Border), 1, 0, 0);
    let scene = Scene::new(2, 1, vec![left, right]);

    let outcome = scene.compose();

    assert_eq!(outcome.written_cells, 2);
    assert_eq!(outcome.surface.cell(0, 0).unwrap().ch, 'A');
    assert_eq!(outcome.surface.role(0, 0), Some(&RoleTag::Text));
    assert_eq!(outcome.surface.cell(1, 0).unwrap().ch, 'B');
    assert_eq!(outcome.surface.role(1, 0), Some(&RoleTag::Border));
}

#[test]
fn element_identity_is_distinct_from_role() {
    let mut element = scene_element("titleCard", element_surface('T', RoleTag::Text), 0, 0, 0);
    element.layer = Some(LayerId::new("foreground"));
    let scene = Scene::new(1, 1, vec![element]);

    let outcome = scene.compose();

    assert_eq!(scene.elements[0].id.as_str(), "titleCard");
    assert_eq!(
        scene.elements[0].layer.as_ref().unwrap().as_str(),
        "foreground"
    );
    assert_eq!(outcome.surface.role(0, 0), Some(&RoleTag::Text));
}

#[test]
fn higher_z_element_overwrites_lower_cell_and_role() {
    let lower = scene_element(
        "backdrop",
        element_surface('A', RoleTag::Background),
        0,
        0,
        0,
    );
    let higher = scene_element("title", element_surface('B', RoleTag::Text), 0, 0, 10);
    let scene = Scene::new(1, 1, vec![higher, lower]);

    let outcome = scene.compose();

    assert_eq!(outcome.surface.cell(0, 0).unwrap().ch, 'B');
    assert_eq!(outcome.surface.role(0, 0), Some(&RoleTag::Text));
}

#[test]
fn z_tie_breaks_by_declaration_order() {
    let first = scene_element("first", element_surface('1', RoleTag::Border), 0, 0, 0);
    let second = scene_element("second", element_surface('2', RoleTag::Text), 0, 0, 0);
    let scene = Scene::new(1, 1, vec![first, second]);

    let outcome = scene.compose();

    assert_eq!(outcome.surface.cell(0, 0).unwrap().ch, '2');
    assert_eq!(outcome.surface.role(0, 0), Some(&RoleTag::Text));
}

#[test]
fn skipped_top_element_preserves_lower_output() {
    let lower = scene_element("lower", element_surface('L', RoleTag::Border), 0, 0, 0);
    let mut transparent = Surface::new(1, 1, RoleTag::Text);
    transparent.set_cell_and_role(0, 0, Cell::default(), RoleTag::Text);
    let mut top = scene_element("top", transparent, 0, 0, 1);
    top.cell_write_policy = CellWritePolicy::SkipTransparentEmpty;
    let scene = Scene::new(1, 1, vec![lower, top]);

    let outcome = scene.compose();

    assert_eq!(outcome.written_cells, 1);
    assert_eq!(outcome.surface.cell(0, 0).unwrap().ch, 'L');
    assert_eq!(outcome.surface.role(0, 0), Some(&RoleTag::Border));
}

#[test]
fn transparent_empty_top_write_can_clear_when_policy_writes() {
    let lower = scene_element("lower", element_surface('L', RoleTag::Border), 0, 0, 0);
    let mut transparent = Surface::new(1, 1, RoleTag::Text);
    transparent.set_cell_and_role(0, 0, Cell::default(), RoleTag::Text);
    let top = scene_element("top", transparent, 0, 0, 1);
    let scene = Scene::new(1, 1, vec![lower, top]);

    let outcome = scene.compose();

    assert_eq!(outcome.written_cells, 2);
    assert!(outcome.surface.cell(0, 0).unwrap().is_empty());
    assert_eq!(outcome.surface.role(0, 0), Some(&RoleTag::Text));
}

#[test]
fn scene_role_policy_can_preserve_lower_role() {
    let lower = scene_element("lower", element_surface('L', RoleTag::Border), 0, 0, 0);
    let mut top = scene_element("top", element_surface('T', RoleTag::Text), 0, 0, 1);
    top.role_write_policy = RoleWritePolicy::PreserveDestination;
    let scene = Scene::new(1, 1, vec![lower, top]);

    let outcome = scene.compose();

    assert_eq!(outcome.surface.cell(0, 0).unwrap().ch, 'T');
    assert_eq!(outcome.surface.role(0, 0), Some(&RoleTag::Border));
}

#[test]
fn scene_role_policy_can_set_explicit_role() {
    let mut element = scene_element("shadow", element_surface('S', RoleTag::Text), 0, 0, 0);
    element.role_write_policy = RoleWritePolicy::SetExplicit {
        role: RoleTag::Shadow,
    };
    let scene = Scene::new(1, 1, vec![element]);

    let outcome = scene.compose();

    assert_eq!(outcome.surface.cell(0, 0).unwrap().ch, 'S');
    assert_eq!(outcome.surface.role(0, 0), Some(&RoleTag::Shadow));
}

#[test]
fn element_placement_uses_scene_coordinates() {
    let element = scene_element("placed", element_surface('P', RoleTag::Text), 2, 1, 0);
    let scene = Scene::new(4, 3, vec![element]);

    let outcome = scene.compose();

    assert_eq!(outcome.surface.cell(2, 1).unwrap().ch, 'P');
    assert!(outcome.surface.cell(0, 0).unwrap().is_empty());
}

#[test]
fn out_of_bounds_element_cells_are_clipped() {
    let mut surface = Surface::new(2, 1, RoleTag::Text);
    surface.set_cell_and_role(0, 0, cell('A'), RoleTag::Text);
    surface.set_cell_and_role(1, 0, cell('B'), RoleTag::Border);
    let element = scene_element("partial", surface, -1, 0, 0);
    let scene = Scene::new(2, 1, vec![element]);

    let outcome = scene.compose();

    assert_eq!(outcome.written_cells, 1);
    assert_eq!(outcome.clipped_cells, 1);
    assert_eq!(outcome.surface.cell(0, 0).unwrap().ch, 'B');
    assert_eq!(outcome.surface.role(0, 0), Some(&RoleTag::Border));
    assert!(outcome.surface.cell(1, 0).unwrap().is_empty());
}

#[test]
fn scene_diagnostics_include_element_identity() {
    let mut element = scene_element("toast", element_surface('!', RoleTag::Text), -1, 0, 0);
    element.clip_policy = ClipPolicy::Warn;
    let scene = Scene::new(1, 1, vec![element]);

    let outcome = scene.compose();

    assert_eq!(outcome.diagnostics.len(), 1);
    assert_eq!(
        outcome.diagnostics[0].code,
        SurfaceDiagnosticCode::SceneElementClipped
    );
    assert_eq!(
        outcome.diagnostics[0].path.as_deref(),
        Some("scene.element[0].id")
    );
    assert!(outcome.diagnostics[0].message.contains("toast"));
}

// <FILE>crates/tui-vfx-next/tests/test_scene_contract.rs</FILE> - <DESC>Phase D1 scene/element/layer composition tests</DESC>
// <VERS>END OF VERSION: 0.2.0</VERS>
