// <FILE>tui-vfx-style/src/models/cls_style_region.rs</FILE> - <DESC>Style region targeting enum (data-only; predicate + bounding-rect + resolved logic lives in fnc_style_region_* siblings; Deserialize and ConfigSchema impls live in fnc_style_region_deserialize / fnc_style_region_schema)</DESC>
// <VERS>VERSION: 5.2.0</VERS>
// <WCTX>Phase 3b: lift RowRange/ColumnRange start+end and Modulo modulus+remainder from raw u16 to BindableU16 so SynthGrid expand/collapse and animated stripe density become first-class authoring primitives. resolved() extracted to its own sibling so the new variant resolution doesn't push this cls_ further past its size budget.</WCTX>
// <CLOG>RowRange/ColumnRange/Modulo fields are now BindableU16 (back-compat: bare integers still deserialise via BindableU16Repr::Bare). resolved() delegates to fnc_style_region_resolved which now also resolves the three new variants when any field is a Binding.</CLOG>

use super::cls_bindable_u16::BindableU16;
use super::{
    fnc_style_region_bounding_rect, fnc_style_region_resolved, fnc_style_region_should_style,
};
use crate::traits::ShaderRuntimeParams;
use serde::{Deserialize, Serialize};
use std::borrow::Cow;
use tui_vfx_types::{Rect, RoleTag};

/// A cell coordinate for per-cell targeting.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, tui_vfx_core::ConfigSchema)]
#[serde(deny_unknown_fields)]
pub struct CellCoord {
    /// X coordinate (column, 0-based from left)
    pub x: u16,
    /// Y coordinate (row, 0-based from top)
    pub y: u16,
}

impl CellCoord {
    /// Create a new cell coordinate.
    pub fn new(x: u16, y: u16) -> Self {
        Self { x, y }
    }
}

/// Axis for modulo-based region targeting.
///
/// Used with `StyleRegion::Modulo` to specify whether the pattern
/// applies to rows (Horizontal) or columns (Vertical).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, tui_vfx_core::ConfigSchema)]
#[serde(rename_all = "PascalCase")]
pub enum ModuloAxis {
    /// Apply modulo pattern to rows (y coordinate)
    /// e.g., for CRT scanlines that span horizontally
    Horizontal,
    /// Apply modulo pattern to columns (x coordinate)
    /// e.g., for vertical stripe effects
    Vertical,
}

/// Specifies which region of a widget should receive style effects.
///
/// # Role-aware targeting (v5.0.0)
///
/// The `Role(RoleTag)` variant addresses cells by their semantic role as
/// carried on the source surface's `RoleMap`. This is the preferred
/// modern form for role-based targeting (border, text, background, …).
///
/// The historical bare variants `BorderOnly`, `TextOnly`, and
/// `BackgroundOnly` are no longer present in the Rust enum — they have
/// been fully replaced by `Role(RoleTag::Border)`, `Role(RoleTag::Text)`,
/// and `Role(RoleTag::Background)` respectively. Existing JSON recipe
/// fixtures that still write the legacy bare strings continue to parse
/// via a custom `Deserialize` impl that lowers them to the canonical
/// `Role(...)` form. Serialization always emits the canonical form, so
/// round-tripping converges.
///
/// # Geometry variants (unchanged)
///
/// `All`, `Rows`, `RowRange`, `Cell`, `Cells`, `Column`, `Columns`,
/// `ColumnRange`, and `Modulo` remain geometry-only variants evaluated
/// by coordinate, unaffected by role information.
///
/// # Custom `Deserialize`
///
/// Rather than leaning on `#[serde(rename_all = "PascalCase")]`, this
/// type implements `Deserialize` by hand so it can accept BOTH:
///
/// - the modern tagged / role form (`"All"`, `{"Rows": …}`,
///   `{"Role": "border"}`, `{"Role": {"Custom": "my_role"}}`), and
/// - the legacy bare strings (`"BorderOnly"`, `"TextOnly"`,
///   `"BackgroundOnly"`) mapped onto canonical `Role(...)` form.
#[derive(Clone, PartialEq, Eq, Default, Debug, Serialize)]
#[serde(rename_all = "PascalCase")]
pub enum StyleRegion {
    /// Apply style to the entire widget (default behavior)
    #[default]
    All,
    /// Apply style to cells whose semantic role matches the tag.
    ///
    /// Source role information is carried on `SemanticScene::roles()` (a
    /// `RoleMap`). When the hot loop evaluates `Role(RoleTag::Border)`
    /// against cell `(x, y)`, it reads `source_roles.get((x, y))` and
    /// emits `true` iff that value equals `Border`.
    Role(RoleTag),
    /// Apply style only to specific rows (0-based from widget top)
    Rows(Vec<u16>),
    /// Apply style to a contiguous range of rows [start, end).
    ///
    /// Both `start` and `end` are [`BindableU16`] so the range itself can be
    /// runtime-bound (the SynthGrid expand/collapse animation drives a
    /// `{"binding": "synth_grid_end_row"}` here). The binding must be
    /// resolved into literals via [`StyleRegion::resolved`] before
    /// `should_style` evaluates the range — the hot render loop in
    /// `tui-vfx-compositor::pipeline::orc_render_pipeline` does this once
    /// per layer per frame.
    RowRange {
        /// First row to include (0-based)
        start: BindableU16,
        /// First row to exclude (exclusive end)
        end: BindableU16,
    },
    /// Apply style to a single cell at (x, y).
    ///
    /// Both coordinates are [`BindableU16`]: either a raw integer
    /// (backwards-compatible with pre-P0.2 recipes) or a
    /// `{"binding": "name"}` form that resolves from `ShaderRuntimeParams`
    /// at render time. The binding must be resolved into a literal via
    /// [`StyleRegion::resolved`] before `should_style` or `bounding_rect`
    /// can interpret the coordinates — the hot render loop in
    /// `tui-vfx-compositor::pipeline::orc_render_pipeline` does this once
    /// per layer per frame.
    Cell {
        /// X coordinate (column, 0-based from left)
        x: BindableU16,
        /// Y coordinate (row, 0-based from top)
        y: BindableU16,
    },
    /// Apply style to multiple specific cells
    Cells(Vec<CellCoord>),
    /// Apply style only to a specific column (0-based from left)
    Column(u16),
    /// Apply style only to specific columns (0-based from left)
    Columns(Vec<u16>),
    /// Apply style to a contiguous range of columns [start, end).
    ///
    /// `start` and `end` are [`BindableU16`] so the range can be driven by a
    /// runtime parameter (column-wise SynthGrid animation, animated
    /// reveal-by-column, etc.). See [`StyleRegion::resolved`] for the
    /// resolution contract — same shape as `RowRange`.
    ColumnRange {
        /// First column to include (0-based)
        start: BindableU16,
        /// First column to exclude (exclusive end)
        end: BindableU16,
    },
    /// Apply style to rows/columns matching a modulo pattern.
    ///
    /// Useful for CRT scanline effects, alternating stripes, or periodic
    /// patterns. `modulus` and `remainder` are [`BindableU16`] so stripe
    /// density and phase can be driven by a runtime parameter (animated
    /// frequency sweep, beat-synced stripes, etc.). See
    /// [`StyleRegion::resolved`] for the resolution contract.
    Modulo {
        /// Which axis to apply the modulo pattern to
        axis: ModuloAxis,
        /// The divisor for the modulo operation (e.g., 2 for every other)
        modulus: BindableU16,
        /// The remainder to match (e.g., 0 for 0,2,4... or 1 for 1,3,5...)
        remainder: BindableU16,
    },
}

impl StyleRegion {
    /// Check if a cell at `(x, y)` within `area` should receive styling,
    /// given the cell's optional semantic `role`.
    ///
    /// Thin delegator to [`fnc_style_region_should_style::should_style`].
    /// For legacy `(x, y, w, h)` call-sites that don't yet have role
    /// information, use [`Self::should_style_legacy`] which threads
    /// `None` automatically.
    pub fn should_style(&self, x: u16, y: u16, role: Option<RoleTag>, area: Rect) -> bool {
        fnc_style_region_should_style::should_style(self, x, y, role, area)
    }

    /// Legacy call-shape helper: lift a `(x, y, width, height)` call into
    /// the new role-aware signature by passing `role = None` and
    /// `area = Rect::new(0, 0, width, height)`.
    ///
    /// This exists for call-sites that cannot yet supply role information
    /// (for example unit tests for the geometry variants). Do not use it
    /// on the hot render path — that path SHOULD thread `role` from the
    /// source `RoleMap`.
    pub fn should_style_legacy(&self, x: u16, y: u16, width: u16, height: u16) -> bool {
        self.should_style(x, y, None, Rect::new(0, 0, width, height))
    }

    /// Get the bounding rectangle for this region within the widget `area`.
    ///
    /// Thin delegator to [`fnc_style_region_bounding_rect::bounding_rect`].
    pub fn bounding_rect(&self, area: Rect) -> Option<Rect> {
        fnc_style_region_bounding_rect::bounding_rect(self, area)
    }

    /// Legacy call-shape helper: return the bounding rectangle for this
    /// region, as a `(x, y, width, height)` tuple, passing a zero-origin
    /// `Rect` of unbounded size so that the result describes the region
    /// itself (not any intersection with the widget area).
    pub fn bounding_rect_legacy(&self) -> Option<(u16, u16, u16, u16)> {
        self.bounding_rect(Rect::new(0, 0, u16::MAX, u16::MAX))
            .map(|r| (r.x, r.y, r.width, r.height))
    }

    /// Convert grid coordinates to region-relative coordinates.
    ///
    /// Returns `Some((local_x, local_y, region_width, region_height))` if the region
    /// has a computable bounding box, or `None` for unbounded regions.
    pub fn to_local_coords(&self, x: u16, y: u16) -> Option<(u16, u16, u16, u16)> {
        self.bounding_rect_legacy()
            .map(|(min_x, min_y, width, height)| {
                let local_x = x.saturating_sub(min_x);
                let local_y = y.saturating_sub(min_y);
                (local_x, local_y, width, height)
            })
    }

    /// Return a version of this region with any runtime-parameter bindings
    /// resolved to concrete literals.
    ///
    /// Borrows when no resolution is needed; clones only for variants whose
    /// `BindableU16` field is currently a `Binding`. Thin delegator to
    /// [`fnc_style_region_resolved::resolved`].
    pub fn resolved<'a>(&'a self, runtime_params: &ShaderRuntimeParams) -> Cow<'a, StyleRegion> {
        fnc_style_region_resolved::resolved(self, runtime_params)
    }
}

// <FILE>tui-vfx-style/src/models/cls_style_region.rs</FILE> - <DESC>Style region targeting enum</DESC>
// <VERS>END OF VERSION: 5.2.0</VERS>
