// <FILE>tui-vfx-compositor/src/traits/sampler.rs</FILE>
// <DESC>Sampler trait + SamplerOutput per-stage output struct (source coord + resolved-coord delta)</DESC>
// <VERS>VERSION: 4.0.0</VERS>
// <WCTX>2026-04-26 packet — return SamplerOutput so the resolved-coord delta can flow into VfxCellContext.with_sampler_resolution and downstream stages can react to per-cell sampler displacement.</WCTX>
// <CLOG>4.0.0: BREAKING — sample(&VfxCellContext) -> Option<(u16, u16)> becomes sample(&VfxCellContext) -> SamplerOutput. SamplerOutput { source: Option<(u16, u16)>, delta_x: i32, delta_y: i32 } with no_displacement / passthrough / displaced builders.</CLOG>

use tui_vfx_types::VfxCellContext;

/// Per-cell output from a Sampler stage application.
///
/// Carries two pieces of data that downstream code reads:
///
/// 1. [`source`] — the source coordinate the orchestrator should sample
///    from for this destination cell. `None` means "transparent / skipped"
///    (same semantics as the legacy `Option<(u16, u16)>` return).
/// 2. [`delta_x`] / [`delta_y`] — the sampler's contribution to the
///    resolved-coord chain in [`VfxCellContext`]. The orchestrator adds
///    these to the prior `resolved_x` / `resolved_y` via
///    [`VfxCellContext::with_sampler_resolution`] before invoking the
///    next stage. Defaults to `(0, 0)` for non-displacing samplers.
///
/// Future per-stage outputs (e.g. a "swap source layer" hint) extend this
/// struct rather than mutating the cross-stage [`VfxCellContext`] bundle.
/// The bundle carries shared per-cell state; this output struct carries
/// stage-specific deltas.
///
/// [`source`]: Self::source
/// [`delta_x`]: Self::delta_x
/// [`delta_y`]: Self::delta_y
#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct SamplerOutput {
    /// Source coordinate to sample from for this destination cell.
    /// `None` skips the cell (transparent).
    pub source: Option<(u16, u16)>,
    /// Sampler's contribution to the resolved-coord chain. Added to the
    /// prior [`VfxCellContext::resolved_x`] by the orchestrator before
    /// the downstream stage runs.
    pub delta_x: i32,
    /// Same for y.
    pub delta_y: i32,
}

impl SamplerOutput {
    /// "No sample" output — transparent / skipped. Source is `None` and
    /// the resolved-coord delta is zero. Use when a sampler decides this
    /// destination cell should not read any source.
    #[inline]
    pub fn no_displacement() -> Self {
        Self { source: None, delta_x: 0, delta_y: 0 }
    }

    /// Pass-through output — read from `(local_x, local_y)` with zero
    /// displacement. Use for non-displacing samplers (color filters
    /// dressed as samplers, identity stages).
    #[inline]
    pub fn passthrough(local_x: u16, local_y: u16) -> Self {
        Self {
            source: Some((local_x, local_y)),
            delta_x: 0,
            delta_y: 0,
        }
    }

    /// Displacing output — read from `(source_x, source_y)` and contribute
    /// `(delta_x, delta_y)` to the resolved-coord chain. Callers compute
    /// `delta_*` against `ctx.local_*`:
    /// `delta_x = source_x as i32 - ctx.local_x as i32`.
    #[inline]
    pub fn displaced(source_x: u16, source_y: u16, delta_x: i32, delta_y: i32) -> Self {
        Self {
            source: Some((source_x, source_y)),
            delta_x,
            delta_y,
        }
    }
}

pub trait Sampler {
    /// Remap a destination cell to a source coordinate, optionally
    /// emitting a resolved-coord delta for downstream stages.
    ///
    /// # Parameters
    /// - `ctx.local_x`, `ctx.local_y`: Destination coordinates to sample / remap.
    /// - `ctx.width`, `ctx.height`: Render area dimensions for spatial awareness.
    /// - `ctx.t`: Animation time (0.0–1.0 for phase-based, seconds for continuous).
    ///
    /// # Returns
    /// A [`SamplerOutput`] carrying:
    /// - `source`: the source coord to read (`None` = transparent / skipped),
    /// - `delta_x` / `delta_y`: the sampler's contribution to the
    ///   resolved-coord chain. Non-displacing samplers return zero deltas
    ///   (e.g. via [`SamplerOutput::passthrough`] /
    ///   [`SamplerOutput::no_displacement`]).
    ///
    /// # Spatial Context
    /// The bundle enables area-aware effects:
    /// - Dynamic centering (e.g., ripple at `(ctx.width/2, ctx.height/2)`)
    /// - Normalized coordinates for position-independent distortion
    /// - Boundary-aware displacement calculations
    ///
    /// Impls that ignore some fields simply do not read them from `ctx`.
    fn sample(&self, ctx: &VfxCellContext) -> SamplerOutput;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sampler_output_no_displacement_is_zero() {
        let out = SamplerOutput::no_displacement();
        assert_eq!(out.source, None);
        assert_eq!(out.delta_x, 0);
        assert_eq!(out.delta_y, 0);
    }

    #[test]
    fn sampler_output_passthrough_carries_local_with_zero_delta() {
        let out = SamplerOutput::passthrough(3, 5);
        assert_eq!(out.source, Some((3, 5)));
        assert_eq!(out.delta_x, 0);
        assert_eq!(out.delta_y, 0);
    }

    #[test]
    fn sampler_output_displaced_carries_deltas() {
        let out = SamplerOutput::displaced(7, 11, 4, 6);
        assert_eq!(out.source, Some((7, 11)));
        assert_eq!(out.delta_x, 4);
        assert_eq!(out.delta_y, 6);
    }
}

// <FILE>tui-vfx-compositor/src/traits/sampler.rs</FILE>
// <VERS>END OF VERSION: 4.0.0</VERS>
