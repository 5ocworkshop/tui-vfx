// <FILE>tui-vfx-geometry/src/types/position_spec.rs</FILE>
// <DESC>Config-friendly position specification (absolute or relative)</DESC>
// <VERS>VERSION: 0.1.0 - 2025-12-17T00:00:00Z</VERS>
// <WCTX>Legacy demo parity: reusable presets</WCTX>
// <CLOG>Added PositionSpec with frame-relative support</CLOG>

use serde::{Deserialize, Serialize};

use super::Position;

/// A config-friendly position that can be resolved against a frame.
///
/// This exists to allow presets to remain portable across terminal sizes.
#[derive(
    Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, tui_vfx_core::ConfigSchema,
)]
#[non_exhaustive]
pub enum PositionSpec {
    /// Signed absolute position (can be offscreen).
    Absolute(Position),
    /// Position expressed in per-mille of the frame (0..=1000).
    ///
    /// - `x_permille = 0`   → left edge
    /// - `x_permille = 500` → center
    /// - `x_permille = 1000` → right edge
    ///
    /// Values are clamped and resolved inside the frame.
    FramePermille {
        #[config(
            help = "X in per-mille of frame width (0..=1000)",
            default = 500,
            min = 0,
            max = 1000
        )]
        x_permille: u16,
        #[config(
            help = "Y in per-mille of frame height (0..=1000)",
            default = 500,
            min = 0,
            max = 1000
        )]
        y_permille: u16,
    },
}

impl Default for PositionSpec {
    fn default() -> Self {
        Self::Absolute(Position::new(0, 0))
    }
}

impl PositionSpec {
    /// Resolves the spec to an absolute signed position within the given `frame_area`.
    pub fn resolve_in_frame(self, frame_area: tui_vfx_types::Rect) -> Position {
        match self {
            PositionSpec::Absolute(p) => p,
            PositionSpec::FramePermille {
                x_permille,
                y_permille,
            } => {
                let w = frame_area.width as i32;
                let h = frame_area.height as i32;
                if w <= 0 || h <= 0 {
                    return Position::new(frame_area.x as i32, frame_area.y as i32);
                }

                let x = (w as i64 * x_permille as i64 / 1000) as i32;
                let y = (h as i64 * y_permille as i64 / 1000) as i32;

                // Clamp to an addressable cell within the frame.
                let x = x.clamp(0, w.saturating_sub(1));
                let y = y.clamp(0, h.saturating_sub(1));

                Position::new(frame_area.x as i32 + x, frame_area.y as i32 + y)
            }
        }
    }
}

// <FILE>tui-vfx-geometry/src/types/position_spec.rs</FILE>
// <DESC>Config-friendly position specification (absolute or relative)</DESC>
// <VERS>END OF VERSION: 0.1.0 - 2025-12-17T00:00:00Z</VERS>
