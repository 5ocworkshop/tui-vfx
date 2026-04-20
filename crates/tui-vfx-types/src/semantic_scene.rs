// <FILE>crates/tui-vfx-types/src/semantic_scene.rs</FILE> - <DESC>SemanticScene: source surface annotated with per-cell role tags</DESC>
// <VERS>VERSION: 0.2.0</VERS>
// <WCTX>Sub-plan A Phase A.2 — expose grid_mut() so pipeline stages can write cells into the destination scene's underlying OwnedGrid</WCTX>
// <CLOG>0.2.0: MINOR — add grid_mut() accessor. The compositor needs a `&mut OwnedGrid` (which implements `Grid`) to write into the destination; grid_mut() pairs with the existing roles_mut() so role-producing stages (e.g. shadow) can write both cells and tags through the SemanticScene interface.
// 0.1.0: initial SemanticScene with OwnedGrid + RoleMap + SceneMetadata; Buffer-parity accessors (area, cell); role accessor; from_grid_with_default_role builder; dimension-mismatch panic in new() documented as library-misuse.</CLOG>

//! `SemanticScene`: a source surface annotated with per-cell role tags.
//!
//! `SemanticScene` is the foundation primitive that unifies **composed
//! scenes** (produced by `tui-vfx-recipes::scene`) and **widget-rendered
//! sources** (produced by lowering `gtd-factory::SemanticBuffer`) into a
//! single type. Both feed the per-cell pipeline identically.
//!
//! # Accessor parity with `Buffer`
//!
//! `SemanticScene` deliberately mirrors the accessor names used by
//! `ratatui`'s `Buffer` so recipe-side and widget-side call sites use a
//! familiar vocabulary:
//!
//! | Buffer accessor | SemanticScene accessor |
//! |-----------------|------------------------|
//! | `area()`        | `area()`               |
//! | `cell((x, y))`  | `cell((x, y))`         |
//!
//! Plus two role-specific accessors:
//!
//! - `role((x, y)) -> Option<RoleTag>`
//! - `roles() -> &RoleMap` / `roles_mut() -> &mut RoleMap`
//!
//! # Examples
//!
//! ```
//! use tui_vfx_types::{Cell, Grid, OwnedGrid, RoleMap, RoleTag, SemanticScene};
//!
//! let mut grid = OwnedGrid::new(4, 3);
//! grid.set(1, 1, Cell::new('X'));
//! let mut roles = RoleMap::empty(4, 3);
//! roles.set((1, 1), RoleTag::Text);
//!
//! let scene = SemanticScene::new(grid, roles);
//!
//! // Buffer-style cell access:
//! assert_eq!(scene.cell((1, 1)).map(|c| c.ch), Some('X'));
//! // Role-tag access:
//! assert_eq!(scene.role((1, 1)), Some(RoleTag::Text));
//! assert_eq!(scene.role((0, 0)), Some(RoleTag::Background));
//! // Area mirrors ratatui's Buffer::area():
//! assert_eq!(scene.area().width, 4);
//! assert_eq!(scene.area().height, 3);
//! ```

use crate::{Cell, Grid, OwnedGrid, Rect, RoleMap, RoleTag, SceneMetadata};

/// A source surface annotated with per-cell semantic role tags.
///
/// Holds an `OwnedGrid` (the cells), a `RoleMap` (role tags matching the
/// grid's dimensions), and `SceneMetadata` (scene-level inspection info).
#[derive(Clone, Debug)]
pub struct SemanticScene {
    grid: OwnedGrid,
    roles: RoleMap,
    metadata: SceneMetadata,
}

impl SemanticScene {
    /// Construct a `SemanticScene` from a grid and role map.
    ///
    /// # Panics
    ///
    /// Panics with a clear diagnostic if `grid.width() != roles.width()`
    /// or `grid.height() != roles.height()`. This is a library-misuse
    /// panic: callers are expected to produce a grid and role map in the
    /// same construction pass, so a mismatch indicates a programmer error
    /// rather than a recoverable runtime condition.
    pub fn new(grid: OwnedGrid, roles: RoleMap) -> Self {
        let gw = grid.width();
        let gh = grid.height();
        let rw = roles.width() as usize;
        let rh = roles.height() as usize;
        assert!(
            gw == rw && gh == rh,
            "SemanticScene::new dimension mismatch: grid is {gw}x{gh} but role map is {rw}x{rh}"
        );
        Self {
            grid,
            roles,
            metadata: SceneMetadata::default(),
        }
    }

    /// Builder: replace the scene's metadata.
    pub fn with_metadata(mut self, metadata: SceneMetadata) -> Self {
        self.metadata = metadata;
        self
    }

    /// Convenience constructor: build a `SemanticScene` from a grid alone,
    /// tagging every cell with `default`.
    ///
    /// This is the migration helper callers reach for when they have a
    /// grid but no role information yet (e.g. legacy call-sites during
    /// Sub-plan A migration). Typical uses pass `RoleTag::Background`.
    pub fn from_grid_with_default_role(grid: OwnedGrid, default: RoleTag) -> Self {
        let w = grid.width() as u16;
        let h = grid.height() as u16;
        let roles = RoleMap::new_with_default(w, h, default);
        Self {
            grid,
            roles,
            metadata: SceneMetadata::default(),
        }
    }

    /// Return the scene's `Rect`: origin `(0, 0)`, width/height matching
    /// the underlying grid. Mirrors `ratatui::Buffer::area()`.
    pub fn area(&self) -> Rect {
        Rect::new(0, 0, self.grid.width() as u16, self.grid.height() as u16)
    }

    /// Borrow the cell at `(x, y)`.
    ///
    /// Returns `None` for out-of-bounds coordinates. Parity with
    /// `ratatui::Buffer::cell((x, y))`.
    pub fn cell(&self, pos: (u16, u16)) -> Option<&Cell> {
        self.grid.get(pos.0 as usize, pos.1 as usize)
    }

    /// Fetch the role tag at `(x, y)`.
    ///
    /// Returns `None` for out-of-bounds coordinates.
    pub fn role(&self, pos: (u16, u16)) -> Option<RoleTag> {
        self.roles.get(pos)
    }

    /// Borrow the underlying cell grid.
    pub fn grid(&self) -> &OwnedGrid {
        &self.grid
    }

    /// Mutably borrow the underlying cell grid.
    ///
    /// Pipeline stages that write cells into the destination use this to
    /// get a `&mut OwnedGrid` (which implements `Grid`). Pair with
    /// `roles_mut()` when a stage also needs to write role tags for the
    /// cells it produces (e.g. the shadow stage writing
    /// `RoleTag::Shadow`).
    pub fn grid_mut(&mut self) -> &mut OwnedGrid {
        &mut self.grid
    }

    /// Borrow the role map.
    pub fn roles(&self) -> &RoleMap {
        &self.roles
    }

    /// Mutably borrow the role map (e.g. for pipeline stages that write
    /// `RoleTag::Shadow` into destination cells).
    pub fn roles_mut(&mut self) -> &mut RoleMap {
        &mut self.roles
    }

    /// Borrow the scene's metadata.
    pub fn metadata(&self) -> &SceneMetadata {
        &self.metadata
    }
}

// <FILE>crates/tui-vfx-types/src/semantic_scene.rs</FILE> - <DESC>SemanticScene foundation primitive</DESC>
// <VERS>END OF VERSION: 0.2.0</VERS>
