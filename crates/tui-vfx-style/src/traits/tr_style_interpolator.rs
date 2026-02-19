// <FILE>tui-vfx-style/src/traits/tr_style_interpolator.rs</FILE> - <DESC>Trait for calculating style at time t</DESC>
// <VERS>VERSION: 0.1.0 - 2025-12-16T20:53:16Z</VERS>
// <WCTX>Turn 3 Implementation</WCTX>
// <CLOG>Initial definition</CLOG>

use tui_vfx_types::Style;
/// Defines how a style changes over time.
pub trait StyleInterpolator {
    /// Calculate the style at a given progress point `t` (0.0 to 1.0).
    ///
    /// # Arguments
    /// * `t` - Normalized progress (usually 0.0 to 1.0).
    /// * `base` - The underlying style of the widget (allows for blending/alpha).
    fn calculate(&self, t: f64, base: Style) -> Style;
}

// <FILE>tui-vfx-style/src/traits/tr_style_interpolator.rs</FILE> - <DESC>Trait for calculating style at time t</DESC>
// <VERS>END OF VERSION: 0.1.0 - 2025-12-16T20:53:16Z</VERS>
