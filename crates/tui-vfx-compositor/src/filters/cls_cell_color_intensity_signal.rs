// <FILE>crates/tui-vfx-compositor/src/filters/cls_cell_color_intensity_signal.rs</FILE>
// <DESC>Cell-color intensity sampler: projects a cell's current color onto the lit/unlit axis</DESC>
// <VERS>VERSION: 0.1.0</VERS>
// <WCTX>Glyph rendering framework Phase 4: CellColorIntensitySignal primitive for SubcellLight delegation</WCTX>
// <CLOG>0.1.0: initial implementation; wraps SubcellLight::sample_color + project_intensity as a reusable struct with intensity_for(&Cell)</CLOG>

use tui_vfx_types::{Cell, Color};

use crate::filters::cls_subcell_light::LightSampleFrom;

/// Projects the current color of a cell onto a lit/unlit axis, yielding a
/// scalar intensity in `[0.0, 1.0]`.
///
/// This struct captures the computation previously private to
/// `SubcellLight::sample_color` + `SubcellLight::project_intensity`. It is
/// intentionally **not** a `Signal` implementation. `Signal::sample_with_context`
/// receives `(t, SignalContext)` — no `Cell` reference — so the trait shape
/// cannot represent cell-color sampling without interior mutability. Rather than
/// contorting the trait, `CellColorIntensitySignal` exposes a direct
/// [`intensity_for`](CellColorIntensitySignal::intensity_for) method that
/// `SubcellLight::apply` and any other compositor code can call inline.
///
/// # Design note
///
/// The cell-color path is deliberately excluded from the `Signal` trait per the
/// glyph rendering framework plan §2 (Layer A decision): "Do not invent a new
/// wrapper trait; concrete `Signal` impls live next to their effect math." A
/// cell-color intensity sampler is not a field sampler; it reads rendered state
/// rather than generating values from coordinates and time. The correct position
/// for this type is as a compositor helper alongside `SubcellLight`.
///
/// # Example
///
/// ```rust,ignore
/// use tui_vfx_compositor::filters::cls_cell_color_intensity_signal::CellColorIntensitySignal;
/// use tui_vfx_compositor::filters::cls_subcell_light::LightSampleFrom;
/// use tui_vfx_types::{Cell, Color, Modifiers};
///
/// let sampler = CellColorIntensitySignal {
///     lit: Color::rgb(220, 220, 220),
///     unlit: Color::rgb(24, 24, 24),
///     sample_from: LightSampleFrom::Background,
/// };
/// let cell = Cell::styled(' ', Color::TRANSPARENT, Color::rgb(122, 122, 122), Modifiers::NONE);
/// let intensity = sampler.intensity_for(&cell);
/// assert!((0.0..=1.0).contains(&intensity));
/// ```
pub struct CellColorIntensitySignal {
    /// The color that maps to intensity `1.0`.
    pub lit: Color,
    /// The color that maps to intensity `0.0`.
    pub unlit: Color,
    /// Which cell color channel to sample.
    pub sample_from: LightSampleFrom,
}

impl CellColorIntensitySignal {
    /// Sample the cell's current color and project it onto the `[unlit, lit]`
    /// axis, returning an intensity in `[0.0, 1.0]`.
    ///
    /// Returns `0.0` when:
    /// - `sampled.a == 0` (fully transparent — no light present), or
    /// - `lit == unlit` (zero-length axis — projection is undefined).
    ///
    /// Byte-equivalent to `SubcellLight::sample_color` followed by
    /// `SubcellLight::project_intensity` for the same `lit`/`unlit`/`sample_from`
    /// configuration. See `test_cls_cell_color_intensity_signal.rs` for
    /// cross-validation fixtures.
    #[inline]
    pub fn intensity_for(&self, cell: &Cell) -> f32 {
        let sampled = match self.sample_from {
            LightSampleFrom::Foreground => cell.fg,
            LightSampleFrom::Background => cell.bg,
        };

        if sampled.a == 0 {
            return 0.0;
        }

        let base = [
            self.unlit.r as f32,
            self.unlit.g as f32,
            self.unlit.b as f32,
        ];
        let lit = [self.lit.r as f32, self.lit.g as f32, self.lit.b as f32];
        let sample = [sampled.r as f32, sampled.g as f32, sampled.b as f32];

        let axis = [lit[0] - base[0], lit[1] - base[1], lit[2] - base[2]];
        let axis_len_sq = axis[0] * axis[0] + axis[1] * axis[1] + axis[2] * axis[2];
        if axis_len_sq <= f32::EPSILON {
            return 0.0;
        }

        let delta = [
            sample[0] - base[0],
            sample[1] - base[1],
            sample[2] - base[2],
        ];
        let projected =
            (delta[0] * axis[0] + delta[1] * axis[1] + delta[2] * axis[2]) / axis_len_sq;
        projected.clamp(0.0, 1.0)
    }
}

#[cfg(test)]
#[path = "test_cls_cell_color_intensity_signal.rs"]
mod tests;

// <FILE>crates/tui-vfx-compositor/src/filters/cls_cell_color_intensity_signal.rs</FILE>
// <DESC>Cell-color intensity sampler: projects a cell's current color onto the lit/unlit axis</DESC>
// <VERS>END OF VERSION: 0.1.0</VERS>
