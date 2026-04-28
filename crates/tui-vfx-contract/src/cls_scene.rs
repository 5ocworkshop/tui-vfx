// <FILE>crates/tui-vfx-contract/src/cls_scene.rs</FILE> - <DESC>Multi-element semantic scene composition contract</DESC>
// <VERS>VERSION: 0.1.0</VERS>
// <WCTX>New kernel Phase D1: compose placed semantic elements into one final surface.</WCTX>
// <CLOG>0.1.0: ADD — lock deterministic placement, z/declaration order, overlap, skip/write, role, clip, and diagnostics semantics.</CLOG>

use tui_vfx_types::RoleTag;

use crate::{
    CellWritePolicy, ClipPolicy, RoleWritePolicy, SceneElement, SceneOutcome, Surface,
    SurfaceDiagnostic,
};

/// Scene composed from one or more placed semantic elements.
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, schemars::JsonSchema)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct Scene {
    /// Width of the final composed scene in cells.
    pub width: usize,
    /// Height of the final composed scene in cells.
    pub height: usize,
    /// Elements composed into the scene.
    pub elements: Vec<SceneElement>,
}

impl Scene {
    /// Create a scene with a final size and declaration-ordered elements.
    pub fn new(width: usize, height: usize, elements: Vec<SceneElement>) -> Self {
        Self {
            width,
            height,
            elements,
        }
    }

    /// Compose scene elements into one final semantic surface.
    pub fn compose(&self) -> SceneOutcome {
        let mut surface = Surface::new(self.width, self.height, RoleTag::Background);
        let mut matched_cells = 0;
        let mut written_cells = 0;
        let mut clipped_cells = 0;
        let mut diagnostics = Vec::new();
        let mut ordered_elements = self.elements.iter().enumerate().collect::<Vec<_>>();
        ordered_elements
            .sort_by_key(|(declaration_index, element)| (element.z_index, *declaration_index));

        for (declaration_index, element) in ordered_elements {
            let mut element_clipped_cells = 0;
            for local_y in 0..element.surface.height() {
                for local_x in 0..element.surface.width() {
                    let scene_x = element.placement.x + local_x as i32;
                    let scene_y = element.placement.y + local_y as i32;
                    let Some((scene_x, scene_y)) =
                        in_scene_bounds(scene_x, scene_y, self.width, self.height)
                    else {
                        clipped_cells += 1;
                        element_clipped_cells += 1;
                        continue;
                    };
                    matched_cells += 1;
                    let cell = *element
                        .surface
                        .cell(local_x, local_y)
                        .expect("local x/y are inside element surface");
                    if element.cell_write_policy == CellWritePolicy::SkipTransparentEmpty
                        && cell.is_empty()
                    {
                        continue;
                    }
                    let role = element
                        .surface
                        .role(local_x, local_y)
                        .expect("local x/y are inside element surface")
                        .clone();
                    surface.set_cell(scene_x, scene_y, cell);
                    match &element.role_write_policy {
                        RoleWritePolicy::PreserveDestination => {}
                        RoleWritePolicy::CopySampledSource => {
                            surface.set_role(scene_x, scene_y, role)
                        }
                        RoleWritePolicy::SetExplicit { role } => {
                            surface.set_role(scene_x, scene_y, role.clone())
                        }
                    }
                    written_cells += 1;
                }
            }
            if element.clip_policy == ClipPolicy::Warn && element_clipped_cells > 0 {
                diagnostics.push(SurfaceDiagnostic::scene_element_clipped(
                    element.id.as_str(),
                    declaration_index,
                    element_clipped_cells,
                ));
            }
        }

        SceneOutcome {
            surface,
            matched_cells,
            written_cells,
            clipped_cells,
            diagnostics,
        }
    }
}

fn in_scene_bounds(x: i32, y: i32, width: usize, height: usize) -> Option<(usize, usize)> {
    if x < 0 || y < 0 {
        return None;
    }
    let x = x as usize;
    let y = y as usize;
    (x < width && y < height).then_some((x, y))
}

// <FILE>crates/tui-vfx-contract/src/cls_scene.rs</FILE> - <DESC>Multi-element semantic scene composition contract</DESC>
// <VERS>END OF VERSION: 0.1.0</VERS>
