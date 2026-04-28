// <FILE>crates/tui-vfx-contract/src/cls_surface.rs</FILE> - <DESC>Grid-first semantic surface type</DESC>
// <VERS>VERSION: 0.4.0</VERS>
// <WCTX>New kernel Phase D0: make Surface schema-reference ready without exposing OwnedGrid storage.</WCTX>
// <CLOG>0.4.0: PATCH — store explicit width/height/cells/roles fields so Serde/Schemars publish the public surface contract shape while preserving behavior.
// 0.3.0: REFACTOR — keep Surface in one cohesive class file and move helper DTOs out.</CLOG>

use tui_vfx_types::{Cell, RoleTag};

use crate::SurfaceMetadata;

/// # Surface
///
/// Dense rectangular semantic render surface used by the v3.1 clean-room kernel.
///
/// A surface owns visual cells and one semantic role per cell in row-major order.
/// The cell and role vectors must both contain `width * height` entries.
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, schemars::JsonSchema)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct Surface {
    /// Surface width in cells.
    width: usize,
    /// Surface height in cells.
    height: usize,
    /// Visual cell grid stored in row-major order.
    cells: Vec<Cell>,
    /// One semantic role per surface cell, stored in row-major order.
    roles: Vec<RoleTag>,
    /// Optional producer and layer metadata.
    metadata: SurfaceMetadata,
}

impl Surface {
    /// Create a new surface filled with transparent empty cells and one role.
    pub fn new(width: usize, height: usize, role: RoleTag) -> Self {
        Self {
            width,
            height,
            cells: vec![Cell::default(); width * height],
            roles: vec![role; width * height],
            metadata: SurfaceMetadata::default(),
        }
    }

    /// Create a surface with explicit cells and roles.
    pub fn from_cells(
        width: usize,
        height: usize,
        cells: Vec<Cell>,
        roles: Vec<RoleTag>,
        metadata: SurfaceMetadata,
    ) -> Self {
        assert_eq!(
            cells.len(),
            width * height,
            "cell count must match dimensions"
        );
        assert_eq!(
            roles.len(),
            width * height,
            "role count must match dimensions"
        );
        Self {
            width,
            height,
            cells,
            roles,
            metadata,
        }
    }

    /// Surface width in cells.
    pub fn width(&self) -> usize {
        self.width
    }

    /// Surface height in cells.
    pub fn height(&self) -> usize {
        self.height
    }

    /// Whether the coordinate is inside the surface.
    pub fn in_bounds(&self, x: usize, y: usize) -> bool {
        x < self.width && y < self.height
    }

    /// Read a cell.
    pub fn cell(&self, x: usize, y: usize) -> Option<&Cell> {
        self.index(x, y).map(|index| &self.cells[index])
    }

    /// Read a role.
    pub fn role(&self, x: usize, y: usize) -> Option<&RoleTag> {
        self.index(x, y).map(|index| &self.roles[index])
    }

    /// Set a cell without changing its role.
    pub fn set_cell(&mut self, x: usize, y: usize, cell: Cell) {
        if let Some(index) = self.index(x, y) {
            self.cells[index] = cell;
        }
    }

    /// Set a role without changing cell channels.
    pub fn set_role(&mut self, x: usize, y: usize, role: RoleTag) {
        if let Some(index) = self.index(x, y) {
            self.roles[index] = role;
        }
    }

    /// Set a cell and role together.
    pub fn set_cell_and_role(&mut self, x: usize, y: usize, cell: Cell, role: RoleTag) {
        self.set_cell(x, y, cell);
        self.set_role(x, y, role);
    }

    /// Borrow metadata.
    pub fn metadata(&self) -> &SurfaceMetadata {
        &self.metadata
    }

    /// Mutably borrow metadata.
    pub fn metadata_mut(&mut self) -> &mut SurfaceMetadata {
        &mut self.metadata
    }

    /// Read all roles in row-major order.
    pub fn roles(&self) -> &[RoleTag] {
        &self.roles
    }

    fn index(&self, x: usize, y: usize) -> Option<usize> {
        self.in_bounds(x, y).then(|| y * self.width + x)
    }
}

// <FILE>crates/tui-vfx-contract/src/cls_surface.rs</FILE> - <DESC>Grid-first semantic surface type</DESC>
// <VERS>END OF VERSION: 0.4.0</VERS>
